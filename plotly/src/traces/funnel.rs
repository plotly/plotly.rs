//! Funnel trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        ConstrainText, Dim, Font, Label, LegendGroupTitle, Line, Marker, Orientation, PlotType,
        TextAnchor, TextPosition, Visible, XAxisId, YAxisId,
    },
    Trace,
};

/// Determines which trace information appears on hover for a funnel trace.
///
/// Unlike the generic [`HoverInfo`](crate::common::HoverInfo), the funnel
/// schema has no `z` flag and adds the funnel-specific `percent initial`,
/// `percent previous` and `percent total` flags. plotly.js accepts any
/// `+`-joined combination of these; the variants below cover the flags
/// individually plus the combinations that are useful in practice.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FunnelHoverInfo {
    Name,
    X,
    Y,
    Text,
    #[serde(rename = "x+y")]
    XAndY,
    #[serde(rename = "x+y+text")]
    XAndYAndText,
    #[serde(rename = "percent initial")]
    PercentInitial,
    #[serde(rename = "percent previous")]
    PercentPrevious,
    #[serde(rename = "percent total")]
    PercentTotal,
    #[serde(rename = "percent initial+percent previous+percent total")]
    AllPercents,
    All,
    None,
    Skip,
}

/// Visually connects consecutive funnel stages.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Connector {
    visible: Option<bool>,
    line: Option<Line>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
}

impl Connector {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Construct a funnel trace.
///
/// # Examples
///
/// ```
/// use plotly::Funnel;
///
/// let x = vec![100, 60, 40];
/// let y = vec!["Visits", "Signups", "Purchases"];
///
/// let trace = Funnel::new(x, y)
///     .orientation(plotly::common::Orientation::Horizontal)
///     .text_info("value+percent previous");
///
/// let expected = serde_json::json!({
///     "type": "funnel",
///     "x": [100, 60, 40],
///     "y": ["Visits", "Signups", "Purchases"],
///     "orientation": "h",
///     "textinfo": "value+percent previous"
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Funnel<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Funnel")]
    r#type: PlotType,
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    opacity: Option<f64>,
    ids: Option<Vec<String>>,
    /// Sets the bar width (in position axis units). Unlike `Bar`, the funnel
    /// trace does not accept a per-point array here.
    width: Option<f64>,
    /// Shifts the position where the bar is drawn (in position axis units).
    /// Unlike `Bar`, the funnel trace does not accept a per-point array here.
    offset: Option<f64>,
    orientation: Option<Orientation>,
    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
    /// Determines which trace information appears on the graph. Plotly.js
    /// expects a `+`-joined flaglist built from any of `label`, `text`,
    /// `percent initial`, `percent previous`, `percent total` and `value`, or
    /// the single value `none`.
    #[serde(rename = "textinfo")]
    text_info: Option<String>,
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(rename = "texttemplatefallback")]
    text_template_fallback: Option<Dim<String>>,
    #[serde(rename = "textangle")]
    text_angle: Option<f64>,
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    #[serde(rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    #[serde(rename = "insidetextanchor")]
    inside_text_anchor: Option<TextAnchor>,
    #[serde(rename = "constraintext")]
    constrain_text: Option<ConstrainText>,
    #[serde(rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<FunnelHoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    marker: Option<Marker>,
    connector: Option<Connector>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    #[serde(rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(rename = "offsetgroup")]
    offset_group: Option<String>,
}

impl<X, Y> Funnel<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Funnel {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> Trace for Funnel<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::DashType;

