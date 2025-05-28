use anyhow::{anyhow, Context, Result};
use fantoccini::{wd::Capabilities, ClientBuilder, Locator};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use tokio::runtime::Runtime;
use webdriver::WebDriver;

use base64::{engine::general_purpose, Engine as _};

#[cfg(not(test))]
use log::{error, info, warn};

#[cfg(test)]
use std::{println as info, println as warn, println as error};

#[cfg(feature = "geckodriver")]
const DRIVER_ARGS: &str =
    r#"{"browserName":"firefox","moz:firefoxOptions":{"args":["--headless", "-disable-gpu"]}}"#;
#[cfg(feature = "chromedriver")]
const DRIVER_ARGS: &str =
    r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless", "--disable-gpu"]}}"#;

mod template;
mod webdriver;

/// Image format for static image export.
#[derive(Debug, Clone, Serialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
    SVG,
    PDF,
    EPS,
}

impl std::fmt::Display for ImageFormat {
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
                Self::EPS => "eps",
            }
        )
    }
}

// TODO: how to avoid cyclic dependency on the ImageFormat and the Plot data
// ideally ImageFormat will be defined in a single place and the `data` field would
// be just a Plot object which is later serialized to JSON
#[derive(Serialize)]
struct PlotData<'a> {
    format: ImageFormat,
    width: usize,
    height: usize,
    scale: f64,
    data: &'a serde_json::Value,
}
pub struct StaticlyBuilder {
    webdriver_port: u32,
    webdriver_url: String,
    spawn_webdriver: bool,
    offline_mode: bool,
}

impl Default for StaticlyBuilder {
    fn default() -> Self {
        Self {
            webdriver_port: webdriver::WEBDRIVER_PORT,
            webdriver_url: webdriver::WEBDRIVER_URL.to_string(),
            spawn_webdriver: true,
            offline_mode: false,
        }
    }
}
impl StaticlyBuilder {
    pub fn webdriver_port(mut self, port: u32) -> Self {
        self.webdriver_port = port;
        self
    }

    pub fn webdriver_url(mut self, url: &str) -> Self {
        self.webdriver_url = url.to_string();
        self
    }

    pub fn spawn_webdriver(mut self, yes: bool) -> Self {
        self.spawn_webdriver = yes;
        self
    }

    pub fn offline_mode(mut self, yes: bool) -> Self {
        self.offline_mode = yes;
        self
    }

    pub fn build(&self) -> Result<Staticly> {
        let mut wd = WebDriver::new(self.webdriver_port, &self.webdriver_url)?;
        if self.spawn_webdriver {
            wd.spawn_webdriver();
        }
        Ok(Staticly {
            webdriver_port: self.webdriver_port,
            webdriver_url: self.webdriver_url.to_string(),
            webdriver: wd,
            offline_mode: self.offline_mode,
        })
    }
}

pub struct Staticly {
    webdriver_port: u32,
    webdriver_url: String,
    webdriver: WebDriver,
    offline_mode: bool,
}

impl Drop for Staticly {
    fn drop(&mut self) {
        if let Err(e) = self.webdriver.stop() {
            error!("Failed to release WebDriver process: {e}");
        }
    }
}

