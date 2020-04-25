//! Scatter plot

use crate::common::color::Color;
use crate::common::{
    Calendar, Dim, ErrorData, Fill, Font, GroupNorm, HoverInfo, Label, Line, Marker, Mode,
    Orientation, PlotType, Position,
};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Scatter<X, Y, Tx, Ty>
where
    X: AsRef<[Tx]>,
    Y: AsRef<[Ty]>,
    Tx: Serialize,
    Ty: num::Num + Serialize,
{
    r#type: PlotType,
    x: X,
    pd_tx: std::marker::PhantomData<Tx>,
    y: Y,
    pd_ty: std::marker::PhantomData<Ty>,
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
    fill_color: Option<String>,
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

impl<X, Y, Tx, Ty> Scatter<X, Y, Tx, Ty>
where
    X: AsRef<[Tx]>,
    Y: AsRef<[Ty]>,
    Tx: Serialize,
    Ty: num::Num + Serialize,
{
    pub fn new(x: X, y: Y) -> Scatter<X, Y, Tx, Ty> {
        Scatter {
            x,
            pd_tx: std::marker::PhantomData,
            y,
            pd_ty: std::marker::PhantomData,
            r#type: PlotType::Scatter,
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

    pub fn name(mut self, name: &str) -> Scatter<X, Y, Tx, Ty> {
        self.name = Some(name.to_owned());
        self
    }

    pub fn visible(mut self, visible: bool) -> Scatter<X, Y, Tx, Ty> {
        self.visible = Some(visible);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> Scatter<X, Y, Tx, Ty> {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend_group(mut self, legend_group: &str) -> Scatter<X, Y, Tx, Ty> {
        self.legend_group = Some(legend_group.to_owned());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Scatter<X, Y, Tx, Ty> {
        self.opacity = Some(opacity);
        self
    }

    pub fn mode(mut self, mode: Mode) -> Scatter<X, Y, Tx, Ty> {
        self.mode = Some(mode);
        self
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Scatter<X, Y, Tx, Ty> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        self
    }

    pub fn text(mut self, text: &str) -> Scatter<X, Y, Tx, Ty> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        self
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Scatter<X, Y, Tx, Ty> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        self
    }

    pub fn text_position(mut self, text_position: Position) -> Scatter<X, Y, Tx, Ty> {
        self.text_position = Some(Dim::Scalar(text_position));
        self
    }

    pub fn text_position_array(mut self, text_position: Vec<Position>) -> Scatter<X, Y, Tx, Ty> {
        self.text_position = Some(Dim::Vector(text_position));
        self
    }

    pub fn text_template(mut self, text_template: &str) -> Scatter<X, Y, Tx, Ty> {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        self
    }

    pub fn text_template_array<S: AsRef<str>>(
        mut self,
        text_template: Vec<S>,
    ) -> Scatter<X, Y, Tx, Ty> {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        self
    }

    pub fn hover_text(mut self, hover_text: &str) -> Scatter<X, Y, Tx, Ty> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        self
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Scatter<X, Y, Tx, Ty> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Scatter<X, Y, Tx, Ty> {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn hover_template(mut self, hover_template: &str) -> Scatter<X, Y, Tx, Ty> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        self
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Scatter<X, Y, Tx, Ty> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Scatter<X, Y, Tx, Ty> {
        self.orientation = Some(orientation);
        self
    }

    pub fn group_norm(mut self, group_norm: GroupNorm) -> Scatter<X, Y, Tx, Ty> {
        self.group_norm = Some(group_norm);
        self
    }

    pub fn stack_group(mut self, stack_group: &str) -> Scatter<X, Y, Tx, Ty> {
        self.stack_group = Some(stack_group.to_owned());
        self
    }

    pub fn marker(mut self, marker: Marker) -> Scatter<X, Y, Tx, Ty> {
        self.marker = Some(marker);
        self
    }

    pub fn line(mut self, line: Line) -> Scatter<X, Y, Tx, Ty> {
        self.line = Some(line);
        self
    }

    pub fn text_font(mut self, text_font: Font) -> Scatter<X, Y, Tx, Ty> {
        self.text_font = Some(text_font);
        self
    }

    pub fn error_x(mut self, error_x: ErrorData) -> Scatter<X, Y, Tx, Ty> {
        self.error_x = Some(error_x);
        self
    }

    pub fn error_y(mut self, error_y: ErrorData) -> Scatter<X, Y, Tx, Ty> {
        self.error_y = Some(error_y);
        self
    }

    pub fn clip_on_axis(mut self, clip_on_axis: bool) -> Scatter<X, Y, Tx, Ty> {
        self.clip_on_axis = Some(clip_on_axis);
        self
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Scatter<X, Y, Tx, Ty> {
        self.connect_gaps = Some(connect_gaps);
        self
    }

    pub fn fill(mut self, fill: Fill) -> Scatter<X, Y, Tx, Ty> {
        self.fill = Some(fill);
        self
    }

    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Scatter<X, Y, Tx, Ty> {
        self.fill_color = Some(fill_color.to_color_string());
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Scatter<X, Y, Tx, Ty> {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_on(mut self, hover_on: &str) -> Scatter<X, Y, Tx, Ty> {
        self.hover_on = Some(hover_on.to_owned());
        self
    }

    pub fn stack_gaps(mut self, stack_gaps: &str) -> Scatter<X, Y, Tx, Ty> {
        self.stack_gaps = Some(stack_gaps.to_owned());
        self
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Scatter<X, Y, Tx, Ty> {
        self.x_calendar = Some(x_calendar);
        self
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Scatter<X, Y, Tx, Ty> {
        self.y_calendar = Some(y_calendar);
        self
    }
}

impl<X, Y, Tx, Ty> Trace for Scatter<X, Y, Tx, Ty>
where
    X: AsRef<[Tx]> + Serialize,
    Y: AsRef<[Ty]> + Serialize,
    Tx: Serialize,
    Ty: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
