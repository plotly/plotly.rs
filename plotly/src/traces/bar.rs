//! Bar plot

use serde::Serialize;

use crate::common::{
    Calendar, ConstrainText, Dim, ErrorData, Font, HoverInfo, Label, Marker, Orientation, PlotType,
    TextAnchor, TextPosition, Visible,
};
use crate::private;
use crate::Trace;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct Bar<X, Y>
where
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
{
    x: Vec<X>,
    y: Vec<Y>,
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    width: Option<usize>,
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
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Bar<X, Y>> {
        Box::new(Bar {
            x,
            y,
            r#type: PlotType::Bar,
            ..Default::default()
        })
    }

    pub fn name(mut self, name: &str) -> Box<Bar<X, Y>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Bar<X, Y>> {
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

    pub fn x_axis(mut self, axis: &str) -> Box<Bar<X, Y>> {
        self.x_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<Bar<X, Y>> {
        self.y_axis = Some(axis.to_owned());
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
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ErrorType;

    #[test]
    fn test_serialize_bar() {
        let bar = Bar::new(vec![1, 2], vec![3, 4])
            .name("Bar")
            .visible(Visible::LegendOnly)
            .show_legend(false)
            .legend_group("legend-group")
            .opacity(0.5)
            .ids(vec!["1"])
            .width(999)
            .offset(5)
            .offset_array(vec![5, 5])
            .text("text")
            .text_array(vec!["text"])
            .text_position(TextPosition::None)
            .text_position_array(vec![TextPosition::None])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .x_axis("xaxis")
            .y_axis("yaxis")
            .orientation(Orientation::Vertical)
            .alignment_group("alignment_group")
            .offset_group("offset_group")
            .marker(Marker::new())
            .text_angle(0.05)
            .text_font(Font::new())
            .error_x(ErrorData::new(ErrorType::Constant))
            .error_y(ErrorData::new(ErrorType::Percent))
            .clip_on_axis(true)
            .constrain_text(ConstrainText::Both)
            .hover_label(Label::new())
            .inside_text_anchor(TextAnchor::End)
            .inside_text_font(Font::new())
            .outside_text_font(Font::new())
            .x_calendar(Calendar::Nanakshahi)
            .y_calendar(Calendar::Ummalqura);

        let expected = json!({
            "type": "bar",
            "x": [1, 2],
            "y": [3, 4],
            "name": "Bar",
            "visible": "legendonly",
            "showlegend": false,
            "legendgroup": "legend-group",
            "opacity": 0.5,
            "ids": ["1"],
            "width": 999,
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
