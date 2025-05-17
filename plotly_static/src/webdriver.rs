use anyhow::Context;
use anyhow::{anyhow, Result};
#[cfg(not(test))]
use log::{debug, error, info, warn};
use plotly::ImageFormat;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::runtime::Runtime;

#[cfg(test)]
use std::{println as info, println as warn, println as error, println as debug};

const WEBDRIVER_PATH_ENV: &str = "WEBDRIVER_PATH";

#[cfg(feature = "geckodriver")]
const WEBDRIVER_APP: &str = "geckodriver";
#[cfg(feature = "geckodriver")]
const DRIVER_ARGS: &str =
    r#"{"browserName":"firefox","moz:firefoxOptions":{"args":["--headless", "--disable-gpu"]}}"#;

#[cfg(feature = "chromedriver")]
const WEBDRIVER_APP: &str = "chromedriver";
#[cfg(feature = "chromedriver")]
const DRIVER_ARGS: &str =
    r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless", "--disable-gpu"]}}"#;

const WEBDRIVER_PORT: u32 = 4444;
const WEBDRIVER_URL: &str = "http://localhost";

pub(crate) struct StaticExporterBuilder {
    port: u32,
    url: String,
    launch_webdriver: bool,
}

impl StaticExporterBuilder {
    pub fn new() -> Self {
        Self {
            port: WEBDRIVER_PORT,
            url: WEBDRIVER_URL.to_string(),
            launch_webdriver: true,
        }
    }

    pub fn webdriver_port(mut self, port: u32) -> Self {
        self.port = port;
        self
    }

    pub fn launch_webdriver(mut self, yes: bool) -> Self {
        self.launch_webdriver = yes;
        self
    }

    pub fn webdriver_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn build(&self) -> Result<StaticExporter> {
        let mut exp = StaticExporter::new()?;
        if self.launch_webdriver {
            exp.spawn_instance();
        }
        Ok(exp)
    }
}

#[derive(Debug)]
struct ExporterInner {
    driver_path: PathBuf,
    process_id: Option<u32>,
}

pub(crate) struct StaticExporter {
    inner: Arc<Mutex<ExporterInner>>,
}

impl StaticExporter {
    fn new() -> Result<Self> {
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
            inner: Arc::new(Mutex::new(ExporterInner {
                driver_path: full_path, // PathBuf::from(path),
                process_id: None,
            })),
        })
    }

    fn spawn_instance(&mut self) {
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
                .arg(format!("--port={WEBDRIVER_PORT}"))
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
        if let Some(id) = self.inner.lock().unwrap().process_id {
            info!("Stopping '{WEBDRIVER_APP}'");
            let mut kill = Command::new("kill").arg(id.to_string()).spawn()?;
            kill.wait()?;
        }
        Ok(())
    }

    pub fn static_export(&self, file: &PathBuf, format: &ImageFormat) -> Result<String> {
        Runtime::new()?
            .block_on(Self::extract(&file, format))
            .with_context(|| "Failed to extract static image from browser session")
    }

    pub async fn extract(file_path: &PathBuf, format: &ImageFormat) -> Result<String> {
        use fantoccini::{wd::Capabilities, ClientBuilder, Locator};
        use std::time::Duration;
        use tokio::time::sleep;

        let cap: Capabilities = serde_json::from_str(DRIVER_ARGS)?;
        let webdriver_url = format!("{WEBDRIVER_URL}:{WEBDRIVER_PORT}");

        let client = ClientBuilder::native()
            .capabilities(cap)
            .connect(&webdriver_url)
            .await
            .with_context(|| "WebDriver session errror")?;

        // client.persist().await?;
        // dbg!(client.session_id().await?);

        // Open generate static plotly html file
        let url = format!("file:{}", file_path.display());
        client.goto(&url).await?;

        // Find the location where the plotly static image is stored by XPath of the StaticTemplate
        let img = client.find(Locator::XPath(r#"/html/body/div/img"#)).await?;
        let src = loop {
            let src = img.attr("src").await?;
            if src.is_none() {
                info!("Waiting 50 msec for PlotlyJS valid image data in 'src' attribute");
                sleep(Duration::from_millis(50)).await;
            } else {
                break src.unwrap();
            }
        };
        // client.close_window().await?;
        client.close().await?;

        match format {
            ImageFormat::SVG => Self::extract_plain(&src, format),
            ImageFormat::PNG | ImageFormat::JPEG => Self::extract_encoded(&src, format),
            _ => return Err(anyhow!("Not implemented for {format}")),
        }
    }

    fn extract_plain(payload: &str, format: &ImageFormat) -> Result<String> {
        use percent_encoding::percent_decode;

        match payload.split_once(",") {
            Some((type_info, data)) => {
                Self::extract_type_info(type_info, format);
                let decoded = percent_decode(data.as_bytes()).decode_utf8()?;
                Ok(decoded.to_string())
            }
            None => Err(anyhow!("'src' attribute has invalid {format} data")),
        }
    }

    fn extract_encoded(payload: &str, format: &ImageFormat) -> Result<String> {
        match payload.split_once(";") {
            Some((type_info, encoded_data)) => {
                Self::extract_type_info(type_info, format);
                Self::extract_encoded_data(encoded_data)
                    .ok_or(anyhow!("No valid image data found in 'src' attribute"))
            }
            None => Err(anyhow!("'src' attribute has invalid base64 data")),
        }
    }

    fn extract_type_info(type_info: &str, format: &ImageFormat) {
        let val = type_info.split_once("/").map(|d| d.1.to_string());
        match val {
            Some(ext) => {
                if !ext.contains(&format.to_string()) {
                    error!("Requested ImageFormat '{}', got '{}'", format, ext);
                }
            }
            None => warn!("Failed to extract static Image Format from 'src' attribute"),
        }
    }

    fn extract_encoded_data(data: &str) -> Option<String> {
        data.split_once(",").map(|d| d.1.to_string())
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

impl Drop for StaticExporter {
    fn drop(&mut self) {
        if let Err(e) = self.stop() {
            error!("Failed to release WebDriver process: {e}");
        }
    }
}
