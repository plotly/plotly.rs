use crate::charts::{owned_string_vector, Calendar, Color, Dim, HoverInfo, Label, Line, PlotType};
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Direction {
    Increasing { line: Line },
    Decreasing { line: Line },
}

#[derive(Serialize, Debug)]
pub struct Candlestick<T, O>
where
    T: Serialize,
    O: num::Num + Serialize,
{
    r#type: PlotType,
    x: Vec<T>,
    open: Vec<O>,
    high: Vec<O>,
    low: Vec<O>,
    close: Vec<O>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "whiskerwidth")]
    whisker_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decreasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
}

impl<T, O> Candlestick<T, O>
where
    T: Serialize,
    O: num::Num + Serialize,
{
    pub fn new(
        x: Vec<T>,
        open: Vec<O>,
        high: Vec<O>,
        low: Vec<O>,
        close: Vec<O>,
    ) -> Box<Candlestick<T, O>> {
        let mut iline = Line::new();
        iline.width = Some(1.0);
        iline.color = Some(Color::Green);
        let mut dline = Line::new();
        dline.width = Some(1.0);
        dline.color = Some(Color::Red);
        Box::new(Candlestick {
            r#type: PlotType::Candlestick,
            x,
            open,
            high,
            low,
            close,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            line: None,
            whisker_width: None,
            increasing: Some(Direction::Increasing { line: iline }),
            decreasing: Some(Direction::Decreasing { line: dline }),
            hover_label: None,
            x_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<Candlestick<T, O>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Candlestick<T, O>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Candlestick<T, O>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Candlestick<T, O>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Candlestick<T, O>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Candlestick<T, O>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Candlestick<T, O>> {
        let text = owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Candlestick<T, O>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Candlestick<T, O>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Candlestick<T, O>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Candlestick<T, O>> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn whisker_width(mut self, whisker_width: f64) -> Box<Candlestick<T, O>> {
        self.whisker_width = Some(whisker_width);
        Box::new(self)
    }

    pub fn increasing(mut self, increasing: Direction) -> Box<Candlestick<T, O>> {
        self.increasing = Some(increasing);
        Box::new(self)
    }

    pub fn decreasing(mut self, decreasing: Direction) -> Box<Candlestick<T, O>> {
        self.decreasing = Some(decreasing);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Candlestick<T, O>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Candlestick<T, O>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Candlestick<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct Ohlc<T, O>
where
    T: Serialize,
    O: num::Num + Serialize,
{
    r#type: PlotType,
    x: Vec<T>,
    open: Vec<O>,
    high: Vec<O>,
    low: Vec<O>,
    close: Vec<O>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decreasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickwidth")]
    tick_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
}

impl<T, O> Ohlc<T, O>
where
    T: Serialize,
    O: num::Num + Serialize,
{
    pub fn new(
        x: Vec<T>,
        open: Vec<O>,
        high: Vec<O>,
        low: Vec<O>,
        close: Vec<O>,
    ) -> Box<Ohlc<T, O>> {
        let mut iline = Line::new();
        iline.width = Some(2.0);
        iline.color = Some(Color::Green);
        let mut dline = Line::new();
        dline.width = Some(2.0);
        dline.color = Some(Color::Red);
        Box::new(Ohlc {
            r#type: PlotType::Ohlc,
            x,
            open,
            high,
            low,
            close,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            line: None,
            increasing: Some(Direction::Increasing { line: iline }),
            decreasing: Some(Direction::Decreasing { line: dline }),
            hover_label: None,
            tick_width: None,
            x_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<Ohlc<T, O>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Ohlc<T, O>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Ohlc<T, O>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Ohlc<T, O>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Ohlc<T, O>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Ohlc<T, O>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Ohlc<T, O>> {
        let text = owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Ohlc<T, O>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Ohlc<T, O>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Ohlc<T, O>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Ohlc<T, O>> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn increasing(mut self, increasing: Direction) -> Box<Ohlc<T, O>> {
        self.increasing = Some(increasing);
        Box::new(self)
    }

    pub fn decreasing(mut self, decreasing: Direction) -> Box<Ohlc<T, O>> {
        self.decreasing = Some(decreasing);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Ohlc<T, O>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn tick_width(mut self, tick_width: f64) -> Box<Ohlc<T, O>> {
        self.tick_width = Some(tick_width);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Ohlc<T, O>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Ohlc<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
