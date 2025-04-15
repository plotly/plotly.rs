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
    // TODO: as with `data`, it would be much better if this were a plotly::ImageFormat, but
    // problems with cyclic dependencies.
    format: String,
    width: usize,
    height: usize,
    scale: f64,
    // TODO: it would be great if this could be a plotly::Plot, but with the current workspace set
    // up, that would be a cyclic dependency.
    data: &'a Value,
}

impl<'a> PlotData<'a> {
    fn new(data: &'a Value, format: &str, width: usize, height: usize, scale: f64) -> PlotData<'a> {
        PlotData {
            format: format.to_string(),
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
            Err(msg) => panic!("Failed tu use Kaleido binary at {} due to {}", path, msg),
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
        format: &str,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut dst = PathBuf::from(dst);
        dst.set_extension(format);

        let image_data = self.convert(plotly_data, format, width, height, scale)?;
        let data = match format {
            "svg" | "eps" => image_data.as_bytes(),
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
        format: &str,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let image_data = self.convert(plotly_data, format, width, height, scale)?;
        Ok(image_data)
    }

    /// Convert the Plotly graph to a static image using Kaleido and return the
    /// result as a String
    pub fn convert(
        &self,
        plotly_data: &Value,
        format: &str,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let p = self.cmd_path.to_str().unwrap();

        #[allow(clippy::zombie_processes)]
        let mut process = Command::new(p)
            .current_dir(self.cmd_path.parent().unwrap())
            .args([
                "plotly",
                "--disable-gpu",
                "--allow-file-access-from-files",
                "--disable-breakpad",
                "--disable-dev-shm-usage",
                "--disable-software-rasterizer",
                "--single-process",
                "--disable-gpu",
                "--no-sandbox",
            ])
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
        println!("Kaleido failed to generate static image for format: {format}.");
        println!("Kaleido stderr output:");
        let stderr = process.stderr.take().unwrap();
        let stderr_lines = BufReader::new(stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
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
                    "type": "scatter",
                    "x": [1, 2, 3, 4],
                    "y": [10, 15, 13, 17],
                    "name": "trace1",
                    "mode": "markers"
                },
                {
                    "type": "scatter",
                    "x": [2, 3, 4, 5],
                    "y": [16, 5, 11, 9],
                    "name": "trace2",
                    "mode": "lines"
                },
                {
                    "type": "scatter",
                    "x": [1, 2, 3, 4],
                    "y": [12, 9, 15, 12],
                    "name": "trace3",
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
        let kaleido_data = PlotData::new(&test_plot, "png", 400, 500, 1.);
        let expected = json!({
            "data": test_plot,
            "format": "png",
            "width": 400,
            "height": 500,
            "scale": 1.0
        });

        assert_eq!(to_value(kaleido_data).unwrap(), expected);
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn save_png() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.png");
        let r = k.save(dst.as_path(), &test_plot, "png", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn save_jpeg() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.jpeg");
        let r = k.save(dst.as_path(), &test_plot, "jpeg", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn save_webp() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.webp");
        let r = k.save(dst.as_path(), &test_plot, "webp", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn save_svg() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.svg");
        let r = k.save(dst.as_path(), &test_plot, "svg", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn save_pdf() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.pdf");
        let r = k.save(dst.as_path(), &test_plot, "pdf", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This generates empty eps files for some reason
    #[test]
    #[ignore]
    fn save_eps() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.eps");
        let r = k.save(dst.as_path(), &test_plot, "eps", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }
}
