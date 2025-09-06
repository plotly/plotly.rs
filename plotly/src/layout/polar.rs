use std::{fmt::Display, num::NonZeroU8};

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{DashType, ExponentFormat, Font, TickFormatStop, Ticks, Title},
    layout::{ArrayShow, CategoryOrder, RangeMode},
    private::NumOrString,
};

/// The layout for a polar (circular) plot, consisting of a radial axis R
/// (distance from the center) and an angular axis Theta (distance around the
/// circumference). See [`ScatterPolar`](crate::traces::ScatterPolar) for
/// details on traces.
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct LayoutPolar {
    /// Sets the angular span of the polar subplot using two angles (in
    /// degrees). Sectors are assumed to be spanned in the counterclockwise
    /// direction, with `0` corresponding to the rightmost limit of the polar
    /// subplot.
    sector: Option<[f64; 2]>,
    /// Sets the fraction of the radius to remove from the center of the polar
    /// subplot. The value wrapped by the [`Hole`] must be between 0.0 and 1.0.
    hole: Option<Hole>,
    /// Sets the background color of the polar subplot.
    #[serde(rename = "bgcolor")]
    bg_color: Option<Box<dyn Color>>,
    /// The attributes describing the radial axis of the plot.
    #[serde(rename = "radialaxis")]
    radial_axis: Option<RadialAxis>,
    /// The attributes describing the angular axis of the plot.
    #[serde(rename = "angularaxis")]
    angular_axis: Option<AngularAxis>,
    /// When the axis type is set to [`RadialAxisType::Category`] or
    /// [`AngularAxisType::Category`] the [`GridShape`] determines whether the
    /// radial axis grid lines and angular axis line are drawn as circular
    /// sectors or as linear (polygon) sectors.
    #[serde(rename = "gridshape")]
    grid_shape: Option<GridShape>,
    /// Controls the persistence of user-driven changes in the axis `range`,
    /// `autorange`, `angle`, and `title` when in the `editable: true`
    /// configuration.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl LayoutPolar {
    /// Create a new layout with default settings.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Describes the radial axis of the plot, extending from the center to the
/// periphery.
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct RadialAxis {
    visible: Option<bool>,
    /// Explicitly set the [`RadialAxisType`]. By default, Plotly attempts to
    /// infer the axis type from the data in the plot's traces.
    #[serde(rename = "type")]
    axis_type: Option<RadialAxisType>,
    /// Setting [`AutoTypeNumbers::Strict`](AutoTypeNumbers) prevents Plotly
    /// from coercing numeric strings in trace data into numbers. This may
    /// affect the inferred [`axis_type`](RadialAxis::axis_type).
    #[serde(rename = "autotypenumbers")]
    auto_type_numbers: Option<AutoTypeNumbers>,
    /// Sets the autorange options. See [`AutoRangeOptions`].
    #[serde(rename = "autorangeoptions")]
    auto_range_options: Option<AutoRangeOptions>,
    #[serde(rename = "autorange")]
    auto_range: Option<AutoRange>,
    /// Set the [`RangeMode`]:
    ///
    /// - [`RangeMode::Normal`]: The range is computed relative to the extremes
    ///   of the input data.
    /// - [`RangeMode::ToZero`]: The range extends to 0, regardless of the input
    ///   data.
    /// - [`RangeMode::NonNegative`]: The range is non-negative, regardless of
    ///   the input data.
    #[serde(rename = "rangemode")]
    range_mode: Option<RangeMode>,
    /// Determines the minimum range of this axis.
    #[serde(rename = "minallowed")]
    min_allowed: Option<NumOrString>,
    /// Determines the maximum range of this axis.
    #[serde(rename = "maxallowed")]
    max_allowed: Option<NumOrString>,
    /// Sets the range of this axis by supplying minimum and maximum values. If
    /// the [`axis_type`](RadialAxis::axis_type) is
    /// [`Log`](RadialAxisType::Log), then input the log of your desired
    /// range. For example, to set the range from 1 to 100, set the log
    /// range from 0 to 2. If the [`axis_type`](RadialAxis::axis_type) is
    /// [`Date`](RadialAxisType::Date), then set the range with date
    /// strings. If the [`axis_type`](RadialAxis::axis_type)
    /// is [`Category`](RadialAxisType::Category), then supply integers, which
    /// are applied serially to each category.
    range: Option<[NumOrString; 2]>,
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    /// Supply categories for this axis. The order is determined by
    /// [`category_order`](RadialAxis::category_order).
    #[serde(rename = "categoryarray")]
    category_array: Option<Vec<NumOrString>>,
    /// Sets the angle (in degrees) from which the radial axis is drawn. By
    /// default, the radial axis line is drawn on the theta=0 line, pointing to
    /// the right. If you set a [`sector`](LayoutPolar::sector), the line will
    /// be drawn on the first angle of the sector.
    angle: Option<f64>,
    /// When [`tick_angle`](PolarAxisTicks::tick_angle) is unspecified, it will
    /// automatically be set to the first angle in this [`Vec`] that is large
    /// enough to prevent label overlap.
    #[serde(rename = "autotickangles")]
    auto_tick_angles: Option<Vec<f64>>,
    /// Determines on which side of the radial axis line the ticks and tick
    /// labels appear.
    #[serde(rename = "side")]
    tick_side: Option<PolarDirection>,
    /// Set the axis [`Title`].
    title: Option<Title>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
    /// Controls the persistence of user-driven changes in radial axis `range`,
    /// `autorange`, `angle`, and `title` when in the `editable: true`
    /// configuration. Defaults to [`ui_revision`](LayoutPolar::ui_revision).
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
    /// Sets the axis attributes. See [`PolarAxisAttributes`].
    #[serde(flatten)]
    axis_attributes: Option<PolarAxisAttributes>,
}

impl RadialAxis {
    /// Create a new radial axis with default settings.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Describes the angular (circular) axis of the plot.
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct AngularAxis {
    visible: Option<bool>,
    /// Explicitly set the [`AngularAxisType`]. By default, Plotly attempts to
    /// infer the axis type from the data in the plot's traces.
    #[serde(rename = "type")]
    axis_type: Option<AngularAxisType>,
    /// Setting [`AutoTypeNumbers::Strict`] prevents Plotly from coercing
    /// numeric strings in trace data into numbers. This may affect the inferred
    /// [`axis_type`](AngularAxis::axis_type).
    #[serde(rename = "autotypenumbers")]
    auto_type_numbers: Option<AutoTypeNumbers>,
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    /// Supply categories for this axis. The order is determined by
    /// [`category_order`](AngularAxis::category_order).
    #[serde(rename = "categoryarray")]
    category_array: Option<Vec<NumOrString>>,
    /// Set the units for the angular axis. See [`ThetaUnit`].
    #[serde(rename = "thetaunit")]
    theta_unit: Option<ThetaUnit>,
    /// If the [`axis_type`](AngularAxis::axis_type) is
    /// [`AngularAxisType::Category`], this value will be used for the angular
    /// period.
    period: Option<usize>,
    /// Sets the direction corresponding to positive angles. See
    /// [`PolarDirection`].
    direction: Option<PolarDirection>,
    /// Sets the start position (in degrees) of the angular axis. By default,
    /// polar subplots with [`direction`](AngularAxis::direction) set to
    /// [`PolarDirection::Counterclockwise`] will get a `rotation` of `0`, which
    /// corresponds to due East (like what mathematicians prefer). Polar
    /// subplots with [`direction`](AngularAxis::direction) set to
    /// [`PolarDirection::Clockwise`] will get a `rotation` of `90`, which
    /// corresponds to due North (like on a compass).
    rotation: Option<f64>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
    /// Controls the persistence of user-driven changes in angular axis
    /// `rotation` when in the `editable: true` configuration. Defaults to
    /// [`ui_revision`](LayoutPolar::ui_revision).
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
    /// Sets the axis attributes. See [`PolarAxisAttributes`].
    #[serde(flatten)]
    axis_attributes: Option<PolarAxisAttributes>,
}

impl AngularAxis {
    /// Create a new angular axis with default settings.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Provides styles for an axis in [`LayoutPolar`]. May be applied to
/// [`RadialAxis`] or [`AngularAxis`].
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct PolarAxisAttributes {
    color: Option<Box<dyn Color>>,
    #[serde(rename = "showline")]
    show_line: Option<bool>,
    #[serde(rename = "linecolor")]
    line_color: Option<Box<dyn Color>>,
    /// Set the width of the axis line in px.
    #[serde(rename = "linewidth")]
    line_width: Option<usize>,
    #[serde(rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(rename = "gridcolor")]
    grid_color: Option<Box<dyn Color>>,
    /// Set the width of the grid lines in px.
    #[serde(rename = "gridwidth")]
    grid_width: Option<usize>,
    #[serde(rename = "griddash")]
    grid_dash: Option<DashType>,
    #[serde(flatten)]
    ticks: Option<PolarAxisTicks>,
}

impl PolarAxisAttributes {
    /// Create a new set of axis attributes with default settings.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Provides styles for the axis ticks in [`LayoutPolar`]. May be applied to
/// [`RadialAxis`] or [`AngularAxis`].
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct PolarAxisTicks {
    #[serde(flatten)]
    tick_mode: Option<PolarTickMode>,
    ticks: Option<Ticks>,
    /// Set the length of the ticks in px.
    #[serde(rename = "ticklen")]
    tick_length: Option<usize>,
    /// Set the width of the ticks in px.
    #[serde(rename = "tickwidth")]
    tick_width: Option<usize>,
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    /// A label will be applied to every `n` ticks, where `n` is the value of
    /// [`tick_label_step`](PolarAxisTicks::tick_label_step). This setting will
    /// be overridden by any text explicitly supplied in
    /// [`PolarTickMode::Array`].
    #[serde(rename = "ticklabelstep")]
    tick_label_step: Option<NonZeroU8>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    /// Applies a label alias to tick or hover labels matching specific
    /// patterns. For example using { US: \'USA\', CA: \'Canada\' } changes US
    /// to USA, and CA to Canada. The labels we would have shown must match the
    /// keys exactly, after adding any tick_prefix or tick_suffix. `label_alias`
    /// can be used with any axis type, and both keys and values can include
    /// html-like tags or MathJax.
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
    /// For an [`exponent_format`](PolarAxisTicks::exponent_format) of
    /// [`SI`](ExponentFormat::SI), hide the SI prefix if the exponent is below
    /// this number.
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
    /// Sets the layer on which the axis is displayed relative to the traces.
    /// When combined with a [`clip_on_axis`](crate::ScatterPolar::clip_on_axis)
    /// value of `false`, markers and text nodes can be set to appear above the
    /// axis.
    layer: Option<AxisLayer>,
}

impl PolarAxisTicks {
    /// Create a new tick layout with default settings.
    pub fn new() -> Self {
        Default::default()
    }
}

/// The type of the angular (circular) axis of a polar plot.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AngularAxisType {
    /// Infer the axis type from the data in its traces.
    #[serde(rename = "-")]
    Default,
    Linear,
    Category,
}

/// Determines whether or not the range of this axis is computed in relation to
/// the input data.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoRange {
    /// Use autorange only to set the maximum value.
    Max,
    /// Use autorange only to set the maximum value on a reversed axis.
    #[serde(rename = "max reversed")]
    MaxReversed,
    /// Use autorange only to set the minimum value.
    Min,
    /// Use autorange only to set the minimum value on a reversed axis.
    #[serde(rename = "min reversed")]
    MinReversed,
    /// Reverse the axis, and use autorange to set both minimum and maximum
    /// values.
    Reversed,
    /// If true, use autorange to set both minimum and maximum values. If false,
    /// do not use autorange to set either value.
    #[serde(untagged)]
    Bool(bool),
}

/// Controls how the maximum and minimum values are calculated by autorange.
#[derive(Clone, Debug, FieldSetter, Serialize)]
pub struct AutoRangeOptions {
    /// Use this value as the autorange minimum.
    #[serde(rename = "minallowed")]
    min_allowed: Option<NumOrString>,
    /// Use this value as the autorange maximum.
    #[serde(rename = "maxallowed")]
    max_allowed: Option<NumOrString>,
    /// Clip the autorange minimum if it goes beyond this value. If
    /// [`min_allowed`](AutoRangeOptions::min_allowed) is also specified, it
    /// will take precedence.
    #[serde(rename = "clipmin")]
    clip_min: Option<NumOrString>,
    /// Clip the autorange maximum if it goes beyond this value. If
    /// [`max_allowed`](AutoRangeOptions::max_allowed) is also specified, it
    /// will take precedence.
    #[serde(rename = "clipmax")]
    clip_max: Option<NumOrString>,
    include: Option<Vec<NumOrString>>,
}

impl AutoRangeOptions {
    /// Create a new set of autorange options with default settings.
    pub fn new() -> Self {
        Default::default()
    }
}

/// Setting [`AutoTypeNumbers::Strict`] prevents Plotly from coercing numeric
/// strings in trace data into numbers. This may affect the inferred
/// [`axis_type`](RadialAxis::axis_type). Coercing/converting is the default
/// behavior.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoTypeNumbers {
    #[serde(rename = "convert types")]
    Convert,
    Strict,
}

/// Whether to layer the axis above or below its traces.
#[derive(Clone, Debug, Serialize)]
pub enum AxisLayer {
    #[serde(rename = "above traces")]
    Above,
    #[serde(rename = "below traces")]
    Below,
}

/// When the axis type is set to [`RadialAxisType::Category`] or
/// [`AngularAxisType::Category`], the [`GridShape`] determines if the radial
/// axis grid lines and angular axis line are drawn as circular sectors or as
/// linear (polygon) sectors.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GridShape {
    Circular,
    Linear,
}

