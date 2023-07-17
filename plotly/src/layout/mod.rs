pub mod themes;
pub mod update_menu;

use std::borrow::Cow;

use plotly_derive::FieldSetter;
use serde::{Serialize, Serializer};
use update_menu::UpdateMenu;

use crate::{
    color::Color,
    common::{
        Anchor, AxisSide, Calendar, ColorBar, ColorScale, DashType, ExponentFormat, Font, Label,
        Orientation, TickFormatStop, TickMode, Title,
    },
    private::{NumOrString, NumOrStringCollection},
};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    #[serde(rename = "-")]
    Default,
    Linear,
    Log,
    Date,
    Category,
    MultiCategory,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AxisConstrain {
    Range,
    Domain,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ConstrainDirection {
    Left,
    Center,
    Right,
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RangeMode {
    Normal,
    ToZero,
    NonNegative,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TicksDirection {
    Outside,
    Inside,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TicksPosition {
    Labels,
    Boundaries,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ArrayShow {
    All,
    First,
    Last,
    None,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BarMode {
    Stack,
    Group,
    Overlay,
    Relative,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BarNorm {
    #[serde(rename = "")]
    Empty,
    Fraction,
    Percent,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BoxMode {
    Group,
    Overlay,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ViolinMode {
    Group,
    Overlay,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WaterfallMode {
    Group,
    Overlay,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TraceOrder {
    Reversed,
    Grouped,
    #[serde(rename = "reversed+grouped")]
    ReversedGrouped,
    Normal,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ItemSizing {
    Trace,
    Constant,
}

#[derive(Debug, Clone)]
pub enum ItemClick {
    Toggle,
    ToggleOthers,
    False,
}

impl Serialize for ItemClick {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Toggle => serializer.serialize_str("toggle"),
            Self::ToggleOthers => serializer.serialize_str("toggleothers"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GroupClick {
    ToggleItem,
    ToggleGroup,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Legend {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    #[serde(rename = "borderwidth")]
    border_width: Option<usize>,
    font: Option<Font>,
    orientation: Option<Orientation>,
    #[serde(rename = "traceorder")]
    trace_order: Option<TraceOrder>,
    #[serde(rename = "tracegroupgap")]
    trace_group_gap: Option<usize>,
    #[serde(rename = "itemsizing")]
    item_sizing: Option<ItemSizing>,
    #[serde(rename = "itemclick")]
    item_click: Option<ItemClick>,
    #[serde(rename = "itemdoubleclick")]
    item_double_click: Option<ItemClick>,
    x: Option<f64>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    y: Option<f64>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    valign: Option<VAlign>,
    title: Option<Title>,
    #[serde(rename = "groupclick")]
    group_click: Option<GroupClick>,
    #[serde(rename = "itemwidth")]
    item_width: Option<usize>,
}

impl Legend {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Margin {
    #[serde(rename = "l")]
    left: Option<usize>,
    #[serde(rename = "r")]
    right: Option<usize>,
    #[serde(rename = "t")]
    top: Option<usize>,
    #[serde(rename = "b")]
    bottom: Option<usize>,
    pad: Option<usize>,
    #[serde(rename = "autoexpand")]
    auto_expand: Option<bool>,
}

impl Margin {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct LayoutColorScale {
    sequential: Option<ColorScale>,
    #[serde(rename = "sequentialminus")]
    sequential_minus: Option<ColorScale>,
    diverging: Option<ColorScale>,
}

impl LayoutColorScale {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SliderRangeMode {
    Auto,
    Fixed,
    Match,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct RangeSliderYAxis {
    #[serde(rename = "rangemode")]
    range_mode: Option<SliderRangeMode>,
    range: Option<NumOrStringCollection>,
}

impl RangeSliderYAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct RangeSlider {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    #[serde(rename = "borderwidth")]
    border_width: Option<u64>,
    #[serde(rename = "autorange")]
    auto_range: Option<bool>,
    range: Option<NumOrStringCollection>,
    thickness: Option<f64>,
    visible: Option<bool>,
    #[serde(rename = "yaxis")]
    y_axis: Option<RangeSliderYAxis>,
}

impl RangeSlider {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SelectorStep {
    Month,
    Year,
    Day,
    Hour,
    Minute,
    Second,
    All,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StepMode {
    Backward,
    ToDate,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct SelectorButton {
    visible: Option<bool>,
    step: Option<SelectorStep>,
    #[serde(rename = "stepmode")]
    step_mode: Option<StepMode>,
    count: Option<usize>,
    label: Option<String>,
    name: Option<String>,
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl SelectorButton {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct RangeSelector {
    visible: Option<bool>,
    buttons: Option<Vec<SelectorButton>>,
    x: Option<f64>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    y: Option<f64>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    font: Option<Font>,
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "activecolor")]
    active_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    #[serde(rename = "borderwidth")]
    border_width: Option<usize>,
}

impl RangeSelector {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ColorAxis {
    cauto: Option<bool>,
    cmin: Option<f64>,
    cmax: Option<f64>,
    cmid: Option<f64>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
}

impl ColorAxis {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpikeMode {
    ToAxis,
    Across,
    Marker,
    #[serde(rename = "toaxis+across")]
    ToaxisAcross,
    #[serde(rename = "toaxis+marker")]
    ToAxisMarker,
    #[serde(rename = "across+marker")]
    AcrossMarker,
    #[serde(rename = "toaxis+across+marker")]
    ToaxisAcrossMarker,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpikeSnap {
    Data,
    Cursor,
    #[serde(rename = "hovered data")]
    HoveredData,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Axis {
    visible: Option<bool>,
    color: Option<Box<dyn Color>>,
    title: Option<Title>,
    #[field_setter(skip)]
    r#type: Option<AxisType>,
    #[serde(rename = "autorange")]
    auto_range: Option<bool>,
    #[serde(rename = "rangemode")]
    range_mode: Option<RangeMode>,
    range: Option<NumOrStringCollection>,
    #[serde(rename = "fixedrange")]
    fixed_range: Option<bool>,
    constrain: Option<AxisConstrain>,
    #[serde(rename = "constraintoward")]
    constrain_toward: Option<ConstrainDirection>,
    #[serde(rename = "tickmode")]
    tick_mode: Option<TickMode>,
    #[serde(rename = "nticks")]
    n_ticks: Option<usize>,

    tick0: Option<f64>,
    dtick: Option<f64>,

    #[field_setter(skip)]
    matches: Option<String>,

    #[serde(rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    ticks: Option<TicksDirection>,
    #[serde(rename = "tickson")]
    ticks_on: Option<TicksPosition>,
    mirror: Option<bool>,
    #[serde(rename = "ticklen")]
    tick_length: Option<usize>,
    #[serde(rename = "tickwidth")]
    tick_width: Option<usize>,
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(rename = "automargin")]
    auto_margin: Option<bool>,
    #[serde(rename = "showspikes")]
    show_spikes: Option<bool>,
    #[serde(rename = "spikecolor")]
    spike_color: Option<Box<dyn Color>>,
    #[serde(rename = "spikethickness")]
    spike_thickness: Option<usize>,
    #[serde(rename = "spikedash")]
    spike_dash: Option<DashType>,
    #[serde(rename = "spikemode")]
    spike_mode: Option<SpikeMode>,
    #[serde(rename = "spikesnap")]
    spike_snap: Option<SpikeSnap>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(rename = "showtickprefix")]
    show_tick_prefix: Option<ArrayShow>,
    #[serde(rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(rename = "showticksuffix")]
    show_tick_suffix: Option<ArrayShow>,
    #[serde(rename = "showexponent")]
    show_exponent: Option<ArrayShow>,
    #[serde(rename = "exponentformat")]
    exponent_format: Option<ExponentFormat>,
    #[serde(rename = "separatethousands")]
    separate_thousands: Option<bool>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickformatstops")]
    tick_format_stops: Option<Vec<TickFormatStop>>,
    #[serde(rename = "hoverformat")]
    hover_format: Option<String>,
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
    #[serde(rename = "zeroline")]
    zero_line: Option<bool>,
    #[serde(rename = "zerolinecolor")]
    zero_line_color: Option<Box<dyn Color>>,
    #[serde(rename = "zerolinewidth")]
    zero_line_width: Option<usize>,
    #[serde(rename = "showdividers")]
    show_dividers: Option<bool>,
    #[serde(rename = "dividercolor")]
    divider_color: Option<Box<dyn Color>>,
    #[serde(rename = "dividerwidth")]
    divider_width: Option<usize>,
    anchor: Option<String>,
    side: Option<AxisSide>,
    overlaying: Option<String>,
    #[field_setter(skip)]
    domain: Option<Vec<f64>>,
    position: Option<f64>,
    #[serde(rename = "rangeslider")]
    range_slider: Option<RangeSlider>,
    #[serde(rename = "rangeselector")]
    range_selector: Option<RangeSelector>,
    calendar: Option<Calendar>,
}

impl Axis {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn matches(mut self, matches: bool) -> Self {
        if matches {
            self.matches = Some(String::from("x"));
        }
        self
    }

    pub fn type_(mut self, t: AxisType) -> Self {
        self.r#type = Some(t);
        self
    }

    pub fn domain(mut self, domain: &[f64]) -> Self {
        self.domain = Some(domain.to_vec());
        self
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum RowOrder {
    #[serde(rename = "top to bottom")]
    TopToBottom,
    #[serde(rename = "bottom to top")]
    BottomToTop,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GridPattern {
    Independent,
    Coupled,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GridXSide {
    Bottom,
    #[serde(rename = "bottom plot")]
    BottomPlot,
    #[serde(rename = "top plot")]
    TopPlot,
    Top,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GridYSide {
    Left,
    #[serde(rename = "left plot")]
    LeftPlot,
    #[serde(rename = "right plot")]
    RightPlot,
    Right,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct GridDomain {
    x: Option<Vec<f64>>,
    y: Option<Vec<f64>>,
}

impl GridDomain {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct LayoutGrid {
    rows: Option<usize>,
    #[serde(rename = "roworder")]
    row_order: Option<RowOrder>,
    columns: Option<usize>,
    #[serde(rename = "subplots")]
    sub_plots: Option<Vec<String>>,
    #[serde(rename = "xaxes")]
    x_axes: Option<Vec<String>>,
    #[serde(rename = "yaxes")]
    y_axes: Option<Vec<String>>,
    pattern: Option<GridPattern>,
    #[serde(rename = "xgap")]
    x_gap: Option<f64>,
    #[serde(rename = "ygap")]
    y_gap: Option<f64>,
    domain: Option<GridDomain>,
    #[serde(rename = "xside")]
    x_side: Option<GridXSide>,
    #[serde(rename = "yside")]
    y_side: Option<GridYSide>,
}

impl LayoutGrid {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub enum UniformTextMode {
    False,
    Hide,
    Show,
}

impl Serialize for UniformTextMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::False => serializer.serialize_bool(false),
            Self::Hide => serializer.serialize_str("hide"),
            Self::Show => serializer.serialize_str("show"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct UniformText {
    mode: Option<UniformTextMode>,
    #[serde(rename = "minsize")]
    min_size: Option<usize>,
}

impl UniformText {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub enum HoverMode {
    X,
    Y,
    Closest,
    False,
    XUnified,
    YUnified,
}

impl Serialize for HoverMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::X => serializer.serialize_str("x"),
            Self::Y => serializer.serialize_str("y"),
            Self::Closest => serializer.serialize_str("closest"),
            Self::False => serializer.serialize_bool(false),
            Self::XUnified => serializer.serialize_str("x unified"),
            Self::YUnified => serializer.serialize_str("y unified"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ModeBar {
    orientation: Option<Orientation>,
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    color: Option<Box<dyn Color>>,
    #[serde(rename = "activecolor")]
    active_color: Option<Box<dyn Color>>,
}

impl ModeBar {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShapeType {
    Circle,
    Rect,
    Path,
    Line,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShapeLayer {
    Below,
    Above,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShapeSizeMode {
    Scaled,
    Pixel,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ShapeLine {
    /// Sets the line color.
    color: Option<Box<dyn Color>>,
    /// Sets the line width (in px).
    width: Option<f64>,
    /// Sets the dash style of lines. Set to a dash type string ("solid", "dot",
    /// "dash", "longdash", "dashdot", or "longdashdot") or a dash length
    /// list in px (eg "5px,10px,2px,2px").
    dash: Option<DashType>,
}

impl ShapeLine {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Shape {
    /// Determines whether or not this shape is visible.
    visible: Option<bool>,
    #[field_setter(skip)]
    r#type: Option<ShapeType>,
    /// Specifies whether shapes are drawn below or above traces.
    layer: Option<ShapeLayer>,
    /// Sets the shape's x coordinate axis. If set to an x axis id (e.g. "x" or
    /// "x2"), the `x` position refers to an x coordinate. If set to
    /// "paper", the `x` position refers to the distance from the left side
    /// of the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the left (right) side. If the axis `type` is "log", then
    /// you must take the log of your desired range. If the axis `type` is
    /// "date", then you must convert the date to unix time in milliseconds.
    #[serde(rename = "xref")]
    x_ref: Option<String>,
    /// Sets the shapes's sizing mode along the x axis. If set to "scaled",
    /// `x0`, `x1` and x coordinates within `path` refer to data values on
    /// the x axis or a fraction of the plot area's width (`xref` set to
    /// "paper"). If set to "pixel", `xanchor` specifies the x position
    /// in terms of data or plot fraction but `x0`, `x1` and x coordinates
    /// within `path` are pixels relative to `xanchor`. This way, the shape
    /// can have a fixed width while maintaining a position relative to data
    /// or plot fraction.
    #[serde(rename = "xsizemode")]
    x_size_mode: Option<ShapeSizeMode>,
    /// Only relevant in conjunction with `xsizemode` set to "pixel". Specifies
    /// the anchor point on the x axis to which `x0`, `x1` and x coordinates
    /// within `path` are relative to. E.g. useful to attach a pixel sized
    /// shape to a certain data value. No effect when `xsizemode` not set to
    /// "pixel".
    #[serde(rename = "xanchor")]
    x_anchor: Option<NumOrString>,
    /// Sets the shape's starting x position. See `type` and `xsizemode` for
    /// more info.
    x0: Option<NumOrString>,
    /// Sets the shape's end x position. See `type` and `xsizemode` for more
    /// info.
    x1: Option<NumOrString>,
    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g.
    /// "y" or "y2"), the `y` position refers to an y coordinate If set to
    /// "paper", the `y` position refers to the distance from the bottom of
    /// the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the bottom (top).
    #[serde(rename = "yref")]
    y_ref: Option<String>,
    /// Sets the shapes's sizing mode along the y axis. If set to "scaled",
    /// `y0`, `y1` and y coordinates within `path` refer to data values on
    /// the y axis or a fraction of the plot area's height (`yref` set to
    /// "paper"). If set to "pixel", `yanchor` specifies the y position
    /// in terms of data or plot fraction but `y0`, `y1` and y coordinates
    /// within `path` are pixels relative to `yanchor`. This way, the shape
    /// can have a fixed height while maintaining a position relative to
    /// data or plot fraction.
    #[serde(rename = "ysizemode")]
    y_size_mode: Option<ShapeSizeMode>,
    /// Only relevant in conjunction with `ysizemode` set to "pixel". Specifies
    /// the anchor point on the y axis to which `y0`, `y1` and y coordinates
    /// within `path` are relative to. E.g. useful to attach a pixel sized
    /// shape to a certain data value. No effect when `ysizemode` not set to
    /// "pixel".
    #[serde(rename = "yanchor")]
    y_anchor: Option<NumOrString>,
    /// Sets the shape's starting y position. See `type` and `ysizemode` for
    /// more info.
    y0: Option<NumOrString>,
    /// Sets the shape's end y position. See `type` and `ysizemode` for more
    /// info.
    y1: Option<NumOrString>,
    /// For `type` "path" - a valid SVG path with the pixel values replaced by
    /// data values in `xsizemode`/`ysizemode` being "scaled" and taken
    /// unmodified as pixels relative to `xanchor` and `yanchor` in case of
    /// "pixel" size mode. There are a few restrictions / quirks
    /// only absolute instructions, not relative. So the allowed segments
    /// are: M, L, H, V, Q, C, T, S, and Z arcs (A) are not allowed because
    /// radius rx and ry are relative. In the future we could consider
    /// supporting relative commands, but we would have to decide on how to
    /// handle date and log axes. Note that even as is, Q and C Bezier paths
    /// that are smooth on linear axes may not be smooth on log, and vice versa.
    /// no chained "polybezier" commands - specify the segment type for each
    /// one. On category axes, values are numbers scaled to the serial
    /// numbers of categories because using the categories themselves
    /// there would be no way to describe fractional positions On data axes:
    /// because space and T are both normal components of path strings, we
    /// can't use either to separate date from time parts. Therefore we'll
    /// use underscore for this purpose: 2015-02-21_13:45:56.789
    path: Option<String>,
    /// Sets the opacity of the shape. Number between or equal to 0 and 1.
    opacity: Option<f64>,
    /// Sets the shape line properties (`color`, `width`, `dash`).
    line: Option<ShapeLine>,
    /// Sets the color filling the shape's interior. Only applies to closed
    /// shapes.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Determines which regions of complex paths constitute the interior. For
    /// more info please visit https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule
    #[serde(rename = "fillrule")]
    fill_rule: Option<FillRule>,
    /// Determines whether the shape could be activated for edit or not. Has no
    /// effect when the older editable shapes mode is enabled via
    /// `config.editable` or `config.edits.shapePosition`.
    editable: Option<bool>,
    /// When used in a template, named items are created in the output figure in
    /// addition to any items the figure already has in this array. You can
    /// modify these items in the output figure by making your own item with
    /// `templateitemname` matching this `name` alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). Has no
    /// effect outside of a template.
    name: Option<String>,
    /// Used to refer to a named item in this array in the template. Named items
    /// from the template will be created even without a matching item in
    /// the input figure, but you can modify one by making an item with
    /// `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is
    /// no template or no matching item, this item will be hidden unless you
    /// explicitly show it with `visible: true`.
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Shape {
    pub fn new() -> Self {
        Default::default()
    }

    /// Specifies the shape type to be drawn. If "line", a line is drawn from
    /// (`x0`,`y0`) to (`x1`,`y1`) with respect to the axes' sizing mode. If
    /// "circle", a circle is drawn from ((`x0`+`x1`)/2, (`y0`+`y1`)/2))
    /// with radius (|(`x0`+`x1`)/2 - `x0`|, |(`y0`+`y1`)/2 -`y0`)|)
    /// with respect to the axes' sizing mode. If "rect", a rectangle is drawn
    /// linking (`x0`,`y0`), (`x1`,`y0`), (`x1`,`y1`), (`x0`,`y1`),
    /// (`x0`,`y0`) with respect to the axes' sizing mode. If "path", draw a
    /// custom SVG path using `path`. with respect to the axes' sizing mode.
    pub fn shape_type(mut self, shape_type: ShapeType) -> Self {
        self.r#type = Some(shape_type);
        self
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DrawDirection {
    Ortho,
    Horizontal,
    Vertical,
    Diagonal,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct NewShape {
    /// Sets the shape line properties (`color`, `width`, `dash`).
    line: Option<ShapeLine>,
    /// Sets the color filling new shapes' interior. Please note that if using a
    /// fillcolor with alpha greater than half, drag inside the active shape
    /// starts moving the shape underneath, otherwise a new shape could be
    /// started over.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Determines the path's interior. For more info please
    /// visit https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule
    #[serde(rename = "fillrule")]
    fill_rule: Option<FillRule>,
    /// Sets the opacity of new shapes. Number between or equal to 0 and 1.
    opacity: Option<f64>,
    /// Specifies whether new shapes are drawn below or above traces.
    layer: Option<ShapeLayer>,
    /// When `dragmode` is set to "drawrect", "drawline" or "drawcircle" this
    /// limits the drag to be horizontal, vertical or diagonal. Using
    /// "diagonal" there is no limit e.g. in drawing lines in any direction.
    /// "ortho" limits the draw to be either horizontal or vertical.
    /// "horizontal" allows horizontal extend. "vertical" allows vertical
    /// extend.
    #[serde(rename = "drawdirection")]
    draw_direction: Option<DrawDirection>,
}

impl NewShape {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ActiveShape {
    /// Sets the color filling the active shape' interior.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Sets the opacity of the active shape. Number between or equal to 0 and
    /// 1.
    opacity: Option<f64>,
}

impl ActiveShape {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ArrowSide {
    End,
    Start,
    #[serde(rename = "end+start")]
    StartEnd,
    None,
}

#[derive(Debug, Clone)]
pub enum ClickToShow {
    False,
    OnOff,
    OnOut,
}

impl Serialize for ClickToShow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::False => serializer.serialize_bool(false),
            Self::OnOff => serializer.serialize_str("onoff"),
            Self::OnOut => serializer.serialize_str("onout"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Annotation {
    /// Determines whether or not this annotation is visible.
    visible: Option<bool>,
    /// Sets the text associated with this annotation. Plotly uses a subset of
    /// HTML tags to do things like newline (<br>), bold (<b></b>), italics
    /// (<i></i>), hyperlinks (<a href='...'></a>). Tags <em>, <sup>, <sub>
    /// <span> are also supported.
    text: Option<String>,
    /// Sets the angle at which the `text` is drawn with respect to the
    /// horizontal.
    #[serde(rename = "textangle")]
    text_angle: Option<f64>,
    /// Sets the annotation text font.
    font: Option<Font>,
    /// Sets an explicit width for the text box. null (default) lets the text
    /// set the box width. Wider text will be clipped. There is no automatic
    /// wrapping; use <br> to start a new line.
    width: Option<f64>,
    /// Sets an explicit height for the text box. null (default) lets the text
    /// set the box height. Taller text will be clipped.
    height: Option<f64>,
    /// Sets the opacity of the annotation (text + arrow).
    opacity: Option<f64>,
    /// Sets the horizontal alignment of the `text` within the box. Has an
    /// effect only if `text` spans two or more lines (i.e. `text` contains
    /// one or more <br> HTML tags) or if an explicit width is set to
    /// override the text width.
    align: Option<HAlign>,
    /// Sets the vertical alignment of the `text` within the box. Has an effect
    /// only if an explicit height is set to override the text height.
    valign: Option<VAlign>,
    /// Sets the background color of the annotation.
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    /// Sets the color of the border enclosing the annotation `text`.
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    /// Sets the padding (in px) between the `text` and the enclosing border.
    #[serde(rename = "borderpad")]
    border_pad: Option<f64>,
    /// Sets the width (in px) of the border enclosing the annotation `text`.
    #[serde(rename = "borderwidth")]
    border_width: Option<f64>,
    /// Determines whether or not the annotation is drawn with an arrow. If
    /// "True", `text` is placed near the arrow's tail. If "False", `text`
    /// lines up with the `x` and `y` provided.
    #[serde(rename = "showarrow")]
    show_arrow: Option<bool>,
    /// Sets the color of the annotation arrow.
    #[serde(rename = "arrowcolor")]
    arrow_color: Option<Box<dyn Color>>,
    /// Sets the end annotation arrow head style. Integer between or equal to 0
    /// and 8.
    #[serde(rename = "arrowhead")]
    arrow_head: Option<u8>,
    /// Sets the start annotation arrow head style. Integer between or equal to
    /// 0 and 8.
    #[serde(rename = "startarrowhead")]
    start_arrow_head: Option<u8>,
    /// Sets the annotation arrow head position.
    #[serde(rename = "arrowside")]
    arrow_side: Option<ArrowSide>,
    /// Sets the size of the end annotation arrow head, relative to
    /// `arrowwidth`. A value of 1 (default) gives a head about 3x as wide
    /// as the line.
    #[serde(rename = "arrowsize")]
    arrow_size: Option<f64>,
    /// Sets the size of the start annotation arrow head, relative to
    /// `arrowwidth`. A value of 1 (default) gives a head about 3x as wide
    /// as the line.
    #[serde(rename = "startarrowsize")]
    start_arrow_size: Option<f64>,
    /// Sets the width (in px) of annotation arrow line.
    #[serde(rename = "arrowwidth")]
    arrow_width: Option<f64>,
    /// Sets a distance, in pixels, to move the end arrowhead away from the
    /// position it is pointing at, for example to point at the edge of a
    /// marker independent of zoom. Note that this shortens the arrow from
    /// the `ax` / `ay` vector, in contrast to `xshift` / `yshift` which
    /// moves everything by this amount.
    #[serde(rename = "standoff")]
    stand_off: Option<f64>,
    /// Sets a distance, in pixels, to move the start arrowhead away from the
    /// position it is pointing at, for example to point at the edge of a
    /// marker independent of zoom. Note that this shortens the arrow from
    /// the `ax` / `ay` vector, in contrast to `xshift` / `yshift`
    /// which moves everything by this amount.
    #[serde(rename = "startstandoff")]
    start_stand_off: Option<f64>,
    /// Sets the x component of the arrow tail about the arrow head. If `axref`
    /// is `pixel`, a positive (negative) component corresponds to an arrow
    /// pointing from right to left (left to right). If `axref` is an axis,
    /// this is an absolute value on that axis, like `x`, NOT a
    /// relative value.
    ax: Option<NumOrString>,
    /// Sets the y component of the arrow tail about the arrow head. If `ayref`
    /// is `pixel`, a positive (negative) component corresponds to an arrow
    /// pointing from bottom to top (top to bottom). If `ayref` is an axis,
    /// this is an absolute value on that axis, like `y`, NOT a
    /// relative value.
    ay: Option<NumOrString>,
    /// Indicates in what terms the tail of the annotation (ax,ay) is specified.
    /// If `pixel`, `ax` is a relative offset in pixels from `x`. If set to
    /// an x axis id (e.g. "x" or "x2"), `ax` is specified in the same terms
    /// as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    #[serde(rename = "axref")]
    ax_ref: Option<String>,
    /// Indicates in what terms the tail of the annotation (ax,ay) is specified.
    /// If `pixel`, `ay` is a relative offset in pixels from `y`. If set to
    /// a y axis id (e.g. "y" or "y2"), `ay` is specified in the same terms
    /// as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    #[serde(rename = "ayref")]
    ay_ref: Option<String>,
    /// Sets the annotation's x coordinate axis. If set to an x axis id (e.g.
    /// "x" or "x2"), the `x` position refers to an x coordinate If set to
    /// "paper", the `x` position refers to the distance from the left side
    /// of the plotting area in normalized coordinates where 0 (1)
    /// corresponds to the left (right) side.
    #[serde(rename = "xref")]
    x_ref: Option<String>,
    /// Sets the annotation's x position. If the axis `type` is "log", then you
    /// must take the log of your desired range. If the axis `type` is
    /// "date", it should be date strings, like date data, though Date
    /// objects and unix milliseconds will be accepted and converted to strings.
    /// If the axis `type` is "category", it should be numbers, using the scale
    /// where each category is assigned a serial number from zero in the
    /// order it appears.
    x: Option<NumOrString>,
    /// Sets the text box's horizontal position anchor This anchor binds the `x`
    /// position to the "left", "center" or "right" of the annotation. For
    /// example, if `x` is set to 1, `xref` to "paper" and `xanchor` to
    /// "right" then the right-most portion of the annotation lines up with
    /// the right-most edge of the plotting area. If "auto", the anchor is
    /// equivalent to "center" for data-referenced annotations or if there
    /// is an arrow, whereas for paper-referenced with no arrow, the anchor
    /// picked corresponds to the closest side.
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    /// Shifts the position of the whole annotation and arrow to the right
    /// (positive) or left (negative) by this many pixels.
    #[serde(rename = "xshift")]
    x_shift: Option<f64>,
    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g.
    /// "y" or "y2"), the `y` position refers to an y coordinate If set to
    /// "paper", the `y` position refers to the distance from the bottom of
    /// the plotting area in normalized coordinates where 0 (1) corresponds
    /// to the bottom (top).
    #[serde(rename = "yref")]
    y_ref: Option<String>,
    /// Sets the annotation's y position. If the axis `type` is "log", then you
    /// must take the log of your desired range. If the axis `type` is
    /// "date", it should be date strings, like date data, though Date
    /// objects and unix milliseconds will be accepted and converted to strings.
    /// If the axis `type` is "category", it should be numbers, using the
    /// scale where each category is assigned a serial number from zero in
    /// the order it appears.
    y: Option<NumOrString>,
    /// Sets the text box's vertical position anchor This anchor binds the `y`
    /// position to the "top", "middle" or "bottom" of the annotation. For
    /// example, if `y` is set to 1, `yref` to "paper" and `yanchor` to
    /// "top" then the top-most portion of the annotation lines up with the
    /// top-most edge of the plotting area. If "auto", the anchor is equivalent
    /// to "middle" for data-referenced annotations or if there is an arrow,
    /// whereas for paper-referenced with no arrow, the anchor picked
    /// corresponds to the closest side.
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    /// Shifts the position of the whole annotation and arrow up (positive) or
    /// down (negative) by this many pixels.
    #[serde(rename = "yshift")]
    y_shift: Option<f64>,
    /// Makes this annotation respond to clicks on the plot. If you click a data
    /// point that exactly matches the `x` and `y` values of this
    /// annotation, and it is hidden (visible: false), it will appear. In
    /// "onoff" mode, you must click the same point again to make it disappear,
    /// so if you click multiple points, you can show multiple annotations.
    /// In "onout" mode, a click anywhere else in the plot (on another data
    /// point or not) will hide this annotation. If you need to show/hide
    /// this annotation in response to different `x` or `y` values, you can set
    /// `xclick` and/or `yclick`. This is useful for example to label the side
    /// of a bar. To label markers though, `standoff` is preferred over
    /// `xclick` and `yclick`.
    #[serde(rename = "clicktoshow")]
    click_to_show: Option<ClickToShow>,
    /// Toggle this annotation when clicking a data point whose `x` value is
    /// `xclick` rather than the annotation's `x` value.
    #[serde(rename = "xclick")]
    x_click: Option<NumOrString>,
    /// Toggle this annotation when clicking a data point whose `y` value is
    /// `yclick` rather than the annotation's `y` value.
    #[serde(rename = "yclick")]
    y_click: Option<NumOrString>,
    /// Sets text to appear when hovering over this annotation. If omitted or
    /// blank, no hover label will appear.
    #[serde(rename = "hovertext")]
    hover_text: Option<String>,
    /// Label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Determines whether the annotation text box captures mouse move and click
    /// events, or allows those events to pass through to data points in the
    /// plot that may be behind the annotation. By default `captureevents`
    /// is "false" unless `hovertext` is provided. If you use the event
    /// `plotly_clickannotation` without `hovertext` you must explicitly enable
    /// `captureevents`.
    #[serde(rename = "captureevents")]
    capture_events: Option<bool>,
    /// When used in a template, named items are created in the output figure in
    /// addition to any items the figure already has in this array. You can
    /// modify these items in the output figure by making your own item with
    /// `templateitemname` matching this `name` alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). Has no
    /// effect outside of a template.
    name: Option<String>,
    /// Used to refer to a named item in this array in the template. Named items
    /// from the template will be created even without a matching item in
    /// the input figure, but you can modify one by making an item with
    /// `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is
    /// no template or no matching item, this item will be hidden unless you
    /// explicitly show it with `visible: true`.
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Annotation {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ClickMode {
    Event,
    Select,
    #[serde(rename = "event+select")]
    EventAndSelect,
    None,
}

#[derive(Debug, Clone)]
pub enum DragMode {
    Zoom,
    Pan,
    Select,
    Lasso,
    DrawClosedPath,
    DrawOpenPath,
    DrawLine,
    DrawRect,
    DrawCircle,
    Orbit,
    Turntable,
    False,
}

impl Serialize for DragMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Zoom => serializer.serialize_str("zoom"),
            Self::Pan => serializer.serialize_str("pan"),
            Self::Select => serializer.serialize_str("select"),
            Self::Lasso => serializer.serialize_str("lasso"),
            Self::DrawClosedPath => serializer.serialize_str("drawclosedpath"),
            Self::DrawOpenPath => serializer.serialize_str("drawopenpath"),
            Self::DrawLine => serializer.serialize_str("drawline"),
            Self::DrawRect => serializer.serialize_str("drawrect"),
            Self::DrawCircle => serializer.serialize_str("drawcircle"),
            Self::Orbit => serializer.serialize_str("orbit"),
            Self::Turntable => serializer.serialize_str("turntable"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SelectDirection {
    #[serde(rename = "h")]
    Horizontal,
    #[serde(rename = "v")]
    Vertical,
    #[serde(rename = "d")]
    Diagonal,
    Any,
}

/// Defines the latitude and longitude at which a map will be centered.
#[derive(Serialize, Clone, Debug)]
pub struct Center {
    lat: f64,
    lon: f64,
}

impl Center {
    /// Create a new instance of `Center`.
    ///
    /// `lat` is the number of degrees north, `lon` is the number of degrees
    /// east.
    pub fn new(lat: f64, lon: f64) -> Self {
        Center { lat, lon }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MapboxStyle {
    #[serde(rename = "carto-darkmatter")]
    CartoDarkMatter,
    CartoPositron,
    OpenStreetMap,
    StamenTerrain,
    StamenToner,
    StamenWatercolor,
    WhiteBg,
    Basic,
    Streets,
    Outdoors,
    Light,
    Dark,
    Satellite,
    SatelliteStreets,
}

#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Mapbox {
    /// Sets the mapbox access token to be used for this mapbox map. Note that
    /// `access_token`s are only required when `style` (e.g with values: basic,
    /// streets, outdoors, light, dark, satellite, satellite-streets)
    /// and/or a layout layer references the Mapbox server.
    #[serde(rename = "accesstoken")]
    access_token: Option<String>,
    /// Sets the bearing angle of the map in degrees counter-clockwise from
    /// North.
    bearing: Option<f64>,
    /// Sets the latitude and longitude of the center of the map.
    center: Option<Center>,
    /// Sets the pitch angle of the map in degrees, where `0` means
    /// perpendicular to the surface of the map.
    pitch: Option<f64>,
    /// Sets the style of the map.
    style: Option<MapboxStyle>,
    /// Sets the zoom level of the map.
    zoom: Option<u8>,
}

impl Mapbox {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
/// If "cube", this scene's axes are drawn as a cube, regardless of the axes'
/// ranges. If "data", this scene's axes are drawn in proportion with the axes'
/// ranges. If "manual", this scene's axes are drawn in proportion with the
/// input of "aspectratio" (the default behavior if "aspectratio" is provided).
/// If "auto", this scene's axes are drawn using the results of "data" except
/// when one axis is more than four times the size of the two others, where in
/// that case the results of "cube" are used.
/// Default: "auto"
pub enum AspectMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "cube")]
    Cube,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "manual")]
    Manual,
}

impl Default for AspectMode {
    fn default() -> Self {
        AspectMode::Auto
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets the (x, y, z) components of the 'eye' camera vector. This vector
/// determines the view point about the origin of this scene.
/// Default: {x: 1.25, y: 1.25, z: 1.25}
pub struct Eye {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl Eye {
    pub fn new() -> Self {
        Eye {
            x: Some(1.25),
            y: Some(1.25),
            z: Some(1.25),
        }
    }
}

impl From<(f64, f64, f64)> for Eye {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Eye {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets the (x, y, z) components of the 'up' camera vector. This vector
/// determines the up direction of this scene with respect to the page. The
/// Default: {x: 0, y: 0, z: 1} which means that the z axis points up.
pub struct Up {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl Up {
    pub fn new() -> Self {
        Up {
            x: Some(0.0),
            y: Some(0.0),
            z: Some(1.0),
        }
    }
}

impl From<(f64, f64, f64)> for Up {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Up {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
/// Sets the projection type. The projection type could be either "perspective"
/// or "orthographic".
/// Default: "perspective"
pub enum ProjectionType {
    #[serde(rename = "perspective")]
    Perspective,
    #[serde(rename = "orthographic")]
    Orthographic,
}

impl Default for ProjectionType {
    fn default() -> Self {
        ProjectionType::Perspective
    }
}

impl From<ProjectionType> for Projection {
    fn from(projection_type: ProjectionType) -> Self {
        Projection {
            projection_type: Some(projection_type),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Container for ProjectionType
/// TODO: Implemented according to https://plotly.com/python/reference/layout/scene/#layout-scene-camera-projection -> Complicating it to projection { type: ..} instead of projection: .. really necessary?
pub struct Projection {
    #[serde(rename = "type")]
    projection_type: Option<ProjectionType>,
}

impl Projection {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets the (x, y, z) components of the 'center' camera vector. This vector
/// determines the translation (x, y, z) space about the center of this scene.
/// Default: {x: 0, y: 0, z: 0} which means that the center of the scene is at
/// the origin.
pub struct CameraCenter {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl CameraCenter {
    pub fn new() -> Self {
        CameraCenter {
            x: Some(0.0),
            y: Some(0.0),
            z: Some(0.0),
        }
    }
}

impl From<(f64, f64, f64)> for CameraCenter {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        CameraCenter {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Container for CameraCenter, Eye, Up, and Projection objects. The camera of a
/// 3D scene.
pub struct Camera {
    center: Option<CameraCenter>,
    eye: Option<Eye>,
    up: Option<Up>,
    projection: Option<Projection>,
}

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets this scene's axis aspectratio.
/// x, y, z must be positive.
/// Default: {x: 1, y: 1, z: 1}
pub struct AspectRatio {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl AspectRatio {
    pub fn new() -> Self {
        AspectRatio {
            x: Some(1.0),
            y: Some(1.0),
            z: Some(1.0),
        }
    }
}

impl From<(f64, f64, f64)> for AspectRatio {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        // assert!(x >= 0.0);
        // assert!(y >= 0.0);
        // assert!(z >= 0.0);
        // TODO: panic if any of the values are negative?
        AspectRatio {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// 3D scene layout
pub struct LayoutScene {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    camera: Option<Camera>,
    #[serde(rename = "aspectmode")]
    aspect_mode: Option<AspectMode>,
    #[serde(rename = "aspectratio")]
    aspect_ratio: Option<AspectRatio>,
    #[serde(rename = "xaxis")]
    x_axis: Option<Axis>,
    #[serde(rename = "yaxis")]
    y_axis: Option<Axis>,
    #[serde(rename = "zaxis")]
    z_axis: Option<Axis>,
    #[serde(rename = "dragmode")]
    drag_mode: Option<DragMode>,
    #[serde(rename = "hovermode")]
    hover_mode: Option<HoverMode>,
    annotations: Option<Vec<Annotation>>,
    // domain: Domain,
    // uirevision: Uirevision,
}

impl LayoutScene {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Template {
    layout: Option<LayoutTemplate>,
}

impl Template {
    pub fn new() -> Self {
        Default::default()
    }
}

#[allow(clippy::from_over_into)]
impl Into<Cow<'static, Template>> for Template {
    fn into(self) -> Cow<'static, Template> {
        Cow::Owned(self)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Cow<'static, Template>> for &'static Template {
    fn into(self) -> Cow<'static, Template> {
        Cow::Borrowed(self)
    }
}

// LayoutTemplate matches Layout except it lacks a field for template
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct LayoutTemplate {
    title: Option<Title>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    legend: Option<Legend>,
    margin: Option<Margin>,
    #[serde(rename = "autosize")]
    auto_size: Option<bool>,
    width: Option<usize>,
    height: Option<usize>,
    font: Option<Font>,
    #[serde(rename = "uniformtext")]
    uniform_text: Option<UniformText>,
    separators: Option<String>,
    #[serde(rename = "paper_bgcolor")]
    paper_background_color: Option<Box<dyn Color>>,
    #[serde(rename = "plot_bgcolor")]
    plot_background_color: Option<Box<dyn Color>>,
    #[serde(rename = "colorscale")]
    color_scale: Option<LayoutColorScale>,
    colorway: Option<Vec<Box<dyn Color>>>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<ColorAxis>,
    #[serde(rename = "modebar")]
    mode_bar: Option<ModeBar>,
    /// Determines the mode of hover interactions. If "closest", a single
    /// hoverlabel will appear for the "closest" point within the
    /// `hoverdistance`. If "x" (or "y"), multiple hoverlabels will appear for
    /// multiple points at the "closest" x- (or y-) coordinate within the
    /// `hoverdistance`, with the caveat that no more than one hoverlabel
    /// will appear per trace. If "x unified" (or "y unified"), a single
    /// hoverlabel will appear multiple points at the closest x- (or y-)
    /// coordinate within the `hoverdistance` with the caveat that no more than
    /// one hoverlabel will appear per trace. In this mode, spikelines are
    /// enabled by default perpendicular to the specified axis.
    /// If false, hover interactions are disabled. If `clickmode` includes the
    /// "select" flag, `hovermode` defaults to "closest". If `clickmode`
    /// lacks the "select" flag, it defaults to "x" or "y" (depending on the
    /// trace's `orientation` value) for plots based on cartesian coordinates.
    /// For anything else the default value is "closest".
    #[serde(rename = "hovermode")]
    hover_mode: Option<HoverMode>,
    #[serde(rename = "clickmode")]
    click_mode: Option<ClickMode>,
    #[serde(rename = "dragmode")]
    drag_mode: Option<DragMode>,
    #[serde(rename = "selectdirection")]
    select_direction: Option<SelectDirection>,
    #[serde(rename = "hoverdistance")]
    hover_distance: Option<i32>,
    #[serde(rename = "spikedistance")]
    spike_distance: Option<i32>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,

    grid: Option<LayoutGrid>,
    calendar: Option<Calendar>,

    #[serde(rename = "xaxis")]
    x_axis: Option<Box<Axis>>,
    #[serde(rename = "yaxis")]
    y_axis: Option<Box<Axis>>,
    #[serde(rename = "xaxis2")]
    x_axis2: Option<Box<Axis>>,
    #[serde(rename = "yaxis2")]
    y_axis2: Option<Box<Axis>>,
    #[serde(rename = "xaxis3")]
    x_axis3: Option<Box<Axis>>,
    #[serde(rename = "yaxis3")]
    y_axis3: Option<Box<Axis>>,
    #[serde(rename = "xaxis4")]
    x_axis4: Option<Box<Axis>>,
    #[serde(rename = "yaxis4")]
    y_axis4: Option<Box<Axis>>,
    #[serde(rename = "xaxis5")]
    x_axis5: Option<Box<Axis>>,
    #[serde(rename = "yaxis5")]
    y_axis5: Option<Box<Axis>>,
    #[serde(rename = "xaxis6")]
    x_axis6: Option<Box<Axis>>,
    #[serde(rename = "yaxis6")]
    y_axis6: Option<Box<Axis>>,
    #[serde(rename = "xaxis7")]
    x_axis7: Option<Box<Axis>>,
    #[serde(rename = "yaxis7")]
    y_axis7: Option<Box<Axis>>,
    #[serde(rename = "xaxis8")]
    x_axis8: Option<Box<Axis>>,
    #[serde(rename = "yaxis8")]
    y_axis8: Option<Box<Axis>>,

    // ternary: Option<LayoutTernary>,
    scene: Option<LayoutScene>,
    // polar: Option<LayoutPolar>,
    annotations: Option<Vec<Annotation>>,
    shapes: Option<Vec<Shape>>,
    #[serde(rename = "newshape")]
    new_shape: Option<NewShape>,
    #[serde(rename = "activeshape")]
    active_shape: Option<ActiveShape>,

    #[serde(rename = "boxmode")]
    box_mode: Option<BoxMode>,
    #[serde(rename = "boxgap")]
    box_gap: Option<f64>,
    #[serde(rename = "boxgroupgap")]
    box_group_gap: Option<f64>,

    #[serde(rename = "barmode")]
    bar_mode: Option<BarMode>,
    #[serde(rename = "barnorm")]
    bar_norm: Option<BarNorm>,
    #[serde(rename = "bargap")]
    bar_gap: Option<f64>,
    #[serde(rename = "bargroupgap")]
    bar_group_gap: Option<f64>,

    #[serde(rename = "violinmode")]
    violin_mode: Option<ViolinMode>,
    #[serde(rename = "violingap")]
    violin_gap: Option<f64>,
    #[serde(rename = "violingroupgap")]
    violin_group_gap: Option<f64>,

    #[serde(rename = "waterfallmode")]
    waterfall_mode: Option<WaterfallMode>,
    #[serde(rename = "waterfallgap")]
    waterfall_gap: Option<f64>,
    #[serde(rename = "waterfallgroupgap")]
    waterfall_group_gap: Option<f64>,

    #[serde(rename = "piecolorway")]
    pie_colorway: Option<Vec<Box<dyn Color>>>,
    #[serde(rename = "extendpiecolors")]
    extend_pie_colors: Option<bool>,

    #[serde(rename = "sunburstcolorway")]
    sunburst_colorway: Option<Vec<Box<dyn Color>>>,
    #[serde(rename = "extendsunburstcolors")]
    extend_sunburst_colors: Option<bool>,
}

impl LayoutTemplate {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        if self.annotations.is_none() {
            self.annotations = Some(Vec::new());
        }
        self.annotations.as_mut().unwrap().push(annotation);
    }

    pub fn add_shape(&mut self, shape: Shape) {
        if self.shapes.is_none() {
            self.shapes = Some(Vec::new());
        }
        self.shapes.as_mut().unwrap().push(shape);
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(kind = "layout")]
pub struct Layout {
    title: Option<Title>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    legend: Option<Legend>,
    margin: Option<Margin>,
    #[serde(rename = "autosize")]
    auto_size: Option<bool>,
    width: Option<usize>,
    height: Option<usize>,
    font: Option<Font>,
    #[serde(rename = "uniformtext")]
    uniform_text: Option<UniformText>,
    separators: Option<String>,
    #[serde(rename = "paper_bgcolor")]
    paper_background_color: Option<Box<dyn Color>>,
    #[serde(rename = "plot_bgcolor")]
    plot_background_color: Option<Box<dyn Color>>,
    #[serde(rename = "colorscale")]
    color_scale: Option<LayoutColorScale>,
    colorway: Option<Vec<Box<dyn Color>>>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<ColorAxis>,
    #[serde(rename = "modebar")]
    mode_bar: Option<ModeBar>,
    /// Determines the mode of hover interactions. If "closest", a single
    /// hoverlabel will appear for the "closest" point within the
    /// `hoverdistance`. If "x" (or "y"), multiple hoverlabels will appear for
    /// multiple points at the "closest" x- (or y-) coordinate within the
    /// `hoverdistance`, with the caveat that no more than one hoverlabel
    /// will appear per trace. If "x unified" (or "y unified"), a single
    /// hoverlabel will appear multiple points at the closest x- (or y-)
    /// coordinate within the `hoverdistance` with the caveat that no more than
    /// one hoverlabel will appear per trace. In this mode, spikelines are
    /// enabled by default perpendicular to the specified axis.
    /// If false, hover interactions are disabled. If `clickmode` includes the
    /// "select" flag, `hovermode` defaults to "closest". If `clickmode`
    /// lacks the "select" flag, it defaults to "x" or "y" (depending on the
    /// trace's `orientation` value) for plots based on cartesian coordinates.
    /// For anything else the default value is "closest".
    #[serde(rename = "hovermode")]
    hover_mode: Option<HoverMode>,
    #[serde(rename = "clickmode")]
    click_mode: Option<ClickMode>,
    #[serde(rename = "dragmode")]
    drag_mode: Option<DragMode>,
    #[serde(rename = "selectdirection")]
    select_direction: Option<SelectDirection>,
    #[serde(rename = "hoverdistance")]
    hover_distance: Option<i32>,
    #[serde(rename = "spikedistance")]
    spike_distance: Option<i32>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[field_setter(skip)]
    template: Option<Box<Cow<'static, Template>>>,

    grid: Option<LayoutGrid>,
    calendar: Option<Calendar>,

    #[serde(rename = "xaxis")]
    x_axis: Option<Box<Axis>>,
    #[serde(rename = "yaxis")]
    y_axis: Option<Box<Axis>>,
    #[serde(rename = "zaxis")]
    z_axis: Option<Box<Axis>>,

    #[serde(rename = "xaxis2")]
    x_axis2: Option<Box<Axis>>,
    #[serde(rename = "yaxis2")]
    y_axis2: Option<Box<Axis>>,
    #[serde(rename = "zaxis2")]
    z_axis2: Option<Box<Axis>>,
    #[serde(rename = "xaxis3")]
    x_axis3: Option<Box<Axis>>,
    #[serde(rename = "yaxis3")]
    y_axis3: Option<Box<Axis>>,
    #[serde(rename = "zaxis3")]
    z_axis3: Option<Box<Axis>>,
    #[serde(rename = "xaxis4")]
    x_axis4: Option<Box<Axis>>,
    #[serde(rename = "yaxis4")]
    y_axis4: Option<Box<Axis>>,
    #[serde(rename = "zaxis4")]
    z_axis4: Option<Box<Axis>>,
    #[serde(rename = "xaxis5")]
    x_axis5: Option<Box<Axis>>,
    #[serde(rename = "yaxis5")]
    y_axis5: Option<Box<Axis>>,
    #[serde(rename = "zaxis5")]
    z_axis5: Option<Box<Axis>>,
    #[serde(rename = "xaxis6")]
    x_axis6: Option<Box<Axis>>,
    #[serde(rename = "yaxis6")]
    y_axis6: Option<Box<Axis>>,
    #[serde(rename = "zaxis6")]
    z_axis6: Option<Box<Axis>>,
    #[serde(rename = "xaxis7")]
    x_axis7: Option<Box<Axis>>,
    #[serde(rename = "yaxis7")]
    y_axis7: Option<Box<Axis>>,
    #[serde(rename = "zaxis7")]
    z_axis7: Option<Box<Axis>>,
    #[serde(rename = "xaxis8")]
    x_axis8: Option<Box<Axis>>,
    #[serde(rename = "yaxis8")]
    y_axis8: Option<Box<Axis>>,
    #[serde(rename = "zaxis8")]
    z_axis8: Option<Box<Axis>>,

    // ternary: Option<LayoutTernary>,
    scene: Option<LayoutScene>,
    // polar: Option<LayoutPolar>,
    annotations: Option<Vec<Annotation>>,
    shapes: Option<Vec<Shape>>,
    #[serde(rename = "newshape")]
    new_shape: Option<NewShape>,
    #[serde(rename = "activeshape")]
    active_shape: Option<ActiveShape>,

    #[serde(rename = "boxmode")]
    box_mode: Option<BoxMode>,
    #[serde(rename = "boxgap")]
    box_gap: Option<f64>,
    #[serde(rename = "boxgroupgap")]
    box_group_gap: Option<f64>,

    #[serde(rename = "barmode")]
    bar_mode: Option<BarMode>,
    #[serde(rename = "barnorm")]
    bar_norm: Option<BarNorm>,
    #[serde(rename = "bargap")]
    bar_gap: Option<f64>,
    #[serde(rename = "bargroupgap")]
    bar_group_gap: Option<f64>,

    #[serde(rename = "violinmode")]
    violin_mode: Option<ViolinMode>,
    #[serde(rename = "violingap")]
    violin_gap: Option<f64>,
    #[serde(rename = "violingroupgap")]
    violin_group_gap: Option<f64>,

    #[serde(rename = "waterfallmode")]
    waterfall_mode: Option<WaterfallMode>,
    #[serde(rename = "waterfallgap")]
    waterfall_gap: Option<f64>,
    #[serde(rename = "waterfallgroupgap")]
    waterfall_group_gap: Option<f64>,

    #[serde(rename = "piecolorway")]
    pie_colorway: Option<Vec<Box<dyn Color>>>,
    #[serde(rename = "extendpiecolors")]
    extend_pie_colors: Option<bool>,

    #[serde(rename = "sunburstcolorway")]
    sunburst_colorway: Option<Vec<Box<dyn Color>>>,
    #[serde(rename = "extendsunburstcolors")]
    extend_sunburst_colors: Option<bool>,

    mapbox: Option<Mapbox>,

    #[serde(rename = "updatemenus")]
    update_menus: Option<Vec<UpdateMenu>>,
}

impl Layout {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        if self.annotations.is_none() {
            self.annotations = Some(Vec::new());
        }
        self.annotations.as_mut().unwrap().push(annotation);
    }

    pub fn add_shape(&mut self, shape: Shape) {
        if self.shapes.is_none() {
            self.shapes = Some(Vec::new());
        }
        self.shapes.as_mut().unwrap().push(shape);
    }

    pub fn template<T>(mut self, template: T) -> Layout
    where
        T: Into<Cow<'static, Template>>,
    {
        self.template = Some(Box::new(template.into()));
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    fn test_serialize_uniform_text_mode() {
        assert_eq!(to_value(UniformTextMode::False).unwrap(), json!(false));
        assert_eq!(to_value(UniformTextMode::Hide).unwrap(), json!("hide"));
        assert_eq!(to_value(UniformTextMode::Show).unwrap(), json!("show"));
    }

    #[test]
    fn test_serialize_click_to_show() {
        assert_eq!(to_value(ClickToShow::False).unwrap(), json!(false));
        assert_eq!(to_value(ClickToShow::OnOff).unwrap(), json!("onoff"));
        assert_eq!(to_value(ClickToShow::OnOut).unwrap(), json!("onout"));
    }

    #[test]
    fn test_serialize_hover_mode() {
        assert_eq!(to_value(HoverMode::X).unwrap(), json!("x"));
        assert_eq!(to_value(HoverMode::Y).unwrap(), json!("y"));
        assert_eq!(to_value(HoverMode::Closest).unwrap(), json!("closest"));
        assert_eq!(to_value(HoverMode::False).unwrap(), json!(false));
        assert_eq!(to_value(HoverMode::XUnified).unwrap(), json!("x unified"));
        assert_eq!(to_value(HoverMode::YUnified).unwrap(), json!("y unified"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_axis_type() {
        assert_eq!(to_value(AxisType::Default).unwrap(), json!("-"));
        assert_eq!(to_value(AxisType::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(AxisType::Log).unwrap(), json!("log"));
        assert_eq!(to_value(AxisType::Date).unwrap(), json!("date"));
        assert_eq!(to_value(AxisType::Category).unwrap(), json!("category"));
        assert_eq!(to_value(AxisType::MultiCategory).unwrap(), json!("multicategory"));
    }

    #[test]
    fn test_serialize_axis_constrain() {
        assert_eq!(to_value(AxisConstrain::Range).unwrap(), json!("range"));
        assert_eq!(to_value(AxisConstrain::Domain).unwrap(), json!("domain"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_constrain_direction() {
        assert_eq!(to_value(ConstrainDirection::Left).unwrap(), json!("left"));
        assert_eq!(to_value(ConstrainDirection::Center).unwrap(), json!("center"));
        assert_eq!(to_value(ConstrainDirection::Right).unwrap(), json!("right"));
        assert_eq!(to_value(ConstrainDirection::Top).unwrap(), json!("top"));
        assert_eq!(to_value(ConstrainDirection::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(ConstrainDirection::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_range_mode() {
        assert_eq!(to_value(RangeMode::Normal).unwrap(), json!("normal"));
        assert_eq!(to_value(RangeMode::ToZero).unwrap(), json!("tozero"));
        assert_eq!(to_value(RangeMode::NonNegative).unwrap(), json!("nonnegative"));
    }

    #[test]
    fn test_serialize_ticks_direction() {
        assert_eq!(to_value(TicksDirection::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(TicksDirection::Inside).unwrap(), json!("inside"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_ticks_position() {
        assert_eq!(to_value(TicksPosition::Labels).unwrap(), json!("labels"));
        assert_eq!(to_value(TicksPosition::Boundaries).unwrap(), json!("boundaries"));
    }

    #[test]
    fn test_serialize_array_show() {
        assert_eq!(to_value(ArrayShow::All).unwrap(), json!("all"));
        assert_eq!(to_value(ArrayShow::First).unwrap(), json!("first"));
        assert_eq!(to_value(ArrayShow::Last).unwrap(), json!("last"));
        assert_eq!(to_value(ArrayShow::None).unwrap(), json!("none"));
    }

    #[test]
    fn test_serialize_bar_mode() {
        assert_eq!(to_value(BarMode::Stack).unwrap(), json!("stack"));
        assert_eq!(to_value(BarMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(BarMode::Overlay).unwrap(), json!("overlay"));
        assert_eq!(to_value(BarMode::Relative).unwrap(), json!("relative"));
    }

    #[test]
    fn test_serialize_bar_norm() {
        assert_eq!(to_value(BarNorm::Empty).unwrap(), json!(""));
        assert_eq!(to_value(BarNorm::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(BarNorm::Percent).unwrap(), json!("percent"));
    }

    #[test]
    fn test_serialize_box_mode() {
        assert_eq!(to_value(BoxMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(BoxMode::Overlay).unwrap(), json!("overlay"));
    }

    #[test]
    fn test_serialize_violin_mode() {
        assert_eq!(to_value(ViolinMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(ViolinMode::Overlay).unwrap(), json!("overlay"));
    }

    #[test]
    fn test_serialize_waterfall_mode() {
        assert_eq!(to_value(WaterfallMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(WaterfallMode::Overlay).unwrap(), json!("overlay"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_trace_order() {
        assert_eq!(to_value(TraceOrder::Reversed).unwrap(), json!("reversed"));
        assert_eq!(to_value(TraceOrder::Grouped).unwrap(), json!("grouped"));
        assert_eq!(to_value(TraceOrder::ReversedGrouped).unwrap(), json!("reversed+grouped"));
        assert_eq!(to_value(TraceOrder::Normal).unwrap(), json!("normal"));
    }

    #[test]
    fn test_serialize_item_sizing() {
        assert_eq!(to_value(ItemSizing::Trace).unwrap(), json!("trace"));
        assert_eq!(to_value(ItemSizing::Constant).unwrap(), json!("constant"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_item_click() {
        assert_eq!(to_value(ItemClick::Toggle).unwrap(), json!("toggle"));
        assert_eq!(to_value(ItemClick::ToggleOthers).unwrap(), json!("toggleothers"));
        assert_eq!(to_value(ItemClick::False).unwrap(), json!(false));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_group_click() {
        assert_eq!(to_value(GroupClick::ToggleItem).unwrap(), json!("toggleitem"));
        assert_eq!(to_value(GroupClick::ToggleGroup).unwrap(), json!("togglegroup"));
    }

    #[test]
    fn test_serialize_legend() {
        let legend = Legend::new()
            .background_color("#123123")
            .border_color("#321321")
            .border_width(500)
            .font(Font::new())
            .orientation(Orientation::Vertical)
            .trace_order(TraceOrder::Normal)
            .trace_group_gap(10)
            .item_sizing(ItemSizing::Trace)
            .item_click(ItemClick::Toggle)
            .item_double_click(ItemClick::False)
            .x(1.0)
            .x_anchor(Anchor::Auto)
            .y(2.0)
            .y_anchor(Anchor::Left)
            .valign(VAlign::Middle)
            .title(Title::new("title"))
            .group_click(GroupClick::ToggleItem)
            .item_width(50);

        let expected = json!({
            "bgcolor": "#123123",
            "bordercolor": "#321321",
            "borderwidth": 500,
            "font": {},
            "orientation": "v",
            "traceorder": "normal",
            "tracegroupgap": 10,
            "itemsizing": "trace",
            "itemclick": "toggle",
            "itemdoubleclick": false,
            "x": 1.0,
            "xanchor": "auto",
            "y": 2.0,
            "yanchor": "left",
            "valign": "middle",
            "title": {"text": "title"},
            "groupclick": "toggleitem",
            "itemwidth": 50
        });

        assert_eq!(to_value(legend).unwrap(), expected)
    }

    #[test]
    fn test_serialize_valign() {
        assert_eq!(to_value(VAlign::Top).unwrap(), json!("top"));
        assert_eq!(to_value(VAlign::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(VAlign::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    fn test_serialize_halign() {
        assert_eq!(to_value(HAlign::Left).unwrap(), json!("left"));
        assert_eq!(to_value(HAlign::Center).unwrap(), json!("center"));
        assert_eq!(to_value(HAlign::Right).unwrap(), json!("right"));
    }

    #[test]
    fn test_serialize_margin() {
        let margin = Margin::new()
            .left(1)
            .right(2)
            .top(3)
            .bottom(4)
            .pad(5)
            .auto_expand(true);
        let expected = json!({
            "l": 1,
            "r": 2,
            "t": 3,
            "b": 4,
            "pad": 5,
            "autoexpand": true,
        });

        assert_eq!(to_value(margin).unwrap(), expected);
    }

    #[test]
    fn test_serialize_layout_color_scale() {
        let layout_color_scale = LayoutColorScale::new()
            .sequential(ColorScale::Palette(ColorScalePalette::Greys))
            .sequential_minus(ColorScale::Palette(ColorScalePalette::Blues))
            .diverging(ColorScale::Palette(ColorScalePalette::Hot));
        let expected = json!({
            "sequential": "Greys",
            "sequentialminus": "Blues",
            "diverging": "Hot"
        });

        assert_eq!(to_value(layout_color_scale).unwrap(), expected);
    }

    #[test]
    fn test_serialize_slider_range_mode() {
        assert_eq!(to_value(SliderRangeMode::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(SliderRangeMode::Fixed).unwrap(), json!("fixed"));
        assert_eq!(to_value(SliderRangeMode::Match).unwrap(), json!("match"));
    }

    #[test]
    fn test_serialize_range_slider_y_axis() {
        let range_slider_y_axis = RangeSliderYAxis::new()
            .range_mode(SliderRangeMode::Match)
            .range(vec![0.2]);
        let expected = json!({
            "rangemode": "match",
            "range": [0.2]
        });

        assert_eq!(to_value(range_slider_y_axis).unwrap(), expected);
    }

    #[test]
    fn test_serialize_range_slider() {
        let range_slider = RangeSlider::new()
            .background_color("#123ABC")
            .border_color("#ABC123")
            .border_width(1000)
            .auto_range(false)
            .range(vec![5_i32])
            .thickness(2000.)
            .visible(true)
            .y_axis(RangeSliderYAxis::new());

        let expected = json!({
            "bgcolor": "#123ABC",
            "bordercolor": "#ABC123",
            "borderwidth": 1000,
            "autorange": false,
            "range": [5],
            "thickness": 2000.0,
            "visible": true,
            "yaxis": {}
        });

        assert_eq!(to_value(range_slider).unwrap(), expected);
    }

    #[test]
    fn test_serialize_selector_step() {
        assert_eq!(to_value(SelectorStep::Month).unwrap(), json!("month"));
        assert_eq!(to_value(SelectorStep::Year).unwrap(), json!("year"));
        assert_eq!(to_value(SelectorStep::Day).unwrap(), json!("day"));
        assert_eq!(to_value(SelectorStep::Hour).unwrap(), json!("hour"));
        assert_eq!(to_value(SelectorStep::Minute).unwrap(), json!("minute"));
        assert_eq!(to_value(SelectorStep::Second).unwrap(), json!("second"));
        assert_eq!(to_value(SelectorStep::All).unwrap(), json!("all"));
    }

    #[test]
    fn test_serialize_step_mode() {
        assert_eq!(to_value(StepMode::Backward).unwrap(), json!("backward"));
        assert_eq!(to_value(StepMode::ToDate).unwrap(), json!("todate"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_spike_mode() {
        assert_eq!(to_value(SpikeMode::ToAxis).unwrap(), json!("toaxis"));
        assert_eq!(to_value(SpikeMode::Across).unwrap(), json!("across"));
        assert_eq!(to_value(SpikeMode::Marker).unwrap(), json!("marker"));
        assert_eq!(to_value(SpikeMode::ToaxisAcross).unwrap(), json!("toaxis+across"));
        assert_eq!(to_value(SpikeMode::ToAxisMarker).unwrap(), json!("toaxis+marker"));
        assert_eq!(to_value(SpikeMode::AcrossMarker).unwrap(), json!("across+marker"));
        assert_eq!(to_value(SpikeMode::ToaxisAcrossMarker).unwrap(), json!("toaxis+across+marker"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_spike_snap() {
        assert_eq!(to_value(SpikeSnap::Data).unwrap(), json!("data"));
        assert_eq!(to_value(SpikeSnap::Cursor).unwrap(), json!("cursor"));
        assert_eq!(to_value(SpikeSnap::HoveredData).unwrap(), json!("hovered data"));
    }

    #[test]
    fn test_serialize_selector_button() {
        let selector_button = SelectorButton::new()
            .visible(false)
            .step(SelectorStep::Hour)
            .step_mode(StepMode::ToDate)
            .count(42)
            .label("label")
            .name("name")
            .template_item_name("something");

        let expected = json!({
            "visible": false,
            "step": "hour",
            "stepmode": "todate",
            "count": 42,
            "label": "label",
            "name": "name",
            "templateitemname": "something",
        });

        assert_eq!(to_value(selector_button).unwrap(), expected);
    }

    #[test]
    fn test_serialize_range_selector() {
        let range_selector = RangeSelector::new()
            .visible(true)
            .buttons(vec![SelectorButton::new()])
            .x(2.0)
            .x_anchor(Anchor::Middle)
            .y(4.0)
            .y_anchor(Anchor::Top)
            .font(Font::new())
            .background_color("#123ABC")
            .border_color("#ABC123")
            .border_width(1000)
            .active_color("#888999");

        let expected = json!({
            "visible": true,
            "buttons": [{}],
            "x": 2.0,
            "xanchor": "middle",
            "y": 4.0,
            "yanchor": "top",
            "font": {},
            "bgcolor": "#123ABC",
            "bordercolor": "#ABC123",
            "borderwidth": 1000,
            "activecolor": "#888999",
        });

        assert_eq!(to_value(range_selector).unwrap(), expected);
    }

    #[test]
    fn test_serialize_color_axis() {
        let color_axis = ColorAxis::new()
            .auto_color_scale(false)
            .cauto(true)
            .cmax(1.0)
            .cmid(0.5)
            .cmin(0.0)
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Greens))
            .reverse_scale(false)
            .show_scale(true);

        let expected = json!({
            "autocolorscale": false,
            "cauto": true,
            "cmin": 0.0,
            "cmid": 0.5,
            "cmax": 1.0,
            "colorbar": {},
            "colorscale": "Greens",
            "reversescale": false,
            "showscale": true,
        });

        assert_eq!(to_value(color_axis).unwrap(), expected);
    }

    #[test]
    fn test_serialize_axis() {
        let axis = Axis::new()
            .visible(false)
            .color("#678123")
            .title(Title::new("title"))
            .type_(AxisType::Date)
            .auto_range(false)
            .range_mode(RangeMode::NonNegative)
            .range(vec![2.0])
            .fixed_range(true)
            .constrain(AxisConstrain::Range)
            .constrain_toward(ConstrainDirection::Middle)
            .tick_mode(TickMode::Auto)
            .n_ticks(600)
            .tick0(5.0)
            .dtick(10.0)
            .matches(true)
            .tick_values(vec![1.0, 2.0])
            .tick_text(vec!["one".to_string(), "two".to_string()])
            .ticks(TicksDirection::Inside)
            .ticks_on(TicksPosition::Boundaries)
            .mirror(false)
            .tick_length(77)
            .tick_width(99)
            .tick_color("#101010")
            .show_tick_labels(false)
            .auto_margin(true)
            .show_spikes(false)
            .spike_color("#ABABAB")
            .spike_thickness(501)
            .spike_dash(DashType::DashDot)
            .spike_mode(SpikeMode::AcrossMarker)
            .spike_snap(SpikeSnap::Data)
            .tick_font(Font::new())
            .tick_angle(2.1)
            .tick_prefix("prefix")
            .show_tick_prefix(ArrayShow::Last)
            .tick_suffix("suffix")
            .show_tick_suffix(ArrayShow::None)
            .show_exponent(ArrayShow::All)
            .exponent_format(ExponentFormat::SmallE)
            .separate_thousands(false)
            .tick_format("tickfmt")
            .tick_format_stops(vec![TickFormatStop::new()])
            .hover_format("hoverfmt")
            .show_line(true)
            .line_color("#CCCDDD")
            .line_width(9)
            .show_grid(false)
            .grid_color("#fff000")
            .grid_width(8)
            .zero_line(true)
            .zero_line_color("#f0f0f0")
            .zero_line_width(7)
            .show_dividers(false)
            .divider_color("#AFAFAF")
            .divider_width(55)
            .anchor("anchor")
            .side(AxisSide::Right)
            .overlaying("overlaying")
            .domain(&[0.0, 1.0])
            .position(0.6)
            .range_slider(RangeSlider::new())
            .range_selector(RangeSelector::new())
            .calendar(Calendar::Coptic);

        let expected = json!({
            "visible": false,
            "color": "#678123",
            "title": {"text": "title"},
            "type": "date",
            "autorange": false,
            "rangemode": "nonnegative",
            "range": [2.0],
            "fixedrange": true,
            "constrain": "range",
            "constraintoward": "middle",
            "tickmode": "auto",
            "nticks": 600,
            "tick0": 5.0,
            "dtick": 10.0,
            "matches": "x",
            "tickvals": [1.0, 2.0],
            "ticktext": ["one", "two"],
            "ticks": "inside",
            "tickson": "boundaries",
            "mirror": false,
            "ticklen": 77,
            "tickwidth": 99,
            "tickcolor": "#101010",
            "showticklabels": false,
            "automargin": true,
            "showspikes": false,
            "spikecolor": "#ABABAB",
            "spikethickness": 501,
            "spikedash": "dashdot",
            "spikemode": "across+marker",
            "spikesnap": "data",
            "tickfont": {},
            "tickangle": 2.1,
            "tickprefix": "prefix",
            "showtickprefix": "last",
            "ticksuffix": "suffix",
            "showticksuffix": "none",
            "showexponent": "all",
            "exponentformat": "e",
            "separatethousands": false,
            "tickformat": "tickfmt",
            "tickformatstops": [{"enabled": true}],
            "hoverformat": "hoverfmt",
            "showline": true,
            "linecolor": "#CCCDDD",
            "linewidth": 9,
            "showgrid": false,
            "gridcolor": "#fff000",
            "gridwidth": 8,
            "zeroline": true,
            "zerolinecolor": "#f0f0f0",
            "zerolinewidth": 7,
            "showdividers": false,
            "dividercolor": "#AFAFAF",
            "dividerwidth": 55,
            "anchor": "anchor",
            "side": "right",
            "overlaying": "overlaying",
            "domain": [0.0, 1.0],
            "position": 0.6,
            "rangeslider": {},
            "rangeselector": {},
            "calendar": "coptic",
        });

        assert_eq!(to_value(axis).unwrap(), expected);
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_row_order() {
        assert_eq!(to_value(RowOrder::TopToBottom).unwrap(), json!("top to bottom"));
        assert_eq!(to_value(RowOrder::BottomToTop).unwrap(), json!("bottom to top"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_grid_pattern() {
        assert_eq!(to_value(GridPattern::Independent).unwrap(), json!("independent"));
        assert_eq!(to_value(GridPattern::Coupled).unwrap(), json!("coupled"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_grid_x_side() {
        assert_eq!(to_value(GridXSide::Bottom).unwrap(), json!("bottom"));
        assert_eq!(to_value(GridXSide::BottomPlot).unwrap(), json!("bottom plot"));
        assert_eq!(to_value(GridXSide::Top).unwrap(), json!("top"));
        assert_eq!(to_value(GridXSide::TopPlot).unwrap(), json!("top plot"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_grid_y_side() {
        assert_eq!(to_value(GridYSide::Left).unwrap(), json!("left"));
        assert_eq!(to_value(GridYSide::LeftPlot).unwrap(), json!("left plot"));
        assert_eq!(to_value(GridYSide::Right).unwrap(), json!("right"));
        assert_eq!(to_value(GridYSide::RightPlot).unwrap(), json!("right plot"));
    }

    #[test]
    fn test_serialize_grid_domain() {
        let grid_domain = GridDomain::new().x(vec![0.0]).y(vec![1.0]);
        let expected = json!({
            "x": [0.0],
            "y": [1.0]
        });

        assert_eq!(to_value(grid_domain).unwrap(), expected);
    }

    #[test]
    fn test_serialize_layout_grid() {
        let layout_grid = LayoutGrid::new()
            .rows(224)
            .row_order(RowOrder::BottomToTop)
            .columns(501)
            .sub_plots(vec!["subplots".to_string()])
            .x_axes(vec!["xaxes".to_string()])
            .y_axes(vec!["yaxes".to_string()])
            .pattern(GridPattern::Coupled)
            .x_gap(2.2)
            .y_gap(4.4)
            .domain(GridDomain::new())
            .x_side(GridXSide::Top)
            .y_side(GridYSide::Right);

        let expected = json!({
            "rows": 224,
            "roworder": "bottom to top",
            "columns": 501,
            "subplots": ["subplots"],
            "xaxes": ["xaxes"],
            "yaxes": ["yaxes"],
            "pattern": "coupled",
            "xgap": 2.2,
            "ygap": 4.4,
            "domain": {},
            "xside": "top",
            "yside": "right",
        });

        assert_eq!(to_value(layout_grid).unwrap(), expected);
    }

    #[test]
    fn test_serialize_uniform_text() {
        let uniform_text = UniformText::new().mode(UniformTextMode::Hide).min_size(5);
        let expected = json!({
            "mode": "hide",
            "minsize": 5
        });

        assert_eq!(to_value(uniform_text).unwrap(), expected);
    }

    #[test]
    fn test_serialize_mode_bar() {
        let mode_bar = ModeBar::new()
            .orientation(Orientation::Horizontal)
            .background_color("#FFF000")
            .color("#000FFF")
            .active_color("#AAABBB");
        let expected = json!({
            "orientation": "h",
            "bgcolor": "#FFF000",
            "color": "#000FFF",
            "activecolor": "#AAABBB",
        });

        assert_eq!(to_value(mode_bar).unwrap(), expected);
    }

    #[test]
    fn test_serialize_shape_type() {
        assert_eq!(to_value(ShapeType::Circle).unwrap(), json!("circle"));
        assert_eq!(to_value(ShapeType::Rect).unwrap(), json!("rect"));
        assert_eq!(to_value(ShapeType::Path).unwrap(), json!("path"));
        assert_eq!(to_value(ShapeType::Line).unwrap(), json!("line"));
    }

    #[test]
    fn test_serialize_shape_layer() {
        assert_eq!(to_value(ShapeLayer::Below).unwrap(), json!("below"));
        assert_eq!(to_value(ShapeLayer::Above).unwrap(), json!("above"));
    }

    #[test]
    fn test_serialize_shape_size_mode() {
        assert_eq!(to_value(ShapeSizeMode::Scaled).unwrap(), json!("scaled"));
        assert_eq!(to_value(ShapeSizeMode::Pixel).unwrap(), json!("pixel"));
    }

    #[test]
    fn test_serialize_fill_rule() {
        assert_eq!(to_value(FillRule::EvenOdd).unwrap(), json!("evenodd"));
        assert_eq!(to_value(FillRule::NonZero).unwrap(), json!("nonzero"));
    }

    #[test]
    fn test_serialize_shape_line() {
        let shape_line = ShapeLine::new()
            .color("#000FFF")
            .width(100.)
            .dash(DashType::LongDashDot);
        let expected = json!({
            "color": "#000FFF",
            "width": 100.0,
            "dash": "longdashdot",
        });

        assert_eq!(to_value(shape_line).unwrap(), expected);
    }

    #[test]
    fn test_serialize_shape() {
        let shape = Shape::new()
            .visible(false)
            .shape_type(ShapeType::Circle)
            .layer(ShapeLayer::Above)
            .x_ref("xref")
            .x_size_mode(ShapeSizeMode::Pixel)
            .x_anchor(5)
            .x0(7)
            .x1(8)
            .y_ref("paper")
            .y0(1)
            .y1(2)
            .y_anchor("yanchor")
            .y_size_mode(ShapeSizeMode::Scaled)
            .path("path")
            .opacity(0.2)
            .line(ShapeLine::new())
            .fill_color("#FEFEFE")
            .fill_rule(FillRule::NonZero)
            .editable(true)
            .name("name")
            .template_item_name("templateitemname");

        let expected = json!({
            "visible": false,
            "type": "circle",
            "layer": "above",
            "xref": "xref",
            "xsizemode": "pixel",
            "xanchor": 5,
            "x0": 7,
            "x1": 8,
            "yref": "paper",
            "y0": 1,
            "y1": 2,
            "yanchor": "yanchor",
            "ysizemode": "scaled",
            "path": "path",
            "opacity": 0.2,
            "line": {},
            "fillcolor": "#FEFEFE",
            "fillrule": "nonzero",
            "editable": true,
            "name": "name",
            "templateitemname": "templateitemname"
        });

        assert_eq!(to_value(shape).unwrap(), expected)
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_draw_direction() {
        assert_eq!(to_value(DrawDirection::Ortho).unwrap(), json!("ortho"));
        assert_eq!(to_value(DrawDirection::Horizontal).unwrap(), json!("horizontal"));
        assert_eq!(to_value(DrawDirection::Vertical).unwrap(), json!("vertical"));
        assert_eq!(to_value(DrawDirection::Diagonal).unwrap(), json!("diagonal"));
    }

    #[test]
    fn test_serialize_new_shape() {
        let new_shape = NewShape::new()
            .line(ShapeLine::new())
            .fill_color("#123ABC")
            .fill_rule(FillRule::EvenOdd)
            .opacity(0.02)
            .layer(ShapeLayer::Below)
            .draw_direction(DrawDirection::Ortho);

        let expected = json!({
            "line": {},
            "fillcolor": "#123ABC",
            "fillrule": "evenodd",
            "opacity": 0.02,
            "layer": "below",
            "drawdirection": "ortho",
        });

        assert_eq!(to_value(new_shape).unwrap(), expected)
    }

    #[test]
    fn test_serialize_active_shape() {
        let active_shape = ActiveShape::new().fill_color("#123ABC").opacity(0.02);

        let expected = json!({
            "fillcolor": "#123ABC",
            "opacity": 0.02,
        });

        assert_eq!(to_value(active_shape).unwrap(), expected);
    }

    #[test]
    fn test_serialize_arrow_side() {
        assert_eq!(to_value(ArrowSide::End).unwrap(), json!("end"));
        assert_eq!(to_value(ArrowSide::Start).unwrap(), json!("start"));
        assert_eq!(to_value(ArrowSide::StartEnd).unwrap(), json!("end+start"));
        assert_eq!(to_value(ArrowSide::None).unwrap(), json!("none"));
    }

    #[test]
    fn test_serialize_annotation() {
        let annotation = Annotation::new()
            .align(HAlign::Center)
            .arrow_color("#464646")
            .arrow_head(2)
            .arrow_size(123.4)
            .arrow_side(ArrowSide::End)
            .arrow_width(111.1)
            .ax("ax")
            .ax_ref("axref")
            .ay("ay")
            .ay_ref("ayref")
            .background_color("#123456")
            .border_color("#456789")
            .border_pad(500.)
            .border_width(1000.)
            .capture_events(false)
            .click_to_show(ClickToShow::OnOff)
            .font(Font::new())
            .height(6.)
            .hover_label(Label::new())
            .hover_text("hovertext")
            .name("name")
            .opacity(0.01)
            .show_arrow(false)
            .stand_off(999.9)
            .start_arrow_head(0)
            .start_stand_off(8.8)
            .start_arrow_size(456.7)
            .template_item_name("templateitemname")
            .text("text")
            .text_angle(5.)
            .valign(VAlign::Middle)
            .visible(true)
            .width(4.)
            .x_ref("xref")
            .x("x")
            .x_anchor(Anchor::Auto)
            .x_click("xclick")
            .x_shift(4.0)
            .y_ref("yref")
            .y("y")
            .y_anchor(Anchor::Bottom)
            .y_click("yclick")
            .y_shift(6.3);

        let expected = json!({
            "visible": true,
            "text": "text",
            "textangle": 5.0,
            "font": {},
            "width": 4.0,
            "height": 6.0,
            "opacity": 0.01,
            "align": "center",
            "valign": "middle",
            "bgcolor": "#123456",
            "bordercolor": "#456789",
            "borderpad": 500.0,
            "borderwidth": 1000.0,
            "showarrow": false,
            "arrowcolor": "#464646",
            "arrowhead": 2,
            "startarrowhead": 0,
            "arrowside": "end",
            "arrowsize": 123.4,
            "startarrowsize": 456.7,
            "arrowwidth": 111.1,
            "standoff": 999.9,
            "startstandoff": 8.8,
            "ax": "ax",
            "ay": "ay",
            "x": "x",
            "y": "y",
            "axref": "axref",
            "ayref": "ayref",
            "xref": "xref",
            "yref": "yref",
            "xanchor": "auto",
            "yanchor": "bottom",
            "xshift": 4.0,
            "yshift": 6.3,
            "clicktoshow": "onoff",
            "xclick": "xclick",
            "yclick": "yclick",
            "hovertext": "hovertext",
            "hoverlabel": {},
            "captureevents": false,
            "name": "name",
            "templateitemname": "templateitemname",
        });

        assert_eq!(to_value(annotation).unwrap(), expected);
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_click_mode() {
        assert_eq!(to_value(ClickMode::Event).unwrap(), json!("event"));
        assert_eq!(to_value(ClickMode::Select).unwrap(), json!("select"));
        assert_eq!(to_value(ClickMode::EventAndSelect).unwrap(), json!("event+select"));
        assert_eq!(to_value(ClickMode::None).unwrap(), json!("none"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_drag_mode() {
        assert_eq!(to_value(DragMode::Zoom).unwrap(), json!("zoom"));
        assert_eq!(to_value(DragMode::Pan).unwrap(), json!("pan"));
        assert_eq!(to_value(DragMode::Select).unwrap(), json!("select"));
        assert_eq!(to_value(DragMode::Lasso).unwrap(), json!("lasso"));
        assert_eq!(to_value(DragMode::DrawClosedPath).unwrap(), json!("drawclosedpath"));
        assert_eq!(to_value(DragMode::DrawOpenPath).unwrap(), json!("drawopenpath"));
        assert_eq!(to_value(DragMode::DrawLine).unwrap(), json!("drawline"));
        assert_eq!(to_value(DragMode::DrawRect).unwrap(), json!("drawrect"));
        assert_eq!(to_value(DragMode::DrawCircle).unwrap(), json!("drawcircle"));
        assert_eq!(to_value(DragMode::Orbit).unwrap(), json!("orbit"));
        assert_eq!(to_value(DragMode::Turntable).unwrap(), json!("turntable"));
        assert_eq!(to_value(DragMode::False).unwrap(), json!(false));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_mapbox_style() {
        assert_eq!(to_value(MapboxStyle::CartoDarkMatter).unwrap(), json!("carto-darkmatter"));
        assert_eq!(to_value(MapboxStyle::CartoPositron).unwrap(), json!("carto-positron"));
        assert_eq!(to_value(MapboxStyle::OpenStreetMap).unwrap(), json!("open-street-map"));
        assert_eq!(to_value(MapboxStyle::StamenTerrain).unwrap(), json!("stamen-terrain"));
        assert_eq!(to_value(MapboxStyle::StamenToner).unwrap(), json!("stamen-toner"));
        assert_eq!(to_value(MapboxStyle::StamenWatercolor).unwrap(), json!("stamen-watercolor"));
        assert_eq!(to_value(MapboxStyle::WhiteBg).unwrap(), json!("white-bg"));
        assert_eq!(to_value(MapboxStyle::Basic).unwrap(), json!("basic"));
        assert_eq!(to_value(MapboxStyle::Streets).unwrap(), json!("streets"));
        assert_eq!(to_value(MapboxStyle::Outdoors).unwrap(), json!("outdoors"));
        assert_eq!(to_value(MapboxStyle::Light).unwrap(), json!("light"));
        assert_eq!(to_value(MapboxStyle::Dark).unwrap(), json!("dark"));
        assert_eq!(to_value(MapboxStyle::Satellite).unwrap(), json!("satellite"));
        assert_eq!(to_value(MapboxStyle::SatelliteStreets).unwrap(), json!("satellite-streets"));
    }

    #[test]
    fn test_serialize_select_direction() {
        assert_eq!(to_value(SelectDirection::Horizontal).unwrap(), json!("h"));
        assert_eq!(to_value(SelectDirection::Vertical).unwrap(), json!("v"));
        assert_eq!(to_value(SelectDirection::Diagonal).unwrap(), json!("d"));
        assert_eq!(to_value(SelectDirection::Any).unwrap(), json!("any"));
    }

    #[test]
    fn test_serialize_layout_template() {
        let layout_template = LayoutTemplate::new()
            .title("Title".into())
            .show_legend(false)
            .legend(Legend::new())
            .margin(Margin::new())
            .auto_size(true)
            .width(10)
            .height(20)
            .font(Font::new())
            .uniform_text(UniformText::new())
            .separators("_")
            .paper_background_color("#FFFFFF")
            .plot_background_color("#151515")
            .color_scale(LayoutColorScale::new())
            .colorway(vec!["#123123"])
            .color_axis(ColorAxis::new())
            .mode_bar(ModeBar::new())
            .hover_mode(HoverMode::Closest)
            .click_mode(ClickMode::EventAndSelect)
            .drag_mode(DragMode::Turntable)
            .select_direction(SelectDirection::Diagonal)
            .hover_distance(321)
            .spike_distance(12)
            .hover_label(Label::new())
            .grid(LayoutGrid::new())
            .calendar(Calendar::Jalali)
            .x_axis(Axis::new())
            .x_axis2(Axis::new())
            .x_axis3(Axis::new())
            .x_axis4(Axis::new())
            .x_axis5(Axis::new())
            .x_axis6(Axis::new())
            .x_axis7(Axis::new())
            .x_axis8(Axis::new())
            .y_axis(Axis::new())
            .y_axis2(Axis::new())
            .y_axis3(Axis::new())
            .y_axis4(Axis::new())
            .y_axis5(Axis::new())
            .y_axis6(Axis::new())
            .y_axis7(Axis::new())
            .y_axis8(Axis::new())
            .annotations(vec![Annotation::new()])
            .shapes(vec![Shape::new()])
            .new_shape(NewShape::new())
            .active_shape(ActiveShape::new())
            .box_mode(BoxMode::Group)
            .box_gap(1.)
            .box_group_gap(2.)
            .bar_mode(BarMode::Overlay)
            .bar_norm(BarNorm::Empty)
            .bar_gap(3.)
            .bar_group_gap(4.)
            .violin_mode(ViolinMode::Overlay)
            .violin_gap(5.)
            .violin_group_gap(6.)
            .waterfall_mode(WaterfallMode::Group)
            .waterfall_gap(7.)
            .waterfall_group_gap(8.)
            .pie_colorway(vec!["#789789"])
            .extend_pie_colors(true)
            .sunburst_colorway(vec!["#654654"])
            .extend_sunburst_colors(false);

        let expected = json!({
            "title": {"text": "Title"},
            "showlegend": false,
            "legend": {},
            "margin": {},
            "autosize": true,
            "width": 10,
            "height": 20,
            "font": {},
            "uniformtext": {},
            "separators": "_",
            "paper_bgcolor": "#FFFFFF",
            "plot_bgcolor": "#151515",
            "colorscale": {},
            "colorway": ["#123123"],
            "coloraxis": {},
            "modebar": {},
            "hovermode": "closest",
            "clickmode": "event+select",
            "dragmode": "turntable",
            "selectdirection": "d",
            "hoverdistance": 321,
            "spikedistance": 12,
            "hoverlabel": {},
            "grid": {},
            "calendar": "jalali",
            "xaxis": {},
            "xaxis2": {},
            "xaxis3": {},
            "xaxis4": {},
            "xaxis5": {},
            "xaxis6": {},
            "xaxis7": {},
            "xaxis8": {},
            "yaxis": {},
            "yaxis2": {},
            "yaxis3": {},
            "yaxis4": {},
            "yaxis5": {},
            "yaxis6": {},
            "yaxis7": {},
            "yaxis8": {},
            "annotations": [{}],
            "shapes": [{}],
            "newshape": {},
            "activeshape": {},
            "boxmode": "group",
            "boxgap": 1.0,
            "boxgroupgap": 2.0,
            "barmode": "overlay",
            "barnorm": "",
            "bargap": 3.0,
            "bargroupgap": 4.0,
            "violinmode": "overlay",
            "violingap": 5.0,
            "violingroupgap": 6.0,
            "waterfallmode": "group",
            "waterfallgap": 7.0,
            "waterfallgroupgap": 8.0,
            "piecolorway": ["#789789"],
            "extendpiecolors": true,
            "sunburstcolorway": ["#654654"],
            "extendsunburstcolors": false,
        });

        assert_eq!(to_value(layout_template).unwrap(), expected);
    }

    #[test]
    fn test_serialize_template() {
        let template = Template::new().layout(LayoutTemplate::new());
        let expected = json!({"layout": {}});

        assert_eq!(to_value(template).unwrap(), expected);
    }

    #[test]
    fn test_serialize_layout() {
        let layout = Layout::new()
            .title("Title".into())
            .show_legend(false)
            .legend(Legend::new())
            .margin(Margin::new())
            .auto_size(true)
            .width(10)
            .height(20)
            .font(Font::new())
            .uniform_text(UniformText::new())
            .separators("_")
            .paper_background_color("#FFFFFF")
            .plot_background_color("#151515")
            .color_scale(LayoutColorScale::new())
            .colorway(vec!["#123123"])
            .color_axis(ColorAxis::new())
            .mode_bar(ModeBar::new())
            .hover_mode(HoverMode::Closest)
            .click_mode(ClickMode::EventAndSelect)
            .drag_mode(DragMode::Turntable)
            .select_direction(SelectDirection::Diagonal)
            .hover_distance(321)
            .spike_distance(12)
            .hover_label(Label::new())
            .template(Template::new())
            .grid(LayoutGrid::new())
            .calendar(Calendar::Jalali)
            .x_axis(Axis::new())
            .x_axis2(Axis::new())
            .x_axis3(Axis::new())
            .x_axis4(Axis::new())
            .x_axis5(Axis::new())
            .x_axis6(Axis::new())
            .x_axis7(Axis::new())
            .x_axis8(Axis::new())
            .y_axis(Axis::new())
            .y_axis2(Axis::new())
            .y_axis3(Axis::new())
            .y_axis4(Axis::new())
            .y_axis5(Axis::new())
            .y_axis6(Axis::new())
            .y_axis7(Axis::new())
            .y_axis8(Axis::new())
            .annotations(vec![Annotation::new()])
            .shapes(vec![Shape::new()])
            .new_shape(NewShape::new())
            .active_shape(ActiveShape::new())
            .box_mode(BoxMode::Group)
            .box_gap(1.)
            .box_group_gap(2.)
            .bar_mode(BarMode::Overlay)
            .bar_norm(BarNorm::Empty)
            .bar_gap(3.)
            .bar_group_gap(4.)
            .violin_mode(ViolinMode::Overlay)
            .violin_gap(5.)
            .violin_group_gap(6.)
            .waterfall_mode(WaterfallMode::Group)
            .waterfall_gap(7.)
            .waterfall_group_gap(8.)
            .pie_colorway(vec!["#789789"])
            .extend_pie_colors(true)
            .sunburst_colorway(vec!["#654654"])
            .extend_sunburst_colors(false)
            .z_axis(Axis::new())
            .scene(LayoutScene::new());

        let expected = json!({
            "title": {"text": "Title"},
            "showlegend": false,
            "legend": {},
            "margin": {},
            "autosize": true,
            "width": 10,
            "height": 20,
            "font": {},
            "uniformtext": {},
            "separators": "_",
            "paper_bgcolor": "#FFFFFF",
            "plot_bgcolor": "#151515",
            "colorscale": {},
            "colorway": ["#123123"],
            "coloraxis": {},
            "modebar": {},
            "hovermode": "closest",
            "clickmode": "event+select",
            "dragmode": "turntable",
            "selectdirection": "d",
            "hoverdistance": 321,
            "spikedistance": 12,
            "hoverlabel": {},
            "template": {},
            "grid": {},
            "calendar": "jalali",
            "xaxis": {},
            "xaxis2": {},
            "xaxis3": {},
            "xaxis4": {},
            "xaxis5": {},
            "xaxis6": {},
            "xaxis7": {},
            "xaxis8": {},
            "yaxis": {},
            "yaxis2": {},
            "yaxis3": {},
            "yaxis4": {},
            "yaxis5": {},
            "yaxis6": {},
            "yaxis7": {},
            "yaxis8": {},
            "annotations": [{}],
            "shapes": [{}],
            "newshape": {},
            "activeshape": {},
            "boxmode": "group",
            "boxgap": 1.0,
            "boxgroupgap": 2.0,
            "barmode": "overlay",
            "barnorm": "",
            "bargap": 3.0,
            "bargroupgap": 4.0,
            "violinmode": "overlay",
            "violingap": 5.0,
            "violingroupgap": 6.0,
            "waterfallmode": "group",
            "waterfallgap": 7.0,
            "waterfallgroupgap": 8.0,
            "piecolorway": ["#789789"],
            "extendpiecolors": true,
            "sunburstcolorway": ["#654654"],
            "extendsunburstcolors": false,
            "zaxis": {},
            "scene": {}
        });

        assert_eq!(to_value(layout).unwrap(), expected);
    }

    #[test]
    fn test_serialize_layout_scene() {
        let layout = Layout::new().scene(
            LayoutScene::new()
                .x_axis(Axis::new())
                .y_axis(Axis::new())
                .z_axis(Axis::new())
                .camera(Camera::new())
                .aspect_mode(AspectMode::Auto)
                .hover_mode(HoverMode::Closest)
                .drag_mode(DragMode::Turntable)
                .background_color("#FFFFFF")
                .annotations(vec![Annotation::new()]),
        );

        let expected = json!({
            "scene": {
                "xaxis": {},
                "yaxis": {},
                "zaxis": {},
                "camera": {},
                "aspectmode": "auto",
                "hovermode": "closest",
                "dragmode": "turntable",
                "bgcolor": "#FFFFFF",
                "annotations": [{}],
            }
        });

        assert_eq!(to_value(layout).unwrap(), expected);
    }

    #[test]
    fn test_serialize_eye() {
        let eye = Eye::new();

        assert_eq!(
            to_value(eye).unwrap(),
            json!({
                "x": 1.25,
                "y": 1.25,
                "z": 1.25,
            })
        );

        let eye = Eye::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(eye).unwrap(), expected);

        let eye: Eye = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(eye).unwrap(), expected);
    }

    #[test]
    fn test_serialize_projection() {
        let projection = Projection::new().projection_type(ProjectionType::default());

        let expected = json!({
            "type": "perspective",
        });

        assert_eq!(to_value(projection).unwrap(), expected);

        let projection = Projection::new().projection_type(ProjectionType::Orthographic);

        let expected = json!({
            "type": "orthographic",
        });

        assert_eq!(to_value(projection).unwrap(), expected);

        let projection: Projection = ProjectionType::Orthographic.into();

        assert_eq!(to_value(projection).unwrap(), expected);
    }

    #[test]
    fn test_serialize_camera_center() {
        let camera_center = CameraCenter::new();

        let expected = json!({
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
        });

        assert_eq!(to_value(camera_center).unwrap(), expected);

        let camera_center = CameraCenter::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(camera_center).unwrap(), expected);

        let camera_center: CameraCenter = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(camera_center).unwrap(), expected);
    }

    #[test]
    fn test_serialize_aspect_ratio() {
        let aspect_ratio = AspectRatio::new();

        let expected = json!({
            "x": 1.0,
            "y": 1.0,
            "z": 1.0,
        });

        assert_eq!(to_value(aspect_ratio).unwrap(), expected);

        let aspect_ratio = AspectRatio::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(aspect_ratio).unwrap(), expected);

        let aspect_ratio: AspectRatio = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(aspect_ratio).unwrap(), expected);
    }

    #[test]
    fn test_serialize_aspect_mode() {
        let aspect_mode = AspectMode::default();

        assert_eq!(to_value(aspect_mode).unwrap(), json!("auto"));

        let aspect_mode = AspectMode::Data;

        assert_eq!(to_value(aspect_mode).unwrap(), json!("data"));

        let aspect_mode = AspectMode::Cube;

        assert_eq!(to_value(aspect_mode).unwrap(), json!("cube"));
    }

    #[test]
    fn test_serialize_up() {
        let up = Up::new();

        let expected = json!({
            "x": 0.0,
            "y": 0.0,
            "z": 1.0,
        });

        assert_eq!(to_value(up).unwrap(), expected);

        let up = Up::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(up).unwrap(), expected);

        let up: Up = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(up).unwrap(), expected);
    }
}
