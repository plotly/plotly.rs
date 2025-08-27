use std::{fmt::Display, num::NonZeroU8};

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{DashType, ExponentFormat, Font, TickFormatStop, TickMode, Ticks, Title},
    layout::{ArrayShow, CategoryOrder, RangeMode},
    private::{NumOrString, NumOrStringCollection},
};

#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct LayoutPolar {
    sector: Option<[f64; 2]>,
    hole: Option<Hole>,
    #[serde(rename = "bgcolor")]
    bg_color: Option<Box<dyn Color>>,
    #[serde(rename = "radialaxis")]
    radial_axis: Option<RadialAxis>,
    #[serde(rename = "angularaxis")]
    angular_axis: Option<AngularAxis>,
    #[serde(rename = "gridshape")]
    grid_shape: Option<GridShape>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

// There is partial overlap with [`Axis`](crate::layout::Axis), for both
// [`PolarAxis`] and [`PolarAxisTicks`] so it may be possible to factor that
// out.
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct PolarAxis {
    color: Option<Box<dyn Color>>,
    #[serde(rename = "showline")]
    show_line: Option<bool>,
    #[serde(rename = "linecolor")]
    line_color: Option<Box<dyn Color>>,
    #[serde(rename = "linewidth")]
    line_width: Option<usize>,
    #[serde(rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(rename = "gridcolor")]
    grid_color: Option<Box<dyn Color>>,
    #[serde(rename = "gridwidth")]
    grid_width: Option<usize>,
    #[serde(rename = "griddash")]
    grid_dash: Option<DashType>,
    #[serde(flatten)]
    ticks: Option<PolarAxisTicks>,
}

#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct PolarAxisTicks {
    #[serde(rename = "tickmode")]
    tick_mode: Option<TickMode>,
    nticks: Option<usize>,
    tick0: Option<NumOrString>,
    dtick: Option<NumOrString>,
    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    ticks: Option<Ticks>,
    #[serde(rename = "ticklen")]
    tick_length: Option<usize>,
    #[serde(rename = "tickwidth")]
    tick_width: Option<usize>,
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    #[serde(rename = "ticklabelstep")]
    tick_label_step: Option<NonZeroU8>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(rename = "labelalias")]
    label_alias: Option<String>,
    #[serde(rename = "minorloglabels")]
    minor_log_labels: Option<MinorLogLabels>,
    #[serde(rename = "showtickprefix")]
    show_tick_prefix: Option<ArrayShow>,
    #[serde(rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(rename = "showticksuffix")]
    show_tick_suffix: Option<ArrayShow>,
    #[serde(rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(rename = "showexponent")]
    show_exponent: Option<ArrayShow>,
    #[serde(rename = "exponentformat")]
    exponent_format: Option<ExponentFormat>,
    #[serde(rename = "minexponent")]
    min_exponent: Option<u8>,
    #[serde(rename = "separatethousands")]
    separate_thousands: Option<bool>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickformatstops")]
    tick_format_stops: Option<TickFormatStop>,
    layer: Option<AxisLayer>,
}

#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct RadialAxis {
    visible: Option<bool>,
    // Should this have `#[field_setter(skip)]`?
    #[serde(rename = "type")]
    axis_type: Option<RadialAxisType>,
    #[serde(rename = "autotypenumbers")]
    auto_type_numbers: Option<AutoTypeNumbers>,
    #[serde(rename = "autorangeoptions")]
    auto_range_options: Option<AutoRangeOptions>,
    #[serde(rename = "autorange")]
    auto_range: Option<AutoRange>,
    #[serde(rename = "rangemode")]
    range_mode: Option<RangeMode>,
    #[serde(rename = "minallowed")]
    min_allowed: Option<NumOrString>,
    #[serde(rename = "maxallowed")]
    max_allowed: Option<NumOrString>,
    range: Option<NumOrStringCollection>,
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    #[serde(rename = "categoryarray")]
    category_array: Option<NumOrStringCollection>,
    angle: Option<f64>,
    #[serde(rename = "autotickangles")]
    auto_tick_angles: Option<Vec<f64>>,
    side: Option<PolarDirection>,
    title: Option<Title>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
    #[serde(flatten)]
    axis_attributes: Option<PolarAxis>,
}

#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct AngularAxis {
    visible: Option<bool>,
    // Should this have `#[field_setter(skip)]`?
    #[serde(rename = "type")]
    axis_type: Option<AngularAxisType>,
    #[serde(rename = "autotypenumbers")]
    auto_type_numbers: Option<AutoTypeNumbers>,
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    #[serde(rename = "categoryarray")]
    category_array: Option<NumOrStringCollection>,
    #[serde(rename = "thetaunit")]
    theta_unit: Option<ThetaUnit>,
    period: Option<usize>,
    direction: Option<PolarDirection>,
    rotation: Option<f64>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
    #[serde(flatten)]
    axis_attributes: Option<PolarAxis>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AngularAxisType {
    #[serde(rename = "-")]
    Default,
    Linear,
    Category,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoRange {
    Max,
    #[serde(rename = "max reversed")]
    MaxReversed,
    Min,
    #[serde(rename = "min reversed")]
    MinReversed,
    Reversed,
    #[serde(untagged)]
    Bool(bool),
}

#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct AutoRangeOptions {
    #[serde(rename = "minallowed")]
    min_allowed: Option<NumOrString>,
    #[serde(rename = "maxallowed")]
    max_allowed: Option<NumOrString>,
    #[serde(rename = "clipmin")]
    clip_min: Option<NumOrString>,
    #[serde(rename = "clipmax")]
    clip_max: Option<NumOrString>,
    include: Option<NumOrStringCollection>,
}

// Perhaps move these enums to plotly/common
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoTypeNumbers {
    #[serde(rename = "convert types")]
    Convert,
    Strict,
}

#[derive(Clone, Debug, Serialize)]
pub enum AxisLayer {
    #[serde(rename = "above traces")]
    Above,
    #[serde(rename = "below traces")]
    Below,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GridShape {
    Circular,
    Linear,
}

#[derive(Clone, Debug, Serialize)]
pub struct Hole(f64);

impl Hole {
    // Just following the boxed error pattern I see elsewhere in the repo. Happy to
    // make a custom error type if that's preferred.
    pub fn new(value: f64) -> Result<Self, Box<dyn std::error::Error>> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(format!("The value for a LayoutPolar angular axis Hole must be between 0.0 and 1.0. Given value: {value}").into())
        }
    }
}

impl AsRef<f64> for Hole {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Display for Hole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MinorLogLabels {
    #[serde(rename = "small digits")]
    SmallDigits,
    Complete,
    None,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PolarDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RadialAxisType {
    #[serde(rename = "-")]
    Default,
    Linear,
    Log,
    Date,
    Category,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ThetaUnit {
    Degrees,
    Radians,
}

mod tests {
    use super::*;

    #[test]
    fn serialize_layout_polar() {
        todo!()
    }
}
