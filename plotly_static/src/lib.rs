use anyhow::Context;
use anyhow::Result;
use askama::Template;
use plotly::ImageFormat;
use plotly::Plot;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use base64::{engine::general_purpose, Engine as _};
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
pub struct PlotlyStatic {
    cmd_path: PathBuf,
}

impl PlotlyStatic {
    pub fn new() -> Self {
        Self::default()
    }
    /// Generate a static image from a Plotly graph and save it to a file
    pub fn save(
        &self,
        dst: &Path,
        plot: &Plot,
        format: &ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut dst = PathBuf::from(dst);
        dst.set_extension(format.to_string());

        let image_data = self.export(plot, &format, width, height, scale)?;
        let data = match format {
            ImageFormat::EPS | ImageFormat::SVG => image_data.as_bytes(),
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
        plot: &Plot,
        format: &ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let image_data = self.export(plot, format, width, height, scale)?;
        Ok(image_data)
    }

    /// Convert the Plotly graph to a static image using Kaleido and return the
    /// result as a String
    pub fn export(
        &self,
        plot: &Plot,
        format: &ImageFormat,
        width: usize,
        height: usize,
        _scale: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use tokio::runtime::Runtime;
        let file = self.generate_static_html_plot(plot, format, width, height)?;
        dbg!(&file);

        let s = Runtime::new()
            .expect("Failed to create Tokio runtime")
            .block_on(self.extract(&file, format))?;
        dbg!(&s);
        Ok(s)
    }

    fn generate_static_html_plot(
        &self,
        plot: &Plot,
        format: &ImageFormat,
        width: usize,
        height: usize,
    ) -> Result<PathBuf> {
        use std::env;

        let rendered = plot.render_static(format, width, height);

        // Set up the temp file with a unique filename.
        let mut tmp_path = env::temp_dir();
        let mut plot_name = Alphanumeric.sample_string(&mut rng(), 22);
        plot_name.push_str(".html");
        plot_name = format!("plotly_{}", plot_name);
        tmp_path.push(plot_name);

        // Save the rendered plot to the temp file.
        let temp_path = tmp_path
            .to_str()
            .context("Failed to convert path to string")?;
        let mut file = File::create(temp_path)?;
        file.write_all(rendered.as_bytes())?;
        file.flush()?;
        Ok(tmp_path)
    }

    async fn extract(&self, file_path: &PathBuf, format: &ImageFormat) -> Result<String> {
        use fantoccini::{ClientBuilder, Locator};
        use std::time::Duration;
        use tokio::time::sleep;
        let client = ClientBuilder::native()
            // .capabilities(cap)
            .connect("http://localhost:4444")
            .await?;

        // Go to the Rust website.
        // client.goto("https://www.rust-lang.org/").await?;
        let url = format!("file:{}", file_path.display());
        client.goto(&url).await?;

        // Let the user marvel at what we achieved.
        // Then close the browser window.
        println!("here");
        // Extract image URLs

        // find the source for the Wikipedia globe
        let img = client.find(Locator::XPath(r#"/html/body/div/img"#)).await?;
        dbg!(&img);
        let mut src = None;
        while src.is_none() {
            println!("trying to get the output returned by PlotlyJS in the 'src' attribute");
            src = img.attr("src").await?; //.expect("image should have a src");
                                          // now build a raw HTTP client request (which also has all current cookies)
            if src.is_none() {
                println!("No response or no data; trying again in 100 msec");
                sleep(Duration::from_millis(100)).await;
            }

            // sshould stop after a number of retries
        }
        println!("Found data!!!");
        println!("Decoding!!!");
        // dbg!(&src);
        let src = src.unwrap();
        // let raw = img.client().raw_client_for(http::Method::GET, &src).await?;

        let bytes = src.split_once(";");

        let t = bytes.unwrap().0;
        let d = bytes.unwrap().1;
        let dd = d.split_once(",");
        let b = dd.unwrap().0;
        // dbg!(b);
        let dd = dd.unwrap().1;
        dbg!(dd);
        client.close().await?;
        Ok(dd.to_string())
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

    /*
    fn can_find_kaleido_executable() {
        let _k = PlotlyStatic::new();
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
     */

    // For MacOS failures, see issue #241 and upstream https://github.com/plotly/Kaleido/issues/323 is resolved
    use ndarray::Array;

    #[test]
    fn save_png() {
        // let test_plot = create_test_plot();

        let n: usize = 100;
        let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec_and_offset().0;
        let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
        let z: Vec<f64> = t.iter().map(|x| x.cos()).collect();

        let trace = plotly::Scatter3D::new(t, y, z).mode(plotly::common::Mode::Lines);
        let mut plot = Plot::new();
        plot.add_trace(trace);

        let k = PlotlyStatic::new();
        let dst = PathBuf::from("example.png");
        let r = k.save(dst.as_path(), &plot, &ImageFormat::PNG, 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(dst.exists());
        let metadata = std::fs::metadata(&dst).expect("Could not retrieve file metadata");
        let file_size = metadata.len();
        assert!(file_size > 0,);
        // assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }

    /*
       #[test]
       fn save_jpeg() {
           let test_plot = create_test_plot();
           let k = PlotlyStatic::new();
           let dst = PathBuf::from("example.jpeg");
           let r = k.save(dst.as_path(), &test_plot, "jpeg", 1200, 900, 4.5);
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
           let k = PlotlyStatic::new();
           let dst = PathBuf::from("example.webp");
           let r = k.save(dst.as_path(), &test_plot, "webp", 1200, 900, 4.5);
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
           let k = PlotlyStatic::new();
           let dst = PathBuf::from("example.svg");
           let r = k.save(dst.as_path(), &test_plot, "svg", 1200, 900, 4.5);
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
           let k = PlotlyStatic::new();
           let dst = PathBuf::from("example.pdf");
           let r = k.save(dst.as_path(), &test_plot, "pdf", 1200, 900, 4.5);
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
        let k = PlotlyStatic::new();
        let dst = PathBuf::from("example.eps");
        let r = k.save(dst.as_path(), &test_plot, "eps", 1200, 900, 4.5);
        assert!(r.is_ok());
        assert!(std::fs::remove_file(dst.as_path()).is_ok());
    }
    */
}
