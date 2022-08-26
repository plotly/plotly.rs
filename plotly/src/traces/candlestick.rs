//! Candlestick trace

use serde::Serialize;

use crate::{
    color::NamedColor,
    common::{Calendar, Dim, Direction, HoverInfo, Label, Line, PlotType, Visible},
    private, Trace,
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
#[derive(Serialize, Debug, Clone)]
pub struct Candlestick<T, O>
where
    T: Serialize + Clone,
    O: Serialize + Clone,
{
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

impl<T, O> Default for Candlestick<T, O>
where
    T: Serialize + Clone,
    O: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            r#type: PlotType::Candlestick,
            x: None,
            open: None,
            high: None,
            low: None,
            close: None,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            x_axis: None,
            y_axis: None,
            line: None,
            whisker_width: None,
            increasing: None,
            decreasing: None,
            hover_label: None,
            x_calendar: None,
        }
    }
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

    pub fn decreasing(mut self, decreasing: Direction) -> Box<Self> {
        self.decreasing = Some(decreasing);
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_string()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn increasing(mut self, increasing: Direction) -> Box<Self> {
        self.increasing = Some(increasing);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_string());
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Self> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_string()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn whisker_width(mut self, whisker_width: f64) -> Box<Self> {
        self.whisker_width = Some(whisker_width);
        Box::new(self)
    }
    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_string());
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_string());
        Box::new(self)
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
