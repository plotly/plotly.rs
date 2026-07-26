//! Waterfall trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        ConstrainText, Dim, Font, Label, LegendGroupTitle, Line, Orientation, PlotType, TextAnchor,
        TextPosition, Visible, XAxisId, YAxisId,
    },
    private::NumOrString,
    Trace,
};

/// Sets how a waterfall bar relates to the running total.
///
/// Unlike the `textinfo` flaglist, which stays a free-form `+`-joined string,
/// `measure` is a `data_array` whose entries plotly.js validates against a
/// closed set: anything it does not recognise as `absolute` or `total` is
/// silently treated as [`Measure::Relative`]. A typo therefore renders a
/// different chart rather than raising an error, which is what earns this a
/// type.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Measure {
    /// The value is a delta applied to the running total. This is plotly.js's
    /// default for any unrecognised entry.
    Relative,
    /// The bar spans from zero to the running total. The supplied value is
    /// ignored, but the slot must still be present: the value, label and
    /// `measure` arrays are read positionally.
    Total,
    /// The value resets the running total, and the bar spans from zero to it.
    Absolute,
}

/// Determines how the connector lines between waterfall bars are drawn.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorMode {
    /// The connector spans the full extent of the bar it leads into.
    Spanning,
    /// The connector is drawn only in the gap between bars. This is plotly.js's
    /// default.
    Between,
}

/// Visually connects consecutive waterfall bars.
///
/// Unlike [`funnel::Connector`](crate::funnel::Connector), a waterfall
/// connector is a line rather than a filled region: it has no `fillcolor` and
/// gains a [`ConnectorMode`]. Both `visible` (`true`) and `mode` (`between`)
/// default on the plotly.js side, so an unset connector still draws.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Connector {
    line: Option<Line>,
    mode: Option<ConnectorMode>,
    visible: Option<bool>,
}

impl Connector {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Marker styling for one class of waterfall bars.
///
/// plotly.js declares every attribute here `arrayOk: false`, so unlike
/// [`common::Marker`](crate::common::Marker) — whose `color` is a [`Dim`] — a
/// waterfall marker colour is a single value. Per-bar colouring is not
/// available: a bar's colour is selected by its [`Measure`], not by its index.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Marker {
    /// Sets the fill colour for this class of bars.
    color: Option<Box<dyn Color>>,
    /// Sets the bar outline styling.
    line: Option<Line>,
}

impl Marker {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Styling for one class of waterfall bars.
///
/// Which block applies to a bar is decided by its [`Measure`]:
/// [`Measure::Absolute`] and [`Measure::Total`] use `totals`, while
/// [`Measure::Relative`] uses `increasing` for a positive value and
/// `decreasing` for a negative one.
///
/// plotly.js defaults are `#3D9970` for increasing, `#FF4136` for decreasing
/// and `#4499FF` for totals.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct MeasureStyle {
    marker: Option<Marker>,
}

impl MeasureStyle {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Determines which trace information appears on hover for a waterfall trace.
///
/// Unlike the generic [`HoverInfo`](crate::common::HoverInfo), the waterfall
/// schema has no `z` flag and adds the waterfall-specific `initial`, `delta`
/// and `final` flags. plotly.js accepts any `+`-joined combination of these;
/// the variants below cover the flags individually plus the combinations that
/// are useful in practice.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WaterfallHoverInfo {
    Name,
    X,
    Y,
    Text,
    #[serde(rename = "x+y")]
    XAndY,
    #[serde(rename = "x+y+text")]
    XAndYAndText,
    Initial,
    Delta,
    Final,
    #[serde(rename = "initial+delta+final")]
    InitialAndDeltaAndFinal,
    All,
    None,
    Skip,
}

