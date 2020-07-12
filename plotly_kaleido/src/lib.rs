//! # Plotly Kaleido
//! Plotly Kaleido implements the `kaleido` feature for [Plotly for Rust](https://github.com/igiagkiozis/plotly)
//!
//! The `kaleido` feature enables `Plot` conversion to the following output formats: png, jpeg, webp, svg, pdf and eps.
//! It has the added benefit over the `orca` feature in that there is no need for an additional installation for the
//! feature to function properly. The Kaleido precompiled binaries are packaged redistributed from the original
//! repository [plotly/Kaleido](https://github.com/plotly/Kaleido).
//!
//! Note that [plotly/Kaleido](https://github.com/plotly/Kaleido) is still in pre-release and as such the `kaleido`
//! feature should be considered in pre-release mode as well.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Serialize)]
struct PlotData {
    format: String,
    width: usize,
    height: usize,
    scale: f64,
    data: String,
}

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

impl PlotData {
    fn new(data: &str, format: &str, width: usize, height: usize, scale: f64) -> PlotData {
        PlotData {
            format: String::from(format),
            width,
            height,
            scale,
            data: String::from(data),
        }
    }

    fn to_json(&self) -> String {
        let data = format!(
            r##"{{"format":"{}","width":{},"height":{},"scale":{},"data":{}}}"##,
            self.format, self.width, self.height, self.scale, self.data
        );
        data.replace(" ", "")
            .replace("\n", "")
            .replace("\t", "")
            .replace("\r", "")
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
            Err(msg) => panic!(msg),
        };

        Kaleido { cmd_path: path }
    }

    fn root_dir() -> Result<PathBuf, &'static str> {
        let mut p = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        p = p.parent().unwrap().to_path_buf();
        p = p.join("plotly_kaleido");
        Ok(p)
    }

    #[cfg(target_os = "linux")]
    fn binary_path() -> Result<PathBuf, &'static str> {
        let mut p = Kaleido::root_dir()?;
        p = p
            .join("kaleido")
            .join("linux")
            .join("kaleido")
            .canonicalize()
            .unwrap();
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

    #[cfg(target_os = "macos")]
    fn binary_path() -> Result<PathBuf, &'static str> {
        let mut p = Kaleido::root_dir()?;
        p = p
            .join("kaleido")
            .join("macos")
            .join("kaleido")
            .canonicalize()
            .unwrap();
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

    #[cfg(target_os = "windows")]
    fn binary_path() -> Result<PathBuf, &'static str> {
        let mut p = Kaleido::root_dir()?;
        p = p.join("kaleido").join("windows").join("kaleido.cmd");
        if !p.exists() {
            return Err("could not find kaleido executable in path");
        }
        Ok(p)
    }

    pub fn save(
        &self,
        dst: &Path,
        plotly_data: &str,
        image_format: &str,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut dst = PathBuf::from(dst);
        dst.set_extension(image_format);

        let p = self.cmd_path.as_path();
        let p = p.to_str().unwrap();
        let p = String::from(p);

        let process = Command::new(p.as_str())
            .current_dir(self.cmd_path.parent().unwrap())
            .args(&["plotly", "--disable-gpu"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn Kaleido binary");

        {
            let plot_data =
                PlotData::new(plotly_data, image_format, width, height, scale).to_json();
            let mut process_stdin = process.stdin.unwrap();
            process_stdin
                .write_all(plot_data.as_bytes())
                .expect("couldn't write to Kaleido stdin");
            process_stdin.flush()?;
        }

        let output_lines = BufReader::new(process.stdout.unwrap()).lines();
        for line in output_lines {
            if let Ok(l) = line {
                let res = KaleidoResult::from(l.as_str());
                if let Some(image_data) = res.result {
                    let data: Vec<u8> = match image_format {
                        "svg" | "eps" => image_data.as_bytes().to_vec(),
                        _ => base64::decode(image_data).unwrap(),
                    };
                    let mut file = File::create(dst.as_path())?;
                    file.write_all(&data)?;
                    file.flush()?;
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::{Path, PathBuf};

    const TEST_PLOT: &str = r#"{
    "data": [{"type":"scatter","x":[1,2,3,4],"y":[10,15,13,17],"name":"trace1","mode":"markers"},
             {"type":"scatter","x":[2,3,4,5],"y":[16,5,11,9],"name":"trace2","mode":"lines"},
             {"type":"scatter","x":[1,2,3,4],"y":[12,9,15,12],"name":"trace3"}],
    "layout": {}
    }"#;

    #[test]
    fn test_can_find_kaleido_executable() {
        let _k = Kaleido::new();
    }

    #[test]
    fn test_plot_data_to_json() {
        let d = PlotData::new(TEST_PLOT, "png", 400, 500, 1.);
        println!("{}", d.to_json());
    }

    #[test]
    fn test_save_png() {
        let k = Kaleido::new();
        let dst = PathBuf::from("example.png");
        let r = k.save(dst.as_path(), TEST_PLOT, "png", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn test_save_jpeg() {
        let k = Kaleido::new();
        let dst = PathBuf::from("example.jpeg");
        let r = k.save(dst.as_path(), TEST_PLOT, "jpeg", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn test_save_webp() {
        let k = Kaleido::new();
        let dst = PathBuf::from("example.webp");
        let r = k.save(dst.as_path(), TEST_PLOT, "webp", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn test_save_svg() {
        let k = Kaleido::new();
        let dst = PathBuf::from("example.svg");
        let r = k.save(dst.as_path(), TEST_PLOT, "svg", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    fn test_save_pdf() {
        let k = Kaleido::new();
        let dst = PathBuf::from("example.pdf");
        let r = k.save(dst.as_path(), TEST_PLOT, "pdf", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    fn test_save_eps() {
        let k = Kaleido::new();
        let dst = PathBuf::from("example.eps");
        let r = k.save(dst.as_path(), TEST_PLOT, "eps", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }
}