/// Removes material from the center of a polar plot, by supplying a percentage
/// of the radial axis to eliminate. The supplied value must be between `0.0`
/// and `1.0`.
#[derive(Clone, Debug, Serialize)]
pub struct Hole(f64);

impl Hole {
    /// Create a new hole from the given value.
    pub fn new(value: f64) -> Result<Self, Box<dyn std::error::Error>> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(format!("The value for a LayoutPolar angular axis Hole must be between 0.0 and 1.0. Given value: {value}").into())
        }
    }

    /// Return the inner value of the hole.
    pub fn inner(&self) -> f64 {
        self.0
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

/// Specify how minor log labels are displayed.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MinorLogLabels {
    #[serde(rename = "small digits")]
    SmallDigits,
    Complete,
    None,
}

/// A direction around the angular axis of a polar plot.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PolarDirection {
    Clockwise,
    Counterclockwise,
}

/// Sets the tick mode for this axis.
///
/// - [`Auto`](PolarTickMode::Auto): the number of ticks is set via
///   [`n_ticks`](PolarTickMode::Auto::n_ticks).
/// - [`Linear`](PolarTickMode::Linear): the placement of the ticks is
///   determined by a starting position
///   ([`tick_0`](PolarTickMode::Linear::tick_0)), and a tick step
///   ([`d_tick`](PolarTickMode::Linear::d_tick)).
/// - [`Array`](PolarTickMode::Array): the placement of the ticks is set via
///   [`tick_values`](PolarTickMode::Array::tick_values), and the tick text is
///   set via [`tick_text`](PolarTickMode::Array::tick_text).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "tickmode")]
pub enum PolarTickMode {
    Auto {
        /// Specifies the maximum number of ticks for this axis. The actual
        /// number of ticks will be chosen automatically to be less than or
        /// equal to [`n_ticks`](PolarTickMode::Auto::n_ticks).
        #[serde(rename = "nticks")]
        n_ticks: Option<usize>,
    },

