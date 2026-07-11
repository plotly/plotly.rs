//! 2D histogram trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{
        Calendar, ColorBar, ColorScale, Dim, Font, HoverInfo, Label, LegendGroupTitle, PlotType,
        Visible, XAxisId, YAxisId,
    },
    private::{NumOrString, NumOrStringCollection},
    traces::heat_map::Smoothing,
    traces::histogram::{Bins, HistFunc, HistNorm},
    Trace,
};

/// Construct a 2D histogram trace.
///
/// A `Histogram2d` bins raw `x`/`y` samples into a 2D grid and renders the
/// binned counts as a heatmap. Provide raw `x`/`y` data and let Plotly.js bin
/// it via `hist_func`/`n_bins_x`/`n_bins_y`/`x_bins`/`y_bins`. Optionally
/// supply per-sample aggregation values via `z` (a 1D array aligned with
/// `x`/`y`) and a `hist_func` such as [`HistFunc::Sum`] or
/// [`HistFunc::Average`] to aggregate those values within each bin instead of
/// counting samples.
///
/// Note: unlike [`HeatMap`](crate::HeatMap), `z` here is per-sample aggregation
/// data aligned with `x`/`y`, not a pre-binned 2D matrix. For a pre-computed
/// matrix, use [`HeatMap`](crate::HeatMap).
///
/// # Examples
///
/// ```
/// use plotly::Histogram2d;
///
/// let trace = Histogram2d::new(vec![1.0, 2.0, 2.0], vec![1.0, 1.0, 3.0]);
///
/// let expected = serde_json::json!({
///     "type": "histogram2d",
///     "x": [1.0, 2.0, 2.0],
///     "y": [1.0, 1.0, 3.0],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Histogram2d<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Histogram2d")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    opacity: Option<f64>,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    z: Option<Vec<Z>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    /// Specifies the binning function used for this histogram trace.
    #[serde(rename = "histfunc")]
    hist_func: Option<HistFunc>,
    /// Specifies the type of normalization used for this histogram trace.
    #[serde(rename = "histnorm")]
    hist_norm: Option<HistNorm>,
    /// Determines whether or not the x-axis bin attributes are picked by an
    /// algorithm.
    #[serde(rename = "autobinx")]
    auto_bin_x: Option<bool>,
    /// Determines whether or not the y-axis bin attributes are picked by an
    /// algorithm.
    #[serde(rename = "autobiny")]
    auto_bin_y: Option<bool>,
    /// Specifies the maximum number of desired bins along the x axis.
    #[serde(rename = "nbinsx")]
    n_bins_x: Option<usize>,
    /// Specifies the maximum number of desired bins along the y axis.
    #[serde(rename = "nbinsy")]
    n_bins_y: Option<usize>,
    /// Sets the binning across the x axis.
    #[serde(rename = "xbins")]
    x_bins: Option<Bins>,
    /// Sets the binning across the y axis.
    #[serde(rename = "ybins")]
    y_bins: Option<Bins>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    zauto: Option<bool>,
    zmin: Option<f64>,
    zmax: Option<f64>,
    zmid: Option<f64>,
    #[serde(rename = "zhoverformat")]
    zhover_format: Option<String>,
    zsmooth: Option<Smoothing>,
    #[serde(rename = "xgap")]
    x_gap: Option<NumOrString>,
    #[serde(rename = "ygap")]
    y_gap: Option<NumOrString>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    #[serde(rename = "xhoverformat")]
    x_hover_format: Option<String>,
    #[serde(rename = "yhoverformat")]
    y_hover_format: Option<String>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "texttemplatefallback")]
    text_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    /// Assigns extra meta information associated with this trace that can be
    /// used in various text attributes.
    meta: Option<NumOrString>,
    /// Assigns extra data to each datum that can be used in hover, click and
    /// selection events.
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    uid: Option<String>,
    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"`
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Sets the width (in px or fraction) of the legend for this trace.
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    /// Controls persistence of user-driven changes to the trace. Defaults to
    /// `layout.uirevision`.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<X, Y> Histogram2d<X, Y, f64>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    /// Build a new 2D histogram from raw `x` and `y` samples. Plotly.js bins
    /// the samples into a 2D grid.
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> Histogram2d<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    /// Build a new 2D histogram from raw `x`/`y` samples plus per-sample
    /// aggregation values `z`. All three vectors are aligned (one entry per
    /// sample); combine with a `hist_func` such as [`HistFunc::Sum`] to
    /// aggregate the `z` values within each `x`/`y` bin.
    pub fn new_xyz(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> Trace for Histogram2d<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    fn serialize_default_histogram2d() {
        let trace = Histogram2d::<f64, f64, f64>::default();
        let expected = json!({"type": "histogram2d"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn serialize_histogram2d_raw() {
        let trace = Histogram2d::new(vec![1.0, 2.0, 2.0], vec![1.0, 1.0, 3.0])
            .hist_func(HistFunc::Count)
            .hist_norm(HistNorm::Probability)
            .auto_bin_x(false)
            .auto_bin_y(false)
            .n_bins_x(10)
            .n_bins_y(10)
            .x_bins(Bins::new(0.0, 4.0, 1.0))
            .y_bins(Bins::new(0.0, 4.0, 1.0))
            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
            .show_scale(true)
            .name("hist2d");

        let expected = json!({
            "type": "histogram2d",
            "x": [1.0, 2.0, 2.0],
            "y": [1.0, 1.0, 3.0],
            "histfunc": "count",
            "histnorm": "probability",
            "autobinx": false,
            "autobiny": false,
            "nbinsx": 10,
            "nbinsy": 10,
            "xbins": {"start": 0.0, "end": 4.0, "size": 1.0},
            "ybins": {"start": 0.0, "end": 4.0, "size": 1.0},
            "colorscale": "Viridis",
            "showscale": true,
            "name": "hist2d",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_histogram2d_aggregation() {
        // `z` is per-sample aggregation data aligned with `x`/`y` (not a
        // pre-binned matrix); `hist_func` aggregates it within each bin.
        let trace = Histogram2d::new_xyz(
            vec![1.0, 2.0, 2.0],
            vec![1.0, 1.0, 3.0],
            vec![10.0, 20.0, 30.0],
        )
        .hist_func(HistFunc::Sum)
        .zauto(true)
        .zmin(0.0)
        .zmax(60.0)
        .zmid(30.0)
        .zsmooth(Smoothing::Best)
        .x_gap(1.0)
        .y_gap("10");

        let expected = json!({
            "type": "histogram2d",
            "x": [1.0, 2.0, 2.0],
            "y": [1.0, 1.0, 3.0],
            "z": [10.0, 20.0, 30.0],
            "histfunc": "sum",
            "zauto": true,
            "zmin": 0.0,
            "zmax": 60.0,
            "zmid": 30.0,
            "zsmooth": "best",
            "xgap": 1.0,
            "ygap": "10",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_histogram2d_legend_and_uirevision() {
        let trace = Histogram2d::new(vec![1.0], vec![2.0])
            .legend_rank(7)
            .legend_width(2.0)
            .ui_revision("rev");
        let v = to_value(trace).unwrap();
        assert_eq!(v["legendrank"], json!(7));
        assert_eq!(v["legendwidth"], json!(2.0));
        assert_eq!(v["uirevision"], json!("rev"));
    }
}
