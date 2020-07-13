//! Scatter plot

use crate::common::color::{Color, ColorWrapper};
use crate::common::{
    Calendar, Dim, ErrorData, Fill, Font, GroupNorm, HoverInfo, Label, Line, Marker, Mode,
    Orientation, PlotType, Position,
};
use crate::Trace;
use crate::{private, Plot};
use serde::Serialize;

use crate::private::copy_iterable_to_vec;
#[cfg(feature = "ndarray")]
use plotly_ndarray::{Array, Ix1, Ix2};

#[derive(Serialize, Debug)]
pub struct Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<X>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<Y>>,
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
    mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textposition")]
    text_position: Option<Dim<Position>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "groupnorm")]
    group_norm: Option<GroupNorm>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackgroup")]
    stack_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textfont")]
    text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_y: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill: Option<Fill>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillcolor")]
    fill_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoveron")]
    hover_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackgaps")]
    stack_gaps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Default for Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    fn default() -> Self {
        Scatter {
            r#type: PlotType::Scatter,
            x: None,
            y: None,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            mode: None,
            ids: None,
            text: None,
            text_position: None,
            text_template: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            x_axis: None,
            y_axis: None,
            orientation: None,
            group_norm: None,
            stack_group: None,
            marker: None,
            line: None,
            text_font: None,
            error_x: None,
            error_y: None,
            clip_on_axis: None,
            connect_gaps: None,
            fill: None,
            fill_color: None,
            hover_label: None,
            hover_on: None,
            stack_gaps: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<X, Y> Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    pub fn new<I, K>(x: I, y: K) -> Box<Self>
    where
        I: IntoIterator<Item = X>,
        K: IntoIterator<Item = Y>,
    {
        let x = copy_iterable_to_vec(x);
        let y = copy_iterable_to_vec(y);
        Box::new(Scatter {
            x: Some(x),
            y: Some(y),
            r#type: PlotType::Scatter,
            ..Default::default()
        })
    }

    #[cfg(feature = "ndarray")]
    pub fn from_array(x: Array<X, Ix1>, y: Array<Y, Ix1>) -> Box<Self> {
        Box::new(Scatter {
            x: Some(x.to_vec()),
            y: Some(y.to_vec()),
            r#type: PlotType::Scatter,
            ..Default::default()
        })
    }

    pub fn open_gl_mode(mut self, on: bool) -> Box<Self> {
        self.r#type = if on {
            PlotType::ScatterGL
        } else {
            PlotType::Scatter
        };
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn mode(mut self, mode: Mode) -> Box<Self> {
        self.mode = Some(mode);
        Box::new(self)
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Self> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn text_position(mut self, text_position: Position) -> Box<Self> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    pub fn text_position_array(mut self, text_position: Vec<Position>) -> Box<Self> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }

    pub fn text_template(mut self, text_template: &str) -> Box<Self> {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        Box::new(self)
    }

    pub fn text_template_array<S: AsRef<str>>(mut self, text_template: Vec<S>) -> Box<Self> {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Self> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn orientation(mut self, orientation: Orientation) -> Box<Self> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    pub fn group_norm(mut self, group_norm: GroupNorm) -> Box<Self> {
        self.group_norm = Some(group_norm);
        Box::new(self)
    }

    pub fn stack_group(mut self, stack_group: &str) -> Box<Self> {
        self.stack_group = Some(stack_group.to_owned());
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<Self> {
        self.marker = Some(marker);
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Self> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn text_font(mut self, text_font: Font) -> Box<Self> {
        self.text_font = Some(text_font);
        Box::new(self)
    }

    pub fn error_x(mut self, error_x: ErrorData) -> Box<Self> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    pub fn error_y(mut self, error_y: ErrorData) -> Box<Self> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    pub fn clip_on_axis(mut self, clip_on_axis: bool) -> Box<Self> {
        self.clip_on_axis = Some(clip_on_axis);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Self> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn fill(mut self, fill: Fill) -> Box<Self> {
        self.fill = Some(fill);
        Box::new(self)
    }

    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Box<Self> {
        self.fill_color = Some(fill_color.to_color());
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on(mut self, hover_on: &str) -> Box<Self> {
        self.hover_on = Some(hover_on.to_owned());
        Box::new(self)
    }

    pub fn stack_gaps(mut self, stack_gaps: &str) -> Box<Self> {
        self.stack_gaps = Some(stack_gaps.to_owned());
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
