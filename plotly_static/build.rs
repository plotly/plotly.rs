use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use tokio::time::sleep;
use webdriver_downloader::prelude::*;

// Enforce that only one driver feature is enabled
#[cfg(all(feature = "geckodriver", feature = "chromedriver"))]
compile_error!("Only one of 'geckodriver' or 'chromedriver' features can be enabled at a time.");

// Enforce that at least one driver feature is enabled
#[cfg(not(any(feature = "geckodriver", feature = "chromedriver")))]
compile_error!("At least one of 'geckodriver' or 'chromedriver' features must be enabled.");

#[cfg(target_os = "windows")]
const DRIVER_EXT: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const DRIVER_EXT: &str = "";

const BROWSER_BIN_PATH_ENV: &str = "BROWSER_PATH";
const WEBDRIVER_BIN_PATH_ENV: &str = "WEBDRIVER_PATH";
const INSTALL_PATH_ENV: &str = "WEBDRIVER_INSTALL_PATH";
const GECKODRIVER_NAME: &str = "geckodriver";
const CHROMEDRIVER_NAME: &str = "chromedriver";

const MAX_DOWNLOAD_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: u64 = 2;

struct WebdriverDownloadConfig {
    driver_name: &'static str,
    get_browser_path: fn() -> Result<PathBuf>,
}

/// Get user's bin directory for driver installs (e.g., $HOME/.local/bin or
/// %USERPROFILE%\.local\bin) or set it to the one specified via the ENV
/// variable `WEBDRIVER_INSTALL_PATH`
fn user_bin_dir() -> PathBuf {
    if let Ok(bin) = env::var(INSTALL_PATH_ENV) {
        return PathBuf::from(bin);
    }
    if let Some(home) = dirs::home_dir() {
        #[cfg(target_os = "windows")]
        {
            return home.join(".local").join("bin");
        }
        #[cfg(not(target_os = "windows"))]
        {
            return home.join(".local").join("bin");
        }
    }
    PathBuf::from(".")
}

/// Check if a driver is already installed at the given path from environment
/// variable
fn is_webdriver_available(bin_name: &str) -> bool {
    // First check environment variable path
    if let Ok(path) = env::var(WEBDRIVER_BIN_PATH_ENV) {
        let bin_path = if cfg!(target_os = "windows") && !path.to_lowercase().ends_with(".exe") {
            format!("{path}{DRIVER_EXT}")
        } else {
            path
        };
        let exe = Path::new(&bin_path);
        if exe.exists() && exe.is_file() {
            println!("{bin_name} found at path specified in {WEBDRIVER_BIN_PATH_ENV}: {bin_path}");
            return true;
        }
    }

    // Check if webdriver exists in user's bin directory
    let bin_dir = user_bin_dir();
    let bin_path = bin_dir.join(format!("{bin_name}{DRIVER_EXT}"));
    if bin_path.exists() && bin_path.is_file() {
        println!(
            "{} found in user's bin directory: {}",
            bin_name,
            bin_path.display()
        );
        println!(
            "cargo:rustc-env=WEBDRIVER_DOWNLOAD_PATH={}",
            bin_path.to_string_lossy()
        );
        return true;
    }

    false
}

async fn download_with_retry(
    driver_info: &impl WebdriverDownloadInfo,
    reinstall: bool,
    skip_verification: bool,
    num_tries: usize,
) -> Result<()> {
    let mut attempts = 0;
    let mut last_error = None;

    while attempts < MAX_DOWNLOAD_RETRIES {
        match download(driver_info, reinstall, skip_verification, num_tries).await {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                last_error = Some(e);
                attempts += 1;
                if attempts < MAX_DOWNLOAD_RETRIES {
                    let delay = Duration::from_secs(INITIAL_RETRY_DELAY * 2u64.pow(attempts - 1));
                    println!(
                        "cargo:warning=Download attempt {attempts} failed, retrying in {delay:?}..."
                    );
                    sleep(delay).await;
                }
            }
        }
    }

    Err(anyhow!(
        "Failed to download driver after {MAX_DOWNLOAD_RETRIES} attempts: {last_error:?}",
    ))
}

fn setup_driver(config: &WebdriverDownloadConfig) -> Result<()> {
    if is_webdriver_available(config.driver_name) {
        return Ok(());
    }
    println!(
        "cargo::warning=You can specify {WEBDRIVER_BIN_PATH_ENV} to an existing {CHROMEDRIVER_NAME}/{GECKODRIVER_NAME} installation to avoid downloads."
    );
    println!(
        "cargo::warning=You can override browser detection using {BROWSER_BIN_PATH_ENV} environment variable."
    );

    println!(
        "cargo::warning={} selected but not installed, will be downloaded ... ",
        config.driver_name
    );

    let browser_path = (config.get_browser_path)()
        .with_context(|| format!("Failed to detect browser path for {}", config.driver_name))?;
    let browser_version = get_browser_version(&browser_path).with_context(|| {
        format!(
            "Failed to get version for browser at {}",
            browser_path.display()
        )
    })?;
    println!("cargo::warning=Browser version detected: {browser_version}");

    let webdriver_bin_dir = user_bin_dir();
    println!("cargo::warning=Driver will be installed in: {webdriver_bin_dir:?}");

    fs::create_dir_all(&webdriver_bin_dir).with_context(|| {
        format!(
            "Failed to create directory: {}",
            webdriver_bin_dir.display()
        )
    })?;
    let webdriver_bin = webdriver_bin_dir.join(config.driver_name);

    println!(
        "cargo::rerun-if-changed={}",
        webdriver_bin.to_string_lossy()
    );

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Failed to create Tokio runtime")?;

    match config.driver_name {
        CHROMEDRIVER_NAME => {
            let driver_info = ChromedriverInfo::new(webdriver_bin.clone(), browser_path);
            runtime
                .block_on(async { download_with_retry(&driver_info, false, true, 1).await })
                .with_context(|| {
                    format!("Failed to download and install {}", config.driver_name)
                })?;
        }
        GECKODRIVER_NAME => {
            let driver_info = GeckodriverInfo::new(webdriver_bin.clone(), browser_path);
            runtime
                .block_on(async { download_with_retry(&driver_info, false, true, 1).await })
                .with_context(|| {
                    format!("Failed to download and install {}", config.driver_name)
                })?;
        }
        _ => return Err(anyhow!("Unsupported driver type: {}", config.driver_name)),
    }

    println!(
        "cargo:rustc-env=WEBDRIVER_DOWNLOAD_PATH={}",
        webdriver_bin.to_string_lossy()
    );

    Ok(())
}

