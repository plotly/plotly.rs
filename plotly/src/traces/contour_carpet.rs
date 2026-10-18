//! Carpet contour trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{ColorBar, ColorScale, LegendGroupTitle, Line, PlotType, Visible, XAxisId, YAxisId},
    private::{NumOrString, NumOrStringCollection},
    traces::contour::Contours,
    Trace,
};

/// Whether the `a`/`b` coordinates of a [`ContourCarpet`] are given as an
/// array or as a linear scaled space (from `a0`/`da` or `b0`/`db`).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CoordinateType {
    Array,
    Scaled,
}

/// Construct a contour trace drawn on a [`Carpet`](crate::Carpet). The
/// carpet is selected by its [`carpet`](ContourCarpet::carpet) id.
///
/// # Examples
///
/// ```
/// use plotly::ContourCarpet;
///
/// let trace = ContourCarpet::new(vec![1.0, 2.0, 3.0, 4.0])
///     .a(vec![0.0, 1.0, 0.0, 1.0])
///     .b(vec![0.0, 0.0, 1.0, 1.0])
///     .carpet("c1");
///
/// let expected = serde_json::json!({
///     "type": "contourcarpet",
///     "z": [1.0, 2.0, 3.0, 4.0],
///     "a": [0.0, 1.0, 0.0, 1.0],
///     "b": [0.0, 0.0, 1.0, 1.0],
///     "carpet": "c1",
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct ContourCarpet<Z, A = f64, B = f64>
where
    Z: Serialize + Clone,
    A: Serialize + Clone,
    B: Serialize + Clone,
{
    #[field_setter(default = "PlotType::ContourCarpet")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    /// The [`carpet`](crate::Carpet::carpet) id of the carpet this trace lies
    /// on. If omitted, the first carpet is used.
    carpet: Option<String>,
    /// Sets the z data, a 1D or 2D array aligned with the carpet points.
    z: Option<Vec<Z>>,
    /// Sets the `a` coordinates.
    a: Option<Vec<A>>,
    /// Alternate to `a`: builds a linear space of `a` coordinates starting at
    /// `a0` with step `da`.
    a0: Option<NumOrString>,
    da: Option<f64>,
    #[serde(rename = "atype")]
    a_type: Option<CoordinateType>,
    /// Sets the `b` coordinates.
    b: Option<Vec<B>>,
    /// Alternate to `b`: builds a linear space of `b` coordinates starting at
    /// `b0` with step `db`.
    b0: Option<NumOrString>,
    db: Option<f64>,
    #[serde(rename = "btype")]
    b_type: Option<CoordinateType>,
    /// Transposes the z data.
    transpose: Option<bool>,
    #[serde(rename = "autocontour")]
    auto_contour: Option<bool>,
    /// Sets the maximum number of contour levels when `autocontour` is
    /// `true`.
    #[serde(rename = "ncontours")]
    n_contours: Option<usize>,
    contours: Option<Contours>,
    line: Option<Line>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    text: Option<Vec<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(rename = "zauto")]
    z_auto: Option<bool>,
    #[serde(rename = "zmin")]
    z_min: Option<f64>,
    #[serde(rename = "zmax")]
    z_max: Option<f64>,
    #[serde(rename = "zmid")]
    z_mid: Option<f64>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    /// Sets the layer on which this trace is displayed, relative to other
    /// SVG traces on the same subplot.
    #[serde(rename = "zorder")]
    z_order: Option<i32>,
    meta: Option<NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<Z> ContourCarpet<Z>
where
    Z: Serialize + Clone,
{
    pub fn new(z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<Z, A, B> Trace for ContourCarpet<Z, A, B>
where
    Z: Serialize + Clone,
    A: Serialize + Clone,
    B: Serialize + Clone,
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
    fn serialize_coordinate_type() {
        assert_eq!(to_value(CoordinateType::Array).unwrap(), json!("array"));
        assert_eq!(to_value(CoordinateType::Scaled).unwrap(), json!("scaled"));
    }

    #[test]
    fn serialize_contour_carpet() {
        let trace = ContourCarpet::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]])
            .name("cc")
            .visible(Visible::True)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .opacity(0.8)
            .ids(vec!["i"])
            .carpet("c1")
            .a(vec![0.0, 1.0])
            .a0(0)
            .da(1.0)
            .a_type(CoordinateType::Array)
            .b(vec![0.0, 1.0])
            .b0(0)
            .db(1.0)
            .b_type(CoordinateType::Scaled)
            .transpose(true)
            .auto_contour(false)
            .n_contours(10)
            .contours(Contours::new().show_lines(true))
            .line(Line::new().width(1.0))
            .fill_color("#abcdef")
            .text(vec!["t"])
            .hover_text(vec!["h"])
            .z_auto(false)
            .z_min(0.0)
            .z_max(4.0)
            .z_mid(2.0)
            .color_axis("coloraxis")
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Hot))
            .auto_color_scale(false)
            .reverse_scale(true)
            .show_scale(false)
            .x_axis("x2")
            .y_axis("y2")
            .z_order(2)
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "contourcarpet",
            "name": "cc",
            "visible": true,
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "opacity": 0.8,
            "ids": ["i"],
            "carpet": "c1",
            "z": [[1.0, 2.0], [3.0, 4.0]],
            "a": [0.0, 1.0],
            "a0": 0,
            "da": 1.0,
            "atype": "array",
            "b": [0.0, 1.0],
            "b0": 0,
            "db": 1.0,
            "btype": "scaled",
            "transpose": true,
            "autocontour": false,
            "ncontours": 10,
            "contours": {"showlines": true},
            "line": {"width": 1.0},
            "fillcolor": "#abcdef",
            "text": ["t"],
            "hovertext": ["h"],
            "zauto": false,
            "zmin": 0.0,
            "zmax": 4.0,
            "zmid": 2.0,
            "coloraxis": "coloraxis",
            "colorbar": {},
            "colorscale": "Hot",
            "autocolorscale": false,
            "reversescale": true,
            "showscale": false,
            "xaxis": "x2",
            "yaxis": "y2",
            "zorder": 2,
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": 1,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
