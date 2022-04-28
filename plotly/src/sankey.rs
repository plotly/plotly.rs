//! Sankey plot

use serde::Serialize;

use crate::common::color::{Color, ColorWrapper};
use crate::common::{Dim, Domain, Font, HoverInfo, Label, LegendGroupTitle, Orientation, PlotType};
use crate::{private, Trace};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Arrangement {
    Snap,
    Perpendicular,
    Freeform,
    Fixed,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Line {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Dim<ColorWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(color.to_color()));
        self
    }

    pub fn color_array<C: Color>(mut self, color: Vec<C>) -> Self {
        let color = private::to_color_array(color);
        self.color = Some(Dim::Vector(color));
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct Node {
    // Missing: customdata, groups
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Dim<ColorWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pad: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thickness: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(color.to_color()));
        self
    }

    pub fn color_array<C: Color>(mut self, color: Vec<C>) -> Self {
        let color = private::to_color_array(color);
        self.color = Some(Dim::Vector(color));
        self
    }

    pub fn label(mut self, label: Vec<&str>) -> Self {
        self.label = Some(label.iter().map(|&el| el.to_string()).collect());
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Self {
        self.hover_info = Some(hover_info);
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

#[derive(Serialize, Debug, Default, Clone)]
pub struct Link<V>
where
    V: Serialize + Default + Clone,
{
    // Missing: colorscales, customdata
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Dim<ColorWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<V>>,
}

impl<V> Link<V>
where
    V: Serialize + Default + Clone,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(color.to_color()));
        self
    }

    pub fn color_array<C: Color>(mut self, color: Vec<C>) -> Self {
        let color = private::to_color_array(color);
        self.color = Some(Dim::Vector(color));
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Self {
        self.hover_info = Some(hover_info);
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

#[derive(Serialize, Debug, Default, Clone)]
pub struct Sankey<V>
where
    V: Serialize + Default + Clone,
{
    // Missing: meta, customdata, uirevision
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Domain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<Link<V>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textfont")]
    text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectedpoints")]
    selected_points: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arrangement: Option<Arrangement>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "valueformat")]
    value_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "valuesuffix")]
    value_suffix: Option<String>,
}

impl<V> Sankey<V>
where
    V: Serialize + Default + Clone,
{
    /// Creates a new empty Sankey diagram.
    pub fn new() -> Box<Self> {
        Box::new(Self {
            r#type: PlotType::Sankey,
            ..Default::default()
        })
    }

    /// Sets the trace name. The trace name appears as the legend item and on hover.
    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
        Box::new(self)
    }

    /// Determines whether or not this trace is visible. If "legendonly", the trace
    /// is not drawn, but can appear as a legend item (provided that the legend itself is visible).
    pub fn visible(mut self, visible: bool) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    /// Sets the legend rank for this trace. Items and groups with smaller ranks are presented on top/left
    /// side while with `"reversed" `legend.trace_order` they are on bottom/right side. The default legendrank
    /// is 1000, so that you can use ranks less than 1000 to place certain items before all unranked items,
    /// and ranks greater than 1000 to go after all unranked items.
    pub fn legend_rank(mut self, legend_rank: usize) -> Box<Self> {
        self.legend_rank = Some(legend_rank);
        Box::new(self)
    }

    /// Set and style the title to appear for the legend group
    pub fn legend_group_title(mut self, legend_group_title: LegendGroupTitle) -> Box<Self> {
        self.legend_group_title = Some(legend_group_title);
        Box::new(self)
    }

    /// Assigns id labels to each datum. These ids are for object constancy of data points during animation.
    pub fn ids(mut self, ids: Vec<&str>) -> Box<Self> {
        self.ids = Some(ids.iter().map(|&id| id.to_string()).collect());
        Box::new(self)
    }

    /// Determines which trace information appear on hover. If `none` or `skip` are set, no information is displayed
    /// upon hovering. But, if `none` is set, click and hover events are still fired. Note that this attribute
    /// is superseded by `node.hover_info` and `link.hover_info` for nodes and links respectively.
    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    /// Sets the hover label for this trace.
    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    /// Sets the font for node labels.
    pub fn text_font(mut self, text_font: Font) -> Box<Self> {
        self.text_font = Some(text_font);
        Box::new(self)
    }

    /// Sets the domain within which the Sankey diagram will be drawn.
    pub fn domain(mut self, domain: Domain) -> Box<Self> {
        self.domain = Some(domain);
        Box::new(self)
    }

    /// Sets the orientation of the Sankey diagram.
    pub fn orientation(mut self, orientation: Orientation) -> Box<Self> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    /// The nodes of the Sankey diagram.
    pub fn node(mut self, node: Node) -> Box<Self> {
        self.node = Some(node);
        Box::new(self)
    }

    /// The links of the Sankey diagram.
    pub fn link(mut self, link: Link<V>) -> Box<Self> {
        self.link = Some(link);
        Box::new(self)
    }

    /// Vector containing integer indices of selected points. Has an effect only for traces that support
    /// selections. Note that an empty vector means an empty selection where the `unselected` are turned
    /// on for all points.
    pub fn selected_points(mut self, selected_points: Vec<usize>) -> Box<Self> {
        self.selected_points = Some(selected_points);
        Box::new(self)
    }

    /// If value is `snap` (the default), the node arrangement is assisted by automatic snapping of elements
    /// to preserve space between nodes specified via `nodepad`. If value is `perpendicular`, the nodes can
    /// only move along a line perpendicular to the flow. If value is `freeform`, the nodes can freely move
    /// on the plane. If value is `fixed`, the nodes are stationary.
    pub fn arrangement(mut self, arrangement: Arrangement) -> Box<Self> {
        self.arrangement = Some(arrangement);
        Box::new(self)
    }

    /// Sets the value formatting rule using d3 formatting mini-languages which are very similar to those in
    /// Python. For numbers, see: https://github.com/d3/d3-format/tree/v1.4.5#d3-format.
    pub fn value_format(mut self, value_format: &str) -> Box<Self> {
        self.value_format = Some(value_format.to_string());
        Box::new(self)
    }

    /// Adds a unit to follow the value in the hover tooltip. Add a space if a separation is necessary from the value.
    pub fn value_suffix(mut self, value_suffix: &str) -> Box<Self> {
        self.value_suffix = Some(value_suffix.to_string());
        Box::new(self)
    }
}

impl<V> Trace for Sankey<V>
where
    V: Serialize + Default + Clone,
{
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::NamedColor;

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
