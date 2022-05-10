#[cfg(feature = "kaleido")]
extern crate plotly_kaleido;

use askama::Template;
use dyn_clone::DynClone;
use erased_serde::Serialize as ErasedSerialize;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{Configuration, Layout};
use rand_distr::Alphanumeric;

#[derive(Template)]
#[template(path = "plotly.min.js", escape = "none")]
struct PlotlyJs;

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
struct PlotTemplate<'a> {
    plot: &'a Plot,
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
    plot: &'a Plot,
    plot_div_id: &'a str,
}

#[derive(Template)]
#[template(path = "jupyter_notebook_plot.html", escape = "none")]
struct JupyterNotebookPlotTemplate<'a> {
    plot: &'a Plot,
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
pub trait Trace: DynClone + ErasedSerialize {
    fn to_json(&self) -> String;
}

dyn_clone::clone_trait_object!(Trace);
erased_serde::serialize_trait_object!(Trace);

#[derive(Default, Serialize, Clone)]
#[serde(transparent)]
pub struct Traces {
    traces: Vec<Box<dyn Trace>>,
}

impl Traces {
    pub fn new() -> Self {
        Self {
            traces: Vec::with_capacity(1),
        }
    }

    pub fn push(&mut self, trace: Box<dyn Trace>) {
        self.traces.push(trace)
    }

