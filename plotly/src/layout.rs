use crate::common::color::{Color, ColorWrapper};
use crate::common::{
    Anchor, Calendar, ColorBar, ColorScale, DashType, Font, Label, Orientation, Side,
    TickFormatStop, TickMode, Title,
};
use crate::plot::Trace;
use crate::private;
use crate::private::{to_num_or_string_wrapper, NumOrString, NumOrStringWrapper, TruthyEnum};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum AxisType {
    #[serde(rename = "-")]
    Default,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "multicategory")]
    MultiCategory,
}

#[derive(Serialize, Debug)]
pub enum AxisConstrain {
    #[serde(rename = "range")]
    Range,
    #[serde(rename = "domain")]
    Domain,
}

#[derive(Serialize, Debug)]
pub enum ConstrainDirection {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "bottom")]
    Bottom,
}

#[derive(Serialize, Debug)]
pub enum RangeMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "tozero")]
    ToZero,
    #[serde(rename = "nonnegative")]
    NonNegative,
}

#[derive(Serialize, Debug)]
pub enum TicksDirection {
    #[serde(rename = "outside")]
    Outside,
    #[serde(rename = "inside")]
    Inside,
}

#[derive(Serialize, Debug)]
pub enum TicksPosition {
    #[serde(rename = "labels")]
    Labels,
    #[serde(rename = "boundaries")]
    Boundaries,
}

#[derive(Serialize, Debug)]
pub enum ArrayShow {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "first")]
    First,
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Debug)]
pub enum BarMode {
    #[serde(rename = "stack")]
    Stack,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "overlay")]
    Overlay,
    #[serde(rename = "relative")]
    Relative,
}

#[derive(Serialize, Debug)]
pub enum BarNorm {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "fraction")]
    Fraction,
    #[serde(rename = "percent")]
    Percent,
}

#[derive(Serialize, Debug)]
pub enum BoxMode {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "overlay")]
    Overlay,
}

#[derive(Serialize, Debug)]
pub enum ViolinMode {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "overlay")]
    Overlay,
}

#[derive(Serialize, Debug)]
pub enum WaterfallMode {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "overlay")]
    Overlay,
}

#[derive(Serialize, Debug, Default)]
pub struct Legend {
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderwidth")]
    border_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceorder")]
    trace_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tracegroupgap")]
    trace_group_gap: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "itemsizing")]
    item_sizing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "itemclick")]
    item_click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "itemdoubleclick")]
    item_double_click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yanchor")]
    y_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valign: Option<VAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
}