    Linear {
        /// Sets the placement of the first tick on this axis. If the
        /// [`axis_type`](RadialAxis::axis_type) is [`RadialAxisType::Log`] then
        /// you must take the log of your starting tick. For example, to set the
        /// starting tick to 100, set [`tick_0`](PolarTickMode::Linear::tick_0)
        /// to 2 (except when [`d_tick`](PolarTickMode::Linear::d_tick) = *L<f>*
        /// (see [`d_tick`](PolarTickMode::Linear::d_tick) for more
        /// information). If the [`axis_type`](RadialAxis::axis_type) is
        /// [`RadialAxisType::Date`], it should be a date string. If the
        /// [`axis_type`](RadialAxis::axis_type) is [`RadialAxisType::Category`]
        /// or [`AngularAxisType::Category`] then supply an integer, which will
        /// be incremented with each category.
        #[serde(rename = "tick0")]
        tick_0: Option<NumOrString>,
        /// Sets the step size between ticks on this axis. Must be a positive
        /// number or a string suitable for *log* or *date* axes. If the axis
        /// type is set to [`RadialAxisType::Log`], ticks will be set every
        /// 10^(n*d_tick) where n is the tick number. For example, to set a tick
        /// mark at 1, 10, 100, 1000, ... set d_tick to 1. To set tick marks at
        /// 1, 100, 10000, ... set dtick to 2. To set tick marks at 1, 5, 25,
        /// 125, 625, 3125, ... set dtick to log_10(5), or 0.69897000433.
        ///
        /// [`RadialAxisType::Log`] has several special values:
        ///
        /// - *L<f>*, where `f` is a positive number, gives ticks linearly
        ///   spaced in value (but not position). For example,
        ///   [`tick_0`](PolarTickMode::Linear::tick_0) = 0.1,
        ///   [`d_tick`](PolarTickMode::Linear::d_tick) = *L0.5* will put ticks
        ///   at 0.1, 0.6, 1.1, 1.6, ...
        /// - To show powers of 10 plus small digits between, use *D1* (all
        ///   digits) or *D2* (only 2 and 5).
        ///   [`tick_0`](PolarTickMode::Linear::tick_0) is ignored for *D1* and
        ///   *D2*. If the axis `type` is [`RadialAxisType::Date`], then you
        ///   must convert the time to milliseconds. For example, to set the
        ///   interval between ticks to one day, set
        ///   [`d_tick`](PolarTickMode::Linear::d_tick) to 86400000.0.
        ///
        /// [`RadialAxisType::Date`] also has special values:
        ///
        /// - *M<n>* gives ticks spaced by a number of months, where `n` is a
        ///   positive integer. To set ticks on the 15th of every third month,
        ///   set [`tick_0`](PolarTickMode::Linear::tick_0) to *2000-01-15* and
        ///   [`d_tick`](PolarTickMode::Linear::d_tick) to *M3*. To set ticks
        ///   every 4 years, set [`d_tick`](PolarTickMode::Linear::d_tick) to
        ///   *M48*
        #[serde(rename = "dtick")]
        d_tick: Option<NumOrString>,
    },

