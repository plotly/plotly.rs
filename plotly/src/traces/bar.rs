//! Bar trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{
        Calendar, ConstrainText, Dim, ErrorData, Font, HoverInfo, Label, LegendGroupTitle, Marker,
        Orientation, PlotType, TextAnchor, TextPosition, Visible,
    },
    Trace,
};

/// Construct a bar trace.
///
/// # Examples
///
/// ```
/// use plotly::Bar;
///
/// let x = vec![0, 1, 2, 3, 4, 5];
/// let y = vec![0, 2, 4, 6, 8, 10];
///
/// let trace = Bar::new(x, y).show_legend(true).opacity(0.5);
///
/// let expected = serde_json::json!({
///     "type": "bar",
///     "x": [0, 1, 2, 3, 4, 5],
///     "y": [0, 2, 4, 6, 8, 10],
///     "showlegend": true,
///     "opacity": 0.5
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Bar<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Bar")]
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
    width: Option<f64>,
    offset: Option<Dim<usize>>,
    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
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
    #[serde(rename = "textangle")]
    text_angle: Option<f64>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    error_x: Option<ErrorData>,
    error_y: Option<ErrorData>,
    #[serde(rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(rename = "constraintext")]
    constrain_text: Option<ConstrainText>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "insidetextanchor")]
    inside_text_anchor: Option<TextAnchor>,
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    #[serde(rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Bar<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Bar {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> Trace for Bar<X, Y>
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
    use crate::common::ErrorType;

    #[test]
    fn test_default_bar() {
        let trace: Bar<i32, i32> = Bar::default();
        let expected = json!({"type": "bar"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_serialize_bar() {
        let bar = Bar::new(vec![1, 2], vec![3, 4])
            .alignment_group("alignment_group")
            .clip_on_axis(true)
            .constrain_text(ConstrainText::Both)
            .error_x(ErrorData::new(ErrorType::Constant))
            .error_y(ErrorData::new(ErrorType::Percent))
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("tmpl")
            .hover_template_array(vec!["tmpl1", "tmpl2"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .ids(vec!["1"])
            .inside_text_anchor(TextAnchor::End)
            .inside_text_font(Font::new())
            .legend_group("legend-group")
            .legend_group_title("legend-group-title")
            .marker(Marker::new())
            .name("Bar")
            .offset(5)
            .offset_array(vec![5, 5])
            .offset_group("offset_group")
            .opacity(0.5)
            .orientation(Orientation::Vertical)
            .outside_text_font(Font::new())
            .show_legend(false)
            .text("text")
            .text_angle(0.05)
            .text_array(vec!["text"])
            .text_font(Font::new())
            .text_position(TextPosition::None)
            .text_position_array(vec![TextPosition::None])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .visible(Visible::LegendOnly)
            .width(999.0)
            .x_axis("xaxis")
            .x_calendar(Calendar::Nanakshahi)
            .y_axis("yaxis")
            .y_calendar(Calendar::Ummalqura);

        let expected = json!({
            "type": "bar",
            "hoverinfo": "all",
            "hovertemplate": ["tmpl1", "tmpl2"],
            "x": [1, 2],
            "y": [3, 4],
            "name": "Bar",
            "visible": "legendonly",
            "showlegend": false,
            "legendgroup": "legend-group",
            "legendgrouptitle": {"text": "legend-group-title"},
            "opacity": 0.5,
            "ids": ["1"],
            "width": 999.0,
            "offset": [5, 5],
            "text": ["text"],
            "textposition": ["none"],
            "texttemplate": ["text_template"],
            "hovertext": ["hover_text"],
            "xaxis": "xaxis",
            "yaxis": "yaxis",
            "orientation": "v",
            "alignmentgroup": "alignment_group",
            "offsetgroup": "offset_group",
            "marker": {},
            "textangle": 0.05,
            "textfont": {},
            "error_x": {"type": "constant"},
            "error_y": {"type": "percent"},
            "cliponaxis": true,
            "constraintext": "both",
            "hoverlabel": {},
            "insidetextanchor": "end",
            "insidetextfont": {},
            "outsidetextfont": {},
            "xcalendar": "nanakshahi",
            "ycalendar": "ummalqura",
        });

        assert_eq!(to_value(bar).unwrap(), expected);
    }
}
