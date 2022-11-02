use std::{fs::File, io::Write, path::Path};

use askama::Template;
use dyn_clone::DynClone;
use erased_serde::Serialize as ErasedSerialize;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::Serialize;

use crate::{Configuration, Layout};

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
struct PlotTemplate<'a> {
    plot: &'a Plot,
    remote_plotly_js: bool,
}

#[derive(Template)]
#[template(path = "static_plot.html", escape = "none")]
struct StaticPlotTemplate<'a> {
    plot: &'a Plot,
    format: ImageFormat,
    remote_plotly_js: bool,
    width: usize,
    height: usize,
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

#[cfg(not(target_family = "wasm"))]
const DEFAULT_HTML_APP_NOT_FOUND: &str = r#"Could not find default application for HTML files.
Consider using the `to_html` method obtain a string representation instead. If using the `kaleido` feature the
`write_image` method can be used to produce a static image in one of the following formats:
- ImageFormat::PNG
- ImageFormat::JPEG
- ImageFormat::WEBP
- ImageFormat::SVG
- ImageFormat::PDF
- ImageFormat::EPS

Used as follows:
let plot = Plot::new();
...
let width = 1024;
let height = 680;
let scale = 1.0;
plot.write_image("filename", ImageFormat::PNG, width, height, scale);

See https://igiagkiozis.github.io/plotly/content/getting_started.html for further details.
"#;

/// Image format for static image export.
#[derive(Debug)]
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

