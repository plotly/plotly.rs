//! Indicator trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::Color;
use crate::private::{NumOrString, NumOrStringCollection};
use crate::{
    common::{Domain, Font, Line, PlotType},
    Trace,
};

/// Determines how the value is displayed: as a `number`, a `delta` relative to
/// a reference, a `gauge`, or any combination of the three.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Number,
    Delta,
    Gauge,
    #[serde(rename = "number+delta")]
    NumberAndDelta,
    #[serde(rename = "number+gauge")]
    NumberAndGauge,
    #[serde(rename = "delta+gauge")]
    DeltaAndGauge,
    #[serde(rename = "number+delta+gauge")]
    NumberAndDeltaAndGauge,
}

/// Horizontal alignment of the number and delta within their container.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Left,
    Center,
    Right,
}

/// Sets the position of the delta relative to the number.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DeltaPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// Sets the shape of the gauge.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GaugeShape {
    Angular,
    Bullet,
}

/// Sets the title of the indicator.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct IndicatorTitle {
    /// Sets the title text.
    text: Option<String>,
    /// Sets the horizontal alignment of the title.
    align: Option<Align>,
    /// Sets the font used to display the title.
    font: Option<Font>,
}

impl IndicatorTitle {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configures how the displayed number is formatted.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Number {
    /// Sets the value formatting rule using d3 formatting mini-languages.
    #[serde(rename = "valueformat")]
    value_format: Option<String>,
    /// Sets a prefix appearing before the number.
    prefix: Option<String>,
    /// Sets a suffix appearing after the number.
    suffix: Option<String>,
    /// Sets the font used to display the number.
    font: Option<Font>,
}

impl Number {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configures the styling of an increasing or decreasing delta.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Direction {
    /// Sets the symbol to display for this delta direction.
    symbol: Option<String>,
    /// Sets the color for this delta direction.
    #[serde(rename = "color")]
    #[field_setter(skip)]
    color: Option<Box<dyn Color>>,
}

impl Direction {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the color for this delta direction.
    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }
}

/// Configures the delta (change relative to a reference value).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Delta {
    /// Sets the reference value against which the delta is computed.
    reference: Option<f64>,
    /// Shows the delta as a percentage of the reference if `true`.
    relative: Option<bool>,
    /// Sets the value formatting rule using d3 formatting mini-languages.
    #[serde(rename = "valueformat")]
    value_format: Option<String>,
    /// Sets the position of the delta relative to the number.
    position: Option<DeltaPosition>,
    /// Sets a prefix appearing before the delta.
    prefix: Option<String>,
    /// Sets a suffix appearing after the delta.
    suffix: Option<String>,
    /// Sets the font used to display the delta.
    font: Option<Font>,
    /// Sets the styling used when the delta is increasing.
    increasing: Option<Direction>,
    /// Sets the styling used when the delta is decreasing.
    decreasing: Option<Direction>,
}

impl Delta {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configures the axis of a gauge (its range and ticks).
///
/// This is a minimal subset of a full Cartesian axis, exposing only the
/// attributes relevant to gauges.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct GaugeAxis {
    /// Sets the range of the gauge axis.
    range: Option<[f64; 2]>,
    /// Determines whether or not the gauge axis is visible.
    visible: Option<bool>,
    /// Sets the tick mode of the gauge axis (e.g. `"auto"`, `"linear"`,
    /// `"array"`).
    #[serde(rename = "tickmode")]
    tick_mode: Option<String>,
    /// Sets the number of ticks.
    #[serde(rename = "nticks")]
    n_ticks: Option<usize>,
    /// Sets the values at which ticks are drawn (with `tick_mode` `"array"`).
    #[serde(rename = "tickvals")]
    tick_vals: Option<Vec<f64>>,
    /// Sets the text displayed at the tick values (with `tick_mode` `"array"`).
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
}

