//! # Plotly Kaleido
//! Plotly Kaleido implements the `kaleido` feature for [Plotly.rs](https://github.com/plotly/plotly.rs)
//!
//! The `kaleido` feature enables `Plot` conversion to the following output
//! formats: png, jpeg, webp, svg, pdf and eps. It has the added benefit over
//! the `orca` feature in that there is no need for an additional installation
//! for the feature to function properly. The Kaleido precompiled binaries are
//! packaged redistributed from the original repository [plotly/Kaleido](https://github.com/plotly/Kaleido).
//!
//! Note that [plotly/Kaleido](https://github.com/plotly/Kaleido) is still in pre-release and as such the `kaleido`
//! feature should be considered in pre-release mode as well.

#![cfg(not(target_family = "wasm"))]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Image format for static image export using Kaleido.
///
/// This enum defines all the image formats that can be exported from Plotly
/// plots using the Kaleido engine. Kaleido supports all these formats natively.
#[derive(Debug, Clone)]
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
    /// Portable Document Format
    PDF,
    /// Encapsulated PostScript format (deprecated)
    ///
    /// This format is deprecated since version 0.13.0 and will be removed in
    /// version 0.14.0. Use SVG or PDF instead for vector graphics. EPS is
    /// not supported in the open source version.
    #[deprecated(
        since = "0.13.0",
        note = "Use SVG or PDF instead. EPS variant will be removed in version 0.14.0"
    )]
    EPS,
}

impl std::fmt::Display for ImageFormat {
    /// Converts the ImageFormat to its lowercase string representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use plotly_kaleido::ImageFormat;
    ///
    /// assert_eq!(ImageFormat::PNG.to_string(), "png");
    /// assert_eq!(ImageFormat::SVG.to_string(), "svg");
    /// assert_eq!(ImageFormat::PDF.to_string(), "pdf");
    /// assert_eq!(ImageFormat::EPS.to_string(), "eps");
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

impl serde::Serialize for ImageFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct KaleidoResult {
    code: i32,
    message: Option<String>,
    #[serde(rename = "pdfBgColor")]
    pdf_background_color: Option<String>,
    format: Option<String>,
    result: Option<String>,
    width: Option<usize>,
    height: Option<usize>,
    scale: Option<f64>,
    version: Option<String>,
}

impl KaleidoResult {
    fn from(result: &str) -> KaleidoResult {
        serde_json::from_str(result).unwrap()
    }
}

#[derive(Serialize)]
struct PlotData<'a> {
    format: ImageFormat,
    width: usize,
    height: usize,
    scale: f64,
    // TODO: it would be great if this could be a plotly::Plot, but with the current workspace set
    // up, that would be a cyclic dependency.
    data: &'a Value,
}

