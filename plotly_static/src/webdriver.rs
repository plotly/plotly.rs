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

const CHROME_DRIVER_ARGS: &str =
    r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless", "--disable-gpu"]}}"#;

const FIREFOX_DRIVER_ARGS: &str =
    r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless", "--disable-gpu"]}}"#;

const WEBDRIVER_PATH_ENV: &str = "WEBDRIVER_PATH";

#[cfg(feature = "geckodriver")]
const WEBDRIVER_APP: &str = "geckodriver";

#[cfg(feature = "chromedriver")]
const WEBDRIVER_APP: &str = "chromedriver";

struct InstanceInner {
    driver_path: PathBuf,
    process_id: Option<u32>,
}

pub(crate) struct Instance {
    inner: Arc<Mutex<InstanceInner>>,
}

impl Instance {
    pub fn new() -> Result<Self> {
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

        Self::validate_path(&path)
            .with_context(|| format!("Failed tu use WebDriver binary at {path}"))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(InstanceInner {
                driver_path: PathBuf::from(path),
                process_id: None,
            })),
        })
    }

    pub fn start(&mut self) {
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

            let mut command = Command::new(WEBDRIVER_APP);
            command
                .current_dir(PathBuf::from(inner.driver_path.clone()))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            let mut child = match command.spawn() {
                Ok(c) => {
                    inner.process_id = Some(c.id());
                    c
                }
                Err(e) => {
                    error!("Failed to spawn '{WEBDRIVER_APP}': {e}");
                    return;
                }
            };
            drop(inner);

            match child.wait() {
                Ok(_) => {
                    let stderr = child.stderr.take().unwrap();
                    let stderr_lines = BufReader::new(stderr).lines();
                    for line in stderr_lines {
                        let line = line.unwrap();
                        debug!("{}", line);
                    }
                }
                Err(e) => {
                    error!("Failed to spawn '{WEBDRIVER_APP}': {e}");
                }
            };
        });
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(id) = self.inner.lock().unwrap().process_id {
            info!("Stopping '{WEBDRIVER_APP}'");
            let mut kill = Command::new("kill").arg(id.to_string()).spawn()?;
            kill.wait()?;
        }
        Ok(())
    }

    fn validate_path(dld_path: &str) -> Result<()> {
        let mut p = PathBuf::from(dld_path);
        p = Self::os_binary_path(p)?;
        if !p.exists() {
            Err(anyhow!(
                "'{WEBDRIVER_APP}' executable not found in provided path: '{}'",
                p.display()
            ))
        } else {
            Ok(())
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
