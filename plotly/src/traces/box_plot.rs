//! Box trace

use plotly_derive::FieldSetter;
use serde::{Serialize, Serializer};

use crate::{
    color::Color,
    common::{
        Calendar, Dim, HoverInfo, Label, LegendGroupTitle, Line, Marker, Orientation, PlotType,
        Visible,
    },
    Trace,
};

#[derive(Debug, Clone)]
pub enum BoxMean {
    True,
    False,
    StandardDeviation,
}

impl Serialize for BoxMean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::True => serializer.serialize_bool(true),
            Self::False => serializer.serialize_bool(false),
            Self::StandardDeviation => serializer.serialize_str("sd"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoxPoints {
    All,
    Outliers,
    SuspectedOutliers,
    False,
}

impl Serialize for BoxPoints {
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum QuartileMethod {
    Linear,
    Exclusive,
    Inclusive,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HoverOn {
    Boxes,
    Points,
    #[serde(rename = "boxes+points")]
    BoxesAndPoints,
}

/// Construct a box trace.
///
/// # Examples
///
/// ```
/// use plotly::{BoxPlot, box_plot::BoxPoints};
///
/// let trace = BoxPlot::new(vec![0, 1, 2, 3, 4, 5])
///     .box_points(BoxPoints::All)
///     .jitter(0.3)
///     .point_pos(-1.8);
///
/// let expected = serde_json::json!({
///     "type": "box",
///     "y": [0, 1, 2, 3, 4, 5],
///     "boxpoints": "all",
///     "jitter": 0.3,
///     "pointpos": -1.8
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct BoxPlot<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Box")]
    r#type: PlotType,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
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
    width: Option<usize>,
    text: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    orientation: Option<Orientation>,
    #[serde(rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(rename = "offsetgroup")]
    offset_group: Option<String>,
    marker: Option<Marker>,
    line: Option<Line>,
    #[serde(rename = "boxmean")]
    box_mean: Option<BoxMean>,
    #[serde(rename = "boxpoints")]
    box_points: Option<BoxPoints>,
    notched: Option<bool>,
    #[serde(rename = "notchwidth")]
    notch_width: Option<f64>,
    #[serde(rename = "whiskerwidth")]
    whisker_width: Option<f64>,
    q1: Option<Vec<f64>>,
    median: Option<Vec<f64>>,
    q3: Option<Vec<f64>>,
    #[serde(rename = "upperfence")]
    upper_fence: Option<Vec<f64>>,
    #[serde(rename = "lowerfence")]
    lower_fence: Option<Vec<f64>>,
    #[serde(rename = "notchspan")]
    notch_span: Option<Vec<f64>>,
    mean: Option<Vec<f64>>,
    #[serde(rename = "sd")]
    standard_deviation: Option<Vec<f64>>,
    #[serde(rename = "quartilemethod")]
    quartile_method: Option<QuartileMethod>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoveron")]
    hover_on: Option<HoverOn>,
    #[serde(rename = "pointpos")]
    point_pos: Option<f64>,
    jitter: Option<f64>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<Y> BoxPlot<f64, Y>
where
    Y: Serialize + Clone,
{
    pub fn new(y: Vec<Y>) -> Box<BoxPlot<f64, Y>> {
        Box::new(BoxPlot {
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> BoxPlot<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    pub fn new_xy(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(BoxPlot {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> Trace for BoxPlot<X, Y>
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
    fn test_serialize_box_mean() {
        assert_eq!(to_value(BoxMean::True).unwrap(), json!(true));
        assert_eq!(to_value(BoxMean::False).unwrap(), json!(false));
        assert_eq!(to_value(BoxMean::StandardDeviation).unwrap(), json!("sd"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_box_points() {
        assert_eq!(to_value(BoxPoints::All).unwrap(), json!("all"));
        assert_eq!(to_value(BoxPoints::Outliers).unwrap(), json!("outliers"));
        assert_eq!(to_value(BoxPoints::SuspectedOutliers).unwrap(), json!("suspectedoutliers"));
        assert_eq!(to_value(BoxPoints::False).unwrap(), json!(false));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_quartile_method() {
        assert_eq!(to_value(QuartileMethod::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(QuartileMethod::Exclusive).unwrap(), json!("exclusive"));
        assert_eq!(to_value(QuartileMethod::Inclusive).unwrap(), json!("inclusive"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_hover_on() {
        assert_eq!(to_value(HoverOn::Boxes).unwrap(), json!("boxes"));
        assert_eq!(to_value(HoverOn::Points).unwrap(), json!("points"));
        assert_eq!(to_value(HoverOn::BoxesAndPoints).unwrap(), json!("boxes+points"));
    }

    #[test]
    fn test_default_box_plot() {
        let trace: BoxPlot<i32, i32> = BoxPlot::default();
        let expected = json!({"type": "box"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_box_plot_new() {
        let trace = BoxPlot::new(vec![0.0, 0.1]);
        let expected = json!({
            "type": "box",
            "y": [0.0, 0.1]
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_box_plot() {
        let trace = BoxPlot::new_xy(vec![1, 2, 3], vec![4, 5, 6])
            .alignment_group("alignment_group")
            .box_mean(BoxMean::StandardDeviation)
            .box_points(BoxPoints::All)
            .fill_color("#522622")
            .hover_info(HoverInfo::Name)
            .hover_label(Label::new())
            .hover_on(HoverOn::BoxesAndPoints)
            .hover_template("templ2")
            .hover_template_array(vec!["templ1", "templ2"])
            .hover_text("ok")
            .hover_text_array(vec!["okey", "dokey"])
            .ids(vec!["1", "2"])
            .jitter(0.5)
            .line(Line::new())
            .legend_group("one")
            .legend_group_title(LegendGroupTitle::new("Legend Group Title"))
            .lower_fence(vec![0., 1.])
            .marker(Marker::new())
            .mean(vec![12., 13.])
            .median(vec![4., 5.])
            .name("box")
            .notch_span(vec![10., 11.])
            .notch_width(0.1)
            .notched(true)
            .offset_group("offset_group")
            .opacity(0.6)
            .orientation(Orientation::Horizontal)
            .point_pos(-1.)
            .q1(vec![2., 3.])
            .q3(vec![6., 7.])
            .quartile_method(QuartileMethod::Exclusive)
            .show_legend(false)
            .standard_deviation(vec![14., 15.])
            .text("hi")
            .text_array(vec!["hi", "there"])
            .upper_fence(vec![8., 9.])
            .visible(Visible::LegendOnly)
            .whisker_width(0.2)
            .width(50)
            .x_axis("xaxis")
            .x_calendar(Calendar::Chinese)
            .y_axis("yaxis")
            .y_calendar(Calendar::Coptic);

        let expected = json!({
            "type": "box",
            "alignmentgroup": "alignment_group",
            "boxmean": "sd",
            "boxpoints": "all",
            "fillcolor": "#522622",
            "ids": ["1", "2"],
            "hoverinfo": "name",
            "hoverlabel": {},
            "hoveron": "boxes+points",
            "hovertemplate": ["templ1", "templ2"],
            "hovertext": ["okey", "dokey"],
            "jitter": 0.5,
            "legendgroup": "one",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "line": {},
            "lowerfence": [0.0, 1.0],
            "marker": {},
            "mean": [12.0, 13.0],
            "median": [4.0, 5.0],
            "name": "box",
            "notchspan": [10.0, 11.0],
            "notched": true,
            "notchwidth": 0.1,
            "offsetgroup": "offset_group",
            "opacity": 0.6,
            "orientation": "h",
            "pointpos": -1.0,
            "q1": [2.0, 3.0],
            "q3": [6.0, 7.0],
            "quartilemethod": "exclusive",
            "sd": [14.0, 15.0],
            "showlegend": false,
            "text": ["hi", "there"],
            "upperfence": [8.0, 9.0],
            "visible": "legendonly",
            "whiskerwidth": 0.2,
            "width": 50,
            "x": [1, 2, 3],
            "xaxis": "xaxis",
            "xcalendar": "chinese",
            "y": [4, 5, 6],
            "yaxis": "yaxis",
            "ycalendar": "coptic"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
