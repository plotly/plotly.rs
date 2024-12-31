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
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct ToImageButtonOptions {
    format: Option<ImageButtonFormats>,
    filename: Option<String>,
    height: Option<usize>,
    width: Option<usize>,
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

    /// Set the scale of the downloaded plot image. Title, legend, axis and
    /// canvas sizes will all be multiplied by `scale`.
    pub fn scale(mut self, scale: usize) -> Self {
        self.scale = Some(scale);
        self
    }
}

#[derive(Debug, Clone)]
pub enum DisplayModeBar {
    Hover,
    True,
    False,
}

impl Serialize for DisplayModeBar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Hover => serializer.serialize_str("hover"),
            Self::True => serializer.serialize_bool(true),
            Self::False => serializer.serialize_bool(false),
        }
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
    ZoomInMapbox,
    ZoomOutMapbox,
}

#[derive(Debug, Clone)]
pub enum DoubleClick {
    False,
    Reset,
    AutoSize,
    ResetAutoSize,
}

impl Serialize for DoubleClick {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::False => serializer.serialize_bool(false),
            Self::Reset => serializer.serialize_str("reset"),
            Self::AutoSize => serializer.serialize_str("autosize"),
            Self::ResetAutoSize => serializer.serialize_str("reset+autosize"),
        }
    }
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum PlotGLPixelRatio {
    One = 1,
    Two,
    Three,
    Four,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    // reference is here: https://github.com/plotly/plotly.js/blob/master/src/plot_api/plot_config.js
    // missing: edits, show_sources, mode_bar_buttons, set_background, logging, notify_on_logging,
    // global_transforms, mode_bar_buttons_to_add and locales
    typeset_math: Option<bool>,
    autosizable: Option<bool>,
    scroll_zoom: Option<bool>,
    fill_frame: Option<bool>,
    frame_margins: Option<f64>,
    editable: Option<bool>,
    static_plot: Option<bool>,
    to_image_button_options: Option<ToImageButtonOptions>,
    display_mode_bar: Option<DisplayModeBar>,
    mode_bar_buttons_to_remove: Option<Vec<ModeBarButtonName>>,
    show_link: Option<bool>,
    #[serde(rename = "plotlyServerURL")]
    plotly_server_url: Option<String>,
    #[serde(rename = "topojsonURL")]
    topojson_url: Option<String>,
    link_text: Option<String>,
    mapbox_access_token: Option<String>,
    show_edit_in_chart_studio: Option<bool>,
    locale: Option<String>,
    #[serde(rename = "displaylogo")]
    display_logo: Option<bool>,
    responsive: Option<bool>,
    double_click: Option<DoubleClick>,
    double_click_delay: Option<usize>,
    show_axis_drag_handles: Option<bool>,
    show_axis_range_entry_boxes: Option<bool>,
    show_tips: Option<bool>,
    send_data: Option<bool>,
    watermark: Option<bool>,
    #[serde(rename = "plotGlPixelRatio")]
    plot_gl_pixel_ratio: Option<PlotGLPixelRatio>,
    show_send_to_cloud: Option<bool>,
    queue_length: Option<usize>,
}

impl Configuration {
    /// Create a new default `Configuration` object. Options can be configured
    /// using the provided setter methods.
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines whether the graphs are interactive or not. If `false`, no
    /// interactivity, for export or image generation.
    pub fn static_plot(mut self, static_plot: bool) -> Self {
        self.static_plot = Some(static_plot);
        self
    }

    /// Determines whether math should be typeset or not, when MathJax (either
    /// v2 or v3) is present on the page.
    pub fn typeset_math(mut self, typeset_math: bool) -> Self {
        self.typeset_math = Some(typeset_math);
        self
    }

    /// When set it determines base URL for the "Edit in Chart Studio"
    /// `show_edit_in_chart_studio`/`show_send_to_cloud` mode bar button and
    /// the show_link/send_data on-graph link. To enable sending your data to
    /// Chart Studio Cloud, you need to set both `plotly_server_url` to <https://chart-studio.plotly.com> and
    /// also set `showSendToCloud` to `true`.
    pub fn plotly_server_url(mut self, plotly_server_url: &str) -> Self {
        self.plotly_server_url = Some(plotly_server_url.to_string());
        self
    }

    /// Determines whether the graph is editable or not. Sets all pieces of
    /// `edits` unless a separate `edits` config item overrides individual
    /// parts.
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    /// Determines whether the graphs are plotted with respect to
    /// layout.auto_size: true and infer its container size.
    pub fn autosizable(mut self, autosizable: bool) -> Self {
        self.autosizable = Some(autosizable);
        self
    }

    /// Determines whether to change the layout size when window is resized. In
    /// v3, this option will be removed and will always be true.
    pub fn responsive(mut self, responsive: bool) -> Self {
        self.responsive = Some(responsive);
        self
    }

