//! # Plotly Static Image Export
//!
//! A Rust library for exporting Plotly plots to static images using headless
//! browsers via WebDriver.
//!
//! This library provides a interface for converting Plotly plots provided as
//! JSON values into various static image formats (PNG, JPEG, WEBP, SVG,
//! PDF) using WebDriver and headless browsers.
//!
//! ## Features
//!
//! - **Multiple Formats**: Support for PNG, JPEG, WEBP, SVG, and PDF export
//! - **Headless Rendering**: Uses headless browsers for rendering
//! - **WebDriver Support**: Supports both Chrome (chromedriver) and Firefox
//!   (geckodriver)
//! - **Configurable**: Customizable dimensions, scale, and browser capabilities
//! - **Offline Mode**: Can work with offline bundled JavaScript libraries
//! - **Automatic Management**: Handles WebDriver process lifecycle and cleanup
//! - **Parallelism**: Designed for use in parallel environments (tests, etc.)
//! - **Logging Support**: Integrated logging with `env_logger` support
//!
//! ## Quick Start
//!
//! ```no_run
//! // This example requires a WebDriver-compatible browser (Chrome/Firefox).
//! // It cannot be run as a doc test.
//! use plotly_static::{StaticExporterBuilder, ImageFormat};
//! use serde_json::json;
//! use std::path::Path;
//!
//! // Create a simple plot
//! let plot = json!({
//!     "data": [{
//!         "type": "scatter",
//!         "x": [1, 2, 3, 4],
//!         "y": [10, 11, 12, 13]
//!     }],
//!     "layout": {
//!         "title": "Simple Scatter Plot"
//!     }
//! });
//!
//! // Build and use StaticExporter
//! let mut exporter = StaticExporterBuilder::default()
//!     .build()
//!     .expect("Failed to build StaticExporter");
//!
//! // Export to PNG
//! exporter.write_fig(
//!     Path::new("my_plot"),
//!     &plot,
//!     ImageFormat::PNG,
//!     800,
//!     600,
//!     1.0
//! ).expect("Failed to export plot");
//! ```
//!
//! ## Features and Dependencies
//!
//! ### Required Features
//!
//! You must enable one of the following features:
//!
//! - `chromedriver`: Use Chrome/Chromium for rendering
//! - `geckodriver`: Use Firefox for rendering
//!
//! ### Optional Features
//!
//! - `webdriver_download`: Automatically download WebDriver binaries at build
//!   time
//!
//! ### Example Cargo.toml
//!
//! ```toml
//! [dependencies]
//! plotly_static = { version = "0.0.1", features = ["chromedriver", "webdriver_download"] }
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Configuration
//!
//! ```no_run
//! use plotly_static::StaticExporterBuilder;
//!
//! let exporter = StaticExporterBuilder::default()
//!     .webdriver_port(4444)
//!     .webdriver_url("http://localhost")
//!     .spawn_webdriver(true)
//!     .offline_mode(true)
//!     .webdriver_browser_caps(vec![
//!         "--headless".to_string(),
//!         "--no-sandbox".to_string(),
//!         "--disable-gpu".to_string(),
//!     ])
//!     .build()
//!     .expect("Failed to build StaticExporter");
//! ```
//!
//! ### Browser Binary Configuration
//!
//! You can specify custom browser binaries using environment variables:
//!
//! ```bash
//! # For Chrome/Chromium
//! export BROWSER_PATH="/path/to/chrome"
//!
//! # For Firefox
//! export BROWSER_PATH="/path/to/firefox"
//! ```
//!
//! The library will automatically use these binaries when creating WebDriver
//! sessions.
//!
//! ### String Export
//!
//! ```no_run
//! // This example requires a running WebDriver (chromedriver/geckodriver) and a browser.
//! // It cannot be run as a doc test.
//! use plotly_static::{StaticExporterBuilder, ImageFormat};
//! use serde_json::json;
//!
//! let plot = json!({
//!     "data": [{"type": "scatter", "x": [1,2,3], "y": [4,5,6]}],
//!     "layout": {}
//! });
//!
//! let mut exporter = StaticExporterBuilder::default()
//!     .build()
//!     .expect("Failed to build StaticExporter");
//!
//! let svg_data = exporter.write_to_string(
//!     &plot,
//!     ImageFormat::SVG,
//!     800,
//!     600,
//!     1.0
//! ).expect("Failed to export plot");
//!
//! // svg_data contains SVG markup that can be embedded in HTML
//! ```
//!
//! ### Logging Support
//!
//! The library supports logging via the `log` crate. Enable it with
//! `env_logger`:
//!
//! ```no_run
//! use plotly_static::StaticExporterBuilder;
//!
//! // Initialize logging (typically done once at the start of your application)
//! env_logger::init();
//!
//! // Set log level via environment variable
//! // RUST_LOG=debug cargo run
//!
//! let mut exporter = StaticExporterBuilder::default()
//!     .build()
//!     .expect("Failed to build StaticExporter");
//! ```
//!
//! ### Parallel Usage
//!
//! The library is designed to work safely in parallel environments:
//!
//! ```no_run
//! use plotly_static::{StaticExporterBuilder, ImageFormat};
//! use std::sync::atomic::{AtomicU32, Ordering};
//!
//! // Generate unique ports for parallel usage
//! static PORT_COUNTER: AtomicU32 = AtomicU32::new(4444);
//!
//! fn get_unique_port() -> u32 {
//!     PORT_COUNTER.fetch_add(1, Ordering::SeqCst)
//! }
//!
//! // Each thread/process should use a unique port
//! let mut exporter = StaticExporterBuilder::default()
//!     .webdriver_port(get_unique_port())
//!     .build()
//!     .expect("Failed to build StaticExporter");
//! ```
//!
//! ## WebDriver Management
//!
//! The library automatically manages WebDriver processes:
//!
//! - **Automatic Detection**: Detects if WebDriver is already running on the
//!   specified port
//! - **Process Spawning**: Automatically spawns WebDriver if not already
//!   running
//! - **Connection Reuse**: Reuses existing WebDriver sessions when possible
//! - **Cleanup**: Automatically terminates WebDriver processes when
//!   `StaticExporter` is dropped
//! - **External Sessions**: Can connect to externally managed WebDriver
//!   sessions
//!
//! ### WebDriver Configuration
//!
//! Set the `WEBDRIVER_PATH` environment variable to specify a custom WebDriver
//! binary location (should point to the full executable path):
//!
//! ```bash
//! export WEBDRIVER_PATH=/path/to/chromedriver
//! cargo run
//! ```
//!
//! Or use the `webdriver_download` feature for automatic download at build
//! time.
//!
//! ## Error Handling
//!
//! The library uses `anyhow::Result` for error handling. Common errors include:
//!
//! - WebDriver not available or not running
//! - Invalid Plotly JSON format
//! - File system errors
//! - Browser rendering errors
//!
//! ## Browser Support
//!
//! - **Chrome/Chromium**: Full support via chromedriver
//! - **Firefox**: Full support via geckodriver
//! - **Safari**: Not currently supported
//! - **Edge**: Not currently supported
//!
//! ## Performance Considerations
//!
//! - **Reuse Exporters**: Reuse `StaticExporter` instances for multiple exports
//! - **Parallel Usage**: Use unique ports for parallel operations
//! - **WebDriver Reuse**: The library automatically reuses WebDriver sessions
//!   when possible
//! - **Resource Cleanup**: WebDriver processes are automatically cleaned up on
//!   drop
//!
//! ## Comparison with Kaleido
//!
//! - **No custom Chromium/Chrome external dependency**: Uses standard WebDriver
//!   instead of proprietary Kaleido
//! - **Better Browser Support**: Works with any WebDriver-compatible browser:
//!   Chrome/Chromium,Firefox,Brave
//! - **Extensible**: Easy to control browser capabilities and customize the
//!   StaticExporter instance
//!
//! ## Limitations
//!
//! - Requires a WebDriver-compatible browser
//! - PDF export uses browser JavaScript `html2pdf` (not native Plotly PDF)
//! - EPS is no longer supported and will be removed
//! - Slightly slower than Kaleido
//!
//! ## License
//!
//! MIT License - see LICENSE file for details.

