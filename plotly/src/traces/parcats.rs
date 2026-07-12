//! Parallel categories (parcats) trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{ColorScale, Dim, Domain, PlotType},
    layout::CategoryOrder,
    Trace,
};

/// Sets the shape of the paths connecting the categories.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParcatsLineShape {
    Linear,
    Hspline,
}

/// Sets the drag interaction mode for the categories and dimensions.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParcatsArrangement {
    Perpendicular,
    Freeform,
    Fixed,
}

/// Sets the hover interaction mode for the parcats diagram.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParcatsHoverOn {
    Category,
    Color,
    Dimension,
}

/// Sets the path sorting algorithm.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParcatsSortPaths {
    Forward,
    Backward,
}

/// Determines which trace information appears on hover for a parcats trace.
///
/// Unlike the cartesian [`HoverInfo`](crate::common::HoverInfo), parcats hover
/// info is limited to `count`/`probability` (plus `all`/`none`/`skip`).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParcatsHoverInfo {
    Count,
    Probability,
    #[serde(rename = "count+probability")]
    CountAndProbability,
    All,
    None,
    Skip,
}

/// A single dimension (column) of a [`Parcats`] trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ParcatsDimension<V>
where
    V: Serialize + Clone,
{
    /// The shown name of the dimension.
    label: Option<String>,
    /// Sets the category values, one per data point.
    values: Option<Vec<V>>,
    /// Specifies the ordering logic for the categories in the dimension.
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    /// Sets the order in which categories in this dimension appear, only used
    /// if `category_order` is set to "array".
    #[serde(rename = "categoryarray")]
    category_array: Option<Vec<V>>,
    /// Sets alternative tick labels for the categories in this dimension.
    ticktext: Option<Vec<String>>,
    /// The display index of the dimension, from left to right, zero indexed,
    /// defaults to dimension index.
    #[serde(rename = "displayindex")]
    display_index: Option<usize>,
    /// Determines whether or not this dimension is visible.
    visible: Option<bool>,
}

impl<V> ParcatsDimension<V>
where
    V: Serialize + Clone,
{
    pub fn new() -> Self {
        Default::default()
    }
}

/// Styles the paths (lines) connecting the categories of a [`Parcats`] trace.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ParcatsLine {
    /// Sets the line color. It accepts either a specific color or an array of
    /// numbers that are mapped to the colorscale.
    color: Option<Dim<Box<dyn Color>>>,
    /// Sets the colorscale used to map the `color` values to colors.
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    /// Sets the shape of the paths.
    shape: Option<ParcatsLineShape>,
    /// Sets the lower bound of the color domain.
    cmin: Option<f64>,
    /// Sets the upper bound of the color domain.
    cmax: Option<f64>,
    /// Sets the mid-point of the color domain by scaling `cmin` and/or `cmax`.
    cmid: Option<f64>,
    /// Determines whether or not a colorbar is displayed for this trace.
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
}

impl ParcatsLine {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a parallel categories (parcats) trace.
///
/// A `Parcats` trace visualizes multi-dimensional categorical data as a set of
/// parallel axes, one per [`ParcatsDimension`], with ribbons flowing between
/// the categories. It is domain-based (like Sankey), so it does not share the
/// cartesian subplot grid.
///
/// # Examples
///
/// ```
/// use plotly::Parcats;
/// use plotly::parcats::{ParcatsArrangement, ParcatsDimension};
///
/// let trace = Parcats::new()
///     .dimensions(vec![
///         ParcatsDimension::new().label("A").values(vec!["x", "y", "x"]),
///         ParcatsDimension::new().label("B").values(vec!["p", "q", "p"]),
///     ])
///     .counts_array(vec![1.0, 2.0, 3.0])
///     .arrangement(ParcatsArrangement::Perpendicular);
///
/// let expected = serde_json::json!({
///     "type": "parcats",
///     "dimensions": [
///         {"label": "A", "values": ["x", "y", "x"]},
///         {"label": "B", "values": ["p", "q", "p"]}
///     ],
///     "counts": [1.0, 2.0, 3.0],
///     "arrangement": "perpendicular"
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Parcats<V>
where
    V: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Parcats")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appears as the legend item and on
    /// hover.
    name: Option<String>,
    /// The dimensions (columns) of the parallel-categories diagram.
    dimensions: Option<Vec<ParcatsDimension<V>>>,
    /// Styles the paths connecting the categories.
    line: Option<ParcatsLine>,
    /// The number of observations represented by each state. Defaults to 1 so
    /// that each state represents one observation. This lets callers pass
    /// aggregated per-path weights instead of one row per record.
    counts: Option<Dim<f64>>,
    /// Sets the drag interaction mode for categories and dimensions.
    arrangement: Option<ParcatsArrangement>,
    /// Sort paths so that like colors are bundled together within each
    /// category.
    #[serde(rename = "bundlecolors")]
    bundle_colors: Option<bool>,
    /// Sets the hover interaction mode for the parcats diagram.
    #[serde(rename = "hoveron")]
    hover_on: Option<ParcatsHoverOn>,
    /// Sets the path sorting algorithm.
    #[serde(rename = "sortpaths")]
    sort_paths: Option<ParcatsSortPaths>,
    /// Template string used for rendering the information that appear on hover
    /// box.
    // NOTE: parcats also accepts `line.hovertemplate`; trace-level placement is
    // the more general form and is used here.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<String>,
    /// Determines which trace information appears on hover. Limited to
    /// `count`/`probability` (plus `all`/`none`/`skip`).
    #[serde(rename = "hoverinfo")]
    hover_info: Option<ParcatsHoverInfo>,
    /// Sets the domain within which this parcats trace is drawn.
    domain: Option<Domain>,
}

