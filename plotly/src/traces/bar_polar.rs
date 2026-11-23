//! Polar bar trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    common::{Dim, Label, LegendGroupTitle, Marker, PlotType, Selection, Visible},
    layout::ThetaUnit,
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

/// Construct a polar bar trace: bars drawn on a polar subplot, with the radial
/// span set by `r` and the angular position by `theta`. Typically used for wind
/// roses. Stacking and the gap between bars are controlled by
/// [`LayoutPolar::bar_mode`](crate::layout::LayoutPolar::bar_mode) and
/// [`LayoutPolar::bar_gap`](crate::layout::LayoutPolar::bar_gap).
///
/// # Examples
///
/// ```
/// use plotly::BarPolar;
///
/// let trace = BarPolar::new(vec!["N", "E", "S", "W"], vec![1.5, 2.0, 0.5, 3.0]);
///
/// let expected = serde_json::json!({
///     "type": "barpolar",
///     "theta": ["N", "E", "S", "W"],
///     "r": [1.5, 2.0, 0.5, 3.0],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct BarPolar<Theta, R>
where
    Theta: Serialize + Clone + 'static,
    R: Serialize + Clone + 'static,
{
    #[field_setter(default = "PlotType::BarPolar")]
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
    /// Sets the radial coordinates.
    r: Option<Vec<R>>,
    /// Alternate to `r`: builds a linear space of r coordinates starting at
    /// `r0` with step `dr`.
    r0: Option<NumOrString>,
    dr: Option<f64>,
    /// Sets the angular coordinates.
    theta: Option<Vec<Theta>>,
    /// Alternate to `theta`: builds a linear space of theta coordinates
    /// starting at `theta0` with step `dtheta`.
    theta0: Option<NumOrString>,
    dtheta: Option<f64>,
    /// Sets the unit of input `theta` values. Has an effect only when on a
    /// linear angular axis.
    #[serde(rename = "thetaunit")]
    theta_unit: Option<ThetaUnit>,
    /// Sets where the bar base is drawn (in radial axis units).
    base: Option<Dim<NumOrString>>,
    /// Shifts the angular position where the bar is drawn (in `thetaunit`
    /// units).
    offset: Option<Dim<f64>>,
    /// Sets the bar angular width (in `thetaunit` units).
    width: Option<Dim<f64>>,
    /// Sets a reference between this trace's data coordinates and a polar
    /// subplot, e.g. `"polar"` (the default) or `"polar2"`.
    subplot: Option<String>,
    text: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Flaglist of `r`, `theta`, `text`, `name` joined with `+`, or one of
    /// `all`, `none`, `skip`.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<String>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    marker: Option<Marker>,
    selected: Option<Selection>,
    unselected: Option<Selection>,
    #[serde(rename = "selectedpoints")]
    selected_points: Option<Vec<u32>>,
    meta: Option<NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<Theta, R> BarPolar<Theta, R>
where
    Theta: Serialize + Clone + 'static,
    R: Serialize + Clone + 'static,
{
    pub fn new(theta: Vec<Theta>, r: Vec<R>) -> Box<Self> {
        Box::new(Self {
            theta: Some(theta),
            r: Some(r),
            ..Default::default()
        })
    }
}

impl<Theta, R> Trace for BarPolar<Theta, R>
where
    Theta: Serialize + Clone + 'static,
    R: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn default_bar_polar() {
        let trace = BarPolar::<f64, f64>::default();
        assert_eq!(to_value(trace).unwrap(), json!({"type": "barpolar"}));
    }

    #[test]
    fn serialize_bar_polar() {
        let trace = BarPolar::new(vec![0.0, 90.0], vec![1, 2])
            .name("wind")
            .visible(Visible::True)
            .show_legend(true)
            .legend_group("g")
            .legend_group_title("G")
            .legend_rank(5)
            .legend_width(10.0)
            .opacity(0.8)
            .ids(vec!["a", "b"])
            .r0(0)
            .dr(1.0)
            .theta0(0)
            .dtheta(45.0)
            .theta_unit(ThetaUnit::Gradians)
            .base(NumOrString::F(0.5))
            .offset_array(vec![1.0, 2.0])
            .width(10.0)
            .subplot("polar2")
            .text("text")
            .hover_text_array(vec!["h1", "h2"])
            .hover_info("r+theta")
            .hover_template("%{r}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .marker(Marker::new().opacity(0.5))
            .selected(Selection::new())
            .unselected(Selection::new())
            .selected_points(vec![0])
            .meta("meta")
            .custom_data(vec!["cd"])
            .ui_revision("rev");

        let expected = json!({
            "type": "barpolar",
            "name": "wind",
            "visible": true,
            "showlegend": true,
            "legendgroup": "g",
            "legendgrouptitle": {"text": "G"},
            "legendrank": 5,
            "legendwidth": 10.0,
            "opacity": 0.8,
            "ids": ["a", "b"],
            "r": [1, 2],
            "r0": 0,
            "dr": 1.0,
            "theta": [0.0, 90.0],
            "theta0": 0,
            "dtheta": 45.0,
            "thetaunit": "gradians",
            "base": 0.5,
            "offset": [1.0, 2.0],
            "width": 10.0,
            "subplot": "polar2",
            "text": "text",
            "hovertext": ["h1", "h2"],
            "hoverinfo": "r+theta",
            "hovertemplate": "%{r}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "marker": {"opacity": 0.5},
            "selected": {"marker": {}},
            "unselected": {"marker": {}},
            "selectedpoints": [0],
            "meta": "meta",
            "customdata": ["cd"],
            "uirevision": "rev",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
