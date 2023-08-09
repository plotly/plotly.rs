//! Open-high-low-close (OHLC) trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{
        Calendar, Dim, Direction, HoverInfo, Label, LegendGroupTitle, Line, PlotType, Visible,
    },
    Trace,
};

/// Construct an OHLC trace.
///
/// # Examples
///
/// ```
/// use plotly::Ohlc;
///
/// let trace = Ohlc::new(
///     vec!["2022-08-22", "2022-08-23"],
///     vec![5, 6],
///     vec![8, 10],
///     vec![2, 4],
///     vec![6, 7],
/// );
///
/// let expected = serde_json::json!({
///     "type": "ohlc",
///     "x": ["2022-08-22", "2022-08-23"],
///     "open": [5, 6],
///     "high": [8, 10],
///     "low": [2, 4],
///     "close": [6, 7]
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Ohlc<X, O>
where
    X: Serialize + Clone,
    O: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Ohlc")]
    r#type: PlotType,
    x: Option<Vec<X>>,
    open: Option<Vec<O>>,
    high: Option<Vec<O>>,
    low: Option<Vec<O>>,
    close: Option<Vec<O>>,
    decreasing: Option<Direction>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    increasing: Option<Direction>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    line: Option<Line>,
    name: Option<String>,
    opacity: Option<f64>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    text: Option<Dim<String>>,
    #[serde(rename = "tickwidth")]
    tick_width: Option<f64>,
    visible: Option<Visible>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
}

impl<X, O> Ohlc<X, O>
where
    X: Serialize + Clone,
    O: Serialize + Clone,
{
    pub fn new(x: Vec<X>, open: Vec<O>, high: Vec<O>, low: Vec<O>, close: Vec<O>) -> Box<Self> {
        Box::new(Ohlc {
            r#type: PlotType::Ohlc,
            x: Some(x),
            open: Some(open),
            high: Some(high),
            low: Some(low),
            close: Some(close),
            ..Default::default()
        })
    }
}

impl<X, O> Trace for Ohlc<X, O>
where
    X: Serialize + Clone,
    O: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_default_ohlc() {
        let trace = Ohlc::<u32, u32>::default();
        let expected = json!({"type": "ohlc"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_ohlc() {
        let trace = Ohlc::new(
            vec![0, 1],
            vec![5.0, 6.0],
            vec![10.0, 15.0],
            vec![4.0, 2.0],
            vec![6.0, 13.0],
        )
        .decreasing(Direction::Decreasing { line: Line::new() })
        .hover_info(HoverInfo::X)
        .hover_label(Label::new())
        .hover_text_array(vec!["1", "2"])
        .hover_text("1")
        .increasing(Direction::Increasing { line: Line::new() })
        .legend_group("legendgroup")
        .legend_group_title("Legend Group Title")
        .line(Line::new())
        .name("ohlc_trace")
        .opacity(0.4)
        .show_legend(true)
        .text_array(vec!["3", "4"])
        .text("3")
        .tick_width(0.1)
        .visible(Visible::LegendOnly)
        .x_calendar(Calendar::Nepali);

        let expected = json!({
            "type": "ohlc",
            "x": [0, 1],
            "open": [5.0, 6.0],
            "high": [10.0, 15.0],
            "low": [4.0, 2.0],
            "close": [6.0, 13.0],
            "decreasing": {"line": {}},
            "hoverinfo": "x",
            "hoverlabel": {},
            "hovertext": "1",
            "increasing": {"line": {}},
            "legendgroup": "legendgroup",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "line": {},
            "name": "ohlc_trace",
            "opacity": 0.4,
            "showlegend": true,
            "text": "3",
            "tickwidth": 0.1,
            "visible": "legendonly",
            "xcalendar": "nepali"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