impl Legend {
    pub fn new() -> Legend {
        Default::default()
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> Legend {
        self.background_color = Some(background_color.to_color());
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> Legend {
        self.border_color = Some(border_color.to_color());
        self
    }

    pub fn border_width(mut self, border_width: usize) -> Legend {
        self.border_width = Some(border_width);
        self
    }

    pub fn font(mut self, font: Font) -> Legend {
        self.font = Some(font);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Legend {
        self.orientation = Some(orientation);
        self
    }

    pub fn trace_order(mut self, trace_order: &str) -> Legend {
        self.trace_order = Some(trace_order.to_owned());
        self
    }

    pub fn trace_group_gap(mut self, trace_group_gap: usize) -> Legend {
        self.trace_group_gap = Some(trace_group_gap);
        self
    }

    pub fn item_sizing(mut self, item_sizing: &str) -> Legend {
        self.item_sizing = Some(item_sizing.to_owned());
        self
    }

    pub fn item_click(mut self, item_click: &str) -> Legend {
        self.item_click = Some(item_click.to_owned());
        self
    }

    pub fn item_double_click(mut self, item_double_click: &str) -> Legend {
        self.item_double_click = Some(item_double_click.to_owned());
        self
    }

    pub fn x(mut self, x: f64) -> Legend {
        self.x = Some(x);
        self
    }

    pub fn x_anchor(mut self, x_anchor: Anchor) -> Legend {
        self.x_anchor = Some(x_anchor);
        self
    }

    pub fn y(mut self, y: f64) -> Legend {
        self.y = Some(y);
        self
    }

    pub fn y_anchor(mut self, y_anchor: Anchor) -> Legend {
        self.y_anchor = Some(y_anchor);
        self
    }

    pub fn valign(mut self, valign: VAlign) -> Legend {
        self.valign = Some(valign);
        self
    }

    pub fn title(mut self, title: Title) -> Legend {
        self.title = Some(title);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum VAlign {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "bottom")]
    Bottom,
}

#[derive(Serialize, Debug)]
pub enum HAlign {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
}

#[derive(Serialize, Debug, Default)]
pub struct Margin {
    #[serde(skip_serializing_if = "Option::is_none")]
    l: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    t: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pad: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autoexpand")]
    auto_expand: Option<bool>,
}

impl Margin {
    pub fn new() -> Margin {
        Default::default()
    }

    pub fn left(mut self, left: usize) -> Margin {
        self.l = Some(left);
        self
    }

    pub fn right(mut self, right: usize) -> Margin {
        self.r = Some(right);
        self
    }

    pub fn top(mut self, top: usize) -> Margin {
        self.t = Some(top);
        self
    }

    pub fn bottom(mut self, bottom: usize) -> Margin {
        self.b = Some(bottom);
        self
    }

    pub fn pad(mut self, pad: usize) -> Margin {
        self.pad = Some(pad);
        self
    }

    pub fn auto_expand(mut self, auto_expand: bool) -> Margin {
        self.auto_expand = Some(auto_expand);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct LayoutColorScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    sequential: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sequentialminus")]
    sequential_minus: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diverging: Option<ColorScale>,
}

impl LayoutColorScale {
    pub fn new() -> LayoutColorScale {
        Default::default()
    }

    pub fn sequential(mut self, sequential: ColorScale) -> LayoutColorScale {
        self.sequential = Some(sequential);
        self
    }

    pub fn sequential_minus(mut self, sequential_minus: ColorScale) -> LayoutColorScale {
        self.sequential_minus = Some(sequential_minus);
        self
    }

    pub fn diverging(mut self, diverging: ColorScale) -> LayoutColorScale {
        self.diverging = Some(diverging);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum SliderRangeMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "match")]
    Match,
}

#[derive(Serialize, Debug, Default)]
pub struct RangeSliderYAxis {
    #[serde(skip_serializing_if = "Option::is_none", rename = "rangemode")]
    range_mode: Option<SliderRangeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<NumOrStringWrapper>>,
}

impl RangeSliderYAxis {
    pub fn new() -> RangeSliderYAxis {
        Default::default()
    }

    pub fn range_mode(mut self, range_mode: SliderRangeMode) -> RangeSliderYAxis {
        self.range_mode = Some(range_mode);
        self
    }

    pub fn range<C: NumOrString>(mut self, range: Vec<C>) -> RangeSliderYAxis {
        let wrapped = to_num_or_string_wrapper(range);
        self.range = Some(wrapped);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct RangeSlider {
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderwidth")]
    border_width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autorange")]
    auto_range: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<NumOrStringWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thickness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<RangeSliderYAxis>,
}

impl RangeSlider {
    pub fn new() -> RangeSlider {
        Default::default()
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> RangeSlider {
        self.background_color = Some(background_color.to_color());
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> RangeSlider {
        self.border_color = Some(border_color.to_color());
        self
    }

    pub fn border_width(mut self, border_width: u64) -> RangeSlider {
        self.border_width = Some(border_width);
        self
    }

    pub fn auto_range(mut self, auto_range: bool) -> RangeSlider {
        self.auto_range = Some(auto_range);
        self
    }

    pub fn range<C: NumOrString>(mut self, range: Vec<C>) -> RangeSlider {
        let wrapped = to_num_or_string_wrapper(range);
        self.range = Some(wrapped);
        self
    }

    pub fn thickness(mut self, thickness: f64) -> RangeSlider {
        self.thickness = Some(thickness);
        self
    }

    pub fn visible(mut self, visible: bool) -> RangeSlider {
        self.visible = Some(visible);
        self
    }

    pub fn y_axis(mut self, axis: RangeSliderYAxis) -> RangeSlider {
        self.y_axis = Some(axis);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum SelectorStep {
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "all")]
    All,
}

#[derive(Serialize, Debug)]
pub enum StepMode {
    #[serde(rename = "backward")]
    Backward,
    #[serde(rename = "todate")]
    ToDate,
}

#[derive(Serialize, Debug, Default)]
pub struct SelectorButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<SelectorStep>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stepmode")]
    step_mode: Option<StepMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl SelectorButton {
    pub fn new() -> SelectorButton {
        Default::default()
    }

    pub fn visible(mut self, visible: bool) -> SelectorButton {
        self.visible = Some(visible);
        self
    }

    pub fn step(mut self, step: SelectorStep) -> SelectorButton {
        self.step = Some(step);
        self
    }

    pub fn step_mode(mut self, step_mode: StepMode) -> SelectorButton {
        self.step_mode = Some(step_mode);
        self
    }

    pub fn count(mut self, count: usize) -> SelectorButton {
        self.count = Some(count);
        self
    }

    pub fn label(mut self, label: &str) -> SelectorButton {
        self.label = Some(label.to_owned());
        self
    }

    pub fn name(mut self, name: &str) -> SelectorButton {
        self.name = Some(name.to_owned());
        self
    }

    pub fn template_item_name(mut self, template_item_name: &str) -> SelectorButton {
        self.template_item_name = Some(template_item_name.to_owned());
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct RangeSelector {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<SelectorButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yanchor")]
    y_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activecolor")]
    active_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderwidth")]
    border_width: Option<usize>,
}

impl RangeSelector {
    pub fn new() -> RangeSelector {
        Default::default()
    }

    pub fn visible(mut self, visible: bool) -> RangeSelector {
        self.visible = Some(visible);
        self
    }

    pub fn buttons(mut self, buttons: Vec<SelectorButton>) -> RangeSelector {
        self.buttons = Some(buttons);
        self
    }

    pub fn x(mut self, x: f64) -> RangeSelector {
        self.x = Some(x);
        self
    }

    pub fn x_anchor(mut self, x_anchor: Anchor) -> RangeSelector {
        self.x_anchor = Some(x_anchor);
        self
    }

    pub fn y(mut self, y: f64) -> RangeSelector {
        self.y = Some(y);
        self
    }

    pub fn y_anchor(mut self, y_anchor: Anchor) -> RangeSelector {
        self.y_anchor = Some(y_anchor);
        self
    }

    pub fn font(mut self, font: Font) -> RangeSelector {
        self.font = Some(font);
        self
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> RangeSelector {
        self.background_color = Some(background_color.to_color());
        self
    }

    pub fn active_color<C: Color>(mut self, active_color: C) -> RangeSelector {
        self.background_color = Some(active_color.to_color());
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> RangeSelector {
        self.border_color = Some(border_color.to_color());
        self
    }

    pub fn border_width(mut self, border_width: usize) -> RangeSelector {
        self.border_width = Some(border_width);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct ColorAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    cauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmid: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
}

impl ColorAxis {
    pub fn new() -> ColorAxis {
        Default::default()
    }

    pub fn cauto(mut self, cauto: bool) -> ColorAxis {
        self.cauto = Some(cauto);
        self
    }

    pub fn cmin(mut self, cmin: f64) -> ColorAxis {
        self.cmin = Some(cmin);
        self
    }

    pub fn cmax(mut self, cmax: f64) -> ColorAxis {
        self.cmax = Some(cmax);
        self
    }

    pub fn cmid(mut self, cmid: f64) -> ColorAxis {
        self.cmid = Some(cmid);
        self
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> ColorAxis {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> ColorAxis {
        self.auto_color_scale = Some(auto_color_scale);
        self
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> ColorAxis {
        self.reverse_scale = Some(reverse_scale);
        self
    }

    pub fn show_scale(mut self, show_scale: bool) -> ColorAxis {
        self.show_scale = Some(show_scale);
        self
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> ColorAxis {
        self.color_bar = Some(color_bar);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct Axis {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<AxisType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "auto_range")]
    auto_range: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rangemode")]
    range_mode: Option<RangeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<NumOrStringWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fixedrange")]
    fixed_range: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    constrain: Option<AxisConstrain>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "constraintoward")]
    constrain_toward: Option<ConstrainDirection>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickmode")]
    tick_mode: Option<TickMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nticks")]
    n_ticks: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tick0: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtick: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "tickvals")]
    tick_values: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tick_text")]
    tick_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ticks: Option<TicksDirection>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickson")]
    ticks_on: Option<TicksPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mirror: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ticklen")]
    tick_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickwidth")]
    tick_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickcolor")]
    tick_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "automargin")]
    auto_margin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showspikes")]
    show_spikes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikecolor")]
    spike_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikethickness")]
    spike_thickness: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikedash")]
    spike_dash: Option<DashType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikemode")]
    spike_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikesnap")]
    spike_snap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showtickprefix")]
    show_tick_prefix: Option<ArrayShow>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showticksuffix")]
    show_tick_suffix: Option<ArrayShow>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showexponent")]
    show_exponent: Option<ArrayShow>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exponentformat")]
    exponent_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "separatethousands")]
    separate_thousands: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickformatstops")]
    tick_format_stops: Option<Vec<TickFormatStop>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverformat")]
    hover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showline")]
    show_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "linecolor")]
    line_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "linewidth")]
    line_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridcolor")]
    grid_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridwidth")]
    grid_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zeroline")]
    zero_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zerolinecolor")]
    zero_line_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zerolinewidth")]
    zero_line_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showdividers")]
    show_dividers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dividercolor")]
    divider_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dividerwidth")]
    divider_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overlaying: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rangeslider")]
    range_slider: Option<RangeSlider>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rangeselector")]
    range_selector: Option<RangeSelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar: Option<Calendar>,
}

impl Axis {
    pub fn new() -> Axis {
        Default::default()
    }

    pub fn matches(mut self, matches : bool) -> Axis {
        if matches {
            self.matches = Some(String::from("x"));
        }
        self
    }

    pub fn visible(mut self, visible: bool) -> Axis {
        self.visible = Some(visible);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Axis {
        self.color = Some(color.to_color());
        self
    }

    pub fn title(mut self, title: Title) -> Axis {
        self.title = Some(title);
        self
    }

    pub fn type_(mut self, t: AxisType) -> Axis {
        self.r#type = Some(t);
        self
    }

    pub fn auto_range(mut self, auto_range: bool) -> Axis {
        self.auto_range = Some(auto_range);
        self
    }

    pub fn range_mode(mut self, range_mode: RangeMode) -> Axis {
        self.range_mode = Some(range_mode);
        self
    }

    pub fn range<C: NumOrString>(mut self, range: Vec<C>) -> Axis {
        let wrapped = to_num_or_string_wrapper(range);
        self.range = Some(wrapped);
        self
    }

    pub fn fixed_range(mut self, fixed_range: bool) -> Axis {
        self.fixed_range = Some(fixed_range);
        self
    }

    pub fn constrain(mut self, constrain: AxisConstrain) -> Axis {
        self.constrain = Some(constrain);
        self
    }

    pub fn constrain_toward(mut self, constrain_toward: ConstrainDirection) -> Axis {
        self.constrain_toward = Some(constrain_toward);
        self
    }

    pub fn tick_mode(mut self, tick_mode: TickMode) -> Axis {
        self.tick_mode = Some(tick_mode);
        self
    }

    pub fn n_ticks(mut self, n_ticks: usize) -> Axis {
        self.n_ticks = Some(n_ticks);
        self
    }

    pub fn tick0(mut self, tick0: f64) -> Axis {
        self.tick0 = Some(tick0);
        self
    }

    pub fn dtick(mut self, dtick: f64) -> Axis {
        self.dtick = Some(dtick);
        self
    }

    pub fn tick_values(mut self, tick_values: Vec<f64>) -> Axis {
        self.tick_values = Some(tick_values);
        self
    }

    pub fn tick_text(mut self, tick_text: Vec<String>) -> Axis {
        self.tick_text = Some(tick_text);
        self
    }

    pub fn ticks(mut self, ticks: TicksDirection) -> Axis {
        self.ticks = Some(ticks);
        self
    }

    pub fn ticks_on(mut self, ticks_on: TicksPosition) -> Axis {
        self.ticks_on = Some(ticks_on);
        self
    }

    pub fn mirror(mut self, mirror: bool) -> Axis {
        self.mirror = Some(mirror);
        self
    }

    pub fn tick_length(mut self, tick_length: usize) -> Axis {
        self.tick_length = Some(tick_length);
        self
    }

    pub fn tick_width(mut self, tick_width: usize) -> Axis {
        self.tick_width = Some(tick_width);
        self
    }

    pub fn tick_color<C: Color>(mut self, tick_color: C) -> Axis {
        self.tick_color = Some(tick_color.to_color());
        self
    }

    pub fn show_tick_labels(mut self, show_tick_labels: bool) -> Axis {
        self.show_tick_labels = Some(show_tick_labels);
        self
    }

    pub fn auto_margin(mut self, auto_margin: bool) -> Axis {
        self.auto_margin = Some(auto_margin);
        self
    }

    pub fn show_spikes(mut self, show_spikes: bool) -> Axis {
        self.show_spikes = Some(show_spikes);
        self
    }

    pub fn spike_color<C: Color>(mut self, spike_color: C) -> Axis {
        self.spike_color = Some(spike_color.to_color());
        self
    }

    pub fn spike_thickness(mut self, spike_thickness: usize) -> Axis {
        self.spike_thickness = Some(spike_thickness);
        self
    }

    pub fn spike_dash(mut self, spike_dash: DashType) -> Axis {
        self.spike_dash = Some(spike_dash);
        self
    }

    pub fn spike_mode(mut self, spike_mode: &str) -> Axis {
        self.spike_mode = Some(spike_mode.to_owned());
        self
    }

    pub fn spike_snap(mut self, spike_snap: &str) -> Axis {
        self.spike_snap = Some(spike_snap.to_owned());
        self
    }

    pub fn tick_font(mut self, tick_font: Font) -> Axis {
        self.tick_font = Some(tick_font);
        self
    }

    pub fn tick_angle(mut self, tick_angle: f64) -> Axis {
        self.tick_angle = Some(tick_angle);
        self
    }

    pub fn tick_prefix(mut self, tick_prefix: &str) -> Axis {
        self.tick_prefix = Some(tick_prefix.to_owned());
        self
    }

    pub fn show_tick_prefix(mut self, show_tick_prefix: ArrayShow) -> Axis {
        self.show_tick_prefix = Some(show_tick_prefix);
        self
    }

    pub fn tick_suffix(mut self, tick_suffix: &str) -> Axis {
        self.tick_suffix = Some(tick_suffix.to_owned());
        self
    }

    pub fn show_tick_suffix(mut self, show_tick_suffix: ArrayShow) -> Axis {
        self.show_tick_suffix = Some(show_tick_suffix);
        self
    }

    pub fn show_exponent(mut self, show_exponent: ArrayShow) -> Axis {
        self.show_exponent = Some(show_exponent);
        self
    }

    pub fn exponent_format(mut self, exponent_format: &str) -> Axis {
        self.exponent_format = Some(exponent_format.to_owned());
        self
    }

    pub fn separate_thousands(mut self, separate_thousands: bool) -> Axis {
        self.separate_thousands = Some(separate_thousands);
        self
    }

    pub fn tick_format(mut self, tick_format: &str) -> Axis {
        self.tick_format = Some(tick_format.to_owned());
        self
    }

    pub fn tick_format_stops(mut self, tick_format_stops: Vec<TickFormatStop>) -> Axis {
        self.tick_format_stops = Some(tick_format_stops);
        self
    }

    pub fn hover_format(mut self, hover_format: &str) -> Axis {
        self.hover_format = Some(hover_format.to_owned());
        self
    }

    pub fn show_line(mut self, show_line: bool) -> Axis {
        self.show_line = Some(show_line);
        self
    }

    pub fn line_color<C: Color>(mut self, line_color: C) -> Axis {
        self.line_color = Some(line_color.to_color());
        self
    }

    pub fn line_width(mut self, line_width: usize) -> Axis {
        self.line_width = Some(line_width);
        self
    }

    pub fn show_grid(mut self, show_grid: bool) -> Axis {
        self.show_grid = Some(show_grid);
        self
    }

    pub fn grid_color<C: Color>(mut self, grid_color: C) -> Axis {
        self.grid_color = Some(grid_color.to_color());
        self
    }

    pub fn grid_width(mut self, grid_width: usize) -> Axis {
        self.grid_width = Some(grid_width);
        self
    }

    pub fn zero_line(mut self, zero_line: bool) -> Axis {
        self.zero_line = Some(zero_line);
        self
    }

    pub fn zero_line_color<C: Color>(mut self, zero_line_color: C) -> Axis {
        self.zero_line_color = Some(zero_line_color.to_color());
        self
    }

    pub fn zero_line_width(mut self, zero_line_width: usize) -> Axis {
        self.zero_line_width = Some(zero_line_width);
        self
    }

    pub fn show_dividers(mut self, show_dividers: bool) -> Axis {
        self.show_dividers = Some(show_dividers);
        self
    }

    pub fn divider_color<C: Color>(mut self, divider_color: C) -> Axis {
        self.divider_color = Some(divider_color.to_color());
        self
    }

    pub fn divider_width(mut self, divider_width: usize) -> Axis {
        self.divider_width = Some(divider_width);
        self
    }

    pub fn anchor(mut self, anchor: &str) -> Axis {
        self.anchor = Some(anchor.to_owned());
        self
    }

    pub fn side(mut self, side: Side) -> Axis {
        self.side = Some(side);
        self
    }

    pub fn overlaying(mut self, overlaying: &str) -> Axis {
        self.overlaying = Some(overlaying.to_owned());
        self
    }

    pub fn domain(mut self, domain: &[f64]) -> Axis {
        self.domain = Some(domain.to_vec());
        self
    }

    pub fn position(mut self, position: f64) -> Axis {
        self.position = Some(position);
        self
    }

    pub fn range_slider(mut self, slider: RangeSlider) -> Axis {
        self.range_slider = Some(slider);
        self
    }

    pub fn range_selector(mut self, range_selector: RangeSelector) -> Axis {
        self.range_selector = Some(range_selector);
        self
    }

    pub fn calendar(mut self, calendar: Calendar) -> Axis {
        self.calendar = Some(calendar);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum RowOrder {
    #[serde(rename = "top to bottom")]
    TopToBottom,
    #[serde(rename = "bottom to top")]
    BottomToTop,
}

#[derive(Serialize, Debug)]
pub enum GridPattern {
    #[serde(rename = "independent")]
    Independent,
    #[serde(rename = "coupled")]
    Coupled,
}

#[derive(Serialize, Debug)]
pub enum GridXSide {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom plot")]
    BottomPlot,
    #[serde(rename = "top plot")]
    TopPlot,
    #[serde(rename = "top")]
    Top,
}

#[derive(Serialize, Debug)]
pub enum GridYSide {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "left plot")]
    LeftPlot,
    #[serde(rename = "right plot")]
    RightPlot,
    #[serde(rename = "right")]
    Right,
}

#[derive(Serialize, Debug, Default)]
pub struct GridDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<f64>>,
}

impl GridDomain {
    pub fn new() -> GridDomain {
        Default::default()
    }

    pub fn x(mut self, x: Vec<f64>) -> GridDomain {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: Vec<f64>) -> GridDomain {
        self.y = Some(y);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct LayoutGrid {
    #[serde(skip_serializing_if = "Option::is_none")]
    rows: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "roworder")]
    row_order: Option<RowOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "subplots")]
    sub_plots: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxes")]
    x_axes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxes")]
    y_axes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<GridPattern>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xgap")]
    x_gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ygap")]
    y_gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<GridDomain>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xside")]
    x_side: Option<GridXSide>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yside")]
    y_side: Option<GridYSide>,
}

impl LayoutGrid {
    pub fn new() -> LayoutGrid {
        Default::default()
    }

    pub fn rows(mut self, rows: usize) -> LayoutGrid {
        self.rows = Some(rows);
        self
    }

    pub fn row_order(mut self, row_order: RowOrder) -> LayoutGrid {
        self.row_order = Some(row_order);
        self
    }

    pub fn columns(mut self, columns: usize) -> LayoutGrid {
        self.columns = Some(columns);
        self
    }

    pub fn sub_plots(mut self, sub_plots: Vec<String>) -> LayoutGrid {
        self.sub_plots = Some(sub_plots);
        self
    }

    pub fn x_axes(mut self, x_axes: Vec<String>) -> LayoutGrid {
        self.x_axes = Some(x_axes);
        self
    }

    pub fn y_axes(mut self, y_axes: Vec<String>) -> LayoutGrid {
        self.y_axes = Some(y_axes);
        self
    }

    pub fn pattern(mut self, pattern: GridPattern) -> LayoutGrid {
        self.pattern = Some(pattern);
        self
    }

    pub fn x_gap(mut self, x_gap: f64) -> LayoutGrid {
        self.x_gap = Some(x_gap);
        self
    }

    pub fn y_gap(mut self, y_gap: f64) -> LayoutGrid {
        self.y_gap = Some(y_gap);
        self
    }

    pub fn domain(mut self, domain: GridDomain) -> LayoutGrid {
        self.domain = Some(domain);
        self
    }

    pub fn x_side(mut self, x_side: GridXSide) -> LayoutGrid {
        self.x_side = Some(x_side);
        self
    }
    pub fn y_side(mut self, y_side: GridYSide) -> LayoutGrid {
        self.y_side = Some(y_side);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum UniformTextMode {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "hide")]
    Hide,
    #[serde(rename = "show")]
    Show,
}

#[derive(Serialize, Debug, Default)]
pub struct UniformText {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<TruthyEnum<UniformTextMode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<usize>,
}

impl UniformText {
    pub fn new() -> UniformText {
        Default::default()
    }

    pub fn mode(mut self, mode: UniformTextMode) -> UniformText {
        self.mode = Some(TruthyEnum { e: mode });
        self
    }

    pub fn min_size(mut self, min_size: usize) -> UniformText {
        self.min_size = Some(min_size);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum HoverMode {
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
    #[serde(rename = "closest")]
    Closest,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "x unified")]
    XUnified,
    #[serde(rename = "y unified")]
    YUnified,
}

#[derive(Serialize, Debug, Default)]
pub struct ModeBar {
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activecolor")]
    active_color: Option<ColorWrapper>,
}

impl ModeBar {
    pub fn new() -> ModeBar {
        Default::default()
    }

    pub fn orientation<C: Color>(mut self, orientation: Orientation) -> ModeBar {
        self.orientation = Some(orientation);
        self
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> ModeBar {
        self.background_color = Some(background_color.to_color());
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> ModeBar {
        self.color = Some(color.to_color());
        self
    }

    pub fn active_color<C: Color>(mut self, active_color: C) -> ModeBar {
        self.active_color = Some(active_color.to_color());
        self
    }
}

#[derive(Serialize, Debug)]
pub enum ShapeType {
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "rect")]
    Rect,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "line")]
    Line,
}

