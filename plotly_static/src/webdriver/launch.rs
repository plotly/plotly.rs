//! WebDriver process launch: spawn, connect-to-existing, and binary resolution.

use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
#[cfg(any(test, feature = "debug"))]
use std::{println as debug, println as error, println as info, println as trace, println as warn};

use anyhow::{anyhow, Result};
#[cfg(not(any(test, feature = "debug")))]
use log::{debug, error, info, trace, warn};

use super::{WdInner, WebDriver};

const WEBDRIVER_PATH_ENV: &str = "WEBDRIVER_PATH";

#[cfg(feature = "geckodriver")]
const WEBDRIVER_BIN: &str = "geckodriver";

#[cfg(feature = "chromedriver")]
const WEBDRIVER_BIN: &str = "chromedriver";

impl WebDriver {
    /// Creates a new WebDriver instance for spawning.
    ///
    /// This method creates a WebDriver instance that will later spawn a new
    /// process. It looks for the WebDriver binary in the following order:
    /// 1. `WEBDRIVER_PATH` environment variable (should point to full
    ///    executable path)
    /// 2. Build-time downloaded path (if `webdriver_download` feature is
    ///    enabled)
    ///
    /// Returns a `Result<WebDriver>` where:
    /// - `Ok(webdriver)` - Successfully created the WebDriver instance
    /// - `Err(e)` - Failed to create the instance (e.g., binary not found)
    pub(crate) fn new(port: u32) -> Result<Self> {
        let full_path = Self::get_webdriver_path()?;

        Ok(Self {
            inner: Arc::new(Mutex::new(WdInner {
                webdriver_port: port,
                driver_path: Some(full_path),
                webdriver_child: None,
                is_external: false, // mark it as spawned by us
            })),
        })
    }

    /// Spawn a new WebDriver instance, connecting to existing if available.
    ///
    /// This method provides WebDriver management:
    /// 1. First checks if a WebDriver is already running on the specified port
    /// 2. If found, connects to the existing session
    /// 3. If not found, spawns a process of the current WebDriver instance
    ///
    /// This approach allows for efficient resource usage and supports both
    /// scenarios where WebDriver is managed externally or needs to be spawned.
    ///
    /// Returns a `Result<WebDriver>` where:
    /// - `Ok(webdriver)` - Successfully created or connected to WebDriver
    /// - `Err(e)` - Failed to create or connect to WebDriver
    pub(crate) fn connect_or_spawn(port: u32) -> Result<Self> {
        match Self::try_connect(port) {
            Some(active_instance) => Ok(active_instance),
            None => Self::spawn(port)
                .map_err(|e| anyhow!("Failed to spawn new WebDriver on port {}: {}", port, e)),
        }
    }

    /// Try connecting to an existing WebDriver instance
    pub(crate) fn try_connect(port: u32) -> Option<Self> {
        if !Self::is_webdriver_running(port) {
            return None;
        }
        info!("WebDriver already running on port {port}, connecting to existing session");
        Self::no_spawn_instance(port).ok()
    }

