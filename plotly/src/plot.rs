#[cfg(feature = "kaleido")]
extern crate plotly_kaleido;

use askama::Template;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Layout;
use rand_distr::Alphanumeric;

const PLOTLY_JS: &str = "plotly-2.8.3.min.js";

#[derive(Template)]
#[template(path = "plotly-2.8.3.min.js", escape = "none")]
struct PlotlyJs;

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
struct PlotTemplate<'a> {
    plot_data: &'a str,
    plotly_javascript: &'a str,
    remote_plotly_js: bool,
    export_image: bool,
    image_type: &'a str,
    image_width: usize,
    image_height: usize,
}

#[derive(Template)]
#[template(path = "inline_plot.html", escape = "none")]
struct InlinePlotTemplate<'a> {
    plot_data: &'a str,
    plot_div_id: &'a str,
}

#[derive(Template)]
#[template(path = "jupyter_notebook_plot.html", escape = "none")]
struct JupyterNotebookPlotTemplate<'a> {
    plot_data: &'a str,
    plot_div_id: &'a str,
}

/// Image format for static image export.
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
    SVG,
    PDF,
    EPS,
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
///     let mut plot = Plot::new();
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
#[derive(Default)]
pub struct Plot {
    traces: Vec<Box<dyn Trace>>,
    layout: Option<Layout>,
    remote_plotly_js: bool,
}

const DEFAULT_HTML_APP_NOT_FOUND: &str = r#"Could not find default application for HTML files.
Consider using the `to_html` method to save the plot instead. If using the `kaleido` feature the
`save` method can be used to produce a static image in one of the following formats:
- ImageFormat::PNG
- ImageFormat::JPEG
- ImageFormat::WEBP
- ImageFormat::SVG
- ImageFormat::PDF
- ImageFormat::EPS

used as follows:
let plot = Plot::new();
...
let width = 1024;
let height = 680;
let scale = 1.0;
plot.save("filename", ImageFormat::PNG, width, height, scale);

See https://igiagkiozis.github.io/plotly/content/getting_started.html for further details.
"#;

impl Plot {
    /// Create a new `Plot`.
    pub fn new() -> Plot {
        Plot {
            traces: Vec::with_capacity(1),
            remote_plotly_js: true,
            ..Default::default()
        }
    }

    /// This option results in the plotly.js library being written directly in the html output. The benefit is that the
    /// plot will load faster in the browser and the downside is that the resulting html will be much larger.
    pub fn use_local_plotly(&mut self) {
        self.remote_plotly_js = false;
    }

    /// Add a `Trace` to the `Plot`.
    pub fn add_trace(&mut self, trace: Box<dyn Trace>) {
        self.traces.push(trace);
    }

    /// Add multiple `Trace`s to the `Plot`.
    pub fn add_traces(&mut self, traces: Vec<Box<dyn Trace>>) {
        for trace in traces {
            self.add_trace(trace);
        }
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

        let mut plot_name = String::from_utf8(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(22)
                .collect::<Vec<u8>>(),
        )
        .unwrap();
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

        let mut plot_name = String::from_utf8(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(22)
                .collect::<Vec<u8>>(),
        )
        .unwrap();
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

        let mut plot_name: String = String::from_utf8(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(22)
                .collect::<Vec<u8>>(),
        )
        .unwrap();
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

    /// Renders the contents of the `Plot` and returns it as a String, for embedding in
    /// web-pages or Jupyter notebooks. A `div` is generated with the supplied id followed by the
    /// script that generates the plot. The assumption is that plotly.js is available within the
    /// html page that this element is embedded. If that assumption is violated then the plot will
    /// not be displayed.
    ///
    /// If `plot_div_id` is `None` the plot div id will be randomly generated, otherwise the user
    /// supplied div id is used.
    pub fn to_inline_html<T: Into<Option<&'static str>>>(&self, plot_div_id: T) -> String {
        let plot_div_id = plot_div_id.into();
        match plot_div_id {
            Some(id) => self.render_inline(id.as_ref()),
            None => {
                let rand_id = String::from_utf8(
                    thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(20)
                        .collect::<Vec<u8>>(),
                )
                .unwrap();
                self.render_inline(rand_id.as_str())
            }
        }
    }

    fn to_jupyter_notebook_html(&self) -> String {
        let plot_div_id = String::from_utf8(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(20)
                .collect::<Vec<u8>>(),
        )
        .unwrap();
        let plot_data = self.render_plot_data();

        let tmpl = JupyterNotebookPlotTemplate {
            plot_data: plot_data.as_str(),
            plot_div_id: plot_div_id.as_str(),
        };
        tmpl.render().unwrap()
    }

    /// Display plot in Jupyter Notebook.
    pub fn notebook_display(&self) {
        let plot_data = self.to_jupyter_notebook_html();
        println!(
            "EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT",
            plot_data
        );
    }

    /// Display plot in Jupyter Lab.
    pub fn lab_display(&self) {
        let plot_data = self.to_json();
        println!(
            "EVCXR_BEGIN_CONTENT application/vnd.plotly.v1+json\n{}\nEVCXR_END_CONTENT",
            plot_data
        );
    }