#[derive(Serialize, Debug)]
pub enum ShapeLayer {
    #[serde(rename = "below")]
    Below,
    #[serde(rename = "above")]
    Above,
}

#[derive(Serialize, Debug)]
pub enum ShapeSizeMode {
    #[serde(rename = "scaled")]
    Scaled,
    #[serde(rename = "pixel")]
    Pixel,
}

#[derive(Serialize, Debug)]
pub enum FillRule {
    #[serde(rename = "evenodd")]
    EvenOdd,
    #[serde(rename = "nonzero")]
    NonZero,
}

#[derive(Serialize, Debug, Default)]
pub struct ShapeLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dash: Option<String>,
}

impl ShapeLine {
    pub fn new() -> ShapeLine {
        Default::default()
    }

    /// Sets the line color.
    pub fn color<C: Color>(mut self, color: C) -> ShapeLine {
        self.color = Some(color.to_color());
        self
    }

    /// Sets the line width (in px).
    pub fn width(mut self, width: f64) -> ShapeLine {
        self.width = Some(width);
        self
    }

    /// Sets the dash style of lines. Set to a dash type string ("solid", "dot", "dash", "longdash",
    /// "dashdot", or "longdashdot") or a dash length list in px (eg "5px,10px,2px,2px").
    pub fn dash(mut self, dash: &str) -> ShapeLine {
        self.dash = Some(dash.to_owned());
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct Shape {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<ShapeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layer: Option<ShapeLayer>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xref")]
    x_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xsizemode")]
    x_size_mode: Option<ShapeSizeMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xanchor")]
    x_anchor: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x0: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x1: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yref")]
    y_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ysizemode")]
    y_size_mode: Option<ShapeSizeMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yanchor")]
    y_anchor: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y0: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y1: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<ShapeLine>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillcolor")]
    fill_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillrule")]
    fill_rule: Option<FillRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Shape {
    pub fn new() -> Shape {
        Default::default()
    }

    /// Determines whether or not this shape is visible.
    pub fn visible(mut self, visible: bool) -> Shape {
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
    pub fn shape_type(mut self, shape_type: ShapeType) -> Shape {
        self.r#type = Some(shape_type);
        self
    }

    /// Specifies whether shapes are drawn below or above traces.
    pub fn layer(mut self, layer: ShapeLayer) -> Shape {
        self.layer = Some(layer);
        self
    }

    /// Sets the shape's x coordinate axis. If set to an x axis id (e.g. "x" or "x2"), the `x`
    /// position refers to an x coordinate. If set to "paper", the `x` position refers to the
    /// distance from the left side of the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the left (right) side. If the axis `type` is "log", then you must take the
    /// log of your desired range. If the axis `type` is "date", then you must convert the date to
    /// unix time in milliseconds.
    pub fn x_ref(mut self, x_ref: &str) -> Shape {
        self.x_ref = Some(x_ref.to_owned());
        self
    }

    /// Sets the shapes's sizing mode along the x axis. If set to "scaled", `x0`, `x1` and x
    /// coordinates within `path` refer to data values on the x axis or a fraction of the plot
    /// area's width (`xref` set to "paper"). If set to "pixel", `xanchor` specifies the x position
    /// in terms of data or plot fraction but `x0`, `x1` and x coordinates within `path` are pixels
    /// relative to `xanchor`. This way, the shape can have a fixed width while maintaining a
    /// position relative to data or plot fraction.
    pub fn x_size_mode(mut self, x_size_mode: ShapeSizeMode) -> Shape {
        self.x_size_mode = Some(x_size_mode);
        self
    }

    /// Only relevant in conjunction with `xsizemode` set to "pixel". Specifies the anchor point on
    /// the x axis to which `x0`, `x1` and x coordinates within `path` are relative to. E.g. useful
    /// to attach a pixel sized shape to a certain data value. No effect when `xsizemode` not set
    /// to "pixel".
    pub fn x_anchor<C: NumOrString>(mut self, x_anchor: C) -> Shape {
        self.x_anchor = Some(x_anchor.to_num_or_string());
        self
    }

    /// Sets the shape's starting x position. See `type` and `xsizemode` for more info.
    pub fn x0<C: NumOrString>(mut self, x0: C) -> Shape {
        self.x0 = Some(x0.to_num_or_string());
        self
    }

    /// Sets the shape's end x position. See `type` and `xsizemode` for more info.
    pub fn x1<C: NumOrString>(mut self, x1: C) -> Shape {
        self.x1 = Some(x1.to_num_or_string());
        self
    }

    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g. "y" or "y2"),
    /// the `y` position refers to an y coordinate If set to "paper", the `y` position refers to
    /// the distance from the bottom of the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the bottom (top).
    pub fn y_ref(mut self, y_ref: &str) -> Shape {
        self.y_ref = Some(y_ref.to_owned());
        self
    }

    /// Sets the shapes's sizing mode along the y axis. If set to "scaled", `y0`, `y1` and y
    /// coordinates within `path` refer to data values on the y axis or a fraction of the plot
    /// area's height (`yref` set to "paper"). If set to "pixel", `yanchor` specifies the y position
    /// in terms of data or plot fraction but `y0`, `y1` and y coordinates within `path` are pixels
    /// relative to `yanchor`. This way, the shape can have a fixed height while maintaining a
    /// position relative to data or plot fraction.
    pub fn y_size_mode(mut self, y_size_mode: ShapeSizeMode) -> Shape {
        self.y_size_mode = Some(y_size_mode);
        self
    }

    /// Only relevant in conjunction with `ysizemode` set to "pixel". Specifies the anchor point on
    /// the y axis to which `y0`, `y1` and y coordinates within `path` are relative to. E.g. useful
    /// to attach a pixel sized shape to a certain data value. No effect when `ysizemode` not set
    /// to "pixel".
    pub fn y_anchor<C: NumOrString>(mut self, y_anchor: C) -> Shape {
        self.y_anchor = Some(y_anchor.to_num_or_string());
        self
    }

    /// Sets the shape's starting y position. See `type` and `ysizemode` for more info.
    pub fn y0<C: NumOrString>(mut self, y0: C) -> Shape {
        self.y0 = Some(y0.to_num_or_string());
        self
    }

    /// Sets the shape's end y position. See `type` and `ysizemode` for more info.
    pub fn y1<C: NumOrString>(mut self, y1: C) -> Shape {
        self.y1 = Some(y1.to_num_or_string());
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
    pub fn path(mut self, path: &str) -> Shape {
        self.path = Some(path.to_owned());
        self
    }

    /// Sets the opacity of the shape. Number between or equal to 0 and 1.
    pub fn opacity(mut self, opacity: f64) -> Shape {
        self.opacity = Some(opacity);
        self
    }

    /// Sets the shape line properties (`color`, `width`, `dash`).
    pub fn line(mut self, line: ShapeLine) -> Shape {
        self.line = Some(line);
        self
    }

    /// Sets the color filling the shape's interior. Only applies to closed shapes.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Shape {
        self.fill_color = Some(fill_color.to_color());
        self
    }

    /// Determines which regions of complex paths constitute the interior. For more info please
    /// visit https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule
    pub fn fill_rule(mut self, fill_rule: FillRule) -> Shape {
        self.fill_rule = Some(fill_rule);
        self
    }

    /// Determines whether the shape could be activated for edit or not. Has no effect when the
    /// older editable shapes mode is enabled via `config.editable` or `config.edits.shapePosition`.
    pub fn editable(mut self, editable: bool) -> Shape {
        self.editable = Some(editable);
        self
    }

    /// When used in a template, named items are created in the output figure in addition to any
    /// items the figure already has in this array. You can modify these items in the output figure
    /// by making your own item with `templateitemname` matching this `name` alongside your
    /// modifications (including `visible: false` or `enabled: false` to hide it). Has no effect
    /// outside of a template.
    pub fn name(mut self, name: &str) -> Shape {
        self.name = Some(name.to_owned());
        self
    }

    /// Used to refer to a named item in this array in the template. Named items from the template
    /// will be created even without a matching item in the input figure, but you can modify one
    /// by making an item with `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is no template or no
    /// matching item, this item will be hidden unless you explicitly show it with `visible: true`.
    pub fn template_item_name(mut self, template_item_name: &str) -> Shape {
        self.template_item_name = Some(template_item_name.to_owned());
        self
    }
}

#[derive(Serialize, Debug)]
pub enum DrawDirection {
    #[serde(rename = "ortho")]
    Ortho,
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "diagonal")]
    Diagonal,
}

#[derive(Serialize, Debug, Default)]
pub struct NewShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<ShapeLine>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillcolor")]
    fill_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillrule")]
    fill_rule: Option<FillRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layer: Option<ShapeLayer>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "drawdirection")]
    draw_direction: Option<DrawDirection>,
}