impl GaugeAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configures the value bar drawn inside a gauge.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct GaugeBar {
    /// Sets the background color of the value bar.
    #[serde(rename = "color")]
    #[field_setter(skip)]
    color: Option<Box<dyn Color>>,
    /// Sets the outline of the value bar.
    line: Option<Line>,
    /// Sets the thickness of the value bar relative to the gauge (0 to 1).
    thickness: Option<f64>,
}

impl GaugeBar {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the background color of the value bar.
    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }
}

/// Configures a colored step (band) drawn along the gauge axis.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Step {
    /// Sets the range this step covers along the gauge axis.
    range: Option<[f64; 2]>,
    /// Sets the color of this step.
    #[serde(rename = "color")]
    #[field_setter(skip)]
    color: Option<Box<dyn Color>>,
    /// Sets the outline of this step.
    line: Option<Line>,
    /// Sets the thickness of this step relative to the gauge (0 to 1).
    thickness: Option<f64>,
}

impl Step {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the color of this step.
    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }
}

/// Configures a threshold marker drawn across the gauge.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Threshold {
    /// Sets the line drawn at the threshold value.
    line: Option<Line>,
    /// Sets the thickness of the threshold line relative to the gauge (0 to 1).
    thickness: Option<f64>,
    /// Sets the value at which the threshold is drawn.
    value: Option<f64>,
}

impl Threshold {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configures the gauge (the angular or bullet visualization).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Gauge {
    /// Sets the shape of the gauge.
    shape: Option<GaugeShape>,
    /// Sets the gauge axis (range and ticks).
    axis: Option<GaugeAxis>,
    /// Sets the value bar drawn inside the gauge.
    bar: Option<GaugeBar>,
    /// Sets the gauge background color.
    #[serde(rename = "bgcolor")]
    #[field_setter(skip)]
    background_color: Option<Box<dyn Color>>,
    /// Sets the color of the gauge border.
    #[serde(rename = "bordercolor")]
    #[field_setter(skip)]
    border_color: Option<Box<dyn Color>>,
    /// Sets the width (in px) of the gauge border.
    #[serde(rename = "borderwidth")]
    border_width: Option<f64>,
    /// Sets the colored steps (bands) along the gauge axis.
    steps: Option<Vec<Step>>,
    /// Sets the threshold marker.
    threshold: Option<Threshold>,
}

impl Gauge {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the gauge background color.
    pub fn background_color<C: Color>(mut self, color: C) -> Self {
        self.background_color = Some(Box::new(color));
        self
    }

    /// Sets the color of the gauge border.
    pub fn border_color<C: Color>(mut self, color: C) -> Self {
        self.border_color = Some(Box::new(color));
        self
    }
}

/// Construct an Indicator trace.
///
/// Indicators display a single `value`, optionally with a `delta` relative to a
/// reference and/or a `gauge`. They are commonly used for KPI tiles.
///
/// # Examples
///
/// ```
/// use plotly::Indicator;
/// use plotly::traces::indicator::{Delta, Mode};
///
/// let trace = Indicator::new(120.0)
///     .mode(Mode::NumberAndDelta)
///     .delta(Delta::new().reference(100.0));
///
/// let expected = serde_json::json!({
///     "type": "indicator",
///     "value": 120.0,
///     "mode": "number+delta",
///     "delta": {"reference": 100.0},
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Indicator {
    #[field_setter(default = "PlotType::Indicator")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appears as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible.
    visible: Option<bool>,
    /// Determines how the value is displayed (number, delta, gauge or a
    /// combination).
    mode: Option<Mode>,
    /// Sets the number to be displayed.
    value: Option<f64>,
    /// Sets the horizontal alignment of the number and delta.
    align: Option<Align>,
    /// Sets the domain within which this trace is drawn.
    domain: Option<Domain>,
    /// Sets the title of the indicator.
    title: Option<IndicatorTitle>,
    /// Configures how the displayed number is formatted.
    number: Option<Number>,
    /// Configures the delta relative to a reference value.
    delta: Option<Delta>,
    /// Configures the gauge visualization.
    gauge: Option<Gauge>,
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
    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"`
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Sets the width (in px or fraction) of the legend for this trace.
    #[serde(rename = "legendwidth")]
    legend_width: Option<f64>,
    /// Controls persistence of user-driven changes to the trace. Defaults to
    /// `layout.uirevision`.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl Indicator {
    /// Build a new Indicator trace displaying `value`.
    pub fn new(value: f64) -> Box<Self> {
        Box::new(Self {
            value: Some(value),
            ..Default::default()
        })
    }
}

impl Trace for Indicator {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_mode() {
        assert_eq!(to_value(Mode::Number).unwrap(), json!("number"));
        assert_eq!(
            to_value(Mode::NumberAndDelta).unwrap(),
            json!("number+delta")
        );
        assert_eq!(
            to_value(Mode::NumberAndDeltaAndGauge).unwrap(),
            json!("number+delta+gauge")
        );
    }