// TODO: remove this once version 0.14.0 is out
#![allow(deprecated)]
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::vec;
#[cfg(any(test, feature = "debug"))]
use std::{println as error, println as warn, println as debug};

use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use fantoccini::{wd::Capabilities, Client, ClientBuilder};
#[cfg(not(any(test, feature = "debug")))]
use log::{debug, error, warn};
use serde::Serialize;
use serde_json::map::Map as JsonMap;
use urlencoding::encode;
use webdriver::WebDriver;

use crate::template::{image_export_js_script, pdf_export_js_script};

mod template;
mod webdriver;

/// Supported image formats for static image export.
///
/// This enum defines all the image formats that can be exported from Plotly
/// plots. Note that PDF export is implemented using browser JavaScript
/// functionality from `html2pdf` library, not the native Plotly PDF export.
///
/// # Supported Formats
///
/// - **PNG**: Portable Network Graphics format (recommended for web use)
/// - **JPEG**: Joint Photographic Experts Group format (good for photos)
/// - **WEBP**: Google's modern image format (excellent compression)
/// - **SVG**: Scalable Vector Graphics format (vector-based, scalable)
/// - **PDF**: Portable Document Format (implemented via browser JS)
///
/// # Deprecated Formats
///
/// - **EPS**: Encapsulated PostScript format (deprecated since 0.13.0, will be
///   removed in 0.14.0)
///   - Use SVG or PDF instead for vector graphics
///   - EPS is not supported in the open source version and in versions prior to
///     0.13.0 has been generating empty images.
///
/// # Examples
///
/// ```rust
/// use plotly_static::ImageFormat;
///
/// let format = ImageFormat::PNG;
/// assert_eq!(format.to_string(), "png");
/// ```
#[derive(Debug, Clone, Serialize)]
#[allow(deprecated)]
pub enum ImageFormat {
    /// Portable Network Graphics format
    PNG,
    /// Joint Photographic Experts Group format
    JPEG,
    /// WebP format (Google's image format)
    WEBP,
    /// Scalable Vector Graphics format
    SVG,
    /// Portable Document Format (implemented via browser JS)
    PDF,
    /// Encapsulated PostScript format (deprecated)
    ///
    /// This format is deprecated since version 0.13.0 and will be removed in
    /// version 0.14.0. Use SVG or PDF instead for vector graphics. EPS is
    /// not supported in the open source Plotly ecosystem version.
    #[deprecated(
        since = "0.13.0",
        note = "Use SVG or PDF instead. EPS variant will be removed in version 0.14.0"
    )]
    EPS,
}

