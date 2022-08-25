//! Scatter3D plot

#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1};
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        Calendar, Dim, ErrorData, HoverInfo, Label, LegendGroupTitle, Line, Marker, Mode, PlotType,
        Position, Visible,
    },
    private, Trace,
};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize)]
pub struct ProjectionCoord {
    opacity: Option<f64>,
    scale: Option<f64>,
    show: Option<bool>,
}

impl ProjectionCoord {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the projection opacity.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Sets the scale factor determining the size of the projection marker points.
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }

    /// Sets whether or not projections are shown along the current axis.
    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize)]
pub struct Projection {
    x: Option<ProjectionCoord>,
    y: Option<ProjectionCoord>,
    z: Option<ProjectionCoord>,
}

impl Projection {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn x(mut self, x: ProjectionCoord) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: ProjectionCoord) -> Self {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: ProjectionCoord) -> Self {
        self.z = Some(z);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum SurfaceAxis {
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
}

/// Construct a scatter3D trace.
///
/// # Examples
///
/// ```
/// use plotly::Scatter3D;
///
/// let trace = Scatter3D::new(
///     vec![0.0, 1.0],
///     vec![2.0, 3.0],
///     vec![4.0, 5.0],
/// );
///
/// let expected = serde_json::json!({
///     "type": "scatter3d",
///     "x": [0.0, 1.0],
///     "y": [2.0, 3.0],
///     "z": [4.0, 5.0],
///
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Scatter3D<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    opacity: Option<f64>,
    mode: Option<Mode>,
    ids: Option<Vec<String>>,

    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    z: Option<Vec<Z>>,

    #[serde(rename = "surfacecolor")]
    surface_color: Option<Box<dyn Color>>,
    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<Position>>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "xhoverformat")]
    x_hover_format: Option<String>,
    #[serde(rename = "yhoverformat")]
    y_hover_format: Option<String>,
    #[serde(rename = "zhoverformat")]
    z_hover_format: Option<String>,

    meta: Option<private::NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<private::NumOrStringCollection>,
    scene: Option<String>,
    marker: Option<Marker>,
    line: Option<Line>,
    error_x: Option<ErrorData>,
    error_y: Option<ErrorData>,
    error_z: Option<ErrorData>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    projection: Option<Projection>,
    #[serde(rename = "surfaceaxis")]
    surface_axis: Option<SurfaceAxis>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    #[serde(rename = "zcalendar")]
    z_calendar: Option<Calendar>,
}

impl<X, Y, Z> Default for Scatter3D<X, Y, Z>
where
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
    Z: Serialize + Default + Clone,
{
    fn default() -> Self {
        Self {
            r#type: PlotType::Scatter3D,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            legend_rank: None,
            legend_group_title: None,
            opacity: None,
            mode: None,
            ids: None,
            x: None,
            y: None,
            z: None,
            surface_color: None,
            text: None,
            text_position: None,
            text_template: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            x_hover_format: None,
            y_hover_format: None,
            z_hover_format: None,
            meta: None,
            custom_data: None,
            scene: None,
            marker: None,
            line: None,
            error_x: None,
            error_y: None,
            error_z: None,
            connect_gaps: None,
            hover_label: None,
            projection: None,
            surface_axis: None,
            x_calendar: None,
            y_calendar: None,
            z_calendar: None,
        }
    }
}

