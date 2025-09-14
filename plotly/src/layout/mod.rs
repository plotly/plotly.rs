use std::borrow::Cow;

use plotly_derive::layout_structs;
use plotly_derive::FieldSetter;
use serde::Serialize;
use update_menu::UpdateMenu;

use crate::color::Color;
use crate::common::{Calendar, ColorScale, Font, Label, Orientation, Title};

pub mod themes;
pub mod update_menu;

mod animation;
mod annotation;
mod axis;
mod geo;
mod grid;
mod legend;
mod mapbox;
mod modes;
mod polar;
mod rangebreaks;
mod scene;
mod shape;
mod slider;

// Re-export layout sub-module types
pub use self::animation::{
    Animation, AnimationDirection, AnimationEasing, AnimationMode, AnimationOptions, Frame,
    FrameSettings, TransitionOrdering, TransitionSettings,
};
pub use self::annotation::{Annotation, ArrowSide, ClickToShow};
pub use self::axis::{
    ArrayShow, Axis, AxisConstrain, AxisRange, AxisType, CategoryOrder, ColorAxis,
    ConstrainDirection, RangeMode, RangeSelector, RangeSlider, RangeSliderYAxis, SelectorButton,
    SelectorStep, SliderRangeMode, SpikeMode, SpikeSnap, StepMode, TicksDirection, TicksPosition,
};
pub use self::geo::LayoutGeo;
pub use self::grid::{GridDomain, GridPattern, GridXSide, GridYSide, LayoutGrid, RowOrder};
pub use self::legend::{GroupClick, ItemClick, ItemSizing, Legend, TraceOrder};
pub use self::mapbox::{Center, Mapbox, MapboxStyle};
pub use self::modes::{
    AspectMode, BarMode, BarNorm, BoxMode, ClickMode, UniformTextMode, ViolinMode, WaterfallMode,
};
pub use self::polar::{
    AngularAxis, AngularAxisType, AutoRange, AutoRangeOptions, AutoTypeNumbers, AxisLayer,
    GridShape, Hole, LayoutPolar, MinorLogLabels, PolarAxisAttributes, PolarAxisTicks,
    PolarDirection, PolarTickMode, RadialAxis, RadialAxisType, ThetaUnit,
};
pub use self::rangebreaks::RangeBreak;
pub use self::scene::{
    AspectRatio, Camera, CameraCenter, DragMode, DragMode3D, Eye, HoverMode, LayoutScene,
    Projection, ProjectionType, Rotation, Up,
};
pub use self::shape::{
    ActiveShape, DrawDirection, FillRule, NewShape, Shape, ShapeLayer, ShapeLine, ShapeSizeMode,
    ShapeType,
};
pub use self::slider::{
    Slider, SliderCurrentValue, SliderCurrentValueXAnchor, SliderMethod, SliderStep,
    SliderStepBuilder, SliderTransition, SliderTransitionEasing,
};

/// Error type for ControlBuilder operations
#[derive(Debug)]
pub enum ControlBuilderError {
    RestyleSerializationError(String),
    RelayoutSerializationError(String),
    ValueSerializationError(String),
    InvalidRestyleObject(String),
    InvalidRelayoutObject(String),
    AnimationSerializationError(String),
}

impl std::fmt::Display for ControlBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlBuilderError::RestyleSerializationError(e) => {
                write!(f, "Failed to serialize restyle: {e}")
            }
            ControlBuilderError::RelayoutSerializationError(e) => {
                write!(f, "Failed to serialize relayout: {e}")
            }
            ControlBuilderError::ValueSerializationError(e) => {
                write!(f, "Failed to serialize value: {e}")
            }
            ControlBuilderError::InvalidRestyleObject(s) => {
                write!(f, "Invalid restyle object: expected object but got {s}")
            }
            ControlBuilderError::InvalidRelayoutObject(s) => {
                write!(f, "Invalid relayout object: expected object but got {s}")
            }
            ControlBuilderError::AnimationSerializationError(e) => {
                write!(f, "Failed to serialize animation: {e}")
            }
        }
    }
}

impl std::error::Error for ControlBuilderError {}

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