impl std::fmt::Display for ImageFormat {
    /// Formats the ImageFormat as a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::ImageFormat;
    /// assert_eq!(ImageFormat::SVG.to_string(), "svg");
    /// assert_eq!(ImageFormat::PDF.to_string(), "pdf");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PNG => "png",
                Self::JPEG => "jpeg",
                Self::WEBP => "webp",
                Self::SVG => "svg",
                Self::PDF => "pdf",
                #[allow(deprecated)]
                Self::EPS => "eps",
            }
        )
    }
}

/// TODO: ideally data would be a Plot object which is later serialized to JSON
/// but with the current workspace set up, that would be a cyclic dependency.
#[derive(Serialize)]
struct PlotData<'a> {
    format: ImageFormat,
    width: usize,
    height: usize,
    scale: f64,
    data: &'a serde_json::Value,
}

/// Builder for configuring and creating a `StaticExporter` instance.
///
/// This builder provides an interface for configuring WebDriver settings,
/// browser capabilities, and other options before creating a `StaticExporter`
/// instance. The builder automatically handles WebDriver process management,
/// including detection of existing sessions and automatic spawning when needed.
///
/// # Examples
///
/// ```no_run
/// // This example requires a running WebDriver (chromedriver/geckodriver) and a browser.
/// // It cannot be run as a doc test.
/// use plotly_static::StaticExporterBuilder;
///
/// let exporter = StaticExporterBuilder::default()
///     .webdriver_port(4444)
///     .spawn_webdriver(true)
///     .offline_mode(false)
///     .pdf_export_timeout(500)
///     .build()
///     .expect("Failed to build StaticExporter");
/// ```
///
/// # Default Configuration
///
/// - WebDriver port: 4444
/// - WebDriver URL: "http://localhost"
/// - Spawn webdriver: true (automatically manages WebDriver lifecycle)
/// - Offline mode: false
/// - PDF export timeout: 250ms
/// - Browser capabilities: Default Chrome/Firefox headless options
/// - Automatic WebDriver detection and connection reuse
pub struct StaticExporterBuilder {
    /// WebDriver server port (default: 4444)
    webdriver_port: u32,
    /// WebDriver server base URL (default: "http://localhost")
    webdriver_url: String,
    /// Auto-spawn WebDriver if not running (default: true)
    spawn_webdriver: bool,
    /// Use bundled JS libraries instead of CDN (default: false)
    offline_mode: bool,
    /// PDF export timeout in milliseconds (default: 150)
    pdf_export_timeout: u32,
    /// Browser command-line flags (e.g., "--headless", "--no-sandbox")
    webdriver_browser_caps: Vec<String>,
}

impl Default for StaticExporterBuilder {
    /// Creates a new `StaticExporterBuilder` with default configuration.
    ///
    /// The default configuration includes:
    /// - WebDriver port: 4444
    /// - WebDriver URL: "http://localhost"
    /// - Spawn webdriver: true
    /// - Offline mode: false
    /// - PDF export timeout: 250ms
    /// - Default browser capabilities for headless operation
    fn default() -> Self {
        Self {
            webdriver_port: webdriver::WEBDRIVER_PORT,
            webdriver_url: webdriver::WEBDRIVER_URL.to_string(),
            spawn_webdriver: true,
            offline_mode: false,
            pdf_export_timeout: 150,
            webdriver_browser_caps: {
                #[cfg(feature = "chromedriver")]
                {
                    crate::webdriver::chrome_default_caps()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect()
                }
                #[cfg(feature = "geckodriver")]
                {
                    crate::webdriver::firefox_default_caps()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect()
                }
                #[cfg(not(any(feature = "chromedriver", feature = "geckodriver")))]
                {
                    Vec::new()
                }
            },
        }
    }
}

impl StaticExporterBuilder {
    /// Sets the WebDriver port number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// let builder = StaticExporterBuilder::default()
    ///     .webdriver_port(4444);
    /// ```
    pub fn webdriver_port(mut self, port: u32) -> Self {
        self.webdriver_port = port;
        self
    }

    /// Sets the WebDriver URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// let builder = StaticExporterBuilder::default()
    ///     .webdriver_url("http://localhost");
    /// ```
    pub fn webdriver_url(mut self, url: &str) -> Self {
        self.webdriver_url = url.to_string();
        self
    }

