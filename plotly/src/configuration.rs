use serde::{ser::Serializer, Serialize};
use serde_repr::Serialize_repr;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ImageButtonFormats {
    Png,
    Svg,
    Jpeg,
    Webp,
}

// TODO: should this be behind the plotly-kaleido feature?
#[derive(Serialize, Debug, Default, Clone)]
pub struct ToImageButtonOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ImageButtonFormats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<usize>,
}

impl ToImageButtonOptions {
    /// Create a new empty `ToImageButtonOptions` configuration struct.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the file format of the downloaded plot image.
    pub fn format(mut self, format: ImageButtonFormats) -> Self {
        self.format = Some(format);
        self
    }

    /// Set the filename of the downloaded plot image.
    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    /// Set the height, in pixels, of the downloaded plot image.
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the width, in pixels, of the downloaded plot image.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the scale of the downloaded plot image. Title, legend, axis and canvas sizes
    /// will all be multiplied by `scale`.
    pub fn scale(mut self, scale: usize) -> Self {
        self.scale = Some(scale);
        self
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ModeBarButtonName {
    Zoom2d,
    Pan2d,
    Select2d,
    Lasso2d,
    ZoomIn2d,
    ZoomOut2d,
    AutoScale2d,
    ResetScale2d,
    Zoom3d,
    Pan3d,
    OrbitRotation,
    TableRotation,
    HandleDrag3d,
    ResetCameraDefault3d,
    ResetCameraLastSave3d,
    HoverClosest3d,
    HoverClosestCartesian,
    HoverCompareCartesian,
    ZoomInGeo,
    ZoomOutGeo,
    ResetGeo,
    HoverClosestGeo,
    HoverClosestGl2d,
    HoverClosestPie,
    ToggleHover,
    ResetViews,
    ToImage,
    SendDataToCloud,
    ToggleSpikelines,
    ResetViewMapbox,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase", untagged)]
pub enum DoubleClick {
    #[serde(serialize_with = "serialize_to_false")]
    False,
    Reset,
    AutoSize,
    #[serde(rename = "reset+autosize")]
    ResetAutoSize,
}

fn serialize_to_false<S>(s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_bool(false)
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum PlotGLPixelRatio {
    One,
    Two,
    Three,
    Four,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    // reference is here: https://github.com/plotly/plotly.js/blob/master/src/plot_api/plot_config.js#L22-L86
    // missing edits, show_sources, mode_bar_buttons, set_background, logging, notify_on_logging,
    // global_transforms, mode_bar_buttons_to_add and locales
    #[serde(skip_serializing_if = "Option::is_none")]
    typeset_math: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autosizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scroll_zoom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_frame: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_margins: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_plot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_image_button_options: Option<ToImageButtonOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_mode_bar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode_bar_buttons_to_remove: Option<Vec<ModeBarButtonName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "plotlyServerURL")]
    plotly_server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "topojsonURL")]
    topojson_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapbox_access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_edit_in_chart_studio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_logo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    responsive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    double_click: Option<DoubleClick>,
    #[serde(skip_serializing_if = "Option::is_none")]
    double_click_delay: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_axis_drag_handles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_axis_range_entry_boxes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_tips: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    watermark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plot_gl_pixel_ratio: Option<PlotGLPixelRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_send_to_cloud: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_length: Option<usize>,
}

impl Configuration {
    /// Create a new default `Configuration` object. Options can be configured using the
    /// provided setter methods.
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines whether the graphs are interactive or not. If `false`, no interactivity,
    /// for export or image generation.
    pub fn static_plot(mut self, static_plot: bool) -> Self {
        self.static_plot = Some(static_plot);
        self
    }

    /// Determines whether math should be typeset or not, when MathJax (either v2 or v3) is present on the page.
    pub fn typeset_math(mut self, typeset_math: bool) -> Self {
        self.typeset_math = Some(typeset_math);
        self
    }

