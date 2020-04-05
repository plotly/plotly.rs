use crate::common::color::Color;
use crate::common::{
    Anchor, Calendar, ColorBar, ColorScale, DashType, Font, Label, Orientation, Side,
    TickFormatStops, TickMode, Title,
};
use crate::plot::Trace;
use crate::private;
use serde::Serialize;
use crate::private::TruthyEnum;
use rand::distributions::Uniform;

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
    Cetegory,
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

#[derive(Serialize, Debug)]
pub struct Legend {
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<String>,
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
    valign: Option<Align>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
}

impl Legend {
    pub fn new() -> Legend {
        Legend {
            background_color: None,
            border_color: None,
            border_width: None,
            font: None,
            orientation: None,
            trace_order: None,
            trace_group_gap: None,
            item_sizing: None,
            item_click: None,
            item_double_click: None,
            x: None,
            x_anchor: None,
            y: None,
            y_anchor: None,
            valign: None,
            title: None,
        }
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> Legend {
        self.background_color = Some(background_color.to_color_string());
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> Legend {
        self.border_color = Some(border_color.to_color_string());
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

    pub fn valign(mut self, valign: Align) -> Legend {
        self.valign = Some(valign);
        self
    }

    pub fn title(mut self, title: Title) -> Legend {
        self.title = Some(title);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum Align {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "bottom")]
    Bottom,
}

#[derive(Serialize, Debug)]
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
        Margin {
            l: None,
            r: None,
            t: None,
            b: None,
            pad: None,
            auto_expand: None,
        }
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

#[derive(Serialize, Debug)]
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
        LayoutColorScale {
            sequential: None,
            sequential_minus: None,
            diverging: None,
        }
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
        ColorAxis {
            cauto: None,
            cmin: None,
            cmax: None,
            cmid: None,
            color_scale: None,
            auto_color_scale: None,
            reverse_scale: None,
            show_scale: None,
            color_bar: None,
        }
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

#[derive(Serialize, Debug)]
pub struct Axis {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<AxisType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "auto_range")]
    auto_range: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rangemode")]
    range_mode: Option<RangeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<f64>>,
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
    tick_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "automargin")]
    auto_margin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showspikes")]
    show_spikes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikecolor")]
    spike_color: Option<String>,
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
    tick_format_stops: Option<TickFormatStops>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverformat")]
    hover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showline")]
    show_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "linecolor")]
    line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "linewidth")]
    line_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridcolor")]
    grid_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridwidth")]
    grid_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zeroline")]
    zero_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zerolinecolor")]
    zero_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zerolinewidth")]
    zero_line_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showdividers")]
    show_dividers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dividercolor")]
    divider_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dividerwidth")]
    divider_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar: Option<Calendar>,
}

impl Axis {
    pub fn new() -> Axis {
        Axis {
            visible: None,
            color: None,
            title: None,
            r#type: None,
            auto_range: None,
            range_mode: None,
            range: None,
            fixed_range: None,
            constrain: None,
            constrain_toward: None,
            tick_mode: None,
            n_ticks: None,
            tick0: None,
            dtick: None,
            tick_values: None,
            tick_text: None,
            ticks: None,
            ticks_on: None,
            mirror: None,
            tick_length: None,
            tick_width: None,
            tick_color: None,
            show_tick_labels: None,
            auto_margin: None,
            show_spikes: None,
            spike_color: None,
            spike_thickness: None,
            spike_dash: None,
            spike_mode: None,
            spike_snap: None,
            tick_font: None,
            tick_angle: None,
            tick_prefix: None,
            show_tick_prefix: None,
            tick_suffix: None,
            show_tick_suffix: None,
            show_exponent: None,
            exponent_format: None,
            separate_thousands: None,
            tick_format: None,
            tick_format_stops: None,
            hover_format: None,
            show_line: None,
            line_color: None,
            line_width: None,
            show_grid: None,
            grid_color: None,
            grid_width: None,
            zero_line: None,
            zero_line_color: None,
            zero_line_width: None,
            show_dividers: None,
            divider_color: None,
            divider_width: None,
            side: None,
            domain: None,
            position: None,
            calendar: None,
        }
    }