/// Construct a waterfall trace.
///
/// # Examples
///
/// ```
/// use plotly::{waterfall::Measure, Waterfall};
///
/// let x = vec!["Start", "Gain", "Loss", "End"];
/// let y = vec![100.0, 40.0, -25.0, 0.0];
///
/// let trace = Waterfall::new(x, y)
///     .measure(vec![
///         Measure::Absolute,
///         Measure::Relative,
///         Measure::Relative,
///         Measure::Total,
///     ])
///     .text_info("label+delta");
///
/// let expected = serde_json::json!({
///     "type": "waterfall",
///     "x": ["Start", "Gain", "Loss", "End"],
///     "y": [100.0, 40.0, -25.0, 0.0],
///     "measure": ["absolute", "relative", "relative", "total"],
///     "textinfo": "label+delta"
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Waterfall<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    #[field_setter(default = "PlotType::Waterfall")]
    r#type: PlotType,
    x: Option<Vec<X>>,
    /// Alternate to `x`. Builds a linear space of x coordinates. Use with `dx`
    /// where `x0` is the starting coordinate and `dx` the step.
    x0: Option<NumOrString>,
    /// Sets the x coordinate step. See `x0` for more info.
    dx: Option<f64>,
    y: Option<Vec<Y>>,
    /// Alternate to `y`. Builds a linear space of y coordinates. Use with `dy`
    /// where `y0` is the starting coordinate and `dy` the step.
    y0: Option<NumOrString>,
    /// Sets the y coordinate step. See `y0` for more info.
    dy: Option<f64>,
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
    /// Sets how each value relates to the running total, one entry per data
    /// point. plotly.js reads this positionally against the value array and
    /// treats a missing entry as [`Measure::Relative`], so a shorter array
    /// silently renders a different chart rather than raising an error.
    measure: Option<Vec<Measure>>,
    /// Shifts the starting point of the first bar. Unlike `Bar`, plotly.js
    /// declares this `arrayOk: false` for waterfall, so it is a single value.
    base: Option<f64>,
    /// Sets the bar width (in position axis units).
    width: Option<Dim<f64>>,
    /// Shifts the position where the bar is drawn (in position axis units).
    offset: Option<Dim<f64>>,
    orientation: Option<Orientation>,
    text: Option<Dim<String>>,
    #[serde(rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
    /// Determines which trace information appears on the graph. plotly.js
    /// expects a `+`-joined flaglist built from any of `label`, `text`,
    /// `initial`, `delta` and `final`, or the single value `none`.
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
    hover_info: Option<WaterfallHoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertemplatefallback")]
    hover_template_fallback: Option<Dim<String>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Sets the hover text formatting rule for `x` using d3 formatting
    /// mini-languages. Overrides `xaxis.hoverformat`.
    #[serde(rename = "xhoverformat")]
    x_hover_format: Option<String>,
    /// Sets the hover text formatting rule for `y` using d3 formatting
    /// mini-languages. Overrides `yaxis.hoverformat`.
    #[serde(rename = "yhoverformat")]
    y_hover_format: Option<String>,
    /// Styles bars whose measure is [`Measure::Relative`] with a positive
    /// value.
    increasing: Option<MeasureStyle>,
    /// Styles bars whose measure is [`Measure::Relative`] with a negative
    /// value.
    decreasing: Option<MeasureStyle>,
    /// Styles bars whose measure is [`Measure::Absolute`] or
    /// [`Measure::Total`].
    totals: Option<MeasureStyle>,
    connector: Option<Connector>,
    #[serde(rename = "xaxis")]
    x_axis: Option<XAxisId>,
    #[serde(rename = "yaxis")]
    y_axis: Option<YAxisId>,
    #[serde(rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(rename = "offsetgroup")]
    offset_group: Option<String>,
    /// Sets the layer on which this trace is displayed relative to other SVG
    /// traces on the same subplot. A higher `zorder` appears on top.
    #[serde(rename = "zorder")]
    z_order: Option<i32>,
}

impl<X, Y> Waterfall<X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Waterfall {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }
}

