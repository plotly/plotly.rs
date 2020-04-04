//! Histogram plot

use crate::common::{Calendar, Dim, ErrorData, HoverInfo, Label, Marker, Orientation, PlotType};
use crate::Trace;
use serde::Serialize;

use crate::private;

#[derive(Serialize, Debug)]
pub struct Bins {
    start: f64,
    end: f64,
    size: f64,
}

impl Bins {
    pub fn new(start: f64, end: f64, size: f64) -> Bins {
        Bins { start, end, size }
    }
}

#[derive(Serialize, Debug)]
pub enum HistFunc {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "avg")]
    Average,
    #[serde(rename = "min")]
    Minimum,
    #[serde(rename = "max")]
    Maximum,
}

#[derive(Serialize, Debug)]
pub enum HistNorm {
    #[serde(rename = "")]
    Default,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "probability")]
    Probability,
    #[serde(rename = "density")]
    Density,
    #[serde(rename = "probability density")]
    ProbabilityDensity,
}

#[derive(Serialize, Debug)]
pub enum HistDirection {
    #[serde(rename = "increasing")]
    Increasing,
    #[serde(rename = "decreasing")]
    Decreasing,
}

#[derive(Serialize, Debug)]
pub enum CurrentBin {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
    #[serde(rename = "half")]
    Half,
}

#[derive(Serialize, Debug, Default)]
pub struct Cumulative {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<HistDirection>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentbin")]
    current_bin: Option<CurrentBin>,
}

impl Cumulative {
    pub fn new() -> Cumulative {
        Cumulative {
            enabled: None,
            direction: None,
            current_bin: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Cumulative {
        self.enabled = Some(enabled);
        self
    }

    pub fn direction(mut self, direction: HistDirection) -> Cumulative {
        self.direction = Some(direction);
        self
    }

    pub fn current_bin(mut self, current_bin: CurrentBin) -> Cumulative {
        self.current_bin = Some(current_bin);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct Histogram<H>
where
    H: Serialize,
{
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
    x: Option<Vec<H>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<H>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "histfunc")]
    hist_func: Option<HistFunc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "histnorm")]
    hist_norm: Option<HistNorm>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetgroup")]
    offset_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nbinsx")]
    n_bins_x: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nbinsy")]
    n_bins_y: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autobinx")]
    auto_bin_x: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autobiny")]
    auto_bin_y: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bingroup")]
    bin_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xbins")]
    x_bins: Option<Bins>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ybins")]
    y_bins: Option<Bins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_y: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cumulative: Option<Cumulative>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<H> Histogram<H>
where
    H: Serialize,
{
    pub fn new(x: Vec<H>) -> Box<Histogram<H>> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x),
            y: None,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            hist_func: None,
            hist_norm: None,
            alignment_group: None,
            offset_group: None,
            n_bins_x: None,
            n_bins_y: None,
            auto_bin_x: None,
            auto_bin_y: None,
            bin_group: None,
            x_bins: None,
            y_bins: None,
            marker: None,
            error_x: None,
            error_y: None,
            cumulative: None,
            hover_label: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn new_xy(x: Vec<H>, y: Vec<H>) -> Box<Histogram<H>> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x),
            y: Some(y),
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            hist_func: None,
            hist_norm: None,
            alignment_group: None,
            offset_group: None,
            n_bins_x: None,
            n_bins_y: None,
            auto_bin_x: None,
            auto_bin_y: None,
            bin_group: None,
            x_bins: None,
            y_bins: None,
            marker: None,
            error_x: None,
            error_y: None,
            cumulative: None,
            hover_label: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn new_horizontal(y: Vec<H>) -> Box<Histogram<H>> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: None,
            y: Some(y),
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            orientation: None,
            hist_func: None,
            hist_norm: None,
            alignment_group: None,
            offset_group: None,
            n_bins_x: None,
            n_bins_y: None,
            auto_bin_x: None,
            auto_bin_y: None,
            bin_group: None,
            x_bins: None,
            y_bins: None,
            marker: None,
            error_x: None,
            error_y: None,
            cumulative: None,
            hover_label: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<Histogram<H>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Histogram<H>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Histogram<H>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Histogram<H>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Histogram<H>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Histogram<H>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Histogram<H>> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Histogram<H>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Histogram<H>> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Histogram<H>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Histogram<H>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<Histogram<H>> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn orientation(mut self, orientation: Orientation) -> Box<Histogram<H>> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    pub fn hist_func(mut self, hist_func: HistFunc) -> Box<Histogram<H>> {
        self.hist_func = Some(hist_func);
        Box::new(self)
    }

    pub fn hist_norm(mut self, hist_norm: HistNorm) -> Box<Histogram<H>> {
        self.hist_norm = Some(hist_norm);
        Box::new(self)
    }

    pub fn alignment_group(mut self, alignment_group: &str) -> Box<Histogram<H>> {
        self.alignment_group = Some(alignment_group.to_owned());
        Box::new(self)
    }

    pub fn offset_group(mut self, offset_group: &str) -> Box<Histogram<H>> {
        self.offset_group = Some(offset_group.to_owned());
        Box::new(self)
    }

    pub fn n_bins_x(mut self, n_bins_x: usize) -> Box<Histogram<H>> {
        self.n_bins_x = Some(n_bins_x);
        Box::new(self)
    }

    pub fn n_bins_y(mut self, n_bins_y: usize) -> Box<Histogram<H>> {
        self.n_bins_y = Some(n_bins_y);
        Box::new(self)
    }

    pub fn auto_bin_x(mut self, auto_bin_x: bool) -> Box<Histogram<H>> {
        self.auto_bin_x = Some(auto_bin_x);
        Box::new(self)
    }

    pub fn auto_bin_y(mut self, auto_bin_y: bool) -> Box<Histogram<H>> {
        self.auto_bin_y = Some(auto_bin_y);
        Box::new(self)
    }

    pub fn bin_group(mut self, bin_group: &str) -> Box<Histogram<H>> {
        self.bin_group = Some(bin_group.to_owned());
        Box::new(self)
    }

    pub fn x_bins(mut self, x_bins: Bins) -> Box<Histogram<H>> {
        self.x_bins = Some(x_bins);
        Box::new(self)
    }

    pub fn y_bins(mut self, y_bins: Bins) -> Box<Histogram<H>> {
        self.y_bins = Some(y_bins);
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<Histogram<H>> {
        self.marker = Some(marker);
        Box::new(self)
    }

    pub fn error_x(mut self, error_x: ErrorData) -> Box<Histogram<H>> {
        self.error_x = Some(error_x);
        Box::new(self)
    }

    pub fn error_y(mut self, error_y: ErrorData) -> Box<Histogram<H>> {
        self.error_y = Some(error_y);
        Box::new(self)
    }

    pub fn cumulative(mut self, cumulative: Cumulative) -> Box<Histogram<H>> {
        self.cumulative = Some(cumulative);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Histogram<H>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Histogram<H>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Histogram<H>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<H> Trace for Histogram<H>
where
    H: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
