use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use anyhow::{anyhow, Context, Result};
use tokio::time::sleep;
use webdriver_downloader::prelude::*;

#[cfg(target_os = "windows")]
const DRIVER_EXT: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const DRIVER_EXT: &str = "";

#[cfg(feature = "geckodriver")]
const WEBDRIVER_BIN: &str = "geckodriver";

#[cfg(feature = "chromedriver")]
const WEBDRIVER_BIN: &str = "chromedriver";

const CHROME_PATH_ENV: &str = "CHROME_PATH";
const CHROMEDRIVER_PATH_ENV: &str = "CHROMEDRIVER_PATH";
const FIREFOX_PATH_ENV: &str = "FIREFOX_PATH";
const GECKODRIVER_PATH_ENV: &str = "GECKODRIVER_PATH";
const INSTALL_PATH_ENV: &str = "INSTALL_BIN_PATH";

const MAX_DOWNLOAD_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: u64 = 2;

struct WebdriverDownloadConfig {
    driver_name: &'static str,
    path_env: &'static str,
    get_browser_path: fn() -> Result<PathBuf>,
}

/// Get user's bin directory for driver installs (e.g., $HOME/.local/bin or
/// %USERPROFILE%\.local\bin) or set it to the one specified via the ENV
/// variable `INSTALL_BIN_PATH`
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
fn is_webdriver_available(env_var: &str, executable_name: &str) -> bool {
    // First check environment variable path
    if let Ok(path) = env::var(env_var) {
        let exe_path = if cfg!(windows) && !path.to_lowercase().ends_with(".exe") {
            format!("{}{}", path, DRIVER_EXT)
        } else {
            path
        };
        let exe = Path::new(&exe_path);
        if exe.exists() && exe.is_file() {
            println!(
                "{} found at path specified in {}: {}",
                executable_name, env_var, exe_path
            );
            return true;
        }
    }

    // Check if webdriver exists in user's bin directory
    let bin_dir = user_bin_dir();
    let bin_path = bin_dir.join(format!("{}{}", executable_name, DRIVER_EXT));
    if bin_path.exists() && bin_path.is_file() {
        println!(
            "{} found in user's bin directory: {}",
            executable_name,
            bin_path.display()
        );
        println!(
            "cargo:rustc-env=WEBDRIVER_DOWNLOAD_PATH={}",
            bin_dir.to_string_lossy()
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
                    let delay =
                        Duration::from_secs(INITIAL_RETRY_DELAY * 2u64.pow(attempts as u32 - 1));
                    println!(
                        "cargo:warning=Download attempt {} failed, retrying in {:?}...",
                        attempts, delay
                    );
                    sleep(delay).await;
                }
            }
        }
    }

    Err(anyhow!(
        "Failed to download driver after {} attempts: {:?}",
        MAX_DOWNLOAD_RETRIES,
        last_error
    ))
}

fn setup_driver(config: &WebdriverDownloadConfig) -> Result<()> {
    if is_webdriver_available(config.path_env, config.driver_name) {
        return Ok(());
    }
    println!(
        "cargo::warning={}",
        format!(
            "You can specify {} or {} to an existing installation to avoid downloads.",
            GECKODRIVER_PATH_ENV, CHROMEDRIVER_PATH_ENV
        )
    );
    println!(
        "cargo::warning={}",
        format!(
            "You can override browser detection using {} or {} environment variables.",
            CHROME_PATH_ENV, FIREFOX_PATH_ENV
        )
    );

    println!(
        "cargo::warning={}",
        format!(
            "{} selected but not installed, will be downloaded ... ",
            config.driver_name
        )
    );

    let browser_path = (config.get_browser_path)()
        .with_context(|| format!("Failed to detect browser path for {}", config.driver_name))?;
    let browser_version = get_browser_version(&browser_path)
        .with_context(|| format!("Failed to get version for browser at {}", browser_path.display()))?;
    println!(
        "cargo::warning=Browser version detected: {}",
        browser_version
    );

    let webdriver_bin_dir = user_bin_dir();
    println!(
        "cargo::warning=Driver will be installed in: {:?}",
        webdriver_bin_dir
    );

    fs::create_dir_all(&webdriver_bin_dir)
        .with_context(|| format!("Failed to create directory: {}", webdriver_bin_dir.display()))?;
    let webdriver_bin = webdriver_bin_dir.join(WEBDRIVER_BIN);

    println!(
        "cargo::rerun-if-changed={}",
        webdriver_bin.to_string_lossy()
    );

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Failed to create Tokio runtime")?;

    match config.driver_name {
        "chromedriver" => {
            let driver_info = ChromedriverInfo::new(webdriver_bin.clone(), browser_path);
            runtime.block_on(async { download_with_retry(&driver_info, false, true, 1).await })
                .with_context(|| format!("Failed to download and install {}", config.driver_name))?;
        }
        "geckodriver" => {
            let driver_info = GeckodriverInfo::new(webdriver_bin.clone(), browser_path);
            runtime.block_on(async { download_with_retry(&driver_info, false, true, 1).await })
                .with_context(|| format!("Failed to download and install {}", config.driver_name))?;
        }
        _ => return Err(anyhow!("Unsupported driver type: {}", config.driver_name)),
    }

    println!(
        "cargo:rustc-env=WEBDRIVER_DOWNLOAD_PATH={}",
        webdriver_bin_dir.to_string_lossy()
    );

    Ok(())
}

