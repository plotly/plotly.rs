//! Violin trace

use plotly_derive::FieldSetter;
use serde::{Serialize, Serializer};

// Re-use the box plot's quartile method, whose values (linear/exclusive/inclusive)
// are identical to the violin trace's `quartilemethod` attribute.
pub use super::box_plot::QuartileMethod;
use crate::{
    color::Color,
    common::{
        Dim, HoverInfo, Label, LegendGroupTitle, Line, Marker, Orientation, PlotType, Visible,
        XAxisId, YAxisId,
    },
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

/// Determines which sample points are shown alongside the violin(s).
#[derive(Debug, Clone)]
pub enum ViolinPoints {
    All,
    Outliers,
    SuspectedOutliers,
    False,
}

impl Serialize for ViolinPoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::All => serializer.serialize_str("all"),
            Self::Outliers => serializer.serialize_str("outliers"),
            Self::SuspectedOutliers => serializer.serialize_str("suspectedoutliers"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

/// Sets the metric by which the width of each violin is determined.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ScaleMode {
    Width,
    Count,
}

/// Sets the method by which the span in data space (where the density function
/// is computed) is determined.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpanMode {
    Soft,
    Hard,
    Manual,
}

/// Determines on which side of the position value the density function making
/// up one half of a violin is plotted.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ViolinSide {
    Both,
    Positive,
    Negative,
}

/// Determines what the hover interactions highlight.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HoverOn {
    Violins,
    Points,
    Kde,
    #[serde(rename = "violins+points")]
    ViolinsAndPoints,
    #[serde(rename = "violins+points+kde")]
    ViolinsPointsAndKde,
    All,
}

/// A miniature box plot drawn inside the violins.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct ViolinBox {
    visible: Option<bool>,
    width: Option<f64>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    line: Option<Line>,
}

impl ViolinBox {
    pub fn new() -> Self {
        Default::default()
    }
}

/// A line corresponding to the sample's mean, drawn inside the violins.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct MeanLine {
    visible: Option<bool>,
    color: Option<Box<dyn Color>>,
    width: Option<f64>,
}

impl MeanLine {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Marker styling for selected or unselected points.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct SelectionMarker {
    color: Option<Box<dyn Color>>,
    size: Option<f64>,
    opacity: Option<f64>,
}

impl SelectionMarker {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Sets the styling of selected or unselected points.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Selection {
    marker: Option<SelectionMarker>,
}

impl Selection {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a violin trace.
///
/// # Examples
///
/// ```
/// use plotly::Violin;
///
/// let trace = Violin::new(vec![0, 1, 2, 3, 4, 5])
///     .box_plot(plotly::violin::ViolinBox::new().visible(true))
///     .mean_line(plotly::violin::MeanLine::new().visible(true));
///
/// let expected = serde_json::json!({
///     "type": "violin",
///     "y": [0, 1, 2, 3, 4, 5],
///     "box": {"visible": true},
///     "meanline": {"visible": true}
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Violin<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Violin")]
    r#type: PlotType,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    x0: Option<NumOrString>,
    y0: Option<NumOrString>,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    width: Option<f64>,
    text: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "xhoverformat")]
    x_hover_format: Option<String>,
    #[serde(rename = "yhoverformat")]
    y_hover_format: Option<String>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    orientation: Option<Orientation>,
    #[serde(rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(rename = "offsetgroup")]
    offset_group: Option<String>,
    marker: Option<Marker>,
    line: Option<Line>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    points: Option<ViolinPoints>,
    jitter: Option<f64>,
    #[serde(rename = "pointpos")]
    point_pos: Option<f64>,
    selected: Option<Selection>,
    unselected: Option<Selection>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoveron")]
    hover_on: Option<HoverOn>,
    #[serde(rename = "box")]
    box_plot: Option<ViolinBox>,
    #[serde(rename = "meanline")]
    mean_line: Option<MeanLine>,
    bandwidth: Option<f64>,
    #[serde(rename = "scalegroup")]
    scale_group: Option<String>,
    #[serde(rename = "scalemode")]
    scale_mode: Option<ScaleMode>,
    side: Option<ViolinSide>,
    span: Option<NumOrStringCollection>,
    #[serde(rename = "spanmode")]
    span_mode: Option<SpanMode>,
    #[serde(rename = "quartilemethod")]
    quartile_method: Option<QuartileMethod>,
    #[serde(rename = "zorder")]
    z_order: Option<i32>,
}

