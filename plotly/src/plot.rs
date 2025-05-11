use std::{fs::File, io::Write, path::Path};

use askama::Template;
use dyn_clone::DynClone;
use erased_serde::Serialize as ErasedSerialize;
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use serde::Serialize;

use crate::{Configuration, Layout};

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
struct PlotTemplate<'a> {
    plot: &'a Plot,
    js_scripts: &'a str,
}

#[derive(Template)]
#[template(path = "static_plot.html", escape = "none")]
#[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
struct StaticPlotTemplate<'a> {
    plot: &'a Plot,
    format: ImageFormat,
    js_scripts: &'a str,
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

#[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
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

See https://plotly.github.io/plotly.rs/content/getting_started.html for further details.
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
///     let layout = Layout::new().title("<b>Line and Scatter Plot</b>");
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
    js_scripts: String,
}

impl Plot {
    /// Create a new `Plot`.
    pub fn new() -> Plot {
        Plot {
            traces: Traces::new(),
            js_scripts: Self::js_scripts(),
            ..Default::default()
        }
    }

    /// Switch to CDN for `plotly.js` and `MathJax` components in the standalone
    /// HTML plots rather than using the default local copies of the
    /// Javascript libraries. Method is only available when the feature
    /// `plotly_embed_js` is enabled since without this feature the default
    /// versions used are always the CDN versions.
    #[cfg(feature = "plotly_embed_js")]
    pub fn use_cdn_js(&mut self) {
        self.js_scripts = Self::online_cdn_js();
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
    #[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
    pub fn show(&self) {
        use std::env;

        let rendered = self.render();

        // Set up the temp file with a unique filename.
        let mut temp = env::temp_dir();
        let mut plot_name = Alphanumeric.sample_string(&mut rng(), 22);
        plot_name.push_str(".html");
        plot_name = format!("plotly_{}", plot_name);
        temp.push(plot_name);

        // Save the rendered plot to the temp file.
        let temp_path = temp.to_str().unwrap();

        {
            let mut file = File::create(temp_path).unwrap();
            file.write_all(rendered.as_bytes())
                .expect("failed to write html output");
            file.flush().unwrap();
        }

        // Hand off the job of opening the browser to an OS-specific implementation.
        Plot::show_with_default_app(temp_path);
    }

    /// Display the fully rendered HTML `Plot` in the default system browser.
    ///
    /// The HTML file is generated and saved in the provided filename as long as
    /// the path already exists, after the file is saved, it is read and
    /// displayed by the browser.
    #[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
    pub fn show_html<P: AsRef<Path> + std::clone::Clone>(&self, filename: P) {
        let path = filename.as_ref().to_str().unwrap();
        self.write_html(filename.clone());
        // Hand off the job of opening the browser to an OS-specific implementation.
        Plot::show_with_default_app(path);
    }

    /// Display the fully rendered `Plot` as a static image of the given format
    /// in the default system browser.
    #[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
    pub fn show_image(&self, format: ImageFormat, width: usize, height: usize) {
        use std::env;

        let rendered = self.render_static(format, width, height);

        // Set up the temp file with a unique filename.
        let mut temp = env::temp_dir();
        let mut plot_name = Alphanumeric.sample_string(&mut rng(), 22);
        plot_name.push_str(".html");
        plot_name = format!("plotly_{}", plot_name);
        temp.push(plot_name);

        // Save the rendered plot to the temp file.
        let temp_path = temp.to_str().unwrap();

        {
            let mut file = File::create(temp_path).unwrap();
            file.write_all(rendered.as_bytes())
                .expect("failed to write html output");
            file.flush().unwrap();
        }

        // Hand off the job of opening the browser to an OS-specific implementation.
        Plot::show_with_default_app(temp_path);
    }

    /// Save the rendered `Plot` to a file at the given location.
    ///
    /// This method will render the plot to a full, standalone HTML document,
    /// before saving it to the given location.
    pub fn write_html<P: AsRef<Path>>(&self, filename: P) {
        let rendered = self.to_html();

        let mut file =
            File::create(filename).expect("Provided filepath does not exist or is not accessible");
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
            None => Alphanumeric.sample_string(&mut rng(), 20),
        };
        self.render_inline(&plot_div_id)
    }