impl<X, Y> Trace for Waterfall<X, Y>
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
    fn default_waterfall() {
        let trace: Waterfall<i32, i32> = Waterfall::default();
        let expected = json!({"type": "waterfall"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn serialize_measure() {
        assert_eq!(to_value(Measure::Absolute).unwrap(), json!("absolute"));
        assert_eq!(to_value(Measure::Relative).unwrap(), json!("relative"));
        assert_eq!(to_value(Measure::Total).unwrap(), json!("total"));
    }

    #[test]
    fn serialize_connector() {
        let connector = Connector::new()
            .line(Line::new().width(2.0).dash(DashType::Dot))
            .mode(ConnectorMode::Spanning)
            .visible(true);

        let expected = json!({
            "line": {"width": 2.0, "dash": "dot"},
            "mode": "spanning",
            "visible": true,
        });

        assert_eq!(to_value(connector).unwrap(), expected);
    }

    #[test]
    fn serialize_waterfall_hover_info() {
        assert_eq!(to_value(WaterfallHoverInfo::Name).unwrap(), json!("name"));
        assert_eq!(to_value(WaterfallHoverInfo::XAndY).unwrap(), json!("x+y"));
        assert_eq!(
            to_value(WaterfallHoverInfo::Initial).unwrap(),
            json!("initial")
        );
        assert_eq!(to_value(WaterfallHoverInfo::Delta).unwrap(), json!("delta"));
        assert_eq!(to_value(WaterfallHoverInfo::Final).unwrap(), json!("final"));
        // waterfall's flags are single words, so the combined form carries no
        // internal spaces -- unlike `FunnelHoverInfo::AllPercents`.
        assert_eq!(
            to_value(WaterfallHoverInfo::InitialAndDeltaAndFinal).unwrap(),
            json!("initial+delta+final")
        );
        assert_eq!(to_value(WaterfallHoverInfo::Skip).unwrap(), json!("skip"));
    }

    #[test]
    fn serialize_measure_style() {
        let style = MeasureStyle::new().marker(
            Marker::new()
                .color("#3D9970")
                .line(Line::new().width(1.0).color("#111111")),
        );

        let expected = json!({
            "marker": {
                "color": "#3D9970",
                "line": {"width": 1.0, "color": "#111111"},
            }
        });

        assert_eq!(to_value(style).unwrap(), expected);
    }

    #[test]
    fn serialize_waterfall() {
        let waterfall = Waterfall::new(vec![1, 2], vec![3, 4])
            .alignment_group("alignment_group")
            .base(10.0)
            .clip_on_axis(true)
            .connector(Connector::new().mode(ConnectorMode::Between).visible(true))
            .constrain_text(ConstrainText::Both)
            .decreasing(MeasureStyle::new().marker(Marker::new().color("#FF4136")))
            .dx(1.0)
            .dy(2.0)
            .hover_info(WaterfallHoverInfo::All)
            .hover_label(Label::new())
            .hover_template("tmpl")
            .hover_template_array(vec!["tmpl1", "tmpl2"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .ids(vec!["1"])
            .increasing(MeasureStyle::new().marker(Marker::new().color("#3D9970")))
            .inside_text_anchor(TextAnchor::End)
            .inside_text_font(Font::new())
            .legend_group("legend-group")
            .legend_group_title("legend-group-title")
            .measure(vec![Measure::Absolute, Measure::Relative])
            .name("Waterfall")
            .offset(5.0)
            .offset_array(vec![1.0, 2.0])
            .offset_group("offset_group")
            .opacity(0.5)
            .orientation(Orientation::Horizontal)
            .outside_text_font(Font::new())
            .show_legend(false)
            .text("text")
            .text_angle(0.05)
            .text_array(vec!["text"])
            .text_font(Font::new())
            .text_info("label+delta+final")
            .text_position(TextPosition::Inside)
            .text_position_array(vec![TextPosition::Inside])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .totals(MeasureStyle::new().marker(Marker::new().color("#4499FF")))
            .visible(Visible::LegendOnly)
            .width(999.0)
            .width_array(vec![1.0, 2.0])
            .x0(0)
            .x_axis("x3")
            .x_hover_format("$,.0f")
            .y0(0)
            .y_axis("y3")
            .y_hover_format(".2f")
            .z_order(3);

        let expected = json!({
            "type": "waterfall",
            "x": [1, 2],
            "x0": 0,
            "dx": 1.0,
            "y": [3, 4],
            "y0": 0,
            "dy": 2.0,
            "name": "Waterfall",
            "visible": "legendonly",
            "showlegend": false,
            "legendgroup": "legend-group",
            "legendgrouptitle": {"text": "legend-group-title"},
            "opacity": 0.5,
            "ids": ["1"],
            "measure": ["absolute", "relative"],
            "base": 10.0,
            "width": [1.0, 2.0],
            "offset": [1.0, 2.0],
            "orientation": "h",
            "text": ["text"],
            "textposition": ["inside"],
            "textinfo": "label+delta+final",
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
            "xhoverformat": "$,.0f",
            "yhoverformat": ".2f",
            "increasing": {"marker": {"color": "#3D9970"}},
            "decreasing": {"marker": {"color": "#FF4136"}},
            "totals": {"marker": {"color": "#4499FF"}},
            "connector": {"mode": "between", "visible": true},
            "xaxis": "x3",
            "yaxis": "y3",
            "alignmentgroup": "alignment_group",
            "offsetgroup": "offset_group",
            "zorder": 3,
        });

        assert_eq!(to_value(waterfall).unwrap(), expected);
    }

    #[test]
    fn serialize_horizontal_waterfall_in_subplot() {
        // A budget bridge: three independent totals with the signed difference
        // between each consecutive pair drawn as a floating bar. `Total` bars
        // discard the value plotly.js is handed and re-derive it from the
        // running total, but the slot must still be present because the value,
        // label and `measure` arrays are read positionally.
        let waterfall = Waterfall::new(
            vec![
                191_753_783_000.0,
                -163_374_676_366.76,
                28_379_106_633.24,
                52_599_508_456.01,
                80_978_615_089.25,
            ],
            vec![
                "Planned",
                "Committed - Planned",
                "Committed",
                "Spent - Committed",
                "Spent",
            ],
        )
        .orientation(Orientation::Horizontal)
        .name("Budget bridge: totalplannedcommit -> commit_total -> spent_total")
        .measure(vec![
            Measure::Absolute,
            Measure::Relative,
            Measure::Total,
            Measure::Relative,
            Measure::Total,
        ])
        .text_info("delta")
        .text_position(TextPosition::Outside)
        .hover_text_array(vec!["a", "b", "c", "d", "e"])
        .hover_template("%{hovertext}<extra></extra>")
        .increasing(MeasureStyle::new().marker(Marker::new().color("#2CA02C")))
        .decreasing(MeasureStyle::new().marker(Marker::new().color("#D62728")))
        .totals(MeasureStyle::new().marker(Marker::new().color("#4499FF")))
        .x_axis("x3")
        .y_axis("y3");

        let expected = json!({
            "type": "waterfall",
            "x": [
                191_753_783_000.0,
                -163_374_676_366.76,
                28_379_106_633.24,
                52_599_508_456.01,
                80_978_615_089.25,
            ],
            "y": [
                "Planned",
                "Committed - Planned",
                "Committed",
                "Spent - Committed",
                "Spent",
            ],
            "orientation": "h",
            "name": "Budget bridge: totalplannedcommit -> commit_total -> spent_total",
            "measure": ["absolute", "relative", "total", "relative", "total"],
            "textinfo": "delta",
            "textposition": "outside",
            "hovertext": ["a", "b", "c", "d", "e"],
            "hovertemplate": "%{hovertext}<extra></extra>",
            "increasing": {"marker": {"color": "#2CA02C"}},
            "decreasing": {"marker": {"color": "#D62728"}},
            "totals": {"marker": {"color": "#4499FF"}},
            "xaxis": "x3",
            "yaxis": "y3",
        });

        assert_eq!(to_value(waterfall).unwrap(), expected);
    }
}
