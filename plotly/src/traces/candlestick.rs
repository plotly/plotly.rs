//! Candlestick trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::NamedColor,
    common::{
        Calendar, Dim, Direction, HoverInfo, Label, LegendGroupTitle, Line, PlotType, Visible,
    },
    Trace,
};

/// Construct a candlestick trace.
///
/// # Examples
///
/// ```
/// use plotly::Candlestick;
///
/// let x = vec!["2020-05-22", "2020-05-23"];
/// let open = vec![5, 6];
/// let high = vec![9, 10];
/// let low = vec![3, 5];
/// let close = vec![6, 9];
///
/// let trace = Candlestick::new(x, open, high, low, close)
///     .name("trace_1")
///     .show_legend(true);
///
/// let expected = serde_json::json!({
///     "type": "candlestick",
///     "x": ["2020-05-22", "2020-05-23"],
///     "open": [5, 6],
///     "high": [9, 10],
///     "low": [3, 5],
///     "close": [6, 9],
///     "increasing": {"line": {"color": "green", "width": 1.0}},
///     "decreasing": {"line": {"color": "red", "width": 1.0}},
///     "name": "trace_1",
///     "showlegend": true
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Candlestick<T, O>
where
    T: Serialize + Clone,
    O: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Candlestick")]
    r#type: PlotType,
    x: Option<Vec<T>>,
    open: Option<Vec<O>>,
    high: Option<Vec<O>>,
    low: Option<Vec<O>>,
    close: Option<Vec<O>>,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    opacity: Option<f64>,
    text: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    line: Option<Line>,
    #[serde(rename = "whiskerwidth")]
    whisker_width: Option<f64>,
    increasing: Option<Direction>,
    decreasing: Option<Direction>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
}

impl<T, O> Candlestick<T, O>
where
    T: Serialize + Clone,
    O: Serialize + Clone,
{
    pub fn new(x: Vec<T>, open: Vec<O>, high: Vec<O>, low: Vec<O>, close: Vec<O>) -> Box<Self> {
        let iline = Line::new().width(1.0).color(NamedColor::Green);
        let dline = Line::new().width(1.0).color(NamedColor::Red);
        Box::new(Candlestick {
            x: Some(x),
            open: Some(open),
            high: Some(high),
            low: Some(low),
            close: Some(close),
            increasing: Some(Direction::Increasing { line: iline }),
            decreasing: Some(Direction::Decreasing { line: dline }),
            ..Default::default()
        })
    }
}

impl<X, Y> Trace for Candlestick<X, Y>
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
    fn test_default_candlestick() {
        let trace: Candlestick<i32, i32> = Candlestick::default();
        let expected = json!({"type": "candlestick"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_serialize_candlestick() {
        let trace = Candlestick::new(
            vec!["2020-05-20", "2020-05-21"],
            vec![5, 6],
            vec![9, 11],
            vec![2, 3],
            vec![6, 5],
        )
        .name("candlestick_trace")
        .visible(Visible::True)
        .show_legend(false)
        .legend_group("group_1")
        .legend_group_title(LegendGroupTitle::new("Legend Group Title"))
        .opacity(0.3)
        .text_array(vec!["text", "here"])
        .text("text here")
        .hover_text_array(vec!["hover", "text"])
        .hover_text("hover text")
        .hover_info(HoverInfo::Skip)
        .x_axis("x1")
        .y_axis("y1")
        .line(Line::new())
        .whisker_width(0.4)
        .increasing(Direction::Increasing { line: Line::new() })
        .decreasing(Direction::Decreasing { line: Line::new() })
        .hover_label(Label::new())
        .x_calendar(Calendar::DiscWorld);

        let expected = json!({
            "type": "candlestick",
            "x": ["2020-05-20", "2020-05-21"],
            "open": [5, 6],
            "high": [9, 11],
            "low": [2, 3],
            "close": [6, 5],
            "name": "candlestick_trace",
            "visible": true,
            "showlegend": false,
            "legendgroup": "group_1",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "opacity": 0.3,
            "text": "text here",
            "hovertext": "hover text",
            "hoverinfo": "skip",
            "xaxis": "x1",
            "yaxis": "y1",
            "line": {},
            "whiskerwidth": 0.4,
            "increasing": {"line": {}},
            "decreasing": {"line": {}},
            "hoverlabel": {},
            "xcalendar": "discworld"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