    #[test]
    fn serialize_number_and_delta() {
        let trace = Indicator::new(120.0)
            .mode(Mode::NumberAndDelta)
            .align(Align::Center)
            .title(IndicatorTitle::new().text("Revenue"))
            .number(Number::new().prefix("$").value_format(".2f"))
            .delta(
                Delta::new()
                    .reference(100.0)
                    .relative(true)
                    .position(DeltaPosition::Top)
                    .increasing(Direction::new().color("green").symbol("▲"))
                    .decreasing(Direction::new().color("red").symbol("▼")),
            )
            .domain(Domain::new());

        let expected = json!({
            "type": "indicator",
            "value": 120.0,
            "mode": "number+delta",
            "align": "center",
            "title": {"text": "Revenue"},
            "number": {"prefix": "$", "valueformat": ".2f"},
            "delta": {
                "reference": 100.0,
                "relative": true,
                "position": "top",
                "increasing": {"color": "green", "symbol": "▲"},
                "decreasing": {"color": "red", "symbol": "▼"},
            },
            "domain": {},
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_gauge() {
        let trace = Indicator::new(270.0).mode(Mode::Gauge).gauge(
            Gauge::new()
                .shape(GaugeShape::Angular)
                .axis(GaugeAxis::new().range([0.0, 500.0]))
                .bar(GaugeBar::new().color("darkblue").thickness(0.75))
                .background_color("white")
                .border_color("gray")
                .border_width(1.0)
                .steps(vec![
                    Step::new().range([0.0, 250.0]).color("lightgray"),
                    Step::new().range([250.0, 400.0]).color("gray"),
                ])
                .threshold(
                    Threshold::new()
                        .line(Line::new().color("red").width(4.0))
                        .thickness(0.75)
                        .value(490.0),
                ),
        );

        let expected = json!({
            "type": "indicator",
            "value": 270.0,
            "mode": "gauge",
            "gauge": {
                "shape": "angular",
                "axis": {"range": [0.0, 500.0]},
                "bar": {"color": "darkblue", "thickness": 0.75},
                "bgcolor": "white",
                "bordercolor": "gray",
                "borderwidth": 1.0,
                "steps": [
                    {"range": [0.0, 250.0], "color": "lightgray"},
                    {"range": [250.0, 400.0], "color": "gray"},
                ],
                "threshold": {
                    "line": {"color": "red", "width": 4.0},
                    "thickness": 0.75,
                    "value": 490.0,
                },
            },
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_indicator_legend_and_uirevision() {
        let trace = Indicator::new(1.0)
            .legend_rank(7)
            .legend_width(2.0)
            .ui_revision("rev");
        let v = to_value(trace).unwrap();
        assert_eq!(v["legendrank"], json!(7));
        assert_eq!(v["legendwidth"], json!(2.0));
        assert_eq!(v["uirevision"], json!("rev"));
    }
}
