//! Carpet trace (a 2D `a`/`b` coordinate system drawn on cartesian axes)

use std::collections::HashMap;

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        DashType, ExponentFormat, Font, LegendGroupTitle, PlotType, TickFormatStop, Visible,
        XAxisId, YAxisId,
    },
    layout::{ArrayShow, AutoRange, AutoTypeNumbers, AxisType, CategoryOrder, RangeMode},
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

/// One- or two-dimensional cartesian coordinates of the points of a
/// [`Carpet`].
#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CarpetCoordinates {
    OneDimensional(Vec<f64>),
    TwoDimensional(Vec<Vec<f64>>),
}

impl From<Vec<f64>> for CarpetCoordinates {
    fn from(values: Vec<f64>) -> Self {
        Self::OneDimensional(values)
    }
}

impl From<Vec<Vec<f64>>> for CarpetCoordinates {
    fn from(values: Vec<Vec<f64>>) -> Self {
        Self::TwoDimensional(values)
    }
}

/// How the "cheater plot" x positions are computed when `x` is not provided.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CheaterType {
    Index,
    Value,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CarpetTickMode {
    Linear,
    Array,
}

/// Which ends of the carpet the tick labels of an axis are drawn on.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CarpetShowTickLabels {
    Start,
    End,
    Both,
    None,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct CarpetAxisTitle {
    text: Option<String>,
    font: Option<Font>,
    /// An additional amount by which to offset the title from the tick
    /// labels, in pixels.
    offset: Option<f64>,
}

impl CarpetAxisTitle {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&str> for CarpetAxisTitle {
    fn from(text: &str) -> Self {
        CarpetAxisTitle::new().text(text)
    }
}

impl From<String> for CarpetAxisTitle {
    fn from(text: String) -> Self {
        CarpetAxisTitle::new().text(text)
    }
}

/// The `a` or `b` axis of a [`Carpet`] trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct CarpetAxis {
    title: Option<CarpetAxisTitle>,
    #[serde(rename = "type")]
    axis_type: Option<AxisType>,
    #[serde(rename = "autotypenumbers")]
    auto_type_numbers: Option<AutoTypeNumbers>,
    /// `true`, `false` or [`AutoRange::Reversed`].
    #[serde(rename = "autorange")]
    auto_range: Option<AutoRange>,
    #[serde(rename = "rangemode")]
    range_mode: Option<RangeMode>,
    range: Option<[NumOrString; 2]>,
    #[serde(rename = "fixedrange")]
    fixed_range: Option<bool>,
    #[serde(rename = "cheatertype")]
    cheater_type: Option<CheaterType>,
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    #[serde(rename = "categoryarray")]
    category_array: Option<Vec<NumOrString>>,
    /// Sets the stride between grid lines along the axis, in array index
    /// units.
    #[serde(rename = "arraydtick")]
    array_dtick: Option<usize>,
    /// Sets the starting index of grid lines along the axis.
    #[serde(rename = "arraytick0")]
    array_tick0: Option<usize>,
    #[serde(rename = "tickmode")]
    tick_mode: Option<CarpetTickMode>,
    #[serde(rename = "nticks")]
    n_ticks: Option<usize>,
    #[serde(rename = "tick0")]
    tick0: Option<f64>,
    dtick: Option<f64>,
    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    #[serde(rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickformatstops")]
    tick_format_stops: Option<Vec<TickFormatStop>>,
    #[serde(rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(rename = "showtickprefix")]
    show_tick_prefix: Option<ArrayShow>,
    #[serde(rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(rename = "showticksuffix")]
    show_tick_suffix: Option<ArrayShow>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<CarpetShowTickLabels>,
    /// Replacement text for specific tick labels, keyed by the original
    /// label (e.g. `{"0.5": "half"}`).
    #[serde(rename = "labelalias")]
    label_alias: Option<HashMap<String, String>>,
    #[serde(rename = "labelpadding")]
    label_padding: Option<i32>,
    #[serde(rename = "labelprefix")]
    label_prefix: Option<String>,
    #[serde(rename = "labelsuffix")]
    label_suffix: Option<String>,
    #[serde(rename = "showexponent")]
    show_exponent: Option<ArrayShow>,
    #[serde(rename = "exponentformat")]
    exponent_format: Option<ExponentFormat>,
    #[serde(rename = "minexponent")]
    min_exponent: Option<f64>,
    #[serde(rename = "separatethousands")]
    separate_thousands: Option<bool>,
    color: Option<Box<dyn Color>>,
    #[serde(rename = "showline")]
    show_line: Option<bool>,
    #[serde(rename = "linecolor")]
    line_color: Option<Box<dyn Color>>,
    #[serde(rename = "linewidth")]
    line_width: Option<f64>,
    #[serde(rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(rename = "gridcolor")]
    grid_color: Option<Box<dyn Color>>,
    #[serde(rename = "gridwidth")]
    grid_width: Option<f64>,
    #[serde(rename = "griddash")]
    grid_dash: Option<DashType>,
    /// Sets the number of minor grid ticks per major grid tick.
    #[serde(rename = "minorgridcount")]
    minor_grid_count: Option<usize>,
    #[serde(rename = "minorgridcolor")]
    minor_grid_color: Option<Box<dyn Color>>,
    #[serde(rename = "minorgridwidth")]
    minor_grid_width: Option<f64>,
    #[serde(rename = "minorgriddash")]
    minor_grid_dash: Option<DashType>,
    /// Whether a line is drawn at the start of the axis.
    #[serde(rename = "startline")]
    start_line: Option<bool>,
    #[serde(rename = "startlinecolor")]
    start_line_color: Option<Box<dyn Color>>,
    #[serde(rename = "startlinewidth")]
    start_line_width: Option<f64>,
    /// Whether a line is drawn at the end of the axis.
    #[serde(rename = "endline")]
    end_line: Option<bool>,
    #[serde(rename = "endlinecolor")]
    end_line_color: Option<Box<dyn Color>>,
    #[serde(rename = "endlinewidth")]
    end_line_width: Option<f64>,
    /// Smoothing of the grid lines (0 to 1.3).
    smoothing: Option<f64>,
}

