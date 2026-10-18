//! Parallel coordinates trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{ColorBar, ColorScale, Dim, Domain, Font, LegendGroupTitle, PlotType, Visible},
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

/// Brushed selection on a [`Dimension`], in data units.
#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConstraintRange {
    /// A single `[min, max]` interval.
    Single([f64; 2]),
    /// Several `[min, max]` intervals (requires `multiselect`).
    Multiple(Vec<[f64; 2]>),
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LabelSide {
    Top,
    Bottom,
}

/// One vertical axis of a [`Parcoords`] trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Dimension {
    /// The values of this dimension, one per line.
    values: Option<Vec<f64>>,
    /// The axis label shown above (or below) the dimension.
    label: Option<String>,
    /// The domain range shown on the axis; defaults to the data extent.
    range: Option<[f64; 2]>,
    #[serde(rename = "constraintrange")]
    constraint_range: Option<ConstraintRange>,
    /// Whether several constraint ranges may be brushed on this dimension.
    #[serde(rename = "multiselect")]
    multi_select: Option<bool>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    /// Shows the dimension when `true`, hides it when `false`.
    visible: Option<bool>,
    /// The item name used to refer to this dimension in a template.
    name: Option<String>,
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Dimension {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Lines of a [`Parcoords`] trace and their color scale.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct ParcoordsLine {
    /// A single color, or one numeric value per line mapped through
    /// `colorscale`.
    color: Option<Dim<Box<dyn Color>>>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<String>,
    #[serde(rename = "cauto")]
    c_auto: Option<bool>,
    #[serde(rename = "cmin")]
    c_min: Option<f64>,
    #[serde(rename = "cmax")]
    c_max: Option<f64>,
    #[serde(rename = "cmid")]
    c_mid: Option<f64>,
}

impl ParcoordsLine {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Style of the lines outside the brushed constraint ranges.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct UnselectedLine {
    color: Option<Box<dyn Color>>,
    opacity: Option<f64>,
}

impl UnselectedLine {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Unselected {
    line: Option<UnselectedLine>,
}

impl Unselected {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a parallel coordinates trace: each row of data is drawn as a
/// polyline across a set of vertical [`Dimension`] axes.
///
/// # Examples
///
/// ```
/// use plotly::{parcoords::Dimension, Parcoords};
///
/// let trace = Parcoords::new(vec![
///     Dimension::new().label("A").values(vec![1.0, 2.0]),
///     Dimension::new().label("B").values(vec![3.0, 1.0]),
/// ]);
///
/// let expected = serde_json::json!({
///     "type": "parcoords",
///     "dimensions": [
///         {"label": "A", "values": [1.0, 2.0]},
///         {"label": "B", "values": [3.0, 1.0]},
///     ],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Parcoords {
    #[field_setter(default = "PlotType::Parcoords")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    ids: Option<Vec<String>>,
    dimensions: Option<Vec<Dimension>>,
    line: Option<ParcoordsLine>,
    unselected: Option<Unselected>,
    domain: Option<Domain>,
    /// Sets the angle of the dimension labels, in degrees.
    #[serde(rename = "labelangle")]
    label_angle: Option<f64>,
    #[serde(rename = "labelside")]
    label_side: Option<LabelSide>,
    #[serde(rename = "labelfont")]
    label_font: Option<Font>,
    #[serde(rename = "rangefont")]
    range_font: Option<Font>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    meta: Option<NumOrString>,
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl Parcoords {
    pub fn new(dimensions: Vec<Dimension>) -> Box<Self> {
        Box::new(Self {
            dimensions: Some(dimensions),
            ..Default::default()
        })
    }
}

impl Trace for Parcoords {
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
    fn serialize_constraint_range() {
        assert_eq!(
            to_value(ConstraintRange::Single([1.0, 2.0])).unwrap(),
            json!([1.0, 2.0])
        );
        assert_eq!(
            to_value(ConstraintRange::Multiple(vec![[1.0, 2.0], [4.0, 5.0]])).unwrap(),
            json!([[1.0, 2.0], [4.0, 5.0]])
        );
    }

    #[test]
    fn serialize_dimension() {
        let dimension = Dimension::new()
            .values(vec![1.0, 2.0])
            .label("A")
            .range([0.0, 10.0])
            .constraint_range(ConstraintRange::Single([1.0, 2.0]))
            .multi_select(false)
            .tick_format(".1f")
            .tick_values(vec![0.0, 5.0])
            .tick_text(vec!["zero", "five"])
            .visible(true)
            .name("a")
            .template_item_name("tmpl");

        let expected = json!({
            "values": [1.0, 2.0],
            "label": "A",
            "range": [0.0, 10.0],
            "constraintrange": [1.0, 2.0],
            "multiselect": false,
            "tickformat": ".1f",
            "tickvals": [0.0, 5.0],
            "ticktext": ["zero", "five"],
            "visible": true,
            "name": "a",
            "templateitemname": "tmpl",
        });

        assert_eq!(to_value(dimension).unwrap(), expected);
    }

    #[test]
    fn serialize_parcoords() {
        let trace = Parcoords::new(vec![Dimension::new()])
            .name("pc")
            .visible(Visible::True)
            .legend_group_title("G")
            .legend_rank(1)
            .legend_width(2.0)
            .ids(vec!["a"])
            .line(
                ParcoordsLine::new()
                    .color_array(vec![1.0, 2.0])
                    .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                    .auto_color_scale(false)
                    .reverse_scale(true)
                    .show_scale(true)
                    .color_bar(ColorBar::new())
                    .color_axis("coloraxis")
                    .c_auto(false)
                    .c_min(1.0)
                    .c_max(2.0)
                    .c_mid(1.5),
            )
            .unselected(Unselected::new().line(UnselectedLine::new().color("#aaa").opacity(0.2)))
            .domain(Domain::new().row(1))
            .label_angle(30.0)
            .label_side(LabelSide::Bottom)
            .label_font(Font::new())
            .range_font(Font::new())
            .tick_font(Font::new())
            .meta("m")
            .custom_data(vec!["cd"])
            .ui_revision(1);

        let expected = json!({
            "type": "parcoords",
            "name": "pc",
            "visible": true,
            "legendgrouptitle": {"text": "G"},
            "legendrank": 1,
            "legendwidth": 2.0,
            "ids": ["a"],
            "dimensions": [{}],
            "line": {
                "color": [1.0, 2.0],
                "colorscale": "Viridis",
                "autocolorscale": false,
                "reversescale": true,
                "showscale": true,
                "colorbar": {},
                "coloraxis": "coloraxis",
                "cauto": false,
                "cmin": 1.0,
                "cmax": 2.0,
                "cmid": 1.5,
            },
            "unselected": {"line": {"color": "#aaa", "opacity": 0.2}},
            "domain": {"row": 1},
            "labelangle": 30.0,
            "labelside": "bottom",
            "labelfont": {},
            "rangefont": {},
            "tickfont": {},
            "meta": "m",
            "customdata": ["cd"],
            "uirevision": 1,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