impl<'a> PlotData<'a> {
    fn new(
        data: &'a Value,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> PlotData<'a> {
        PlotData {
            format,
            width,
            height,
            scale,
            data,
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Default)]
pub struct Kaleido {
    cmd_path: PathBuf,
}

impl Kaleido {
    const KALEIDO_PATH_ENV: &str = "KALEIDO_PATH";

    pub fn new() -> Kaleido {
        use std::env;

        let path = match env::var(Self::KALEIDO_PATH_ENV) {
            Ok(runtime_env) => runtime_env,
            Err(runtime_env_err) => match option_env!("KALEIDO_COMPILE_TIME_DLD_PATH") {
                Some(compile_time_path) => compile_time_path.to_string(),
                None => {
                    println!("{}: {}", Self::KALEIDO_PATH_ENV, runtime_env_err);
                    println!("Use `kaleido_download` feature to automatically download, install and use Kaleido when targeting applications that run on the host machine.");
                    println!("Use `{}` environment variable when targeting applications intended to run on different machines. Manually install Kaleido on the target machine and point {} to the installation location.", Self::KALEIDO_PATH_ENV, Self::KALEIDO_PATH_ENV
                    );
                    std::process::exit(1);
                }
            },
        };

        let path = match Kaleido::binary_path(&path) {
            Ok(kaleido_path) => kaleido_path,
            Err(msg) => panic!("Failed tu use Kaleido binary at {path} due to {msg}"),
        };

        Kaleido { cmd_path: path }
    }

    fn binary_path(dld_path: &str) -> Result<PathBuf, &'static str> {
        let mut p = PathBuf::from(dld_path);
        p = Self::os_binary_path(p);
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn os_binary_path(path: PathBuf) -> PathBuf {
        match path.join("kaleido").canonicalize() {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "Failed to find Kaleido binary at '{}': {e}",
                    path.to_string_lossy()
                );
                panic!("{e}");
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn os_binary_path(path: PathBuf) -> PathBuf {
        path.join("kaleido.cmd")
    }

    /// Generate a static image from a Plotly graph and save it to a file
    pub fn save(
        &self,
        dst: &Path,
        plotly_data: &Value,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut dst = PathBuf::from(dst);
        dst.set_extension(format.to_string());

        let image_data = self.convert(plotly_data, format.clone(), width, height, scale)?;
        #[allow(deprecated)]
        let data = match format {
            ImageFormat::SVG | ImageFormat::EPS => image_data.as_bytes(),
            _ => &general_purpose::STANDARD.decode(image_data).unwrap(),
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
    pub fn image_to_string(
        &self,
        plotly_data: &Value,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let image_data = self.convert(plotly_data, format.clone(), width, height, scale)?;
        Ok(image_data)
    }

    /// Convert the Plotly graph to a static image using Kaleido and return the
    /// result as a String
    pub fn convert(
        &self,
        plotly_data: &Value,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let p = self.cmd_path.to_str().unwrap();
        let format_str = format.to_string();

        // Removed flag 'disable-gpu' as it causes issues on MacOS and other platforms
        // see Kaleido issue #323
        let cmd_args = vec![
            "plotly",
            "--allow-file-access-from-files",
            "--disable-breakpad",
            "--disable-dev-shm-usage",
            "--disable-software-rasterizer",
            "--single-process",
            "--no-sandbox",
        ];

        #[allow(clippy::zombie_processes)]
        let mut process = Command::new(p)
            .current_dir(self.cmd_path.parent().unwrap())
            .args(cmd_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| {
                panic!(
                    "{}",
                    format!(
                        "failed to spawn Kaleido binary at {}",
                        self.cmd_path.to_string_lossy()
                    )
                    .to_string()
                )
            });
        {
            let plot_data = PlotData::new(plotly_data, format, width, height, scale).to_json();
            let mut process_stdin = process.stdin.take().unwrap();
            process_stdin
                .write_all(plot_data.as_bytes())
                .expect("couldn't write to Kaleido stdin");
            process_stdin.flush()?;
        }

        let output_lines = BufReader::new(process.stdout.take().unwrap()).lines();
        for line in output_lines.map_while(Result::ok) {
            let res = KaleidoResult::from(line.as_str());
            if let Some(image_data) = res.result {
                // TODO: this should be refactored
                // The assumption is that  KaleidoResult contains a single image.
                // We should end the loop on the first valid one.
                // If that is not the case, prior implementation would have returned the last
                // valid image
                return Ok(image_data);
            }
        }

        // Don't eat up Kaleido/Chromium errors but show them in the terminal
        println!("Kaleido failed to generate static image for format: {format_str}.");
        println!("Kaleido stderr output:");
        let stderr = process.stderr.take().unwrap();
        let stderr_lines = BufReader::new(stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{line}");
        }

        Ok(String::default())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde_json::{json, to_value};

    use super::*;

    fn create_test_plot() -> Value {
        to_value(json!({
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
            "layout": {}
        }))
        .unwrap()
    }

    #[test]
    fn can_find_kaleido_executable() {
        let _k = Kaleido::new();
    }

    #[test]
    fn plot_data_to_json() {
        let test_plot = create_test_plot();
        let kaleido_data = PlotData::new(&test_plot, ImageFormat::PNG, 400, 500, 1.);
        let expected = json!({
            "data": test_plot,
            "format": "png",
            "width": 400,
            "height": 500,
            "scale": 1.0
        });

        assert_eq!(to_value(kaleido_data).unwrap(), expected);
    }

    // For MacOS failures, see issue #241 and upstream https://github.com/plotly/Kaleido/issues/323 is resolved
    #[test]
    fn save_png() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.png");
        let r = k.save(dst.as_path(), &test_plot, ImageFormat::PNG, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_jpeg() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.jpeg");
        let r = k.save(dst.as_path(), &test_plot, ImageFormat::JPEG, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_webp() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.webp");
        let r = k.save(dst.as_path(), &test_plot, ImageFormat::WEBP, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_svg() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.svg");
        let r = k.save(dst.as_path(), &test_plot, ImageFormat::SVG, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn save_pdf() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.pdf");
        let r = k.save(dst.as_path(), &test_plot, ImageFormat::PDF, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // Kaleido generates empty eps files
    #[test]
    #[ignore]
    fn save_eps() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.eps");
        #[allow(deprecated)]
        let r = k.save(dst.as_path(), &test_plot, ImageFormat::EPS, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }
}