    /// Controls whether to automatically spawn a WebDriver process.
    ///
    /// If `true`, automatically spawns a WebDriver process. If `false`,
    /// expects an existing WebDriver server to be running.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// // Auto-spawn WebDriver
    /// let builder = StaticExporterBuilder::default()
    ///     .spawn_webdriver(true);
    ///
    /// // Use existing WebDriver server
    /// let builder = StaticExporterBuilder::default()
    ///     .spawn_webdriver(false);
    /// ```
    pub fn spawn_webdriver(mut self, yes: bool) -> Self {
        self.spawn_webdriver = yes;
        self
    }

    /// Controls whether to use offline mode with bundled JavaScript libraries.
    ///
    /// If `true`, uses bundled JavaScript libraries instead of CDN. If `false`,
    /// downloads libraries from CDN.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// // Use bundled libraries (no internet required)
    /// let builder = StaticExporterBuilder::default()
    ///     .offline_mode(true);
    ///
    /// // Use CDN libraries
    /// let builder = StaticExporterBuilder::default()
    ///     .offline_mode(false);
    /// ```
    pub fn offline_mode(mut self, yes: bool) -> Self {
        self.offline_mode = yes;
        self
    }

    /// Sets the PDF export timeout in milliseconds.
    ///
    /// This timeout controls how long to wait for the SVG image to load before
    /// proceeding with PDF generation. A longer timeout may be needed for
    /// complex plots or slower systems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// // Set a longer timeout for complex plots
    /// let builder = StaticExporterBuilder::default()
    ///     .pdf_export_timeout(500);
    ///
    /// // Use default timeout (150ms)
    /// let builder = StaticExporterBuilder::default()
    ///     .pdf_export_timeout(150);
    /// ```
    pub fn pdf_export_timeout(mut self, timeout_ms: u32) -> Self {
        self.pdf_export_timeout = timeout_ms;
        self
    }

    /// Sets custom browser capabilities for the WebDriver.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// let custom_caps = vec![
    ///     "--headless".to_string(),
    ///     "--no-sandbox".to_string(),
    ///     "--disable-gpu".to_string(),
    /// ];
    ///
    /// let builder = StaticExporterBuilder::default()
    ///     .webdriver_browser_caps(custom_caps);
    /// ```
    pub fn webdriver_browser_caps(mut self, caps: Vec<String>) -> Self {
        self.webdriver_browser_caps = caps;
        self
    }

    /// Builds a `StaticExporter` instance with the current configuration.
    ///
    /// This method creates a new `StaticExporter` instance with all the
    /// configured settings. The method manages WebDriver:
    ///
    /// - If `spawn_webdriver` is enabled, it first tries to connect to an
    ///   existing WebDriver session on the specified port, and only spawns a
    ///   new process if none is found
    /// - If `spawn_webdriver` is disabled, it creates a connection to an
    ///   existing WebDriver without spawning
    ///
    /// Returns a `Result<StaticExporter>` where:
    /// - `Ok(exporter)` - Successfully created the StaticExporter instance
    /// - `Err(e)` - Failed to create the instance (e.g., WebDriver not
    ///   available, port conflicts, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_static::StaticExporterBuilder;
    ///
    /// let exporter = StaticExporterBuilder::default()
    ///     .webdriver_port(4444)
    ///     .build()
    ///     .expect("Failed to build StaticExporter");
    /// ```
    pub fn build(&self) -> Result<StaticExporter> {
        let wd = self.create_webdriver()?;

        let runtime = std::sync::Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime"),
        );

        Ok(StaticExporter {
            webdriver_port: self.webdriver_port,
            webdriver_url: self.webdriver_url.clone(),
            webdriver: wd,
            offline_mode: self.offline_mode,
            pdf_export_timeout: self.pdf_export_timeout,
            webdriver_browser_caps: self.webdriver_browser_caps.clone(),
            runtime,
            webdriver_client: None,
        })
    }

    /// Create a new WebDriver instance based on the spawn_webdriver flag
    fn create_webdriver(&self) -> Result<WebDriver> {
        match self.spawn_webdriver {
            // Try to connect to existing WebDriver or spawn new if not available
            true => WebDriver::connect_or_spawn(self.webdriver_port),
            // Create the WebDriver instance without spawning
            false => WebDriver::new(self.webdriver_port),
        }
    }
}

