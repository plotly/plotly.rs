//! Sankey trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::{Color, ColorArray},
    common::{Dim, Domain, Font, HoverInfo, Label, LegendGroupTitle, Orientation, PlotType},
    Trace,
};

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Arrangement {
    Snap,
    Perpendicular,
    Freeform,
    Fixed,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
pub struct Line {
    color: Option<Dim<Box<dyn Color>>>,
    width: Option<f64>,
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(Box::new(color)));
        self
    }

    pub fn color_array<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.color = Some(Dim::Vector(ColorArray(colors).into()));
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Clone)]
pub struct Node {
    // Missing: customdata, groups
    color: Option<Dim<Box<dyn Color>>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    label: Option<Vec<String>>,
    line: Option<Line>,
    pad: Option<usize>,
    thickness: Option<usize>,
    x: Option<f64>,
    y: Option<f64>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(Box::new(color)));
        self
    }

    pub fn color_array<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.color = Some(Dim::Vector(ColorArray(colors).into()));
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Self {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_template(mut self, hover_template: &str) -> Self {
        self.hover_template = Some(Dim::Scalar(hover_template.to_string()));
        self
    }

    pub fn label(mut self, label: Vec<&str>) -> Self {
        self.label = Some(label.iter().map(|&el| el.to_string()).collect());
        self
    }

    pub fn line(mut self, line: Line) -> Self {
        self.line = Some(line);
        self
    }

    pub fn pad(mut self, pad: usize) -> Self {
        self.pad = Some(pad);
        self
    }

    pub fn thickness(mut self, thickness: usize) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct Link<V>
where
    V: Serialize + Clone,
{
    // Missing: colorscales, customdata
    color: Option<Dim<Box<dyn Color>>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    line: Option<Line>,
    source: Option<Vec<usize>>,
    target: Option<Vec<usize>>,
    value: Option<Vec<V>>,
}

impl<V> Default for Link<V>
where
    V: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            color: None,
            hover_info: None,
            hover_label: None,
            hover_template: None,
            line: None,
            source: None,
            target: None,
            value: None,
        }
    }
}

impl<V> Link<V>
where
    V: Serialize + Clone,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(Box::new(color)));
        self
    }

    pub fn color_array<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.color = Some(Dim::Vector(ColorArray(colors).into()));
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Self {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_template(mut self, hover_template: &str) -> Self {
        self.hover_template = Some(Dim::Scalar(hover_template.to_string()));
        self
    }

    pub fn line(mut self, line: Line) -> Self {
        self.line = Some(line);
        self
    }

    pub fn source(mut self, source: Vec<usize>) -> Self {
        self.source = Some(source);
        self
    }

    pub fn target(mut self, target: Vec<usize>) -> Self {
        self.target = Some(target);
        self
    }

    pub fn value(mut self, target: Vec<V>) -> Self {
        self.value = Some(target);
        self
    }
}