#[cfg(feature = "chromedriver")]
fn get_chrome_path() -> Result<PathBuf> {
    if let Ok(chrome_path) = env::var(BROWSER_BIN_PATH_ENV) {
        let path = PathBuf::from(&chrome_path);
        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!("Chrome not found on path: {chrome_path}")).with_context(|| {
                format!("Please set {BROWSER_BIN_PATH_ENV} to a valid Chrome installation")
            })
        }
    } else {
        let new_browser_path = os_specific::chromedriver_for_testing::default_browser_path()?;
        let old_browser_path = os_specific::chromedriver_old::default_browser_path()?;
        if new_browser_path.exists() {
            Ok(new_browser_path)
        } else if old_browser_path.exists() {
            Ok(old_browser_path)
        } else {
            Err(anyhow!("Chrome browser not detected")).with_context(|| {
                format!("Use {BROWSER_BIN_PATH_ENV} to point to a valid Chrome installation")
            })
        }
    }
}

#[cfg(feature = "geckodriver")]
fn get_firefox_path() -> Result<PathBuf> {
    if let Ok(firefox_path) = env::var(BROWSER_BIN_PATH_ENV) {
        let path = PathBuf::from(firefox_path.clone());
        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!("Firefox not found on path: {firefox_path}")).with_context(|| {
                format!("Please set {BROWSER_BIN_PATH_ENV} to a valid Firefox installation",)
            })
        }
    } else {
        let browser_path = os_specific::geckodriver::default_browser_path()?;
        if browser_path.exists() {
            Ok(browser_path)
        } else {
            Err(anyhow!("Firefox browser not detected")).with_context(|| {
                format!("Use {BROWSER_BIN_PATH_ENV} to point to a valid Firefox installation",)
            })
        }
    }
}

fn get_browser_version(path: &PathBuf) -> Result<String> {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .with_context(|| format!("Failed to execute browser at {}", path.display()))?;
    let out_str = String::from_utf8_lossy(&output.stdout);
    out_str
        .split_whitespace()
        .find(|s| s.chars().next().unwrap_or(' ').is_ascii_digit())
        .map(|v| v.to_string())
        .ok_or(anyhow!(
            "Failed to get browser version for browser: {}",
            path.display()
        ))
        .with_context(|| {
            format!(
                "Browser at {} did not return a valid version string",
                path.display()
            )
        })
}

async fn download(
    driver_info: &impl WebdriverDownloadInfo,
    reinstall: bool,
    skip_verification: bool,
    num_tries: usize,
) -> Result<(), WebdriverDownloadError> {
    if !reinstall && driver_info.is_installed().await {
        println!("cargo::warning=Driver already installed ...");
        Ok(())
    } else {
        if skip_verification {
            driver_info.download_install().await?;
        } else {
            driver_info.download_verify_install(num_tries).await?;
        }

        println!("cargo::warning=Driver installed successfully ...");
        Ok(())
    }
}

fn main() -> Result<()> {
    if cfg!(feature = "webdriver_download") {
        println!("cargo:rerun-if-changed=src/lib.rs");
        let webdriver_bin_dir = user_bin_dir();
        println!(
            "cargo::rerun-if-changed={}",
            webdriver_bin_dir.to_string_lossy()
        );

        #[cfg(feature = "chromedriver")]
        {
            let config = WebdriverDownloadConfig {
                driver_name: CHROMEDRIVER_NAME,
                get_browser_path: get_chrome_path,
            };
            setup_driver(&config)?;
        }

        #[cfg(feature = "geckodriver")]
        {
            let config = WebdriverDownloadConfig {
                driver_name: GECKODRIVER_NAME,
                get_browser_path: get_firefox_path,
            };
            setup_driver(&config)?;
        }

        #[cfg(not(any(feature = "chromedriver", feature = "geckodriver")))]
        {
            println!("cargo::warning=No specific driver feature enabled, skipping driver setup");
        }
    } else {
        #[cfg(feature = "chromedriver")]
        {
            let msg = format!("'webdriver_download' feature disabled. Please install a '{CHROMEDRIVER_NAME}' version manually and make the environment variable 'WEBDRIVER_PATH' point to it.");
            println!("cargo::warning={msg}");
        }
        #[cfg(feature = "geckodriver")]
        {
            let msg = format!("'webdriver_download' feature disabled. Please install a '{GECKODRIVER_NAME}' version manually and make the environment variable 'WEBDRIVER_PATH' point to it.");
            println!("cargo::warning={msg}");
        }
        #[cfg(not(any(feature = "chromedriver", feature = "geckodriver")))]
        {
            println!("cargo::warning='webdriver_download' feature disabled and no driver feature enabled");
        }
    }
    Ok(())
}
