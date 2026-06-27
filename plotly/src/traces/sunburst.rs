//! Sunburst trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::private::{NumOrString, NumOrStringCollection};
use crate::traces::treemap::BranchValues;
use crate::{
    common::{Dim, Domain, Font, HoverInfo, Label, Marker, Orientation, PlotType},
    Trace,
};

/// Configures the appearance of the leaf nodes of a [`Sunburst`].
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Leaf {
    /// Sets the opacity of the leaves. With colorscale it is defaulted to `1`;
    /// otherwise it is defaulted to `0.7`.
    opacity: Option<f64>,
}

impl Leaf {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a Sunburst trace.
///
/// Sunburst charts visualize hierarchical data spanning outwards radially from
/// the root to the leaves. The hierarchy is defined via the `labels` and
/// `parents` fields (the root is the item whose parent is an empty string).
///
/// # Examples
///
/// ```
/// use plotly::Sunburst;
///
/// let trace = Sunburst::new(
///     vec!["Eve", "Cain", "Seth"],
///     vec!["", "Eve", "Eve"],
/// )
/// .values(vec![10, 14, 12]);
///
/// let expected = serde_json::json!({
///     "type": "sunburst",
///     "labels": ["Eve", "Cain", "Seth"],
///     "parents": ["", "Eve", "Eve"],
///     "values": [10, 14, 12],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Sunburst<V>
where
    V: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Sunburst")]
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
    /// Sets the styling of the leaves of the sunburst.
    leaf: Option<Leaf>,
    /// Rotates the whole diagram counterclockwise by some angle. By default the
    /// first slice starts at 3 o'clock. The 'rotation' property is a number
    /// between -360 and 360.
    rotation: Option<f64>,
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
    /// Sets the font used for `text_info` lying inside the sector.
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    /// Sets the font used for `text_info` lying outside the sector.
    #[serde(rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    /// Controls the orientation of the text inside chart sectors.
    #[serde(rename = "insidetextorientation")]
    inside_text_orientation: Option<Orientation>,
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

impl<V> Sunburst<V>
where
    V: Serialize + Clone + 'static,
{
    /// Build a new Sunburst trace from its `labels` and `parents`.
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

impl<V> Trace for Sunburst<V>
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
    fn serialize_leaf() {
        let leaf = Leaf::new().opacity(0.7);
        assert_eq!(to_value(leaf).unwrap(), json!({ "opacity": 0.7 }));
    }

    #[test]
    fn serialize_sunburst() {
        let trace = Sunburst::new(vec!["Eve", "Cain", "Seth"], vec!["", "Eve", "Eve"])
            .values(vec![10, 14, 12])
            .branch_values(BranchValues::Remainder)
            .count("leaves")
            .level("Eve")
            .max_depth(2)
            .name("family")
            .visible(true)
            .opacity(0.8)
            .ids(vec!["eve", "cain", "seth"])
            .domain(Domain::new())
            .marker(Marker::new())
            .leaf(Leaf::new().opacity(0.6))
            .rotation(45.0)
            .sort(true)
            .text_info("label+value")
            .text_font(Font::new())
            .inside_text_font(Font::new())
            .outside_text_font(Font::new())
            .inside_text_orientation(Orientation::Radial)
            .hover_info(HoverInfo::All)
            .hover_label(Label::new())
            .hover_template("%{label}: %{value}")
            .meta("meta")
            .custom_data(vec!["custom_data"])
            .uid("uid-1");
        let expected = json!({
            "type": "sunburst",
            "labels": ["Eve", "Cain", "Seth"],
            "parents": ["", "Eve", "Eve"],
            "values": [10, 14, 12],
            "branchvalues": "remainder",
            "count": "leaves",
            "level": "Eve",
            "maxdepth": 2,
            "name": "family",
            "visible": true,
            "opacity": 0.8,
            "ids": ["eve", "cain", "seth"],
            "domain": {},
            "marker": {},
            "leaf": {"opacity": 0.6},
            "rotation": 45.0,
            "sort": true,
            "textinfo": "label+value",
            "textfont": {},
            "insidetextfont": {},
            "outsidetextfont": {},
            "insidetextorientation": "r",
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