    /// Create a new WebDriver instance without a path to the webdriver binary,
    /// as it is used for connecting to an existing WebDriver session and not
    /// spawning a new process.
    pub(crate) fn no_spawn_instance(port: u32) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(WdInner {
                webdriver_port: port,
                driver_path: None,
                webdriver_child: None,
                is_external: true, // Mark as external since we didn't spawn it
            })),
        })
    }

    /// Spawns the WebDriver process
    ///
    /// This method starts the WebDriver process in a background thread and
    /// captures its output for logging. The process is spawned with the
    /// specified port and appropriate I/O redirection.
    ///
    /// The spawned process will be automatically managed and can be stopped
    /// using the `stop()` method.
    pub(crate) fn spawn(port: u32) -> Result<Self> {
        debug!("No WebDriver running on port {port}, creating new instance and spawning");

        let mut wd = Self::new(port)?;
        wd.spawn_webdriver()?;

        if Self::is_webdriver_running(port) {
            info!("Successfully created and started WebDriver on port {port}");
            Ok(wd)
        } else {
            let diagnostics = wd.get_diagnostics();
            error!(
                "WebDriver failed to start properly on port {port}. Diagnostics:\n{diagnostics}"
            );
            Err(anyhow!(
                "WebDriver failed to start properly on port {}",
                port
            ))
        }
    }

    pub(crate) fn spawn_webdriver(&mut self) -> Result<()> {
        let port = self.inner.lock().unwrap().webdriver_port;
        let driver_path = self.inner.lock().unwrap().driver_path.clone();

        info!("Spawning {WEBDRIVER_BIN} on port {port} with path: {driver_path:?}");

        if Self::is_webdriver_running(port) {
            warn!("WebDriver already running on port {port}, attempting to connect instead");
            return Ok(());
        }

        self.validate_spawn_prerequisites()?;

        let mut child = self.spawn_process(&driver_path, port)?;

        self.setup_output_monitoring(&mut child, port);

        self.store_child_process(child);

        self.wait_for_ready(port)
    }

    fn validate_spawn_prerequisites(&self) -> Result<()> {
        let inner = self
            .inner
            .lock()
            .map_err(|e| anyhow!("Failed to acquire lock: {}", e))?;

        // Check if driver path exists
        let driver_path = inner.driver_path.as_ref().ok_or_else(|| {
            error!(
                "WebDriver diagnostics after missing driver path:\n{}",
                self.get_diagnostics()
            );
            anyhow!("No driver path available for spawning")
        })?;

        // Check if binary exists
        if !driver_path.exists() {
            error!(
                "WebDriver diagnostics after missing binary:\n{}",
                self.get_diagnostics()
            );
            return Err(anyhow!("WebDriver binary does not exist: {driver_path:?}"));
        }

        Ok(())
    }

    fn spawn_process(&self, driver_path: &Option<PathBuf>, port: u32) -> Result<Child> {
        let driver_path = driver_path.as_ref().unwrap(); // Safe unwrap since we validated above

        let mut command = Self::create_command(driver_path, port);
        Self::log_command(&command);

        match command.spawn() {
            Ok(child) => Ok(child),
            Err(e) => {
                #[cfg(not(target_os = "windows"))]
                {
                    Err(self.handle_spawn_error(e, &command, "standard method"))
                }
                #[cfg(target_os = "windows")]
                {
                    self.spawn_with_fallback(driver_path, port, e)
                }
            }
        }
    }

    /// Windows-specific fallback spawn method when CREATE_NO_WINDOW fails
    #[cfg(target_os = "windows")]
    fn spawn_with_fallback(
        &self,
        driver_path: &PathBuf,
        port: u32,
        original_error: std::io::Error,
    ) -> Result<Child> {
        // If CREATE_NO_WINDOW fails, try without any special flags
        error!("Failed to spawn with CREATE_NO_WINDOW: {original_error}");
        error!("Trying without special creation flags...");

        let mut fallback_command = Self::standard_command(driver_path, port);
        Self::log_command(&fallback_command);

        match fallback_command.spawn() {
            Ok(child) => {
                info!("Successfully spawned WebDriver without special creation flags");
                Ok(child)
            }
            Err(fallback_e) => {
                error!("Original error: {original_error}");
                error!("Fallback error: {fallback_e}");
                Err(self.handle_spawn_error(fallback_e, &fallback_command, "fallback method"))
            }
        }
    }

    /// Creates a command with standard configuration (no Windows flags)
    fn standard_command(driver_path: &PathBuf, port: u32) -> Command {
        let mut command = Command::new(driver_path);
        command.arg(format!("--port={port}"));

        // Add verbose flag only for chromedriver
        #[cfg(feature = "chromedriver")]
        command.arg("--verbose");

        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        command
    }

    /// Creates a command with Windows-specific flags
    #[cfg(target_os = "windows")]
    fn create_command(driver_path: &PathBuf, port: u32) -> Command {
        use std::os::windows::process::CommandExt;

        let mut command = Self::standard_command(driver_path, port);
        // Try with CREATE_NO_WINDOW for headless operation
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW flag
        command
    }

    #[cfg(not(target_os = "windows"))]
    fn create_command(driver_path: &PathBuf, port: u32) -> Command {
        Self::standard_command(driver_path, port)
    }

    /// Logs command execution details
    fn log_command(command: &Command) {
        info!(
            "Executing command: {:?} {:?}",
            command.get_program(),
            command.get_args()
        );
    }

    /// Handles spawn errors with appropriate logging and diagnostics
    fn handle_spawn_error(
        &self,
        e: std::io::Error,
        command: &Command,
        attempt: &str,
    ) -> anyhow::Error {
        error!("Failed to spawn '{WEBDRIVER_BIN}' with {attempt}: {e}");
        error!(
            "Command was: {:?} {:?}",
            command.get_program(),
            command.get_args()
        );

        #[cfg(target_os = "windows")]
        if attempt == "CREATE_NO_WINDOW" {
            error!("Windows: Check if antivirus is blocking the process");
            error!("Windows: Check if the binary has proper permissions");
        }

        error!(
            "WebDriver diagnostics after spawn failure:\n{}",
            self.get_diagnostics()
        );

        anyhow!("Failed to spawn '{WEBDRIVER_BIN}': {}", e)
    }

    fn setup_output_monitoring(&self, child: &mut Child, port: u32) {
        // Monitor stderr
        if let Some(stderr) = child.stderr.take() {
            let port_for_logging = port;
            thread::spawn(move || {
                info!("Starting stderr monitoring for WebDriver on port {port_for_logging}");
                let stderr_lines = BufReader::new(stderr).lines();
                for line in stderr_lines.map_while(Result::ok) {
                    trace!("WebDriver[{port_for_logging}] stderr: {line}");
                }
                info!("Stderr monitoring ended for WebDriver on port {port_for_logging}");
            });
        }

        // Monitor stdout
        if let Some(stdout) = child.stdout.take() {
            let port_for_logging = port;
            thread::spawn(move || {
                info!("Starting stdout monitoring for WebDriver on port {port_for_logging}");
                let stdout_lines = BufReader::new(stdout).lines();
                for line in stdout_lines.map_while(Result::ok) {
                    trace!("WebDriver[{port_for_logging}] stdout: {line}");
                }
                info!("Stdout monitoring ended for WebDriver on port {port_for_logging}");
            });
        }
    }

    fn store_child_process(&mut self, child: Child) {
        let mut inner = self.inner.lock().unwrap();
        inner.webdriver_child = Some(child);
        info!("WebDriver process stored, waiting for it to become ready...");
    }

    fn wait_for_ready(&self, port: u32) -> Result<()> {
        let start_time = std::time::Instant::now();
        let timeout_duration = if cfg!(target_os = "windows") {
            std::time::Duration::from_secs(60)
        } else {
            std::time::Duration::from_secs(30)
        };

        while start_time.elapsed() < timeout_duration {
            if Self::is_webdriver_running(port) {
                info!(
                    "WebDriver is ready on port {} after {:?}",
                    port,
                    start_time.elapsed()
                );
                return Ok(());
            }

            // Check if process is still alive
            if let Some(child) = self.inner.lock().unwrap().webdriver_child.as_mut() {
                if let Ok(Some(_)) = child.try_wait() {
                    error!("WebDriver process exited before becoming ready on port {port}");
                    return Err(anyhow!(
                        "WebDriver process exited before becoming ready on port {}",
                        port
                    ));
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        error!("WebDriver failed to become ready on port {port} within {timeout_duration:?}");
        Err(anyhow!(
            "WebDriver failed to become ready on port {} within {:?}",
            port,
            timeout_duration
        ))
    }

    fn get_webdriver_path() -> Result<PathBuf> {
        use std::env;

        let path = match env::var(WEBDRIVER_PATH_ENV) {
            Ok(runtime_env) => runtime_env,
            Err(runtime_env_err) => match option_env!("WEBDRIVER_DOWNLOAD_PATH") {
                Some(compile_time_path) => compile_time_path.to_string(),
                None => {
                    debug!("{WEBDRIVER_PATH_ENV}: {runtime_env_err}");
                    warn!("Use the plotly_static's `webdriver_download` feature to automatically download, install and use the the chosen WebDriver for supported platforms. Or manually set {WEBDRIVER_PATH_ENV} to point to the WebDriver binary.");
                    return Err(anyhow!(
                        "WebDriver binary not available. Set {} environment variable or use the webdriver_download feature",
                        WEBDRIVER_PATH_ENV
                    ));
                }
            },
        };

        Self::full_path(&path).map_err(|e| anyhow!("Invalid WebDriver path '{}': {}", path, e))
    }

    fn full_path(path: &str) -> Result<PathBuf> {
        let p = PathBuf::from(path);
        if !p.exists() {
            Err(anyhow!(
                "WebDriver executable not found at provided path: '{}'",
                p.display()
            ))
        } else {
            Ok(p)
        }
    }
}