#[cfg(feature = "geckodriver")]
fn get_firefox_path() -> Result<PathBuf> {
    if let Ok(firefox_path) = env::var(FIREFOX_PATH_ENV) {
        let path = PathBuf::from(firefox_path);
        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!("Firefox not found on path: {firefox_path}"))
                .with_context(|| format!("Please set {} to a valid Firefox installation", FIREFOX_PATH_ENV))
        }
    } else {
        let browser_path = os_specific::geckodriver::default_browser_path()?;
        if browser_path.exists() {
            Ok(browser_path)
        } else {
            Err(anyhow!("Firefox browser not detected"))
                .with_context(|| format!("Use {} to point to a valid Firefox installation", FIREFOX_PATH_ENV))
        }
    }
}

#[cfg(feature = "chromedriver")]
fn get_chrome_path() -> Result<PathBuf> {
    if let Ok(chrome_path) = env::var(CHROME_PATH_ENV) {
        let path = PathBuf::from(&chrome_path);
        if path.exists() {
            Ok(path)
        } else {
            Err(anyhow!("Chrome not found on path: {}", chrome_path))
                .with_context(|| format!("Please set {} to a valid Chrome installation", CHROME_PATH_ENV))
        }
    } else {
        let new_browser_path = os_specific::chromedriver_for_testing::default_browser_path()?;
        let old_browser_path = os_specific::chromedriver_old::default_browser_path()?;
        if new_browser_path.exists() {
            Ok(new_browser_path)
        } else if old_browser_path.exists() {
            Ok(old_browser_path)
        } else {
            Err(anyhow!("Chrome browser not detected"))
                .with_context(|| format!("Use {} to point to a valid Chrome installation", CHROME_PATH_ENV))
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
        .find(|s| s.chars().next().unwrap_or(' ').is_digit(10))
        .map(|v| v.to_string())
        .ok_or(anyhow!(
            "Failed to get browser version for browser: {}",
            path.display()
        ))
        .with_context(|| format!("Browser at {} did not return a valid version string", path.display()))
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

        println!("cargo::warning=Driver installed succesfully ...");
        Ok(())
    }
}

fn main() -> Result<()> {
    if cfg!(feature = "download") {
        println!("cargo:rerun-if-changed=src/lib.rs");
        let webdriver_bin_dir = user_bin_dir();
        let webdriver_bin = webdriver_bin_dir.join(WEBDRIVER_BIN);
        println!(
            "cargo::rerun-if-changed={}",
            webdriver_bin.to_string_lossy()
        );


        #[cfg(feature = "chromedriver")]
        {
            let config = WebdriverDownloadConfig {
                driver_name: WEBDRIVER_BIN,
                path_env: CHROMEDRIVER_PATH_ENV,
                get_browser_path: get_chrome_path,
            };
            setup_driver(&config)?;
        }

        #[cfg(feature = "geckodriver")]
        {
            let config = WebdriverDownloadConfig {
                driver_name: WEBDRIVER_BIN,
                path_env: GECKODRIVER_PATH_ENV,
                get_browser_path: get_firefox_path,
            };
            setup_driver(&config)?;
        }
    } else {
        let msg = format!("'download' feature disabled. Please install a {} or {} version manually and make the environment variable 'WEBDRIVER_PATH' point to it.",
            GECKODRIVER_PATH_ENV, CHROMEDRIVER_PATH_ENV);
        println!("cargo::warning={msg}");
    }
    Ok(())
}
