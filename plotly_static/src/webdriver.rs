//! WebDriver management module for plotly_static.
//!
//! This module provides WebDriver process management, including:
//! - Automatic detection of existing WebDriver sessions
//! - Process spawning and lifecycle management
//! - Connection reuse and cleanup
//! - Support for both spawned and external WebDriver instances
//!
//! The module is designed to work safely in parallel environments.

#[cfg(any(feature = "chromedriver", feature = "geckodriver"))]
mod launch;

use std::path::PathBuf;
use std::process::Child;
use std::sync::{Arc, Mutex};
#[cfg(any(test, feature = "debug"))]
use std::{println as info, println as warn};

use anyhow::{anyhow, Result};
#[cfg(not(any(test, feature = "debug")))]
use log::{info, warn};

/// Default WebDriver port
pub(crate) const WEBDRIVER_PORT: u32 = 4444;
/// Default WebDriver URL
pub(crate) const WEBDRIVER_URL: &str = "http://127.0.0.1";

#[cfg(all(feature = "chromedriver", not(target_os = "windows")))]
pub(crate) fn chrome_default_caps() -> Vec<&'static str> {
    vec![
        "--headless",
        "--no-sandbox",
        "--disable-gpu-sandbox",
        "--disable-dev-shm-usage",
        "--disable-extensions",
        "--disable-background-networking",
        "--disable-sync",
        "--disable-translate",
        "--disable-background-timer-throttling",
        "--disable-renderer-backgrounding",
        "--disable-features=VizDisplayCompositor",
        "--memory-pressure-off",
        // macOS-specific flags from choreographer
        "--enable-unsafe-swiftshader",
        "--use-mock-keychain",
        "--password-store=basic",
        "--disable-web-security",
        "--disable-breakpad",
        "--no-first-run",
        "--no-default-browser-check",
        // Additional flags for better PDF generation support
        "--disable-backgrounding-occluded-windows",
        "--disable-ipc-flooding-protection",
        "--enable-logging",
        "--v=1",
    ]
}

#[cfg(all(feature = "chromedriver", target_os = "windows"))]
pub(crate) fn chrome_default_caps() -> Vec<&'static str> {
    vec![
        "--headless=new",
        "--no-sandbox",
        "--disable-dev-shm-usage",
        "--disable-breakpad",
        "--no-first-run",
        "--no-default-browser-check",
        // Stability flags to prevent renderer crashes
        "--disable-background-networking",
        "--disable-sync",
        "--disable-translate",
        "--disable-background-timer-throttling",
        "--disable-renderer-backgrounding",
        "--disable-backgrounding-occluded-windows",
        "--disable-ipc-flooding-protection",
        "--disable-extensions",
        // Minimal flags for Windows headless operation without disabling GPU
        "--hide-scrollbars",
        "--mute-audio",
        "--use-angle=swiftshader",
        "--disable-software-rasterizer",
    ]
}

#[cfg(feature = "geckodriver")]
pub(crate) fn firefox_default_caps() -> Vec<&'static str> {
    vec![
        "-headless",   // Essential for headless operation (single dash for Firefox)
        "--no-remote", // Prevents connecting to existing Firefox instances
    ]
}

/// Internal WebDriver state
#[derive(Debug)]
struct WdInner {
    webdriver_port: u32,
    driver_path: Option<PathBuf>,
    webdriver_child: Option<Child>,
    is_external: bool, /* Marker for whether this WebDriver was spawned by us or connected to
                        * existing */
}

/// WebDriver management struct
///
/// This struct provides WebDriver process management with the following
/// features:
/// - Automatic detection of existing WebDriver sessions
/// - Process spawning and lifecycle management
/// - Connection reuse and cleanup
/// - Support for both spawned and external WebDriver instances
/// - Thread-safe operations using Arc<Mutex<>> for internal state
#[derive(Debug)]
pub struct WebDriver {
    inner: Arc<Mutex<WdInner>>,
}