impl<X, Y, Z> Scatter3D<X, Y, Z>
where
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
    Z: Serialize + Default + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            r#type: PlotType::Scatter3D,
            x: Some(x),
            y: Some(y),
            z: Some(z),
            ..Default::default()
        })
    }

    #[cfg(feature = "plotly_ndarray")]
    pub fn from_array(x: Array<X, Ix1>, y: Array<Y, Ix1>, z: Array<Z, Ix1>) -> Box<Self> {
        Box::new(Scatter3D {
            r#type: PlotType::Scatter3D,
            x: Some(x.to_vec()),
            y: Some(y.to_vec()),
            z: Some(z.to_vec()),
            ..Default::default()
        })
    }

    /// Sets the trace name. The trace name is used as the label for the trace in the legend, as well
    /// as when the trace is hovered hover.
    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
        Box::new(self)
    }

    /// Determines whether or not this trace is visible. If `Visible::LegendOnly`, the trace is not
    /// drawn, but can appear as a legend item (provided that the legend itself is visible).
    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    /// Determines whether or not an item corresponding to this trace is shown in the legend.
    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    /// Sets the legend group for this trace. Traces part of the same legend group show/hide at the
    /// same time when toggling legend items.
    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_string());
        Box::new(self)
    }

    /// Sets the `LegendGroupTitle` object for the trace.
    pub fn legend_group_title(mut self, legend_group_title: LegendGroupTitle) -> Box<Self> {
        self.legend_group_title = Some(legend_group_title);
        Box::new(self)
    }

    /// Sets the legend rank for this trace. Items and groups with smaller ranks are presented on top/left side while with
    /// `"reversed" `legend.traceorder` they are on bottom/right side. The default legendrank is 1000, so that you
    /// can use ranks less than 1000 to place certain items before all unranked items, and ranks greater than 1000 to
    /// go after all unranked items.
    pub fn legend_rank(mut self, legend_rank: usize) -> Box<Self> {
        self.legend_rank = Some(legend_rank);
        Box::new(self)
    }

    /// Sets the opacity of the trace.
    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    /// Determines the drawing mode for this scatter trace. If the provided `Mode` includes
    /// "Text" then the `text` elements appear at the coordinates. Otherwise, the `text` elements
    /// appear on hover. If there are less than 20 points and the trace is not stacked then the
    /// default is `Mode::LinesMarkers`, otherwise it is `Mode::Lines`.
    pub fn mode(mut self, mode: Mode) -> Box<Self> {
        self.mode = Some(mode);
        Box::new(self)
    }

    /// Assigns id labels to each datum. These ids for object constancy of data points during
    /// animation. Should be an array of strings, not numbers or any other type.
    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    /// Sets the surface fill color.
    pub fn surface_color<C: Color>(mut self, color: C) -> Box<Self> {
        self.surface_color = Some(Box::new(color));
        Box::new(self)
    }

    /// Sets text element associated with each (x, y, z) triplet. The same tet will be applied to each data
    /// point. If the trace `HoverInfo` contains a "text" flag and `hover_text` is not set, these elements
    /// will be seen in the hover labels.
    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_string()));
        Box::new(self)
    }

    /// Sets text elements associated with each (x, y, z) triplet. The items are mapped sequentially to
    /// this trace's (x, y, z) coordinates. If trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover labels.
    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    /// Sets the positions of the `text` elements with respects to the (x, y) coordinates.
    pub fn text_position(mut self, text_position: Position) -> Box<Self> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    /// Sets the positions of the `text` elements with respects to the (x, y) coordinates.
    pub fn text_position_array(mut self, text_position: Vec<Position>) -> Box<Self> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }

    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    pub fn text_template(mut self, text_template: &str) -> Box<Self> {
        self.text_template = Some(Dim::Scalar(text_template.to_string()));
        Box::new(self)
    }

    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    pub fn text_template_array<S: AsRef<str>>(mut self, text_template: Vec<S>) -> Box<Self> {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    /// Sets hover text elements associated with each (x, y, z) triplet. The same text will be associated
    /// with all datas points. To be seen, the trace `hover_info` must contain a "Text" flag.
    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_string()));
        Box::new(self)
    }

    /// Sets hover text elements associated with each (x, y, z) triplet. The items are mapped sequentially across
    /// this trace's (x,y) coordinates. To be seen, the trace `hover_info` must contain a "Text" flag.
    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    /// Determines which trace information appears on hover. If `HoverInfo::None` or `HoverInfo::Skip`
    /// are set, no information is displayed upon hovering. But, if `HoverInfo::None` is set, click
    /// and hover events are still fired.
    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_string()));
        Box::new(self)
    }

    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    /// Sets the hover text formatting rulefor `x` using d3 formatting mini-languages which are very similar
    /// to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for
    /// dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's
    /// date formatter: "%h" for half of the year as a decimal number as well as "%{n}f" for fractional seconds
    /// with n digits. For example, "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display
    /// "09~15~23.46". By default the values are formatted using `x_axis.hover_format`.
    pub fn x_hover_format(mut self, hover_format: &str) -> Box<Self> {
        self.x_hover_format = Some(hover_format.to_string());
        Box::new(self)
    }

    /// Sets the hover text formatting rulefor `y` using d3 formatting mini-languages which are very similar
    /// to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for
    /// dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's
    /// date formatter: "%h" for half of the year as a decimal number as well as "%{n}f" for fractional seconds
    /// with n digits. For example, "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display
    /// "09~15~23.46". By default the values are formatted using `y_axis.hover_format`.
    pub fn y_hover_format(mut self, hover_format: &str) -> Box<Self> {
        self.y_hover_format = Some(hover_format.to_string());
        Box::new(self)
    }

    /// Sets the hover text formatting rulefor `z` using d3 formatting mini-languages which are very similar
    /// to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for
    /// dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's
    /// date formatter: "%h" for half of the year as a decimal number as well as "%{n}f" for fractional seconds
    /// with n digits. For example, "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display
    /// "09~15~23.46". By default the values are formatted using `z_axis.hover_format`.
    pub fn z_hover_format(mut self, hover_format: &str) -> Box<Self> {
        self.z_hover_format = Some(hover_format.to_string());
        Box::new(self)
    }

    /// Assigns extra meta information associated with this trace that can be used in various text
    /// attributes. Attributes such as trace `name`, graph, axis and colorbar `title.text`,
    /// annotation `text` `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same trace, simply use
    /// `%{meta[i]}` where `i` is the index or key of the `meta` item in question. To access trace
    /// `meta` in layout attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of the
    /// `meta` and `n` is the trace index.
    pub fn meta<V: Into<private::NumOrString>>(mut self, meta: V) -> Box<Self> {
        self.meta = Some(meta.into());
        Box::new(self)
    }

    /// Assigns extra data each datum. This may be useful when listening to hover, click and
    /// selection events. Note that, "scatter" traces also appends customdata items in the markers
    /// DOM elements.
    pub fn custom_data<V: Into<private::NumOrString> + Clone>(
        mut self,
        custom_data: Vec<V>,
    ) -> Box<Self> {
        self.custom_data = Some(custom_data.into());
        Box::new(self)
    }

    /// Sets a reference between this trace's 3D coordinate system and a 3D scene. If "scene" (the
    /// default value), the (x,y,z) coordinates refer to `layout.scene`. If "scene2", the (x, y, z)
    /// coordinates refer to `layout.scene2`, and so on.
    pub fn scene(mut self, scene: &str) -> Box<Self> {
        self.scene = Some(scene.to_string());
        Box::new(self)
    }

    /// Configure the projection for each axis.
    pub fn projection(mut self, projection: Projection) -> Box<Self> {
        self.projection = Some(projection);
        Box::new(self)
    }

    /// Determines how points are displayed and joined.
    pub fn marker(mut self, marker: Marker) -> Box<Self> {
        self.marker = Some(marker);
        Box::new(self)
    }

    /// Line display properties.
    pub fn line(mut self, line: Line) -> Box<Self> {
        self.line = Some(line);
        Box::new(self)
    }

    /// x-axis error display properties.
    pub fn error_x(mut self, error_x: ErrorData) -> Box<Self> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    /// y-axis error display properties.
    pub fn error_y(mut self, error_y: ErrorData) -> Box<Self> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    /// z-axis error display properties.
    pub fn error_z(mut self, error_z: ErrorData) -> Box<Self> {
        self.error_z = Some(error_z);
        Box::new(self)
    }

    /// Determines whether or not gaps (i.e. {nan} or missing values) in the provided data arrays
    /// are connected.
    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Self> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    /// Properties of label displayed on mouse hover.
    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    /// If `SurfaceAxis::MinusOne`, the scatter points are not filled with a surface. If one of the remaining
    /// three variants, the scatter points are filled with a Delaunay surface about the x, y, z respectively.
    pub fn surface_axis(mut self, surface_axis: SurfaceAxis) -> Box<Self> {
        self.surface_axis = Some(surface_axis);
        Box::new(self)
    }

    /// Sets the calendar system to use with `x` date data.
    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    /// Sets the calendar system to use with `y` date data.
    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }

    /// Sets the calendar system to use with `z` date data.
    pub fn z_calendar(mut self, z_calendar: Calendar) -> Box<Self> {
        self.z_calendar = Some(z_calendar);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for Scatter3D<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ErrorType;

    #[test]
    fn test_serialize_projection() {
        let projection = Projection::new()
            .x(ProjectionCoord::new())
            .y(ProjectionCoord::new())
            .z(ProjectionCoord::new());
        let expected = json!({"x": {}, "y": {}, "z": {}});

        assert_eq!(to_value(projection).unwrap(), expected);
    }

    #[test]
    fn test_serialize_projection_coord() {
        let projection_coord = ProjectionCoord::new().opacity(0.75).scale(5.0).show(false);
        let expected = json!({"opacity": 0.75, "scale": 5.0, "show": false});

        assert_eq!(to_value(projection_coord).unwrap(), expected);
    }

    #[test]
    fn test_serialize_surface_axis() {
        assert_eq!(to_value(SurfaceAxis::MinusOne).unwrap(), json!("-1"));
        assert_eq!(to_value(SurfaceAxis::Zero).unwrap(), json!("0"));
        assert_eq!(to_value(SurfaceAxis::One).unwrap(), json!("1"));
        assert_eq!(to_value(SurfaceAxis::Two).unwrap(), json!("2"));
    }

    #[test]
    fn test_serialize_default_scatter3d() {
        let trace = Scatter3D::<f64, f64, f64>::default();
        let expected = json!({"type": "scatter3d"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_serialize_scatter3d() {
        let trace = Scatter3D::new(vec![0, 1], vec![2, 3], vec![4, 5])
            .connect_gaps(true)
            .custom_data(vec!["custom_data"])
            .error_x(ErrorData::new(ErrorType::SquareRoot))
            .error_y(ErrorData::new(ErrorType::Percent))
            .error_z(ErrorData::new(ErrorType::Data))
            .hover_label(Label::new())
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .hover_info(HoverInfo::XAndYAndZ)
            .hover_template("hover_template")
            .hover_template_array(vec!["hover_template"])
            .ids(vec!["1"])
            .legend_group("legend_group")
            .legend_rank(1000)
            .legend_group_title(LegendGroupTitle::new("Legend Group Title"))
            .line(Line::new())
            .marker(Marker::new())
            .meta("meta")
            .mode(Mode::LinesText)
            .name("trace_name")
            .opacity(0.2)
            .projection(Projection::new())
            .scene("scene2")
            .show_legend(true)
            .surface_axis(SurfaceAxis::One)
            .surface_color("#123456")
            .text("text")
            .text_array(vec!["text"])
            .text_position(Position::BottomLeft)
            .text_position_array(vec![Position::TopCenter])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .visible(Visible::True)
            .x_calendar(Calendar::Chinese)
            .x_hover_format("x_hover_format")
            .y_calendar(Calendar::Coptic)
            .y_hover_format("y_hover_format")
            .z_calendar(Calendar::Ummalqura)
            .z_hover_format("z_hover_format");

        let expected = json!({
            "type": "scatter3d",
            "connectgaps": true,
            "customdata": ["custom_data"],
            "error_x": {"type": "sqrt"},
            "error_y": {"type": "percent"},
            "error_z": {"type": "data"},
            "ids": ["1"],
            "hoverinfo": "x+y+z",
            "hoverlabel": {},
            "hovertemplate": ["hover_template"],
            "hovertext": ["hover_text"],
            "legendgroup": "legend_group",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "legendrank": 1000,
            "line": {},
            "marker": {},
            "meta": "meta",
            "mode": "lines+text",
            "name": "trace_name",
            "opacity": 0.2,
            "projection": {},
            "scene": "scene2",
            "showlegend": true,
            "surfaceaxis": "1",
            "surfacecolor": "#123456",
            "text": ["text"],
            "textposition": ["top center"],
            "texttemplate": ["text_template"],
            "visible": true,
            "x": [0, 1],
            "xhoverformat": "x_hover_format",
            "xcalendar": "chinese",
            "y": [2, 3],
            "ycalendar": "coptic",
            "yhoverformat": "y_hover_format",
            "z": [4, 5],
            "zcalendar": "ummalqura",
            "zhoverformat": "z_hover_format",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
