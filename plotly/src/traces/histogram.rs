//! Histogram plot

#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1, Ix2};
use plotly_derive::FieldSetter;
use serde::Serialize;

#[cfg(feature = "plotly_ndarray")]
use crate::ndarray::ArrayTraces;
use crate::{
    common::{Calendar, Dim, ErrorData, HoverInfo, Label, Marker, Orientation, PlotType, Visible},
    Trace,
};

#[derive(Serialize, Clone, Debug)]
pub struct Bins {
    start: f64,
    end: f64,
    size: f64,
}

impl Bins {
    pub fn new(start: f64, end: f64, size: f64) -> Self {
        Self { start, end, size }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Cumulative {
    enabled: Option<bool>,
    direction: Option<HistDirection>,
    #[serde(rename = "currentbin")]
    current_bin: Option<CurrentBin>,
}

impl Cumulative {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CurrentBin {
    Include,
    Exclude,
    Half,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HistDirection {
    Increasing,
    Decreasing,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HistFunc {
    Count,
    Sum,
    #[serde(rename = "avg")]
    Average,
    #[serde(rename = "min")]
    Minimum,
    #[serde(rename = "max")]
    Maximum,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HistNorm {
    #[serde(rename = "")]
    Default,
    Percent,
    Probability,
    Density,
    #[serde(rename = "probability density")]
    ProbabilityDensity,
}

/// Construct a histogram trace.
///
/// # Examples
///
/// ```
/// use plotly::Histogram;
///
/// let trace = Histogram::new_xy(
///     vec![0, 1, 2, 3],
///     vec![5, 8, 2, 3]
/// );
///
/// let expected = serde_json::json!({
///     "type": "histogram",
///     "x": [0, 1, 2, 3],
///     "y": [5, 8, 2, 3],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Histogram<H>
where
    H: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Histogram")]
    r#type: PlotType,
    #[serde(rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(rename = "autobinx")]
    auto_bin_x: Option<bool>,
    #[serde(rename = "autobiny")]
    auto_bin_y: Option<bool>,
    cumulative: Option<Cumulative>,
    #[serde(rename = "bingroup")]
    bin_group: Option<String>,
    error_x: Option<ErrorData>,
    error_y: Option<ErrorData>,
    #[serde(rename = "histfunc")]
    hist_func: Option<HistFunc>,
    #[serde(rename = "histnorm")]
    hist_norm: Option<HistNorm>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    marker: Option<Marker>,
    #[serde(rename = "nbinsx")]
    n_bins_x: Option<usize>,
    #[serde(rename = "nbinsy")]
    n_bins_y: Option<usize>,
    name: Option<String>,
    #[serde(rename = "offsetgroup")]
    offset_group: Option<String>,
    opacity: Option<f64>,
    orientation: Option<Orientation>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    text: Option<Dim<String>>,
    visible: Option<Visible>,
    x: Option<Vec<H>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "xbins")]
    x_bins: Option<Bins>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    y: Option<Vec<H>>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(rename = "ybins")]
    y_bins: Option<Bins>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<H> Histogram<H>
where
    H: Serialize + Clone + 'static,
{
    pub fn new(x: Vec<H>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            ..Default::default()
        })
    }

    pub fn new_xy(x: Vec<H>, y: Vec<H>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    pub fn new_vertical(y: Vec<H>) -> Box<Self> {
        Box::new(Self {
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
            x: Some(x.to_vec()),
            ..Default::default()
        })
    }
}

impl<H> Trace for Histogram<H>
where
    H: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ErrorType;

    #[test]
    fn test_serialize_bins() {
        let bins = Bins::new(0.0, 10.0, 5.0);
        let expected = json!({
            "start": 0.0,
            "end": 10.0,
            "size": 5.0
        });

        assert_eq!(to_value(bins).unwrap(), expected);
    }

    #[test]
    fn test_serialize_cumulative() {
        let cumulative = Cumulative::new()
            .enabled(true)
            .direction(HistDirection::Decreasing)
            .current_bin(CurrentBin::Exclude);

        let expected = json!({
            "enabled": true,
            "direction": "decreasing",
            "currentbin": "exclude"
        });

        assert_eq!(to_value(cumulative).unwrap(), expected);
    }

    #[test]
    fn test_serialize_current_bin() {
        assert_eq!(to_value(CurrentBin::Include).unwrap(), json!("include"));
        assert_eq!(to_value(CurrentBin::Exclude).unwrap(), json!("exclude"));
        assert_eq!(to_value(CurrentBin::Half).unwrap(), json!("half"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_hist_direction() {
        assert_eq!(to_value(HistDirection::Increasing).unwrap(), json!("increasing"));
        assert_eq!(to_value(HistDirection::Decreasing).unwrap(), json!("decreasing"));
    }

    #[test]
    fn test_serialize_hist_func() {
        assert_eq!(to_value(HistFunc::Count).unwrap(), json!("count"));
        assert_eq!(to_value(HistFunc::Sum).unwrap(), json!("sum"));
        assert_eq!(to_value(HistFunc::Average).unwrap(), json!("avg"));
        assert_eq!(to_value(HistFunc::Minimum).unwrap(), json!("min"));
        assert_eq!(to_value(HistFunc::Maximum).unwrap(), json!("max"));
    }
    #[test]
    #[rustfmt::skip]
    fn test_serialize_hist_norm() {
        assert_eq!(to_value(HistNorm::Default).unwrap(), json!(""));
        assert_eq!(to_value(HistNorm::Percent).unwrap(), json!("percent"));
        assert_eq!(to_value(HistNorm::Probability).unwrap(), json!("probability"));
        assert_eq!(to_value(HistNorm::Density).unwrap(), json!("density"));
        assert_eq!(to_value(HistNorm::ProbabilityDensity).unwrap(), json!("probability density"));
    }

    #[test]
    fn test_serialize_default_histogram() {
        let trace = Histogram::<i32>::default();
        let expected = json!({"type": "histogram"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_new_xy_histogram() {
        let trace = Histogram::new_xy(vec![0, 1, 2, 3], vec![4, 5, 6, 7]);
        let expected = json!({
            "type": "histogram",
            "x": [0, 1, 2, 3],
            "y": [4, 5, 6, 7],
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_new_vertical_histogram() {
        let trace = Histogram::new_vertical(vec![0, 1, 2, 3]);
        let expected = json!({
            "type": "histogram",
            "y": [0, 1, 2, 3]
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_histogram() {
        let trace = Histogram::new(vec![0, 1, 2])
            .alignment_group("alignmentgroup")
            .auto_bin_x(true)
            .auto_bin_y(false)
            .bin_group("bingroup")
            .cumulative(Cumulative::new())
            .error_x(ErrorData::new(ErrorType::SquareRoot))
            .error_y(ErrorData::new(ErrorType::Constant))
            .hist_func(HistFunc::Average)
            .hist_norm(HistNorm::Default)
            .hover_info(HoverInfo::Skip)
            .hover_label(Label::new())
            .hover_template("hovertemplate")
            .hover_template_array(vec!["hover_template_1", "hover_template_2"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text_1", "hover_text_2"])
            .legend_group("legendgroup")
            .marker(Marker::new())
            .n_bins_x(5)
            .n_bins_y(10)
            .name("histogram_trace")
            .offset_group("offsetgroup")
            .opacity(0.1)
            .show_legend(true)
            .text("text")
            .text_array(vec!["text_1", "text_2"])
            .visible(Visible::True)
            .x_axis("xaxis")
            .x_bins(Bins::new(1.0, 2.0, 1.0))
            .x_calendar(Calendar::Julian)
            .y_axis("yaxis")
            .y_bins(Bins::new(2.0, 3.0, 4.0))
            .y_calendar(Calendar::Mayan);

        let expected = json!({
            "type": "histogram",
            "alignmentgroup": "alignmentgroup",
            "autobinx": true,
            "autobiny": false,
            "bingroup": "bingroup",
            "cumulative": {},
            "error_x": {"type": "sqrt"},
            "error_y": {"type": "constant"},
            "histfunc": "avg",
            "histnorm": "",
            "hoverinfo": "skip",
            "hoverlabel": {},
            "hovertemplate": ["hover_template_1", "hover_template_2"],
            "hovertext": ["hover_text_1", "hover_text_2"],
            "legendgroup": "legendgroup",
            "marker": {},
            "nbinsx": 5,
            "nbinsy": 10,
            "name": "histogram_trace",
            "offsetgroup": "offsetgroup",
            "opacity": 0.1,
            "showlegend": true,
            "text": ["text_1", "text_2"],
            "visible": true,
            "x": [0, 1, 2],
            "xaxis": "xaxis",
            "xbins": {"start": 1.0, "end": 2.0, "size": 1.0},
            "xcalendar": "julian",
            "yaxis": "yaxis",
            "ybins": {"start": 2.0, "end": 3.0, "size": 4.0},
            "ycalendar": "mayan"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
