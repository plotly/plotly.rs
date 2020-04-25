//! Open-high-low-close (OHLC) plot

use crate::common::color::NamedColor;
use crate::common::{Calendar, Dim, Direction, HoverInfo, Label, Line, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

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
    pub fn new(x: Vec<T>, open: Vec<O>, high: Vec<O>, low: Vec<O>, close: Vec<O>) -> Ohlc<T, O> {
        let iline = Line::new().width(2.0).color(NamedColor::Green);
        let dline = Line::new().width(2.0).color(NamedColor::Red);
        Ohlc {
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
        }
    }

    pub fn name(mut self, name: &str) -> Ohlc<T, O> {
        self.name = Some(name.to_owned());
        self
    }

    pub fn visible(mut self, visible: bool) -> Ohlc<T, O> {
        self.visible = Some(visible);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> Ohlc<T, O> {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend_group(mut self, legend_group: &str) -> Ohlc<T, O> {
        self.legend_group = Some(legend_group.to_owned());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Ohlc<T, O> {
        self.opacity = Some(opacity);
        self
    }

    pub fn text(mut self, text: &str) -> Ohlc<T, O> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        self
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Ohlc<T, O> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        self
    }

    pub fn hover_text(mut self, hover_text: &str) -> Ohlc<T, O> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        self
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Ohlc<T, O> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Ohlc<T, O> {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn line(mut self, line: Line) -> Ohlc<T, O> {
        self.line = Some(line);
        self
    }

    pub fn increasing(mut self, increasing: Direction) -> Ohlc<T, O> {
        self.increasing = Some(increasing);
        self
    }

    pub fn decreasing(mut self, decreasing: Direction) -> Ohlc<T, O> {
        self.decreasing = Some(decreasing);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Ohlc<T, O> {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn tick_width(mut self, tick_width: f64) -> Ohlc<T, O> {
        self.tick_width = Some(tick_width);
        self
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Ohlc<T, O> {
        self.x_calendar = Some(x_calendar);
        self
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
