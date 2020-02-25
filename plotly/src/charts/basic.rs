use crate::charts::{
    owned_string_vector, Calendar, Color, ConstrainText, Dim, ErrorData, Fill, Font, GroupNorm,
    HoverInfo, Label, Line, Marker, Mode, Orientation, PlotType, Position, TextAnchor,
    TextPosition,
};
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Scatter<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    r#type: PlotType,
    x: Vec<X>,
    y: Vec<Y>,
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
    fill_color: Option<Color>,
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

impl<X, Y> Scatter<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Scatter<X, Y>> {
        Box::new(Scatter {
            x,
            y,
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
        })
    }

    pub fn name(mut self, name: &str) -> Box<Scatter<X, Y>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Scatter<X, Y>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Scatter<X, Y>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Scatter<X, Y>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Scatter<X, Y>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn mode(mut self, mode: Mode) -> Box<Scatter<X, Y>> {
        self.mode = Some(mode);
        Box::new(self)
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Scatter<X, Y>> {
        let ids = owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Scatter<X, Y>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Scatter<X, Y>> {
        let text = owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn text_position(mut self, text_position: Position) -> Box<Scatter<X, Y>> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    pub fn text_position_array(mut self, text_position: Vec<Position>) -> Box<Scatter<X, Y>> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }

    pub fn text_template(mut self, text_template: &str) -> Box<Scatter<X, Y>> {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        Box::new(self)
    }

    pub fn text_template_array<S: AsRef<str>>(
        mut self,
        text_template: Vec<S>,
    ) -> Box<Scatter<X, Y>> {
        let text_template = owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Scatter<X, Y>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Scatter<X, Y>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Scatter<X, Y>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Scatter<X, Y>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<Scatter<X, Y>> {
        let hover_template = owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn orientation(mut self, orientation: Orientation) -> Box<Scatter<X, Y>> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    pub fn group_norm(mut self, group_norm: GroupNorm) -> Box<Scatter<X, Y>> {
        self.group_norm = Some(group_norm);
        Box::new(self)
    }

    pub fn stack_group(mut self, stack_group: &str) -> Box<Scatter<X, Y>> {
        self.stack_group = Some(stack_group.to_owned());
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<Scatter<X, Y>> {
        self.marker = Some(marker);
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Scatter<X, Y>> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn text_font(mut self, text_font: Font) -> Box<Scatter<X, Y>> {
        self.text_font = Some(text_font);
        Box::new(self)
    }

    pub fn error_x(mut self, error_x: ErrorData) -> Box<Scatter<X, Y>> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    pub fn error_y(mut self, error_y: ErrorData) -> Box<Scatter<X, Y>> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    pub fn clip_on_axis(mut self, clip_on_axis: bool) -> Box<Scatter<X, Y>> {
        self.clip_on_axis = Some(clip_on_axis);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Scatter<X, Y>> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn fill(mut self, fill: Fill) -> Box<Scatter<X, Y>> {
        self.fill = Some(fill);
        Box::new(self)
    }

    pub fn fill_color(mut self, fill_color: Color) -> Box<Scatter<X, Y>> {
        self.fill_color = Some(fill_color);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Scatter<X, Y>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on(mut self, hover_on: &str) -> Box<Scatter<X, Y>> {
        self.hover_on = Some(hover_on.to_owned());
        Box::new(self)
    }

    pub fn stack_gaps(mut self, stack_gaps: &str) -> Box<Scatter<X, Y>> {
        self.stack_gaps = Some(stack_gaps.to_owned());
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Scatter<X, Y>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Scatter<X, Y>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Scatter<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct Bar<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    x: Vec<X>,
    y: Vec<Y>,
    r#type: PlotType,
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
    ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Dim<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetgroup")]
    offset_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textangle")]
    text_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textfont")]
    text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_y: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "constraintext")]
    constrain_text: Option<ConstrainText>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "insidetextanchor")]
    inside_text_anchor: Option<TextAnchor>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Bar<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Bar<X, Y>> {
        Box::new(Bar {
            x,
            y,
            r#type: PlotType::Bar,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            ids: None,
            width: None,
            offset: None,
            text: None,
            text_position: None,
            text_template: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            alignment_group: None,
            offset_group: None,
            marker: None,
            text_angle: None,
            text_font: None,
            error_x: None,
            error_y: None,
            clip_on_axis: None,
            constrain_text: None,
            hover_label: None,
            inside_text_anchor: None,
            inside_text_font: None,
            outside_text_font: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<Bar<X, Y>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Bar<X, Y>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Bar<X, Y>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Bar<X, Y>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Bar<X, Y>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<Bar<X, Y>> {
        let ids = owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    pub fn width(mut self, width: usize) -> Box<Bar<X, Y>> {
        self.width = Some(width);
        Box::new(self)
    }

    pub fn offset(mut self, offset: usize) -> Box<Bar<X, Y>> {
        self.offset = Some(Dim::Scalar(offset));
        Box::new(self)
    }

    pub fn offset_array(mut self, offset: Vec<usize>) -> Box<Bar<X, Y>> {
        self.offset = Some(Dim::Vector(offset));
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Bar<X, Y>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Bar<X, Y>> {
        let text = owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn text_position(mut self, text_position: TextPosition) -> Box<Bar<X, Y>> {
        self.text_position = Some(Dim::Scalar(text_position));
        Box::new(self)
    }

    pub fn text_position_array(mut self, text_position: Vec<TextPosition>) -> Box<Bar<X, Y>> {
        self.text_position = Some(Dim::Vector(text_position));
        Box::new(self)
    }

    pub fn text_template(mut self, text_template: &str) -> Box<Bar<X, Y>> {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        Box::new(self)
    }

    pub fn text_template_array<S: AsRef<str>>(mut self, text_template: Vec<S>) -> Box<Bar<X, Y>> {
        let text_template = owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Bar<X, Y>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Bar<X, Y>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Bar<X, Y>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Bar<X, Y>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Bar<X, Y>> {
        let hover_template = owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn orientation(mut self, orientation: Orientation) -> Box<Bar<X, Y>> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    pub fn alignment_group(mut self, alignment_group: &str) -> Box<Bar<X, Y>> {
        self.alignment_group = Some(alignment_group.to_owned());
        Box::new(self)
    }

    pub fn offset_group(mut self, offset_group: &str) -> Box<Bar<X, Y>> {
        self.offset_group = Some(offset_group.to_owned());
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<Bar<X, Y>> {
        self.marker = Some(marker);
        Box::new(self)
    }

    pub fn text_angle(mut self, text_angle: f64) -> Box<Bar<X, Y>> {
        self.text_angle = Some(text_angle);
        Box::new(self)
    }

    pub fn text_font(mut self, text_font: Font) -> Box<Bar<X, Y>> {
        self.text_font = Some(text_font);
        Box::new(self)
    }

    pub fn error_x(mut self, error_x: ErrorData) -> Box<Bar<X, Y>> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    pub fn error_y(mut self, error_y: ErrorData) -> Box<Bar<X, Y>> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    pub fn clip_on_axis(mut self, clip_on_axis: bool) -> Box<Bar<X, Y>> {
        self.clip_on_axis = Some(clip_on_axis);
        Box::new(self)
    }

    pub fn constrain_text(mut self, constrain_text: ConstrainText) -> Box<Bar<X, Y>> {
        self.constrain_text = Some(constrain_text);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Bar<X, Y>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn inside_text_anchor(mut self, inside_text_anchor: TextAnchor) -> Box<Bar<X, Y>> {
        self.inside_text_anchor = Some(inside_text_anchor);
        Box::new(self)
    }

    pub fn inside_text_font(mut self, inside_text_font: Font) -> Box<Bar<X, Y>> {
        self.inside_text_font = Some(inside_text_font);
        Box::new(self)
    }

    pub fn outside_text_font(mut self, outside_text_font: Font) -> Box<Bar<X, Y>> {
        self.outside_text_font = Some(outside_text_font);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Bar<X, Y>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Bar<X, Y>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for Bar<X, Y>
where
    X: Serialize,
    Y: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