impl Staticly {
    /// Generate a static image from a Plotly graph and save it to a file
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
            ImageFormat::EPS | ImageFormat::SVG => image_data.as_bytes(),
            _ => &general_purpose::STANDARD.decode(image_data)?,
        };
        let mut file = File::create(dst.as_path())?;
        file.write_all(data)?;
        file.flush()?;

        Ok(())
    }

    /// Generate a static image from a Plotly graph and return it as a String
    /// The output may be base64 encoded or a plain text depending on the image
    /// format provided as argument. SVG and EPS are returned in plain text
    /// while JPEG, PNG, WEBP will be returned as a base64 encoded string.
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

    fn static_export(&mut self, data: &PlotData) -> Result<String> {
        let file = template::generate_html_file(self.offline_mode)?;
        Runtime::new()?
            .block_on(self.extract(&file, data))
            .with_context(|| "Failed to extract static image from browser session")
    }

    async fn extract(&mut self, file_path: &PathBuf, plot: &PlotData<'_>) -> Result<String> {
        info!("Export static plot using WebDriver");
        let cap: Capabilities = serde_json::from_str(DRIVER_ARGS)?;
        let webdriver_url = format!("{}:{}", self.webdriver_url, self.webdriver_port,);

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

        let js = r#"
            const plot = arguments[0];
            console.log(plot);
            const graph_div = document.getElementById("plotly-html-element");
            Plotly.newPlot(graph_div, plot)
            const img_element = document.getElementById("plotly-img-element");
            const data = Plotly.toImage(graph_div, {
                    format: arguments[1],
                    width: arguments[2],
                    height: arguments[3],
                }
            );
            return data;
            "#;

        let args = vec![
            plot.data.clone(),
            plot.format.to_string().into(),
            plot.width.try_into()?,
            plot.height.try_into()?,
        ];
        // dbg!(&args[0]);
        // dbg!(&args);
        let data = client.execute(js, args).await?;
        // dbg!(&data);
        // Really ... really guys ...
        let src = data.as_str().unwrap_or_default();

        client.close().await?;

        match plot.format {
            ImageFormat::SVG => Self::extract_plain(&src, &plot.format),
            ImageFormat::PNG | ImageFormat::JPEG => Self::extract_encoded(&src, &plot.format),
            _ => return Err(anyhow!("Not implemented for {}", plot.format)),
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
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

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
            "layout": {},
            "config": {},
        }))
        .unwrap()
    }

    #[test]
    // #[ignore]
    fn save_png() {
        let test_plot = create_test_plot();

        let mut ps = StaticlyBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(4444)
            .build()
            .unwrap();
        let dst = PathBuf::from("example.png");
        ps.write_fig(dst.as_path(), &test_plot, ImageFormat::PNG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_jpeg() {
        let test_plot = create_test_plot();
        let mut ps = StaticlyBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(4445)
            .build()
            .unwrap();
        let dst = PathBuf::from("example.jpeg");
        ps.write_fig(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_jpeg_sequentially() {
        let test_plot = create_test_plot();
        let mut ps = StaticlyBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(4446)
            .build()
            .unwrap();

        let dst = PathBuf::from("example.jpeg");
        ps.write_fig(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());

        let dst = PathBuf::from("example2.jpeg");
        ps.write_fig(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    #[ignore]
    fn save_webp() {
        let test_plot = create_test_plot();
        let mut k = StaticlyBuilder::default().build().unwrap();
        let dst = PathBuf::from("example.webp");
        k.write_fig(dst.as_path(), &test_plot, ImageFormat::WEBP, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    // #[ignore]
    fn save_svg() {
        let test_plot = create_test_plot();
        let mut ps = StaticlyBuilder::default()
            .spawn_webdriver(true)
            .webdriver_port(4447)
            .build()
            .unwrap();
        let dst = PathBuf::from("example.svg");
        ps.write_fig(dst.as_path(), &test_plot, ImageFormat::SVG, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    #[ignore]
    fn save_pdf() {
        let test_plot = create_test_plot();
        let mut k = StaticlyBuilder::default().build().unwrap();
        let dst = PathBuf::from("example.pdf");
        k.write_fig(dst.as_path(), &test_plot, ImageFormat::PDF, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    #[ignore]
    fn save_eps() {
        let test_plot = create_test_plot();
        let mut k = StaticlyBuilder::default().build().unwrap();
        let dst = PathBuf::from("example.eps");
        k.write_fig(dst.as_path(), &test_plot, ImageFormat::EPS, 1200, 900, 4.5)
            .unwrap();
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }
}