/// Construct a Sankey trace.
///
/// # Examples
///
/// ```
/// use plotly::{
///     Sankey,
///     common::Orientation,
///     sankey::{Line, Link, Node}
/// };
///
/// let line = Line::new().color("#00FF00").width(0.5);
///
/// let node = Node::new()
///     .line(line)
///     .pad(15)
///     .thickness(30)
///     .label(vec!["A1", "A2", "B1", "B2", "C1", "C2"])
///     .color("#0000FF");
///
/// let link = Link::new()
///     .value(vec![8, 4, 2, 8, 4, 2])
///     .source(vec![0, 1, 0, 2, 3, 3])
///     .target(vec![2, 3, 3, 4, 4, 5]);
///
/// let trace = Sankey::new()
///     .node(node)
///     .link(link)
///     .orientation(Orientation::Horizontal);
///
/// let expected = serde_json::json!({
///     "type": "sankey",
///     "orientation": "h",
///     "node": {
///         "color": "#0000FF",
///         "label": ["A1", "A2", "B1", "B2", "C1", "C2"],
///         "thickness": 30,
///         "pad": 15,
///         "line": {
///             "color": "#00FF00",
///             "width": 0.5,
///         }
///     },
///     "link": {
///         "source": [0, 1, 0, 2, 3, 3],
///         "target": [2, 3, 3, 4, 4, 5],
///         "value": [8, 4, 2, 8, 4, 2]
///     }
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Sankey<V>
where
    V: Serialize + Clone,
{
    // Missing: meta, customdata, uirevision
    #[field_setter(default = "PlotType::Sankey")]
    r#type: PlotType,
    /// If value is `snap` (the default), the node arrangement is assisted by
    /// automatic snapping of elements to preserve space between nodes
    /// specified via `nodepad`. If value is `perpendicular`, the nodes can
    /// only move along a line perpendicular to the flow. If value is
    /// `freeform`, the nodes can freely move on the plane. If value is
    /// `fixed`, the nodes are stationary.
    arrangement: Option<Arrangement>,
    /// Sets the domain within which the Sankey diagram will be drawn.
    domain: Option<Domain>,
    /// Assigns id labels to each datum. These ids are for object constancy of
    /// data points during animation.
    ids: Option<Vec<String>>,
    /// Determines which trace information appear on hover. If `none` or `skip`
    /// are set, no information is displayed upon hovering. But, if `none`
    /// is set, click and hover events are still fired. Note that this attribute
    /// is superseded by `node.hover_info` and `link.hover_info` for nodes and
    /// links respectively.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    /// Sets the hover label for this trace.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Set and style the title to appear for the legend group
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000, so that you can use ranks less than 1000 to
    /// place certain items before all unranked items, and ranks greater
    /// than 1000 to go after all unranked items.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// The links of the Sankey diagram.
    link: Option<Link<V>>,
    /// Sets the trace name. The trace name appears as the legend item and on
    /// hover.
    name: Option<String>,
    /// The nodes of the Sankey diagram.
    node: Option<Node>,
    /// Sets the orientation of the Sankey diagram.
    orientation: Option<Orientation>,
    /// Vector containing integer indices of selected points. Has an effect only
    /// for traces that support selections. Note that an empty vector means
    /// an empty selection where the `unselected` are turned on for all
    /// points.
    #[serde(rename = "selectedpoints")]
    selected_points: Option<Vec<usize>>,
    /// Sets the font for node labels.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    /// Sets the value formatting rule using d3 formatting mini-languages which
    /// are very similar to those in Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format.
    #[serde(rename = "valueformat")]
    value_format: Option<String>,
    /// Adds a unit to follow the value in the hover tooltip. Add a space if a
    /// separation is necessary from the value.
    #[serde(rename = "valuesuffix")]
    value_suffix: Option<String>,
    /// Determines whether or not this trace is visible. If "legendonly", the
    /// trace is not drawn, but can appear as a legend item (provided that
    /// the legend itself is visible).
    visible: Option<bool>,
}

impl<V> Sankey<V>
where
    V: Serialize + Clone,
{
    /// Creates a new empty Sankey diagram.
    pub fn new() -> Box<Self> {
        Box::default()
    }
}

