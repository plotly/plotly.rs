use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use webdriver_downloader::prelude::*;

#[cfg(target_os = "windows")]
const CHROMEDRIVER_EXT: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const CHROMEDRIVER_EXT: &str = "";

#[cfg(feature = "geckodriver")]
const WEBDRIVER_BIN: &str = "geckodriver";

#[cfg(feature = "chromedriver")]
const WEBDRIVER_BIN: &str = "chromedriver";

const CHROME_PATH_ENV: &str = "CHROME_PATH";
const CHROMEDRIVER_PATH_ENV: &str = "CHROMEDRIVER_PATH";

const FIREFOX_PATH_ENV: &str = "FIREFOX_PATH";
const GECKODRIVER_PATH_ENV: &str = "GECKODRIVER_PATH";

const INSTALL_PATH_ENV: &str = "INSTALL_BIN_PATH";

/// Get user's bin directory for driver installs (e.g., $HOME/.local/bin or %USERPROFILE%\.local\bin)
/// or set it to the one specified via the ENV variable `INSTALL_BIN_PATH`
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

/// Check if a driver is already installed at the given path from environment variable
fn is_webdriver_path_set(env_var: &str, executable_name: &str) -> bool {
    if let Ok(path) = env::var(env_var) {
        let exe_path = if cfg!(windows) && !path.to_lowercase().ends_with(".exe") {
            format!("{}{}", path, CHROMEDRIVER_EXT)
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
    false
}

fn main() -> Result<()> {
    if cfg!(feature = "download") {
        println!(
            "cargo::warning={}",
            "You can specify GECKODRIVER_PATH or CHROMEDRIVER_PATH to an existing installation to avoid downloads."
        );
        println!(
        "cargo::warning={}",
        "You can override browser detection using CHROME_PATH or FIREFOX_PATH environment variables."
    );

        println!("cargo:rerun-if-changed=src/lib.rs");

        if cfg!(feature = "chromedriver") {
            if is_webdriver_path_set(CHROMEDRIVER_PATH_ENV, "chromedriver") {
                return Ok(());
            } else {
                println!(
                    "cargo::warning={}",
                    "CHROMEDRIVER selected but not installed, will be downloaded ... "
                );

                let chrome_path = get_chrome_path()?;
                let chrome_version = get_browser_version(&chrome_path)?;
                println!("cargo::warning=Chrome version detected: {}", chrome_version);

                println!(
                    "cargo::warning=Driver will be installed in: {:?}",
                    user_bin_dir()
                );

                let webdriver_bin_dir = user_bin_dir();
                fs::create_dir_all(&webdriver_bin_dir)?;
                let webdriver_bin = webdriver_bin_dir.join(WEBDRIVER_BIN);

                println!(
                    "cargo::rerun-if-changed={}",
                    webdriver_bin.to_string_lossy()
                );

                let driver_info = ChromedriverInfo::new(webdriver_bin.clone(), chrome_path);

                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime");
                runtime
                    .block_on(async { download(&driver_info, false, true, 1).await })
                    .unwrap();

                println!(
                    "cargo:rustc-env=WEBDRIVER_DLD_PATH={}",
                    webdriver_bin_dir.to_string_lossy()
                );
            }
        }
        if cfg!(feature = "geckodriver") {
            if is_webdriver_path_set(GECKODRIVER_PATH_ENV, "geckodriver") {
                return Ok(());
            } else {
                println!(
                    "cargo::warning={}",
                    "GECKODRIVER selected but not installed, will be downloaded ... "
                );
                let firefox_path = get_firefox_path()?;
                let firefox_version = get_browser_version(&firefox_path)?;
                println!(
                    "cargo::warning=Firefox version detected: {}",
                    firefox_version
                );

                println!(
                    "cargo::warning=Driver will be installed in: {:?}",
                    user_bin_dir()
                );

                let webdriver_bin_dir = user_bin_dir();
                fs::create_dir_all(&webdriver_bin_dir)?;
                let webdriver_bin = webdriver_bin_dir.join(WEBDRIVER_BIN);

                println!(
                    "cargo::rerun-if-changed={}",
                    webdriver_bin.to_string_lossy()
                );

                let driver_info = GeckodriverInfo::new(
                    webdriver_bin.clone(),
                    os_specific::geckodriver::default_browser_path()?,
                );

                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create Tokio runtime");
                runtime
                    .block_on(async { download(&driver_info, false, true, 1).await })
                    .unwrap();

                println!(
                    "cargo:rustc-env=WEBDRIVER_DLD_PATH={}",
                    webdriver_bin_dir.to_string_lossy()
                );
            }
        }
    } else {
        let msg = "'download' feature disabled. Please install a GECKODRIVER or CHROMEDRIVER version manually and make the environment variable 'WEBDRIVER_PATH' point to it.".to_string();
        println!("cargo::warning={msg}");
    }
    Ok(())
}

fn get_firefox_path() -> Result<PathBuf> {
    if let Ok(firefox_path) = env::var(FIREFOX_PATH_ENV) {
        if Path::new(&firefox_path).exists() {
            return Ok(PathBuf::from(firefox_path));
        } else {
            Err(anyhow!("Firefox not found on path: {firefox_path}"))
        }
    } else {
        let browser_path = os_specific::geckodriver::default_browser_path()?;
        if browser_path.exists() {
            Ok(browser_path)
        } else {
            Err(anyhow!("Firefox browser not detected. Use {FIREFOX_PATH_ENV} to point to a valid Firefox installation ..."))
        }
    }
}

fn get_chrome_path() -> Result<PathBuf> {
    if let Ok(chrome_path) = env::var(CHROME_PATH_ENV) {
        if Path::new(&chrome_path).exists() {
            return Ok(PathBuf::from(chrome_path));
        } else {
            Err(anyhow!("Chrome not found on path: {chrome_path}"))
        }
    } else {
        let new_browser_path = os_specific::chromedriver_for_testing::default_browser_path()?;
        let old_browser_path = os_specific::chromedriver_old::default_browser_path()?;
        if new_browser_path.exists() {
            Ok(new_browser_path)
        } else if old_browser_path.exists() {
            Ok(old_browser_path)
        } else {
            Err(anyhow!("Chrome browser not detected. Use {CHROME_PATH_ENV} to point to a valid Chrome installation ..."))
        }
    }
}

fn get_browser_version(path: &PathBuf) -> Result<String> {
    let output = Command::new(path).arg("--version").output()?;
    let out_str = String::from_utf8_lossy(&output.stdout);
    out_str
        .split_whitespace()
        .find(|s| s.chars().next().unwrap_or(' ').is_digit(10))
        .map(|v| v.to_string())
        .ok_or(anyhow!(
            "Failed to get browser version for browser: {}",
            path.display()
        ))
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
