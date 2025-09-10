pub mod color;

use plotly_derive::FieldSetter;
use serde::{Serialize, Serializer};

use crate::{
    color::{Color, ColorArray},
    private,
};

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Direction {
    Increasing { line: Line },
    Decreasing { line: Line },
}

#[derive(Clone, Debug)]
pub enum Visible {
    True,
    False,
    LegendOnly,
}

impl Serialize for Visible {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::True => serializer.serialize_bool(true),
            Self::False => serializer.serialize_bool(false),
            Self::LegendOnly => serializer.serialize_str("legendonly"),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HoverInfo {
    X,
    Y,
    Z,
    #[serde(rename = "x+y")]
    XAndY,
    #[serde(rename = "x+z")]
    XAndZ,
    #[serde(rename = "y+z")]
    YAndZ,
    #[serde(rename = "x+y+z")]
    XAndYAndZ,
    Text,
    Name,
    All,
    None,
    Skip,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct LegendGroupTitle {
    text: Option<String>,
    font: Option<Font>,
}

impl From<&str> for LegendGroupTitle {
    fn from(title: &str) -> Self {
        LegendGroupTitle::with_text(title)
    }
}

impl From<String> for LegendGroupTitle {
    fn from(value: String) -> Self {
        LegendGroupTitle::with_text(value)
    }
}

impl From<&String> for LegendGroupTitle {
    fn from(value: &String) -> Self {
        LegendGroupTitle::with_text(value)
    }
}

impl LegendGroupTitle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_text<S: Into<String>>(text: S) -> Self {
        LegendGroupTitle {
            text: Some(text.into()),
            ..Default::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Domain {
    column: Option<usize>,
    row: Option<usize>,
    x: Option<[f64; 2]>,
    y: Option<[f64; 2]>,
}

impl Domain {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn row(mut self, row: usize) -> Self {
        self.row = Some(row);
        self
    }

    pub fn x(mut self, x: &[f64; 2]) -> Self {
        self.x = Some(x.to_owned());
        self
    }

    pub fn y(mut self, y: &[f64; 2]) -> Self {
        self.y = Some(y.to_owned());
        self
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TextPosition {
    Inside,
    Outside,
    Auto,
    None,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConstrainText {
    Inside,
    Outside,
    Both,
    None,
}

#[derive(Serialize, Clone, Debug)]
pub enum Orientation {
    #[serde(rename = "a")]
    Auto,
    #[serde(rename = "v")]
    Vertical,
    #[serde(rename = "h")]
    Horizontal,
    #[serde(rename = "r")]
    Radial,
    #[serde(rename = "t")]
    Tangential,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Fill {
    ToZeroY,
    ToZeroX,
    ToNextY,
    ToNextX,
    ToSelf,
    ToNext,
    None,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Calendar {
    Gregorian,
    Chinese,
    Coptic,
    DiscWorld,
    Ethiopian,
    Hebrew,
    Islamic,
    Julian,
    Mayan,
    Nanakshahi,
    Nepali,
    Persian,
    Jalali,
    Taiwan,
    Thai,
    Ummalqura,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Dim<T>
where
    T: Serialize,
{
    Scalar(T),
    Vector(Vec<T>),
    Matrix(Vec<Vec<T>>),
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlotType {
    Scatter,
    ScatterGL,
    Scatter3D,
    ScatterMapbox,
    ScatterGeo,
    ScatterPolar,
    ScatterPolarGL,
    Bar,
    Box,
    Candlestick,
    Contour,
    HeatMap,
    Histogram,
    Histogram2dContour,
    Image,
    Mesh3D,
    Ohlc,
    Sankey,
    Surface,
    DensityMapbox,
    Table,
    Pie,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Lines,
    Markers,
    Text,
    #[serde(rename = "lines+markers")]
    LinesMarkers,
    #[serde(rename = "lines+text")]
    LinesText,
    #[serde(rename = "markers+text")]
    MarkersText,
    #[serde(rename = "lines+markers+text")]
    LinesMarkersText,
    None,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Ticks {
    Outside,
    Inside,
    #[serde(rename = "")]
    None,
}

#[derive(Serialize, Clone, Debug)]
pub enum Position {
    #[serde(rename = "top left")]
    TopLeft,
    #[serde(rename = "top center")]
    TopCenter,
    #[serde(rename = "top right")]
    TopRight,
    #[serde(rename = "middle left")]
    MiddleLeft,
    #[serde(rename = "middle center")]
    MiddleCenter,
    #[serde(rename = "middle right")]
    MiddleRight,
    #[serde(rename = "bottom left")]
    BottomLeft,
    #[serde(rename = "bottom center")]
    BottomCenter,
    #[serde(rename = "bottom right")]
    BottomRight,
    #[serde(rename = "inside")]
    Inside,
    #[serde(rename = "outside")]
    Outside,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MarkerSymbol {
    Circle,
    CircleOpen,
    CircleDot,
    CircleOpenDot,
    Square,
    SquareOpen,
    SquareDot,
    SquareOpenDot,
    Diamond,
    DiamondOpen,
    DiamondDot,
    DiamondOpenDot,
    Cross,
    CrossOpen,
    CrossDot,
    CrossOpenDot,
    X,
    XOpen,
    XDot,
    XOpenDot,
    TriangleUp,
    TriangleUpOpen,
    TriangleUpDot,
    TriangleUpOpenDot,
    TriangleDown,
    TriangleDownOpen,
    TriangleDownDot,
    TriangleDownOpenDot,
    TriangleLeft,
    TriangleLeftOpen,
    TriangleLeftDot,
    TriangleLeftOpenDot,
    TriangleRight,
    TriangleRightOpen,
    TriangleRightDot,
    TriangleRightOpenDot,
    #[serde(rename = "triangle-ne")]
    TriangleNE,
    #[serde(rename = "triangle-ne-open")]
    TriangleNEOpen,
    #[serde(rename = "triangle-ne-dot")]
    TriangleNEDot,
    #[serde(rename = "triangle-ne-open-dot")]
    TriangleNEOpenDot,
    #[serde(rename = "triangle-se")]
    TriangleSE,
    #[serde(rename = "triangle-se-open")]
    TriangleSEOpen,
    #[serde(rename = "triangle-se-dot")]
    TriangleSEDot,
    #[serde(rename = "triangle-se-open-dot")]
    TriangleSEOpenDot,
    #[serde(rename = "triangle-sw")]
    TriangleSW,
    #[serde(rename = "triangle-sw-open")]
    TriangleSWOpen,
    #[serde(rename = "triangle-sw-dot")]
    TriangleSWDot,
    #[serde(rename = "triangle-sw-open-dot")]
    TriangleSWOpenDot,
    #[serde(rename = "triangle-nw")]
    TriangleNW,
    #[serde(rename = "triangle-nw-open")]
    TriangleNWOpen,
    #[serde(rename = "triangle-nw-dot")]
    TriangleNWDot,
    #[serde(rename = "triangle-nw-open-dot")]
    TriangleNWOpenDot,
    Pentagon,
    PentagonOpen,
    PentagonDot,
    PentagonOpenDot,
    Hexagon,
    HexagonOpen,
    HexagonDot,
    HexagonOpenDot,
    Hexagon2,
    Hexagon2Open,
    Hexagon2Dot,
    Hexagon2OpenDot,
    Octagon,
    OctagonOpen,
    OctagonDot,
    OctagonOpenDot,
    Star,
    StarOpen,
    StarDot,
    StarOpenDot,
    Hexagram,
    HexagramOpen,
    HexagramDot,
    HexagramOpenDot,
    StarTriangleUp,
    StarTriangleUpOpen,
    StarTriangleUpDot,
    StarTriangleUpOpenDot,
    StarTriangleDown,
    StarTriangleDownOpen,
    StarTriangleDownDot,
    StarTriangleDownOpenDot,
    StarSquare,
    StarSquareOpen,
    StarSquareDot,
    StarSquareOpenDot,
    StarDiamond,
    StarDiamondOpen,
    StarDiamondDot,
    StarDiamondOpenDot,
    DiamondTall,
    DiamondTallOpen,
    DiamondTallDot,
    DiamondTallOpenDot,
    DiamondWide,
    DiamondWideOpen,
    DiamondWideDot,
    DiamondWideOpenDot,
    Hourglass,
    HourglassOpen,
    #[serde(rename = "bowtie")]
    BowTie,
    #[serde(rename = "bowtie-open")]
    BowTieOpen,
    CircleCross,
    CircleCrossOpen,
    CircleX,
    CircleXOpen,
    SquareCross,
    SquareCrossOpen,
    SquareX,
    SquareXOpen,
    DiamondCross,
    DiamondCrossOpen,
    DiamondX,
    DiamondXOpen,
    CrossThin,
    CrossThinOpen,
    XThin,
    XThinOpen,
    Asterisk,
    AsteriskOpen,
    Hash,
    HashOpen,
    HashDot,
    HashOpenDot,
    YUp,
    YUpOpen,
    YDown,
    YDownOpen,
    YLeft,
    YLeftOpen,
    YRight,
    YRightOpen,
    #[serde(rename = "line-ew")]
    LineEW,
    #[serde(rename = "line-ew-open")]
    LineEWOpen,
    #[serde(rename = "line-ns")]
    LineNS,
    #[serde(rename = "line-ns-open")]
    LineNSOpen,
    #[serde(rename = "line-ne")]
    LineNE,
    #[serde(rename = "line-ne-open")]
    LineNEOpen,
    #[serde(rename = "line-nw")]
    LineNW,
    #[serde(rename = "line-nw-open")]
    LineNWOpen,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TickMode {
    Auto,
    Linear,
    Array,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DashType {
    Solid,
    Dot,
    Dash,
    LongDash,
    DashDot,
    LongDashDot,
}

#[derive(Serialize, Clone, Debug)]
pub struct ColorScaleElement(pub f64, pub String);

#[derive(Serialize, Clone, Debug)]
pub enum ColorScalePalette {
    Greys,
    YlGnBu,
    Greens,
    YlOrRd,
    Bluered,
    RdBu,
    Reds,
    Blues,
    Picnic,
    Rainbow,
    Portland,
    Jet,
    Hot,
    Blackbody,
    Earth,
    Electric,
    Viridis,
    Cividis,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ColorScale {
    Palette(ColorScalePalette),
    Vector(Vec<ColorScaleElement>),
}

impl From<ColorScalePalette> for ColorScale {
    fn from(src: ColorScalePalette) -> Self {
        ColorScale::Palette(src)
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LineShape {
    Linear,
    Spline,
    Hv,
    Vh,
    Hvh,
    Vhv,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Line {
    width: Option<f64>,
    shape: Option<LineShape>,
    smoothing: Option<f64>,
    dash: Option<DashType>,
    simplify: Option<bool>,
    color: Option<Box<dyn Color>>,
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
    #[serde(rename = "outliercolor")]
    outlier_color: Option<Box<dyn Color>>,
    #[serde(rename = "outlierwidth")]
    outlier_width: Option<usize>,
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GradientType {
    Radial,
    Horizontal,
    Vertical,
    None,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SizeMode {
    Diameter,
    Area,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ThicknessMode {
    Fraction,
    Pixels,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Anchor {
    Auto,
    Left,
    Center,
    Right,
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ExponentFormat {
    None,
    #[serde(rename = "e")]
    SmallE,
    #[serde(rename = "E")]
    CapitalE,
    Power,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "B")]
    B,
}

#[derive(Serialize, Clone, Debug)]
pub struct Gradient {
    r#type: GradientType,
    color: Dim<Box<dyn Color>>,
}

impl Gradient {
    pub fn new<C: Color>(gradient_type: GradientType, color: C) -> Self {
        Gradient {
            r#type: gradient_type,
            color: Dim::Scalar(Box::new(color)),
        }
    }

    pub fn new_array<C: Color>(gradient_type: GradientType, colors: Vec<C>) -> Self {
        Gradient {
            r#type: gradient_type,
            color: Dim::Vector(ColorArray(colors).into()),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct TickFormatStop {
    #[field_setter(default = "true")]
    enabled: bool,
    #[serde(rename = "dtickrange")]
    dtick_range: Option<private::NumOrStringCollection>,
    value: Option<String>,
    name: Option<String>,
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl TickFormatStop {
    pub fn new() -> Self {
        TickFormatStop {
            enabled: true,
            ..Default::default()
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Show {
    All,
    First,
    Last,
    None,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct ColorBar {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    #[serde(rename = "borderwidth")]
    border_width: Option<usize>,
    dtick: Option<f64>,
    #[serde(rename = "exponentformat")]
    exponent_format: Option<ExponentFormat>,
    len: Option<usize>,
    #[serde(rename = "lenmode")]
    len_mode: Option<ThicknessMode>,
    #[serde(rename = "nticks")]
    n_ticks: Option<usize>,
    orientation: Option<Orientation>,
    #[serde(rename = "outlinecolor")]
    outline_color: Option<Box<dyn Color>>,
    #[serde(rename = "outlinewidth")]
    outline_width: Option<usize>,
    #[serde(rename = "separatethousands")]
    separate_thousands: Option<bool>,
    #[serde(rename = "showexponent")]
    show_exponent: Option<Show>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(rename = "showtickprefix")]
    show_tick_prefix: Option<Show>,
    #[serde(rename = "showticksuffix")]
    show_tick_suffix: Option<Show>,
    thickness: Option<usize>,
    #[serde(rename = "thicknessmode")]
    thickness_mode: Option<ThicknessMode>,
    #[serde(rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    #[serde(rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(rename = "tickformatstops")]
    tick_format_stops: Option<Vec<TickFormatStop>>,
    #[serde(rename = "ticklen")]
    tick_len: Option<usize>,
    #[serde(rename = "tickmode")]
    tick_mode: Option<TickMode>,
    #[serde(rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    #[serde(rename = "tickvals")]
    tick_vals: Option<Vec<f64>>,
    #[serde(rename = "tickwidth")]
    tick_width: Option<usize>,
    tick0: Option<f64>,
    ticks: Option<Ticks>,
    title: Option<Title>,
    x: Option<f64>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(rename = "xpad")]
    x_pad: Option<f64>,
    y: Option<f64>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    #[serde(rename = "ypad")]
    y_pad: Option<f64>,
}

impl ColorBar {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AxisSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Serialize, Debug, Clone)]
pub enum PatternShape {
    #[serde(rename = "")]
    None,
    #[serde(rename = "-")]
    HorizontalLine,
    #[serde(rename = "|")]
    VerticalLine,
    #[serde(rename = "/")]
    RightDiagonalLine,
    #[serde(rename = "\\")]
    LeftDiagonalLine,
    #[serde(rename = "+")]
    Cross,
    #[serde(rename = "x")]
    DiagonalCross,
    #[serde(rename = ".")]
    Dot,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PatternFillMode {
    Replace,
    Overlay,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Pattern {
    shape: Option<Dim<PatternShape>>,
    #[serde(rename = "fillmode")]
    fill_mode: Option<PatternFillMode>,
    #[serde(rename = "bgcolor")]
    background_color: Option<Dim<Box<dyn Color>>>,
    #[serde(rename = "fgcolor")]
    foreground_color: Option<Dim<Box<dyn Color>>>,
    #[serde(rename = "fgopacity")]
    foreground_opacity: Option<f64>,
    size: Option<Dim<f64>>,
    solidity: Option<Dim<f64>>,
}

impl Pattern {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Marker {
    symbol: Option<MarkerSymbol>,
    opacity: Option<f64>,
    size: Option<Dim<usize>>,
    #[serde(rename = "maxdisplayed")]
    max_displayed: Option<usize>,
    #[serde(rename = "sizeref")]
    size_ref: Option<usize>,
    #[serde(rename = "sizemin")]
    size_min: Option<usize>,
    #[serde(rename = "sizemode")]
    size_mode: Option<SizeMode>,
    line: Option<Line>,
    gradient: Option<Gradient>,
    /// Marker option specific for Scatter and other common traces
    color: Option<Dim<Box<dyn Color>>>,
    /// Marker option specific for Pie charts to set the colors of the sectors
    #[field_setter(skip)]
    colors: Option<Vec<Box<dyn Color>>>,
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
    #[serde(rename = "outliercolor")]
    outlier_color: Option<Box<dyn Color>>,
    pattern: Option<Pattern>,
}

impl Marker {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn colors<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.colors = Some(ColorArray(colors).into());
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Font {
    family: Option<String>,
    size: Option<usize>,
    color: Option<Box<dyn Color>>,
}

impl Font {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Right,
    Top,
    Bottom,
    Left,
    #[serde(rename = "top left")]
    TopLeft,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Reference {
    Container,
    Paper,
}

#[derive(Serialize, Clone, Debug)]
pub struct Pad {
    t: usize,
    b: usize,
    l: usize,
}

impl Pad {
    pub fn new(t: usize, b: usize, l: usize) -> Self {
        Pad { t, b, l }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Title {
    text: Option<String>,
    font: Option<Font>,
    side: Option<Side>,
    #[serde(rename = "xref")]
    x_ref: Option<Reference>,
    #[serde(rename = "yref")]
    y_ref: Option<Reference>,
    x: Option<f64>,
    y: Option<f64>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    pad: Option<Pad>,
}

impl From<&str> for Title {
    fn from(title: &str) -> Self {
        Title::with_text(title)
    }
}

impl From<String> for Title {
    fn from(value: String) -> Self {
        Title::with_text(value)
    }
}

impl From<&String> for Title {
    fn from(value: &String) -> Self {
        Title::with_text(value)
    }
}

impl Title {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_text<S: Into<String>>(text: S) -> Self {
        Title {
            text: Some(text.into()),
            ..Default::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Label {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    font: Option<Font>,
    align: Option<String>,
    #[serde(rename = "namelength")]
    name_length: Option<Dim<i32>>,
}

impl Label {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum ErrorType {
    #[default]
    Percent,
    Constant,
    #[serde(rename = "sqrt")]
    SquareRoot,
    Data,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct ErrorData {
    #[field_setter(default = "ErrorType::Percent")]
    r#type: ErrorType,
    array: Option<Vec<f64>>,
    visible: Option<bool>,
    symmetric: Option<bool>,
    #[serde(rename = "arrayminus")]
    array_minus: Option<Vec<f64>>,
    value: Option<f64>,
    #[serde(rename = "valueminus")]
    value_minus: Option<f64>,
    #[serde(rename = "traceref")]
    trace_ref: Option<usize>,
    #[serde(rename = "tracerefminus")]
    trace_ref_minus: Option<usize>,
    copy_ystyle: Option<bool>,
    color: Option<Box<dyn Color>>,
    thickness: Option<f64>,
    width: Option<usize>,
}

impl ErrorData {
    pub fn new(error_type: ErrorType) -> Self {
        ErrorData {
            r#type: error_type,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HoverOn {
    Points,
    Fills,
    #[serde(rename = "points+fills")]
    PointsAndFills,
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::color::NamedColor;

    #[test]
    fn serialize_domain() {
        let domain = Domain::new().column(0).row(0).x(&[0., 1.]).y(&[0., 1.]);
        let expected = json!({
            "column": 0,
            "row": 0,
            "x": [0.0, 1.0],
            "y": [0.0, 1.0],
        });

        assert_eq!(to_value(domain).unwrap(), expected);
    }

    #[test]
    fn serialize_direction() {
        // TODO: I think `Direction` would be better as a struct, with `fillcolor` and
        // `line` attributes
        let inc = Direction::Increasing { line: Line::new() };
        let expected = json!({"line": {}});
        assert_eq!(to_value(inc).unwrap(), expected);

        let dec = Direction::Decreasing { line: Line::new() };
        let expected = json!({"line": {}});
        assert_eq!(to_value(dec).unwrap(), expected);
    }

    #[test]
    fn serialize_hover_info() {
        assert_eq!(to_value(HoverInfo::X).unwrap(), json!("x"));
        assert_eq!(to_value(HoverInfo::Y).unwrap(), json!("y"));
        assert_eq!(to_value(HoverInfo::Z).unwrap(), json!("z"));
        assert_eq!(to_value(HoverInfo::XAndY).unwrap(), json!("x+y"));
        assert_eq!(to_value(HoverInfo::XAndZ).unwrap(), json!("x+z"));
        assert_eq!(to_value(HoverInfo::YAndZ).unwrap(), json!("y+z"));
        assert_eq!(to_value(HoverInfo::XAndYAndZ).unwrap(), json!("x+y+z"));
        assert_eq!(to_value(HoverInfo::Text).unwrap(), json!("text"));
        assert_eq!(to_value(HoverInfo::Name).unwrap(), json!("name"));
        assert_eq!(to_value(HoverInfo::All).unwrap(), json!("all"));
        assert_eq!(to_value(HoverInfo::None).unwrap(), json!("none"));
        assert_eq!(to_value(HoverInfo::Skip).unwrap(), json!("skip"));
    }

    #[test]
    fn serialize_text_position() {
        assert_eq!(to_value(TextPosition::Inside).unwrap(), json!("inside"));
        assert_eq!(to_value(TextPosition::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(TextPosition::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(TextPosition::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_constrain_text() {
        assert_eq!(to_value(ConstrainText::Inside).unwrap(), json!("inside"));
        assert_eq!(to_value(ConstrainText::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(ConstrainText::Both).unwrap(), json!("both"));
        assert_eq!(to_value(ConstrainText::None).unwrap(), json!("none"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_orientation() {
        assert_eq!(to_value(Orientation::Vertical).unwrap(), json!("v"));
        assert_eq!(to_value(Orientation::Horizontal).unwrap(), json!("h"));
    }

    #[test]
    fn serialize_fill() {
        assert_eq!(to_value(Fill::ToZeroY).unwrap(), json!("tozeroy"));
        assert_eq!(to_value(Fill::ToZeroX).unwrap(), json!("tozerox"));
        assert_eq!(to_value(Fill::ToNextY).unwrap(), json!("tonexty"));
        assert_eq!(to_value(Fill::ToNextX).unwrap(), json!("tonextx"));
        assert_eq!(to_value(Fill::ToSelf).unwrap(), json!("toself"));
        assert_eq!(to_value(Fill::ToNext).unwrap(), json!("tonext"));
        assert_eq!(to_value(Fill::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_calendar() {
        assert_eq!(to_value(Calendar::Gregorian).unwrap(), json!("gregorian"));
        assert_eq!(to_value(Calendar::Chinese).unwrap(), json!("chinese"));
        assert_eq!(to_value(Calendar::Coptic).unwrap(), json!("coptic"));
        assert_eq!(to_value(Calendar::DiscWorld).unwrap(), json!("discworld"));
        assert_eq!(to_value(Calendar::Ethiopian).unwrap(), json!("ethiopian"));
        assert_eq!(to_value(Calendar::Hebrew).unwrap(), json!("hebrew"));
        assert_eq!(to_value(Calendar::Islamic).unwrap(), json!("islamic"));
        assert_eq!(to_value(Calendar::Julian).unwrap(), json!("julian"));
        assert_eq!(to_value(Calendar::Mayan).unwrap(), json!("mayan"));
        assert_eq!(to_value(Calendar::Nanakshahi).unwrap(), json!("nanakshahi"));
        assert_eq!(to_value(Calendar::Nepali).unwrap(), json!("nepali"));
        assert_eq!(to_value(Calendar::Persian).unwrap(), json!("persian"));
        assert_eq!(to_value(Calendar::Jalali).unwrap(), json!("jalali"));
        assert_eq!(to_value(Calendar::Taiwan).unwrap(), json!("taiwan"));
        assert_eq!(to_value(Calendar::Thai).unwrap(), json!("thai"));
        assert_eq!(to_value(Calendar::Ummalqura).unwrap(), json!("ummalqura"));
    }

    #[test]
    fn serialize_dim() {
        assert_eq!(to_value(Dim::Scalar(0)).unwrap(), json!(0));
        assert_eq!(to_value(Dim::Vector(vec![0])).unwrap(), json!([0]));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_plot_type() {
        assert_eq!(to_value(PlotType::Scatter).unwrap(), json!("scatter"));
        assert_eq!(to_value(PlotType::ScatterGL).unwrap(), json!("scattergl"));
        assert_eq!(to_value(PlotType::Scatter3D).unwrap(), json!("scatter3d"));
        assert_eq!(to_value(PlotType::ScatterGeo).unwrap(), json!("scattergeo"));
        assert_eq!(to_value(PlotType::ScatterPolar).unwrap(), json!("scatterpolar"));
        assert_eq!(to_value(PlotType::ScatterPolarGL).unwrap(), json!("scatterpolargl"));
        assert_eq!(to_value(PlotType::Bar).unwrap(), json!("bar"));
        assert_eq!(to_value(PlotType::Box).unwrap(), json!("box"));
        assert_eq!(to_value(PlotType::Candlestick).unwrap(), json!("candlestick"));
        assert_eq!(to_value(PlotType::Contour).unwrap(), json!("contour"));
        assert_eq!(to_value(PlotType::HeatMap).unwrap(), json!("heatmap"));
        assert_eq!(to_value(PlotType::Histogram).unwrap(), json!("histogram"));
        assert_eq!(to_value(PlotType::Histogram2dContour).unwrap(), json!("histogram2dcontour"));
        assert_eq!(to_value(PlotType::Ohlc).unwrap(), json!("ohlc"));
        assert_eq!(to_value(PlotType::Sankey).unwrap(), json!("sankey"));
        assert_eq!(to_value(PlotType::Surface).unwrap(), json!("surface"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_mode() {
        assert_eq!(to_value(Mode::Lines).unwrap(), json!("lines"));
        assert_eq!(to_value(Mode::Markers).unwrap(), json!("markers"));
        assert_eq!(to_value(Mode::Text).unwrap(), json!("text"));
        assert_eq!(to_value(Mode::LinesMarkers).unwrap(), json!("lines+markers"));
        assert_eq!(to_value(Mode::LinesText).unwrap(), json!("lines+text"));
        assert_eq!(to_value(Mode::MarkersText).unwrap(), json!("markers+text"));
        assert_eq!(to_value(Mode::LinesMarkersText).unwrap(), json!("lines+markers+text"));
        assert_eq!(to_value(Mode::None).unwrap(), json!("none"));
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
    #[rustfmt::skip]
    fn serialize_position() {
        assert_eq!(to_value(Position::TopLeft).unwrap(), json!("top left"));
        assert_eq!(to_value(Position::TopCenter).unwrap(), json!("top center"));
        assert_eq!(to_value(Position::TopRight).unwrap(), json!("top right"));
        assert_eq!(to_value(Position::MiddleLeft).unwrap(), json!("middle left"));
        assert_eq!(to_value(Position::MiddleCenter).unwrap(), json!("middle center"));
        assert_eq!(to_value(Position::MiddleRight).unwrap(), json!("middle right"));
        assert_eq!(to_value(Position::BottomLeft).unwrap(), json!("bottom left"));
        assert_eq!(to_value(Position::BottomCenter).unwrap(), json!("bottom center"));
        assert_eq!(to_value(Position::BottomRight).unwrap(), json!("bottom right"));
    }

    #[test]
    fn serialize_ticks() {
        assert_eq!(to_value(Ticks::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(Ticks::Inside).unwrap(), json!("inside"));
        assert_eq!(to_value(Ticks::None).unwrap(), json!(""));
    }

    #[test]
    fn serialize_show() {
        assert_eq!(to_value(Show::All).unwrap(), json!("all"));
        assert_eq!(to_value(Show::First).unwrap(), json!("first"));
        assert_eq!(to_value(Show::Last).unwrap(), json!("last"));
        assert_eq!(to_value(Show::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_default_color_bar() {
        let color_bar = ColorBar::new();
        let expected = json!({});

        assert_eq!(to_value(color_bar).unwrap(), expected);
    }

    #[test]
    fn serialize_color_bar() {
        let color_bar = ColorBar::new()
            .background_color("#123456")
            .border_color("#123456")
            .border_width(19)
            .dtick(1.0)
            .exponent_format(ExponentFormat::CapitalE)
            .len(99)
            .len_mode(ThicknessMode::Pixels)
            .n_ticks(500)
            .orientation(Orientation::Horizontal)
            .outline_color("#789456")
            .outline_width(7)
            .separate_thousands(true)
            .show_exponent(Show::All)
            .show_tick_labels(true)
            .show_tick_prefix(Show::First)
            .show_tick_suffix(Show::Last)
            .thickness(5)
            .thickness_mode(ThicknessMode::Fraction)
            .tick_angle(90.0)
            .tick_color("#777999")
            .tick_font(Font::new())
            .tick_format("tick_format")
            .tick_format_stops(vec![TickFormatStop::new()])
            .tick_len(1)
            .tick_mode(TickMode::Auto)
            .tick_prefix("prefix")
            .tick_suffix("suffix")
            .tick_text(vec!["txt"])
            .tick_vals(vec![1.0, 2.0])
            .tick_width(55)
            .tick0(0.0)
            .ticks(Ticks::Outside)
            .title(Title::new())
            .x(5.0)
            .x_anchor(Anchor::Bottom)
            .x_pad(2.2)
            .y(1.0)
            .y_anchor(Anchor::Auto)
            .y_pad(8.8);

        let expected = json!({
            "bgcolor": "#123456",
            "bordercolor": "#123456",
            "borderwidth": 19,
            "dtick": 1.0,
            "exponentformat": "E",
            "len": 99,
            "lenmode": "pixels",
            "nticks": 500,
            "orientation": "h",
            "outlinecolor": "#789456",
            "outlinewidth": 7,
            "separatethousands": true,
            "showexponent": "all",
            "showticklabels": true,
            "showtickprefix": "first",
            "showticksuffix": "last",
            "thickness": 5,
            "thicknessmode": "fraction",
            "tickangle": 90.0,
            "tickcolor": "#777999",
            "tickfont": {},
            "tickformat": "tick_format",
            "tickformatstops": [{"enabled": true}],
            "ticklen": 1,
            "tickmode": "auto",
            "tickprefix": "prefix",
            "ticksuffix": "suffix",
            "ticktext": ["txt"],
            "tickvals": [1.0, 2.0],
            "tickwidth": 55,
            "tick0": 0.0,
            "ticks": "outside",
            "title": {},
            "x": 5.0,
            "xanchor": "bottom",
            "xpad": 2.2,
            "y": 1.0,
            "yanchor": "auto",
            "ypad": 8.8
        });

        assert_eq!(to_value(color_bar).unwrap(), expected);
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_marker_symbol() {
        assert_eq!(to_value(MarkerSymbol::Circle).unwrap(), json!("circle"));
        assert_eq!(to_value(MarkerSymbol::CircleOpen).unwrap(), json!("circle-open"));
        assert_eq!(to_value(MarkerSymbol::CircleDot).unwrap(), json!("circle-dot"));
        assert_eq!(to_value(MarkerSymbol::CircleOpenDot).unwrap(), json!("circle-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Square).unwrap(), json!("square"));
        assert_eq!(to_value(MarkerSymbol::SquareOpen).unwrap(), json!("square-open"));
        assert_eq!(to_value(MarkerSymbol::SquareDot).unwrap(), json!("square-dot"));
        assert_eq!(to_value(MarkerSymbol::SquareOpenDot).unwrap(), json!("square-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Diamond).unwrap(), json!("diamond"));
        assert_eq!(to_value(MarkerSymbol::DiamondOpen).unwrap(), json!("diamond-open"));
        assert_eq!(to_value(MarkerSymbol::DiamondDot).unwrap(), json!("diamond-dot"));
        assert_eq!(to_value(MarkerSymbol::DiamondOpenDot).unwrap(), json!("diamond-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Cross).unwrap(), json!("cross"));
        assert_eq!(to_value(MarkerSymbol::CrossOpen).unwrap(), json!("cross-open"));
        assert_eq!(to_value(MarkerSymbol::CrossDot).unwrap(), json!("cross-dot"));
        assert_eq!(to_value(MarkerSymbol::CrossOpenDot).unwrap(), json!("cross-open-dot"));
        assert_eq!(to_value(MarkerSymbol::X).unwrap(), json!("x"));
        assert_eq!(to_value(MarkerSymbol::XOpen).unwrap(), json!("x-open"));
        assert_eq!(to_value(MarkerSymbol::XDot).unwrap(), json!("x-dot"));
        assert_eq!(to_value(MarkerSymbol::XOpenDot).unwrap(), json!("x-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleUp).unwrap(), json!("triangle-up"));
        assert_eq!(to_value(MarkerSymbol::TriangleUpOpen).unwrap(), json!("triangle-up-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleUpDot).unwrap(), json!("triangle-up-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleUpOpenDot).unwrap(), json!("triangle-up-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleDown).unwrap(), json!("triangle-down"));
        assert_eq!(to_value(MarkerSymbol::TriangleDownOpen).unwrap(), json!("triangle-down-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleDownDot).unwrap(), json!("triangle-down-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleDownOpenDot).unwrap(), json!("triangle-down-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleLeft).unwrap(), json!("triangle-left"));
        assert_eq!(to_value(MarkerSymbol::TriangleLeftOpen).unwrap(), json!("triangle-left-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleLeftDot).unwrap(), json!("triangle-left-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleLeftOpenDot).unwrap(), json!("triangle-left-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleRight).unwrap(), json!("triangle-right"));
        assert_eq!(to_value(MarkerSymbol::TriangleRightOpen).unwrap(), json!("triangle-right-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleRightDot).unwrap(), json!("triangle-right-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleRightOpenDot).unwrap(), json!("triangle-right-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleNE).unwrap(), json!("triangle-ne"));
        assert_eq!(to_value(MarkerSymbol::TriangleNEOpen).unwrap(), json!("triangle-ne-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleNEDot).unwrap(), json!("triangle-ne-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleNEOpenDot).unwrap(), json!("triangle-ne-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleSE).unwrap(), json!("triangle-se"));
        assert_eq!(to_value(MarkerSymbol::TriangleSEOpen).unwrap(), json!("triangle-se-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleSEDot).unwrap(), json!("triangle-se-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleSEOpenDot).unwrap(), json!("triangle-se-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleSW).unwrap(), json!("triangle-sw"));
        assert_eq!(to_value(MarkerSymbol::TriangleSWOpen).unwrap(), json!("triangle-sw-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleSWDot).unwrap(), json!("triangle-sw-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleSWOpenDot).unwrap(), json!("triangle-sw-open-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleNW).unwrap(), json!("triangle-nw"));
        assert_eq!(to_value(MarkerSymbol::TriangleNWOpen).unwrap(), json!("triangle-nw-open"));
        assert_eq!(to_value(MarkerSymbol::TriangleNWDot).unwrap(), json!("triangle-nw-dot"));
        assert_eq!(to_value(MarkerSymbol::TriangleNWOpenDot).unwrap(), json!("triangle-nw-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Pentagon).unwrap(), json!("pentagon"));
        assert_eq!(to_value(MarkerSymbol::PentagonOpen).unwrap(), json!("pentagon-open"));
        assert_eq!(to_value(MarkerSymbol::PentagonDot).unwrap(), json!("pentagon-dot"));
        assert_eq!(to_value(MarkerSymbol::PentagonOpenDot).unwrap(), json!("pentagon-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Hexagon).unwrap(), json!("hexagon"));
        assert_eq!(to_value(MarkerSymbol::HexagonOpen).unwrap(), json!("hexagon-open"));
        assert_eq!(to_value(MarkerSymbol::HexagonDot).unwrap(), json!("hexagon-dot"));
        assert_eq!(to_value(MarkerSymbol::HexagonOpenDot).unwrap(), json!("hexagon-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Hexagon2).unwrap(), json!("hexagon2"));
        assert_eq!(to_value(MarkerSymbol::Hexagon2Open).unwrap(), json!("hexagon2-open"));
        assert_eq!(to_value(MarkerSymbol::Hexagon2Dot).unwrap(), json!("hexagon2-dot"));
        assert_eq!(to_value(MarkerSymbol::Hexagon2OpenDot).unwrap(), json!("hexagon2-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Octagon).unwrap(), json!("octagon"));
        assert_eq!(to_value(MarkerSymbol::OctagonOpen).unwrap(), json!("octagon-open"));
        assert_eq!(to_value(MarkerSymbol::OctagonDot).unwrap(), json!("octagon-dot"));
        assert_eq!(to_value(MarkerSymbol::OctagonOpenDot).unwrap(), json!("octagon-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Star).unwrap(), json!("star"));
        assert_eq!(to_value(MarkerSymbol::StarOpen).unwrap(), json!("star-open"));
        assert_eq!(to_value(MarkerSymbol::StarDot).unwrap(), json!("star-dot"));
        assert_eq!(to_value(MarkerSymbol::StarOpenDot).unwrap(), json!("star-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Hexagram).unwrap(), json!("hexagram"));
        assert_eq!(to_value(MarkerSymbol::HexagramOpen).unwrap(), json!("hexagram-open"));
        assert_eq!(to_value(MarkerSymbol::HexagramDot).unwrap(), json!("hexagram-dot"));
        assert_eq!(to_value(MarkerSymbol::HexagramOpenDot).unwrap(), json!("hexagram-open-dot"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleUp).unwrap(), json!("star-triangle-up"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleUpOpen).unwrap(), json!("star-triangle-up-open"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleUpDot).unwrap(), json!("star-triangle-up-dot"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleUpOpenDot).unwrap(), json!("star-triangle-up-open-dot"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleDown).unwrap(), json!("star-triangle-down"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleDownOpen).unwrap(), json!("star-triangle-down-open"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleDownDot).unwrap(), json!("star-triangle-down-dot"));
        assert_eq!(to_value(MarkerSymbol::StarTriangleDownOpenDot).unwrap(), json!("star-triangle-down-open-dot"));
        assert_eq!(to_value(MarkerSymbol::StarSquare).unwrap(), json!("star-square"));
        assert_eq!(to_value(MarkerSymbol::StarSquareOpen).unwrap(), json!("star-square-open"));
        assert_eq!(to_value(MarkerSymbol::StarSquareDot).unwrap(), json!("star-square-dot"));
        assert_eq!(to_value(MarkerSymbol::StarSquareOpenDot).unwrap(), json!("star-square-open-dot"));
        assert_eq!(to_value(MarkerSymbol::StarDiamond).unwrap(), json!("star-diamond"));
        assert_eq!(to_value(MarkerSymbol::StarDiamondOpen).unwrap(), json!("star-diamond-open"));
        assert_eq!(to_value(MarkerSymbol::StarDiamondDot).unwrap(), json!("star-diamond-dot"));
        assert_eq!(to_value(MarkerSymbol::StarDiamondOpenDot).unwrap(), json!("star-diamond-open-dot"));
        assert_eq!(to_value(MarkerSymbol::DiamondTall).unwrap(), json!("diamond-tall"));
        assert_eq!(to_value(MarkerSymbol::DiamondTallOpen).unwrap(), json!("diamond-tall-open"));
        assert_eq!(to_value(MarkerSymbol::DiamondTallDot).unwrap(), json!("diamond-tall-dot"));
        assert_eq!(to_value(MarkerSymbol::DiamondTallOpenDot).unwrap(), json!("diamond-tall-open-dot"));
        assert_eq!(to_value(MarkerSymbol::DiamondWide).unwrap(), json!("diamond-wide"));
        assert_eq!(to_value(MarkerSymbol::DiamondWideOpen).unwrap(), json!("diamond-wide-open"));
        assert_eq!(to_value(MarkerSymbol::DiamondWideDot).unwrap(), json!("diamond-wide-dot"));
        assert_eq!(to_value(MarkerSymbol::DiamondWideOpenDot).unwrap(), json!("diamond-wide-open-dot"));
        assert_eq!(to_value(MarkerSymbol::Hourglass).unwrap(), json!("hourglass"));
        assert_eq!(to_value(MarkerSymbol::HourglassOpen).unwrap(), json!("hourglass-open"));
        assert_eq!(to_value(MarkerSymbol::BowTie).unwrap(), json!("bowtie"));
        assert_eq!(to_value(MarkerSymbol::BowTieOpen).unwrap(), json!("bowtie-open"));
        assert_eq!(to_value(MarkerSymbol::CircleCross).unwrap(), json!("circle-cross"));
        assert_eq!(to_value(MarkerSymbol::CircleCrossOpen).unwrap(), json!("circle-cross-open"));
        assert_eq!(to_value(MarkerSymbol::CircleX).unwrap(), json!("circle-x"));
        assert_eq!(to_value(MarkerSymbol::CircleXOpen).unwrap(), json!("circle-x-open"));
        assert_eq!(to_value(MarkerSymbol::SquareCross).unwrap(), json!("square-cross"));
        assert_eq!(to_value(MarkerSymbol::SquareCrossOpen).unwrap(), json!("square-cross-open"));
        assert_eq!(to_value(MarkerSymbol::SquareX).unwrap(), json!("square-x"));
        assert_eq!(to_value(MarkerSymbol::SquareXOpen).unwrap(), json!("square-x-open"));
        assert_eq!(to_value(MarkerSymbol::DiamondCross).unwrap(), json!("diamond-cross"));
        assert_eq!(to_value(MarkerSymbol::DiamondCrossOpen).unwrap(), json!("diamond-cross-open"));
        assert_eq!(to_value(MarkerSymbol::DiamondX).unwrap(), json!("diamond-x"));
        assert_eq!(to_value(MarkerSymbol::DiamondXOpen).unwrap(), json!("diamond-x-open"));
        assert_eq!(to_value(MarkerSymbol::CrossThin).unwrap(), json!("cross-thin"));
        assert_eq!(to_value(MarkerSymbol::CrossThinOpen).unwrap(), json!("cross-thin-open"));
        assert_eq!(to_value(MarkerSymbol::XThin).unwrap(), json!("x-thin"));
        assert_eq!(to_value(MarkerSymbol::XThinOpen).unwrap(), json!("x-thin-open"));
        assert_eq!(to_value(MarkerSymbol::Asterisk).unwrap(), json!("asterisk"));
        assert_eq!(to_value(MarkerSymbol::AsteriskOpen).unwrap(), json!("asterisk-open"));
        assert_eq!(to_value(MarkerSymbol::Hash).unwrap(), json!("hash"));
        assert_eq!(to_value(MarkerSymbol::HashOpen).unwrap(), json!("hash-open"));
        assert_eq!(to_value(MarkerSymbol::HashDot).unwrap(), json!("hash-dot"));
        assert_eq!(to_value(MarkerSymbol::HashOpenDot).unwrap(), json!("hash-open-dot"));
        assert_eq!(to_value(MarkerSymbol::YUp).unwrap(), json!("y-up"));
        assert_eq!(to_value(MarkerSymbol::YUpOpen).unwrap(), json!("y-up-open"));
        assert_eq!(to_value(MarkerSymbol::YDown).unwrap(), json!("y-down"));
        assert_eq!(to_value(MarkerSymbol::YDownOpen).unwrap(), json!("y-down-open"));
        assert_eq!(to_value(MarkerSymbol::YLeft).unwrap(), json!("y-left"));
        assert_eq!(to_value(MarkerSymbol::YLeftOpen).unwrap(), json!("y-left-open"));
        assert_eq!(to_value(MarkerSymbol::YRight).unwrap(), json!("y-right"));
        assert_eq!(to_value(MarkerSymbol::YRightOpen).unwrap(), json!("y-right-open"));
        assert_eq!(to_value(MarkerSymbol::LineEW).unwrap(), json!("line-ew"));
        assert_eq!(to_value(MarkerSymbol::LineEWOpen).unwrap(), json!("line-ew-open"));
        assert_eq!(to_value(MarkerSymbol::LineNS).unwrap(), json!("line-ns"));
        assert_eq!(to_value(MarkerSymbol::LineNSOpen).unwrap(), json!("line-ns-open"));
        assert_eq!(to_value(MarkerSymbol::LineNE).unwrap(), json!("line-ne"));
        assert_eq!(to_value(MarkerSymbol::LineNEOpen).unwrap(), json!("line-ne-open"));
        assert_eq!(to_value(MarkerSymbol::LineNW).unwrap(), json!("line-nw"));
        assert_eq!(to_value(MarkerSymbol::LineNWOpen).unwrap(), json!("line-nw-open"));
    }

    #[test]
    fn serialize_tick_mode() {
        assert_eq!(to_value(TickMode::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(TickMode::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(TickMode::Array).unwrap(), json!("array"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_dash_type() {
        assert_eq!(to_value(DashType::Solid).unwrap(), json!("solid"));
        assert_eq!(to_value(DashType::Dot).unwrap(), json!("dot"));
        assert_eq!(to_value(DashType::Dash).unwrap(), json!("dash"));
        assert_eq!(to_value(DashType::LongDash).unwrap(), json!("longdash"));
        assert_eq!(to_value(DashType::DashDot).unwrap(), json!("dashdot"));
        assert_eq!(to_value(DashType::LongDashDot).unwrap(), json!("longdashdot"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_color_scale_element() {
        assert_eq!(to_value(ColorScaleElement(0., "red".to_string())).unwrap(), json!([0.0, "red"]));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_color_scale_palette() {
        assert_eq!(to_value(ColorScalePalette::Greys).unwrap(), json!("Greys"));
        assert_eq!(to_value(ColorScalePalette::YlGnBu).unwrap(), json!("YlGnBu"));
        assert_eq!(to_value(ColorScalePalette::Greens).unwrap(), json!("Greens"));
        assert_eq!(to_value(ColorScalePalette::YlOrRd).unwrap(), json!("YlOrRd"));
        assert_eq!(to_value(ColorScalePalette::Bluered).unwrap(), json!("Bluered"));
        assert_eq!(to_value(ColorScalePalette::RdBu).unwrap(), json!("RdBu"));
        assert_eq!(to_value(ColorScalePalette::Reds).unwrap(), json!("Reds"));
        assert_eq!(to_value(ColorScalePalette::Blues).unwrap(), json!("Blues"));
        assert_eq!(to_value(ColorScalePalette::Picnic).unwrap(), json!("Picnic"));
        assert_eq!(to_value(ColorScalePalette::Rainbow).unwrap(), json!("Rainbow"));
        assert_eq!(to_value(ColorScalePalette::Portland).unwrap(), json!("Portland"));
        assert_eq!(to_value(ColorScalePalette::Jet).unwrap(), json!("Jet"));
        assert_eq!(to_value(ColorScalePalette::Hot).unwrap(), json!("Hot"));
        assert_eq!(to_value(ColorScalePalette::Blackbody).unwrap(), json!("Blackbody"));
        assert_eq!(to_value(ColorScalePalette::Earth).unwrap(), json!("Earth"));
        assert_eq!(to_value(ColorScalePalette::Electric).unwrap(), json!("Electric"));
        assert_eq!(to_value(ColorScalePalette::Viridis).unwrap(), json!("Viridis"));
        assert_eq!(to_value(ColorScalePalette::Cividis).unwrap(), json!("Cividis"));
    }

    #[test]
    fn serialize_color_scale() {
        assert_eq!(
            to_value(ColorScale::Palette(ColorScalePalette::Greys)).unwrap(),
            json!("Greys")
        );
        assert_eq!(
            to_value(ColorScale::Vector(vec![ColorScaleElement(
                0.0,
                "red".to_string()
            )]))
            .unwrap(),
            json!([[0.0, "red"]])
        );
    }

    #[test]
    fn serialize_line_shape() {
        assert_eq!(to_value(LineShape::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(LineShape::Spline).unwrap(), json!("spline"));
        assert_eq!(to_value(LineShape::Hv).unwrap(), json!("hv"));
        assert_eq!(to_value(LineShape::Vh).unwrap(), json!("vh"));
        assert_eq!(to_value(LineShape::Hvh).unwrap(), json!("hvh"));
        assert_eq!(to_value(LineShape::Vhv).unwrap(), json!("vhv"));
    }

    #[test]
    fn serialize_line() {
        let line = Line::new()
            .width(0.1)
            .shape(LineShape::Linear)
            .smoothing(1.0)
            .dash(DashType::Dash)
            .simplify(true)
            .color("#FFFFFF")
            .cauto(true)
            .cmin(0.0)
            .cmax(1.0)
            .cmid(0.5)
            .color_scale(ColorScale::Palette(ColorScalePalette::Greys))
            .auto_color_scale(true)
            .reverse_scale(true)
            .outlier_color("#111111")
            .outlier_width(1);

        let expected = json!({
            "width": 0.1,
            "shape": "linear",
            "smoothing": 1.0,
            "dash": "dash",
            "simplify": true,
            "color": "#FFFFFF",
            "cauto": true,
            "cmin": 0.0,
            "cmax": 1.0,
            "cmid": 0.5,
            "colorscale": "Greys",
            "autocolorscale": true,
            "reversescale": true,
            "outliercolor": "#111111",
            "outlierwidth": 1
        });

        assert_eq!(to_value(line).unwrap(), expected);
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_gradient_type() {
        assert_eq!(to_value(GradientType::Radial).unwrap(), json!("radial"));
        assert_eq!(to_value(GradientType::Horizontal).unwrap(), json!("horizontal"));
        assert_eq!(to_value(GradientType::Vertical).unwrap(), json!("vertical"));
        assert_eq!(to_value(GradientType::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_size_mode() {
        assert_eq!(to_value(SizeMode::Diameter).unwrap(), json!("diameter"));
        assert_eq!(to_value(SizeMode::Area).unwrap(), json!("area"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_thickness_mode() {
        assert_eq!(to_value(ThicknessMode::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(ThicknessMode::Pixels).unwrap(), json!("pixels"));
    }

    #[test]
    fn serialize_anchor() {
        assert_eq!(to_value(Anchor::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(Anchor::Left).unwrap(), json!("left"));
        assert_eq!(to_value(Anchor::Center).unwrap(), json!("center"));
        assert_eq!(to_value(Anchor::Right).unwrap(), json!("right"));
        assert_eq!(to_value(Anchor::Top).unwrap(), json!("top"));
        assert_eq!(to_value(Anchor::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(Anchor::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    fn serialize_text_anchor() {
        assert_eq!(to_value(TextAnchor::Start).unwrap(), json!("start"));
        assert_eq!(to_value(TextAnchor::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(TextAnchor::End).unwrap(), json!("end"));
    }

    #[test]
    fn serialize_exponent_format() {
        assert_eq!(to_value(ExponentFormat::None).unwrap(), json!("none"));
        assert_eq!(to_value(ExponentFormat::SmallE).unwrap(), json!("e"));
        assert_eq!(to_value(ExponentFormat::CapitalE).unwrap(), json!("E"));
        assert_eq!(to_value(ExponentFormat::Power).unwrap(), json!("power"));
        assert_eq!(to_value(ExponentFormat::SI).unwrap(), json!("SI"));
        assert_eq!(to_value(ExponentFormat::B).unwrap(), json!("B"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_gradient() {
        let gradient = Gradient::new(GradientType::Horizontal, "#FFFFFF");
        let expected = json!({"color": "#FFFFFF", "type": "horizontal"});
        assert_eq!(to_value(gradient).unwrap(), expected);

        let gradient = Gradient::new_array(GradientType::Horizontal, vec!["#FFFFFF"]);
        let expected = json!({"color": ["#FFFFFF"], "type": "horizontal"});
        assert_eq!(to_value(gradient).unwrap(), expected);
    }

    #[test]
    fn serialize_tick_format_stop_default() {
        let tick_format_stop = TickFormatStop::new();
        let expected = json!({"enabled": true});
        assert_eq!(to_value(tick_format_stop).unwrap(), expected);
    }

    #[test]
    fn serialize_tick_format_stop() {
        let tick_format_stop = TickFormatStop::new()
            .enabled(false)
            .dtick_range(vec![0.0, 1.0])
            .value("value")
            .name("name")
            .template_item_name("template_item_name");
        let expected = json!({
            "enabled": false,
            "dtickrange": [0.0, 1.0],
            "value": "value",
            "name": "name",
            "templateitemname": "template_item_name"
        });
        assert_eq!(to_value(tick_format_stop).unwrap(), expected);
    }

    #[test]
    fn serialize_pattern_shape() {
        assert_eq!(to_value(PatternShape::None).unwrap(), json!(""));
        assert_eq!(to_value(PatternShape::HorizontalLine).unwrap(), json!("-"));
        assert_eq!(to_value(PatternShape::VerticalLine).unwrap(), json!("|"));
        assert_eq!(
            to_value(PatternShape::RightDiagonalLine).unwrap(),
            json!("/")
        );
        assert_eq!(
            to_value(PatternShape::LeftDiagonalLine).unwrap(),
            json!("\\")
        );
        assert_eq!(to_value(PatternShape::Cross).unwrap(), json!("+"));
        assert_eq!(to_value(PatternShape::DiagonalCross).unwrap(), json!("x"));
        assert_eq!(to_value(PatternShape::Dot).unwrap(), json!("."));
    }

    #[test]
    fn serialize_pattern_fill_mode() {
        assert_eq!(
            to_value(PatternFillMode::Replace).unwrap(),
            json!("replace")
        );
        assert_eq!(
            to_value(PatternFillMode::Overlay).unwrap(),
            json!("overlay")
        );
    }

    #[test]
    fn serialize_pattern() {
        let pattern = Pattern::new()
            .shape_array(vec![
                PatternShape::HorizontalLine,
                PatternShape::VerticalLine,
            ])
            .fill_mode(PatternFillMode::Overlay)
            .background_color_array(vec![NamedColor::Black, NamedColor::Blue])
            .foreground_color_array(vec![NamedColor::Red, NamedColor::Green])
            .foreground_opacity(0.9)
            .size_array(vec![10.0, 20.0])
            .solidity_array(vec![0.1, 0.2]);

        let expected = json!({
            "shape": ["-", "|"],
            "fillmode": "overlay",
            "bgcolor": ["black", "blue"],
            "fgcolor": ["red", "green"],
            "fgopacity": 0.9,
            "size": [10.0, 20.0],
            "solidity": [0.1, 0.2]
        });

        assert_eq!(to_value(pattern).unwrap(), expected);
    }

    #[test]
    fn serialize_marker() {
        let marker = Marker::new()
            .symbol(MarkerSymbol::Circle)
            .opacity(0.1)
            .size(1)
            .max_displayed(5)
            .size_ref(5)
            .size_min(1)
            .size_mode(SizeMode::Area)
            .line(Line::new())
            .gradient(Gradient::new(GradientType::Radial, "#FFFFFF"))
            .color(NamedColor::Blue)
            .colors(vec![NamedColor::Black, NamedColor::Blue])
            .color_array(vec![NamedColor::Black, NamedColor::Blue])
            .cauto(true)
            .cmin(0.0)
            .cmax(1.0)
            .cmid(0.5)
            .color_scale(ColorScale::Palette(ColorScalePalette::Earth))
            .auto_color_scale(true)
            .reverse_scale(true)
            .show_scale(true)
            .color_bar(ColorBar::new())
            .outlier_color("#FFFFFF")
            .pattern(
                Pattern::new()
                    .shape(PatternShape::Cross)
                    .foreground_color(NamedColor::Red)
                    .size(10.0),
            );

        let expected = json!({
            "symbol": "circle",
            "opacity": 0.1,
            "size": 1,
            "maxdisplayed": 5,
            "sizeref": 5,
            "sizemin": 1,
            "sizemode": "area",
            "line": {},
            "gradient": {"type": "radial", "color": "#FFFFFF"},
            "color": ["black", "blue"],
            "colors": ["black", "blue"],
            "colorbar": {},
            "cauto": true,
            "cmin": 0.0,
            "cmax": 1.0,
            "cmid": 0.5,
            "colorscale": "Earth",
            "autocolorscale": true,
            "reversescale": true,
            "showscale": true,
            "outliercolor": "#FFFFFF",
            "pattern": {
                "shape": "+",
                "fgcolor": "red",
                "size": 10.0
            }
        });

        assert_eq!(to_value(marker).unwrap(), expected);
    }

    #[test]
    fn serialize_font() {
        let font = Font::new().family("family").size(100).color("#FFFFFF");
        let expected = json!({
            "family": "family",
            "size": 100,
            "color": "#FFFFFF"
        });

        assert_eq!(to_value(font).unwrap(), expected);
    }

    #[test]
    fn serialize_side() {
        assert_eq!(to_value(Side::Right).unwrap(), json!("right"));
        assert_eq!(to_value(Side::Top).unwrap(), json!("top"));
        assert_eq!(to_value(Side::Bottom).unwrap(), json!("bottom"));
        assert_eq!(to_value(Side::Left).unwrap(), json!("left"));
        assert_eq!(to_value(Side::TopLeft).unwrap(), json!("top left"));
    }

    #[test]
    fn serialize_reference() {
        assert_eq!(to_value(Reference::Container).unwrap(), json!("container"));
        assert_eq!(to_value(Reference::Paper).unwrap(), json!("paper"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_legend_group_title() {
        assert_eq!(to_value(LegendGroupTitle::new()).unwrap(), json!({}));
        assert_eq!(to_value(LegendGroupTitle::with_text("title_str").font(Font::default())).unwrap(), json!({"font": {}, "text": "title_str"}));
        assert_eq!(to_value(LegendGroupTitle::from(String::from("title_string"))).unwrap(), json!({"text" : "title_string"}));
        assert_eq!(to_value(LegendGroupTitle::from(&String::from("title_string"))).unwrap(), json!({"text" : "title_string"}));
    }

    #[test]
    fn serialize_pad() {
        let pad = Pad::new(1, 2, 3);
        let expected = json!({
            "t": 1,
            "b": 2,
            "l": 3
        });

        assert_eq!(to_value(pad).unwrap(), expected);
    }

    #[test]
    fn serialize_title() {
        let title = Title::with_text("title")
            .font(Font::new())
            .side(Side::Top)
            .x_ref(Reference::Paper)
            .y_ref(Reference::Paper)
            .x(0.5)
            .y(0.5)
            .x_anchor(Anchor::Auto)
            .y_anchor(Anchor::Auto)
            .pad(Pad::new(0, 0, 0));
        let expected = json!({
            "text": "title",
            "font": {},
            "side": "top",
            "xref": "paper",
            "yref": "paper",
            "x": 0.5,
            "y": 0.5,
            "xanchor": "auto",
            "yanchor": "auto",
            "pad": {"t": 0, "b": 0, "l": 0}
        });

        assert_eq!(to_value(title).unwrap(), expected);
    }

    #[test]
    fn serialize_title_from_str() {
        let title = Title::from("from");
        let expected = json!({"text": "from"});

        assert_eq!(to_value(title).unwrap(), expected);

        let title: Title = "into".into();
        let expected = json!({"text": "into"});

        assert_eq!(to_value(title).unwrap(), expected);
    }

    #[test]
    fn serialize_label() {
        let label = Label::new()
            .background_color("#FFFFFF")
            .border_color("#000000")
            .font(Font::new())
            .align("something")
            .name_length_array(vec![5, 10])
            .name_length(6);
        let expected = json!({
            "bgcolor": "#FFFFFF",
            "bordercolor": "#000000",
            "font": {},
            "align": "something",
            "namelength": 6,
        });

        assert_eq!(to_value(label).unwrap(), expected);
    }

    #[test]
    fn serialize_error_type() {
        assert_eq!(to_value(ErrorType::Percent).unwrap(), json!("percent"));
        assert_eq!(to_value(ErrorType::Constant).unwrap(), json!("constant"));
        assert_eq!(to_value(ErrorType::SquareRoot).unwrap(), json!("sqrt"));
        assert_eq!(to_value(ErrorType::Data).unwrap(), json!("data"));
    }

    #[test]
    fn serialize_error_type_default() {
        assert_eq!(to_value(ErrorType::default()).unwrap(), json!("percent"));
    }

    #[test]
    fn serialize_error_data() {
        let error_data = ErrorData::new(ErrorType::Constant)
            .array(vec![0.1, 0.2])
            .visible(true)
            .symmetric(false)
            .array_minus(vec![0.05, 0.1])
            .value(5.0)
            .value_minus(2.5)
            .trace_ref(1)
            .trace_ref_minus(1)
            .copy_ystyle(true)
            .color("#AAAAAA")
            .thickness(2.0)
            .width(5);
        let expected = json!({
            "type": "constant",
            "array": [0.1, 0.2],
            "visible": true,
            "symmetric": false,
            "arrayminus": [0.05, 0.1],
            "value": 5.0,
            "valueminus": 2.5,
            "traceref": 1,
            "tracerefminus": 1,
            "copy_ystyle": true,
            "color": "#AAAAAA",
            "thickness": 2.0,
            "width": 5,
        });

        assert_eq!(to_value(error_data).unwrap(), expected)
    }

    #[test]
    fn serialize_visible() {
        assert_eq!(to_value(Visible::True).unwrap(), json!(true));
        assert_eq!(to_value(Visible::False).unwrap(), json!(false));
        assert_eq!(to_value(Visible::LegendOnly).unwrap(), json!("legendonly"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_hover_on() {
        assert_eq!(to_value(HoverOn::Points).unwrap(), json!("points"));
        assert_eq!(to_value(HoverOn::Fills).unwrap(), json!("fills"));
        assert_eq!(to_value(HoverOn::PointsAndFills).unwrap(), json!("points+fills"));

    }

    #[test]
    fn serialize_slider_anchor() {
        assert_eq!(to_value(Anchor::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(Anchor::Left).unwrap(), json!("left"));
        assert_eq!(to_value(Anchor::Center).unwrap(), json!("center"));
        assert_eq!(to_value(Anchor::Right).unwrap(), json!("right"));
        assert_eq!(to_value(Anchor::Top).unwrap(), json!("top"));
        assert_eq!(to_value(Anchor::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(Anchor::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    #[allow(clippy::needless_borrows_for_generic_args)]
    fn title_method_can_take_string() {
        ColorBar::new().title("Title");
        ColorBar::new().title(String::from("Title"));
        ColorBar::new().title(&String::from("Title"));
        ColorBar::new().title(Title::with_text("Title"));
    }
}
