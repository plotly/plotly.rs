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

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use base64::{engine::general_purpose, Engine as _};
use directories::ProjectDirs;
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
    pub fn new() -> Kaleido {
        let path = match Kaleido::binary_path() {
            Ok(path) => path,
            Err(msg) => panic!("{}", msg),
        };

        Kaleido { cmd_path: path }
    }

    fn root_dir() -> Result<PathBuf, &'static str> {
        let project_dirs = ProjectDirs::from("org", "plotly", "kaleido")
            .expect("Could not create plotly_kaleido config directory.");
        Ok(project_dirs.config_dir().into())
    }

    #[cfg(target_os = "linux")]
    fn binary_path() -> Result<PathBuf, &'static str> {
        let mut p = Kaleido::root_dir()?;
        p = p.join("kaleido").canonicalize().unwrap();
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

    #[cfg(target_os = "macos")]
    fn binary_path() -> Result<PathBuf, &'static str> {
        let mut p = Kaleido::root_dir()?;
        p = p.join("kaleido").canonicalize().unwrap();
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

    #[cfg(target_os = "windows")]
    fn binary_path() -> Result<PathBuf, &'static str> {
        let mut p = Kaleido::root_dir()?;
        p = p.join("kaleido.cmd");
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

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

        let p = self.cmd_path.as_path();
        let p = p.to_str().unwrap();
        let p = String::from(p);

        let mut process = Command::new(p.as_str())
            .current_dir(self.cmd_path.parent().unwrap())
            .args([
                "plotly",
                "--disable-gpu",
                "--allow-file-access-from-files",
                "--disable-breakpad",
                "--disable-dev-shm-usage",
                "--disable-software-rasterizer",
                "--single-process",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn Kaleido binary");

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
                let data: Vec<u8> = match format {
                    "svg" | "eps" => image_data.as_bytes().to_vec(),
                    _ => general_purpose::STANDARD.decode(image_data).unwrap(),
                };
                let mut file = File::create(dst.as_path())?;
                file.write_all(&data)?;
                file.flush()?;
            }
        }

        Ok(())
    }

    // similar to save, but returns b64 string
    pub fn to_b64(
        &self,
        plotly_data: &Value,
        format: &str,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {

        let p = self.cmd_path.as_path();
        let p = p.to_str().unwrap();
        let p = String::from(p);

        let mut process = Command::new(p.as_str())
            .current_dir(self.cmd_path.parent().unwrap())
            .args([
                "plotly",
                "--disable-gpu",
                "--allow-file-access-from-files",
                "--disable-breakpad",
                "--disable-dev-shm-usage",
                "--disable-software-rasterizer",
                "--single-process",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn Kaleido binary");

        {
            let plot_data = PlotData::new(plotly_data, format, width, height, scale).to_json();
            let mut process_stdin = process.stdin.take().unwrap();
            process_stdin
                .write_all(plot_data.as_bytes())
                .expect("couldn't write to Kaleido stdin");
            process_stdin.flush()?;
        }

        let output_lines = BufReader::new(process.stdout.take().unwrap()).lines();

        let mut b64_str: String = "".into();
        for line in output_lines.map_while(Result::ok) {
            let res = KaleidoResult::from(line.as_str());
            if let Some(image_data) = res.result {
                b64_str = image_data
            }
        }

        Ok(b64_str)
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
    fn test_can_find_kaleido_executable() {
        let _k = Kaleido::new();
    }

    #[test]
    fn test_plot_data_to_json() {
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
    #[cfg(target_os = "linux")]
    #[test]
    fn test_save_png() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.png");
        let r = k.save(dst.as_path(), &test_plot, "png", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_save_jpeg() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.jpeg");
        let r = k.save(dst.as_path(), &test_plot, "jpeg", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_save_webp() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.webp");
        let r = k.save(dst.as_path(), &test_plot, "webp", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_save_svg() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.svg");
        let r = k.save(dst.as_path(), &test_plot, "svg", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This seems to fail unpredictably on MacOs.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_save_pdf() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.pdf");
        let r = k.save(dst.as_path(), &test_plot, "pdf", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    // This doesn't work for some reason
    #[test]
    #[ignore]
    fn test_save_eps() {
        let test_plot = create_test_plot();
        let k = Kaleido::new();
        let dst = PathBuf::from("example.eps");
        let r = k.save(dst.as_path(), &test_plot, "eps", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }
}