    /// When set it determines base URL for the "Edit in Chart Studio" `show_edit_in_chart_studio`/`show_send_to_cloud`
    /// mode bar button and the show_link/send_data on-graph link. To enable sending your data to Chart Studio
    /// Cloud, you need to set both `plotly_server_url` to "https://chart-studio.plotly.com" and
    /// also set `showSendToCloud` to `true`.
    pub fn plotly_server_url(mut self, plotly_server_url: &str) -> Self {
        self.plotly_server_url = Some(plotly_server_url.to_string());
        self
    }

    /// Determines whether the graph is editable or not. Sets all pieces of `edits` unless a separate
    /// `edits` config item overrides individual parts.
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    /// Determines whether the graphs are plotted with respect to layout.auto_size: true and infer
    /// its container size.
    pub fn autosizable(mut self, autosizable: bool) -> Self {
        self.autosizable = Some(autosizable);
        self
    }

    /// Determines whether to change the layout size when window is resized. In v3, this option will be
    /// removed and will always be true.
    pub fn responsive(mut self, responsive: bool) -> Self {
        self.responsive = Some(responsive);
        self
    }

    /// When `layout.auto_size` is turned on, determines whether the graph fills the container
    /// (the default) or the screen (if set to `true`).
    pub fn fill_frame(mut self, fill_frame: bool) -> Self {
        self.fill_frame = Some(fill_frame);
        self
    }

    /// When `layout.auto_size` is turned on, set the frame margins in fraction of the graph size.
    pub fn frame_margins(mut self, frame_margins: f64) -> Self {
        // TODO: plotly supports a minimum value of 0 and a maximum value of 0.5
        self.frame_margins = Some(frame_margins);
        self
    }

    /// Determines whether mouse wheel or two-finger scroll zooms is enable. Turned on by default for
    /// gl3d, geo and mapbox subplots (as these subplot types do not have zoombox via pan), but
    /// turned off by default for cartesian subplots. Set `scroll_zoom` to `false` to disable scrolling for
    /// all subplots.
    pub fn scroll_zoom(mut self, scroll_zoom: bool) -> Self {
        self.scroll_zoom = Some(scroll_zoom);
        self
    }

    /// Sets the double click interaction mode. Has an effect only in cartesian plots. If `false`,
    /// double click is disable. If `reset`, double click resets the axis ranges to their initial values.
    /// If `autosize`, double click set the axis ranges to their autorange values. If `reset+autosize`,
    /// the odd double clicks resets the axis ranges to their initial values and even double clicks set
    /// the axis ranges to their autorange values.
    pub fn double_click(mut self, double_click: DoubleClick) -> Self {
        self.double_click = Some(double_click);
        self
    }

    /// Sets the delay for registering a double-click in ms. This is the time interval (in ms) between
    /// first mousedown and 2nd mouseup to constitute a double-click. This setting propagates to all
    /// on-subplot double clicks (except for geo and mapbox) and on-legend double clicks.
    pub fn double_click_delay(mut self, double_click_delay: usize) -> Self {
        self.double_click_delay = Some(double_click_delay);
        self
    }

    /// Set to `false` to omit cartesian axis pan/zoom drag handles.
    pub fn show_axis_drag_handles(mut self, show_axis_drag_handles: bool) -> Self {
        self.show_axis_drag_handles = Some(show_axis_drag_handles);
        self
    }

    /// Set to `false` to omit direct range entry at the pan/zoom drag points, note that `show_axis_drag_handles`
    /// must be enabled to have an effect.
    pub fn show_axis_range_entry_boxes(mut self, show_axis_range_entry_boxes: bool) -> Self {
        self.show_axis_range_entry_boxes = Some(show_axis_range_entry_boxes);
        self
    }

    /// Determines whether or not tips are shown while interacting with the resulting graphs.
    pub fn show_tips(mut self, show_tips: bool) -> Self {
        self.show_tips = Some(show_tips);
        self
    }

    /// Determines whether a link to Chart Studio Cloud is displayed at the bottom right corner of resulting
    /// graphs. Use with `send_data` and `link_text`.
    pub fn show_link(mut self, show_link: bool) -> Self {
        self.show_link = Some(show_link);
        self
    }

    /// Sets the text appearing in the `showLink` link.
    pub fn link_text(mut self, link_text: &str) -> Self {
        self.link_text = Some(link_text.to_string());
        self
    }