impl<V> Parcats<V>
where
    V: Serialize + Clone,
{
    /// Creates a new empty parcats trace.
    pub fn new() -> Box<Self> {
        Box::default()
    }
}

impl<V> Trace for Parcats<V>
where
    V: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::color::NamedColor;

    #[test]
    fn serialize_default_parcats() {
        let trace = Parcats::<i32>::default();
        let expected = json!({"type": "parcats"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn serialize_basic_parcats_trace() {
        let trace = Parcats::new()
            .name("parcats")
            .dimensions(vec![
                ParcatsDimension::new()
                    .label("Sex")
                    .values(vec!["M", "F", "F"]),
                ParcatsDimension::new()
                    .label("Survived")
                    .values(vec!["yes", "no", "yes"]),
            ])
            .counts_array(vec![1.0, 2.0, 3.0])
            .arrangement(ParcatsArrangement::Perpendicular)
            .bundle_colors(true)
            .hover_on(ParcatsHoverOn::Category)
            .sort_paths(ParcatsSortPaths::Forward);

        let expected = json!({
            "type": "parcats",
            "name": "parcats",
            "dimensions": [
                {"label": "Sex", "values": ["M", "F", "F"]},
                {"label": "Survived", "values": ["yes", "no", "yes"]}
            ],
            "counts": [1.0, 2.0, 3.0],
            "arrangement": "perpendicular",
            "bundlecolors": true,
            "hoveron": "category",
            "sortpaths": "forward"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_parcats_line() {
        let line = ParcatsLine::new()
            .color(NamedColor::Blue)
            .shape(ParcatsLineShape::Hspline)
            .cmin(0.0)
            .cmax(1.0)
            .show_scale(true);
        let expected = json!({
            "color": "blue",
            "shape": "hspline",
            "cmin": 0.0,
            "cmax": 1.0,
            "showscale": true
        });

        assert_eq!(to_value(line).unwrap(), expected);
    }

    #[test]
    fn serialize_parcats_hover_info() {
        assert_eq!(to_value(ParcatsHoverInfo::Count).unwrap(), json!("count"));
        assert_eq!(
            to_value(ParcatsHoverInfo::Probability).unwrap(),
            json!("probability")
        );
        assert_eq!(
            to_value(ParcatsHoverInfo::CountAndProbability).unwrap(),
            json!("count+probability")
        );
        assert_eq!(to_value(ParcatsHoverInfo::Skip).unwrap(), json!("skip"));

        let trace = Parcats::<i32>::new().hover_info(ParcatsHoverInfo::Count);
        assert_eq!(to_value(trace).unwrap()["hoverinfo"], json!("count"));
    }

    #[test]
    fn serialize_parcats_dimension() {
        let dim = ParcatsDimension::new()
            .label("A")
            .values(vec![0, 1, 0])
            .category_order(CategoryOrder::CategoryAscending)
            .category_array(vec![0, 1])
            .ticktext(vec!["zero", "one"])
            .display_index(2)
            .visible(true);
        let expected = json!({
            "label": "A",
            "values": [0, 1, 0],
            "categoryorder": "category ascending",
            "categoryarray": [0, 1],
            "ticktext": ["zero", "one"],
            "displayindex": 2,
            "visible": true
        });

        assert_eq!(to_value(dim).unwrap(), expected);
    }
}