/// A struct that implements `Trace` can be serialized to json format that is
/// understood by Plotly.js.
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

    pub fn is_empty(&self) -> bool {
        self.traces.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Trace>> {
        self.traces.iter()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// Plot is a container for structs that implement the `Trace` trait. Optionally
/// a `Layout` can also be specified. Its function is to serialize `Trace`s and
/// the `Layout` in html format and display and/or persist the resulting plot.
///
/// # Examples
///
/// ```rust
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
///     let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12])
///         .name("trace3");
///
///     let mut plot = Plot::new();
///     plot.add_trace(trace1);
///     plot.add_trace(trace2);
///     plot.add_trace(trace3);
///
///     let layout = Layout::new().title("<b>Line and Scatter Plot</b>".into());
///     plot.set_layout(layout);
///     
///     # if false {  // We don't actually want to try and display the plot in a browser when running a doctest.
///     plot.show();
///     # }
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

impl Plot {
    /// Create a new `Plot`.
    pub fn new() -> Plot {
        Plot {
            traces: Traces::new(),
            remote_plotly_js: true,
            ..Default::default()
        }
    }

    /// This option results in the plotly.js library being written directly in
    /// the html output. The benefit is that the plot will load faster in
    /// the browser and the downside is that the resulting html will be much
    /// larger.
    ///
    /// Note that when using `Plot::to_inline_html()`, it is assumed that the
    /// `plotly.js` library is already in scope, so setting this attribute
    /// will have no effect.
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

    /// Display the fully rendered HTML `Plot` in the default system browser.
    ///
    /// The HTML file is saved in a temp file, from which it is read and
    /// displayed by the browser.
    pub fn show(&self) {
        use std::env;

        let rendered = self.render();

        // Set up the temp file with a unique filename.
        let mut temp = env::temp_dir();
        let mut plot_name = Alphanumeric.sample_string(&mut thread_rng(), 22);
        plot_name.push_str(".html");
        plot_name = format!("plotly_{}", plot_name);
        temp.push(plot_name);

        // Save the rendered plot to the temp file.
        let temp_path = temp.to_str().unwrap();
        let mut file = File::create(temp_path).unwrap();
        file.write_all(rendered.as_bytes())
            .expect("failed to write html output");
        file.flush().unwrap();

        // Hand off the job of opening the browser to an OS-specific implementation.
        Plot::show_with_default_app(temp_path);
    }

    /// Display the fully rendered `Plot` as a static image of the given format
    /// in the default system browser.
    #[cfg(not(target_family = "wasm"))]
    pub fn show_image(&self, format: ImageFormat, width: usize, height: usize) {
        use std::env;

        let rendered = self.render_static(format, width, height);

        // Set up the temp file with a unique filename.
        let mut temp = env::temp_dir();
        let mut plot_name = Alphanumeric.sample_string(&mut thread_rng(), 22);
        plot_name.push_str(".html");
        plot_name = format!("plotly_{}", plot_name);
        temp.push(plot_name);

        // Save the rendered plot to the temp file.
        let temp_path = temp.to_str().unwrap();
        let mut file = File::create(temp_path).unwrap();
        file.write_all(rendered.as_bytes())
            .expect("failed to write html output");
        file.flush().unwrap();

        // Hand off the job of opening the browser to an OS-specific implementation.
        Plot::show_with_default_app(temp_path);
    }

    /// Save the rendered `Plot` to a file at the given location.
    ///
    /// This method will render the plot to a full, standalone HTML document,
    /// before saving it to the given location.
    pub fn write_html<P: AsRef<Path>>(&self, filename: P) {
        let rendered = self.to_html();

        let mut file = File::create(filename).unwrap();
        file.write_all(rendered.as_bytes())
            .expect("failed to write html output");
        file.flush().unwrap();
    }

    /// Convert a `Plot` to an HTML string representation.
    ///
    /// This method will generate a full, standalone HTML document. To generate
    /// a minimal HTML string which can be embedded within an existing HTML
    /// page, use `Plot::to_inline_html()`.
    pub fn to_html(&self) -> String {
        self.render()
    }

    /// Renders the contents of the `Plot` and returns it as a String suitable
    /// for embedding within web pages or Jupyter notebooks.
    ///
    /// A `div` is generated with the supplied id followed by the `script` block
    /// that generates the plot. The assumption is that `plotly.js` is
    /// available within the HTML page that this element is embedded. If
    /// that assumption is violated then the plot will not be displayed.
    ///
    /// If `plot_div_id` is `None` the plot div id will be randomly generated,
    /// otherwise the user-supplied `plot_div_id` is used.
    ///
    /// To generate a full, standalone HTML string or file, use
    /// `Plot::to_html()` and `Plot::write_html()`, respectively.
    pub fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let plot_div_id = match plot_div_id {
            Some(id) => id.to_string(),
            None => Alphanumeric.sample_string(&mut thread_rng(), 20),
        };
        self.render_inline(&plot_div_id)
    }

    fn to_jupyter_notebook_html(&self) -> String {
        let plot_div_id = Alphanumeric.sample_string(&mut thread_rng(), 20);

        let tmpl = JupyterNotebookPlotTemplate {
            plot: self,
            plot_div_id: &plot_div_id,
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

    /// Displays the plot in Jupyter Lab; if running a Jupyter Notebook then use
    /// the `notebook_display()` method instead.
    pub fn evcxr_display(&self) {
        self.lab_display();
    }

    /// Convert the `Plot` to a static image of the given image format and save
    /// at the given location.
    #[cfg(feature = "kaleido")]
    pub fn write_image<P: AsRef<Path>>(
        &self,
        filename: P,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) {
        let kaleido = plotly_kaleido::Kaleido::new();
        kaleido
            .save(
                filename.as_ref(),
                &serde_json::to_value(self).unwrap(),
                &format.to_string(),
                width,
                height,
                scale,
            )
            .unwrap_or_else(|_| panic!("failed to export plot to {:?}", filename.as_ref()));
    }

    fn render(&self) -> String {
        let tmpl = PlotTemplate {
            plot: self,
            remote_plotly_js: self.remote_plotly_js,
        };
        tmpl.render().unwrap()
    }

    fn render_static(&self, format: ImageFormat, width: usize, height: usize) -> String {
        let tmpl = StaticPlotTemplate {
            plot: self,
            format,
            remote_plotly_js: self.remote_plotly_js,
            width,
            height,
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
        // The only reason this could fail is if to_json() produces structurally
        // incorrect JSON. That would be a bug, and would require fixing in the
        // to_json()/serialization methods, rather than here
        js_sys::JSON::parse(&self.to_json())
            .expect("Invalid JSON")
            .dyn_into::<js_sys::Object>()
            .expect("Invalid JSON structure - expected a top-level Object")
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
    use std::path::PathBuf;

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
        let inline_plot_data = plot.to_inline_html(Some("replace_this_with_the_div_id"));
        assert!(inline_plot_data.contains("replace_this_with_the_div_id"));
        plot.to_inline_html(None);
    }

    #[test]
    fn test_jupyter_notebook_plot() {
        let plot = create_test_plot();
        plot.to_jupyter_notebook_html();
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
    #[ignore] // Don't really want it to try and open a browsert window every time we run a test.
    #[cfg(not(feature = "wasm"))]
    fn test_show_image() {
        let plot = create_test_plot();
        plot.show_image(ImageFormat::PNG, 1024, 680);
    }

    #[test]
    fn test_save_html() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.html");
        plot.write_html(&dst);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_png() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.png");
        plot.write_image(&dst, ImageFormat::PNG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_jpeg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.jpeg");
        plot.write_image(&dst, ImageFormat::JPEG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_svg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.svg");
        plot.write_image(&dst, ImageFormat::SVG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[ignore] // This seems to fail unpredictably on MacOs.
    #[cfg(feature = "kaleido")]
    fn test_save_to_eps() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.eps");
        plot.write_image(&dst, ImageFormat::EPS, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_pdf() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.pdf");
        plot.write_image(&dst, ImageFormat::PDF, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn test_save_to_webp() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.webp");
        plot.write_image(&dst, ImageFormat::WEBP, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }
}
