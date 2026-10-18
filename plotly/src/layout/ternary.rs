use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{DashType, Domain, ExponentFormat, Font, TickFormatStop, TickMode, Ticks, Title},
    layout::{ArrayShow, AxisLayer},
    private::NumOrString,
};

/// One of the three axes (`a`, `b`, `c`) of a ternary subplot.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct TernaryAxis {
    title: Option<Title>,
    /// The minimum value visible on this axis. The maximum is determined by
    /// the sum minus the minimum values of the other two axes.
    min: Option<f64>,
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
    #[serde(rename = "tickmode")]
    tick_mode: Option<TickMode>,
    #[serde(rename = "nticks")]
    n_ticks: Option<usize>,
    #[serde(rename = "tick0")]
    tick0: Option<NumOrString>,
    #[serde(rename = "dtick")]
    dtick: Option<NumOrString>,
    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    ticks: Option<Ticks>,
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
    #[serde(rename = "tickformatstops")]
    tick_format_stops: Option<Vec<TickFormatStop>>,
    #[serde(rename = "ticklabelstep")]
    tick_label_step: Option<usize>,
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
    #[serde(rename = "showexponent")]
    show_exponent: Option<ArrayShow>,
    #[serde(rename = "exponentformat")]
    exponent_format: Option<ExponentFormat>,
    #[serde(rename = "minexponent")]
    min_exponent: Option<f64>,
    #[serde(rename = "separatethousands")]
    separate_thousands: Option<bool>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl TernaryAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The layout of a ternary subplot: a triangle whose three axes `a`, `b` and
/// `c` always add up to [`sum`](LayoutTernary::sum). See
/// [`ScatterTernary`](crate::traces::ScatterTernary) for details on traces.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct LayoutTernary {
    #[serde(rename = "aaxis")]
    a_axis: Option<TernaryAxis>,
    #[serde(rename = "baxis")]
    b_axis: Option<TernaryAxis>,
    #[serde(rename = "caxis")]
    c_axis: Option<TernaryAxis>,
    /// The number each triplet should add up to.
    sum: Option<f64>,
    #[serde(rename = "bgcolor")]
    bg_color: Option<Box<dyn Color>>,
    domain: Option<Domain>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl LayoutTernary {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_ternary_axis() {
        let axis = TernaryAxis::new()
            .title("A")
            .min(0.1)
            .color("#444")
            .show_line(true)
            .line_color("#111")
            .line_width(2.0)
            .show_grid(true)
            .grid_color("#eee")
            .grid_width(1.0)
            .grid_dash(DashType::Dot)
            .layer(AxisLayer::Below)
            .tick_mode(TickMode::Linear)
            .n_ticks(5)
            .tick0(0)
            .dtick(0.2)
            .tick_values(vec![0.5])
            .tick_text(vec!["half"])
            .ticks(Ticks::Outside)
            .tick_length(4.0)
            .tick_width(1.0)
            .tick_color("#222")
            .tick_angle(45.0)
            .tick_font(Font::new())
            .tick_format(".0%")
            .tick_format_stops(vec![TickFormatStop::new()])
            .tick_label_step(2)
            .tick_prefix("p")
            .show_tick_prefix(ArrayShow::All)
            .tick_suffix("s")
            .show_tick_suffix(ArrayShow::Last)
            .show_tick_labels(true)
            .label_alias("alias")
            .show_exponent(ArrayShow::None)
            .exponent_format(ExponentFormat::Power)
            .min_exponent(3.0)
            .separate_thousands(true)
            .hover_format(".2f")
            .ui_revision(1);

        let expected = json!({
            "title": {"text": "A"},
            "min": 0.1,
            "color": "#444",
            "showline": true,
            "linecolor": "#111",
            "linewidth": 2.0,
            "showgrid": true,
            "gridcolor": "#eee",
            "gridwidth": 1.0,
            "griddash": "dot",
            "layer": "below traces",
            "tickmode": "linear",
            "nticks": 5,
            "tick0": 0,
            "dtick": 0.2,
            "tickvals": [0.5],
            "ticktext": ["half"],
            "ticks": "outside",
            "ticklen": 4.0,
            "tickwidth": 1.0,
            "tickcolor": "#222",
            "tickangle": 45.0,
            "tickfont": {},
            "tickformat": ".0%",
            "tickformatstops": [{"enabled": true}],
            "ticklabelstep": 2,
            "tickprefix": "p",
            "showtickprefix": "all",
            "ticksuffix": "s",
            "showticksuffix": "last",
            "showticklabels": true,
            "labelalias": "alias",
            "showexponent": "none",
            "exponentformat": "power",
            "minexponent": 3.0,
            "separatethousands": true,
            "hoverformat": ".2f",
            "uirevision": 1,
        });

        assert_eq!(to_value(axis).unwrap(), expected);
    }

    #[test]
    fn serialize_layout_ternary() {
        let layout = LayoutTernary::new()
            .a_axis(TernaryAxis::new().min(0.0))
            .b_axis(TernaryAxis::new())
            .c_axis(TernaryAxis::new())
            .sum(100.0)
            .bg_color("#fafafa")
            .domain(Domain::new().x(&[0.0, 0.5]))
            .ui_revision("r");

        let expected = json!({
            "aaxis": {"min": 0.0},
            "baxis": {},
            "caxis": {},
            "sum": 100.0,
            "bgcolor": "#fafafa",
            "domain": {"x": [0.0, 0.5]},
            "uirevision": "r",
        });

        assert_eq!(to_value(layout).unwrap(), expected);
    }
}
