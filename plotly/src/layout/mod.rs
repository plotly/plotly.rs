use serde::{Serialize, Serializer};
use std::borrow::Cow;
use update_menu::UpdateMenu;

use crate::common::Domain;
use crate::{
    color::Color,
    common::{Calendar, ColorScale, Font, Label, Orientation, Title},
};
use plotly_derive::FieldSetter;

pub mod themes;
pub mod update_menu;

mod annotation;
mod axis;
mod grid;
mod legend;
mod shape;

// Re-export layout sub-module types
pub use self::annotation::{Annotation, ArrowSide, ClickToShow};
pub use self::axis::{
    ArrayShow, Axis, AxisConstrain, AxisType, CategoryOrder, ColorAxis, ConstrainDirection,
    RangeMode, RangeSelector, RangeSlider, RangeSliderYAxis, SelectorButton, SelectorStep,
    SliderRangeMode, StepMode, TicksDirection, TicksPosition,
};
pub use self::grid::{GridDomain, GridPattern, GridXSide, GridYSide, LayoutGrid, RowOrder};
pub use self::legend::{Legend, TraceOrder};
pub use self::shape::{
    ActiveShape, DrawDirection, FillRule, NewShape, Shape, ShapeLayer, ShapeLine, ShapeSizeMode,
    ShapeType,
};

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

#[derive(Debug, Clone)]
/// Determines the mode of drag interactions.
pub enum DragMode3D {
    Zoom,
    Pan,
    Turntable,
    Orbit,
    False,
}

impl Serialize for DragMode3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Zoom => serializer.serialize_str("zoom"),
            Self::Pan => serializer.serialize_str("pan"),
            Self::Turntable => serializer.serialize_str("turntable"),
            Self::Orbit => serializer.serialize_str("orbit"),
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
    /// Sets the domain within which the mapbox will be drawn.
    domain: Option<Domain>,
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
#[derive(Default)]
pub enum AspectMode {
    #[serde(rename = "auto")]
    #[default]
    Auto,
    #[serde(rename = "cube")]
    Cube,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "manual")]
    Manual,
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

#[derive(Default, Serialize, Debug, Clone)]
/// Sets the projection type. The projection type could be either "perspective"
/// or "orthographic".
/// Default: "perspective"
pub enum ProjectionType {
    #[default]
    #[serde(rename = "perspective")]
    Perspective,
    #[serde(rename = "orthographic")]
    Orthographic,
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
/// Container for Projection options.
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
    drag_mode: Option<DragMode3D>,
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
    fn serialize_uniform_text_mode() {
        assert_eq!(to_value(UniformTextMode::False).unwrap(), json!(false));
        assert_eq!(to_value(UniformTextMode::Hide).unwrap(), json!("hide"));
        assert_eq!(to_value(UniformTextMode::Show).unwrap(), json!("show"));
    }

    #[test]
    fn serialize_hover_mode() {
        assert_eq!(to_value(HoverMode::X).unwrap(), json!("x"));
        assert_eq!(to_value(HoverMode::Y).unwrap(), json!("y"));
        assert_eq!(to_value(HoverMode::Closest).unwrap(), json!("closest"));
        assert_eq!(to_value(HoverMode::False).unwrap(), json!(false));
        assert_eq!(to_value(HoverMode::XUnified).unwrap(), json!("x unified"));
        assert_eq!(to_value(HoverMode::YUnified).unwrap(), json!("y unified"));
    }

    #[test]
    fn serialize_bar_mode() {
        assert_eq!(to_value(BarMode::Stack).unwrap(), json!("stack"));
        assert_eq!(to_value(BarMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(BarMode::Overlay).unwrap(), json!("overlay"));
        assert_eq!(to_value(BarMode::Relative).unwrap(), json!("relative"));
    }

    #[test]
    fn serialize_bar_norm() {
        assert_eq!(to_value(BarNorm::Empty).unwrap(), json!(""));
        assert_eq!(to_value(BarNorm::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(BarNorm::Percent).unwrap(), json!("percent"));
    }

    #[test]
    fn serialize_box_mode() {
        assert_eq!(to_value(BoxMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(BoxMode::Overlay).unwrap(), json!("overlay"));
    }

    #[test]
    fn serialize_violin_mode() {
        assert_eq!(to_value(ViolinMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(ViolinMode::Overlay).unwrap(), json!("overlay"));
    }

    #[test]
    fn serialize_waterfall_mode() {
        assert_eq!(to_value(WaterfallMode::Group).unwrap(), json!("group"));
        assert_eq!(to_value(WaterfallMode::Overlay).unwrap(), json!("overlay"));
    }

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
    #[rustfmt::skip]
    fn serialize_click_mode() {
        assert_eq!(to_value(ClickMode::Event).unwrap(), json!("event"));
        assert_eq!(to_value(ClickMode::Select).unwrap(), json!("select"));
        assert_eq!(to_value(ClickMode::EventAndSelect).unwrap(), json!("event+select"));
        assert_eq!(to_value(ClickMode::None).unwrap(), json!("none"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_drag_mode() {
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
    fn serialize_mapbox_style() {
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

    #[test]
    fn serialize_layout_scene() {
        let layout = Layout::new().scene(
            LayoutScene::new()
                .x_axis(Axis::new())
                .y_axis(Axis::new())
                .z_axis(Axis::new())
                .camera(Camera::new())
                .aspect_mode(AspectMode::Auto)
                .hover_mode(HoverMode::Closest)
                .drag_mode(DragMode3D::Turntable)
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
    fn serialize_eye() {
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
    fn serialize_projection() {
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
    fn serialize_camera_center() {
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
    fn serialize_aspect_ratio() {
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
    fn serialize_aspect_mode() {
        let aspect_mode = AspectMode::default();

        assert_eq!(to_value(aspect_mode).unwrap(), json!("auto"));

        let aspect_mode = AspectMode::Data;

        assert_eq!(to_value(aspect_mode).unwrap(), json!("data"));

        let aspect_mode = AspectMode::Cube;

        assert_eq!(to_value(aspect_mode).unwrap(), json!("cube"));
    }

    #[test]
    fn serialize_up() {
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
