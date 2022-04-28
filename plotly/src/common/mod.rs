use serde::Serialize;

pub mod color;

use crate::common::color::ColorWrapper;
use crate::private;
use crate::private::{to_num_or_string_wrapper, NumOrString, NumOrStringWrapper};
use color::Color;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Direction {
    Increasing { line: Line },
    Decreasing { line: Line },
}

#[derive(Serialize, Clone, Debug)]
pub enum Visible {
    #[serde(rename = "x")]
    True,
    #[serde(rename = "x")]
    False,
    #[serde(rename = "x")]
    LegendOnly,
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
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GroupNorm {
    #[serde(rename = "")]
    Default,
    Fraction,
    Percent,
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
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlotType {
    Scatter,
    ScatterGL,
    Scatter3D,
    ScatterPolar,
    ScatterPolarGL,
    Bar,
    Box,
    Candlestick,
    Contour,
    HeatMap,
    Histogram,
    Histogram2dContour,
    Ohlc,
    Surface,
}

impl Default for PlotType {
    fn default() -> Self {
        PlotType::Scatter
    }
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
pub struct ColorScaleElement(f64, String);

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

#[derive(Serialize, Clone, Debug, Default)]
pub struct Line {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<LineShape>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smoothing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dash: Option<DashType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    simplify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "outliercolor")]
    outlier_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlierwidth")]
    outlier_width: Option<usize>,
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn shape(mut self, shape: LineShape) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn smoothing(mut self, smoothing: f64) -> Self {
        self.smoothing = Some(smoothing);
        self
    }

    pub fn dash(mut self, dash: DashType) -> Self {
        self.dash = Some(dash);
        self
    }

    pub fn simplify(mut self, simplify: bool) -> Self {
        self.simplify = Some(simplify);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(color.to_color());
        self
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

    pub fn outlier_color<C: Color>(mut self, outlier_color: C) -> Self {
        self.outlier_color = Some(outlier_color.to_color());
        self
    }

    pub fn outlier_width(mut self, outlier_width: usize) -> Self {
        self.outlier_width = Some(outlier_width);
        self
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
    color: Dim<ColorWrapper>,
}

impl Gradient {
    pub fn new<C: Color + Serialize>(gradient_type: GradientType, color: Dim<C>) -> Gradient {
        let color = match color {
            Dim::Scalar(c) => Dim::Scalar(c.to_color()),
            Dim::Vector(c) => Dim::Vector(private::to_color_array(c)),
        };
        Gradient {
            r#type: gradient_type,
            color,
        }
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct TickFormatStop {
    enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dtickrange")]
    dtick_range: Option<Vec<NumOrStringWrapper>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl TickFormatStop {
    pub fn new() -> TickFormatStop {
        TickFormatStop {
            enabled: true,
            ..Default::default()
        }
    }

    pub fn enabled(mut self, enabled: bool) -> TickFormatStop {
        self.enabled = enabled;
        self
    }

    pub fn dtick_range<C: NumOrString>(mut self, range: Vec<C>) -> TickFormatStop {
        let wrapped = to_num_or_string_wrapper(range);
        self.dtick_range = Some(wrapped);
        self
    }

    pub fn value(mut self, value: &str) -> TickFormatStop {
        self.value = Some(value.to_owned());
        self
    }

    pub fn name(mut self, name: &str) -> TickFormatStop {
        self.name = Some(name.to_owned());
        self
    }

    pub fn template_item_name(mut self, name: &str) -> TickFormatStop {
        self.template_item_name = Some(name.to_owned());
        self
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ColorBar {
    #[serde(skip_serializing_if = "Option::is_none", rename = "thicknessmode")]
    thickness_mode: Option<ThicknessMode>,
    thickness: usize,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lenmode")]
    len_mode: Option<ThicknessMode>,
    len: usize,
    x: f64,
    #[serde(rename = "xanchor")]
    x_anchor: Anchor,
    #[serde(rename = "xpad")]
    x_pad: f64,
    y: f64,
    #[serde(rename = "yanchor")]
    y_anchor: Anchor,
    #[serde(rename = "ypad")]
    y_pad: f64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlinecolor")]
    outline_color: Option<ColorWrapper>,
    #[serde(rename = "outlinewidth")]
    outline_width: usize,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<ColorWrapper>,
    #[serde(rename = "borderwidth")]
    border_width: usize,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickmode")]
    tick_mode: Option<TickMode>,
    #[serde(rename = "nticks")]
    n_ticks: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    tick0: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dtick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickvals")]
    tick_vals: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ticktext")]
    tick_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ticks: Option<String>,
    #[serde(rename = "ticklen")]
    tick_len: usize,
    #[serde(rename = "tickwidth")]
    tick_width: usize,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickcolor")]
    tick_color: Option<ColorWrapper>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickformatstops")]
    tick_format_stops: Option<Vec<TickFormatStop>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickprefix")]
    tick_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showtickprefix")]
    show_tick_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ticksuffix")]
    tick_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showticksuffix")]
    show_tick_suffix: Option<String>,
    separate_thousands: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "exponentformat")]
    exponent_format: Option<ExponentFormat>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showexponent")]
    show_exponent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
}

impl Default for ColorBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorBar {
    pub fn new() -> ColorBar {
        ColorBar {
            thickness: 30,
            len: 1,
            x: 1.02,
            x_anchor: Anchor::Left,
            x_pad: 10.0,
            y: 0.5,
            y_anchor: Anchor::Middle,
            y_pad: 10.0,
            outline_width: 1,
            border_width: 0,
            n_ticks: 0,
            tick_len: 5,
            tick_width: 1,
            show_tick_labels: true,
            separate_thousands: true,
            ..Default::default()
        }
    }

    pub fn thickness_mode(mut self, thickness_mode: ThicknessMode) -> ColorBar {
        self.thickness_mode = Some(thickness_mode);
        self
    }

    pub fn thickness(mut self, thickness: usize) -> ColorBar {
        self.thickness = thickness;
        self
    }

    pub fn len_mode(mut self, len_mode: ThicknessMode) -> ColorBar {
        self.len_mode = Some(len_mode);
        self
    }

    pub fn len(mut self, len: usize) -> ColorBar {
        self.len = len;
        self
    }

    pub fn x(mut self, x: f64) -> ColorBar {
        self.x = x;
        self
    }

    pub fn x_anchor(mut self, x_anchor: Anchor) -> ColorBar {
        self.x_anchor = x_anchor;
        self
    }

    pub fn x_pad(mut self, x_pad: f64) -> ColorBar {
        self.x_pad = x_pad;
        self
    }

    pub fn y(mut self, y: f64) -> ColorBar {
        self.y = y;
        self
    }

    pub fn y_anchor(mut self, y_anchor: Anchor) -> ColorBar {
        self.y_anchor = y_anchor;
        self
    }

    pub fn y_pad(mut self, y_pad: f64) -> ColorBar {
        self.y_pad = y_pad;
        self
    }

    pub fn outline_color<C: Color>(mut self, outline_color: C) -> ColorBar {
        self.outline_color = Some(outline_color.to_color());
        self
    }

    pub fn outline_width(mut self, outline_width: usize) -> ColorBar {
        self.outline_width = outline_width;
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> ColorBar {
        self.border_color = Some(border_color.to_color());
        self
    }

    pub fn border_width(mut self, border_width: usize) -> ColorBar {
        self.border_width = border_width;
        self
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> ColorBar {
        self.background_color = Some(background_color.to_color());
        self
    }

    pub fn tick_mode(mut self, tick_mode: TickMode) -> ColorBar {
        self.tick_mode = Some(tick_mode);
        self
    }

    pub fn n_ticks(mut self, n_ticks: usize) -> ColorBar {
        self.n_ticks = n_ticks;
        self
    }

    pub fn tick0(mut self, tick0: f64) -> ColorBar {
        self.tick0 = Some(tick0);
        self
    }

    pub fn dtick(mut self, dtick: f64) -> ColorBar {
        self.dtick = Some(dtick);
        self
    }

    pub fn tick_vals(mut self, tick_vals: Vec<f64>) -> ColorBar {
        self.tick_vals = Some(tick_vals);
        self
    }

    pub fn tick_text(mut self, tick_text: Vec<String>) -> ColorBar {
        self.tick_text = Some(tick_text);
        self
    }

    pub fn ticks(mut self, ticks: &str) -> ColorBar {
        self.ticks = Some(ticks.to_owned());
        self
    }

    pub fn tick_len(mut self, tick_len: usize) -> ColorBar {
        self.tick_len = tick_len;
        self
    }

    pub fn tick_width(mut self, tick_width: usize) -> ColorBar {
        self.tick_width = tick_width;
        self
    }

    pub fn tick_color<C: Color>(mut self, tick_color: C) -> ColorBar {
        self.tick_color = Some(tick_color.to_color());
        self
    }

    pub fn show_tick_labels(mut self, show_tick_labels: bool) -> ColorBar {
        self.show_tick_labels = show_tick_labels;
        self
    }

    pub fn tick_font(mut self, tick_font: Font) -> ColorBar {
        self.tick_font = Some(tick_font);
        self
    }

    pub fn tick_angle(mut self, tick_angle: f64) -> ColorBar {
        self.tick_angle = Some(tick_angle);
        self
    }

    pub fn tick_format(mut self, tick_format: &str) -> ColorBar {
        self.tick_format = Some(tick_format.to_owned());
        self
    }

    pub fn tick_format_stops(mut self, tick_format_stops: Vec<TickFormatStop>) -> ColorBar {
        self.tick_format_stops = Some(tick_format_stops);
        self
    }

    pub fn tick_prefix(mut self, tick_prefix: &str) -> ColorBar {
        self.tick_prefix = Some(tick_prefix.to_owned());
        self
    }

    pub fn show_tick_prefix(mut self, show_tick_prefix: &str) -> ColorBar {
        self.show_tick_prefix = Some(show_tick_prefix.to_owned());
        self
    }

    pub fn tick_suffix(mut self, tick_suffix: &str) -> ColorBar {
        self.tick_suffix = Some(tick_suffix.to_owned());
        self
    }

    pub fn show_tick_suffix(mut self, show_tick_suffix: &str) -> ColorBar {
        self.show_tick_suffix = Some(show_tick_suffix.to_owned());
        self
    }

    pub fn separate_thousands(mut self, separate_thousands: bool) -> ColorBar {
        self.separate_thousands = separate_thousands;
        self
    }

    pub fn exponent_format(mut self, exponent_format: ExponentFormat) -> ColorBar {
        self.exponent_format = Some(exponent_format);
        self
    }

    pub fn show_exponent(mut self, show_exponent: &str) -> ColorBar {
        self.show_exponent = Some(show_exponent.to_owned());
        self
    }

    pub fn title(mut self, title: Title) -> ColorBar {
        self.title = Some(title);
        self
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Marker {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<MarkerSymbol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<Dim<usize>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxdisplayed")]
    max_displayed: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sizeref")]
    size_ref: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sizemin")]
    size_min: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sizemin")]
    size_mode: Option<SizeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gradient: Option<Gradient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Dim<ColorWrapper>>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "outliercolor")]
    outlier_color: Option<ColorWrapper>,
}

impl Marker {
    pub fn new() -> Marker {
        Default::default()
    }

    pub fn symbol(mut self, symbol: MarkerSymbol) -> Marker {
        self.symbol = Some(symbol);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Marker {
        self.opacity = Some(opacity);
        self
    }

    pub fn size(mut self, size: usize) -> Marker {
        self.size = Some(Dim::Scalar(size));
        self
    }

    pub fn size_array(mut self, size: Vec<usize>) -> Marker {
        self.size = Some(Dim::Vector(size));
        self
    }

    pub fn max_displayed(mut self, size: usize) -> Marker {
        self.max_displayed = Some(size);
        self
    }

    pub fn size_ref(mut self, size: usize) -> Marker {
        self.size_ref = Some(size);
        self
    }

    pub fn size_min(mut self, size: usize) -> Marker {
        self.size_min = Some(size);
        self
    }

    pub fn size_mode(mut self, mode: SizeMode) -> Marker {
        self.size_mode = Some(mode);
        self
    }

    pub fn line(mut self, line: Line) -> Marker {
        self.line = Some(line);
        self
    }

    pub fn gradient(mut self, gradient: Gradient) -> Marker {
        self.gradient = Some(gradient);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Marker {
        self.color = Some(Dim::Scalar(color.to_color()));
        self
    }

    pub fn color_array<C: Color>(mut self, color: Vec<C>) -> Marker {
        let color = private::to_color_array(color);
        self.color = Some(Dim::Vector(color));
        self
    }

    pub fn cauto(mut self, cauto: bool) -> Marker {
        self.cauto = Some(cauto);
        self
    }

    pub fn cmin(mut self, cmin: f64) -> Marker {
        self.cmin = Some(cmin);
        self
    }

    pub fn cmax(mut self, cmax: f64) -> Marker {
        self.cmax = Some(cmax);
        self
    }

    pub fn cmid(mut self, cmid: f64) -> Marker {
        self.cmid = Some(cmid);
        self
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Marker {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Marker {
        self.auto_color_scale = Some(auto_color_scale);
        self
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Marker {
        self.reverse_scale = Some(reverse_scale);
        self
    }

    pub fn show_scale(mut self, show_scale: bool) -> Marker {
        self.show_scale = Some(show_scale);
        self
    }

    pub fn color_bar(mut self, colorbar: ColorBar) -> Marker {
        self.color_bar = Some(colorbar);
        self
    }

    pub fn outlier_color<C: Color>(mut self, outlier_color: C) -> Marker {
        self.outlier_color = Some(outlier_color.to_color());
        self
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Font {
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
}

impl Font {
    pub fn new() -> Font {
        Default::default()
    }

    pub fn family(mut self, family: &str) -> Font {
        self.family = Some(family.to_owned());
        self
    }

    pub fn size(mut self, size: usize) -> Font {
        self.size = Some(size);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Font {
        self.color = Some(color.to_color());
        self
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum Side {
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "top left")]
    TopLeft,
}

#[derive(Serialize, Clone, Debug)]
pub enum Reference {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "paper")]
    Paper,
}

#[derive(Serialize, Clone, Debug)]
pub struct Pad {
    t: usize,
    b: usize,
    l: usize,
}

impl Pad {
    pub fn new(t: usize, b: usize, l: usize) -> Pad {
        Pad { t, b, l }
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Title {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xref")]
    x_ref: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yref")]
    y_ref: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yanchor")]
    y_anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pad: Option<Pad>,
}

impl From<&str> for Title {
    fn from(title: &str) -> Self {
        Title::new(title)
    }
}

impl Title {
    pub fn new(text: &str) -> Title {
        Title {
            text: text.to_owned(),
            ..Default::default()
        }
    }

    pub fn font(mut self, font: Font) -> Title {
        self.font = Some(font);
        self
    }

    pub fn side(mut self, side: Side) -> Title {
        self.side = Some(side);
        self
    }

    pub fn x_ref(mut self, xref: Reference) -> Title {
        self.x_ref = Some(xref);
        self
    }

    pub fn y_ref(mut self, yref: Reference) -> Title {
        self.y_ref = Some(yref);
        self
    }

    pub fn x(mut self, x: f64) -> Title {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: f64) -> Title {
        self.y = Some(y);
        self
    }

    pub fn x_anchor(mut self, anchor: Anchor) -> Title {
        self.x_anchor = Some(anchor);
        self
    }

    pub fn y_anchor(mut self, anchor: Anchor) -> Title {
        self.y_anchor = Some(anchor);
        self
    }

    pub fn pad(mut self, pad: Pad) -> Title {
        self.pad = Some(pad);
        self
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "namelength")]
    name_length: Option<Dim<i32>>,
}

impl Label {
    pub fn new() -> Label {
        Default::default()
    }

    pub fn background_color<C: Color>(mut self, background_color: C) -> Label {
        self.background_color = Some(background_color.to_color());
        self
    }

    pub fn border_color<C: Color>(mut self, border_color: C) -> Label {
        self.border_color = Some(border_color.to_color());
        self
    }

    pub fn font(mut self, font: Font) -> Label {
        self.font = Some(font);
        self
    }

    pub fn align(mut self, align: &str) -> Label {
        self.align = Some(align.to_owned());
        self
    }

    pub fn name_length(mut self, name_length: i32) -> Label {
        self.name_length = Some(Dim::Scalar(name_length));
        self
    }

    pub fn name_length_array(mut self, name_length: Vec<i32>) -> Label {
        self.name_length = Some(Dim::Vector(name_length));
        self
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum ErrorType {
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "sqrt")]
    SquareRoot,
    #[serde(rename = "data")]
    Data,
}

impl Default for ErrorType {
    fn default() -> Self {
        ErrorType::Percent
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ErrorData {
    r#type: ErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    array: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symmetric: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "arrayminus")]
    array_minus: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "valueminus")]
    value_minus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "traceref")]
    trace_ref: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tracerefminus")]
    trace_ref_minus: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_ystyle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<ColorWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thickness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
}

impl ErrorData {
    pub fn new(error_type: ErrorType) -> ErrorData {
        ErrorData {
            r#type: error_type,
            ..Default::default()
        }
    }

    pub fn array(mut self, array: Vec<f64>) -> ErrorData {
        self.array = Some(array);
        self
    }

    pub fn visible(mut self, visible: bool) -> ErrorData {
        self.visible = Some(visible);
        self
    }

    pub fn symmetric(mut self, symmetric: bool) -> ErrorData {
        self.symmetric = Some(symmetric);
        self
    }

    pub fn array_minus(mut self, array_minus: Vec<f64>) -> ErrorData {
        self.array_minus = Some(array_minus);
        self
    }

    pub fn value(mut self, value: f64) -> ErrorData {
        self.value = Some(value);
        self
    }

    pub fn value_minus(mut self, value_minus: f64) -> ErrorData {
        self.value_minus = Some(value_minus);
        self
    }

    pub fn trace_ref(mut self, trace_ref: usize) -> ErrorData {
        self.trace_ref = Some(trace_ref);
        self
    }

    pub fn trace_ref_minus(mut self, trace_ref_minus: usize) -> ErrorData {
        self.trace_ref_minus = Some(trace_ref_minus);
        self
    }

    pub fn copy_ystyle(mut self, copy_ystyle: bool) -> ErrorData {
        self.copy_ystyle = Some(copy_ystyle);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> ErrorData {
        self.color = Some(color.to_color());
        self
    }

    pub fn thickness(mut self, thickness: f64) -> ErrorData {
        self.thickness = Some(thickness);
        self
    }

    pub fn width(mut self, width: usize) -> ErrorData {
        self.width = Some(width);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_direction() {
        // TODO: I think `Direction` would be better as a struct, with `fillcolor` and `line` attributes
        let inc = Direction::Increasing { line: Line::new() };
        let expected = json!({"line": {}});
        assert_eq!(to_value(inc).unwrap(), expected);

        let dec = Direction::Decreasing { line: Line::new() };
        let expected = json!({"line": {}});
        assert_eq!(to_value(dec).unwrap(), expected);
    }

    #[test]
    fn test_serialize_hover_info() {
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
    fn test_serialize_text_position() {
        assert_eq!(to_value(TextPosition::Inside).unwrap(), json!("inside"));
        assert_eq!(to_value(TextPosition::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(TextPosition::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(TextPosition::None).unwrap(), json!("none"));
    }

    #[test]
    fn test_serialize_constrain_text() {
        assert_eq!(to_value(ConstrainText::Inside).unwrap(), json!("inside"));
        assert_eq!(to_value(ConstrainText::Outside).unwrap(), json!("outside"));
        assert_eq!(to_value(ConstrainText::Both).unwrap(), json!("both"));
        assert_eq!(to_value(ConstrainText::None).unwrap(), json!("none"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_orientation() {
        assert_eq!(to_value(Orientation::Vertical).unwrap(), json!("vertical"));
        assert_eq!(to_value(Orientation::Horizontal).unwrap(), json!("horizontal"));
    }

    #[test]
    fn test_serialize_group_norm() {
        assert_eq!(to_value(GroupNorm::Default).unwrap(), json!(""));
        assert_eq!(to_value(GroupNorm::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(GroupNorm::Percent).unwrap(), json!("percent"));
    }

    #[test]
    fn test_serialize_fill() {
        assert_eq!(to_value(Fill::ToZeroY).unwrap(), json!("tozeroy"));
        assert_eq!(to_value(Fill::ToZeroX).unwrap(), json!("tozerox"));
        assert_eq!(to_value(Fill::ToNextY).unwrap(), json!("tonexty"));
        assert_eq!(to_value(Fill::ToNextX).unwrap(), json!("tonextx"));
        assert_eq!(to_value(Fill::ToSelf).unwrap(), json!("toself"));
        assert_eq!(to_value(Fill::ToNext).unwrap(), json!("tonext"));
        assert_eq!(to_value(Fill::None).unwrap(), json!("none"));
    }

    #[test]
    fn test_serialize_calendar() {
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
    fn test_serialize_dim() {
        assert_eq!(to_value(Dim::Scalar(0)).unwrap(), json!(0));
        assert_eq!(to_value(Dim::Vector(vec![0])).unwrap(), json!([0]));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_plot_type() {
        assert_eq!(to_value(PlotType::Scatter).unwrap(), json!("scatter"));
        assert_eq!(to_value(PlotType::ScatterGL).unwrap(), json!("scattergl"));
        assert_eq!(to_value(PlotType::Scatter3D).unwrap(), json!("scatter3d"));
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
        assert_eq!(to_value(PlotType::Surface).unwrap(), json!("surface"));
    }

    #[test]
    fn test_default_plot_type() {
        assert_eq!(PlotType::default(), PlotType::Scatter);
    }

    #[test]
    fn test_serialize_mode() {
        assert_eq!(to_value(Mode::Lines).unwrap(), json!("lines"));
        assert_eq!(to_value(Mode::Markers).unwrap(), json!("markers"));
        assert_eq!(to_value(Mode::Text).unwrap(), json!("text"));
        assert_eq!(
            to_value(Mode::LinesMarkers).unwrap(),
            json!("lines+markers")
        );
        assert_eq!(to_value(Mode::LinesText).unwrap(), json!("lines+text"));
        assert_eq!(to_value(Mode::MarkersText).unwrap(), json!("markers+text"));
        assert_eq!(
            to_value(Mode::LinesMarkersText).unwrap(),
            json!("lines+markers+text")
        );
        assert_eq!(to_value(Mode::None).unwrap(), json!("none"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_position() {
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
    #[rustfmt::skip]
    fn test_serialize_marker_symbol() {
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
    fn test_serialize_tick_mode() {
        assert_eq!(to_value(TickMode::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(TickMode::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(TickMode::Array).unwrap(), json!("array"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_dash_type() {
        assert_eq!(to_value(DashType::Solid).unwrap(), json!("solid"));
        assert_eq!(to_value(DashType::Dot).unwrap(), json!("dot"));
        assert_eq!(to_value(DashType::Dash).unwrap(), json!("dash"));
        assert_eq!(to_value(DashType::LongDash).unwrap(), json!("longdash"));
        assert_eq!(to_value(DashType::DashDot).unwrap(), json!("dashdot"));
        assert_eq!(to_value(DashType::LongDashDot).unwrap(), json!("longdashdot"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_color_scale_element() {
        assert_eq!(to_value(ColorScaleElement(0., "red".to_string())).unwrap(), json!([0.0, "red"]));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_color_scale_palette() {
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
    fn test_serialize_color_scale() {
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
    fn test_serialize_line_shape() {
        assert_eq!(to_value(LineShape::Linear).unwrap(), json!("linear"));
        assert_eq!(to_value(LineShape::Spline).unwrap(), json!("spline"));
        assert_eq!(to_value(LineShape::Hv).unwrap(), json!("hv"));
        assert_eq!(to_value(LineShape::Vh).unwrap(), json!("vh"));
        assert_eq!(to_value(LineShape::Hvh).unwrap(), json!("hvh"));
        assert_eq!(to_value(LineShape::Vhv).unwrap(), json!("vhv"));
    }

    #[test]
    fn test_serialize_line() {
        let line = Line::new()
            .width(0.1)
            .shape(LineShape::Linear)
            .smoothing(1.0)
            .dash(DashType::Dash)
            .simplify(true)
            .color("#ffffff")
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
    fn test_serialize_gradient_type() {
        assert_eq!(to_value(GradientType::Radial).unwrap(), json!("radial"));
        assert_eq!(to_value(GradientType::Horizontal).unwrap(), json!("horizontal"));
        assert_eq!(to_value(GradientType::Vertical).unwrap(), json!("vertical"));
        assert_eq!(to_value(GradientType::None).unwrap(), json!("none"));
    }

    #[test]
    fn test_serialize_size_mode() {
        assert_eq!(to_value(SizeMode::Diameter).unwrap(), json!("diameter"));
        assert_eq!(to_value(SizeMode::Area).unwrap(), json!("area"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_thickness_mode() {
        assert_eq!(to_value(ThicknessMode::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(ThicknessMode::Pixels).unwrap(), json!("pixels"));
    }

    #[test]
    fn test_serialize_anchor() {
        assert_eq!(to_value(Anchor::Auto).unwrap(), json!("auto"));
        assert_eq!(to_value(Anchor::Left).unwrap(), json!("left"));
        assert_eq!(to_value(Anchor::Center).unwrap(), json!("center"));
        assert_eq!(to_value(Anchor::Right).unwrap(), json!("right"));
        assert_eq!(to_value(Anchor::Top).unwrap(), json!("top"));
        assert_eq!(to_value(Anchor::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(Anchor::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    fn test_serialize_text_anchor() {
        assert_eq!(to_value(TextAnchor::Start).unwrap(), json!("start"));
        assert_eq!(to_value(TextAnchor::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(TextAnchor::End).unwrap(), json!("end"));
    }

    #[test]
    fn test_serialize_exponent_format() {
        assert_eq!(to_value(ExponentFormat::None).unwrap(), json!("none"));
        assert_eq!(to_value(ExponentFormat::SmallE).unwrap(), json!("e"));
        assert_eq!(to_value(ExponentFormat::CapitalE).unwrap(), json!("E"));
        assert_eq!(to_value(ExponentFormat::Power).unwrap(), json!("power"));
        assert_eq!(to_value(ExponentFormat::SI).unwrap(), json!("SI"));
        assert_eq!(to_value(ExponentFormat::B).unwrap(), json!("B"));
    }
}
