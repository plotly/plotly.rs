//! Treemap trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::{Color, ColorArray};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::{
    common::{
        ColorBar, ColorScale, Dim, Domain, Font, HoverInfo, Label, Line, Pattern, PlotType,
        Position,
    },
    Trace,
};

/// Determines how the items in `values` are summed up the hierarchy.
///
/// When set to `Remainder`, the value of a parent sector is the sum of its
/// `values` plus the values of its child sectors. When set to `Total`, the
/// value of a parent sector is taken to be the total of its child sectors.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BranchValues {
    Remainder,
    Total,
}

/// Determines the tiling algorithm used to lay out the rectangles.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Packing {
    Squarify,
    Binary,
    Dice,
    Slice,
    SliceDice,
    DiceSlice,
}

/// Determines on which side of the the treemap the `pathbar` should be
/// presented.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Top,
    Bottom,
}

/// Configures the tiling behaviour of a [`Treemap`].
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Tiling {
    /// Determines the tiling algorithm used to lay out the rectangles.
    packing: Option<Packing>,
    /// When using `Packing::Squarify`, this option determines the aspect ratio
    /// targeted for each tile. The default value of `1` produces tiles that
    /// are as square as possible.
    #[serde(rename = "squarifyratio")]
    squarify_ratio: Option<f64>,
    /// Determines if the positions obtained from solver are flipped on each
    /// axis. Valid combinations of `"x"` and `"y"` (e.g. `"x"`, `"y"`,
    /// `"x+y"`).
    flip: Option<String>,
    /// Sets the inner padding (in px) between sectors.
    pad: Option<f64>,
}

impl Tiling {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configures the `pathbar` (the breadcrumb header) of a [`Treemap`].
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct PathBar {
    /// Determines if the path bar is drawn.
    visible: Option<bool>,
    /// Determines on which side of the treemap the path bar is presented.
    side: Option<Side>,
    /// Determines which shape is used to draw the edges between the path bar's
    /// sectors. One of `">"`, `"<"`, `"|"`, `"/"` or `"\\"`.
    #[serde(rename = "edgeshape")]
    edge_shape: Option<String>,
    /// Sets the thickness of the path bar (in px).
    thickness: Option<usize>,
    /// Sets the font used inside the path bar.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
}

impl PathBar {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Sets the inner padding (in px) of the treemap tiles.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Pad {
    /// Sets the padding form the top (in px).
    #[serde(rename = "t")]
    top: Option<f64>,
    /// Sets the padding form the left (in px).
    #[serde(rename = "l")]
    left: Option<f64>,
    /// Sets the padding form the right (in px).
    #[serde(rename = "r")]
    right: Option<f64>,
    /// Sets the padding form the bottom (in px).
    #[serde(rename = "b")]
    bottom: Option<f64>,
}

impl Pad {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Determines if the sector colors are faded towards the background color of
/// the chart as a function of depth.
///
/// Serializes to the values understood by Plotly.js: `Enabled` -> `true`,
/// `Disabled` -> `false`, `Reversed` -> `"reversed"`.
#[derive(Clone, Debug)]
pub enum DepthFade {
    Enabled,
    Disabled,
    Reversed,
}

impl Serialize for DepthFade {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DepthFade::Enabled => serializer.serialize_bool(true),
            DepthFade::Disabled => serializer.serialize_bool(false),
            DepthFade::Reversed => serializer.serialize_str("reversed"),
        }
    }
}

/// Configures the appearance of the sectors of a [`Treemap`].
///
/// This is a treemap-specific marker: in addition to the shared
/// color/colorscale machinery it exposes `pad`, `corner_radius` and
/// `depth_fade`, which the common [`Marker`](crate::common::Marker) does not.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Marker {
    /// Sets the color of each sector of the treemap. If not specified, the
    /// default trace color set is used to pick the sector colors.
    #[field_setter(skip)]
    colors: Option<Vec<Box<dyn Color>>>,
    /// Determines whether the colorscale is a default palette (`true`) or the
    /// palette determined by `cmin` and `cmax`.
    cauto: Option<bool>,
    /// Sets the lower bound of the color domain.
    cmin: Option<f64>,
    /// Sets the upper bound of the color domain.
    cmax: Option<f64>,
    /// Sets the mid-point of the color domain by scaling `cmin` and/or `cmax`
    /// to be equidistant to this point.
    cmid: Option<f64>,
    /// Sets the colorscale.
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    /// Determines whether the colorscale is picked using the sign of the input
    /// `colors`.
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    /// Reverses the color mapping if `true`.
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    /// Determines whether or not a colorbar is displayed.
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    /// Sets the colorbar.
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    /// Sets the outline of the sectors.
    line: Option<Line>,
    /// Sets the pattern within the sectors.
    pattern: Option<Pattern>,
    /// Sets the inner padding (in px) of the tiles.
    pad: Option<Pad>,
    /// Sets the maximum rounding of the corners (in px) of the tiles.
    #[serde(rename = "cornerradius")]
    corner_radius: Option<f64>,
    /// Determines if the sector colors are faded towards the background color
    /// of the chart as a function of depth.
    #[serde(rename = "depthfade")]
    depth_fade: Option<DepthFade>,
}