impl WebDriver {
    /// Stops the WebDriver process.
    ///
    /// This method manages WebDriver process termination:
    /// - Only terminates processes that were spawned by this instance
    /// - Leaves externally managed WebDriver sessions running
    /// - Logs warnings when attempting to stop external sessions
    ///
    /// Returns `Ok(())` on success, or an error if the process termination
    /// fails.
    pub fn stop(&mut self) -> Result<()> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|e| anyhow!("Failed to acquire lock: {}", e))?;

        // Only stop the process if we spawned it (not if it's external)
        if !inner.is_external {
            if let Some(child) = inner.webdriver_child.as_mut() {
                info!("Stopping WebDriver (PID: {})", child.id());
                let _ = child.kill();
                let _ = child.wait();
            }
        } else {
            warn!(
                "Not stopping external WebDriver on port {} as it was not spawned by us",
                inner.webdriver_port
            );
        }
        Ok(())
    }

    /// Get diagnostic information about the WebDriver process.
    ///
    /// This method provides detailed information about the WebDriver process
    /// for debugging purposes.
    ///
    /// Returns a string with diagnostic information.
    pub(crate) fn get_diagnostics(&self) -> String {
        let mut inner = self.inner.lock().unwrap();
        let mut diagnostics = String::new();

        diagnostics.push_str("WebDriver Diagnostics:\n");
        diagnostics.push_str(&format!("  Port: {}\n", inner.webdriver_port));
        diagnostics.push_str(&format!("  Driver Path: {:?}\n", inner.driver_path));
        diagnostics.push_str(&format!("  Is External: {}\n", inner.is_external));

        if let Some(child) = inner.webdriver_child.as_mut() {
            diagnostics.push_str(&format!("  Process ID: {}\n", child.id()));

            // Check if process is still running
            match child.try_wait() {
                Ok(None) => diagnostics.push_str("  Process Status: Running\n"),
                Ok(Some(status)) => {
                    diagnostics.push_str(&format!("  Process Status: Exited with {status:?}\n"))
                }
                Err(e) => {
                    diagnostics.push_str(&format!("  Process Status: Error checking status: {e}\n"))
                }
            }
        } else {
            diagnostics.push_str("  Process ID: None (no child process)\n");
        }

        // Check if WebDriver is responding
        let is_running = Self::is_webdriver_running(inner.webdriver_port);
        diagnostics.push_str(&format!("  WebDriver Responding: {is_running}\n"));

        // Check port availability
        let url = format!("{WEBDRIVER_URL}:{}/status", inner.webdriver_port);
        diagnostics.push_str(&format!("  Status URL: {url}\n"));

        #[cfg(target_os = "windows")]
        {
            diagnostics.push_str("  Platform: Windows\n");
            // Check if port is in use using Windows-specific commands
            if let Ok(output) = std::process::Command::new("netstat").args(["-an"]).output() {
                let netstat_output = String::from_utf8_lossy(&output.stdout);
                if netstat_output.contains(&format!(":{}", inner.webdriver_port)) {
                    diagnostics.push_str(&format!(
                        "  Port {} appears to be in use (netstat)\n",
                        inner.webdriver_port
                    ));
                } else {
                    diagnostics.push_str(&format!(
                        "  Port {} appears to be free (netstat)\n",
                        inner.webdriver_port
                    ));
                }
            }

            // Check if chromedriver is in PATH (Windows-specific)
            #[cfg(all(target_os = "windows", feature = "chromedriver"))]
            {
                if let Ok(output) = std::process::Command::new("where")
                    .arg("chromedriver")
                    .output()
                {
                    let where_output = String::from_utf8_lossy(&output.stdout);
                    diagnostics
                        .push_str(&format!("  Chromedriver in PATH: {}", where_output.trim()));
                } else {
                    diagnostics.push_str("  Chromedriver not found in PATH\n");
                }
            }

            // Check if geckodriver is in PATH (Windows-specific)
            #[cfg(all(target_os = "windows", feature = "geckodriver"))]
            {
                if let Ok(output) = std::process::Command::new("where")
                    .arg("geckodriver")
                    .output()
                {
                    let where_output = String::from_utf8_lossy(&output.stdout);
                    diagnostics
                        .push_str(&format!("  Geckodriver in PATH: {}", where_output.trim()));
                } else {
                    diagnostics.push_str("  Geckodriver not found in PATH\n");
                }
            }

            // Check Windows Defender status (Windows-specific)
            #[cfg(target_os = "windows")]
            {
                if let Ok(output) = std::process::Command::new("powershell")
                    .args([
                        "-Command",
                        "Get-MpComputerStatus | Select-Object RealTimeProtectionEnabled",
                    ])
                    .output()
                {
                    let defender_output = String::from_utf8_lossy(&output.stdout);
                    diagnostics.push_str(&format!(
                        "  Windows Defender Real-time Protection: {}",
                        defender_output.trim()
                    ));
                }
            }
        }

        diagnostics
    }

    /// Check if a WebDriver is already running on the specified port.
    ///
    /// This method performs a WebDriver check by:
    /// 1. Making an HTTP GET request to `/status` endpoint
    /// 2. Checking for HTTP 200 response
    /// 3. Verifying the response contains "ready" indicating the service is
    ///    ready
    ///
    /// Returns `true` if WebDriver is running and ready, `false` otherwise.
    pub(crate) fn is_webdriver_running(port: u32) -> bool {
        let url = format!("{WEBDRIVER_URL}:{port}/status");

        // Add timeout to prevent hanging
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::blocking::Client::new());

        client
            .get(&url)
            .send()
            .ok()
            .filter(|response| response.status().as_u16() == 200)
            .and_then(|response| response.text().ok())
            .map(|text| text.contains("ready"))
            .unwrap_or(false)
    }
}