    /// When `layout.auto_size` is turned on, determines whether the graph fills
    /// the container (the default) or the screen (if set to `true`).
    pub fn fill_frame(mut self, fill_frame: bool) -> Self {
        self.fill_frame = Some(fill_frame);
        self
    }

    /// When `layout.auto_size` is turned on, set the frame margins in fraction
    /// of the graph size.
    pub fn frame_margins(mut self, frame_margins: f64) -> Self {
        // TODO: plotly supports a minimum value of 0 and a maximum value of 0.5
        self.frame_margins = Some(frame_margins);
        self
    }

    /// Determines whether mouse wheel or two-finger scroll zooms is enable.
    /// Turned on by default for gl3d, geo and mapbox subplots (as these
    /// subplot types do not have zoombox via pan), but turned off by
    /// default for cartesian subplots. Set `scroll_zoom` to `false` to disable
    /// scrolling for all subplots.
    pub fn scroll_zoom(mut self, scroll_zoom: bool) -> Self {
        self.scroll_zoom = Some(scroll_zoom);
        self
    }

    /// Sets the double click interaction mode. Has an effect only in cartesian
    /// plots. If `false`, double click is disable. If `reset`, double click
    /// resets the axis ranges to their initial values. If `autosize`,
    /// double click set the axis ranges to their autorange values. If
    /// `reset+autosize`, the odd double clicks resets the axis ranges to
    /// their initial values and even double clicks set the axis ranges to
    /// their autorange values.
    pub fn double_click(mut self, double_click: DoubleClick) -> Self {
        self.double_click = Some(double_click);
        self
    }

    /// Sets the delay for registering a double-click in ms. This is the time
    /// interval (in ms) between first mousedown and 2nd mouseup to
    /// constitute a double-click. This setting propagates to all on-subplot
    /// double clicks (except for geo and mapbox) and on-legend double clicks.
    pub fn double_click_delay(mut self, double_click_delay: usize) -> Self {
        self.double_click_delay = Some(double_click_delay);
        self
    }

    /// Set to `false` to omit cartesian axis pan/zoom drag handles.
    pub fn show_axis_drag_handles(mut self, show_axis_drag_handles: bool) -> Self {
        self.show_axis_drag_handles = Some(show_axis_drag_handles);
        self
    }

    /// Set to `false` to omit direct range entry at the pan/zoom drag points,
    /// note that `show_axis_drag_handles` must be enabled to have an
    /// effect.
    pub fn show_axis_range_entry_boxes(mut self, show_axis_range_entry_boxes: bool) -> Self {
        self.show_axis_range_entry_boxes = Some(show_axis_range_entry_boxes);
        self
    }

    /// Determines whether or not tips are shown while interacting with the
    /// resulting graphs.
    pub fn show_tips(mut self, show_tips: bool) -> Self {
        self.show_tips = Some(show_tips);
        self
    }

    /// Determines whether a link to Chart Studio Cloud is displayed at the
    /// bottom right corner of resulting graphs. Use with `send_data` and
    /// `link_text`.
    pub fn show_link(mut self, show_link: bool) -> Self {
        self.show_link = Some(show_link);
        self
    }

    /// Sets the text appearing in the `showLink` link.
    pub fn link_text(mut self, link_text: &str) -> Self {
        self.link_text = Some(link_text.to_string());
        self
    }

    /// If `show_link` is true, does it contain data just link to a Chart Studio
    /// Cloud file?
    pub fn send_data(mut self, send_data: bool) -> Self {
        self.send_data = Some(send_data);
        self
    }

    /// Determines the mode bar display mode. If `true`, the mode bar is always
    /// visible. If `false`, the mode bar is always hidden. If `hover`, the
    /// mode bar is visible while the mouse cursor is on the graph
    /// container.
    pub fn display_mode_bar(mut self, display_mode_bar: DisplayModeBar) -> Self {
        self.display_mode_bar = Some(display_mode_bar);
        self
    }

    /// Should we include a ModeBar button, labeled "Edit in Chart Studio" that
    /// sends this chart to chart-studio.plotly.com (formerly plot.ly) or
    /// another plotly server as specified by `plotly_server_url`
    /// for editing, export, etc? Prior to version 1.43.0 this button was
    /// included by default, now it is opt-in using this flag. Note that
    /// this button can (depending on `plotly_server_url` being set) send your
    /// data to an external server. However that server does not persist your
    /// data until you arrive at the Chart Studio and explicitly click
    /// "Save".
    pub fn show_send_to_cloud(mut self, show_send_to_cloud: bool) -> Self {
        self.show_send_to_cloud = Some(show_send_to_cloud);
        self
    }

    /// Same as `show_send_to_cloud`, but use a pencil icon instead of a
    /// floppy-disk. Note that if both `show_send_to_cloud` and
    /// `show_edit_in_chart_studio` are turned on, only
    /// `show_edit_in_chart_studio` will be honored.
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