    /// Displays the plot in Jupyter Lab; if running a Jupyter Notebook then use the
    /// `notebook_display()` method instead.
    pub fn evcxr_display(&self) {
        self.lab_display();
    }

    /// Saves the `Plot` to the selected image format.
    #[cfg(feature = "kaleido")]
    pub fn save<P: AsRef<Path>>(
        &self,
        filename: P,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) {
        let kaleido = plotly_kaleido::Kaleido::new();
        let plot_data = self.to_json();
        let image_format = match format {
            ImageFormat::PNG => "png",
            ImageFormat::JPEG => "jpeg",
            ImageFormat::SVG => "svg",
            ImageFormat::PDF => "pdf",
            ImageFormat::EPS => "eps",
            ImageFormat::WEBP => "webp",
        };
        kaleido
            .save(
                filename.as_ref(),
                plot_data.as_str(),
                image_format,
                width,
                height,
                scale,
            )
            .unwrap_or_else(|_| panic!("failed to export plot to {:?}", filename.as_ref()));
    }

    fn plotly_js_path() -> PathBuf {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let templates = root.join("templates");
        templates.join(PLOTLY_JS)
    }

    fn render_plot_data(&self) -> String {
        let mut plot_data = String::new();
        for (idx, trace) in self.traces.iter().enumerate() {
            let s = trace.serialize();
            plot_data.push_str(format!("var trace_{} = {};\n", idx, s).as_str());
        }
        // plot_data.push_str("\n");
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
            Some(layout) => format!("var layout = {};", Trace::serialize(layout)),
            None => {
                let mut s = String::from("var layout = {");
                s.push_str("};");
                s
            }
        };
        plot_data.push_str(layout_data.as_str());
        plot_data
    }

    fn render(
        &self,
        export_image: bool,
        image_type: &str,
        image_width: usize,
        image_height: usize,
    ) -> String {
        let plot_data = self.render_plot_data();
        let plotly_js = PlotlyJs {}.render().unwrap();
        let tmpl = PlotTemplate {
            plot_data: plot_data.as_str(),
            plotly_javascript: plotly_js.as_str(),
            remote_plotly_js: self.remote_plotly_js,
            export_image,
            image_type,
            image_width,
            image_height,
        };
        tmpl.render().unwrap()
    }

    fn render_inline(&self, plot_div_id: &str) -> String {
        let plot_data = self.render_plot_data();

        let tmpl = InlinePlotTemplate {
            plot_data: plot_data.as_str(),
            plot_div_id,
        };
        tmpl.render().unwrap()
    }

    pub fn to_json(&self) -> String {
        let mut plot_data: Vec<String> = Vec::new();
        for trace in self.traces.iter() {
            let s = trace.serialize();
            plot_data.push(s);
        }
        let layout_data = match &self.layout {
            Some(layout) => Trace::serialize(layout),
            None => "{}".to_owned(),
        };

        let mut json_data = String::new();
        json_data.push_str(r#"{"data": ["#);

        for (index, data) in plot_data.iter().enumerate() {
            if index < plot_data.len() - 1 {
                json_data.push_str(data);
                json_data.push_str(r#","#);
            } else {
                json_data.push_str(data);
                json_data.push_str("]");
            }
        }
        json_data.push_str(format!(r#", "layout": {}"#, layout_data).as_str());
        json_data.push_str("}");
        json_data
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
        Command::new("open")
            .args(&[temp_path])
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
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
        let mut plot = Plot::new();
        plot.add_trace(trace1);
        plot
    }

    #[test]
    fn test_to_json() {
        let plot = create_test_plot();
        let plot_json = plot.to_json();
        println!("{}", plot_json);
    }

    #[test]
    fn test_inline_plot() {
        let plot = create_test_plot();
        let inline_plot_data = plot.to_inline_html("replace_this_with_the_div_id");
        assert!(inline_plot_data.contains("replace_this_with_the_div_id"));
        println!("{}", inline_plot_data);
        let random_div_id = plot.to_inline_html(None);
        println!("{}", random_div_id);
    }

    #[test]
    fn test_jupyter_notebook_plot() {
        let plot = create_test_plot();
        let inline_plot_data = plot.to_jupyter_notebook_html();
        println!("{}", inline_plot_data);
    }

    #[test]
    fn test_notebook_display() {
        let plot = create_test_plot();
        plot.notebook_display();
    }

    #[test]
    fn test_lab_display() {
        let plot = create_test_plot();
        plot.lab_display();
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_png() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.png");
        plot.save(&dst, ImageFormat::PNG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_jpeg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.jpeg");
        plot.save(&dst, ImageFormat::JPEG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_svg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.svg");
        plot.save(&dst, ImageFormat::SVG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "kaleido")]
    fn test_save_to_eps() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.eps");
        plot.save(&dst, ImageFormat::EPS, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_pdf() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.pdf");
        plot.save(&dst, ImageFormat::PDF, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_webp() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.webp");
        plot.save(&dst, ImageFormat::WEBP, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }
}