/// Main struct for exporting Plotly plots to static images.
///
/// This struct provides methods to convert Plotly JSON plots into various
/// static image formats using a headless browser via WebDriver.
///
/// # Examples
///
/// ```no_run
/// // This example requires a running WebDriver (chromedriver/geckodriver) and a browser.
/// // It cannot be run as a doc test.
/// use plotly_static::{StaticExporterBuilder, ImageFormat};
/// use serde_json::json;
/// use std::path::Path;
///
/// // Create a simple plot
/// let plot = json!({
///     "data": [{
///         "type": "scatter",
///         "x": [1, 2, 3],
///         "y": [4, 5, 6]
///     }],
///     "layout": {}
/// });
///
/// // Build StaticExporter instance
/// let mut exporter = StaticExporterBuilder::default()
///     .build()
///     .expect("Failed to build StaticExporter");
///
/// // Export to PNG
/// exporter.write_fig(
///     Path::new("output"),
///     &plot,
///     ImageFormat::PNG,
///     800,
///     600,
///     1.0
/// ).expect("Failed to export plot");
/// ```
///
/// # Features
///
/// - Supports multiple image formats (PNG, JPEG, WEBP, SVG, PDF)
/// - Uses headless browser for rendering
/// - Configurable dimensions and scale
/// - Offline mode support
/// - Automatic WebDriver management
pub struct StaticExporter {
    /// WebDriver server port (default: 4444)
    webdriver_port: u32,

    /// WebDriver server base URL (default: "http://localhost")
    webdriver_url: String,

    /// WebDriver process manager for spawning and cleanup
    webdriver: WebDriver,

    /// Use bundled JS libraries instead of CDN
    offline_mode: bool,

    /// PDF export timeout in milliseconds
    pdf_export_timeout: u32,

    /// Browser command-line flags (e.g., "--headless", "--no-sandbox")
    webdriver_browser_caps: Vec<String>,

    /// Tokio runtime for async operations
    runtime: std::sync::Arc<tokio::runtime::Runtime>,

    /// Cached WebDriver client for session reuse
    webdriver_client: Option<Client>,
}

impl Drop for StaticExporter {
    /// Automatically cleans up WebDriver resources when the `StaticExporter`
    /// instance is dropped.
    ///
    /// This ensures that the WebDriver process is properly terminated and
    /// resources are released, even if the instance goes out of scope
    /// unexpectedly.
    ///
    /// - Only terminates WebDriver processes that were spawned by this instance
    /// - Leaves externally managed WebDriver sessions running
    /// - Logs errors but doesn't panic if cleanup fails
    fn drop(&mut self) {
        // Close the WebDriver client if it exists
        if let Some(client) = self.webdriver_client.take() {
            let runtime = self.runtime.clone();
            runtime.block_on(async {
                if let Err(e) = client.close().await {
                    error!("Failed to close WebDriver client: {e}");
                }
            });
        }

        // Stop the WebDriver process
        if let Err(e) = self.webdriver.stop() {
            error!("Failed to stop WebDriver: {e}");
        }
    }
}

impl StaticExporter {
    /// Exports a Plotly plot to a static image file.
    ///
    /// This method renders the provided Plotly JSON plot using a headless
    /// browser and saves the result as an image file in the specified
    /// format.
    ///
    /// Returns `Ok(())` on success, or an error if the export fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // This example requires a running WebDriver (chromedriver/geckodriver) and a browser.
    /// // It cannot be run as a doc test.
    /// use plotly_static::{StaticExporterBuilder, ImageFormat};
    /// use serde_json::json;
    /// use std::path::Path;
    ///
    /// let plot = json!({
    ///     "data": [{"type": "scatter", "x": [1,2,3], "y": [4,5,6]}],
    ///     "layout": {}
    /// });
    ///
    /// let mut exporter = StaticExporterBuilder::default().build().unwrap();
    ///
    /// exporter.write_fig(
    ///     Path::new("my_plot"),
    ///     &plot,
    ///     ImageFormat::PNG,
    ///     1200,
    ///     800,
    ///     2.0
    /// ).expect("Failed to export plot");
    /// // Creates "my_plot.png" with 1200x800 pixels at 2x scale
    /// ```
    ///
    /// # Notes
    ///
    /// - The file extension is automatically added based on the format
    /// - SVG format outputs plain text, others output binary data
    /// - PDF format uses browser JavaScript for generation
    pub fn write_fig(
        &mut self,
        dst: &Path,
        plot: &serde_json::Value,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut dst = PathBuf::from(dst);
        dst.set_extension(format.to_string());

        let plot_data = PlotData {
            format: format.clone(),
            width,
            height,
            scale,
            data: plot,
        };

        let image_data = self.export(plot_data)?;
        let data = match format {
            ImageFormat::SVG => image_data.as_bytes(),
            _ => &general_purpose::STANDARD.decode(image_data)?,
        };
        let mut file = File::create(dst.as_path())?;
        file.write_all(data)?;
        file.flush()?;

        Ok(())
    }