    /// Determines whether or not the plotly logo is displayed on the end of the
    /// mode bar.
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

    /// Set the URL to topojson used in geo charts. By default, the topojson
    /// files are fetched from cdn.plot.ly. For example, set this option to:
    /// "<path-to-plotly.js>/dist/topojson/" to render geographical feature
    /// using the topojson files that ship with the plotly.js module.
    pub fn topojson_url(mut self, topojson_url: &str) -> Self {
        self.topojson_url = Some(topojson_url.to_string());
        self
    }

    /// Mapbox access token (required to plot mapbox trace types). If using an
    /// Mapbox Atlas server, set this option to "" so that plotly.js won't
    /// attempt to authenticate to the public Mapbox server.
    pub fn mapbox_access_token(mut self, mapbox_access_token: &str) -> Self {
        self.mapbox_access_token = Some(mapbox_access_token.to_string());
        self
    }

    /// Sets the length of the undo/redo queue.
    pub fn queue_length(mut self, queue_length: usize) -> Self {
        self.queue_length = Some(queue_length);
        self
    }

    /// Sets which localization to use. When using this setting, make sure that
    /// the appropriate locale is present in the HTML file. For example, to
    /// use the "fr" locale, <script src="https://cdn.plot.ly/plotly-locale-fr-latest.js"></script> must be present.
    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_image_button_formats() {
        assert_eq!(to_value(ImageButtonFormats::Png).unwrap(), json!("png"));
        assert_eq!(to_value(ImageButtonFormats::Svg).unwrap(), json!("svg"));
        assert_eq!(to_value(ImageButtonFormats::Jpeg).unwrap(), json!("jpeg"));
        assert_eq!(to_value(ImageButtonFormats::Webp).unwrap(), json!("webp"));
    }
    #[test]
    fn test_serialize_to_image_button_options() {
        let options = ToImageButtonOptions::new()
            .format(ImageButtonFormats::Jpeg)
            .filename("filename")
            .height(500)
            .width(250)
            .scale(2);
        let expected = json!({
            "format": "jpeg",
            "filename": "filename",
            "height": 500,
            "width": 250,
            "scale": 2
        });

        assert_eq!(to_value(options).unwrap(), expected)
    }