    pub fn visible(mut self, visible: bool) -> Axis {
        self.visible = Some(visible);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Axis {
        self.color = Some(color.to_color_string());
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

    pub fn range(mut self, range: Vec<f64>) -> Axis {
        self.range = Some(range);
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
        self.tick_color = Some(tick_color.to_color_string());
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
        self.spike_color = Some(spike_color.to_color_string());
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

    pub fn tick_format_stops(mut self, tick_format_stops: TickFormatStops) -> Axis {
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
        self.line_color = Some(line_color.to_color_string());
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
        self.grid_color = Some(grid_color.to_color_string());
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
        self.zero_line_color = Some(zero_line_color.to_color_string());
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
        self.divider_color = Some(divider_color.to_color_string());
        self
    }

    pub fn divider_width(mut self, divider_width: usize) -> Axis {
        self.divider_width = Some(divider_width);
        self
    }

    pub fn side(mut self, side: Side) -> Axis {
        self.side = Some(side);
        self
    }

    pub fn domain(mut self, domain: Vec<f64>) -> Axis {
        self.domain = Some(domain);
        self
    }

    pub fn position(mut self, position: f64) -> Axis {
        self.position = Some(position);
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
    #[serde(rename = "independent")]
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

#[derive(Serialize, Debug)]
pub struct GridDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<f64>>,
}

impl GridDomain {
    pub fn new() -> GridDomain {
        GridDomain { x: None, y: None }
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

#[derive(Serialize, Debug)]
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
        LayoutGrid {
            rows: None,
            row_order: None,
            columns: None,
            sub_plots: None,
            x_axes: None,
            y_axes: None,
            pattern: None,
            x_gap: None,
            y_gap: None,
            domain: None,
            x_side: None,
            y_side: None,
        }
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

#[derive(Serialize, Debug)]
pub struct UniformText {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<TruthyEnum<UniformTextMode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<usize>,
}

impl UniformText {
    pub fn new() -> UniformText {
        UniformText {
            mode: None,
            min_size: None,
        }
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

#[derive(Serialize, Debug)]
pub struct ModeBar {
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activecolor")]
    active_color: Option<String>,
}

impl ModeBar {
    pub fn new() -> ModeBar {
        ModeBar {
            orientation: None,
            background_color: None,
            color: None,
            active_color: None,
        }
    }

    pub fn orientation<C: Color>(mut self, orientation: Orientation) -> ModeBar {
        self.orientation = Some(orientation);
        self
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> ModeBar {
        self.background_color = Some(background_color.to_color_string());
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> ModeBar {
        self.color = Some(color.to_color_string());
        self
    }

    pub fn active_color<C: Color>(mut self, active_color: C) -> ModeBar {
        self.active_color = Some(active_color.to_color_string());
        self
    }
}

#[derive(Serialize, Debug)]
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
    paper_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "plot_bgcolor")]
    plot_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<LayoutColorScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colorway: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    xaxis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yaxis: Option<Axis>,

    // ternary: Option<LayoutTernary>,
    // scene: Option<LayoutScene>,

    // polar: Option<LayoutPolar>,
    // annotations: Option<LayoutAnnotations>,
    // shapes: Option<LayoutShapes>,
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
    pie_colorway: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extendpiecolors")]
    extend_pie_colors: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "sunburstcolorway")]
    sunburst_colorway: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extendsuburstcolors")]
    extend_sunburst_colors: Option<bool>,
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            title: None,
            show_legend: None,
            legend: None,
            margin: None,
            auto_size: None,
            width: None,
            height: None,
            font: None,
            uniform_text: None,
            separators: None,
            paper_background_color: None,
            plot_background_color: None,
            color_scale: None,
            colorway: None,
            color_axis: None,
            mode_bar: None,
            hover_mode: None,
            click_mode: None,
            drag_mode: None,
            select_direction: None,
            hover_distance: None,
            spike_distance: None,
            hover_label: None,
            grid: None,
            calendar: None,
            xaxis: None,
            yaxis: None,
            template: None,
            box_mode: None,
            box_gap: None,
            box_group_gap: None,
            bar_mode: None,
            bar_norm: None,
            bar_gap: None,
            bar_group_gap: None,

            violin_mode: None,
            violin_gap: None,
            violin_group_gap: None,

            waterfall_mode: None,
            waterfall_gap: None,
            waterfall_group_gap: None,

            pie_colorway: None,
            extend_pie_colors: None,

            sunburst_colorway: None,
            extend_sunburst_colors: None,
        }
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
        self.paper_background_color = Some(paper_background_color.to_color_string());
        self
    }

    pub fn plot_background_color<C: Color>(mut self, plot_background_color: C) -> Layout {
        self.plot_background_color = Some(plot_background_color.to_color_string());
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

    pub fn xaxis(mut self, xaxis: Axis) -> Layout {
        self.xaxis = Some(xaxis);
        self
    }

    pub fn yaxis(mut self, yaxis: Axis) -> Layout {
        self.yaxis = Some(yaxis);
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