impl<Y> Violin<f64, Y>
where
    Y: Serialize + Clone,
{
    pub fn new(y: Vec<Y>) -> Box<Violin<f64, Y>> {
        Box::new(Violin {
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> Violin<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    pub fn new_xy(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Violin {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> Trace for Violin<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
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
    #[rustfmt::skip]
    fn serialize_violin_points() {
        assert_eq!(to_value(ViolinPoints::All).unwrap(), json!("all"));
        assert_eq!(to_value(ViolinPoints::Outliers).unwrap(), json!("outliers"));
        assert_eq!(to_value(ViolinPoints::SuspectedOutliers).unwrap(), json!("suspectedoutliers"));
        assert_eq!(to_value(ViolinPoints::False).unwrap(), json!(false));
    }

    #[test]
    fn serialize_scale_mode() {
        assert_eq!(to_value(ScaleMode::Width).unwrap(), json!("width"));
        assert_eq!(to_value(ScaleMode::Count).unwrap(), json!("count"));
    }

    #[test]
    fn serialize_span_mode() {
        assert_eq!(to_value(SpanMode::Soft).unwrap(), json!("soft"));
        assert_eq!(to_value(SpanMode::Hard).unwrap(), json!("hard"));
        assert_eq!(to_value(SpanMode::Manual).unwrap(), json!("manual"));
    }

    #[test]
    fn serialize_span_numeric_and_dates() {
        // Numeric span (typical case).
        let numeric = Violin::new(vec![1, 2, 3]).span(vec![0., 10.]);
        assert_eq!(to_value(numeric).unwrap()["span"], json!([0.0, 10.0]));

        // Date/string span for date-valued axes (plotly.js types `span` as `any`).
        let dated = Violin::new(vec![1, 2, 3])
            .span_mode(SpanMode::Manual)
            .span(vec!["2020-01-01", "2020-12-31"]);
        assert_eq!(
            to_value(dated).unwrap()["span"],
            json!(["2020-01-01", "2020-12-31"])
        );
    }

    #[test]
    fn serialize_violin_side() {
        assert_eq!(to_value(ViolinSide::Both).unwrap(), json!("both"));
        assert_eq!(to_value(ViolinSide::Positive).unwrap(), json!("positive"));
        assert_eq!(to_value(ViolinSide::Negative).unwrap(), json!("negative"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_hover_on() {
        assert_eq!(to_value(HoverOn::Violins).unwrap(), json!("violins"));
        assert_eq!(to_value(HoverOn::Points).unwrap(), json!("points"));
        assert_eq!(to_value(HoverOn::Kde).unwrap(), json!("kde"));
        assert_eq!(to_value(HoverOn::ViolinsAndPoints).unwrap(), json!("violins+points"));
        assert_eq!(to_value(HoverOn::ViolinsPointsAndKde).unwrap(), json!("violins+points+kde"));
        assert_eq!(to_value(HoverOn::All).unwrap(), json!("all"));
    }

    #[test]
    fn serialize_violin_box() {
        let violin_box = ViolinBox::new()
            .visible(true)
            .width(0.3)
            .fill_color("#ff0000")
            .line(Line::new());
        let expected = json!({
            "visible": true,
            "width": 0.3,
            "fillcolor": "#ff0000",
            "line": {}
        });

        assert_eq!(to_value(violin_box).unwrap(), expected);
    }

    #[test]
    fn serialize_mean_line() {
        let mean_line = MeanLine::new().visible(true).color("#00ff00").width(2.0);
        let expected = json!({
            "visible": true,
            "color": "#00ff00",
            "width": 2.0
        });

        assert_eq!(to_value(mean_line).unwrap(), expected);
    }

    #[test]
    fn serialize_selection() {
        let selection = Selection::new().marker(
            SelectionMarker::new()
                .color("#0000ff")
                .size(8.0)
                .opacity(0.5),
        );
        let expected = json!({
            "marker": {"color": "#0000ff", "size": 8.0, "opacity": 0.5}
        });

        assert_eq!(to_value(selection).unwrap(), expected);
    }

    #[test]
    fn default_violin() {
        let trace: Violin<i32, i32> = Violin::default();
        let expected = json!({"type": "violin"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn violin_new() {
        let trace = Violin::new(vec![0.0, 0.1]);
        let expected = json!({
            "type": "violin",
            "y": [0.0, 0.1]
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_violin() {
        let trace = Violin::new_xy(vec![1, 2, 3], vec![4, 5, 6])
            .alignment_group("alignment_group")
            .bandwidth(0.2)
            .box_plot(ViolinBox::new().visible(true))
            .fill_color("#522622")
            .hover_info(HoverInfo::Name)
            .hover_label(Label::new())
            .hover_on(HoverOn::ViolinsPointsAndKde)
            .hover_template("templ2")
            .hover_template_array(vec!["templ1", "templ2"])
            .hover_text("ok")
            .hover_text_array(vec!["okey", "dokey"])
            .ids(vec!["1", "2"])
            .jitter(0.5)
            .line(Line::new())
            .legend_group("one")
            .legend_group_title("Legend Group Title")
            .marker(Marker::new())
            .mean_line(MeanLine::new().visible(true))
            .name("violin")
            .offset_group("offset_group")
            .opacity(0.6)
            .orientation(Orientation::Horizontal)
            .points(ViolinPoints::All)
            .point_pos(-1.)
            .quartile_method(QuartileMethod::Exclusive)
            .scale_group("scale_group")
            .scale_mode(ScaleMode::Count)
            .selected(Selection::new().marker(SelectionMarker::new().color("#111111")))
            .show_legend(false)
            .side(ViolinSide::Positive)
            .span(vec![0., 10.])
            .span_mode(SpanMode::Manual)
            .text("hi")
            .text_array(vec!["hi", "there"])
            .unselected(Selection::new().marker(SelectionMarker::new().opacity(0.1)))
            .visible(Visible::LegendOnly)
            .width(50.0)
            .x0(1)
            .x_axis("x1")
            .x_hover_format("x_hover_format")
            .y0(2)
            .y_axis("y1")
            .y_hover_format("y_hover_format")
            .z_order(2);

        let expected = json!({
            "type": "violin",
            "alignmentgroup": "alignment_group",
            "bandwidth": 0.2,
            "box": {"visible": true},
            "fillcolor": "#522622",
            "ids": ["1", "2"],
            "hoverinfo": "name",
            "hoverlabel": {},
            "hoveron": "violins+points+kde",
            "hovertemplate": ["templ1", "templ2"],
            "hovertext": ["okey", "dokey"],
            "jitter": 0.5,
            "legendgroup": "one",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "line": {},
            "marker": {},
            "meanline": {"visible": true},
            "name": "violin",
            "offsetgroup": "offset_group",
            "opacity": 0.6,
            "orientation": "h",
            "points": "all",
            "pointpos": -1.0,
            "quartilemethod": "exclusive",
            "scalegroup": "scale_group",
            "scalemode": "count",
            "selected": {"marker": {"color": "#111111"}},
            "showlegend": false,
            "side": "positive",
            "span": [0.0, 10.0],
            "spanmode": "manual",
            "text": ["hi", "there"],
            "unselected": {"marker": {"opacity": 0.1}},
            "visible": "legendonly",
            "width": 50.0,
            "x": [1, 2, 3],
            "x0": 1,
            "xaxis": "x1",
            "xhoverformat": "x_hover_format",
            "y": [4, 5, 6],
            "y0": 2,
            "yaxis": "y1",
            "yhoverformat": "y_hover_format",
            "zorder": 2
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