    pub fn len(&self) -> usize {
        self.traces.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Trace>> {
        self.traces.iter()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
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
/// use plotly::{Layout, Plot, Scatter};
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
///
///     let layout = Layout::new().title("<b>Line and Scatter Plot</b>".into());
///     plot.set_layout(layout);
///
///     plot.show();
/// }
///
/// fn main() -> std::io::Result<()> {
///     line_and_scatter_plot();
///     Ok(())
/// }
/// ```
#[derive(Default, Serialize, Clone)]
pub struct Plot {
    #[serde(rename = "data")]
    traces: Traces,
    layout: Layout,
    #[serde(rename = "config")]
    configuration: Configuration,
    #[serde(skip)]
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
            traces: Traces::new(),
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
        self.layout = layout;
    }

    /// Set the `Configuration` to be used by `Plot`.
    pub fn set_configuration(&mut self, configuration: Configuration) {
        self.configuration = configuration;
    }

    /// Get the contained data elements.
    pub fn data(&self) -> &Traces {
        &self.traces
    }

    /// Get the layout specification of the plot.
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Get the configuration specification of the plot.
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    /// Renders the contents of the `Plot` and displays them in the system default browser.
    ///
    /// This will serialize the `Trace`s and `Layout` in an html page which is saved in the temp
    /// directory. For example on Linux it will generate a file `plotly_<22 random characters>.html`
    /// in the /tmp directory.
    #[cfg(not(feature = "wasm"))]
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
    #[cfg(not(feature = "wasm"))]
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
    #[cfg(not(feature = "wasm"))]
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
    ///
    /// In contrast to `Plot::write_html`, this will save the resulting html to a file located at a
    /// user specified location, instead of a writing it to anything that implements `std::io::Write`.
    pub fn to_html<P: AsRef<Path>>(&self, filename: P) {
        let mut file = File::create(filename.as_ref()).unwrap();

        self.write_html(&mut file);
    }

    /// Renders the contents of the `Plot` to HTML and outputs them to a writable buffer.
    ///
    /// In contrast to `Plot::to_html`, this will save the resulting html to a byte buffer using the
    /// `std::io::Write` trait, instead of to a user specified file.
    pub fn write_html<W: Write>(&self, buffer: &mut W) {
        let rendered = self.render(false, "", 0, 0);
        let rendered = rendered.as_bytes();

        buffer
            .write_all(rendered)
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

        let tmpl = JupyterNotebookPlotTemplate {
            plot: self,
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
                &serde_json::to_value(self).unwrap(),
                image_format,
                width,
                height,
                scale,
            )
            .unwrap_or_else(|_| panic!("failed to export plot to {:?}", filename.as_ref()));
    }

    fn render(
        &self,
        export_image: bool,
        image_type: &str,
        image_width: usize,
        image_height: usize,
    ) -> String {
        let plotly_js = PlotlyJs {}.render().unwrap();
        let tmpl = PlotTemplate {
            plot: self,
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
        let tmpl = InlinePlotTemplate {
            plot: self,
            plot_div_id,
        };
        tmpl.render().unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[cfg(feature = "wasm")]
    /// Convert a `Plot` to a native Javasript `js_sys::Object`.
    pub fn to_js_object(&self) -> js_sys::Object {
        use wasm_bindgen::JsCast;
        // The only reason this could fail is if to_json() produces structurally incorrect JSON. That
        // would be a bug, and would require fixing in the to_json()/serialization methods, rather than here
        js_sys::JSON::parse(&self.to_json())
            .expect("Invalid JSON")
            .dyn_into::<js_sys::Object>()
            .expect("Invalid JSON structure - expected an top-level Object")
    }

    #[cfg(target_os = "linux")]
    fn show_with_default_app(temp_path: &str) {
        use std::process::Command;
        Command::new("xdg-open")
            .args(&[temp_path])
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }

    #[cfg(target_os = "macos")]
    fn show_with_default_app(temp_path: &str) {
        use std::process::Command;
        Command::new("open")
            .args(&[temp_path])
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }

    #[cfg(target_os = "windows")]
    fn show_with_default_app(temp_path: &str) {
        use std::process::Command;
        Command::new("cmd")
            .arg("/C")
            .arg(format!(r#"start {}"#, temp_path))
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }
}

impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.to_json() == other.to_json()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::Scatter;

    fn create_test_plot() -> Plot {
        let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2]).name("trace1");
        let mut plot = Plot::new();
        plot.add_trace(trace1);
        plot
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
    fn test_plot_serialize_simple() {
        let plot = create_test_plot();
        let expected = json!({
            "data": [
                {
                    "type": "scatter",
                    "name": "trace1",
                    "x": [0, 1, 2],
                    "y": [6, 10, 2]
                }
            ],
            "layout": {},
            "config": {},
        });

        assert_eq!(to_value(plot).unwrap(), expected);
    }

    #[test]
    fn test_plot_serialize_with_layout() {
        let mut plot = create_test_plot();
        let layout = Layout::new().title("Title".into());
        plot.set_layout(layout);

        let expected = json!({
            "data": [
                {
                    "type": "scatter",
                    "name": "trace1",
                    "x": [0, 1, 2],
                    "y": [6, 10, 2]
                }
            ],
            "layout": {
                "title": {
                    "text": "Title"
                }
            },
            "config": {},
        });

        assert_eq!(to_value(plot).unwrap(), expected);
    }

    #[test]
    fn test_data_to_json() {
        let plot = create_test_plot();
        let expected = json!([
            {
                "type": "scatter",
                "name": "trace1",
                "x": [0, 1, 2],
                "y": [6, 10, 2]
            }
        ]);

        assert_eq!(to_value(plot.data()).unwrap(), expected);
    }

    #[test]
    fn test_empty_layout_to_json() {
        let plot = create_test_plot();
        let expected = json!({});

        assert_eq!(to_value(plot.layout()).unwrap(), expected);
    }

    #[test]
    fn test_layout_to_json() {
        let mut plot = create_test_plot();
        let layout = Layout::new().title("TestTitle".into());
        plot.set_layout(layout);

        let expected = json!({
            "title": {"text": "TestTitle"}
        });

        assert_eq!(to_value(plot.layout()).unwrap(), expected);
    }

    #[test]
    fn test_plot_eq() {
        let plot1 = create_test_plot();
        let plot2 = create_test_plot();

        assert!(plot1 == plot2);
    }

    #[test]
    fn test_plot_neq() {
        let plot1 = create_test_plot();
        let trace2 = Scatter::new(vec![10, 1, 2], vec![6, 10, 2]).name("trace2");
        let mut plot2 = Plot::new();
        plot2.add_trace(trace2);

        assert!(plot1 != plot2);
    }

    #[test]
    fn test_plot_clone() {
        let plot1 = create_test_plot();
        let plot2 = plot1.clone();

        assert!(plot1 == plot2);
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