    /// Exports a Plotly plot to a string representation.
    ///
    /// This method renders the provided Plotly JSON plot and returns the result
    /// as a string. The format of the string depends on the image format:
    /// - SVG: Returns plain SVG text
    /// - PNG/JPEG/WEBP/PDF: Returns base64-encoded data
    ///
    /// Returns the image data as a string on success, or an error if the export
    /// fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // This example requires a running WebDriver (chromedriver/geckodriver) and a browser.
    /// // It cannot be run as a doc test.
    /// use plotly_static::{StaticExporterBuilder, ImageFormat};
    /// use serde_json::json;
    ///
    /// let plot = json!({
    ///     "data": [{"type": "scatter", "x": [1,2,3], "y": [4,5,6]}],
    ///     "layout": {}
    /// });
    ///
    /// let mut exporter = StaticExporterBuilder::default().build().unwrap();
    ///
    /// let svg_data = exporter.write_to_string(
    ///     &plot,
    ///     ImageFormat::SVG,
    ///     800,
    ///     600,
    ///     1.0
    /// ).expect("Failed to export plot");
    ///
    /// // svg_data contains the SVG markup as a string
    /// assert!(svg_data.starts_with("<svg"));
    /// ```
    ///
    /// # Notes
    ///
    /// - SVG format returns plain text that can be embedded in HTML
    /// - Other formats return base64-encoded data that can be used in data URLs
    /// - This method is useful when you need the image data as a string rather
    ///   than a file
    pub fn write_to_string(
        &mut self,
        plot: &serde_json::Value,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let plot_data = PlotData {
            format,
            width,
            height,
            scale,
            data: plot,
        };
        let image_data = self.export(plot_data)?;
        Ok(image_data)
    }

    /// Convert the Plotly graph to a static image using Kaleido and return the
    /// result as a String
    pub(crate) fn export(&mut self, plot: PlotData) -> Result<String> {
        let data = self.static_export(&plot)?;
        Ok(data)
    }

    fn static_export(&mut self, plot: &PlotData<'_>) -> Result<String> {
        let html_content = template::get_html_body(self.offline_mode);
        let runtime = self.runtime.clone();
        runtime
            .block_on(self.extract(&html_content, plot))
            .with_context(|| "Failed to extract static image from browser session")
    }

    async fn extract(&mut self, html_content: &str, plot: &PlotData<'_>) -> Result<String> {
        let caps = self.build_webdriver_caps()?;
        debug!("Use WebDriver and headless browser to export static plot");
        let webdriver_url = format!("{}:{}", self.webdriver_url, self.webdriver_port,);

        // Reuse existing client or create new one
        let client = if let Some(ref client) = self.webdriver_client {
            debug!("Reusing existing WebDriver session");
            client.clone()
        } else {
            debug!("Creating new WebDriver session");
            let new_client = ClientBuilder::native()
                .capabilities(caps)
                .connect(&webdriver_url)
                .await
                .with_context(|| "WebDriver session error")?;
            self.webdriver_client = Some(new_client.clone());
            new_client
        };

        // For offline mode, write HTML to file to avoid data URI size limits since JS
        // libraries are embedded in the file
        let url = if self.offline_mode {
            let temp_file = template::to_file(html_content)
                .with_context(|| "Failed to create temporary HTML file")?;
            format!("file://{}", temp_file.to_string_lossy())
        } else {
            // For online mode, use data URI (smaller size since JS is loaded from CDN)
            format!("data:text/html,{}", encode(html_content))
        };

        // Open the HTML
        client.goto(&url).await?;

        let (js_script, args) = match plot.format {
            ImageFormat::PDF => {
                // Always use SVG for PDF export
                let args = vec![
                    plot.data.clone(),
                    ImageFormat::SVG.to_string().into(),
                    plot.width.into(),
                    plot.height.into(),
                    plot.scale.into(),
                ];

                (pdf_export_js_script(self.pdf_export_timeout), args)
            }
            _ => {
                let args = vec![
                    plot.data.clone(),
                    plot.format.to_string().into(),
                    plot.width.into(),
                    plot.height.into(),
                    plot.scale.into(),
                ];

                (image_export_js_script(), args)
            }
        };

        let data = client.execute_async(&js_script, args).await?;

        // Don't close the client - keep it for reuse
        // client.close().await?;

        let result = data.as_str().ok_or(anyhow!(
            "Failed to execute Plotly.toImage in browser session"
        ))?;

        if let Some(err) = result.strip_prefix("ERROR:") {
            return Err(anyhow!("JavaScript error during export: {err}"));
        }

        match plot.format {
            ImageFormat::SVG => Self::extract_plain(result, &plot.format),
            ImageFormat::PNG | ImageFormat::JPEG | ImageFormat::WEBP | ImageFormat::PDF => {
                Self::extract_encoded(result, &plot.format)
            }
            #[allow(deprecated)]
            ImageFormat::EPS => {
                error!("EPS format is deprecated. Use SVG or PDF instead.");
                Self::extract_encoded(result, &plot.format)
            }
        }
    }