    #[test]
    fn default_funnel() {
        let trace: Funnel<i32, i32> = Funnel::default();
        let expected = json!({"type": "funnel"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn serialize_connector() {
        let connector = Connector::new()
            .visible(true)
            .line(Line::new().width(2.0).dash(DashType::Dot))
            .fill_color("#aabbcc");

        let expected = json!({
            "visible": true,
            "line": {"width": 2.0, "dash": "dot"},
            "fillcolor": "#aabbcc",
        });

        assert_eq!(to_value(connector).unwrap(), expected);
    }

    #[test]
    fn serialize_funnel_hover_info() {
        assert_eq!(to_value(FunnelHoverInfo::Name).unwrap(), json!("name"));
        assert_eq!(to_value(FunnelHoverInfo::XAndY).unwrap(), json!("x+y"));
        assert_eq!(
            to_value(FunnelHoverInfo::PercentInitial).unwrap(),
            json!("percent initial")
        );
        assert_eq!(
            to_value(FunnelHoverInfo::PercentPrevious).unwrap(),
            json!("percent previous")
        );
        assert_eq!(
            to_value(FunnelHoverInfo::AllPercents).unwrap(),
            json!("percent initial+percent previous+percent total")
        );
        assert_eq!(to_value(FunnelHoverInfo::Skip).unwrap(), json!("skip"));
    }

    #[test]
    fn serialize_funnel_with_percent_hover_info() {
        let funnel =
            Funnel::new(vec![100, 60], vec!["a", "b"]).hover_info(FunnelHoverInfo::PercentPrevious);

        let expected = json!({
            "type": "funnel",
            "x": [100, 60],
            "y": ["a", "b"],
            "hoverinfo": "percent previous",
        });

        assert_eq!(to_value(funnel).unwrap(), expected);
    }

    #[test]
    fn serialize_funnel() {
        let funnel = Funnel::new(vec![1, 2], vec![3, 4])
            .alignment_group("alignment_group")
            .clip_on_axis(true)
            .connector(Connector::new().visible(true))
            .constrain_text(ConstrainText::Both)
            .hover_info(FunnelHoverInfo::All)
            .hover_label(Label::new())
            .hover_template("tmpl")
            .hover_template_array(vec!["tmpl1", "tmpl2"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .ids(vec!["1"])
            .inside_text_anchor(TextAnchor::End)
            .inside_text_font(Font::new())
            .legend_group("legend-group")
            .legend_group_title("legend-group-title")
            .marker(Marker::new())
            .name("Funnel")
            .offset(5.0)
            .offset_group("offset_group")
            .opacity(0.5)
            .orientation(Orientation::Horizontal)
            .outside_text_font(Font::new())
            .show_legend(false)
            .text("text")
            .text_angle(0.05)
            .text_array(vec!["text"])
            .text_font(Font::new())
            .text_info("value+percent previous")
            .text_position(TextPosition::Inside)
            .text_position_array(vec![TextPosition::Inside])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .visible(Visible::LegendOnly)
            .width(999.0)
            .x_axis("x3")
            .y_axis("y3");

        let expected = json!({
            "type": "funnel",
            "x": [1, 2],
            "y": [3, 4],
            "name": "Funnel",
            "visible": "legendonly",
            "showlegend": false,
            "legendgroup": "legend-group",
            "legendgrouptitle": {"text": "legend-group-title"},
            "opacity": 0.5,
            "ids": ["1"],
            "width": 999.0,
            "offset": 5.0,
            "orientation": "h",
            "text": ["text"],
            "textposition": ["inside"],
            "textinfo": "value+percent previous",
            "texttemplate": ["text_template"],
            "textangle": 0.05,
            "textfont": {},
            "insidetextfont": {},
            "outsidetextfont": {},
            "insidetextanchor": "end",
            "constraintext": "both",
            "cliponaxis": true,
            "hovertext": ["hover_text"],
            "hoverinfo": "all",
            "hovertemplate": ["tmpl1", "tmpl2"],
            "hoverlabel": {},
            "marker": {},
            "connector": {"visible": true},
            "xaxis": "x3",
            "yaxis": "y3",
            "alignmentgroup": "alignment_group",
            "offsetgroup": "offset_group",
        });

        assert_eq!(to_value(funnel).unwrap(), expected);
    }

    #[test]
    fn serialize_horizontal_funnel_in_subplot() {
        let funnel = Funnel::new(
            vec![1234567890.0, 800000000.0, 500000000.0],
            vec!["Spent", "Committed", "Planned"],
        )
        .orientation(Orientation::Horizontal)
        .name("Pipeline funnel: totalplannedcommit -> commit_total -> spent_total")
        .text_info("value+percent previous")
        .text_position(TextPosition::Inside)
        .hover_text_array(vec!["a", "b", "c"])
        .hover_template("%{hovertext}<extra></extra>")
        .marker(Marker::new().color_array(vec!["#111111", "#222222", "#333333"]))
        .connector(Connector::new().visible(true))
        .x_axis("x3")
        .y_axis("y3");

        let expected = json!({
            "type": "funnel",
            "x": [1234567890.0, 800000000.0, 500000000.0],
            "y": ["Spent", "Committed", "Planned"],
            "orientation": "h",
            "name": "Pipeline funnel: totalplannedcommit -> commit_total -> spent_total",
            "textinfo": "value+percent previous",
            "textposition": "inside",
            "hovertext": ["a", "b", "c"],
            "hovertemplate": "%{hovertext}<extra></extra>",
            "marker": {"color": ["#111111", "#222222", "#333333"]},
            "connector": {"visible": true},
            "xaxis": "x3",
            "yaxis": "y3",
        });

        assert_eq!(to_value(funnel).unwrap(), expected);
    }
}
