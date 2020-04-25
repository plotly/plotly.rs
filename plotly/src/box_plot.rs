//! Box plot

use crate::common::color::Color;
use crate::common::{Calendar, Dim, HoverInfo, Label, Line, Marker, Orientation, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum BoxMean {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "sd")]
    StandardDeviation,
}

#[derive(Serialize, Debug)]
pub enum BoxPoints {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "outliers")]
    Outliers,
    #[serde(rename = "suspectedoutliers")]
    SuspectedOutliers,
    #[serde(rename = "false")]
    False,
}

#[derive(Serialize, Debug)]
pub enum QuartileMethod {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "exclusive")]
    Exclusive,
    #[serde(rename = "inclusive")]
    Inclusive,
}

#[derive(Serialize, Debug)]
pub struct BoxPlot<Y, X>
where
    Y: Serialize,
    X: Serialize,
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
    box_mean: Option<private::TruthyEnum<BoxMean>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "boxpoints")]
    box_points: Option<private::TruthyEnum<BoxPoints>>,
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
    fill_color: Option<String>,
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
    Y: Serialize,
{
    pub fn new(y: Vec<Y>) -> BoxPlot<Y, f64> {
        BoxPlot {
            r#type: PlotType::Box,
            x: None,
            y: Some(y),
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            ids: None,
            width: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            alignment_group: None,
            offset_group: None,
            marker: None,
            line: None,
            box_mean: None,
            box_points: None,
            notched: None,
            notch_width: None,
            whisker_width: None,
            q1: None,
            median: None,
            q3: None,
            lower_fence: None,
            notch_span: None,
            mean: None,
            standard_deviation: None,
            quartile_method: None,
            fill_color: None,
            hover_label: None,
            hover_on: None,
            point_pos: None,
            jitter: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<Y, X> BoxPlot<Y, X>
where
    Y: Serialize,
    X: Serialize,
{
    pub fn new_xy(x: Vec<X>, y: Vec<Y>) -> BoxPlot<Y, X> {
        BoxPlot {
            r#type: PlotType::Box,
            x: Some(x),
            y: Some(y),
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            ids: None,
            width: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            alignment_group: None,
            offset_group: None,
            marker: None,
            line: None,
            box_mean: None,
            box_points: None,
            notched: None,
            notch_width: None,
            whisker_width: None,
            q1: None,
            median: None,
            q3: None,
            lower_fence: None,
            notch_span: None,
            mean: None,
            standard_deviation: None,
            quartile_method: None,
            fill_color: None,
            hover_label: None,
            hover_on: None,
            point_pos: None,
            jitter: None,
            x_calendar: None,
            y_calendar: None,
        }
    }

    pub fn horizontal(x: Vec<X>) -> BoxPlot<f64, X> {
        BoxPlot {
            r#type: PlotType::Box,
            x: Some(x),
            y: None,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            ids: None,
            width: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            alignment_group: None,
            offset_group: None,
            marker: None,
            line: None,
            box_mean: None,
            box_points: None,
            notched: None,
            notch_width: None,
            whisker_width: None,
            q1: None,
            median: None,
            q3: None,
            lower_fence: None,
            notch_span: None,
            mean: None,
            standard_deviation: None,
            quartile_method: None,
            fill_color: None,
            hover_label: None,
            hover_on: None,
            point_pos: None,
            jitter: None,
            x_calendar: None,
            y_calendar: None,
        }
    }

    pub fn name(mut self, name: &str) -> BoxPlot<Y, X> {
        self.name = Some(name.to_owned());
        self
    }

    pub fn visible(mut self, visible: bool) -> BoxPlot<Y, X> {
        self.visible = Some(visible);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> BoxPlot<Y, X> {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend_group(mut self, legend_group: &str) -> BoxPlot<Y, X> {
        self.legend_group = Some(legend_group.to_owned());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> BoxPlot<Y, X> {
        self.opacity = Some(opacity);
        self
    }

    pub fn ids<S: AsRef<str>>(mut self, ids: Vec<S>) -> BoxPlot<Y, X> {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        self
    }

    pub fn width(mut self, width: usize) -> BoxPlot<Y, X> {
        self.width = Some(width);
        self
    }

    pub fn text(mut self, text: &str) -> BoxPlot<Y, X> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        self
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> BoxPlot<Y, X> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        self
    }

    pub fn hover_text(mut self, hover_text: &str) -> BoxPlot<Y, X> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        self
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> BoxPlot<Y, X> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> BoxPlot<Y, X> {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn hover_template(mut self, hover_template: &str) -> BoxPlot<Y, X> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        self
    }

    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> BoxPlot<Y, X> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> BoxPlot<Y, X> {
        self.orientation = Some(orientation);
        self
    }

    pub fn alignment_group(mut self, alignment_group: &str) -> BoxPlot<Y, X> {
        self.alignment_group = Some(alignment_group.to_owned());
        self
    }

    pub fn offset_group(mut self, offset_group: &str) -> BoxPlot<Y, X> {
        self.offset_group = Some(offset_group.to_owned());
        self
    }

    pub fn marker(mut self, marker: Marker) -> BoxPlot<Y, X> {
        self.marker = Some(marker);
        self
    }

    pub fn line(mut self, line: Line) -> BoxPlot<Y, X> {
        self.line = Some(line);
        self
    }

    pub fn box_mean(mut self, box_mean: BoxMean) -> BoxPlot<Y, X> {
        self.box_mean = Some(private::TruthyEnum { e: box_mean });
        self
    }

    pub fn box_points(mut self, box_points: BoxPoints) -> BoxPlot<Y, X> {
        self.box_points = Some(private::TruthyEnum { e: box_points });
        self
    }

    pub fn notched(mut self, notched: bool) -> BoxPlot<Y, X> {
        self.notched = Some(notched);
        self
    }

    pub fn notch_width(mut self, notch_width: f64) -> BoxPlot<Y, X> {
        self.notch_width = Some(notch_width);
        self
    }

    pub fn whisker_width(mut self, whisker_width: f64) -> BoxPlot<Y, X> {
        self.whisker_width = Some(whisker_width);
        self
    }

    pub fn q1(mut self, q1: Vec<f64>) -> BoxPlot<Y, X> {
        self.q1 = Some(q1);
        self
    }

    pub fn median(mut self, median: Vec<f64>) -> BoxPlot<Y, X> {
        self.median = Some(median);
        self
    }

    pub fn q3(mut self, q3: Vec<f64>) -> BoxPlot<Y, X> {
        self.q3 = Some(q3);
        self
    }

    pub fn lower_fence(mut self, lower_fence: Vec<f64>) -> BoxPlot<Y, X> {
        self.lower_fence = Some(lower_fence);
        self
    }

    pub fn notch_span(mut self, notch_span: Vec<f64>) -> BoxPlot<Y, X> {
        self.notch_span = Some(notch_span);
        self
    }

    pub fn mean(mut self, mean: Vec<f64>) -> BoxPlot<Y, X> {
        self.mean = Some(mean);
        self
    }

    pub fn standard_deviation(mut self, standard_deviation: Vec<f64>) -> BoxPlot<Y, X> {
        self.standard_deviation = Some(standard_deviation);
        self
    }

    pub fn quartile_method(mut self, quartile_method: QuartileMethod) -> BoxPlot<Y, X> {
        self.quartile_method = Some(quartile_method);
        self
    }

    pub fn fill_color<C: Color>(mut self, fill_color: C) -> BoxPlot<Y, X> {
        self.fill_color = Some(fill_color.to_color_string());
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> BoxPlot<Y, X> {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_on(mut self, hover_on: &str) -> BoxPlot<Y, X> {
        self.hover_on = Some(hover_on.to_owned());
        self
    }

    pub fn point_pos(mut self, point_pos: f64) -> BoxPlot<Y, X> {
        self.point_pos = Some(point_pos);
        self
    }

    pub fn jitter(mut self, jitter: f64) -> BoxPlot<Y, X> {
        self.jitter = Some(jitter);
        self
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> BoxPlot<Y, X> {
        self.x_calendar = Some(x_calendar);
        self
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> BoxPlot<Y, X> {
        self.y_calendar = Some(y_calendar);
        self
    }
}

impl<X, Y> Trace for BoxPlot<X, Y>
where
    X: Serialize,
    Y: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