/// Generates Layout and LayoutTemplate
/// LayoutTemplate matches Layout except it lacks a field for template
/// See layout/layout.rs for the full field list and doc comments.
/// Layout is identical to LayoutTemplate except it has a `template` field and
/// #[field_setter(kind = "layout")] See layout/layout.rs for the full field
/// list and doc comments.
#[layout_structs]
pub struct LayoutFields {
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
    #[serde(flatten)]
    #[field_setter(skip)]
    axes: Option<std::collections::BTreeMap<String, Box<Axis>>>,
    // Keeping these hardcoded axes for backward compatibility
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
    geo: Option<LayoutGeo>,
    polar: Option<LayoutPolar>,
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
    sliders: Option<Vec<Slider>>,
}

impl Layout {
    pub fn axis_by_name<S: Into<String>>(&mut self, axis_name: S, axis: Axis) -> &mut Self {
        if self.axes.is_none() {
            self.axes = Some(std::collections::BTreeMap::new());
        }
        self.axes
            .as_mut()
            .unwrap()
            .insert(axis_name.into(), Box::new(axis));
        self
    }

    pub fn set_x_axis(&mut self, index: usize, axis: Axis) -> &mut Self {
        let key = if index == 1 {
            "xaxis".to_string()
        } else {
            format!("xaxis{index}")
        };
        self.axis_by_name(key, axis)
    }
    pub fn set_y_axis(&mut self, index: usize, axis: Axis) -> &mut Self {
        let key = if index == 1 {
            "yaxis".to_string()
        } else {
            format!("yaxis{index}")
        };
        self.axis_by_name(key, axis)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    fn serialize_valign() {
        assert_eq!(to_value(VAlign::Top).unwrap(), json!("top"));
        assert_eq!(to_value(VAlign::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(VAlign::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    fn serialize_halign() {
        assert_eq!(to_value(HAlign::Left).unwrap(), json!("left"));
        assert_eq!(to_value(HAlign::Center).unwrap(), json!("center"));
        assert_eq!(to_value(HAlign::Right).unwrap(), json!("right"));
    }
    #[test]
    fn serialize_margin() {
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
    fn serialize_layout_color_scale() {
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
    fn serialize_mode_bar() {
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
    fn serialize_arrow_side() {
        assert_eq!(to_value(ArrowSide::End).unwrap(), json!("end"));
        assert_eq!(to_value(ArrowSide::Start).unwrap(), json!("start"));
        assert_eq!(to_value(ArrowSide::StartEnd).unwrap(), json!("end+start"));
        assert_eq!(to_value(ArrowSide::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_select_direction() {
        assert_eq!(to_value(SelectDirection::Horizontal).unwrap(), json!("h"));
        assert_eq!(to_value(SelectDirection::Vertical).unwrap(), json!("v"));
        assert_eq!(to_value(SelectDirection::Diagonal).unwrap(), json!("d"));
        assert_eq!(to_value(SelectDirection::Any).unwrap(), json!("any"));
    }

    #[test]
    fn serialize_layout_template() {
        let layout_template = LayoutTemplate::new()
            .title("Title")
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
            .z_axis(Axis::new())
            .z_axis2(Axis::new())
            .z_axis3(Axis::new())
            .z_axis4(Axis::new())
            .z_axis5(Axis::new())
            .z_axis6(Axis::new())
            .z_axis7(Axis::new())
            .z_axis8(Axis::new())
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
            .mapbox(Mapbox::new())
            .update_menus(vec![UpdateMenu::new()])
            .sliders(vec![Slider::new()]);

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
            "zaxis": {},
            "zaxis2": {},
            "zaxis3": {},
            "zaxis4": {},
            "zaxis5": {},
            "zaxis6": {},
            "zaxis7": {},
            "zaxis8": {},
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
            "mapbox": {},
            "updatemenus": [{}],
            "sliders": [{}],
        });

        assert_eq!(to_value(layout_template).unwrap(), expected);
    }

    #[test]
    fn serialize_template() {
        let template = Template::new().layout(LayoutTemplate::new());
        let expected = json!({"layout": {}});

        assert_eq!(to_value(template).unwrap(), expected);
    }

    #[test]
    fn serialize_layout() {
        let layout = Layout::new()
            .title("Title")
            .title(String::from("Title"))
            .title(Title::with_text("Title"))
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
}
