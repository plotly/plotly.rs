//! Scatter3D plot

#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1};
use plotly_derive::FieldSetter;
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
#[derive(Debug, FieldSetter, Clone, Serialize)]
pub struct ProjectionCoord {
    opacity: Option<f64>,
    scale: Option<f64>,
    show: Option<bool>,
}

impl ProjectionCoord {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, FieldSetter, Clone, Serialize)]
pub struct Projection {
    x: Option<ProjectionCoord>,
    y: Option<ProjectionCoord>,
    z: Option<ProjectionCoord>,
}

impl Projection {
    pub fn new() -> Self {
        Default::default()
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
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Scatter3D<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Scatter3D")]
    r#type: PlotType,
    /// Sets the trace name. The trace name is used as the label for the trace
    /// in the legend, as well as when the trace is hovered hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible. If
    /// `Visible::LegendOnly`, the trace is not drawn, but can appear as a
    /// legend item (provided that the legend itself is visible).
    visible: Option<Visible>,
    /// Determines whether or not an item corresponding to this trace is shown
    /// in the legend.
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    /// Sets the legend group for this trace. Traces part of the same legend
    /// group show/hide at the same time when toggling legend items.
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"
    /// `legend.traceorder` they are on bottom/right side. The default
    /// legendrank is 1000, so that you can use ranks less than 1000 to
    /// place certain items before all unranked items, and ranks greater than
    /// 1000 to go after all unranked items.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Sets the `LegendGroupTitle` object for the trace.
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    /// Sets the opacity of the trace.
    opacity: Option<f64>,
    /// Determines the drawing mode for this scatter trace. If the provided
    /// `Mode` includes "Text" then the `text` elements appear at the
    /// coordinates. Otherwise, the `text` elements appear on hover. If
    /// there are less than 20 points and the trace is not stacked then the
    /// default is `Mode::LinesMarkers`, otherwise it is `Mode::Lines`.
    mode: Option<Mode>,
    /// Assigns id labels to each datum. These ids for object constancy of data
    /// points during animation. Should be an array of strings, not numbers
    /// or any other type.
    ids: Option<Vec<String>>,

    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    z: Option<Vec<Z>>,

    /// Sets the surface fill color.
    #[serde(rename = "surfacecolor")]
    surface_color: Option<Box<dyn Color>>,
    /// Sets text element associated with each (x, y, z) triplet. The same tet
    /// will be applied to each data point. If the trace `HoverInfo`
    /// contains a "text" flag and `hover_text` is not set, these elements
    /// will be seen in the hover labels.
    text: Option<Dim<String>>,
    /// Sets the positions of the `text` elements with respects to the (x, y)
    /// coordinates.
    #[serde(rename = "textposition")]
    text_position: Option<Dim<Position>>,
    /// Template string used for rendering the information text that appear on
    /// points. Note that this will override `textinfo`. Variables are
    /// inserted using %{variable}, for example "y: %{y}". Numbers are
    /// formatted using d3-format's syntax %{variable:d3-format}, for example "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using
    /// d3-time-format's syntax %{variable|d3-time-format}, for example
    /// "Day: %{2019-01-01|%A}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified
    /// per-point (the ones that are `arrayOk: true`) are available.
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    /// Sets hover text elements associated with each (x, y, z) triplet. The
    /// same text will be associated with all datas points. To be seen, the
    /// trace `hover_info` must contain a "Text" flag.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Determines which trace information appears on hover. If
    /// `HoverInfo::None` or `HoverInfo::Skip` are set, no information is
    /// displayed upon hovering. But, if `HoverInfo::None` is set, click and
    /// hover events are still fired.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    /// Template string used for rendering the information that appear on hover
    /// box. Note that this will override `HoverInfo`. Variables are
    /// inserted using %{variable}, for example "y: %{y}". Numbers are
    /// formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's
    /// syntax %{variable|d3-time-format}, for example "Day:
    /// %{2019-01-01|%A}". https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in
    /// `hovertemplate` are the ones emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available. Anything contained in tag
    /// `<extra>` is displayed in the secondary box, for example
    /// "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    /// Sets the hover text formatting rulefor `x` using d3 formatting
    /// mini-languages which are very similar to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for
    /// dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's
    /// date formatter: "%h" for half of the year as a decimal number as well as
    /// "%{n}f" for fractional seconds with n digits. For example,
    /// "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display
    /// "09~15~23.46". By default the values are formatted using
    /// `x_axis.hover_format`.
    #[serde(rename = "xhoverformat")]
    x_hover_format: Option<String>,
    /// Sets the hover text formatting rulefor `y` using d3 formatting
    /// mini-languages which are very similar to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for
    /// dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's
    /// date formatter: "%h" for half of the year as a decimal number as well as
    /// "%{n}f" for fractional seconds with n digits. For example,
    /// "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display
    /// "09~15~23.46". By default the values are formatted using
    /// `y_axis.hover_format`.
    #[serde(rename = "yhoverformat")]
    y_hover_format: Option<String>,
    /// Sets the hover text formatting rulefor `z` using d3 formatting
    /// mini-languages which are very similar to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format. And for
    /// dates see: https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format. We add two items to d3's
    /// date formatter: "%h" for half of the year as a decimal number as well as
    /// "%{n}f" for fractional seconds with n digits. For example,
    /// "2016-10-13 09:15:23.456" with tickformat "%H~%M~%S.%2f" would display
    /// "09~15~23.46". By default the values are formatted using
    /// `z_axis.hover_format`.
    #[serde(rename = "zhoverformat")]
    z_hover_format: Option<String>,

    /// Assigns extra meta information associated with this trace that can be
    /// used in various text attributes. Attributes such as trace `name`,
    /// graph, axis and colorbar `title.text`, annotation `text`
    /// `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same
    /// trace, simply use `%{meta[i]}` where `i` is the index or key of the
    /// `meta` item in question. To access trace `meta` in layout
    /// attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of
    /// the `meta` and `n` is the trace index.
    meta: Option<private::NumOrString>,
    /// Assigns extra data each datum. This may be useful when listening to
    /// hover, click and selection events. Note that, "scatter" traces also
    /// appends customdata items in the markers DOM elements.
    #[serde(rename = "customdata")]
    custom_data: Option<private::NumOrStringCollection>,
    /// Sets a reference between this trace's 3D coordinate system and a 3D
    /// scene. If "scene" (the default value), the (x,y,z) coordinates refer
    /// to `layout.scene`. If "scene2", the (x, y, z) coordinates refer to
    /// `layout.scene2`, and so on.
    scene: Option<String>,
    /// Determines how points are displayed and joined.
    marker: Option<Marker>,
    /// Line display properties.
    line: Option<Line>,
    /// x-axis error display properties.
    error_x: Option<ErrorData>,
    /// y-axis error display properties.
    error_y: Option<ErrorData>,
    /// z-axis error display properties.
    error_z: Option<ErrorData>,
    /// Determines whether or not gaps (i.e. {nan} or missing values) in the
    /// provided data arrays are connected.
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    /// Properties of label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Configure the projection for each axis.
    projection: Option<Projection>,
    /// If `SurfaceAxis::MinusOne`, the scatter points are not filled with a
    /// surface. If one of the remaining three variants, the scatter points
    /// are filled with a Delaunay surface about the x, y, z respectively.
    #[serde(rename = "surfaceaxis")]
    surface_axis: Option<SurfaceAxis>,
    /// Sets the calendar system to use with `x` date data.
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    /// Sets the calendar system to use with `y` date data.
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    /// Sets the calendar system to use with `z` date data.
    #[serde(rename = "zcalendar")]
    z_calendar: Option<Calendar>,
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
