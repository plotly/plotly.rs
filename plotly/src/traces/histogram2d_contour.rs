//! 2D histogram contour trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{
        Calendar, ColorBar, ColorScale, Dim, Font, HoverInfo, Label, LegendGroupTitle, Line,
        PlotType, Visible, XAxisId, YAxisId,
    },
    private::{NumOrString, NumOrStringCollection},
    traces::contour::Contours,
    traces::histogram::{Bins, HistFunc, HistNorm},
    Trace,
};

/// Construct a 2D histogram contour trace.
///
/// A `Histogram2dContour` bins raw `x`/`y` samples into a 2D grid, like
/// [`Histogram2d`](crate::Histogram2d), and renders the binned counts as
/// contours instead of a heatmap. Contour levels are set with
/// `auto_contour`/`n_contours` or explicitly via [`Contours`]. Provide raw
/// `x`/`y` data and let Plotly.js bin it via `hist_func`/`n_bins_x`/`n_bins_y`/
/// `x_bins`/`y_bins`. Optionally supply per-sample aggregation values via `z`
/// (a 1D array aligned with `x`/`y`) and a `hist_func` such as
/// [`HistFunc::Sum`] or [`HistFunc::Average`] to aggregate those values within
/// each bin instead of counting samples.
///
/// Note: unlike [`HeatMap`](crate::HeatMap), `z` here is per-sample aggregation
/// data aligned with `x`/`y`, not a pre-binned 2D matrix. For a pre-computed
/// matrix, use [`HeatMap`](crate::HeatMap).
///
/// # Examples
///
/// ```
/// use plotly::Histogram2dContour;
///
/// let trace = Histogram2dContour::new(vec![1.0, 2.0, 2.0], vec![1.0, 1.0, 3.0]);
///
/// let expected = serde_json::json!({
///     "type": "histogram2dcontour",
///     "x": [1.0, 2.0, 2.0],
///     "y": [1.0, 1.0, 3.0],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Histogram2dContour<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Histogram2dContour")]
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
    ids: Option<Vec<String>>,
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
    /// Traces sharing a `bingroup` (with matching axis types) share the same
    /// x and y bin settings.
    #[serde(rename = "bingroup")]
    bin_group: Option<String>,
    #[serde(rename = "xbingroup")]
    x_bin_group: Option<String>,
    #[serde(rename = "ybingroup")]
    y_bin_group: Option<String>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,
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
    /// Determines whether the contour levels are chosen automatically
    /// (`true`) or set by `contours.start`/`end`/`size`.
    #[serde(rename = "autocontour")]
    auto_contour: Option<bool>,
    /// Sets the maximum number of contour levels when `auto_contour` is
    /// `true`.
    #[serde(rename = "ncontours")]
    n_contours: Option<usize>,
    contours: Option<Contours>,
    /// Sets the contour line color, dash, width and smoothing.
    line: Option<Line>,
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

impl<X, Y> Histogram2dContour<X, Y, f64>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    /// Build a new 2D histogram contour from raw `x` and `y` samples. Plotly.js
    /// bins the samples into a 2D grid.
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> Histogram2dContour<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    /// Build a new 2D histogram contour from raw `x`/`y` samples plus
    /// per-sample aggregation values `z`. All three vectors are aligned
    /// (one entry per sample); combine with a `hist_func` such as
    /// [`HistFunc::Sum`] to aggregate the `z` values within each `x`/`y`
    /// bin.
    pub fn new_xyz(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> Trace for Histogram2dContour<X, Y, Z>
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
    fn default_histogram2d_contour() {
        let trace: Histogram2dContour<f64, f64, f64> = Histogram2dContour::default();
        assert_eq!(
            to_value(trace).unwrap(),
            json!({"type": "histogram2dcontour"})
        );
    }

    #[test]
    fn serialize_histogram2d_contour() {
        let trace = Histogram2dContour::new_xyz(vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0])
            .name("h2c")
            .ids(vec!["i1", "i2"])
            .bin_group("bg")
            .x_bin_group("xbg")
            .y_bin_group("ybg")
            .color_axis("coloraxis2")
            .hist_func(HistFunc::Sum)
            .n_bins_x(10)
            .n_bins_y(20)
            .auto_contour(false)
            .n_contours(5)
            .contours(Contours::new().show_labels(true))
            .line(Line::new().width(0.5))
            .color_scale(ColorScale::Palette(ColorScalePalette::Blues))
            .x_axis("x2")
            .y_axis("y2");

        let expected = json!({
            "type": "histogram2dcontour",
            "name": "h2c",
            "ids": ["i1", "i2"],
            "bingroup": "bg",
            "xbingroup": "xbg",
            "ybingroup": "ybg",
            "coloraxis": "coloraxis2",
            "x": [1.0, 2.0],
            "y": [3.0, 4.0],
            "z": [5.0, 6.0],
            "histfunc": "sum",
            "nbinsx": 10,
            "nbinsy": 20,
            "autocontour": false,
            "ncontours": 5,
            "contours": {"showlabels": true},
            "line": {"width": 0.5},
            "colorscale": "Blues",
            "xaxis": "x2",
            "yaxis": "y2",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
