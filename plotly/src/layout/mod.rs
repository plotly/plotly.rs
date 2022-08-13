pub mod themes;

use std::borrow::Cow;

use serde::{Serialize, Serializer};

use crate::color::{Color, ColorArray};
use crate::common::{
    Anchor, AxisSide, Calendar, ColorBar, ColorScale, DashType, ExponentFormat, Font, Label,
    Orientation, TickFormatStop, TickMode, Title,
};
use crate::private::{NumOrString, NumOrStringCollection};

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
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn background_color<C: Color>(mut self, background_color: C) -> Self {
        self.background_color = Some(Box::new(background_color));
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> Self {
        self.border_color = Some(Box::new(border_color));
        self
    }

    pub fn border_width(mut self, border_width: usize) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn trace_order(mut self, trace_order: TraceOrder) -> Self {
        self.trace_order = Some(trace_order);
        self
    }

    pub fn trace_group_gap(mut self, trace_group_gap: usize) -> Self {
        self.trace_group_gap = Some(trace_group_gap);
        self
    }

    pub fn item_sizing(mut self, item_sizing: ItemSizing) -> Self {
        self.item_sizing = Some(item_sizing);
        self
    }

    pub fn item_click(mut self, item_click: ItemClick) -> Self {
        self.item_click = Some(item_click);
        self
    }

    pub fn item_double_click(mut self, item_double_click: ItemClick) -> Self {
        self.item_double_click = Some(item_double_click);
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn x_anchor(mut self, x_anchor: Anchor) -> Self {
        self.x_anchor = Some(x_anchor);
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    pub fn y_anchor(mut self, y_anchor: Anchor) -> Self {
        self.y_anchor = Some(y_anchor);
        self
    }

    pub fn valign(mut self, valign: VAlign) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn group_click(mut self, group_click: GroupClick) -> Self {
        self.group_click = Some(group_click);
        self
    }

    pub fn item_width(mut self, item_width: usize) -> Self {
        self.item_width = Some(item_width);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct Margin {
    l: Option<usize>,
    r: Option<usize>,
    t: Option<usize>,
    b: Option<usize>,
    pad: Option<usize>,
    #[serde(rename = "autoexpand")]
    auto_expand: Option<bool>,
}

impl Margin {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn left(mut self, left: usize) -> Self {
        self.l = Some(left);
        self
    }

    pub fn right(mut self, right: usize) -> Self {
        self.r = Some(right);
        self
    }

    pub fn top(mut self, top: usize) -> Self {
        self.t = Some(top);
        self
    }

    pub fn bottom(mut self, bottom: usize) -> Self {
        self.b = Some(bottom);
        self
    }

    pub fn pad(mut self, pad: usize) -> Self {
        self.pad = Some(pad);
        self
    }

    pub fn auto_expand(mut self, auto_expand: bool) -> Self {
        self.auto_expand = Some(auto_expand);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn sequential(mut self, sequential: ColorScale) -> Self {
        self.sequential = Some(sequential);
        self
    }

    pub fn sequential_minus(mut self, sequential_minus: ColorScale) -> Self {
        self.sequential_minus = Some(sequential_minus);
        self
    }

    pub fn diverging(mut self, diverging: ColorScale) -> Self {
        self.diverging = Some(diverging);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct RangeSliderYAxis {
    #[serde(rename = "rangemode")]
    range_mode: Option<SliderRangeMode>,
    range: Option<NumOrStringCollection>,
}

impl RangeSliderYAxis {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn range_mode(mut self, range_mode: SliderRangeMode) -> Self {
        self.range_mode = Some(range_mode);
        self
    }

    pub fn range<V: Into<NumOrString> + Clone>(mut self, range: Vec<V>) -> Self {
        self.range = Some(range.into());
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn background_color<C: Color>(mut self, background_color: C) -> Self {
        self.background_color = Some(Box::new(background_color));
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> Self {
        self.border_color = Some(Box::new(border_color));
        self
    }

    pub fn border_width(mut self, border_width: u64) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn auto_range(mut self, auto_range: bool) -> Self {
        self.auto_range = Some(auto_range);
        self
    }

    pub fn range<V: Into<NumOrString> + Clone>(mut self, range: Vec<V>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn thickness(mut self, thickness: f64) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn y_axis(mut self, axis: RangeSliderYAxis) -> Self {
        self.y_axis = Some(axis);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn step(mut self, step: SelectorStep) -> Self {
        self.step = Some(step);
        self
    }

    pub fn step_mode(mut self, step_mode: StepMode) -> Self {
        self.step_mode = Some(step_mode);
        self
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_owned());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn template_item_name(mut self, template_item_name: &str) -> Self {
        self.template_item_name = Some(template_item_name.to_owned());
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn buttons(mut self, buttons: Vec<SelectorButton>) -> Self {
        self.buttons = Some(buttons);
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    pub fn x_anchor(mut self, x_anchor: Anchor) -> Self {
        self.x_anchor = Some(x_anchor);
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    pub fn y_anchor(mut self, y_anchor: Anchor) -> Self {
        self.y_anchor = Some(y_anchor);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> Self {
        self.background_color = Some(Box::new(background_color));
        self
    }

    pub fn active_color<C: Color>(mut self, active_color: C) -> Self {
        self.active_color = Some(Box::new(active_color));
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> Self {
        self.border_color = Some(Box::new(border_color));
        self
    }

    pub fn border_width(mut self, border_width: usize) -> Self {
        self.border_width = Some(border_width);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn cauto(mut self, cauto: bool) -> Self {
        self.cauto = Some(cauto);
        self
    }

    pub fn cmin(mut self, cmin: f64) -> Self {
        self.cmin = Some(cmin);
        self
    }

    pub fn cmax(mut self, cmax: f64) -> Self {
        self.cmax = Some(cmax);
        self
    }

    pub fn cmid(mut self, cmid: f64) -> Self {
        self.cmid = Some(cmid);
        self
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Self {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Self {
        self.auto_color_scale = Some(auto_color_scale);
        self
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Self {
        self.reverse_scale = Some(reverse_scale);
        self
    }

    pub fn show_scale(mut self, show_scale: bool) -> Self {
        self.show_scale = Some(show_scale);
        self
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Self {
        self.color_bar = Some(color_bar);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct Axis {
    visible: Option<bool>,
    color: Option<Box<dyn Color>>,
    title: Option<Title>,
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

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn type_(mut self, t: AxisType) -> Self {
        self.r#type = Some(t);
        self
    }

    pub fn auto_range(mut self, auto_range: bool) -> Self {
        self.auto_range = Some(auto_range);
        self
    }

    pub fn range_mode(mut self, range_mode: RangeMode) -> Self {
        self.range_mode = Some(range_mode);
        self
    }

    pub fn range<V: Into<NumOrString> + Clone>(mut self, range: Vec<V>) -> Self {
        self.range = Some(range.into());
        self
    }

    pub fn fixed_range(mut self, fixed_range: bool) -> Self {
        self.fixed_range = Some(fixed_range);
        self
    }

    pub fn constrain(mut self, constrain: AxisConstrain) -> Self {
        self.constrain = Some(constrain);
        self
    }

    pub fn constrain_toward(mut self, constrain_toward: ConstrainDirection) -> Self {
        self.constrain_toward = Some(constrain_toward);
        self
    }

    pub fn tick_mode(mut self, tick_mode: TickMode) -> Self {
        self.tick_mode = Some(tick_mode);
        self
    }

    pub fn n_ticks(mut self, n_ticks: usize) -> Self {
        self.n_ticks = Some(n_ticks);
        self
    }

    pub fn tick0(mut self, tick0: f64) -> Self {
        self.tick0 = Some(tick0);
        self
    }

    pub fn dtick(mut self, dtick: f64) -> Self {
        self.dtick = Some(dtick);
        self
    }

    pub fn tick_values(mut self, tick_values: Vec<f64>) -> Self {
        self.tick_values = Some(tick_values);
        self
    }

    pub fn tick_text(mut self, tick_text: Vec<String>) -> Self {
        self.tick_text = Some(tick_text);
        self
    }

    pub fn ticks(mut self, ticks: TicksDirection) -> Self {
        self.ticks = Some(ticks);
        self
    }

    pub fn ticks_on(mut self, ticks_on: TicksPosition) -> Self {
        self.ticks_on = Some(ticks_on);
        self
    }

    pub fn mirror(mut self, mirror: bool) -> Self {
        self.mirror = Some(mirror);
        self
    }

    pub fn tick_length(mut self, tick_length: usize) -> Self {
        self.tick_length = Some(tick_length);
        self
    }

    pub fn tick_width(mut self, tick_width: usize) -> Self {
        self.tick_width = Some(tick_width);
        self
    }

    pub fn tick_color<C: Color>(mut self, tick_color: C) -> Self {
        self.tick_color = Some(Box::new(tick_color));
        self
    }

    pub fn show_tick_labels(mut self, show_tick_labels: bool) -> Self {
        self.show_tick_labels = Some(show_tick_labels);
        self
    }

    pub fn auto_margin(mut self, auto_margin: bool) -> Self {
        self.auto_margin = Some(auto_margin);
        self
    }

    pub fn show_spikes(mut self, show_spikes: bool) -> Self {
        self.show_spikes = Some(show_spikes);
        self
    }

    pub fn spike_color<C: Color>(mut self, spike_color: C) -> Self {
        self.spike_color = Some(Box::new(spike_color));
        self
    }

    pub fn spike_thickness(mut self, spike_thickness: usize) -> Self {
        self.spike_thickness = Some(spike_thickness);
        self
    }

    pub fn spike_dash(mut self, spike_dash: DashType) -> Self {
        self.spike_dash = Some(spike_dash);
        self
    }

    pub fn spike_mode(mut self, spike_mode: SpikeMode) -> Self {
        self.spike_mode = Some(spike_mode);
        self
    }

    pub fn spike_snap(mut self, spike_snap: SpikeSnap) -> Self {
        self.spike_snap = Some(spike_snap);
        self
    }

    pub fn tick_font(mut self, tick_font: Font) -> Self {
        self.tick_font = Some(tick_font);
        self
    }

    pub fn tick_angle(mut self, tick_angle: f64) -> Self {
        self.tick_angle = Some(tick_angle);
        self
    }

    pub fn tick_prefix(mut self, tick_prefix: &str) -> Self {
        self.tick_prefix = Some(tick_prefix.to_owned());
        self
    }

    pub fn show_tick_prefix(mut self, show_tick_prefix: ArrayShow) -> Self {
        self.show_tick_prefix = Some(show_tick_prefix);
        self
    }

    pub fn tick_suffix(mut self, tick_suffix: &str) -> Self {
        self.tick_suffix = Some(tick_suffix.to_owned());
        self
    }

    pub fn show_tick_suffix(mut self, show_tick_suffix: ArrayShow) -> Self {
        self.show_tick_suffix = Some(show_tick_suffix);
        self
    }

    pub fn show_exponent(mut self, show_exponent: ArrayShow) -> Self {
        self.show_exponent = Some(show_exponent);
        self
    }

    pub fn exponent_format(mut self, exponent_format: ExponentFormat) -> Self {
        self.exponent_format = Some(exponent_format);
        self
    }

    pub fn separate_thousands(mut self, separate_thousands: bool) -> Self {
        self.separate_thousands = Some(separate_thousands);
        self
    }

    pub fn tick_format(mut self, tick_format: &str) -> Self {
        self.tick_format = Some(tick_format.to_owned());
        self
    }

    pub fn tick_format_stops(mut self, tick_format_stops: Vec<TickFormatStop>) -> Self {
        self.tick_format_stops = Some(tick_format_stops);
        self
    }

    pub fn hover_format(mut self, hover_format: &str) -> Self {
        self.hover_format = Some(hover_format.to_owned());
        self
    }

    pub fn show_line(mut self, show_line: bool) -> Self {
        self.show_line = Some(show_line);
        self
    }

    pub fn line_color<C: Color>(mut self, line_color: C) -> Self {
        self.line_color = Some(Box::new(line_color));
        self
    }

    pub fn line_width(mut self, line_width: usize) -> Self {
        self.line_width = Some(line_width);
        self
    }

    pub fn show_grid(mut self, show_grid: bool) -> Self {
        self.show_grid = Some(show_grid);
        self
    }

    pub fn grid_color<C: Color>(mut self, grid_color: C) -> Self {
        self.grid_color = Some(Box::new(grid_color));
        self
    }

    pub fn grid_width(mut self, grid_width: usize) -> Self {
        self.grid_width = Some(grid_width);
        self
    }

    pub fn zero_line(mut self, zero_line: bool) -> Self {
        self.zero_line = Some(zero_line);
        self
    }

    pub fn zero_line_color<C: Color>(mut self, zero_line_color: C) -> Self {
        self.zero_line_color = Some(Box::new(zero_line_color));
        self
    }

    pub fn zero_line_width(mut self, zero_line_width: usize) -> Self {
        self.zero_line_width = Some(zero_line_width);
        self
    }

    pub fn show_dividers(mut self, show_dividers: bool) -> Self {
        self.show_dividers = Some(show_dividers);
        self
    }

    pub fn divider_color<C: Color>(mut self, divider_color: C) -> Self {
        self.divider_color = Some(Box::new(divider_color));
        self
    }

    pub fn divider_width(mut self, divider_width: usize) -> Self {
        self.divider_width = Some(divider_width);
        self
    }

    pub fn anchor(mut self, anchor: &str) -> Self {
        self.anchor = Some(anchor.to_owned());
        self
    }

    pub fn side(mut self, side: AxisSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn overlaying(mut self, overlaying: &str) -> Self {
        self.overlaying = Some(overlaying.to_owned());
        self
    }

    pub fn domain(mut self, domain: &[f64]) -> Self {
        self.domain = Some(domain.to_vec());
        self
    }

    pub fn position(mut self, position: f64) -> Self {
        self.position = Some(position);
        self
    }

    pub fn range_slider(mut self, slider: RangeSlider) -> Self {
        self.range_slider = Some(slider);
        self
    }

    pub fn range_selector(mut self, range_selector: RangeSelector) -> Self {
        self.range_selector = Some(range_selector);
        self
    }

    pub fn calendar(mut self, calendar: Calendar) -> Self {
        self.calendar = Some(calendar);
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct GridDomain {
    x: Option<Vec<f64>>,
    y: Option<Vec<f64>>,
}

impl GridDomain {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn x(mut self, x: Vec<f64>) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: Vec<f64>) -> Self {
        self.y = Some(y);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn row_order(mut self, row_order: RowOrder) -> Self {
        self.row_order = Some(row_order);
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn sub_plots(mut self, sub_plots: Vec<String>) -> Self {
        self.sub_plots = Some(sub_plots);
        self
    }

    pub fn x_axes(mut self, x_axes: Vec<String>) -> Self {
        self.x_axes = Some(x_axes);
        self
    }

    pub fn y_axes(mut self, y_axes: Vec<String>) -> Self {
        self.y_axes = Some(y_axes);
        self
    }

    pub fn pattern(mut self, pattern: GridPattern) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn x_gap(mut self, x_gap: f64) -> Self {
        self.x_gap = Some(x_gap);
        self
    }

    pub fn y_gap(mut self, y_gap: f64) -> Self {
        self.y_gap = Some(y_gap);
        self
    }

    pub fn domain(mut self, domain: GridDomain) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn x_side(mut self, x_side: GridXSide) -> Self {
        self.x_side = Some(x_side);
        self
    }
    pub fn y_side(mut self, y_side: GridYSide) -> Self {
        self.y_side = Some(y_side);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct UniformText {
    mode: Option<UniformTextMode>,
    #[serde(rename = "minsize")]
    min_size: Option<usize>,
}

impl UniformText {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn mode(mut self, mode: UniformTextMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn min_size(mut self, min_size: usize) -> Self {
        self.min_size = Some(min_size);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
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

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> Self {
        self.background_color = Some(Box::new(background_color));
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }

    pub fn active_color<C: Color>(mut self, active_color: C) -> Self {
        self.active_color = Some(Box::new(active_color));
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct ShapeLine {
    color: Option<Box<dyn Color>>,
    width: Option<f64>,
    dash: Option<DashType>,
}

impl ShapeLine {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the line color.
    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Box::new(color));
        self
    }

    /// Sets the line width (in px).
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the dash style of lines. Set to a dash type string ("solid", "dot", "dash", "longdash",
    /// "dashdot", or "longdashdot") or a dash length list in px (eg "5px,10px,2px,2px").
    pub fn dash(mut self, dash: DashType) -> Self {
        self.dash = Some(dash);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct Shape {
    visible: Option<bool>,
    r#type: Option<ShapeType>,
    layer: Option<ShapeLayer>,
    #[serde(rename = "xref")]
    x_ref: Option<String>,
    #[serde(rename = "xsizemode")]
    x_size_mode: Option<ShapeSizeMode>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<NumOrString>,
    x0: Option<NumOrString>,
    x1: Option<NumOrString>,
    #[serde(rename = "yref")]
    y_ref: Option<String>,
    #[serde(rename = "ysizemode")]
    y_size_mode: Option<ShapeSizeMode>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<NumOrString>,
    y0: Option<NumOrString>,
    y1: Option<NumOrString>,
    path: Option<String>,
    opacity: Option<f64>,
    line: Option<ShapeLine>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    #[serde(rename = "fillrule")]
    fill_rule: Option<FillRule>,
    editable: Option<bool>,
    name: Option<String>,
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Shape {
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines whether or not this shape is visible.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// Specifies the shape type to be drawn. If "line", a line is drawn from (`x0`,`y0`) to
    /// (`x1`,`y1`) with respect to the axes' sizing mode. If "circle", a circle is drawn from
    /// ((`x0`+`x1`)/2, (`y0`+`y1`)/2)) with radius (|(`x0`+`x1`)/2 - `x0`|, |(`y0`+`y1`)/2 -`y0`)|)
    /// with respect to the axes' sizing mode. If "rect", a rectangle is drawn linking
    /// (`x0`,`y0`), (`x1`,`y0`), (`x1`,`y1`), (`x0`,`y1`), (`x0`,`y0`) with respect to the axes'
    /// sizing mode. If "path", draw a custom SVG path using `path`. with respect to the axes'
    /// sizing mode.
    pub fn shape_type(mut self, shape_type: ShapeType) -> Self {
        self.r#type = Some(shape_type);
        self
    }

    /// Specifies whether shapes are drawn below or above traces.
    pub fn layer(mut self, layer: ShapeLayer) -> Self {
        self.layer = Some(layer);
        self
    }

    /// Sets the shape's x coordinate axis. If set to an x axis id (e.g. "x" or "x2"), the `x`
    /// position refers to an x coordinate. If set to "paper", the `x` position refers to the
    /// distance from the left side of the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the left (right) side. If the axis `type` is "log", then you must take the
    /// log of your desired range. If the axis `type` is "date", then you must convert the date to
    /// unix time in milliseconds.
    pub fn x_ref(mut self, x_ref: &str) -> Self {
        self.x_ref = Some(x_ref.to_owned());
        self
    }

    /// Sets the shapes's sizing mode along the x axis. If set to "scaled", `x0`, `x1` and x
    /// coordinates within `path` refer to data values on the x axis or a fraction of the plot
    /// area's width (`xref` set to "paper"). If set to "pixel", `xanchor` specifies the x position
    /// in terms of data or plot fraction but `x0`, `x1` and x coordinates within `path` are pixels
    /// relative to `xanchor`. This way, the shape can have a fixed width while maintaining a
    /// position relative to data or plot fraction.
    pub fn x_size_mode(mut self, x_size_mode: ShapeSizeMode) -> Self {
        self.x_size_mode = Some(x_size_mode);
        self
    }

    /// Only relevant in conjunction with `xsizemode` set to "pixel". Specifies the anchor point on
    /// the x axis to which `x0`, `x1` and x coordinates within `path` are relative to. E.g. useful
    /// to attach a pixel sized shape to a certain data value. No effect when `xsizemode` not set
    /// to "pixel".
    pub fn x_anchor<V: Into<NumOrString>>(mut self, x_anchor: V) -> Self {
        self.x_anchor = Some(x_anchor.into());
        self
    }

    /// Sets the shape's starting x position. See `type` and `xsizemode` for more info.
    pub fn x0<V: Into<NumOrString>>(mut self, x0: V) -> Self {
        self.x0 = Some(x0.into());
        self
    }

    /// Sets the shape's end x position. See `type` and `xsizemode` for more info.
    pub fn x1<V: Into<NumOrString>>(mut self, x1: V) -> Self {
        self.x1 = Some(x1.into());
        self
    }

    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g. "y" or "y2"),
    /// the `y` position refers to an y coordinate If set to "paper", the `y` position refers to
    /// the distance from the bottom of the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the bottom (top).
    pub fn y_ref(mut self, y_ref: &str) -> Self {
        self.y_ref = Some(y_ref.to_owned());
        self
    }

    /// Sets the shapes's sizing mode along the y axis. If set to "scaled", `y0`, `y1` and y
    /// coordinates within `path` refer to data values on the y axis or a fraction of the plot
    /// area's height (`yref` set to "paper"). If set to "pixel", `yanchor` specifies the y position
    /// in terms of data or plot fraction but `y0`, `y1` and y coordinates within `path` are pixels
    /// relative to `yanchor`. This way, the shape can have a fixed height while maintaining a
    /// position relative to data or plot fraction.
    pub fn y_size_mode(mut self, y_size_mode: ShapeSizeMode) -> Self {
        self.y_size_mode = Some(y_size_mode);
        self
    }

    /// Only relevant in conjunction with `ysizemode` set to "pixel". Specifies the anchor point on
    /// the y axis to which `y0`, `y1` and y coordinates within `path` are relative to. E.g. useful
    /// to attach a pixel sized shape to a certain data value. No effect when `ysizemode` not set
    /// to "pixel".
    pub fn y_anchor<V: Into<NumOrString>>(mut self, y_anchor: V) -> Self {
        self.y_anchor = Some(y_anchor.into());
        self
    }

    /// Sets the shape's starting y position. See `type` and `ysizemode` for more info.
    pub fn y0<V: Into<NumOrString>>(mut self, y0: V) -> Self {
        self.y0 = Some(y0.into());
        self
    }

    /// Sets the shape's end y position. See `type` and `ysizemode` for more info.
    pub fn y1<V: Into<NumOrString>>(mut self, y1: V) -> Self {
        self.y1 = Some(y1.into());
        self
    }

    /// For `type` "path" - a valid SVG path with the pixel values replaced by data values in
    /// `xsizemode`/`ysizemode` being "scaled" and taken unmodified as pixels relative to
    /// `xanchor` and `yanchor` in case of "pixel" size mode. There are a few restrictions / quirks
    /// only absolute instructions, not relative. So the allowed segments
    /// are: M, L, H, V, Q, C, T, S, and Z arcs (A) are not allowed because radius rx and ry are
    /// relative. In the future we could consider supporting relative commands, but we would have
    /// to decide on how to handle date and log axes. Note that even as is, Q and C Bezier paths
    /// that are smooth on linear axes may not be smooth on log, and vice versa. no chained
    /// "polybezier" commands - specify the segment type for each one. On category axes, values are
    /// numbers scaled to the serial numbers of categories because using the categories themselves
    /// there would be no way to describe fractional positions On data axes: because space and T are
    /// both normal components of path strings, we can't use either to separate date from time parts.
    /// Therefore we'll use underscore for this purpose: 2015-02-21_13:45:56.789
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_owned());
        self
    }

    /// Sets the opacity of the shape. Number between or equal to 0 and 1.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Sets the shape line properties (`color`, `width`, `dash`).
    pub fn line(mut self, line: ShapeLine) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the color filling the shape's interior. Only applies to closed shapes.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Self {
        self.fill_color = Some(Box::new(fill_color));
        self
    }

    /// Determines which regions of complex paths constitute the interior. For more info please
    /// visit https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule
    pub fn fill_rule(mut self, fill_rule: FillRule) -> Self {
        self.fill_rule = Some(fill_rule);
        self
    }

    /// Determines whether the shape could be activated for edit or not. Has no effect when the
    /// older editable shapes mode is enabled via `config.editable` or `config.edits.shapePosition`.
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    /// When used in a template, named items are created in the output figure in addition to any
    /// items the figure already has in this array. You can modify these items in the output figure
    /// by making your own item with `templateitemname` matching this `name` alongside your
    /// modifications (including `visible: false` or `enabled: false` to hide it). Has no effect
    /// outside of a template.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    /// Used to refer to a named item in this array in the template. Named items from the template
    /// will be created even without a matching item in the input figure, but you can modify one
    /// by making an item with `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is no template or no
    /// matching item, this item will be hidden unless you explicitly show it with `visible: true`.
    pub fn template_item_name(mut self, template_item_name: &str) -> Self {
        self.template_item_name = Some(template_item_name.to_owned());
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct NewShape {
    line: Option<ShapeLine>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    #[serde(rename = "fillrule")]
    fill_rule: Option<FillRule>,
    opacity: Option<f64>,
    layer: Option<ShapeLayer>,
    #[serde(rename = "drawdirection")]
    draw_direction: Option<DrawDirection>,
}

impl NewShape {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the shape line properties (`color`, `width`, `dash`).
    pub fn line(mut self, line: ShapeLine) -> Self {
        self.line = Some(line);
        self
    }

    /// Sets the color filling new shapes' interior. Please note that if using a fillcolor with
    /// alpha greater than half, drag inside the active shape starts moving the shape underneath,
    /// otherwise a new shape could be started over.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Self {
        self.fill_color = Some(Box::new(fill_color));
        self
    }

    /// Determines the path's interior. For more info please
    /// visit https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule
    pub fn fill_rule(mut self, fill_rule: FillRule) -> Self {
        self.fill_rule = Some(fill_rule);
        self
    }

    /// Sets the opacity of new shapes. Number between or equal to 0 and 1.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Specifies whether new shapes are drawn below or above traces.
    pub fn layer(mut self, layer: ShapeLayer) -> Self {
        self.layer = Some(layer);
        self
    }

    /// When `dragmode` is set to "drawrect", "drawline" or "drawcircle" this limits the drag to be
    /// horizontal, vertical or diagonal. Using "diagonal" there is no limit e.g. in drawing lines
    /// in any direction. "ortho" limits the draw to be either horizontal or vertical. "horizontal"
    /// allows horizontal extend. "vertical" allows vertical extend.
    pub fn draw_direction(mut self, draw_direction: DrawDirection) -> Self {
        self.draw_direction = Some(draw_direction);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct ActiveShape {
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    opacity: Option<f64>,
}

impl ActiveShape {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the color filling the active shape' interior.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Self {
        self.fill_color = Some(Box::new(fill_color));
        self
    }

    /// Sets the opacity of the active shape. Number between or equal to 0 and 1.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
pub struct Annotation {
    visible: Option<bool>,
    text: Option<String>,
    #[serde(rename = "textangle")]
    text_angle: Option<f64>,
    font: Option<Font>,
    width: Option<f64>,
    height: Option<f64>,
    opacity: Option<f64>,
    align: Option<HAlign>,
    valign: Option<VAlign>,
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    #[serde(rename = "borderpad")]
    border_pad: Option<f64>,
    #[serde(rename = "borderwidth")]
    border_width: Option<f64>,
    #[serde(rename = "showarrow")]
    show_arrow: Option<bool>,
    #[serde(rename = "arrowcolor")]
    arrow_color: Option<Box<dyn Color>>,
    #[serde(rename = "arrowhead")]
    arrow_head: Option<u8>,
    #[serde(rename = "startarrowhead")]
    start_arrow_head: Option<u8>,
    #[serde(rename = "arrowside")]
    arrow_side: Option<ArrowSide>,
    #[serde(rename = "arrowsize")]
    arrow_size: Option<f64>,
    #[serde(rename = "startarrowsize")]
    start_arrow_size: Option<f64>,
    #[serde(rename = "arrowwidth")]
    arrow_width: Option<f64>,
    #[serde(rename = "standoff")]
    stand_off: Option<f64>,
    #[serde(rename = "startstandoff")]
    start_stand_off: Option<f64>,
    ax: Option<NumOrString>,
    ay: Option<NumOrString>,
    #[serde(rename = "axref")]
    ax_ref: Option<String>,
    #[serde(rename = "ayref")]
    ay_ref: Option<String>,
    #[serde(rename = "xref")]
    x_ref: Option<String>,
    x: Option<NumOrString>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(rename = "xshift")]
    x_shift: Option<f64>,
    #[serde(rename = "yref")]
    y_ref: Option<String>,
    y: Option<NumOrString>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    #[serde(rename = "yshift")]
    y_shift: Option<f64>,
    #[serde(rename = "clicktoshow")]
    click_to_show: Option<ClickToShow>,
    #[serde(rename = "xclick")]
    x_click: Option<NumOrString>,
    #[serde(rename = "yclick")]
    y_click: Option<NumOrString>,
    #[serde(rename = "hovertext")]
    hover_text: Option<String>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "captureevents")]
    capture_events: Option<bool>,
    name: Option<String>,
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Annotation {
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines whether or not this annotation is visible.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// Sets the text associated with this annotation. Plotly uses a subset of HTML tags to do
    /// things like newline (<br>), bold (<b></b>), italics (<i></i>), hyperlinks
    /// (<a href='...'></a>). Tags <em>, <sup>, <sub> <span> are also supported.
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_owned());
        self
    }

    /// Sets the angle at which the `text` is drawn with respect to the horizontal.
    pub fn text_angle(mut self, text_angle: f64) -> Self {
        self.text_angle = Some(text_angle);
        self
    }

    /// Sets the annotation text font.
    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets an explicit width for the text box. null (default) lets the text set the box width.
    /// Wider text will be clipped. There is no automatic wrapping; use <br> to start a new line.
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets an explicit height for the text box. null (default) lets the text set the box height.
    /// Taller text will be clipped.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the opacity of the annotation (text + arrow).
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Sets the horizontal alignment of the `text` within the box. Has an effect only if `text`
    /// spans two or more lines (i.e. `text` contains one or more <br> HTML tags) or if an explicit
    /// width is set to override the text width.
    pub fn align(mut self, align: HAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Sets the vertical alignment of the `text` within the box. Has an effect only if an explicit
    /// height is set to override the text height.
    pub fn valign(mut self, valign: VAlign) -> Self {
        self.valign = Some(valign);
        self
    }

    /// Sets the background color of the annotation.
    pub fn background_color<C: Color>(mut self, background_color: C) -> Self {
        self.background_color = Some(Box::new(background_color));
        self
    }

    /// Sets the color of the border enclosing the annotation `text`.
    pub fn border_color<C: Color>(mut self, border_color: C) -> Self {
        self.border_color = Some(Box::new(border_color));
        self
    }

    /// Sets the padding (in px) between the `text` and the enclosing border.
    pub fn border_pad(mut self, border_pad: f64) -> Self {
        self.border_pad = Some(border_pad);
        self
    }

    /// Sets the width (in px) of the border enclosing the annotation `text`.
    pub fn border_width(mut self, border_width: f64) -> Self {
        self.border_width = Some(border_width);
        self
    }

    /// Determines whether or not the annotation is drawn with an arrow. If "True", `text` is
    /// placed near the arrow's tail. If "False", `text` lines up with the `x` and `y` provided.
    pub fn show_arrow(mut self, show_arrow: bool) -> Self {
        self.show_arrow = Some(show_arrow);
        self
    }

    /// Sets the color of the annotation arrow.
    pub fn arrow_color<C: Color>(mut self, arrow_color: C) -> Self {
        self.arrow_color = Some(Box::new(arrow_color));
        self
    }

    /// Sets the end annotation arrow head style. Integer between or equal to 0 and 8.
    pub fn arrow_head(mut self, arrow_head: u8) -> Self {
        self.arrow_head = Some(arrow_head);
        self
    }

    /// Sets the start annotation arrow head style. Integer between or equal to 0 and 8.
    pub fn start_arrow_head(mut self, start_arrow_head: u8) -> Self {
        self.start_arrow_head = Some(start_arrow_head);
        self
    }

    /// Sets the annotation arrow head position.
    pub fn arrow_side(mut self, arrow_side: ArrowSide) -> Self {
        self.arrow_side = Some(arrow_side);
        self
    }

    /// Sets the size of the end annotation arrow head, relative to `arrowwidth`. A value of 1
    /// (default) gives a head about 3x as wide as the line.
    pub fn arrow_size(mut self, arrow_size: f64) -> Self {
        self.arrow_size = Some(arrow_size);
        self
    }

    /// Sets the size of the start annotation arrow head, relative to `arrowwidth`. A value of 1
    /// (default) gives a head about 3x as wide as the line.
    pub fn start_arrow_size(mut self, start_arrow_size: f64) -> Self {
        self.start_arrow_size = Some(start_arrow_size);
        self
    }

    /// Sets the width (in px) of annotation arrow line.
    pub fn arrow_width(mut self, arrow_width: f64) -> Self {
        self.arrow_width = Some(arrow_width);
        self
    }

    /// Sets a distance, in pixels, to move the end arrowhead away from the position it is pointing
    /// at, for example to point at the edge of a marker independent of zoom. Note that this
    /// shortens the arrow from the `ax` / `ay` vector, in contrast to `xshift` / `yshift` which
    /// moves everything by this amount.
    pub fn stand_off(mut self, stand_off: f64) -> Self {
        self.stand_off = Some(stand_off);
        self
    }

    /// Sets a distance, in pixels, to move the start arrowhead away from the position it is
    /// pointing at, for example to point at the edge of a marker independent of zoom. Note that
    /// this shortens the arrow from the `ax` / `ay` vector, in contrast to `xshift` / `yshift`
    /// which moves everything by this amount.
    pub fn start_stand_off(mut self, start_stand_off: f64) -> Self {
        self.start_stand_off = Some(start_stand_off);
        self
    }

    /// Sets the x component of the arrow tail about the arrow head. If `axref` is `pixel`, a
    /// positive (negative) component corresponds to an arrow pointing from right to left (left
    /// to right). If `axref` is an axis, this is an absolute value on that axis, like `x`, NOT a
    /// relative value.
    pub fn ax<V: Into<NumOrString>>(mut self, ax: V) -> Self {
        self.ax = Some(ax.into());
        self
    }

    /// Sets the y component of the arrow tail about the arrow head. If `ayref` is `pixel`, a
    /// positive (negative) component corresponds to an arrow pointing from bottom to top (top to
    /// bottom). If `ayref` is an axis, this is an absolute value on that axis, like `y`, NOT a
    /// relative value.
    pub fn ay<V: Into<NumOrString>>(mut self, ay: V) -> Self {
        self.ay = Some(ay.into());
        self
    }

    /// Indicates in what terms the tail of the annotation (ax,ay) is specified. If `pixel`, `ax`
    /// is a relative offset in pixels from `x`. If set to an x axis id (e.g. "x" or "x2"), `ax` is
    /// specified in the same terms as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    pub fn ax_ref(mut self, ax_ref: &str) -> Self {
        self.ax_ref = Some(ax_ref.to_owned());
        self
    }

    /// Indicates in what terms the tail of the annotation (ax,ay) is specified. If `pixel`, `ay`
    /// is a relative offset in pixels from `y`. If set to a y axis id (e.g. "y" or "y2"), `ay` is
    /// specified in the same terms as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    pub fn ay_ref(mut self, ay_ref: &str) -> Self {
        self.ay_ref = Some(ay_ref.to_owned());
        self
    }

    /// Sets the annotation's x coordinate axis. If set to an x axis id (e.g. "x" or "x2"), the `x`
    /// position refers to an x coordinate If set to "paper", the `x` position refers to the
    /// distance from the left side of the plotting area in normalized coordinates where 0 (1)
    /// corresponds to the left (right) side.
    pub fn x_ref(mut self, x_ref: &str) -> Self {
        self.x_ref = Some(x_ref.to_owned());
        self
    }

    /// Sets the annotation's x position. If the axis `type` is "log", then you must take the log
    /// of your desired range. If the axis `type` is "date", it should be date strings, like date
    /// data, though Date objects and unix milliseconds will be accepted and converted to strings.
    /// If the axis `type` is "category", it should be numbers, using the scale where each category
    /// is assigned a serial number from zero in the order it appears.
    pub fn x<V: Into<NumOrString>>(mut self, x: V) -> Self {
        self.x = Some(x.into());
        self
    }

    /// Sets the text box's horizontal position anchor This anchor binds the `x` position to the
    /// "left", "center" or "right" of the annotation. For example, if `x` is set to 1, `xref` to
    /// "paper" and `xanchor` to "right" then the right-most portion of the annotation lines up with
    /// the right-most edge of the plotting area. If "auto", the anchor is equivalent to "center"
    /// for data-referenced annotations or if there is an arrow, whereas for paper-referenced with
    /// no arrow, the anchor picked corresponds to the closest side.
    pub fn x_anchor(mut self, x_anchor: Anchor) -> Self {
        self.x_anchor = Some(x_anchor);
        self
    }

    /// Shifts the position of the whole annotation and arrow to the right (positive) or left
    /// (negative) by this many pixels.
    pub fn x_shift(mut self, x_shift: f64) -> Self {
        self.x_shift = Some(x_shift);
        self
    }

    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g. "y" or "y2"), the `y`
    /// position refers to an y coordinate If set to "paper", the `y` position refers to the
    /// distance from the bottom of the plotting area in normalized coordinates where 0 (1)
    /// corresponds to the bottom (top).
    pub fn y_ref(mut self, y_ref: &str) -> Self {
        self.y_ref = Some(y_ref.to_owned());
        self
    }

    /// Sets the annotation's y position. If the axis `type` is "log", then you must take the log of
    /// your desired range. If the axis `type` is "date", it should be date strings, like date data,
    /// though Date objects and unix milliseconds will be accepted and converted to strings. If the
    /// axis `type` is "category", it should be numbers, using the scale where each category is
    /// assigned a serial number from zero in the order it appears.
    pub fn y<V: Into<NumOrString>>(mut self, y: V) -> Self {
        self.y = Some(y.into());
        self
    }

    /// Sets the text box's vertical position anchor This anchor binds the `y` position to the
    /// "top", "middle" or "bottom" of the annotation. For example, if `y` is set to 1, `yref` to
    /// "paper" and `yanchor` to "top" then the top-most portion of the annotation lines up with the
    /// top-most edge of the plotting area. If "auto", the anchor is equivalent to "middle" for
    /// data-referenced annotations or if there is an arrow, whereas for paper-referenced with no
    /// arrow, the anchor picked corresponds to the closest side.
    pub fn y_anchor(mut self, y_anchor: Anchor) -> Self {
        self.y_anchor = Some(y_anchor);
        self
    }

    /// Shifts the position of the whole annotation and arrow up (positive) or down (negative) by
    /// this many pixels.
    pub fn y_shift(mut self, y_shift: f64) -> Self {
        self.y_shift = Some(y_shift);
        self
    }

    /// Makes this annotation respond to clicks on the plot. If you click a data point that exactly
    /// matches the `x` and `y` values of this annotation, and it is hidden (visible: false), it
    /// will appear. In "onoff" mode, you must click the same point again to make it disappear, so
    /// if you click multiple points, you can show multiple annotations. In "onout" mode, a click
    /// anywhere else in the plot (on another data point or not) will hide this annotation. If you
    /// need to show/hide this annotation in response to different `x` or `y` values, you can set
    /// `xclick` and/or `yclick`. This is useful for example to label the side of a bar. To label
    /// markers though, `standoff` is preferred over `xclick` and `yclick`.
    pub fn click_to_show(mut self, click_to_show: ClickToShow) -> Self {
        self.click_to_show = Some(click_to_show);
        self
    }

    /// Toggle this annotation when clicking a data point whose `x` value is `xclick` rather than
    /// the annotation's `x` value.
    pub fn x_click<V: Into<NumOrString>>(mut self, x_click: V) -> Self {
        self.x_click = Some(x_click.into());
        self
    }

    /// Toggle this annotation when clicking a data point whose `y` value is `yclick` rather than
    /// the annotation's `y` value.
    pub fn y_click<V: Into<NumOrString>>(mut self, y_click: V) -> Self {
        self.y_click = Some(y_click.into());
        self
    }

    /// Sets text to appear when hovering over this annotation. If omitted or blank, no hover label
    /// will appear.
    pub fn hover_text(mut self, hover_text: &str) -> Self {
        self.hover_text = Some(hover_text.to_owned());
        self
    }

    /// Label displayed on mouse hover.
    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    /// Determines whether the annotation text box captures mouse move and click events, or allows
    /// those events to pass through to data points in the plot that may be behind the annotation.
    /// By default `captureevents` is "false" unless `hovertext` is provided. If you use the event
    /// `plotly_clickannotation` without `hovertext` you must explicitly enable `captureevents`.
    pub fn capture_events(mut self, capture_events: bool) -> Self {
        self.capture_events = Some(capture_events);
        self
    }

    /// When used in a template, named items are created in the output figure in addition to any
    /// items the figure already has in this array. You can modify these items in the output figure
    /// by making your own item with `templateitemname` matching this `name` alongside your
    /// modifications (including `visible: false` or `enabled: false` to hide it). Has no effect
    /// outside of a template.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    /// Used to refer to a named item in this array in the template. Named items from the template
    /// will be created even without a matching item in the input figure, but you can modify one by
    /// making an item with `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is no template or no
    /// matching item, this item will be hidden unless you explicitly show it with `visible: true`.
    pub fn template_item_name(mut self, template_item_name: &str) -> Self {
        self.template_item_name = Some(template_item_name.to_owned());
        self
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

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct Template {
    layout: Option<LayoutTemplate>,
}

impl Template {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn layout(mut self, layout: LayoutTemplate) -> Self {
        self.layout = Some(layout);
        self
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
#[derive(Serialize, Debug, Default, Clone)]
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
    // scene: Option<LayoutScene>,
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

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> Self {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = Some(legend);
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn auto_size(mut self, auto_size: bool) -> Self {
        self.auto_size = Some(auto_size);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn uniform_text(mut self, uniform_text: UniformText) -> Self {
        self.uniform_text = Some(uniform_text);
        self
    }

    pub fn separators(mut self, separators: &str) -> Self {
        self.separators = Some(separators.to_owned());
        self
    }

    pub fn paper_background_color<C: Color>(mut self, paper_background_color: C) -> Self {
        self.paper_background_color = Some(Box::new(paper_background_color));
        self
    }

    pub fn plot_background_color<C: Color>(mut self, plot_background_color: C) -> Self {
        self.plot_background_color = Some(Box::new(plot_background_color));
        self
    }

    pub fn color_scale(mut self, color_scale: LayoutColorScale) -> Self {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn colorway<C: Color>(mut self, colorway: Vec<C>) -> Self {
        self.colorway = Some(ColorArray(colorway).into());
        self
    }

    pub fn color_axis(mut self, color_axis: ColorAxis) -> Self {
        self.color_axis = Some(color_axis);
        self
    }

    pub fn mode_bar(mut self, mode_bar: ModeBar) -> Self {
        self.mode_bar = Some(mode_bar);
        self
    }

    /// Determines the mode of hover interactions. If "closest", a single hoverlabel will appear for the "closest"
    /// point within the `hoverdistance`. If "x" (or "y"), multiple hoverlabels will appear for multiple points at
    /// the "closest" x- (or y-) coordinate within the `hoverdistance`, with the caveat that no more than one hoverlabel
    /// will appear per trace. If "x unified" (or "y unified"), a single hoverlabel will appear multiple points at
    /// the closest x- (or y-) coordinate within the `hoverdistance` with the caveat that no more than one hoverlabel
    /// will appear per trace. In this mode, spikelines are enabled by default perpendicular to the specified axis.
    /// If false, hover interactions are disabled. If `clickmode` includes the "select" flag, `hovermode` defaults to
    /// "closest". If `clickmode` lacks the "select" flag, it defaults to "x" or "y"
    /// (depending on the trace's `orientation` value) for plots based on cartesian coordinates. For anything
    /// else the default value is "closest".
    pub fn hover_mode(mut self, hover_mode: HoverMode) -> Self {
        self.hover_mode = Some(hover_mode);
        self
    }

    pub fn click_mode(mut self, click_mode: ClickMode) -> Self {
        self.click_mode = Some(click_mode);
        self
    }

    pub fn drag_mode(mut self, drag_mode: DragMode) -> Self {
        self.drag_mode = Some(drag_mode);
        self
    }

    pub fn select_direction(mut self, select_direction: SelectDirection) -> Self {
        self.select_direction = Some(select_direction);
        self
    }

    pub fn hover_distance(mut self, hover_distance: i32) -> Self {
        self.hover_distance = Some(hover_distance);
        self
    }

    pub fn spike_distance(mut self, spike_distance: i32) -> Self {
        self.spike_distance = Some(spike_distance);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn grid(mut self, grid: LayoutGrid) -> Self {
        self.grid = Some(grid);
        self
    }

    pub fn calendar(mut self, calendar: Calendar) -> Self {
        self.calendar = Some(calendar);
        self
    }

    pub fn x_axis(mut self, xaxis: Axis) -> Self {
        self.x_axis = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis(mut self, yaxis: Axis) -> Self {
        self.y_axis = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis2(mut self, xaxis: Axis) -> Self {
        self.x_axis2 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis2(mut self, yaxis: Axis) -> Self {
        self.y_axis2 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis3(mut self, xaxis: Axis) -> Self {
        self.x_axis3 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis3(mut self, yaxis: Axis) -> Self {
        self.y_axis3 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis4(mut self, xaxis: Axis) -> Self {
        self.x_axis4 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis4(mut self, yaxis: Axis) -> Self {
        self.y_axis4 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis5(mut self, xaxis: Axis) -> Self {
        self.x_axis5 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis5(mut self, yaxis: Axis) -> Self {
        self.y_axis5 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis6(mut self, xaxis: Axis) -> Self {
        self.x_axis6 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis6(mut self, yaxis: Axis) -> Self {
        self.y_axis6 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis7(mut self, xaxis: Axis) -> Self {
        self.x_axis7 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis7(mut self, yaxis: Axis) -> Self {
        self.y_axis7 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis8(mut self, xaxis: Axis) -> Self {
        self.x_axis8 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis8(mut self, yaxis: Axis) -> Self {
        self.y_axis8 = Some(Box::new(yaxis));
        self
    }

    pub fn annotations(mut self, annotations: Vec<Annotation>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        if self.annotations.is_none() {
            self.annotations = Some(Vec::new());
        }
        self.annotations.as_mut().unwrap().push(annotation);
    }

    pub fn shapes(mut self, shapes: Vec<Shape>) -> Self {
        self.shapes = Some(shapes);
        self
    }

    pub fn add_shape(&mut self, shape: Shape) {
        if self.shapes.is_none() {
            self.shapes = Some(Vec::new());
        }
        self.shapes.as_mut().unwrap().push(shape);
    }

    pub fn new_shape(mut self, new_shape: NewShape) -> Self {
        self.new_shape = Some(new_shape);
        self
    }

    pub fn active_shape(mut self, active_shape: ActiveShape) -> Self {
        self.active_shape = Some(active_shape);
        self
    }

    pub fn box_mode(mut self, box_mode: BoxMode) -> Self {
        self.box_mode = Some(box_mode);
        self
    }

    pub fn box_gap(mut self, box_gap: f64) -> Self {
        self.box_gap = Some(box_gap);
        self
    }

    pub fn box_group_gap(mut self, box_group_gap: f64) -> Self {
        self.box_group_gap = Some(box_group_gap);
        self
    }

    pub fn bar_mode(mut self, bar_mode: BarMode) -> Self {
        self.bar_mode = Some(bar_mode);
        self
    }

    pub fn bar_norm(mut self, bar_norm: BarNorm) -> Self {
        self.bar_norm = Some(bar_norm);
        self
    }

    pub fn bar_gap(mut self, bar_gap: f64) -> Self {
        self.bar_gap = Some(bar_gap);
        self
    }

    pub fn bar_group_gap(mut self, bar_group_gap: f64) -> Self {
        self.bar_group_gap = Some(bar_group_gap);
        self
    }

    pub fn violin_mode(mut self, violin_mode: ViolinMode) -> Self {
        self.violin_mode = Some(violin_mode);
        self
    }

    pub fn violin_gap(mut self, violin_gap: f64) -> Self {
        self.violin_gap = Some(violin_gap);
        self
    }

    pub fn violin_group_gap(mut self, violin_group_gap: f64) -> Self {
        self.violin_group_gap = Some(violin_group_gap);
        self
    }

    pub fn waterfall_mode(mut self, waterfall_mode: WaterfallMode) -> Self {
        self.waterfall_mode = Some(waterfall_mode);
        self
    }

    pub fn waterfall_gap(mut self, waterfall_gap: f64) -> Self {
        self.waterfall_gap = Some(waterfall_gap);
        self
    }

    pub fn waterfall_group_gap(mut self, waterfall_group_gap: f64) -> Self {
        self.waterfall_group_gap = Some(waterfall_group_gap);
        self
    }

    pub fn pie_colorway<C: Color>(mut self, pie_colorway: Vec<C>) -> Self {
        self.pie_colorway = Some(ColorArray(pie_colorway).into());
        self
    }

    pub fn extend_pie_colors(mut self, extend_pie_colors: bool) -> Self {
        self.extend_pie_colors = Some(extend_pie_colors);
        self
    }

    pub fn sunburst_colorway<C: Color>(mut self, sunburst_colorway: Vec<C>) -> Self {
        self.sunburst_colorway = Some(ColorArray(sunburst_colorway).into());
        self
    }

    pub fn extend_sunburst_colors(mut self, extend_sunburst_colors: bool) -> Self {
        self.extend_sunburst_colors = Some(extend_sunburst_colors);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
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
    // scene: Option<LayoutScene>,
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

impl Layout {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> Self {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = Some(legend);
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn auto_size(mut self, auto_size: bool) -> Self {
        self.auto_size = Some(auto_size);
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn uniform_text(mut self, uniform_text: UniformText) -> Self {
        self.uniform_text = Some(uniform_text);
        self
    }

    pub fn separators(mut self, separators: &str) -> Self {
        self.separators = Some(separators.to_owned());
        self
    }

    pub fn paper_background_color<C: Color>(mut self, paper_background_color: C) -> Self {
        self.paper_background_color = Some(Box::new(paper_background_color));
        self
    }

    pub fn plot_background_color<C: Color>(mut self, plot_background_color: C) -> Self {
        self.plot_background_color = Some(Box::new(plot_background_color));
        self
    }

    pub fn color_scale(mut self, color_scale: LayoutColorScale) -> Self {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn colorway<C: Color>(mut self, colorway: Vec<C>) -> Self {
        self.colorway = Some(ColorArray(colorway).into());
        self
    }

    pub fn color_axis(mut self, color_axis: ColorAxis) -> Self {
        self.color_axis = Some(color_axis);
        self
    }

    pub fn mode_bar(mut self, mode_bar: ModeBar) -> Self {
        self.mode_bar = Some(mode_bar);
        self
    }

    /// Determines the mode of hover interactions. If "closest", a single hoverlabel will appear for the "closest"
    /// point within the `hoverdistance`. If "x" (or "y"), multiple hoverlabels will appear for multiple points at
    /// the "closest" x- (or y-) coordinate within the `hoverdistance`, with the caveat that no more than one hoverlabel
    /// will appear per trace. If "x unified" (or "y unified"), a single hoverlabel will appear multiple points at
    /// the closest x- (or y-) coordinate within the `hoverdistance` with the caveat that no more than one hoverlabel
    /// will appear per trace. In this mode, spikelines are enabled by default perpendicular to the specified axis.
    /// If false, hover interactions are disabled. If `clickmode` includes the "select" flag, `hovermode` defaults to
    /// "closest". If `clickmode` lacks the "select" flag, it defaults to "x" or "y"
    /// (depending on the trace's `orientation` value) for plots based on cartesian coordinates. For anything
    /// else the default value is "closest".
    pub fn hover_mode(mut self, hover_mode: HoverMode) -> Self {
        self.hover_mode = Some(hover_mode);
        self
    }

    pub fn click_mode(mut self, click_mode: ClickMode) -> Self {
        self.click_mode = Some(click_mode);
        self
    }

    pub fn drag_mode(mut self, drag_mode: DragMode) -> Self {
        self.drag_mode = Some(drag_mode);
        self
    }

    pub fn select_direction(mut self, select_direction: SelectDirection) -> Self {
        self.select_direction = Some(select_direction);
        self
    }

    pub fn hover_distance(mut self, hover_distance: i32) -> Self {
        self.hover_distance = Some(hover_distance);
        self
    }

    pub fn spike_distance(mut self, spike_distance: i32) -> Self {
        self.spike_distance = Some(spike_distance);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn grid(mut self, grid: LayoutGrid) -> Self {
        self.grid = Some(grid);
        self
    }

    pub fn calendar(mut self, calendar: Calendar) -> Self {
        self.calendar = Some(calendar);
        self
    }

    pub fn x_axis(mut self, xaxis: Axis) -> Self {
        self.x_axis = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis(mut self, yaxis: Axis) -> Self {
        self.y_axis = Some(Box::new(yaxis));
        self
    }

    pub fn z_axis(mut self, zaxis: Axis) -> Layout {
        self.z_axis = Some(Box::new(zaxis));
        self
    }

    pub fn x_axis2(mut self, xaxis: Axis) -> Layout {
        self.x_axis2 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis2(mut self, yaxis: Axis) -> Self {
        self.y_axis2 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis3(mut self, xaxis: Axis) -> Self {
        self.x_axis3 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis3(mut self, yaxis: Axis) -> Self {
        self.y_axis3 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis4(mut self, xaxis: Axis) -> Self {
        self.x_axis4 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis4(mut self, yaxis: Axis) -> Self {
        self.y_axis4 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis5(mut self, xaxis: Axis) -> Self {
        self.x_axis5 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis5(mut self, yaxis: Axis) -> Self {
        self.y_axis5 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis6(mut self, xaxis: Axis) -> Self {
        self.x_axis6 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis6(mut self, yaxis: Axis) -> Self {
        self.y_axis6 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis7(mut self, xaxis: Axis) -> Self {
        self.x_axis7 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis7(mut self, yaxis: Axis) -> Self {
        self.y_axis7 = Some(Box::new(yaxis));
        self
    }

    pub fn x_axis8(mut self, xaxis: Axis) -> Self {
        self.x_axis8 = Some(Box::new(xaxis));
        self
    }

    pub fn y_axis8(mut self, yaxis: Axis) -> Self {
        self.y_axis8 = Some(Box::new(yaxis));
        self
    }

    pub fn annotations(mut self, annotations: Vec<Annotation>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        if self.annotations.is_none() {
            self.annotations = Some(Vec::new());
        }
        self.annotations.as_mut().unwrap().push(annotation);
    }

    pub fn shapes(mut self, shapes: Vec<Shape>) -> Self {
        self.shapes = Some(shapes);
        self
    }

    pub fn add_shape(&mut self, shape: Shape) {
        if self.shapes.is_none() {
            self.shapes = Some(Vec::new());
        }
        self.shapes.as_mut().unwrap().push(shape);
    }

    pub fn new_shape(mut self, new_shape: NewShape) -> Self {
        self.new_shape = Some(new_shape);
        self
    }

    pub fn active_shape(mut self, active_shape: ActiveShape) -> Self {
        self.active_shape = Some(active_shape);
        self
    }

    pub fn template<T>(mut self, template: T) -> Layout
    where
        T: Into<Cow<'static, Template>>,
    {
        self.template = Some(Box::new(template.into()));
        self
    }

    pub fn box_mode(mut self, box_mode: BoxMode) -> Self {
        self.box_mode = Some(box_mode);
        self
    }

    pub fn box_gap(mut self, box_gap: f64) -> Self {
        self.box_gap = Some(box_gap);
        self
    }

    pub fn box_group_gap(mut self, box_group_gap: f64) -> Self {
        self.box_group_gap = Some(box_group_gap);
        self
    }

    pub fn bar_mode(mut self, bar_mode: BarMode) -> Self {
        self.bar_mode = Some(bar_mode);
        self
    }

    pub fn bar_norm(mut self, bar_norm: BarNorm) -> Self {
        self.bar_norm = Some(bar_norm);
        self
    }

    pub fn bar_gap(mut self, bar_gap: f64) -> Self {
        self.bar_gap = Some(bar_gap);
        self
    }

    pub fn bar_group_gap(mut self, bar_group_gap: f64) -> Self {
        self.bar_group_gap = Some(bar_group_gap);
        self
    }

    pub fn violin_mode(mut self, violin_mode: ViolinMode) -> Self {
        self.violin_mode = Some(violin_mode);
        self
    }

    pub fn violin_gap(mut self, violin_gap: f64) -> Self {
        self.violin_gap = Some(violin_gap);
        self
    }

    pub fn violin_group_gap(mut self, violin_group_gap: f64) -> Self {
        self.violin_group_gap = Some(violin_group_gap);
        self
    }

    pub fn waterfall_mode(mut self, waterfall_mode: WaterfallMode) -> Self {
        self.waterfall_mode = Some(waterfall_mode);
        self
    }

    pub fn waterfall_gap(mut self, waterfall_gap: f64) -> Self {
        self.waterfall_gap = Some(waterfall_gap);
        self
    }

    pub fn waterfall_group_gap(mut self, waterfall_group_gap: f64) -> Self {
        self.waterfall_group_gap = Some(waterfall_group_gap);
        self
    }

    pub fn pie_colorway<C: Color>(mut self, pie_colorway: Vec<C>) -> Self {
        self.pie_colorway = Some(ColorArray(pie_colorway).into());
        self
    }

    pub fn extend_pie_colors(mut self, extend_pie_colors: bool) -> Self {
        self.extend_pie_colors = Some(extend_pie_colors);
        self
    }

    pub fn sunburst_colorway<C: Color>(mut self, sunburst_colorway: Vec<C>) -> Self {
        self.sunburst_colorway = Some(ColorArray(sunburst_colorway).into());
        self
    }

    pub fn extend_sunburst_colors(mut self, extend_sunburst_colors: bool) -> Self {
        self.extend_sunburst_colors = Some(extend_sunburst_colors);
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
            .cauto(true)
            .cmin(0.0)
            .cmid(0.5)
            .cmax(1.0)
            .color_scale(ColorScale::Palette(ColorScalePalette::Greens))
            .auto_color_scale(false)
            .reverse_scale(false)
            .show_scale(true);
        // .color_bar(); Awaiting fix from other branch

        let expected = json!({
            "cauto": true,
            "cmin": 0.0,
            "cmid": 0.5,
            "cmax": 1.0,
            "colorscale": "Greens",
            "autocolorscale": false,
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
            .visible(true)
            .text("text")
            .text_angle(5.)
            .font(Font::new())
            .width(4.)
            .height(6.)
            .opacity(0.01)
            .align(HAlign::Center)
            .valign(VAlign::Middle)
            .background_color("#123456")
            .border_color("#456789")
            .border_pad(500.)
            .border_width(1000.)
            .show_arrow(false)
            .arrow_color("#464646")
            .arrow_head(2)
            .start_arrow_head(0)
            .arrow_size(123.4)
            .start_arrow_size(456.7)
            .arrow_width(111.1)
            .stand_off(999.9)
            .start_stand_off(8.8)
            .ax("ax")
            .ay("ay")
            .ax_ref("axref")
            .ay_ref("ayref")
            .x_ref("xref")
            .x("x")
            .x_anchor(Anchor::Auto)
            .x_shift(4.0)
            .y_ref("yref")
            .y("y")
            .y_anchor(Anchor::Bottom)
            .y_shift(6.3)
            .click_to_show(ClickToShow::OnOff)
            .x_click("xclick")
            .y_click("yclick")
            .hover_text("hovertext")
            .hover_label(Label::new())
            .capture_events(false)
            .name("name")
            .template_item_name("templateitemname");

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
        });

        assert_eq!(to_value(layout).unwrap(), expected);
    }
}