impl Marker {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the color of each sector of the treemap.
    pub fn colors<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.colors = Some(ColorArray(colors).into());
        self
    }
}

/// Construct a Treemap trace.
///
/// Treemap charts visualize hierarchical data using nested rectangles. The
/// hierarchy is defined via the `labels` and `parents` fields (the root is the
/// item whose parent is an empty string).
///
/// # Examples
///
/// ```
/// use plotly::Treemap;
///
/// let trace = Treemap::new(
///     vec!["Eve", "Cain", "Seth"],
///     vec!["", "Eve", "Eve"],
/// )
/// .values(vec![10, 14, 12]);
///
/// let expected = serde_json::json!({
///     "type": "treemap",
///     "labels": ["Eve", "Cain", "Seth"],
///     "parents": ["", "Eve", "Eve"],
///     "values": [10, 14, 12],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
///
/// Treemap styling uses the treemap-specific [`Marker`], which exposes the
/// `pad`, `corner_radius` and `depth_fade` attributes in addition to the shared
/// color/colorscale/line machinery.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Treemap<V>
where
    V: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Treemap")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appears as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible.
    visible: Option<bool>,
    /// Sets the opacity of the trace.
    opacity: Option<f64>,
    /// Assigns id labels to each datum. These ids are for object constancy of
    /// data points during animation.
    ids: Option<Vec<String>>,
    /// Sets the labels of each of the sectors.
    labels: Option<Vec<String>>,
    /// Sets the parent sectors for each of the sectors. Empty string items
    /// `""` are understood to reference the root node in the hierarchy.
    parents: Option<Vec<String>>,
    /// Sets the values associated with each of the sectors. Use with
    /// `branch_values` to determine how the values are summed.
    values: Option<Vec<V>>,
    /// Determines how the items in `values` are summed. When set to
    /// `Remainder`, the value of a parent is the sum of its `values` plus those
    /// of its children. When set to `Total`, the value of a parent is the total
    /// of its children.
    #[serde(rename = "branchvalues")]
    branch_values: Option<BranchValues>,
    /// Determines default for `values` when it is not provided, by inferring a
    /// `count`, i.e. the number of `"branches"` or `"leaves"`.
    count: Option<String>,
    /// Sets the level from which this trace hierarchy is rendered. Set `level`
    /// to `""` to start from the root node in the hierarchy.
    level: Option<NumOrString>,
    /// Sets the number of rendered sectors from any given `level`. Set
    /// `max_depth` to `-1` to render all the levels in the hierarchy.
    #[serde(rename = "maxdepth")]
    max_depth: Option<i32>,
    /// Sets the domain within which this trace is drawn.
    domain: Option<Domain>,
    marker: Option<Marker>,
    /// Sets the tiling behaviour of the treemap.
    tiling: Option<Tiling>,
    /// Sets the path bar (breadcrumb header) of the treemap.
    #[serde(rename = "pathbar")]
    path_bar: Option<PathBar>,
    /// Determines whether or not the sectors are reordered from largest to
    /// smallest.
    sort: Option<bool>,
    /// Sets text elements associated with each sector. If trace `text_info`
    /// contains a `"text"` flag, these elements will be seen on the chart.
    text: Option<Dim<String>>,
    /// Determines which trace information appears on the graph.
    #[serde(rename = "textinfo")]
    text_info: Option<String>,
    /// Template string used for rendering the information text that appears on
    /// points.
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "texttemplatefallback")]
    text_template_fallback: Option<Dim<String>>,
    /// Sets the font used for `text_info`.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    /// Sets the positions of the `text` elements.
    #[serde(rename = "textposition")]
    text_position: Option<Position>,
    /// Sets the font used for `text_info` lying inside the sector.
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    /// Sets the font used for `text_info` lying outside the sector.
    #[serde(rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    /// Determines which trace information appears on hover.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Sets hover text elements associated with each sector.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Template string used for rendering the information that appears on the
    /// hover box.
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
    /// Assign an id to this trace. Use this to provide object constancy between
    /// traces during animations and transitions.
    uid: Option<String>,
}

impl<V> Treemap<V>
where
    V: Serialize + Clone + 'static,
{
    /// Build a new Treemap trace from its `labels` and `parents`.
    ///
    /// The root sector is the one whose parent is an empty string `""`.
    pub fn new<L, P>(labels: Vec<L>, parents: Vec<P>) -> Box<Self>
    where
        L: Into<String>,
        P: Into<String>,
    {
        Box::new(Self {
            labels: Some(labels.into_iter().map(Into::into).collect()),
            parents: Some(parents.into_iter().map(Into::into).collect()),
            ..Default::default()
        })
    }
}