    fn extract_plain(payload: &str, format: &ImageFormat) -> Result<String> {
        match payload.split_once(",") {
            Some((type_info, data)) => {
                Self::extract_type_info(type_info, format);
                let decoded = urlencoding::decode(data)?;
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
                    error!("Requested ImageFormat '{format}', got '{ext}'");
                }
            }
            None => warn!("Failed to extract static Image Format from 'src' attribute"),
        }
    }

    fn extract_encoded_data(data: &str) -> Option<String> {
        data.split_once(",").map(|d| d.1.to_string())
    }

    /// Get Firefox preferences optimized for CI environments.
    ///
    /// These preferences force software rendering and enable WebGL in headless
    /// mode to work around graphics/WebGL issues in CI environments.
    #[cfg(feature = "geckodriver")]
    fn get_firefox_ci_preferences() -> serde_json::Map<String, serde_json::Value> {
        let mut prefs = serde_json::Map::new();

        // Force software rendering and enable WebGL in headless mode
        prefs.insert(
            "layers.acceleration.disabled".to_string(),
            serde_json::json!(true),
        );
        prefs.insert("gfx.webrender.all".to_string(), serde_json::json!(false));
        prefs.insert(
            "gfx.webrender.software".to_string(),
            serde_json::json!(true),
        );
        prefs.insert("webgl.disabled".to_string(), serde_json::json!(false));
        prefs.insert("webgl.force-enabled".to_string(), serde_json::json!(true));
        prefs.insert("webgl.enable-webgl2".to_string(), serde_json::json!(true));

        // Force software WebGL implementation
        prefs.insert(
            "webgl.software-rendering".to_string(),
            serde_json::json!(true),
        );
        prefs.insert(
            "webgl.software-rendering.force".to_string(),
            serde_json::json!(true),
        );

        // Disable hardware acceleration completely
        prefs.insert(
            "gfx.canvas.azure.accelerated".to_string(),
            serde_json::json!(false),
        );
        prefs.insert(
            "gfx.canvas.azure.accelerated-layers".to_string(),
            serde_json::json!(false),
        );
        prefs.insert(
            "gfx.content.azure.backends".to_string(),
            serde_json::json!("cairo"),
        );

        // Force software rendering for all graphics
        prefs.insert("gfx.2d.force-enabled".to_string(), serde_json::json!(true));
        prefs.insert("gfx.2d.force-software".to_string(), serde_json::json!(true));

        prefs
    }

    fn build_webdriver_caps(&self) -> Result<Capabilities> {
        // Define browser capabilities
        let mut caps = JsonMap::new();
        let mut browser_opts = JsonMap::new();
        let browser_args = self.webdriver_browser_caps.clone();

        browser_opts.insert("args".to_string(), serde_json::json!(browser_args));

        // Add Chrome binary capability if BROWSER_PATH is set
        #[cfg(feature = "chromedriver")]
        if let Ok(chrome_path) = std::env::var("BROWSER_PATH") {
            browser_opts.insert("binary".to_string(), serde_json::json!(chrome_path));
            debug!("Added Chrome binary capability: {chrome_path}");
        }
        // Add Firefox binary capability if BROWSER_PATH is set
        #[cfg(feature = "geckodriver")]
        if let Ok(firefox_path) = std::env::var("BROWSER_PATH") {
            browser_opts.insert("binary".to_string(), serde_json::json!(firefox_path));
            debug!("Added Firefox binary capability: {firefox_path}");
        }

        // Add Firefox-specific preferences for CI environments
        #[cfg(feature = "geckodriver")]
        {
            let prefs = Self::get_firefox_ci_preferences();
            browser_opts.insert("prefs".to_string(), serde_json::json!(prefs));
            debug!("Added Firefox preferences for CI compatibility");
        }

        caps.insert(
            "browserName".to_string(),
            serde_json::json!(get_browser_name()),
        );
        caps.insert(
            get_options_key().to_string(),
            serde_json::json!(browser_opts),
        );

        debug!("WebDriver capabilities: {caps:?}");

        Ok(caps)
    }

    /// Get diagnostic information about the underlying WebDriver process.
    ///
    /// This method provides detailed information about the WebDriver process
    /// for debugging purposes, including process status, port information,
    /// and connection details.
    pub fn get_webdriver_diagnostics(&self) -> String {
        self.webdriver.get_diagnostics()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;

    fn init() {
        let _ = env_logger::try_init();
    }

    // Helper to generate unique ports for parallel tests
    static PORT_COUNTER: AtomicU32 = AtomicU32::new(4444);

    fn get_unique_port() -> u32 {
        PORT_COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    fn create_test_plot() -> serde_json::Value {
        serde_json::to_value(serde_json::json!(
            {
            "data": [
              {
                "name": "Surface",
                "type": "surface",
                "x": [
                  1.0,
                  2.0,
                  3.0
                ],
                "y": [
                  4.0,
                  5.0,
                  6.0
                ],
                "z": [
                  [
                    1.0,
                    2.0,
                    3.0
                  ],
                  [
                    4.0,
                    5.0,
                    6.0
                  ],
                  [
                    7.0,
                    8.0,
                    9.0
                  ]
                ]
              }
            ],
            "layout": {
                "autosize": false,
                "width": 1200,
                "height": 900,
                "scene": {
                    "domain": {
                        "x": [0.15, 0.95],
                        "y": [0.15, 0.95]
                    },
                    "aspectmode": "data",
                    "aspectratio": {
                        "x": 1,
                        "y": 1,
                        "z": 1
                    },
                    "camera": {
                        "eye": {"x": 1.5, "y": 1.5, "z": 1.5}
                    }
                },
                "config": {
                    "responsive": false
                },
            },
        }))
        .unwrap()
    }

    #[test]
    fn save_png() {
        init();
        let test_plot = create_test_plot();

        let mut export = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .build()
            .unwrap();
        let dst = PathBuf::from("static_example.png");
        export
            .write_fig(dst.as_path(), &test_plot, ImageFormat::PNG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_jpeg() {
        init();
        let test_plot = create_test_plot();
        let mut export = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .build()
            .unwrap();
        let dst = PathBuf::from("static_example.jpeg");
        export
            .write_fig(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_svg() {
        init();
        let test_plot = create_test_plot();
        let mut export = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .build()
            .unwrap();
        let dst = PathBuf::from("static_example.svg");
        export
            .write_fig(dst.as_path(), &test_plot, ImageFormat::SVG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_webp() {
        init();
        let test_plot = create_test_plot();
        let mut export = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .build()
            .unwrap();
        let dst = PathBuf::from("static_example.webp");
        export
            .write_fig(dst.as_path(), &test_plot, ImageFormat::WEBP, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_pdf() {
        init();
        let test_plot = create_test_plot();
        #[cfg(feature = "debug")]
        let mut exporter = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .pdf_export_timeout(750)
            .build()
            .unwrap();

        #[cfg(not(feature = "debug"))]
        let mut exporter = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .build()
            .unwrap();

        let dst = PathBuf::from("static_example.pdf");
        exporter
            .write_fig(dst.as_path(), &test_plot, ImageFormat::PDF, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 600000,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_jpeg_sequentially() {
        init();
        let test_plot = create_test_plot();
        let mut export = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(get_unique_port())
            .build()
            .unwrap();

        let dst = PathBuf::from("static_example.jpeg");
        export
            .write_fig(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());

        let dst = PathBuf::from("example2.jpeg");
        export
            .write_fig(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    #[cfg(feature = "chromedriver")]
    // Skip this test for geckodriver as it doesn't support multiple concurrent
    // sessions on the same process as gracefully as chromedriver
    fn test_webdriver_process_reuse() {
        init();
        let test_plot = create_test_plot();

        // Use a unique port to test actual WebDriver process reuse
        let test_port = get_unique_port();

        // Create first exporter - this should spawn a new WebDriver
        let mut export1 = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(test_port)
            .build()
            .unwrap();

        // Export first image
        let dst1 = PathBuf::from("process_reuse_1.png");
        export1
            .write_fig(dst1.as_path(), &test_plot, ImageFormat::PNG, 800, 600, 1.0)
            .unwrap();
        assert!(dst1.exists());
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst1.as_path()).is_ok());

        // Create second exporter on the same port - this should connect to existing
        // WebDriver process (but create a new session)
        let mut export2 = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(test_port)
            .build()
            .unwrap();

        // Export second image using a new session on the same WebDriver process
        let dst2 = PathBuf::from("process_reuse_2.png");
        export2
            .write_fig(dst2.as_path(), &test_plot, ImageFormat::PNG, 800, 600, 1.0)
            .unwrap();
        assert!(dst2.exists());
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst2.as_path()).is_ok());

        // Create third exporter on the same port - should also connect to existing
        // WebDriver process
        let mut export3 = StaticExporterBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(test_port)
            .build()
            .unwrap();

        // Export third image using another new session on the same WebDriver process
        let dst3 = PathBuf::from("process_reuse_3.png");
        export3
            .write_fig(dst3.as_path(), &test_plot, ImageFormat::PNG, 800, 600, 1.0)
            .unwrap();
        assert!(dst3.exists());
        #[cfg(not(feature = "debug"))]
        assert!(std::fs::remove_file(dst3.as_path()).is_ok());
    }
}

#[cfg(feature = "chromedriver")]
mod chrome {
    /// Returns the browser name for Chrome WebDriver.
    ///
    /// This function returns "chrome" as the browser identifier for Chrome
    /// WebDriver. It's used internally to configure WebDriver capabilities.
    pub fn get_browser_name() -> &'static str {
        "chrome"
    }

    /// Returns the Chrome-specific options key for WebDriver capabilities.
    ///
    /// This function returns "goog:chromeOptions" which is the standard key
    /// for Chrome-specific WebDriver options.
    pub fn get_options_key() -> &'static str {
        "goog:chromeOptions"
    }
}

#[cfg(feature = "geckodriver")]
mod firefox {
    /// Returns the browser name for Firefox WebDriver.
    ///
    /// This function returns "firefox" as the browser identifier for Firefox
    /// WebDriver. It's used internally to configure WebDriver capabilities.
    pub fn get_browser_name() -> &'static str {
        "firefox"
    }

    /// Returns the Firefox-specific options key for WebDriver capabilities.
    ///
    /// This function returns "moz:firefoxOptions" which is the standard key
    /// for Firefox-specific WebDriver options.
    pub fn get_options_key() -> &'static str {
        "moz:firefoxOptions"
    }
}

#[cfg(feature = "chromedriver")]
use chrome::{get_browser_name, get_options_key};
#[cfg(feature = "geckodriver")]
use firefox::{get_browser_name, get_options_key};
