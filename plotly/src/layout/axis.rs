use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::Color;
use crate::common::{
    Anchor, AxisSide, Calendar, ColorBar, ColorScale, DashType, ExponentFormat, Font,
    TickFormatStop, TickMode, Title,
};
use crate::layout::RangeBreak;
use crate::private::NumOrStringCollection;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpikeSnap {
    Data,
    Cursor,
    #[serde(rename = "hovered data")]
    HoveredData,
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
pub enum RangeMode {
    Normal,
    ToZero,
    NonNegative,
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
pub enum ArrayShow {
    All,
    First,
    Last,
    None,
}

#[derive(Serialize, Debug, Clone)]
pub enum CategoryOrder {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "category ascending")]
    CategoryAscending,
    #[serde(rename = "category descending")]
    CategoryDescending,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "total ascending")]
    TotalAscending,
    #[serde(rename = "total descending")]
    TotalDescending,
    #[serde(rename = "min ascending")]
    MinAscending,
    #[serde(rename = "min descending")]
    MinDescending,
    #[serde(rename = "max ascending")]
    MaxAscending,
    #[serde(rename = "max descending")]
    MaxDescending,
    #[serde(rename = "sum ascending")]
    SumAscending,
    #[serde(rename = "sum descending")]
    SumDescending,
    #[serde(rename = "mean ascending")]
    MeanAscending,
    #[serde(rename = "mean descending")]
    MeanDescending,
    #[serde(rename = "geometric mean ascending")]
    GeometricMeanAscending,
    #[serde(rename = "geometric mean descending")]
    GeometricMeanDescending,
    #[serde(rename = "median ascending")]
    MedianAscending,
    #[serde(rename = "median descending")]
    MedianDescending,
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SliderRangeMode {
    Auto,
    Fixed,
    Match,
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
pub struct Axis {
    visible: Option<bool>,
    /// Sets the order in which categories on this axis appear. Only has an
    /// effect if `category_order` is set to [`CategoryOrder::Array`].
    /// Used with `category_order`.
    #[serde(rename = "categoryarray")]
    category_array: Option<NumOrStringCollection>,
    /// Specifies the ordering logic for the case of categorical variables.
    /// By default, plotly uses [`CategoryOrder::Trace`], which specifies
    /// the order that is present in the data supplied. Set `category_order` to
    /// [`CategoryOrder::CategoryAscending`] or
    /// [`CategoryOrder::CategoryDescending`] if order should be determined
    /// by the alphanumerical order of the category names. Set `category_order`
    /// to [`CategoryOrder::Array`] to derive the ordering from the attribute
    /// `category_array`. If a category is not found in the `category_array`
    /// array, the sorting behavior for that attribute will be identical to the
    /// [`CategoryOrder::Trace`] mode. The unspecified categories will follow
    /// the categories in `category_array`. Set `category_order` to
    /// [`CategoryOrder::TotalAscending`] or
    /// [`CategoryOrder::TotalDescending`] if order should be determined by the
    /// numerical order of the values. Similarly, the order can be determined
    /// by the min, max, sum, mean, geometric mean or median of all the values.
    #[serde(rename = "categoryorder")]
    category_order: Option<CategoryOrder>,
    color: Option<Box<dyn Color>>,
    title: Option<Title>,
    #[field_setter(skip)]
    r#type: Option<AxisType>,
    #[serde(rename = "autorange")]
    auto_range: Option<bool>,
    #[serde(rename = "rangebreaks")]
    range_breaks: Option<Vec<RangeBreak>>,
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

    #[serde(rename = "scaleanchor")]
    scale_anchor: Option<String>,
    #[serde(rename = "scaleratio")]
    scale_ratio: Option<f64>,

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

    pub fn matches(mut self, matches: &str) -> Self {
        self.matches = Some(matches.to_string());
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

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    #[rustfmt::skip]
    fn serialize_axis_type() {
        assert_eq!(to_value(AxisType::Default).unwrap(), json!("-"));
        assert_eq!(to_value(AxisType::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(AxisType::Log).unwrap(), json!("log"));
        assert_eq!(to_value(AxisType::Date).unwrap(), json!("date"));
        assert_eq!(to_value(AxisType::Category).unwrap(), json!("category"));
        assert_eq!(to_value(AxisType::MultiCategory).unwrap(), json!("multicategory"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_range_mode() {
        assert_eq!(to_value(RangeMode::Normal).unwrap(), json!("normal"));
        assert_eq!(to_value(RangeMode::ToZero).unwrap(), json!("tozero"));
        assert_eq!(to_value(RangeMode::NonNegative).unwrap(), json!("nonnegative"));
    }

    #[test]
    fn serialize_ticks_direction() {
        assert_eq!(to_value(TicksDirection::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(TicksDirection::Inside).unwrap(), json!("inside"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_ticks_position() {
        assert_eq!(to_value(TicksPosition::Labels).unwrap(), json!("labels"));
        assert_eq!(to_value(TicksPosition::Boundaries).unwrap(), json!("boundaries"));
    }

    #[test]
    fn serialize_axis_constrain() {
        assert_eq!(to_value(AxisConstrain::Range).unwrap(), json!("range"));
        assert_eq!(to_value(AxisConstrain::Domain).unwrap(), json!("domain"));
    }

    #[test]
    fn serialize_array_show() {
        assert_eq!(to_value(ArrayShow::All).unwrap(), json!("all"));
        assert_eq!(to_value(ArrayShow::First).unwrap(), json!("first"));
        assert_eq!(to_value(ArrayShow::Last).unwrap(), json!("last"));
        assert_eq!(to_value(ArrayShow::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_color_axis() {
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
    #[rustfmt::skip]
    fn serialize_constrain_direction() {
        assert_eq!(to_value(ConstrainDirection::Left).unwrap(), json!("left"));
        assert_eq!(to_value(ConstrainDirection::Center).unwrap(), json!("center"));
        assert_eq!(to_value(ConstrainDirection::Right).unwrap(), json!("right"));
        assert_eq!(to_value(ConstrainDirection::Top).unwrap(), json!("top"));
        assert_eq!(to_value(ConstrainDirection::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(ConstrainDirection::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_category_order() {
        assert_eq!(to_value(CategoryOrder::Trace).unwrap(), json!("trace"));
        assert_eq!(to_value(CategoryOrder::CategoryAscending).unwrap(), json!("category ascending"));
        assert_eq!(to_value(CategoryOrder::CategoryDescending).unwrap(), json!("category descending"));
        assert_eq!(to_value(CategoryOrder::Array).unwrap(), json!("array"));
        assert_eq!(to_value(CategoryOrder::TotalAscending).unwrap(), json!("total ascending"));
        assert_eq!(to_value(CategoryOrder::TotalDescending).unwrap(), json!("total descending"));
        assert_eq!(to_value(CategoryOrder::MinAscending).unwrap(), json!("min ascending"));
        assert_eq!(to_value(CategoryOrder::MinDescending).unwrap(), json!("min descending"));
        assert_eq!(to_value(CategoryOrder::MaxAscending).unwrap(), json!("max ascending"));
        assert_eq!(to_value(CategoryOrder::MaxDescending).unwrap(), json!("max descending"));
        assert_eq!(to_value(CategoryOrder::SumAscending).unwrap(), json!("sum ascending"));
        assert_eq!(to_value(CategoryOrder::SumDescending).unwrap(), json!("sum descending"));
        assert_eq!(to_value(CategoryOrder::MeanAscending).unwrap(), json!("mean ascending"));
        assert_eq!(to_value(CategoryOrder::MeanDescending).unwrap(), json!("mean descending"));
        assert_eq!(to_value(CategoryOrder::GeometricMeanAscending).unwrap(), json!("geometric mean ascending"));
        assert_eq!(to_value(CategoryOrder::GeometricMeanDescending).unwrap(), json!("geometric mean descending"));
        assert_eq!(to_value(CategoryOrder::MedianAscending).unwrap(), json!("median ascending"));
        assert_eq!(to_value(CategoryOrder::MedianDescending).unwrap(), json!("median descending"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_axis_side() {
        assert_eq!(to_value(AxisSide::Left).unwrap(), json!("left"));
        assert_eq!(to_value(AxisSide::Top).unwrap(), json!("top"));
        assert_eq!(to_value(AxisSide::Right).unwrap(), json!("right"));
        assert_eq!(to_value(AxisSide::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    fn serialize_selector_step() {
        assert_eq!(to_value(SelectorStep::Month).unwrap(), json!("month"));
        assert_eq!(to_value(SelectorStep::Year).unwrap(), json!("year"));
        assert_eq!(to_value(SelectorStep::Day).unwrap(), json!("day"));
        assert_eq!(to_value(SelectorStep::Hour).unwrap(), json!("hour"));
        assert_eq!(to_value(SelectorStep::Minute).unwrap(), json!("minute"));
        assert_eq!(to_value(SelectorStep::Second).unwrap(), json!("second"));
        assert_eq!(to_value(SelectorStep::All).unwrap(), json!("all"));
    }

    #[test]
    fn serialize_step_mode() {
        assert_eq!(to_value(StepMode::Backward).unwrap(), json!("backward"));
        assert_eq!(to_value(StepMode::ToDate).unwrap(), json!("todate"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_spike_mode() {
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
    fn serialize_spike_snap() {
        assert_eq!(to_value(SpikeSnap::Data).unwrap(), json!("data"));
        assert_eq!(to_value(SpikeSnap::Cursor).unwrap(), json!("cursor"));
        assert_eq!(to_value(SpikeSnap::HoveredData).unwrap(), json!("hovered data"));
    }

    #[test]
    fn serialize_selector_button() {
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
    fn serialize_range_selector() {
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
    fn serialize_range_slider() {
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
    fn serialize_range_slider_y_axis() {
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
    fn serialize_slider_range_mode() {
        assert_eq!(to_value(SliderRangeMode::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(SliderRangeMode::Fixed).unwrap(), json!("fixed"));
        assert_eq!(to_value(SliderRangeMode::Match).unwrap(), json!("match"));
    }

    #[test]
    fn serialize_axis() {
        let axis = Axis::new()
            .visible(false)
            .color("#678123")
            .title(Title::with_text("title"))
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
            .matches("x")
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
            .calendar(Calendar::Coptic)
            .category_order(CategoryOrder::Array)
            .category_array(vec!["Category0", "Category1"]);

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
            "categoryorder": "array",
            "categoryarray": ["Category0", "Category1"]
        });

        assert_eq!(to_value(axis).unwrap(), expected);
    }
}
