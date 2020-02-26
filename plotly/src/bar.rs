//! Bar plot

use crate::common::{
    Calendar, ConstrainText, Dim, ErrorData, Font, HoverInfo, Label, Marker, Orientation, PlotType,
    TextAnchor, TextPosition,
};
use crate::Trace;
use serde::Serialize;

use crate::private;

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
        let ids = private::owned_string_vector(ids);
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
        let text = private::owned_string_vector(text);
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
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Bar<X, Y>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Bar<X, Y>> {
        let hover_text = private::owned_string_vector(hover_text);
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
        let hover_template = private::owned_string_vector(hover_template);
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