impl NewShape {
    pub fn new() -> NewShape {
        Default::default()
    }

    /// Sets the shape line properties (`color`, `width`, `dash`).
    pub fn line(mut self, line: ShapeLine) -> NewShape {
        self.line = Some(line);
        self
    }

    /// Sets the color filling new shapes' interior. Please note that if using a fillcolor with
    /// alpha greater than half, drag inside the active shape starts moving the shape underneath,
    /// otherwise a new shape could be started over.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> NewShape {
        self.fill_color = Some(fill_color.to_color());
        self
    }

    /// Determines the path's interior. For more info please
    /// visit https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule
    pub fn fill_rule(mut self, fill_rule: FillRule) -> NewShape {
        self.fill_rule = Some(fill_rule);
        self
    }

    /// Sets the opacity of new shapes. Number between or equal to 0 and 1.
    pub fn opacity(mut self, opacity: f64) -> NewShape {
        self.opacity = Some(opacity);
        self
    }

    /// Specifies whether new shapes are drawn below or above traces.
    pub fn layer(mut self, layer: ShapeLayer) -> NewShape {
        self.layer = Some(layer);
        self
    }

    /// When `dragmode` is set to "drawrect", "drawline" or "drawcircle" this limits the drag to be
    /// horizontal, vertical or diagonal. Using "diagonal" there is no limit e.g. in drawing lines
    /// in any direction. "ortho" limits the draw to be either horizontal or vertical. "horizontal"
    /// allows horizontal extend. "vertical" allows vertical extend.
    pub fn draw_direction(mut self, draw_direction: DrawDirection) -> NewShape {
        self.draw_direction = Some(draw_direction);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct ActiveShape {
    #[serde(skip_serializing_if = "Option::is_none", rename = "fillcolor")]
    fill_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
}

impl ActiveShape {
    pub fn new() -> ActiveShape {
        Default::default()
    }

    /// Sets the color filling the active shape' interior.
    pub fn fill_color<C: Color>(mut self, fill_color: C) -> ActiveShape {
        self.fill_color = Some(fill_color.to_color());
        self
    }

    /// Sets the opacity of the active shape. Number between or equal to 0 and 1.
    pub fn opacity(mut self, opacity: f64) -> ActiveShape {
        self.opacity = Some(opacity);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum ArrowSide {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end+start")]
    StartEnd,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Debug)]
pub enum ClickToShow {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "onoff")]
    OnOff,
    #[serde(rename = "onout")]
    OnOut,
}

