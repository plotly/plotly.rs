#[cfg(feature = "orca")]
extern crate plotly_orca;

use askama::Template;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Layout;

const PLOTLY_JS: &str = "plotly-1.52.2.min.js";

#[derive(Template)]
#[template(path = "plotly-1.52.2.min.js", escape = "none")]
struct PlotlyJs;

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
struct PlotTemplate<'a> {
    plot_data: &'a str,
    plotly_javascript: &'a str,
    export_image: bool,
    image_type: &'a str,
    image_width: usize,
    image_height: usize,
}

/// A struct that implements `Trace` can be serialized to json format that is understood by Plotly.js.
pub trait Trace {
    fn serialize(&self) -> String;
}

/// Plot is a container for structs that implement the `Trace` trait. Optionally a `Layout` can
/// also be specified. Its function is to serialize `Trace`s and the `Layout` in html format and
/// display and/or persist the resulting plot.
///
/// # Examples
///
/// ```
/// extern crate plotly;
/// use plotly::common::Mode;
/// use plotly::{Plot, Scatter};
///
/// fn line_and_scatter_plot() {
///     let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
///         .name("trace1")
///         .mode(Mode::Markers);
///     let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
///         .name("trace2")
///         .mode(Mode::Lines);
///     let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");
///
///     let mut plot = Plot::default();
///     plot.add_trace(trace1);
///     plot.add_trace(trace2);
///     plot.add_trace(trace3);
///     plot.show();
/// }
///
/// fn main() -> std::io::Result<()> {
///     line_and_scatter_plot();
///     Ok(())
/// }
/// ```
pub struct Plot {
    traces: Vec<Box<dyn Trace>>,
    layout: Option<Layout>,
}

impl Default for Plot {
    fn default() -> Self {
        Self {
            traces: Vec::with_capacity(1),
            layout: None,
        }
    }
}

const DEFAULT_HTML_APP_NOT_FOUND: &str = r#"Could not find default application for HTML files.
Consider using the `to_html` method to save the plot instead. If using the `orca` feature the following
additional formats are available accessed by following methods:
- to_png
- to_jpeg
- to_webp
- to_svg
- to_pdf
- to_eps
"#;

impl Plot {
    /// Create a new `Plot`.
    pub fn new() -> Plot {
        Plot {
            traces: Vec::with_capacity(1),
            layout: None,
        }
    }

    /// Add a `Trace` to the `Plot`.
    pub fn add_trace(&mut self, trace: Box<dyn Trace>) {
        self.traces.push(trace);
    }