impl CarpetAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a carpet trace: a 2D coordinate system defined by the `a` and
/// `b` parameters, mapped onto cartesian `x`/`y` positions. Carpets are used
/// as the base for [`ScatterCarpet`](crate::ScatterCarpet) and
/// [`ContourCarpet`](crate::ContourCarpet) traces, which reference a carpet
/// through its [`carpet`](Carpet::carpet) id.
///
/// # Examples
///
/// ```
/// use plotly::Carpet;
///
/// let trace = Carpet::new(vec![4.0, 4.0, 5.0, 5.0], vec![1.0, 2.0, 1.0, 2.0])
///     .y(vec![2.0, 3.5, 4.0, 4.5])
///     .carpet("c1");
///
/// let expected = serde_json::json!({
///     "type": "carpet",
///     "a": [4.0, 4.0, 5.0, 5.0],
///     "b": [1.0, 2.0, 1.0, 2.0],
///     "y": [2.0, 3.5, 4.0, 4.5],
///     "carpet": "c1",
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Carpet<A, B>
where
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Carpet")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    /// An identifier for this carpet, so that `scattercarpet` and
    /// `contourcarpet` traces can specify a carpet plot on which they lie.
    carpet: Option<String>,
    /// An array containing values of the first parameter value.
    a: Option<Vec<A>>,
    /// Alternate to `a`: builds a linear space of `a` coordinates starting at
    /// `a0` with step `da`.
    a0: Option<f64>,
    da: Option<f64>,
    /// An array containing values of the second parameter value.
    b: Option<Vec<B>>,
    /// Alternate to `b`: builds a linear space of `b` coordinates starting at
    /// `b0` with step `db`.
    b0: Option<f64>,
    db: Option<f64>,
    /// A one- or two-dimensional array of x coordinates at each carpet point.
    /// If omitted, the plot is a cheater plot and the x axis is hidden.
    #[field_setter(skip)]
    x: Option<CarpetCoordinates>,
    /// A one- or two-dimensional array of y coordinates at each carpet point.
    #[field_setter(skip)]
    y: Option<CarpetCoordinates>,
    #[serde(rename = "aaxis")]
    a_axis: Option<CarpetAxis>,
    #[serde(rename = "baxis")]
    b_axis: Option<CarpetAxis>,
    /// The shift applied to each successive row of data in creating a
    /// cheater plot.
    #[serde(rename = "cheaterslope")]
    cheater_slope: Option<f64>,
    /// Sets the default color of axis lines and labels.
    color: Option<Box<dyn Color>>,
    /// The default font used for axis and tick labels.
    font: Option<Font>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    /// Sets the layer on which this trace is displayed, relative to other
    /// SVG traces on the same subplot.
    #[serde(rename = "zorder")]
    z_order: Option<i32>,
    meta: Option<NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<A, B> Carpet<A, B>