#[derive(Serialize, Debug, Default)]
pub struct Annotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textangle")]
    text_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<HAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valign: Option<VAlign>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderpad")]
    border_pad: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderwidth")]
    border_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showarrow")]
    show_arrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "arrowcolor")]
    arrow_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "arrowhead")]
    arrow_head: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startarrowhead")]
    start_arrow_head: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "arrowside")]
    arrow_side: Option<ArrowSide>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "arrowsize")]
    arrow_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startarrowsize")]
    start_arrow_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "arrowwidth")]
    arrow_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "standoff")]
    stand_off: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startstandoff")]
    start_stand_off: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ax: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ay: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "axref")]
    ax_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ayref")]
    ay_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xref")]
    x_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xshift")]
    x_shift: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yref")]
    y_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yanchor")]
    y_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yshift")]
    y_shift: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "clicktoshow")]
    click_to_show: Option<TruthyEnum<ClickToShow>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xclick")]
    x_click: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yclick")]
    y_click: Option<NumOrStringWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureevents")]
    capture_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Annotation {
    pub fn new() -> Annotation {
        Default::default()
    }

    /// Determines whether or not this annotation is visible.
    pub fn visible(mut self, visible: bool) -> Annotation {
        self.visible = Some(visible);
        self
    }

    /// Sets the text associated with this annotation. Plotly uses a subset of HTML tags to do
    /// things like newline (<br>), bold (<b></b>), italics (<i></i>), hyperlinks
    /// (<a href='...'></a>). Tags <em>, <sup>, <sub> <span> are also supported.
    pub fn text(mut self, text: &str) -> Annotation {
        self.text = Some(text.to_owned());
        self
    }

    /// Sets the angle at which the `text` is drawn with respect to the horizontal.
    pub fn text_angle(mut self, text_angle: f64) -> Annotation {
        self.text_angle = Some(text_angle);
        self
    }

    /// Sets the annotation text font.
    pub fn font(mut self, font: Font) -> Annotation {
        self.font = Some(font);
        self
    }

    /// Sets an explicit width for the text box. null (default) lets the text set the box width.
    /// Wider text will be clipped. There is no automatic wrapping; use <br> to start a new line.
    pub fn width(mut self, width: f64) -> Annotation {
        self.width = Some(width);
        self
    }

    /// Sets an explicit height for the text box. null (default) lets the text set the box height.
    /// Taller text will be clipped.
    pub fn height(mut self, height: f64) -> Annotation {
        self.height = Some(height);
        self
    }

    /// Sets the opacity of the annotation (text + arrow).
    pub fn opacity(mut self, opacity: f64) -> Annotation {
        self.opacity = Some(opacity);
        self
    }

    /// Sets the horizontal alignment of the `text` within the box. Has an effect only if `text`
    /// spans two or more lines (i.e. `text` contains one or more <br> HTML tags) or if an explicit
    /// width is set to override the text width.
    pub fn align(mut self, align: HAlign) -> Annotation {
        self.align = Some(align);
        self
    }

    /// Sets the vertical alignment of the `text` within the box. Has an effect only if an explicit
    /// height is set to override the text height.
    pub fn valign(mut self, valign: VAlign) -> Annotation {
        self.valign = Some(valign);
        self
    }

    /// Sets the background color of the annotation.
    pub fn background_color<C: Color>(mut self, background_color: C) -> Annotation {
        self.background_color = Some(background_color.to_color());
        self
    }

    /// Sets the color of the border enclosing the annotation `text`.
    pub fn border_color<C: Color>(mut self, border_color: C) -> Annotation {
        self.border_color = Some(border_color.to_color());
        self
    }

    /// Sets the padding (in px) between the `text` and the enclosing border.
    pub fn border_pad(mut self, border_pad: f64) -> Annotation {
        self.border_pad = Some(border_pad);
        self
    }

    /// Sets the width (in px) of the border enclosing the annotation `text`.
    pub fn border_width(mut self, border_width: f64) -> Annotation {
        self.border_width = Some(border_width);
        self
    }

    /// Determines whether or not the annotation is drawn with an arrow. If "True", `text` is
    /// placed near the arrow's tail. If "False", `text` lines up with the `x` and `y` provided.
    pub fn show_arrow(mut self, show_arrow: bool) -> Annotation {
        self.show_arrow = Some(show_arrow);
        self
    }

    /// Sets the color of the annotation arrow.
    pub fn arrow_color<C: Color>(mut self, arrow_color: C) -> Annotation {
        self.arrow_color = Some(arrow_color.to_color());
        self
    }

    /// Sets the end annotation arrow head style. Integer between or equal to 0 and 8.
    pub fn arrow_head(mut self, arrow_head: u8) -> Annotation {
        self.arrow_head = Some(arrow_head);
        self
    }

    /// Sets the start annotation arrow head style. Integer between or equal to 0 and 8.
    pub fn start_arrow_head(mut self, start_arrow_head: u8) -> Annotation {
        self.start_arrow_head = Some(start_arrow_head);
        self
    }

    /// Sets the annotation arrow head position.
    pub fn arrow_side(mut self, arrow_side: ArrowSide) -> Annotation {
        self.arrow_side = Some(arrow_side);
        self
    }

    /// Sets the size of the end annotation arrow head, relative to `arrowwidth`. A value of 1
    /// (default) gives a head about 3x as wide as the line.
    pub fn arrow_size(mut self, arrow_size: f64) -> Annotation {
        self.arrow_size = Some(arrow_size);
        self
    }

    /// Sets the size of the start annotation arrow head, relative to `arrowwidth`. A value of 1
    /// (default) gives a head about 3x as wide as the line.
    pub fn start_arrow_size(mut self, start_arrow_size: f64) -> Annotation {
        self.start_arrow_size = Some(start_arrow_size);
        self
    }

    /// Sets the width (in px) of annotation arrow line.
    pub fn arrow_width(mut self, arrow_width: f64) -> Annotation {
        self.arrow_width = Some(arrow_width);
        self
    }

    /// Sets a distance, in pixels, to move the end arrowhead away from the position it is pointing
    /// at, for example to point at the edge of a marker independent of zoom. Note that this
    /// shortens the arrow from the `ax` / `ay` vector, in contrast to `xshift` / `yshift` which
    /// moves everything by this amount.
    pub fn stand_off(mut self, stand_off: f64) -> Annotation {
        self.stand_off = Some(stand_off);
        self
    }

    /// Sets a distance, in pixels, to move the start arrowhead away from the position it is
    /// pointing at, for example to point at the edge of a marker independent of zoom. Note that
    /// this shortens the arrow from the `ax` / `ay` vector, in contrast to `xshift` / `yshift`
    /// which moves everything by this amount.
    pub fn start_stand_off(mut self, start_stand_off: f64) -> Annotation {
        self.start_stand_off = Some(start_stand_off);
        self
    }

    /// Sets the x component of the arrow tail about the arrow head. If `axref` is `pixel`, a
    /// positive (negative) component corresponds to an arrow pointing from right to left (left
    /// to right). If `axref` is an axis, this is an absolute value on that axis, like `x`, NOT a
    /// relative value.
    pub fn ax<C: NumOrString>(mut self, ax: C) -> Annotation {
        self.ax = Some(ax.to_num_or_string());
        self
    }

    /// Sets the y component of the arrow tail about the arrow head. If `ayref` is `pixel`, a
    /// positive (negative) component corresponds to an arrow pointing from bottom to top (top to
    /// bottom). If `ayref` is an axis, this is an absolute value on that axis, like `y`, NOT a
    /// relative value.
    pub fn ay<C: NumOrString>(mut self, ay: C) -> Annotation {
        self.ay = Some(ay.to_num_or_string());
        self
    }

    /// Indicates in what terms the tail of the annotation (ax,ay) is specified. If `pixel`, `ax`
    /// is a relative offset in pixels from `x`. If set to an x axis id (e.g. "x" or "x2"), `ax` is
    /// specified in the same terms as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    pub fn ax_ref(mut self, ax_ref: &str) -> Annotation {
        self.ax_ref = Some(ax_ref.to_owned());
        self
    }

    /// Indicates in what terms the tail of the annotation (ax,ay) is specified. If `pixel`, `ay`
    /// is a relative offset in pixels from `y`. If set to a y axis id (e.g. "y" or "y2"), `ay` is
    /// specified in the same terms as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    pub fn ay_ref(mut self, ay_ref: &str) -> Annotation {
        self.ay_ref = Some(ay_ref.to_owned());
        self
    }

    /// Sets the annotation's x coordinate axis. If set to an x axis id (e.g. "x" or "x2"), the `x`
    /// position refers to an x coordinate If set to "paper", the `x` position refers to the
    /// distance from the left side of the plotting area in normalized coordinates where 0 (1)
    /// corresponds to the left (right) side.
    pub fn x_ref(mut self, x_ref: &str) -> Annotation {
        self.x_ref = Some(x_ref.to_owned());
        self
    }

    /// Sets the annotation's x position. If the axis `type` is "log", then you must take the log
    /// of your desired range. If the axis `type` is "date", it should be date strings, like date
    /// data, though Date objects and unix milliseconds will be accepted and converted to strings.
    /// If the axis `type` is "category", it should be numbers, using the scale where each category
    /// is assigned a serial number from zero in the order it appears.
    pub fn x<C: NumOrString>(mut self, x: C) -> Annotation {
        self.x = Some(x.to_num_or_string());
        self
    }

    /// Sets the text box's horizontal position anchor This anchor binds the `x` position to the
    /// "left", "center" or "right" of the annotation. For example, if `x` is set to 1, `xref` to
    /// "paper" and `xanchor` to "right" then the right-most portion of the annotation lines up with
    /// the right-most edge of the plotting area. If "auto", the anchor is equivalent to "center"
    /// for data-referenced annotations or if there is an arrow, whereas for paper-referenced with
    /// no arrow, the anchor picked corresponds to the closest side.
    pub fn x_anchor(mut self, x_anchor: Anchor) -> Annotation {
        self.x_anchor = Some(x_anchor);
        self
    }

    /// Shifts the position of the whole annotation and arrow to the right (positive) or left
    /// (negative) by this many pixels.
    pub fn x_shift(mut self, x_shift: f64) -> Annotation {
        self.x_shift = Some(x_shift);
        self
    }

    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g. "y" or "y2"), the `y`
    /// position refers to an y coordinate If set to "paper", the `y` position refers to the
    /// distance from the bottom of the plotting area in normalized coordinates where 0 (1)
    /// corresponds to the bottom (top).
    pub fn y_ref(mut self, y_ref: &str) -> Annotation {
        self.y_ref = Some(y_ref.to_owned());
        self
    }

    /// Sets the annotation's y position. If the axis `type` is "log", then you must take the log of
    /// your desired range. If the axis `type` is "date", it should be date strings, like date data,
    /// though Date objects and unix milliseconds will be accepted and converted to strings. If the
    /// axis `type` is "category", it should be numbers, using the scale where each category is
    /// assigned a serial number from zero in the order it appears.
    pub fn y<C: NumOrString>(mut self, y: C) -> Annotation {
        self.y = Some(y.to_num_or_string());
        self
    }

    /// Sets the text box's vertical position anchor This anchor binds the `y` position to the
    /// "top", "middle" or "bottom" of the annotation. For example, if `y` is set to 1, `yref` to
    /// "paper" and `yanchor` to "top" then the top-most portion of the annotation lines up with the
    /// top-most edge of the plotting area. If "auto", the anchor is equivalent to "middle" for
    /// data-referenced annotations or if there is an arrow, whereas for paper-referenced with no
    /// arrow, the anchor picked corresponds to the closest side.
    pub fn y_anchor(mut self, y_anchor: Anchor) -> Annotation {
        self.y_anchor = Some(y_anchor);
        self
    }

    /// Shifts the position of the whole annotation and arrow up (positive) or down (negative) by
    /// this many pixels.
    pub fn y_shift(mut self, y_shift: f64) -> Annotation {
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
    pub fn click_to_show(mut self, click_to_show: TruthyEnum<ClickToShow>) -> Annotation {
        self.click_to_show = Some(click_to_show);
        self
    }

    /// Toggle this annotation when clicking a data point whose `x` value is `xclick` rather than
    /// the annotation's `x` value.
    pub fn x_click<C: NumOrString>(mut self, x_click: C) -> Annotation {
        self.x_click = Some(x_click.to_num_or_string());
        self
    }

    /// Toggle this annotation when clicking a data point whose `y` value is `yclick` rather than
    /// the annotation's `y` value.
    pub fn y_click<C: NumOrString>(mut self, y_click: C) -> Annotation {
        self.y_click = Some(y_click.to_num_or_string());
        self
    }

    /// Sets text to appear when hovering over this annotation. If omitted or blank, no hover label
    /// will appear.
    pub fn hover_text(mut self, hover_text: &str) -> Annotation {
        self.hover_text = Some(hover_text.to_owned());
        self
    }

    /// Label displayed on mouse hover.
    pub fn hover_label(mut self, hover_label: Label) -> Annotation {
        self.hover_label = Some(hover_label);
        self
    }

    /// Determines whether the annotation text box captures mouse move and click events, or allows
    /// those events to pass through to data points in the plot that may be behind the annotation.
    /// By default `captureevents` is "false" unless `hovertext` is provided. If you use the event
    /// `plotly_clickannotation` without `hovertext` you must explicitly enable `captureevents`.
    pub fn capture_events(mut self, capture_events: bool) -> Annotation {
        self.capture_events = Some(capture_events);
        self
    }

    /// When used in a template, named items are created in the output figure in addition to any
    /// items the figure already has in this array. You can modify these items in the output figure
    /// by making your own item with `templateitemname` matching this `name` alongside your
    /// modifications (including `visible: false` or `enabled: false` to hide it). Has no effect
    /// outside of a template.
    pub fn name(mut self, name: &str) -> Annotation {
        self.name = Some(name.to_owned());
        self
    }

    /// Used to refer to a named item in this array in the template. Named items from the template
    /// will be created even without a matching item in the input figure, but you can modify one by
    /// making an item with `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is no template or no
    /// matching item, this item will be hidden unless you explicitly show it with `visible: true`.
    pub fn template_item_name(mut self, template_item_name: &str) -> Annotation {
        self.template_item_name = Some(template_item_name.to_owned());
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct Layout {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend: Option<Legend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    margin: Option<Margin>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autosize")]
    auto_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uniform_text: Option<UniformText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separators: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "paper_bgcolor")]
    paper_background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "plot_bgcolor")]
    plot_background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<LayoutColorScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colorway: Option<Vec<ColorWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "coloraxis")]
    color_axis: Option<ColorAxis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "modebar")]
    mode_bar: Option<ModeBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovermode")]
    hover_mode: Option<TruthyEnum<HoverMode>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "clickmode")]
    click_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dragmode")]
    drag_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "selectdirection")]
    select_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverdistance")]
    hover_distance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikedistance")]
    spike_distance: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    grid: Option<LayoutGrid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<Axis>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis2")]
    x_axis2: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis2")]
    y_axis2: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis3")]
    x_axis3: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis3")]
    y_axis3: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis4")]
    x_axis4: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis4")]
    y_axis4: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis5")]
    x_axis5: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis5")]
    y_axis5: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis6")]
    x_axis6: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis6")]
    y_axis6: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis7")]
    x_axis7: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis7")]
    y_axis7: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis8")]
    x_axis8: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis8")]
    y_axis8: Option<Axis>,

    // ternary: Option<LayoutTernary>,
    // scene: Option<LayoutScene>,
    // polar: Option<LayoutPolar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<Vec<Annotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shapes: Option<Vec<Shape>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "newshape")]
    new_shape: Option<NewShape>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeshape")]
    active_shape: Option<ActiveShape>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "boxmode")]
    box_mode: Option<BoxMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "boxgap")]
    box_gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "boxgroupgap")]
    box_group_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "barmode")]
    bar_mode: Option<BarMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "barnorm")]
    bar_norm: Option<BarNorm>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bargap")]
    bar_gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bargroupgap")]
    bar_group_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "violinmode")]
    violin_mode: Option<ViolinMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violingap")]
    violin_gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "violingroupgap")]
    violin_group_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "waterfallmode")]
    waterfall_mode: Option<WaterfallMode>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "waterfallgap")]
    waterfall_gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "waterfallgroupgap")]
    waterfall_group_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "piecolorway")]
    pie_colorway: Option<Vec<ColorWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extendpiecolors")]
    extend_pie_colors: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "sunburstcolorway")]
    sunburst_colorway: Option<Vec<ColorWrapper>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "extendsuburstcolors"
    )]
    extend_sunburst_colors: Option<bool>,
}

