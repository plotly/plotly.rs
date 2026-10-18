use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{DashType, Domain, Font, Ticks},
    layout::{ArrayShow, AxisLayer},
};

/// Which side of the real axis its tick labels are drawn on.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RealAxisSide {
    Top,
    Bottom,
}

/// Whether ticks on the real axis of a Smith chart are drawn, and on which
/// side.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RealAxisTicks {
    Top,
    Bottom,
    #[serde(rename = "")]
    None,
}

/// The real (resistance) axis of a Smith chart.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct RealAxis {
    visible: Option<bool>,
    side: Option<RealAxisSide>,
    color: Option<Box<dyn Color>>,
    #[serde(rename = "showline")]
    show_line: Option<bool>,
    #[serde(rename = "linecolor")]
    line_color: Option<Box<dyn Color>>,
    #[serde(rename = "linewidth")]
    line_width: Option<f64>,
    #[serde(rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(rename = "gridcolor")]
    grid_color: Option<Box<dyn Color>>,
    #[serde(rename = "gridwidth")]
    grid_width: Option<f64>,
    #[serde(rename = "griddash")]
    grid_dash: Option<DashType>,
    layer: Option<AxisLayer>,
    /// The values at which ticks (and grid circles) appear.
    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    ticks: Option<RealAxisTicks>,
    #[serde(rename = "ticklen")]
    tick_length: Option<f64>,
    #[serde(rename = "tickwidth")]
    tick_width: Option<f64>,
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    #[serde(rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(rename = "showtickprefix")]
    show_tick_prefix: Option<ArrayShow>,
    #[serde(rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(rename = "showticksuffix")]
    show_tick_suffix: Option<ArrayShow>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(rename = "labelalias")]
    label_alias: Option<String>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
}

impl RealAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The imaginary (reactance) axis of a Smith chart.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct ImaginaryAxis {
    visible: Option<bool>,
    color: Option<Box<dyn Color>>,
    #[serde(rename = "showline")]
    show_line: Option<bool>,
    #[serde(rename = "linecolor")]
    line_color: Option<Box<dyn Color>>,
    #[serde(rename = "linewidth")]
    line_width: Option<f64>,
    #[serde(rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(rename = "gridcolor")]
    grid_color: Option<Box<dyn Color>>,
    #[serde(rename = "gridwidth")]
    grid_width: Option<f64>,
    #[serde(rename = "griddash")]
    grid_dash: Option<DashType>,
    layer: Option<AxisLayer>,
    /// The values at which ticks (and grid arcs) appear.
    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    ticks: Option<Ticks>,
    #[serde(rename = "ticklen")]
    tick_length: Option<f64>,
    #[serde(rename = "tickwidth")]
    tick_width: Option<f64>,
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(rename = "showtickprefix")]
    show_tick_prefix: Option<ArrayShow>,
    #[serde(rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(rename = "showticksuffix")]
    show_tick_suffix: Option<ArrayShow>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(rename = "labelalias")]
    label_alias: Option<String>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
}

impl ImaginaryAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The layout of a Smith chart subplot, used for RF / impedance plots. See
/// [`ScatterSmith`](crate::traces::ScatterSmith) for details on traces.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct LayoutSmith {
    #[serde(rename = "realaxis")]
    real_axis: Option<RealAxis>,
    #[serde(rename = "imaginaryaxis")]
    imaginary_axis: Option<ImaginaryAxis>,
    #[serde(rename = "bgcolor")]
    bg_color: Option<Box<dyn Color>>,
    domain: Option<Domain>,
}

impl LayoutSmith {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_real_axis() {
        let axis = RealAxis::new()
            .visible(true)
            .side(RealAxisSide::Bottom)
            .color("#444")
            .show_line(true)
            .line_color("#111")
            .line_width(1.0)
            .show_grid(false)
            .grid_color("#eee")
            .grid_width(2.0)
            .grid_dash(DashType::Dash)
            .layer(AxisLayer::Above)
            .tick_values(vec![0.2, 0.5, 1.0])
            .ticks(RealAxisTicks::Top)
            .tick_length(5.0)
            .tick_width(2.0)
            .tick_color("#222")
            .tick_angle(90.0)
            .tick_font(Font::new())
            .tick_format(".1f")
            .tick_prefix("p")
            .show_tick_prefix(ArrayShow::First)
            .tick_suffix("ohm")
            .show_tick_suffix(ArrayShow::All)
            .show_tick_labels(true)
            .label_alias("alias")
            .hover_format(".2f");

        let expected = json!({
            "visible": true,
            "side": "bottom",
            "color": "#444",
            "showline": true,
            "linecolor": "#111",
            "linewidth": 1.0,
            "showgrid": false,
            "gridcolor": "#eee",
            "gridwidth": 2.0,
            "griddash": "dash",
            "layer": "above traces",
            "tickvals": [0.2, 0.5, 1.0],
            "ticks": "top",
            "ticklen": 5.0,
            "tickwidth": 2.0,
            "tickcolor": "#222",
            "tickangle": 90.0,
            "tickfont": {},
            "tickformat": ".1f",
            "tickprefix": "p",
            "showtickprefix": "first",
            "ticksuffix": "ohm",
            "showticksuffix": "all",
            "showticklabels": true,
            "labelalias": "alias",
            "hoverformat": ".2f",
        });

        assert_eq!(to_value(axis).unwrap(), expected);
        assert_eq!(to_value(RealAxisTicks::None).unwrap(), json!(""));
    }

    #[test]
    fn serialize_layout_smith() {
        let layout = LayoutSmith::new()
            .real_axis(RealAxis::new())
            .imaginary_axis(
                ImaginaryAxis::new()
                    .ticks(Ticks::Inside)
                    .tick_values(vec![1.0]),
            )
            .bg_color("#fff")
            .domain(Domain::new().row(0));

        let expected = json!({
            "realaxis": {},
            "imaginaryaxis": {"ticks": "inside", "tickvals": [1.0]},
            "bgcolor": "#fff",
            "domain": {"row": 0},
        });

        assert_eq!(to_value(layout).unwrap(), expected);
    }
}