    /// Set the `Layout` to be used by `Plot`.
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
    }

    /// Renders the contents of the `Plot` and displays them in the system default browser.
    ///
    /// This will serialize the `Trace`s and `Layout` in an html page which is saved in the temp
    /// directory. For example on Linux it will generate a file `plotly_<22 random characters>.html`
    /// in the /tmp directory.
    pub fn show(&self) {
        let rendered = self.render(false, "", 0, 0);
        let rendered = rendered.as_bytes();
        let mut temp = env::temp_dir();

        let mut plot_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_name.push_str(".html");
        plot_name = format!("plotly_{}", plot_name);

        temp.push(plot_name);
        let temp_path = temp.to_str().unwrap();
        {
            let mut file = File::create(temp_path).unwrap();
            file.write_all(rendered)
                .expect("failed to write html output");
            file.flush().unwrap();
        }

        Plot::show_with_default_app(temp_path);
    }

    /// Renders the contents of the `Plot`, creates a png raster and displays it in the system default browser.
    ///
    /// To save the resulting png right-click on the resulting image and select `Save As...`.
    pub fn show_png(&self, width: usize, height: usize) {
        let rendered = self.render(true, "png", width, height);
        let rendered = rendered.as_bytes();
        let mut temp = env::temp_dir();

        let mut plot_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_name.push_str(".html");

        temp.push(plot_name);
        let temp_path = temp.to_str().unwrap();
        {
            let mut file = File::create(temp_path).unwrap();
            file.write_all(rendered)
                .expect("failed to write html output");
            file.flush().unwrap();
        }

        Plot::show_with_default_app(temp_path);
    }

    /// Renders the contents of the `Plot`, creates a jpeg raster and displays it in the system default browser.
    ///
    /// To save the resulting png right-click on the resulting image and select `Save As...`.
    pub fn show_jpeg(&self, width: usize, height: usize) {
        let rendered = self.render(true, "jpg", width, height);
        let rendered = rendered.as_bytes();
        let mut temp = env::temp_dir();

        let mut plot_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_name.push_str(".html");

        temp.push(plot_name);
        let temp_path = temp.to_str().unwrap();
        {
            let mut file = File::create(temp_path).unwrap();
            file.write_all(rendered)
                .expect("failed to write html output");
            file.flush().unwrap();
        }

        Plot::show_with_default_app(temp_path);
    }

    /// Renders the contents of the `Plot` and displays it in the system default browser.
    ///
    /// In contrast to `Plot::show()` this will save the resulting html in a user specified location
    /// instead of the system temp directory.
    pub fn to_html<P: AsRef<Path>>(&self, filename: P) {
        let rendered = self.render(false, "", 0, 0);
        let rendered = rendered.as_bytes();
        let mut file = File::create(filename.as_ref()).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");
    }

    /// Saves the `Plot` to png format.
    #[cfg(feature = "orca")]
    pub fn to_png<P: AsRef<Path>>(&self, filename: P, width: usize, height: usize) {
        let orca = plotly_orca::Orca::from(Plot::plotly_js_path());
        let rendered = self.render_orca_format();
        orca.save_png(filename.as_ref(), &rendered, width, height);
    }

    /// Saves the `Plot` to jpeg format.
    #[cfg(feature = "orca")]
    pub fn to_jpeg<P: AsRef<Path>>(&self, filename: P, width: usize, height: usize) {
        let orca = plotly_orca::Orca::from(Plot::plotly_js_path());
        let rendered = self.render_orca_format();
        orca.save_jpeg(filename.as_ref(), &rendered, width, height);
    }

    /// Saves the `Plot` to webp format.
    #[cfg(feature = "orca")]
    pub fn to_webp<P: AsRef<Path>>(&self, filename: P, width: usize, height: usize) {
        let orca = plotly_orca::Orca::from(Plot::plotly_js_path());
        let rendered = self.render_orca_format();
        orca.save_webp(filename.as_ref(), &rendered, width, height);
    }

    /// Saves the `Plot` to svg format.
    #[cfg(feature = "orca")]
    pub fn to_svg<P: AsRef<Path>>(&self, filename: P, width: usize, height: usize) {
        let orca = plotly_orca::Orca::from(Plot::plotly_js_path());
        let rendered = self.render_orca_format();
        orca.save_svg(filename.as_ref(), &rendered, width, height);
    }

    /// Saves the `Plot` to pdf format.
    #[cfg(feature = "orca")]
    pub fn to_pdf<P: AsRef<Path>>(&self, filename: P, width: usize, height: usize) {
        let orca = plotly_orca::Orca::from(Plot::plotly_js_path());
        let rendered = self.render_orca_format();
        orca.save_pdf(filename.as_ref(), &rendered, width, height);
    }

    /// Saves the `Plot` to eps format.
    #[cfg(feature = "orca")]
    pub fn to_eps<P: AsRef<Path>>(&self, filename: P, width: usize, height: usize) {
        let orca = plotly_orca::Orca::from(Plot::plotly_js_path());
        let rendered = self.render_orca_format();
        orca.save_eps(filename.as_ref(), &rendered, width, height);
    }

    fn plotly_js_path() -> PathBuf {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let templates = root.join("templates");
        templates.join(PLOTLY_JS)
    }

    fn render(
        &self,
        export_image: bool,
        image_type: &str,
        image_width: usize,
        image_height: usize,
    ) -> String {
        let mut plot_data = String::new();
        for (idx, trace) in self.traces.iter().enumerate() {
            let s = trace.serialize();
            plot_data.push_str(format!("var trace_{} = {};\n", idx, s).as_str());
        }
        plot_data.push_str("\n");
        plot_data.push_str("var data = [");
        for idx in 0..self.traces.len() {
            if idx != self.traces.len() - 1 {
                plot_data.push_str(format!("trace_{},", idx).as_str());
            } else {
                plot_data.push_str(format!("trace_{}", idx).as_str());
            }
        }
        plot_data.push_str("];\n");
        let layout_data = match &self.layout {
            Some(layout) => format!("var layout = {};\n", Trace::serialize(layout)),
            None => {
                let mut s = String::from("var layout = {");
                s.push_str("};\n");
                s
            }
        };
        plot_data.push_str(layout_data.as_str());

        let plotly_js = PlotlyJs {}.render().unwrap();
        let tmpl = PlotTemplate {
            plot_data: plot_data.as_str(),
            plotly_javascript: plotly_js.as_str(),
            export_image,
            image_type,
            image_width,
            image_height,
        };
        tmpl.render().unwrap()
    }

    fn render_orca_format(&self) -> String {
        let mut plot_data: Vec<String> = Vec::new();
        for trace in self.traces.iter() {
            let s = trace.serialize();
            plot_data.push(s);
        }
        let layout_data = match &self.layout {
            Some(layout) => Trace::serialize(layout),
            None => "{}".to_owned(),
        };

        let mut orca_data = String::new();
        orca_data.push_str(r#"{"data": ["#);

        for (index, data) in plot_data.iter().enumerate() {
            if index < plot_data.len() - 1 {
                orca_data.push_str(data);
                orca_data.push_str(r#","#);
            } else {
                orca_data.push_str(data);
                orca_data.push_str("]");
            }
        }
        orca_data.push_str(format!(r#", "layout": {}"#, layout_data).as_str());
        orca_data.push_str("}");
        orca_data
    }

    #[cfg(target_os = "linux")]
    fn show_with_default_app(temp_path: &str) {
        Command::new("xdg-open")
            .args(&[temp_path])
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }

    #[cfg(target_os = "macos")]
    fn show_with_default_app(temp_path: &str) {
        Command::new("open").args(&[temp_path]).output().expect(DEFAULT_HTML_APP_NOT_FOUND);
    }

    #[cfg(target_os = "windows")]
    fn show_with_default_app(temp_path: &str) {
        Command::new("cmd")
            .arg("/C")
            .arg(format!(r#"start {}"#, temp_path))
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scatter;

    fn create_test_plot() -> Plot {
        let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2]).name("trace1");
        let mut plot = Plot::default();
        plot.add_trace(trace1);
        plot
    }

    #[test]
    #[cfg(feature = "orca")]
    fn test_to_png() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.png");
        plot.to_png(&dst, 1024, 680);
        assert!(dst.exists());
        std::fs::remove_file(&dst);
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "orca")]
    fn test_to_jpeg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.jpeg");
        plot.to_jpeg(&dst, 1024, 680);
        assert!(dst.exists());
        std::fs::remove_file(&dst);
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "orca")]
    fn test_to_webp() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.webp");
        plot.to_webp(&dst, 1024, 680);
        assert!(dst.exists());
        std::fs::remove_file(&dst);
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "orca")]
    fn test_to_svg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.svg");
        plot.to_svg(&dst, 1024, 680);
        assert!(dst.exists());
        std::fs::remove_file(&dst);
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "orca")]
    fn test_to_pdf() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.pdf");
        plot.to_pdf(&dst, 1024, 680);
        assert!(dst.exists());
        std::fs::remove_file(&dst);
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "orca")]
    fn test_to_eps() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.eps");
        plot.to_eps(&dst, 1024, 680);
        assert!(dst.exists());
        std::fs::remove_file(&dst);
        assert!(!dst.exists());
    }
}