where
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    pub fn new(a: Vec<A>, b: Vec<B>) -> Box<Self> {
        Box::new(Self {
            a: Some(a),
            b: Some(b),
            ..Default::default()
        })
    }

    /// Sets the x coordinates at each carpet point, as a 1D or 2D array.
    pub fn x(mut self, x: impl Into<CarpetCoordinates>) -> Box<Self> {
        self.x = Some(x.into());
        Box::new(self)
    }

    /// Sets the y coordinates at each carpet point, as a 1D or 2D array.
    pub fn y(mut self, y: impl Into<CarpetCoordinates>) -> Box<Self> {
        self.y = Some(y.into());
        Box::new(self)
    }
}

impl<A, B> Trace for Carpet<A, B>
where
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_enums() {
        assert_eq!(to_value(CheaterType::Index).unwrap(), json!("index"));
        assert_eq!(to_value(CheaterType::Value).unwrap(), json!("value"));
        assert_eq!(to_value(CarpetTickMode::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(CarpetTickMode::Array).unwrap(), json!("array"));
        assert_eq!(
            to_value(CarpetShowTickLabels::Start).unwrap(),
            json!("start")
        );
        assert_eq!(to_value(CarpetShowTickLabels::End).unwrap(), json!("end"));
        assert_eq!(to_value(CarpetShowTickLabels::Both).unwrap(), json!("both"));
        assert_eq!(to_value(CarpetShowTickLabels::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_carpet_axis() {
        let axis = CarpetAxis::new()
            .title(
                CarpetAxisTitle::new()
                    .text("A")
                    .font(Font::new())
                    .offset(5.0),
            )
            .axis_type(AxisType::Linear)
            .auto_type_numbers(AutoTypeNumbers::Strict)
            .auto_range(AutoRange::Reversed)
            .range_mode(RangeMode::ToZero)
            .range([NumOrString::F(0.0), NumOrString::F(10.0)])
            .fixed_range(true)
            .cheater_type(CheaterType::Index)
            .category_order(CategoryOrder::Array)
            .category_array(vec![NumOrString::S("x".to_string())])
            .array_dtick(2)
            .array_tick0(1)
            .tick_mode(CarpetTickMode::Linear)
            .n_ticks(4)
            .tick0(0.0)
            .dtick(1.0)
            .tick_values(vec![1.0])
            .tick_text(vec!["one"])
            .tick_angle(30.0)
            .tick_font(Font::new())
            .tick_format(".1f")
            .tick_format_stops(vec![TickFormatStop::new()])
            .tick_prefix("p")
            .show_tick_prefix(ArrayShow::First)
            .tick_suffix("s")
            .show_tick_suffix(ArrayShow::Last)
            .show_tick_labels(CarpetShowTickLabels::Both)
            .label_alias(HashMap::from([("4".to_string(), "four".to_string())]))
            .label_padding(10)
            .label_prefix("a = ")
            .label_suffix("!")
            .show_exponent(ArrayShow::All)
            .exponent_format(ExponentFormat::SI)
            .min_exponent(2.0)
            .separate_thousands(false)
            .color("#111")
            .show_line(true)
            .line_color("#222")
            .line_width(1.5)
            .show_grid(true)
            .grid_color("#333")
            .grid_width(0.5)
            .grid_dash(DashType::Dot)
            .minor_grid_count(4)
            .minor_grid_color("#444")
            .minor_grid_width(0.25)
            .minor_grid_dash(DashType::Dash)
            .start_line(true)
            .start_line_color("#555")
            .start_line_width(2.0)
            .end_line(false)
            .end_line_color("#666")
            .end_line_width(3.0)
            .smoothing(1.0);

        let expected = json!({
            "title": {"text": "A", "font": {}, "offset": 5.0},
            "type": "linear",
            "autotypenumbers": "strict",
            "autorange": "reversed",
            "rangemode": "tozero",
            "range": [0.0, 10.0],
            "fixedrange": true,
            "cheatertype": "index",
            "categoryorder": "array",
            "categoryarray": ["x"],
            "arraydtick": 2,
            "arraytick0": 1,
            "tickmode": "linear",
            "nticks": 4,
            "tick0": 0.0,
            "dtick": 1.0,
            "tickvals": [1.0],
            "ticktext": ["one"],
            "tickangle": 30.0,
            "tickfont": {},
            "tickformat": ".1f",
            "tickformatstops": [{"enabled": true}],
            "tickprefix": "p",
            "showtickprefix": "first",
            "ticksuffix": "s",
            "showticksuffix": "last",
            "showticklabels": "both",
            "labelalias": {"4": "four"},
            "labelpadding": 10,
            "labelprefix": "a = ",
            "labelsuffix": "!",
            "showexponent": "all",
            "exponentformat": "SI",
            "minexponent": 2.0,
            "separatethousands": false,
            "color": "#111",
            "showline": true,
            "linecolor": "#222",
            "linewidth": 1.5,
            "showgrid": true,
            "gridcolor": "#333",
            "gridwidth": 0.5,
            "griddash": "dot",
            "minorgridcount": 4,
            "minorgridcolor": "#444",
            "minorgridwidth": 0.25,
            "minorgriddash": "dash",
            "startline": true,
            "startlinecolor": "#555",
            "startlinewidth": 2.0,
            "endline": false,
            "endlinecolor": "#666",
            "endlinewidth": 3.0,
            "smoothing": 1.0,
        });

        assert_eq!(to_value(axis).unwrap(), expected);
    }

    #[test]
    fn serialize_carpet() {
        let trace = Carpet::new(vec![1.0, 2.0], vec![3.0, 4.0])
            .name("carpet")
            .visible(Visible::True)
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.9)
            .ids(vec!["i"])
            .carpet("c1")
            .a0(0.0)
            .da(1.0)
            .b0(0.0)
            .db(1.0)
            .x(vec![vec![0.0, 1.0], vec![2.0, 3.0]])
            .y(vec![vec![4.0, 5.0], vec![6.0, 7.0]])
            .a_axis(CarpetAxis::new().title("a".into()))
            .b_axis(CarpetAxis::new())
            .cheater_slope(1.5)
            .color("#444")
            .font(Font::new())
            .x_axis("x2")
            .y_axis("y2")
            .z_order(3)
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "carpet",
            "name": "carpet",
            "visible": true,
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.9,
            "ids": ["i"],
            "carpet": "c1",
            "a": [1.0, 2.0],
            "a0": 0.0,
            "da": 1.0,
            "b": [3.0, 4.0],
            "b0": 0.0,
            "db": 1.0,
            "x": [[0.0, 1.0], [2.0, 3.0]],
            "y": [[4.0, 5.0], [6.0, 7.0]],
            "aaxis": {"title": {"text": "a"}},
            "baxis": {},
            "cheaterslope": 1.5,
            "color": "#444",
            "font": {},
            "xaxis": "x2",
            "yaxis": "y2",
            "zorder": 3,
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": 1,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