impl<V> Trace for Sankey<V>
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
    fn test_serialize_default_sankey() {
        let trace = Sankey::<i32>::default();
        let expected = json!({"type": "sankey"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_basic_sankey_trace() {
        // Mimic the plot here, minus the layout:
        // https://plotly.com/javascript/sankey-diagram/#basic-sankey-diagram
        let trace = Sankey::new()
            .orientation(Orientation::Horizontal)
            .node(
                Node::new()
                    .pad(15)
                    .thickness(30)
                    .line(Line::new().color(NamedColor::Black).width(0.5))
                    .label(vec!["A1", "A2", "B1", "B2", "C1", "C2"])
                    .color_array(vec![
                        NamedColor::Blue,
                        NamedColor::Blue,
                        NamedColor::Blue,
                        NamedColor::Blue,
                        NamedColor::Blue,
                        NamedColor::Blue,
                    ]),
            )
            .link(
                Link::new()
                    .value(vec![8, 4, 2, 8, 4, 2])
                    .source(vec![0, 1, 0, 2, 3, 3])
                    .target(vec![2, 3, 3, 4, 4, 5]),
            );

        let expected = json!({
            "link": {
                "source": [0, 1, 0, 2, 3, 3],
                "target": [2, 3, 3, 4, 4, 5],
                "value": [8, 4, 2, 8, 4, 2]
            },
            "orientation": "h",
            "type": "sankey",
            "node": {
                "color": ["blue", "blue", "blue", "blue", "blue", "blue"],
                "label": ["A1", "A2", "B1", "B2", "C1", "C2"],
                "line": {
                    "color": "black",
                    "width": 0.5
                },
                "pad": 15,
                "thickness": 30
            }
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_full_sankey_trace() {
        let trace = Sankey::<i32>::new()
            .name("sankey")
            .visible(true)
            .legend_rank(1000)
            .legend_group_title(LegendGroupTitle::new("Legend Group Title"))
            .ids(vec!["one"])
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .domain(Domain::new())
            .orientation(Orientation::Horizontal)
            .node(Node::new())
            .link(Link::new())
            .text_font(Font::new())
            .selected_points(vec![0])
            .arrangement(Arrangement::Fixed)
            .value_format(".3f")
            .value_suffix("nT");

        let expected = json!({
            "type": "sankey",
            "name": "sankey",
            "visible": true,
            "legendrank": 1000,
            "legendgrouptitle": {"text": "Legend Group Title"},
            "ids": ["one"],
            "hoverinfo": "all",
            "hoverlabel": {},
            "domain": {},
            "orientation": "h",
            "node": {},
            "link": {},
            "textfont": {},
            "selectedpoints": [0],
            "arrangement": "fixed",
            "valueformat": ".3f",
            "valuesuffix": "nT"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_arrangement() {
        assert_eq!(to_value(Arrangement::Snap).unwrap(), json!("snap"));
        assert_eq!(
            to_value(Arrangement::Perpendicular).unwrap(),
            json!("perpendicular")
        );
        assert_eq!(to_value(Arrangement::Freeform).unwrap(), json!("freeform"));
        assert_eq!(to_value(Arrangement::Fixed).unwrap(), json!("fixed"));
    }

    #[test]
    fn test_serialize_line() {
        let line = Line::new()
            .color_array(vec![NamedColor::Black, NamedColor::Blue])
            .color(NamedColor::Black)
            .width(0.1);
        let expected = json!({
            "color": "black",
            "width": 0.1
        });

        assert_eq!(to_value(line).unwrap(), expected)
    }

    #[test]
    fn test_serialize_node() {
        let node = Node::new()
            .color(NamedColor::Blue)
            .color_array(vec![NamedColor::Blue])
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("template")
            .line(Line::new())
            .pad(5)
            .thickness(10)
            .x(0.5)
            .y(0.25);
        let expected = json!({
            "color": ["blue"],
            "hoverinfo": "all",
            "hoverlabel": {},
            "hovertemplate": "template",
            "line": {},
            "pad": 5,
            "thickness": 10,
            "x": 0.5,
            "y": 0.25
        });

        assert_eq!(to_value(node).unwrap(), expected)
    }

    #[test]
    fn test_serialize_link() {
        let link = Link::new()
            .color_array(vec![NamedColor::Blue])
            .color(NamedColor::Blue)
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("template")
            .line(Line::new())
            .value(vec![2, 2, 2])
            .source(vec![0, 1, 2])
            .target(vec![1, 2, 0]);
        let expected = json!({
            "color": "blue",
            "hoverinfo": "all",
            "hoverlabel": {},
            "hovertemplate": "template",
            "line": {},
            "source": [0, 1, 2],
            "target": [1, 2, 0],
            "value": [2, 2, 2],
        });

        assert_eq!(to_value(link).unwrap(), expected)
    }
}
