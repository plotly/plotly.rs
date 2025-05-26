use anyhow::Context;
use anyhow::{anyhow, Result};
#[cfg(not(test))]
use log::{debug, error, info, warn};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(test)]
use std::{println as info, println as warn, println as error, println as debug};

const WEBDRIVER_PATH_ENV: &str = "WEBDRIVER_PATH";

#[cfg(feature = "geckodriver")]
const WEBDRIVER_APP: &str = "geckodriver";

#[cfg(feature = "chromedriver")]
const WEBDRIVER_APP: &str = "chromedriver";

pub(crate) const WEBDRIVER_PORT: u32 = 4444;
pub(crate) const WEBDRIVER_URL: &str = "http://localhost";

#[derive(Debug)]
struct WdInner {
    webdriver_port: u32,
    webdriver_url: String,
    driver_path: PathBuf,
    webdriver_process_id: Option<u32>,
}

pub struct WebDriver {
    inner: Arc<Mutex<WdInner>>,
}

impl WebDriver {
    pub(crate) fn new(port: u32, url: &str) -> Result<Self> {
        use std::env;

        let path = match env::var(WEBDRIVER_PATH_ENV) {
            Ok(runtime_env) => runtime_env,
            Err(runtime_env_err) => match option_env!("WEBDRIVER_DLD_PATH") {
                Some(compile_time_path) => compile_time_path.to_string(),
                None => {
                    debug!("{WEBDRIVER_PATH_ENV}: {runtime_env_err}");
                    info!("Use the `download` feature to automatically download, install and use the the chosen WebDriver for supported platforms");
                    info!("Use `{WEBDRIVER_PATH_ENV}` environment variable for applications intended to run on different machines. Manually install the desired WebDriver on the target machine and point {WEBDRIVER_PATH_ENV} to the installation path.");
                    return Err(anyhow!("WebDriver binary not available"));
                }
            },
        };

        let full_path = Self::full_path(&path)
            .with_context(|| format!("Failed tu use WebDriver binary at {path}"))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(WdInner {
                webdriver_port: port,
                webdriver_url: url.to_string(),
                driver_path: full_path,
                webdriver_process_id: None,
            })),
        })
    }

    pub(crate) fn spawn_webdriver(&mut self) {
        info!("Spawning {WEBDRIVER_APP}");
        let local_self = self.inner.clone();

        let _ = thread::spawn(move || {
            let mut inner = match local_self.lock() {
                Ok(inner) => inner,
                Err(e) => {
                    error!("Could not acquire lock: {e}");
                    return;
                }
            };

            let mut command = Command::new(inner.driver_path.clone());
            command
                .arg(format!("--port={}", inner.webdriver_port))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            let mut child = match command.spawn() {
                Ok(c) => {
                    inner.webdriver_process_id = Some(c.id());
                    c
                }
                Err(e) => {
                    error!("Failed to spawn '{WEBDRIVER_APP}': {e}");
                    return;
                }
            };
            drop(inner);

            match child.wait() {
                Ok(c) => {
                    info!("Terminated with ExitStatus: {c}");
                    let stderr = child.stderr.take().unwrap();
                    let stderr_lines = BufReader::new(stderr).lines();
                    for line in stderr_lines {
                        let line = line.unwrap();
                        debug!("{}", line);
                    }
                }
                Err(e) => {
                    error!("Failed waiting on '{WEBDRIVER_APP}': {e}");
                }
            };
        });
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(id) = self.inner.lock().unwrap().webdriver_process_id {
            info!("Stopping '{WEBDRIVER_APP}'");
            let mut kill = Command::new("kill").arg(id.to_string()).spawn()?;
            kill.wait()?;
        }
        Ok(())
    }

    fn full_path(path: &str) -> Result<PathBuf> {
        let mut p = PathBuf::from(path);
        p = Self::os_binary_path(p)?;
        if !p.exists() {
            Err(anyhow!(
                "'{WEBDRIVER_APP}' executable not found in provided path: '{}'",
                p.display()
            ))
        } else {
            Ok(p)
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn os_binary_path(path: PathBuf) -> Result<PathBuf> {
        match path.join(WEBDRIVER_APP).canonicalize() {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!(
                "No {WEBDRIVER_APP} found at '{}': {e}",
                path.display()
            )),
        }
    }

    #[cfg(target_os = "windows")]
    fn os_binary_path(path: PathBuf) -> PathBuf {
        let app = format!("{WEBDRIVER_APP}.exe");
        path.join(app)
    }
}