    /// If `show_link` is true, does it contain data just link to a Chart Studio Cloud file?
    pub fn send_data(mut self, send_data: bool) -> Self {
        self.send_data = Some(send_data);
        self
    }

    /// Determines the mode bar display mode. If `true`, the mode bar is always visible. If `false`,
    /// the mode bar is always hidden. If omitted, the mode bar is visible while the mouse cursor
    /// is on the graph container.
    pub fn display_mode_bar(mut self, display_mode_bar: bool) -> Self {
        self.display_mode_bar = Some(display_mode_bar);
        self
    }

    /// Should we include a ModeBar button, labeled "Edit in Chart Studio" that sends this chart to
    /// chart-studio.plotly.com (formerly plot.ly) or another plotly server as specified by `plotly_server_url`
    /// for editing, export, etc? Prior to version 1.43.0 this button was included by default, now it is
    /// opt-in using this flag. Note that this button can (depending on `plotly_server_url` being set) send your
    /// data to an external server. However that server does not persist your data until you arrive at the Chart
    /// Studio and explicitly click "Save".
    pub fn show_send_to_cloud(mut self, show_send_to_cloud: bool) -> Self {
        self.show_send_to_cloud = Some(show_send_to_cloud);
        self
    }

    /// Same as `show_send_to_cloud`, but use a pencil icon instead of a floppy-disk. Note that if both
    /// `show_send_to_cloud` and `show_edit_in_chart_studio` are turned on, only `show_edit_in_chart_studio` will
    /// be honored.
    pub fn show_edit_in_chart_studio(mut self, show_edit_in_chart_studio: bool) -> Self {
        self.show_edit_in_chart_studio = Some(show_edit_in_chart_studio);
        self
    }

    /// Remove mode bar buttons by name.
    pub fn mode_bar_buttons_to_remove(
        mut self,
        mode_bar_buttons_to_remove: Vec<ModeBarButtonName>,
    ) -> Self {
        self.mode_bar_buttons_to_remove = Some(mode_bar_buttons_to_remove);
        self
    }

    /// Statically override options for toImage modebar button.
    pub fn to_image_button_options(
        mut self,
        to_image_button_options: ToImageButtonOptions,
    ) -> Self {
        self.to_image_button_options = Some(to_image_button_options);
        self
    }

    /// Determines whether or not the plotly logo is displayed on the end of the mode bar.
    pub fn display_logo(mut self, display_logo: bool) -> Self {
        self.display_logo = Some(display_logo);
        self
    }

    /// Watermark the images with the company's logo.
    pub fn watermark(mut self, watermark: bool) -> Self {
        self.watermark = Some(watermark);
        self
    }

    /// Set the pixel ratio during WebGL image export.
    pub fn plot_gl_pixel_ratio(mut self, plot_gl_pixel_ratio: PlotGLPixelRatio) -> Self {
        self.plot_gl_pixel_ratio = Some(plot_gl_pixel_ratio);
        self
    }

    /// Set the URL to topojson used in geo charts. By default, the topojson files are fetched from
    /// cdn.plot.ly. For example, set this option to: "<path-to-plotly.js>/dist/topojson/" to render
    /// geographical feature using the topojson files that ship with the plotly.js module.
    pub fn topojson_url(mut self, topojson_url: &str) -> Self {
        self.topojson_url = Some(topojson_url.to_string());
        self
    }

    /// Mapbox access token (required to plot mapbox trace types). If using an Mapbox Atlas server, set
    /// this option to "" so that plotly.js won't attempt to authenticate to the public Mapbox server.
    pub fn mapbox_access_token(mut self, mapbox_access_token: &str) -> Self {
        self.mapbox_access_token = Some(mapbox_access_token.to_string());
        self
    }

    /// Sets the length of the undo/redo queue.
    pub fn queue_length(mut self, queue_length: usize) -> Self {
        self.queue_length = Some(queue_length);
        self
    }

    /// Sets which localization to use. When using this setting, make sure that the appropriate locale is
    /// present in the HTML file. For example, to use the "fr" locale,
    /// <script src="https://cdn.plot.ly/plotly-locale-fr-latest.js"></script> must be present.
    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_string());
        self
    }
}
