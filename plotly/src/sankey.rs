//! Sankey plot

use serde::Serialize;

use crate::common::color::{Color, ColorWrapper};
use crate::common::{Dim, HoverInfo, Label, Orientation, PlotType};
use crate::{private, Trace};

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
    // Missing: customdata, groups, label
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
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
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
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
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

    pub fn value(mut self, value: Vec<V>) -> Self {
        self.value = Some(value);
        self
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct Sankey<V>
where
    V: Serialize + Default + Clone,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<Link<V>>,
}

impl<V> Sankey<V>
where
    V: Serialize + Default + Clone,
{
    pub fn new() -> Box<Self> {
        Box::new(Self {
            r#type: PlotType::Sankey,
            ..Default::default()
        })
    }

    /// Sets the orientation of the Sankey diagram.
    pub fn orientation(mut self, orientation: Orientation) -> Box<Self> {
        self.orientation = Some(orientation);
        Box::new(self)
    }

    /// The nodes of the Sankey plot.
    pub fn node(mut self, node: Node) -> Box<Self> {
        self.node = Some(node);
        Box::new(self)
    }

    /// The links of the Sankey plot.
    pub fn link(mut self, link: Link<V>) -> Box<Self> {
        self.link = Some(link);
        Box::new(self)
    }
}

impl<V> Trace for Sankey<V>
where
    V: Serialize + Default + Clone,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::NamedColor;

    use super::*;

    #[test]
    fn build_basic_sankey_trace() {
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
                    .source(vec![0, 1, 0, 2, 3, 3])
                    .target(vec![2, 3, 3, 4, 4, 5])
                    .value(vec![8, 4, 2, 8, 4, 2]),
            );

        let expected = r#"{"type":"sankey","orientation":"h","node":{"color":["blue","blue","blue","blue","blue","blue"],"label":["A1","A2","B1","B2","C1","C2"],"line":{"color":"black","width":0.5},"pad":15,"thickness":30},"link":{"source":[0,1,0,2,3,3],"target":[2,3,3,4,4,5],"value":[8,4,2,8,4,2]}}"#;

        assert_eq!(serde_json::to_string(&trace).unwrap(), expected);
    }
}
