//! Funnel area trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::private::{NumOrString, NumOrStringCollection};
use crate::{
    common::{Dim, Domain, Font, Label, LegendGroupTitle, Marker, PlotType, TextPosition, Visible},
    Trace,
};

#[derive(Serialize, Clone, Debug)]
pub enum FunnelAreaTitlePosition {
    #[serde(rename = "top left")]
    TopLeft,
    #[serde(rename = "top center")]
    TopCenter,
    #[serde(rename = "top right")]
    TopRight,
}

/// Title of a funnel area trace, drawn above the funnel.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct FunnelAreaTitle {
    text: Option<String>,
    font: Option<Font>,
    position: Option<FunnelAreaTitlePosition>,
}

impl FunnelAreaTitle {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<&str> for FunnelAreaTitle {
    fn from(text: &str) -> Self {
        FunnelAreaTitle::new().text(text)
    }
}

impl From<String> for FunnelAreaTitle {
    fn from(text: String) -> Self {
        FunnelAreaTitle::new().text(text)
    }
}

/// Construct a funnel area trace: a pie-like chart where each stage is drawn
/// as a slice of a funnel whose area is proportional to its value.
///
/// # Examples
///
/// ```
/// use plotly::FunnelArea;
///
/// let trace = FunnelArea::new(vec![5, 4, 3])
///     .labels(vec!["The 1st", "The 2nd", "The 3rd"]);
///
/// let expected = serde_json::json!({
///     "type": "funnelarea",
///     "values": [5, 4, 3],
///     "labels": ["The 1st", "The 2nd", "The 3rd"],
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct FunnelArea<V>
where
    V: Serialize + Clone,
{
    #[field_setter(default = "PlotType::FunnelArea")]
    r#type: PlotType,
    /// Sets the values of the sectors. If omitted, occurrences of each label
    /// are counted.
    values: Option<Vec<V>>,
    /// Sets the sector labels. Duplicated labels have their values summed.
    labels: Option<Vec<String>>,
    /// Alternate to `labels`: builds a numeric set of labels starting at
    /// `label0` with step `dlabel`.
    label0: Option<f64>,
    dlabel: Option<f64>,
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
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    meta: Option<NumOrString>,
    domain: Option<Domain>,
    /// Sets the ratio between height and width.
    #[serde(rename = "aspectratio")]
    aspect_ratio: Option<f64>,
    /// Sets the ratio between the bottom length and the maximum top length.
    #[serde(rename = "baseratio")]
    base_ratio: Option<f64>,
    /// Funnel area traces sharing a non-empty scale group are sized relative
    /// to their totals.
    #[serde(rename = "scalegroup")]
    scale_group: Option<String>,
    marker: Option<Marker>,
    title: Option<FunnelAreaTitle>,
    text: Option<Dim<String>>,
    /// Flaglist of `label`, `text`, `value`, `percent` joined with `+`, or
    /// `none`.
    #[serde(rename = "textinfo")]
    text_info: Option<String>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "texttemplatefallback")]
    text_template_fallback: Option<Dim<String>>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Flaglist of `label`, `text`, `value`, `percent`, `name` joined with
    /// `+`, or one of `all`, `none`, `skip`.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<String>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl<V> FunnelArea<V>
where
    V: Serialize + Clone,
{
    pub fn new(values: Vec<V>) -> Box<Self> {
        Box::new(Self {
            values: Some(values),
            ..Default::default()
        })
    }
}

impl<V> Trace for FunnelArea<V>
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
    fn default_funnel_area() {
        let trace: FunnelArea<i32> = FunnelArea::default();
        assert_eq!(trace.to_json(), json!({"type": "funnelarea"}).to_string());
    }

    #[test]
    fn serialize_title_position() {
        assert_eq!(
            to_value(FunnelAreaTitlePosition::TopLeft).unwrap(),
            json!("top left")
        );
        assert_eq!(
            to_value(FunnelAreaTitlePosition::TopCenter).unwrap(),
            json!("top center")
        );
        assert_eq!(
            to_value(FunnelAreaTitlePosition::TopRight).unwrap(),
            json!("top right")
        );
    }

    #[test]
    fn serialize_funnel_area() {
        let trace = FunnelArea::new(vec![5, 4, 3])
            .labels(vec!["a", "b", "c"])
            .label0(1.0)
            .dlabel(2.0)
            .name("funnelarea")
            .visible(Visible::LegendOnly)
            .show_legend(false)
            .legend_group("group")
            .legend_group_title("Group")
            .legend_rank(10)
            .legend_width(20.0)
            .opacity(0.5)
            .ids(vec!["i1", "i2", "i3"])
            .custom_data(vec!["c1"])
            .meta("meta")
            .domain(Domain::new().column(1))
            .aspect_ratio(1.5)
            .base_ratio(0.2)
            .scale_group("sg")
            .marker(Marker::new())
            .title(
                FunnelAreaTitle::new()
                    .text("Title")
                    .position(FunnelAreaTitlePosition::TopLeft),
            )
            .text_array(vec!["t1", "t2", "t3"])
            .text_info("label+percent")
            .text_position(TextPosition::Inside)
            .text_template("%{label}")
            .text_template_fallback("-")
            .text_font(Font::new())
            .inside_text_font(Font::new())
            .hover_text("hover")
            .hover_info("label+value")
            .hover_template("%{value}")
            .hover_template_fallback("-")
            .hover_label(Label::new())
            .ui_revision(1);

        let expected = json!({
            "type": "funnelarea",
            "values": [5, 4, 3],
            "labels": ["a", "b", "c"],
            "label0": 1.0,
            "dlabel": 2.0,
            "name": "funnelarea",
            "visible": "legendonly",
            "showlegend": false,
            "legendgroup": "group",
            "legendgrouptitle": {"text": "Group"},
            "legendrank": 10,
            "legendwidth": 20.0,
            "opacity": 0.5,
            "ids": ["i1", "i2", "i3"],
            "customdata": ["c1"],
            "meta": "meta",
            "domain": {"column": 1},
            "aspectratio": 1.5,
            "baseratio": 0.2,
            "scalegroup": "sg",
            "marker": {},
            "title": {"text": "Title", "position": "top left"},
            "text": ["t1", "t2", "t3"],
            "textinfo": "label+percent",
            "textposition": "inside",
            "texttemplate": "%{label}",
            "texttemplatefallback": "-",
            "textfont": {},
            "insidetextfont": {},
            "hovertext": "hover",
            "hoverinfo": "label+value",
            "hovertemplate": "%{value}",
            "hovertemplatefallback": "-",
            "hoverlabel": {},
            "uirevision": 1,
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