impl<V> Trace for Treemap<V>
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

    #[test]
    fn serialize_branch_values() {
        assert_eq!(
            to_value(BranchValues::Remainder).unwrap(),
            json!("remainder")
        );
        assert_eq!(to_value(BranchValues::Total).unwrap(), json!("total"));
    }

    #[test]
    fn serialize_packing() {
        assert_eq!(to_value(Packing::Squarify).unwrap(), json!("squarify"));
        assert_eq!(to_value(Packing::Binary).unwrap(), json!("binary"));
        assert_eq!(to_value(Packing::Dice).unwrap(), json!("dice"));
        assert_eq!(to_value(Packing::Slice).unwrap(), json!("slice"));
        assert_eq!(to_value(Packing::SliceDice).unwrap(), json!("slice-dice"));
        assert_eq!(to_value(Packing::DiceSlice).unwrap(), json!("dice-slice"));
    }

    #[test]
    fn serialize_tiling() {
        let tiling = Tiling::new()
            .packing(Packing::SliceDice)
            .squarify_ratio(1.5)
            .flip("x+y")
            .pad(2.0);
        let expected = json!({
            "packing": "slice-dice",
            "squarifyratio": 1.5,
            "flip": "x+y",
            "pad": 2.0,
        });
        assert_eq!(to_value(tiling).unwrap(), expected);
    }

    #[test]
    fn serialize_path_bar() {
        let path_bar = PathBar::new()
            .visible(true)
            .side(Side::Top)
            .edge_shape(">")
            .thickness(20)
            .text_font(Font::new());
        let expected = json!({
            "visible": true,
            "side": "top",
            "edgeshape": ">",
            "thickness": 20,
            "textfont": {},
        });
        assert_eq!(to_value(path_bar).unwrap(), expected);
    }

    #[test]
    fn serialize_depth_fade() {
        assert_eq!(to_value(DepthFade::Enabled).unwrap(), json!(true));
        assert_eq!(to_value(DepthFade::Disabled).unwrap(), json!(false));
        assert_eq!(to_value(DepthFade::Reversed).unwrap(), json!("reversed"));
    }

    #[test]
    fn serialize_marker() {
        let marker = Marker::new()
            .colors(vec!["#1f77b4", "#ff7f0e"])
            .show_scale(true)
            .cmin(0.0)
            .cmax(10.0)
            .line(Line::new().width(2.0))
            .pattern(Pattern::new())
            .pad(Pad::new().top(1.0).left(2.0).right(3.0).bottom(4.0))
            .corner_radius(5.0)
            .depth_fade(DepthFade::Reversed);
        let expected = json!({
            "colors": ["#1f77b4", "#ff7f0e"],
            "showscale": true,
            "cmin": 0.0,
            "cmax": 10.0,
            "line": {"width": 2.0},
            "pattern": {},
            "pad": {"t": 1.0, "l": 2.0, "r": 3.0, "b": 4.0},
            "cornerradius": 5.0,
            "depthfade": "reversed",
        });
        assert_eq!(to_value(marker).unwrap(), expected);
    }

    #[test]
    fn serialize_treemap() {
        let trace = Treemap::new(vec!["Eve", "Cain", "Seth"], vec!["", "Eve", "Eve"])
            .values(vec![10, 14, 12])
            .branch_values(BranchValues::Total)
            .count("leaves")
            .level("Eve")
            .max_depth(3)
            .name("family")
            .visible(true)
            .opacity(0.8)
            .ids(vec!["eve", "cain", "seth"])
            .domain(Domain::new())
            .marker(Marker::new())
            .tiling(Tiling::new().packing(Packing::Squarify))
            .path_bar(PathBar::new().visible(true).side(Side::Top))
            .sort(true)
            .text_info("label+value")
            .text_font(Font::new())
            .text_position(Position::TopCenter)
            .inside_text_font(Font::new())
            .outside_text_font(Font::new())
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("%{label}: %{value}")
            .meta("meta")
            .custom_data(vec!["custom_data"])
            .uid("uid-1");
        let expected = json!({
            "type": "treemap",
            "labels": ["Eve", "Cain", "Seth"],
            "parents": ["", "Eve", "Eve"],
            "values": [10, 14, 12],
            "branchvalues": "total",
            "count": "leaves",
            "level": "Eve",
            "maxdepth": 3,
            "name": "family",
            "visible": true,
            "opacity": 0.8,
            "ids": ["eve", "cain", "seth"],
            "domain": {},
            "marker": {},
            "tiling": {"packing": "squarify"},
            "pathbar": {"visible": true, "side": "top"},
            "sort": true,
            "textinfo": "label+value",
            "textfont": {},
            "textposition": "top center",
            "insidetextfont": {},
            "outsidetextfont": {},
            "hoverinfo": "all",
            "hoverlabel": {},
            "hovertemplate": "%{label}: %{value}",
            "meta": "meta",
            "customdata": ["custom_data"],
            "uid": "uid-1",
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
