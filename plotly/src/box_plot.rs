//! Box plot

use crate::common::color::{Color, ColorWrapper};
use crate::common::{
    Calendar, Dim, HoverInfo, Label, Line, Marker, Orientation, PlotType, Visible,
};
use crate::private;
use crate::Trace;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum BoxMean {
    True,
    False,
    StandardDeviation,
}

impl Serialize for BoxMean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::True => serializer.serialize_bool(true),
            Self::False => serializer.serialize_bool(false),
            Self::StandardDeviation => serializer.serialize_str("sd"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoxPoints {
    All,
    Outliers,
    SuspectedOutliers,
    False,
}

impl Serialize for BoxPoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::All => serializer.serialize_str("all"),
            Self::Outliers => serializer.serialize_str("outliers"),
            Self::SuspectedOutliers => serializer.serialize_str("suspectedoutliers"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum QuartileMethod {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "exclusive")]
    Exclusive,
    #[serde(rename = "inclusive")]
    Inclusive,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct BoxPlot<Y, X>
where
    Y: Serialize + Default + Clone,
    X: Serialize + Default + Clone,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<X>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<Y>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<Visible>,
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
    text: Option<Dim<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetgroup")]
    offset_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "boxmean")]
    box_mean: Option<BoxMean>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "boxpoints")]
    box_points: Option<BoxPoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "notchwidth")]
    notch_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "whiskerwidth")]
    whisker_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q1: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    median: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q3: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lowerfence")]
    lower_fence: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "notchspan")]
    notch_span: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mean: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sd")]
    standard_deviation: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quartilemethod")]
    quartile_method: Option<QuartileMethod>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillcolor")]
    fill_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoveron")]
    hover_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pointpos")]
    point_pos: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jitter: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<Y> BoxPlot<Y, f64>
where
    Y: Serialize + Default + Clone,
{
    pub fn new(y: Vec<Y>) -> Box<BoxPlot<Y, f64>> {
        Box::new(BoxPlot {
            r#type: PlotType::Box,
            x: None,
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<Y, X> BoxPlot<Y, X>
where
    Y: Serialize + Default + Clone,
    X: Serialize + Default + Clone,
{
    pub fn new_xy(x: Vec<X>, y: Vec<Y>) -> Box<BoxPlot<Y, X>> {
        Box::new(BoxPlot {
            r#type: PlotType::Box,
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    pub fn horizontal(x: Vec<X>) -> Box<BoxPlot<f64, X>> {
        Box::new(BoxPlot {
            r#type: PlotType::Box,
            x: Some(x),
            y: None,
            ..Default::default()
        })
    }

    pub fn name(mut self, name: &str) -> Box<BoxPlot<Y, X>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<BoxPlot<Y, X>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<BoxPlot<Y, X>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<BoxPlot<Y, X>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<BoxPlot<Y, X>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> Box<BoxPlot<Y, X>> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        Box::new(self)
    }

    pub fn width(mut self, width: usize) -> Box<BoxPlot<Y, X>> {
        self.width = Some(width);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<BoxPlot<Y, X>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<BoxPlot<Y, X>> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<BoxPlot<Y, X>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<BoxPlot<Y, X>> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<BoxPlot<Y, X>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<BoxPlot<Y, X>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn x_axis(mut self, axis: &str) -> Box<BoxPlot<Y, X>> {
        self.x_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<BoxPlot<Y, X>> {
        self.y_axis = Some(axis.to_owned());
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<BoxPlot<Y, X>> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn orientation(mut self, orientation: Orientation) -> Box<BoxPlot<Y, X>> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    pub fn alignment_group(mut self, alignment_group: &str) -> Box<BoxPlot<Y, X>> {
        self.alignment_group = Some(alignment_group.to_owned());
        Box::new(self)
    }

    pub fn offset_group(mut self, offset_group: &str) -> Box<BoxPlot<Y, X>> {
        self.offset_group = Some(offset_group.to_owned());
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<BoxPlot<Y, X>> {
        self.marker = Some(marker);
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<BoxPlot<Y, X>> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn box_mean(mut self, box_mean: BoxMean) -> Box<BoxPlot<Y, X>> {
        self.box_mean = Some(box_mean);
        Box::new(self)
    }

    pub fn box_points(mut self, box_points: BoxPoints) -> Box<BoxPlot<Y, X>> {
        self.box_points = Some(box_points);
        Box::new(self)
    }

    pub fn notched(mut self, notched: bool) -> Box<BoxPlot<Y, X>> {
        self.notched = Some(notched);
        Box::new(self)
    }

    pub fn notch_width(mut self, notch_width: f64) -> Box<BoxPlot<Y, X>> {
        self.notch_width = Some(notch_width);
        Box::new(self)
    }

    pub fn whisker_width(mut self, whisker_width: f64) -> Box<BoxPlot<Y, X>> {
        self.whisker_width = Some(whisker_width);
        Box::new(self)
    }

    pub fn q1(mut self, q1: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.q1 = Some(q1);
        Box::new(self)
    }

    pub fn median(mut self, median: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.median = Some(median);
        Box::new(self)
    }

    pub fn q3(mut self, q3: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.q3 = Some(q3);
        Box::new(self)
    }

    pub fn lower_fence(mut self, lower_fence: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.lower_fence = Some(lower_fence);
        Box::new(self)
    }

    pub fn notch_span(mut self, notch_span: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.notch_span = Some(notch_span);
        Box::new(self)
    }

    pub fn mean(mut self, mean: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.mean = Some(mean);
        Box::new(self)
    }

    pub fn standard_deviation(mut self, standard_deviation: Vec<f64>) -> Box<BoxPlot<Y, X>> {
        self.standard_deviation = Some(standard_deviation);
        Box::new(self)
    }

    pub fn quartile_method(mut self, quartile_method: QuartileMethod) -> Box<BoxPlot<Y, X>> {
        self.quartile_method = Some(quartile_method);
        Box::new(self)
    }

    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Box<BoxPlot<Y, X>> {
        self.fill_color = Some(fill_color.to_color());
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<BoxPlot<Y, X>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on(mut self, hover_on: &str) -> Box<BoxPlot<Y, X>> {
        self.hover_on = Some(hover_on.to_owned());
        Box::new(self)
    }

    pub fn point_pos(mut self, point_pos: f64) -> Box<BoxPlot<Y, X>> {
        self.point_pos = Some(point_pos);
        Box::new(self)
    }

    pub fn jitter(mut self, jitter: f64) -> Box<BoxPlot<Y, X>> {
        self.jitter = Some(jitter);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<BoxPlot<Y, X>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<BoxPlot<Y, X>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y> Trace for BoxPlot<X, Y>
where
    X: Serialize + Default + Clone,
    Y: Serialize + Default + Clone,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_box_mean() {
        assert_eq!(to_value(BoxMean::True).unwrap(), json!(true));
        assert_eq!(to_value(BoxMean::False).unwrap(), json!(false));
        assert_eq!(to_value(BoxMean::StandardDeviation).unwrap(), json!("sd"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_box_points() {
        assert_eq!(to_value(BoxPoints::All).unwrap(), json!("all"));
        assert_eq!(to_value(BoxPoints::Outliers).unwrap(), json!("outliers"));
        assert_eq!(to_value(BoxPoints::SuspectedOutliers).unwrap(), json!("suspectedoutliers"));
        assert_eq!(to_value(BoxPoints::False).unwrap(), json!(false));
    }
}