    #[test]
    fn test_serialize_display_mode_bar() {
        assert_eq!(to_value(DisplayModeBar::Hover).unwrap(), json!("hover"));
        assert_eq!(to_value(DisplayModeBar::True).unwrap(), json!(true));
        assert_eq!(to_value(DisplayModeBar::False).unwrap(), json!(false));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_mode_bar_button_name() {
        assert_eq!(to_value(ModeBarButtonName::Zoom2d).unwrap(), json!("zoom2d"));
        assert_eq!(to_value(ModeBarButtonName::Pan2d).unwrap(), json!("pan2d"));
        assert_eq!(to_value(ModeBarButtonName::Select2d).unwrap(), json!("select2d"));
        assert_eq!(to_value(ModeBarButtonName::Lasso2d).unwrap(), json!("lasso2d"));
        assert_eq!(to_value(ModeBarButtonName::ZoomIn2d).unwrap(), json!("zoomIn2d"));
        assert_eq!(to_value(ModeBarButtonName::ZoomOut2d).unwrap(), json!("zoomOut2d"));
        assert_eq!(to_value(ModeBarButtonName::AutoScale2d).unwrap(), json!("autoScale2d"));
        assert_eq!(to_value(ModeBarButtonName::ResetScale2d).unwrap(), json!("resetScale2d"));
        assert_eq!(to_value(ModeBarButtonName::Zoom3d).unwrap(), json!("zoom3d"));
        assert_eq!(to_value(ModeBarButtonName::Pan3d).unwrap(), json!("pan3d"));
        assert_eq!(to_value(ModeBarButtonName::ResetCameraDefault3d).unwrap(), json!("resetCameraDefault3d"));
        assert_eq!(to_value(ModeBarButtonName::ResetCameraLastSave3d).unwrap(), json!("resetCameraLastSave3d"));
        assert_eq!(to_value(ModeBarButtonName::HoverClosest3d).unwrap(), json!("hoverClosest3d"));
        assert_eq!(to_value(ModeBarButtonName::OrbitRotation).unwrap(), json!("orbitRotation"));
        assert_eq!(to_value(ModeBarButtonName::TableRotation).unwrap(), json!("tableRotation"));
        assert_eq!(to_value(ModeBarButtonName::HoverClosestCartesian).unwrap(), json!("hoverClosestCartesian"));
        assert_eq!(to_value(ModeBarButtonName::HoverCompareCartesian).unwrap(), json!("hoverCompareCartesian"));
        assert_eq!(to_value(ModeBarButtonName::ZoomInGeo).unwrap(), json!("zoomInGeo"));
        assert_eq!(to_value(ModeBarButtonName::ZoomOutGeo).unwrap(), json!("zoomOutGeo"));
        assert_eq!(to_value(ModeBarButtonName::ResetGeo).unwrap(), json!("resetGeo"));
        assert_eq!(to_value(ModeBarButtonName::HoverClosestGeo).unwrap(), json!("hoverClosestGeo"));
        assert_eq!(to_value(ModeBarButtonName::HoverClosestGl2d).unwrap(), json!("hoverClosestGl2d"));
        assert_eq!(to_value(ModeBarButtonName::HoverClosestPie).unwrap(), json!("hoverClosestPie"));
        assert_eq!(to_value(ModeBarButtonName::ToggleHover).unwrap(), json!("toggleHover"));
        assert_eq!(to_value(ModeBarButtonName::ResetViews).unwrap(), json!("resetViews"));
        assert_eq!(to_value(ModeBarButtonName::ToImage).unwrap(), json!("toImage"));
        assert_eq!(to_value(ModeBarButtonName::SendDataToCloud).unwrap(), json!("sendDataToCloud"));
        assert_eq!(to_value(ModeBarButtonName::ToggleSpikelines).unwrap(), json!("toggleSpikelines"));
        assert_eq!(to_value(ModeBarButtonName::ResetViewMapbox).unwrap(), json!("resetViewMapbox"));
        assert_eq!(to_value(ModeBarButtonName::ZoomInMapbox).unwrap(), json!("zoomInMapbox"));
        assert_eq!(to_value(ModeBarButtonName::ZoomOutMapbox).unwrap(), json!("zoomOutMapbox"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_double_click() {
        assert_eq!(to_value(DoubleClick::False).unwrap(), json!(false));
        assert_eq!(to_value(DoubleClick::Reset).unwrap(), json!("reset"));
        assert_eq!(to_value(DoubleClick::AutoSize).unwrap(), json!("autosize"));
        assert_eq!(to_value(DoubleClick::ResetAutoSize).unwrap(), json!("reset+autosize"));
    }

    #[test]
    fn test_serialize_plot_gl_pixel_ratio() {
        assert_eq!(to_value(PlotGLPixelRatio::One).unwrap(), json!(1));
        assert_eq!(to_value(PlotGLPixelRatio::Two).unwrap(), json!(2));
        assert_eq!(to_value(PlotGLPixelRatio::Three).unwrap(), json!(3));
        assert_eq!(to_value(PlotGLPixelRatio::Four).unwrap(), json!(4));
    }

    #[test]
    fn test_serialize_configuration() {
        let config = Configuration::new()
            .static_plot(true)
            .typeset_math(true)
            .plotly_server_url("server_url")
            .editable(false)
            .autosizable(false)
            .responsive(true)
            .fill_frame(false)
            .frame_margins(2.0)
            .scroll_zoom(false)
            .double_click(DoubleClick::ResetAutoSize)
            .double_click_delay(50)
            .show_axis_drag_handles(false)
            .show_axis_range_entry_boxes(true)
            .show_tips(false)
            .show_link(true)
            .link_text("link text")
            .send_data(false)
            .display_mode_bar(DisplayModeBar::Hover)
            .show_send_to_cloud(true)
            .show_edit_in_chart_studio(false)
            .mode_bar_buttons_to_remove(vec![ModeBarButtonName::Zoom2d])
            .to_image_button_options(ToImageButtonOptions::new())
            .display_logo(false)
            .watermark(true)
            .plot_gl_pixel_ratio(PlotGLPixelRatio::Four)
            .topojson_url("topojson_url")
            .mapbox_access_token("123")
            .queue_length(100)
            .locale("en");

        let expected = json!({
            "staticPlot": true,
            "typesetMath": true,
            "plotlyServerURL": "server_url",
            "editable": false,
            "autosizable": false,
            "responsive": true,
            "fillFrame": false,
            "frameMargins": 2.0,
            "scrollZoom": false,
            "doubleClick": "reset+autosize",
            "doubleClickDelay": 50,
            "showAxisDragHandles": false,
            "showAxisRangeEntryBoxes": true,
            "showTips": false,
            "showLink": true,
            "linkText": "link text",
            "sendData": false,
            "displayModeBar": "hover",
            "showSendToCloud": true,
            "showEditInChartStudio": false,
            "modeBarButtonsToRemove": ["zoom2d"],
            "toImageButtonOptions": {},
            "displaylogo": false,
            "watermark": true,
            "plotGlPixelRatio": 4,
            "topojsonURL": "topojson_url",
            "mapboxAccessToken": "123",
            "queueLength": 100,
            "locale": "en"
        });

        assert_eq!(to_value(config).unwrap(), expected);
    }
}