impl Layout {
    pub fn new() -> Layout {
        Default::default()
    }

    pub fn title(mut self, title: Title) -> Layout {
        self.title = Some(title);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> Layout {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend(mut self, legend: Legend) -> Layout {
        self.legend = Some(legend);
        self
    }

    pub fn margin(mut self, margin: Margin) -> Layout {
        self.margin = Some(margin);
        self
    }

    pub fn auto_size(mut self, auto_size: bool) -> Layout {
        self.auto_size = Some(auto_size);
        self
    }

    pub fn width(mut self, width: usize) -> Layout {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: usize) -> Layout {
        self.height = Some(height);
        self
    }

    pub fn font(mut self, font: Font) -> Layout {
        self.font = Some(font);
        self
    }

    pub fn uniform_text(mut self, uniform_text: UniformText) -> Layout {
        self.uniform_text = Some(uniform_text);
        self
    }

    pub fn separators(mut self, separators: &str) -> Layout {
        self.separators = Some(separators.to_owned());
        self
    }

    pub fn paper_background_color<C: Color>(mut self, paper_background_color: C) -> Layout {
        self.paper_background_color = Some(paper_background_color.to_color());
        self
    }

    pub fn plot_background_color<C: Color>(mut self, plot_background_color: C) -> Layout {
        self.plot_background_color = Some(plot_background_color.to_color());
        self
    }

    pub fn color_scale(mut self, color_scale: LayoutColorScale) -> Layout {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn colorway<C: Color>(mut self, colorway: Vec<C>) -> Layout {
        let colorway = private::to_color_array(colorway);
        self.colorway = Some(colorway);
        self
    }

    pub fn color_axis(mut self, color_axis: ColorAxis) -> Layout {
        self.color_axis = Some(color_axis);
        self
    }

    pub fn mode_bar(mut self, mode_bar: ModeBar) -> Layout {
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
    pub fn hover_mode(mut self, hover_mode: HoverMode) -> Layout {
        self.hover_mode = Some(TruthyEnum { e: hover_mode });
        self
    }

    pub fn click_mode(mut self, click_mode: &str) -> Layout {
        self.click_mode = Some(click_mode.to_owned());
        self
    }

    pub fn drag_mode(mut self, drag_mode: &str) -> Layout {
        self.drag_mode = Some(drag_mode.to_owned());
        self
    }

    pub fn select_direction(mut self, select_direction: &str) -> Layout {
        self.select_direction = Some(select_direction.to_owned());
        self
    }

    pub fn hover_distance(mut self, hover_distance: i32) -> Layout {
        self.hover_distance = Some(hover_distance);
        self
    }

    pub fn spike_distance(mut self, spike_distance: i32) -> Layout {
        self.spike_distance = Some(spike_distance);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Layout {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn grid(mut self, grid: LayoutGrid) -> Layout {
        self.grid = Some(grid);
        self
    }

    pub fn calendar(mut self, calendar: Calendar) -> Layout {
        self.calendar = Some(calendar);
        self
    }

    pub fn x_axis(mut self, xaxis: Axis) -> Layout {
        self.x_axis = Some(xaxis);
        self
    }

    pub fn y_axis(mut self, yaxis: Axis) -> Layout {
        self.y_axis = Some(yaxis);
        self
    }

    pub fn x_axis2(mut self, xaxis: Axis) -> Layout {
        self.x_axis2 = Some(xaxis);
        self
    }

    pub fn y_axis2(mut self, yaxis: Axis) -> Layout {
        self.y_axis2 = Some(yaxis);
        self
    }

    pub fn x_axis3(mut self, xaxis: Axis) -> Layout {
        self.x_axis3 = Some(xaxis);
        self
    }

    pub fn y_axis3(mut self, yaxis: Axis) -> Layout {
        self.y_axis3 = Some(yaxis);
        self
    }

    pub fn x_axis4(mut self, xaxis: Axis) -> Layout {
        self.x_axis4 = Some(xaxis);
        self
    }

    pub fn y_axis4(mut self, yaxis: Axis) -> Layout {
        self.y_axis4 = Some(yaxis);
        self
    }

    pub fn x_axis5(mut self, xaxis: Axis) -> Layout {
        self.x_axis5 = Some(xaxis);
        self
    }

    pub fn y_axis5(mut self, yaxis: Axis) -> Layout {
        self.y_axis5 = Some(yaxis);
        self
    }

    pub fn x_axis6(mut self, xaxis: Axis) -> Layout {
        self.x_axis6 = Some(xaxis);
        self
    }

    pub fn y_axis6(mut self, yaxis: Axis) -> Layout {
        self.y_axis6 = Some(yaxis);
        self
    }

    pub fn x_axis7(mut self, xaxis: Axis) -> Layout {
        self.x_axis7 = Some(xaxis);
        self
    }

    pub fn y_axis7(mut self, yaxis: Axis) -> Layout {
        self.y_axis7 = Some(yaxis);
        self
    }

    pub fn x_axis8(mut self, xaxis: Axis) -> Layout {
        self.x_axis8 = Some(xaxis);
        self
    }

    pub fn y_axis8(mut self, yaxis: Axis) -> Layout {
        self.y_axis8 = Some(yaxis);
        self
    }

    pub fn annotations(mut self, annotations: Vec<Annotation>) -> Layout {
        self.annotations = Some(annotations);
        self
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        if self.annotations.is_none() {
            self.annotations = Some(Vec::new());
        }
        self.annotations.as_mut().unwrap().push(annotation);
    }

    pub fn shapes(mut self, shapes: Vec<Shape>) -> Layout {
        self.shapes = Some(shapes);
        self
    }

    pub fn add_shape(&mut self, shape: Shape) {
        if self.shapes.is_none() {
            self.shapes = Some(Vec::new());
        }
        self.shapes.as_mut().unwrap().push(shape);
    }

    pub fn new_shape(mut self, new_shape: NewShape) -> Layout {
        self.new_shape = Some(new_shape);
        self
    }

    pub fn active_shape(mut self, active_shape: ActiveShape) -> Layout {
        self.active_shape = Some(active_shape);
        self
    }

    pub fn template(mut self, template: &str) -> Layout {
        self.template = Some(template.to_owned());
        self
    }

    pub fn box_mode(mut self, box_mode: BoxMode) -> Layout {
        self.box_mode = Some(box_mode);
        self
    }

    pub fn box_gap(mut self, box_gap: f64) -> Layout {
        self.box_gap = Some(box_gap);
        self
    }

    pub fn box_group_gap(mut self, box_group_gap: f64) -> Layout {
        self.box_group_gap = Some(box_group_gap);
        self
    }

    pub fn bar_mode(mut self, bar_mode: BarMode) -> Layout {
        self.bar_mode = Some(bar_mode);
        self
    }

    pub fn bar_norm(mut self, bar_norm: BarNorm) -> Layout {
        self.bar_norm = Some(bar_norm);
        self
    }

    pub fn bar_gap(mut self, bar_gap: f64) -> Layout {
        self.bar_gap = Some(bar_gap);
        self
    }

    pub fn bar_group_gap(mut self, bar_group_gap: f64) -> Layout {
        self.bar_group_gap = Some(bar_group_gap);
        self
    }

    pub fn violin_mode(mut self, violin_mode: ViolinMode) -> Layout {
        self.violin_mode = Some(violin_mode);
        self
    }

    pub fn violin_gap(mut self, violin_gap: f64) -> Layout {
        self.violin_gap = Some(violin_gap);
        self
    }

    pub fn violin_group_gap(mut self, violin_group_gap: f64) -> Layout {
        self.violin_group_gap = Some(violin_group_gap);
        self
    }

    pub fn waterfall_mode(mut self, waterfall_mode: WaterfallMode) -> Layout {
        self.waterfall_mode = Some(waterfall_mode);
        self
    }

    pub fn waterfall_gap(mut self, waterfall_gap: f64) -> Layout {
        self.waterfall_gap = Some(waterfall_gap);
        self
    }

    pub fn waterfall_group_gap(mut self, waterfall_group_gap: f64) -> Layout {
        self.waterfall_group_gap = Some(waterfall_group_gap);
        self
    }

    pub fn pie_colorway<C: Color>(mut self, pie_colorway: Vec<C>) -> Layout {
        let pie_colorway = private::to_color_array(pie_colorway);
        self.pie_colorway = Some(pie_colorway);
        self
    }

    pub fn extend_pie_colors(mut self, extend_pie_colors: bool) -> Layout {
        self.extend_pie_colors = Some(extend_pie_colors);
        self
    }

    pub fn sunburst_colorway<C: Color>(mut self, sunburst_colorway: Vec<C>) -> Layout {
        let sunburst_colorway = private::to_color_array(sunburst_colorway);
        self.sunburst_colorway = Some(sunburst_colorway);
        self
    }

    pub fn extend_sunburst_colors(mut self, extend_sunburst_colors: bool) -> Layout {
        self.extend_sunburst_colors = Some(extend_sunburst_colors);
        self
    }
}

impl Trace for Layout {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