    Array {
        /// Sets the values at which ticks on this axis appear.
        #[serde(rename = "tickvals")]
        tick_values: Option<Vec<f64>>,
        /// Sets the text associated with each value in
        /// [`tick_values`](PolarTickMode::Array::tick_values).
        #[serde(rename = "ticktext")]
        tick_text: Option<Vec<String>>,
    },
}

/// The type of the radial (center outward) axis of a polar plot.
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

/// Specify the units for the angular axis of a polar plot.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ThetaUnit {
    Degrees,
    Radians,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::Mode, Layout, Plot, ScatterPolar};

    // The focus of the test is serialization, so we test all options for
    // [`LayoutPolar`], even though some of those options would normally be mutually
    // exclusive.
    #[test]
    fn serialize_layout_polar() {
        let ticks = PolarAxisTicks::new()
            .tick_mode(PolarTickMode::Auto { n_ticks: None })
            .tick_color("#dddddd")
            .tick_font(Font::new().color("#eeeeee"));

        let axis_attributes = PolarAxisAttributes::new()
            .color("#111111")
            .show_line(true)
            .line_color("#ffff00")
            .line_width(5)
            .show_grid(true)
            .grid_color("#444444")
            .grid_width(2)
            .grid_dash(DashType::Solid)
            .ticks(ticks);

        let radial_axis = RadialAxis::new()
            .visible(true)
            .axis_type(RadialAxisType::Linear)
            .auto_type_numbers(AutoTypeNumbers::Strict)
            .auto_range_options(AutoRangeOptions::new().min_allowed(1))
            .auto_range(AutoRange::Bool(true))
            .range_mode(RangeMode::Normal)
            .min_allowed(0_u32)
            .max_allowed(105_u32)
            .range([5.into(), 100.into()])
            .category_order(CategoryOrder::Trace)
            .category_array(vec!["category 1".into(), "category 2".into()])
            .angle(0.0)
            .auto_tick_angles(vec![0.0, 12.2, 30.85])
            .tick_side(PolarDirection::Counterclockwise)
            .title("My Title")
            .hover_format("%{label}: <br>Popularity: %{percent} </br> %{text}")
            .axis_attributes(axis_attributes.clone());

        let angular_axis = AngularAxis::new()
            .visible(true)
            .axis_type(AngularAxisType::Default)
            .auto_type_numbers(AutoTypeNumbers::Convert)
            .category_order(CategoryOrder::CategoryAscending)
            .category_array(vec!["category 3".into(), "category 4".into()])
            .theta_unit(ThetaUnit::Radians)
            .period(10)
            .direction(PolarDirection::Counterclockwise)
            .rotation(5.0)
            .hover_format("GDP: %{x} <br>Life Expectancy: %{y}")
            .axis_attributes(axis_attributes);

        let layout_polar = LayoutPolar::new()
            .sector([0.0, 270.0])
            .hole(Hole::new(0.2).unwrap())
            .bg_color("#dddddd")
            .radial_axis(radial_axis)
            .angular_axis(angular_axis)
            .grid_shape(GridShape::Circular);

        let layout = Layout::new().polar(layout_polar);
        let json = serde_json::to_string(&layout).unwrap();

        let expected = r##"{"polar":{"sector":[0.0,270.0],"hole":0.2,"bgcolor":"#dddddd","radialaxis":{"visible":true,"type":"linear","autotypenumbers":"strict","autorangeoptions":{"minallowed":1,"maxallowed":null,"clipmin":null,"clipmax":null,"include":null},"autorange":true,"rangemode":"normal","minallowed":0,"maxallowed":105,"range":[5,100],"categoryorder":"trace","categoryarray":["category 1","category 2"],"angle":0.0,"autotickangles":[0.0,12.2,30.85],"side":"counterclockwise","title":{"text":"My Title"},"hoverformat":"%{label}: <br>Popularity: %{percent} </br> %{text}","uirevision":null,"color":"#111111","showline":true,"linecolor":"#ffff00","linewidth":5,"showgrid":true,"gridcolor":"#444444","gridwidth":2,"griddash":"solid","tickmode":"auto","nticks":null,"ticks":null,"ticklen":null,"tickwidth":null,"tickcolor":"#dddddd","ticklabelstep":null,"showticklabels":null,"labelalias":null,"minorloglabels":null,"showtickprefix":null,"tickprefix":null,"showticksuffix":null,"ticksuffix":null,"showexponent":null,"exponentformat":null,"minexponent":null,"separatethousands":null,"tickfont":{"color":"#eeeeee"},"tickangle":null,"tickformat":null,"tickformatstops":null,"layer":null},"angularaxis":{"visible":true,"type":"-","autotypenumbers":"convert types","categoryorder":"category ascending","categoryarray":["category 3","category 4"],"thetaunit":"radians","period":10,"direction":"counterclockwise","rotation":5.0,"hoverformat":"GDP: %{x} <br>Life Expectancy: %{y}","uirevision":null,"color":"#111111","showline":true,"linecolor":"#ffff00","linewidth":5,"showgrid":true,"gridcolor":"#444444","gridwidth":2,"griddash":"solid","tickmode":"auto","nticks":null,"ticks":null,"ticklen":null,"tickwidth":null,"tickcolor":"#dddddd","ticklabelstep":null,"showticklabels":null,"labelalias":null,"minorloglabels":null,"showtickprefix":null,"tickprefix":null,"showticksuffix":null,"ticksuffix":null,"showexponent":null,"exponentformat":null,"minexponent":null,"separatethousands":null,"tickfont":{"color":"#eeeeee"},"tickangle":null,"tickformat":null,"tickformatstops":null,"layer":null},"gridshape":"circular","uirevision":null}}"##;

        assert_eq!(json, expected);
    }
}