    fn to_jupyter_notebook_html(&self) -> String {
        let plot_div_id = Alphanumeric.sample_string(&mut rng(), 20);

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

    /// Convert the `Plot` to a static image and return the image as a `base64`
    /// String Supported formats are [ImageFormat::JPEG], [ImageFormat::PNG]
    /// and [ImageFormat::WEBP]
    #[cfg(feature = "kaleido")]
    pub fn to_base64(
        &self,
        format: ImageFormat,
        width: usize,
        height: usize,
        scale: f64,
    ) -> String {
        match format {
            ImageFormat::JPEG | ImageFormat::PNG | ImageFormat::WEBP => {
                let kaleido = plotly_kaleido::Kaleido::new();
                kaleido
                    .image_to_string(
                        &serde_json::to_value(self).unwrap(),
                        &format.to_string(),
                        width,
                        height,
                        scale,
                    )
                    .unwrap_or_else(|_| panic!("Kaleido failed to generate image"))
            }
            _ => {
                eprintln!("Cannot generate base64 string for ImageFormat:{format}. Allowed formats are JPEG, PNG, WEBP");
                String::default()
            }
        }
    }

    /// Convert the `Plot` to SVG and return it as a String.
    #[cfg(feature = "kaleido")]
    pub fn to_svg(&self, width: usize, height: usize, scale: f64) -> String {
        let kaleido = plotly_kaleido::Kaleido::new();
        kaleido
            .image_to_string(
                &serde_json::to_value(self).unwrap(),
                "svg",
                width,
                height,
                scale,
            )
            .unwrap_or_else(|_| panic!("Kaleido failed to generate image"))
    }

    fn render(&self) -> String {
        let tmpl = PlotTemplate {
            plot: self,
            js_scripts: &self.js_scripts,
        };
        tmpl.render().unwrap()
    }

    #[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
    fn render_static(&self, format: ImageFormat, width: usize, height: usize) -> String {
        let tmpl = StaticPlotTemplate {
            plot: self,
            format,
            js_scripts: &self.js_scripts,
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

    fn js_scripts() -> String {
        if cfg!(feature = "plotly_embed_js") {
            Self::offline_js_sources()
        } else {
            Self::online_cdn_js()
        }
    }

    fn offline_js_sources() -> String {
        let local_plotly_js = include_str!("../templates/plotly.min.js");
        let local_tex_mml_js = include_str!("../templates/tex-mml-chtml-3.2.0.js");
        let local_tex_svg_js = include_str!("../templates/tex-svg-3.2.2.js");
        format!(
            "<script type=\"text/javascript\">{}</script>\n
            <script type=\"text/javascript\">
            /**
             * tex-mml-chtml JS script
             **/
            {}
            </script>\n
            <script type=\"text/javascript\">
            /**
             * tex-svg JS script
             **/
            {}
            </script>\n",
            local_plotly_js, local_tex_mml_js, local_tex_svg_js
        )
        .to_string()
    }

    fn online_cdn_js() -> String {
        r##"<script src="https://cdn.plot.ly/plotly-2.12.1.min.js"></script>
        <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
        <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.0/es5/tex-mml-chtml.js"></script>
        "##
        .to_string()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[cfg(target_family = "wasm")]
    /// Convert a `Plot` to a native Javasript `js_sys::Object`.
    pub fn to_js_object(&self) -> wasm_bindgen_futures::js_sys::Object {
        use wasm_bindgen_futures::js_sys;
        use wasm_bindgen_futures::wasm_bindgen::JsCast;
        // The only reason this could fail is if to_json() produces structurally
        // incorrect JSON. That would be a bug, and would require fixing in the
        // to_json()/serialization methods, rather than here
        js_sys::JSON::parse(&self.to_json())
            .expect("Invalid JSON")
            .dyn_into::<js_sys::Object>()
            .expect("Invalid JSON structure - expected a top-level Object")
    }

    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    fn show_with_default_app(temp_path: &str) {
        use std::process::Command;
        Command::new("xdg-open")
            .args([temp_path])
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }

    #[cfg(target_os = "macos")]
    fn show_with_default_app(temp_path: &str) {
        use std::process::Command;
        Command::new("open")
            .args([temp_path])
            .output()
            .expect(DEFAULT_HTML_APP_NOT_FOUND);
    }

    #[cfg(target_os = "windows")]
    fn show_with_default_app(temp_path: &str) {
        use std::process::Command;
        Command::new("cmd")
            .args(&["/C", "start", &format!(r#"{}"#, temp_path)])
            .spawn()
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
    use super::*;
    use crate::Scatter;
    #[cfg(not(target_os = "macos"))]
    use base64::engine::general_purpose;
    use serde_json::{json, to_value};
    use std::path::PathBuf;
    fn create_test_plot() -> Plot {
        let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2]).name("trace1");
        let mut plot = Plot::new();
        plot.add_trace(trace1);
        plot
    }

    #[test]
    fn inline_plot() {
        let plot = create_test_plot();
        let inline_plot_data = plot.to_inline_html(Some("replace_this_with_the_div_id"));
        assert!(inline_plot_data.contains("replace_this_with_the_div_id"));
        plot.to_inline_html(None);
    }

    #[test]
    fn jupyter_notebook_plot() {
        let plot = create_test_plot();
        plot.to_jupyter_notebook_html();
    }

    #[test]
    fn notebook_display() {
        let plot = create_test_plot();
        plot.notebook_display();
    }

    #[test]
    fn lab_display() {
        let plot = create_test_plot();
        plot.lab_display();
    }

    #[test]
    fn plot_serialize_simple() {
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
    fn plot_serialize_with_layout() {
        let mut plot = create_test_plot();
        let layout = Layout::new().title("Title");
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
    fn data_to_json() {
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
    fn empty_layout_to_json() {
        let plot = create_test_plot();
        let expected = json!({});

        assert_eq!(to_value(plot.layout()).unwrap(), expected);
    }

    #[test]
    fn layout_to_json() {
        let mut plot = create_test_plot();
        let layout = Layout::new().title("TestTitle");
        plot.set_layout(layout);

        let expected = json!({
            "title": {"text": "TestTitle"}
        });

        assert_eq!(to_value(plot.layout()).unwrap(), expected);
    }

    #[test]
    fn plot_eq() {
        let plot1 = create_test_plot();
        let plot2 = create_test_plot();

        assert!(plot1 == plot2);
    }

    #[test]
    fn plot_neq() {
        let plot1 = create_test_plot();
        let trace2 = Scatter::new(vec![10, 1, 2], vec![6, 10, 2]).name("trace2");
        let mut plot2 = Plot::new();
        plot2.add_trace(trace2);

        assert!(plot1 != plot2);
    }

    #[test]
    fn plot_clone() {
        let plot1 = create_test_plot();
        let plot2 = plot1.clone();

        assert!(plot1 == plot2);
    }

    #[test]
    #[ignore] // Don't really want it to try and open a browser window every time we run a test.
    #[cfg(not(target_family = "wasm"))]
    fn show_image() {
        let plot = create_test_plot();
        plot.show_image(ImageFormat::PNG, 1024, 680);
    }

    #[test]
    fn save_html() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.html");
        plot.write_html(&dst);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    #[cfg(feature = "kaleido")]
    fn save_to_png() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.png");
        plot.write_image(&dst, ImageFormat::PNG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    #[cfg(feature = "kaleido")]
    fn save_to_jpeg() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.jpeg");
        plot.write_image(&dst, ImageFormat::JPEG, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    #[cfg(feature = "kaleido")]
    fn save_to_svg() {
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
    fn save_to_eps() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.eps");
        plot.write_image(&dst, ImageFormat::EPS, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    #[cfg(feature = "kaleido")]
    fn save_to_pdf() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.pdf");
        plot.write_image(&dst, ImageFormat::PDF, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    #[cfg(feature = "kaleido")]
    fn save_to_webp() {
        let plot = create_test_plot();
        let dst = PathBuf::from("example.webp");
        plot.write_image(&dst, ImageFormat::WEBP, 1024, 680, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    #[cfg(feature = "kaleido")]
    fn image_to_base64() {
        let plot = create_test_plot();

        let image_base64 = plot.to_base64(ImageFormat::PNG, 200, 150, 1.0);

        assert!(!image_base64.is_empty());

        let result_decoded = general_purpose::STANDARD.decode(image_base64).unwrap();
        let expected = "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";
        let expected_decoded = general_purpose::STANDARD.decode(expected).unwrap();

        // Comparing the result seems to end up being a flaky test.
        // Limit the comparison to the first characters;
        // As image contents seem to be slightly inconsistent across platforms
        assert_eq!(expected_decoded[..2], result_decoded[..2]);
    }

    #[test]
    #[cfg(feature = "kaleido")]
    fn image_to_base64_invalid_format() {
        let plot = create_test_plot();
        let image_base64 = plot.to_base64(ImageFormat::EPS, 200, 150, 1.0);
        assert!(image_base64.is_empty());
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    #[cfg(feature = "kaleido")]
    fn image_to_svg_string() {
        let plot = create_test_plot();
        let image_svg = plot.to_svg(200, 150, 1.0);

        assert!(!image_svg.is_empty());

        let expected = "<svg class=\"main-svg\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"200\" height=\"150\" style=\"\" viewBox=\"0 0 200 150\"><rect x=\"0\" y=\"0\" width=\"200\" height=\"150\" style=\"fill: rgb(255, 255, 255); fill-opacity: 1;\"/><defs id=\"defs-2dc70a\"><g class=\"clips\"><clipPath id=\"clip2dc70axyplot\" class=\"plotclip\"><rect width=\"40\" height=\"2\"/></clipPath><clipPath class=\"axesclip\" id=\"clip2dc70ax\"><rect x=\"80\" y=\"0\" width=\"40\" height=\"150\"/></clipPath><clipPath class=\"axesclip\" id=\"clip2dc70ay\"><rect x=\"0\" y=\"82\" width=\"200\" height=\"2\"/></clipPath><clipPath class=\"axesclip\" id=\"clip2dc70axy\"><rect x=\"80\" y=\"82\" width=\"40\" height=\"2\"/></clipPath></g><g class=\"gradients\"/></defs><g class=\"bglayer\"/><g class=\"layer-below\"><g class=\"imagelayer\"/><g class=\"shapelayer\"/></g><g class=\"cartesianlayer\"><g class=\"subplot xy\"><g class=\"layer-subplot\"><g class=\"shapelayer\"/><g class=\"imagelayer\"/></g><g class=\"gridlayer\"><g class=\"x\"><path class=\"xgrid crisp\" transform=\"translate(100,0)\" d=\"M0,82v2\" style=\"stroke: rgb(238, 238, 238); stroke-opacity: 1; stroke-width: 1px;\"/><path class=\"xgrid crisp\" transform=\"translate(114.25,0)\" d=\"M0,82v2\" style=\"stroke: rgb(238, 238, 238); stroke-opacity: 1; stroke-width: 1px;\"/></g><g class=\"y\"/></g><g class=\"zerolinelayer\"><path class=\"xzl zl crisp\" transform=\"translate(85.75,0)\" d=\"M0,82v2\" style=\"stroke: rgb(68, 68, 68); stroke-opacity: 1; stroke-width: 1px;\"/></g><path class=\"xlines-below\"/><path class=\"ylines-below\"/><g class=\"overlines-below\"/><g class=\"xaxislayer-below\"/><g class=\"yaxislayer-below\"/><g class=\"overaxes-below\"/><g class=\"plot\" transform=\"translate(80,82)\" clip-path=\"url('#clip2dc70axyplot')\"><g class=\"scatterlayer mlayer\"><g class=\"trace scatter trace86f735\" style=\"stroke-miterlimit: 2; opacity: 1;\"><g class=\"fills\"/><g class=\"errorbars\"/><g class=\"lines\"><path class=\"js-line\" d=\"M5.75,1L20,0L34.25,2\" style=\"vector-effect: non-scaling-stroke; fill: none; stroke: rgb(31, 119, 180); stroke-opacity: 1; stroke-width: 2px; opacity: 1;\"/></g><g class=\"points\"><path class=\"point\" transform=\"translate(5.75,1)\" d=\"M3,0A3,3 0 1,1 0,-3A3,3 0 0,1 3,0Z\" style=\"opacity: 1; stroke-width: 0px; fill: rgb(31, 119, 180); fill-opacity: 1;\"/><path class=\"point\" transform=\"translate(20,0)\" d=\"M3,0A3,3 0 1,1 0,-3A3,3 0 0,1 3,0Z\" style=\"opacity: 1; stroke-width: 0px; fill: rgb(31, 119, 180); fill-opacity: 1;\"/><path class=\"point\" transform=\"translate(34.25,2)\" d=\"M3,0A3,3 0 1,1 0,-3A3,3 0 0,1 3,0Z\" style=\"opacity: 1; stroke-width: 0px; fill: rgb(31, 119, 180); fill-opacity: 1;\"/></g><g class=\"text\"/></g></g></g><g class=\"overplot\"/><path class=\"xlines-above crisp\" d=\"M0,0\" style=\"fill: none;\"/><path class=\"ylines-above crisp\" d=\"M0,0\" style=\"fill: none;\"/><g class=\"overlines-above\"/><g class=\"xaxislayer-above\"><g class=\"xtick\"><text text-anchor=\"middle\" x=\"0\" y=\"97\" transform=\"translate(85.75,0)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">0</text></g><g class=\"xtick\"><text text-anchor=\"middle\" x=\"0\" y=\"97\" transform=\"translate(100,0)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">1</text></g><g class=\"xtick\"><text text-anchor=\"middle\" x=\"0\" y=\"97\" transform=\"translate(114.25,0)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">2</text></g></g><g class=\"yaxislayer-above\"><g class=\"ytick\"><text text-anchor=\"end\" x=\"79\" y=\"4.199999999999999\" transform=\"translate(0,84)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">2</text></g><g class=\"ytick\"><text text-anchor=\"end\" x=\"79\" y=\"4.199999999999999\" transform=\"translate(0,83.5)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">4</text></g><g class=\"ytick\"><text text-anchor=\"end\" x=\"79\" y=\"4.199999999999999\" transform=\"translate(0,83)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">6</text></g><g class=\"ytick\"><text text-anchor=\"end\" x=\"79\" y=\"4.199999999999999\" transform=\"translate(0,82.5)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">8</text></g><g class=\"ytick\"><text text-anchor=\"end\" x=\"79\" y=\"4.199999999999999\" transform=\"translate(0,82)\" style=\"font-family: 'Open Sans', verdana, arial, sans-serif; font-size: 12px; fill: rgb(68, 68, 68); fill-opacity: 1; white-space: pre;\">10</text></g></g><g class=\"overaxes-above\"/></g></g><g class=\"polarlayer\"/><g class=\"ternarylayer\"/><g class=\"geolayer\"/><g class=\"funnelarealayer\"/><g class=\"pielayer\"/><g class=\"treemaplayer\"/><g class=\"sunburstlayer\"/><g class=\"glimages\"/><defs id=\"topdefs-2dc70a\"><g class=\"clips\"/></defs><g class=\"layer-above\"><g class=\"imagelayer\"/><g class=\"shapelayer\"/></g><g class=\"infolayer\"><g class=\"g-gtitle\"/><g class=\"g-xtitle\"/><g class=\"g-ytitle\"/></g></svg>";
        // Limit the test to the first LEN characters as generated SVGs
        // seem to contain uniquely generated IDs
        const LEN: usize = 10;
        assert_eq!(expected[..LEN], image_svg[..LEN]);
    }

    #[cfg(target_os = "macos")]
    #[test]
    #[cfg(feature = "kaleido")]
    fn save_surface_to_png() {
        use crate::Surface;
        let mut plot = Plot::new();
        let z_matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let x_unique = vec![1.0, 2.0, 3.0];
        let y_unique = vec![4.0, 5.0, 6.0];
        let surface = Surface::new(z_matrix)
            .x(x_unique)
            .y(y_unique)
            .name("Surface");

        plot.add_trace(surface);
        let dst = PathBuf::from("example.png");
        plot.write_image("example.png", ImageFormat::PNG, 800, 600, 1.0);
        assert!(dst.exists());
        assert!(std::fs::remove_file(&dst).is_ok());
        assert!(!dst.exists());
        assert!(!plot.to_base64(ImageFormat::PNG, 1024, 680, 1.0).is_empty());
    }
}
