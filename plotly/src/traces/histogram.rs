//! Histogram plot

use crate::common::{
    Calendar, Dim, ErrorData, HoverInfo, Label, Marker, Orientation, PlotType, Visible,
};
use crate::Trace;
use serde::Serialize;

use crate::private;
use crate::private::copy_iterable_to_vec;

#[cfg(feature = "plotly_ndarray")]
use crate::ndarray::ArrayTraces;
#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1, Ix2};

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Debug)]
pub enum HistDirection {
    #[serde(rename = "increasing")]
    Increasing,
    #[serde(rename = "decreasing")]
    Decreasing,
}

#[derive(Serialize, Clone, Debug)]
pub enum CurrentBin {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
    #[serde(rename = "half")]
    Half,
}

#[derive(Serialize, Clone, Debug, Default)]
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

#[derive(Serialize, Clone, Debug)]
pub struct Histogram<H>
where
    H: Serialize + Clone + Default + 'static,
{
    r#type: PlotType,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<String>,
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

impl<H> Default for Histogram<H>
where
    H: Serialize + Clone + Default + 'static,
{
    fn default() -> Self {
        Histogram {
            r#type: PlotType::Histogram,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            x: None,
            y: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            x_axis: None,
            y_axis: None,
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
        }
    }
}

impl<H> Histogram<H>
where
    H: Serialize + Clone + Default + 'static,
{
    pub fn new<I>(x: I) -> Box<Self>
    where
        I: IntoIterator<Item = H>,
    {
        let x = copy_iterable_to_vec(x);
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x),
            ..Default::default()
        })
    }

    pub fn new_xy<I>(x: I, y: I) -> Box<Self>
    where
        I: IntoIterator<Item = H>,
    {
        let x = copy_iterable_to_vec(x);
        let y = copy_iterable_to_vec(y);
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    pub fn new_vertical<I>(y: I) -> Box<Self>
    where
        I: IntoIterator<Item = H>,
    {
        let y = copy_iterable_to_vec(y);
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            y: Some(y),
            ..Default::default()
        })
    }

    /// Produces `Histogram` traces from a 2 dimensional tensor (`traces_matrix`) indexed by `x`. This
    /// function requires the `ndarray` feature.
    ///
    /// # Arguments
    /// * `x`             - One dimensional array (or view) that represents the `x` axis coordinates.
    /// * `traces_matrix` - Two dimensional array (or view) containing the `y` axis coordinates of
    /// the traces.
    /// * `array_traces`  - Determines whether the traces are arranged in the matrix over the
    /// columns (`ArrayTraces::OverColumns`) or over the rows (`ArrayTraces::OverRows`).
    ///
    /// # Examples
    ///
    /// ```
    /// use plotly::common::Mode;
    /// use plotly::{Plot, Histogram, ArrayTraces};
    /// use ndarray::{Array, Ix1, Ix2};
    /// use rand_distr::{Distribution, Normal};
    /// use plotly::Layout;
    /// use plotly::layout::BarMode;
    ///
    /// fn ndarray_to_traces() {
    ///     let n: usize = 1_250;
    ///     let mut rng = rand::thread_rng();
    ///     let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    ///     let mut ys: Array<f64, Ix2> = Array::zeros((n, 4));
    ///     let mut count = 0.;
    ///     for mut row in ys.columns_mut() {
    ///         let tmp: Vec<f64> = Normal::new(4. * count, 1.).unwrap().sample_iter(&mut rng).take(n).collect();
    ///         for i in 0..row.len() {
    ///             row[i] = tmp[i];
    ///         }
    ///         count += 1.;
    ///     }
    ///
    ///     let traces = Histogram::default()
    ///         .opacity(0.5)
    ///         .auto_bin_x(true)
    ///         .to_traces(ys, ArrayTraces::OverColumns);
    ///
    ///     let layout = Layout::new().bar_mode(BarMode::Overlay);
    ///
    ///     let mut plot = Plot::new();
    ///     plot.set_layout(layout);
    ///     plot.add_traces(traces);
    ///     plot.show();
    /// }
    /// fn main() -> std::io::Result<()> {
    ///     ndarray_to_traces();
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "plotly_ndarray")]
    pub fn to_traces(
        &self,
        traces_matrix: Array<H, Ix2>,
        array_traces: ArrayTraces,
    ) -> Vec<Box<dyn Trace>> {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let mut trace_vectors = private::trace_vectors_from(traces_matrix, array_traces);
        trace_vectors.reverse();
        while !trace_vectors.is_empty() {
            let mut sc = Box::new(self.clone());
            let data = trace_vectors.pop();
            if let Some(d) = data {
                sc.x = Some(d);
                traces.push(sc);
            }
        }

        traces
    }

    #[cfg(feature = "plotly_ndarray")]
    pub fn from_array(x: Array<H, Ix1>) -> Box<Self> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x.to_vec()),
            ..Default::default()
        })
    }

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Self> {
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

    pub fn text(mut self, text: &str) -> Box<Self> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
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

    pub fn hist_func(mut self, hist_func: HistFunc) -> Box<Self> {
        self.hist_func = Some(hist_func);
        Box::new(self)
    }

    pub fn hist_norm(mut self, hist_norm: HistNorm) -> Box<Self> {
        self.hist_norm = Some(hist_norm);
        Box::new(self)
    }

    pub fn alignment_group(mut self, alignment_group: &str) -> Box<Self> {
        self.alignment_group = Some(alignment_group.to_owned());
        Box::new(self)
    }

    pub fn offset_group(mut self, offset_group: &str) -> Box<Self> {
        self.offset_group = Some(offset_group.to_owned());
        Box::new(self)
    }

    pub fn n_bins_x(mut self, n_bins_x: usize) -> Box<Self> {
        self.n_bins_x = Some(n_bins_x);
        Box::new(self)
    }

    pub fn n_bins_y(mut self, n_bins_y: usize) -> Box<Self> {
        self.n_bins_y = Some(n_bins_y);
        Box::new(self)
    }

    pub fn auto_bin_x(mut self, auto_bin_x: bool) -> Box<Self> {
        self.auto_bin_x = Some(auto_bin_x);
        Box::new(self)
    }

    pub fn auto_bin_y(mut self, auto_bin_y: bool) -> Box<Self> {
        self.auto_bin_y = Some(auto_bin_y);
        Box::new(self)
    }

    pub fn bin_group(mut self, bin_group: &str) -> Box<Self> {
        self.bin_group = Some(bin_group.to_owned());
        Box::new(self)
    }

    pub fn x_bins(mut self, x_bins: Bins) -> Box<Self> {
        self.x_bins = Some(x_bins);
        Box::new(self)
    }

    pub fn y_bins(mut self, y_bins: Bins) -> Box<Self> {
        self.y_bins = Some(y_bins);
        Box::new(self)
    }

    pub fn marker(mut self, marker: Marker) -> Box<Self> {
        self.marker = Some(marker);
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

    pub fn cumulative(mut self, cumulative: Cumulative) -> Box<Self> {
        self.cumulative = Some(cumulative);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
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

impl<H> Trace for Histogram<H>
where
    H: Serialize + Clone + Default + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
