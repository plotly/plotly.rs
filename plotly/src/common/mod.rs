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
pub enum HoverInfo {
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
    #[serde(rename = "z")]
    Z,
    #[serde(rename = "x+y")]
    XAndY,
    #[serde(rename = "x+z")]
    XAndZ,
    #[serde(rename = "y+z")]
    YAndZ,
    #[serde(rename = "x+y+z")]
    XAndYAndZ,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "skip")]
    Skip,
}

#[derive(Serialize, Clone, Debug)]
pub enum TextPosition {
    #[serde(rename = "inside")]
    Inside,
    #[serde(rename = "outside")]
    Outside,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Clone, Debug)]
pub enum ConstrainText {
    #[serde(rename = "inside")]
    Inside,
    #[serde(rename = "outside")]
    Outside,
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Clone, Debug)]
pub enum Orientation {
    #[serde(rename = "v")]
    Vertical,
    #[serde(rename = "h")]
    Horizontal,
}

#[derive(Serialize, Clone, Debug)]
pub enum GroupNorm {
    #[serde(rename = "")]
    Default,
    #[serde(rename = "fraction")]
    Fraction,
    #[serde(rename = "percent")]
    Percent,
}

#[derive(Serialize, Clone, Debug)]
pub enum Fill {
    #[serde(rename = "tozeroy")]
    ToZeroY,
    #[serde(rename = "tozerox")]
    ToZeroX,
    #[serde(rename = "tonexty")]
    ToNextY,
    #[serde(rename = "tonextx")]
    ToNextX,
    #[serde(rename = "toself")]
    ToSelf,
    #[serde(rename = "tonext")]
    ToNext,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Clone, Debug)]
pub enum Calendar {
    #[serde(rename = "gregorian")]
    Gregorian,
    #[serde(rename = "chinese")]
    Chinese,
    #[serde(rename = "coptic")]
    Coptic,
    #[serde(rename = "discworld")]
    DiscWorld,
    #[serde(rename = "ethiopian")]
    Ethiopian,
    #[serde(rename = "hebrew")]
    Hebrew,
    #[serde(rename = "islamic")]
    Islamic,
    #[serde(rename = "julian")]
    Julian,
    #[serde(rename = "mayan")]
    Mayan,
    #[serde(rename = "nanakshahi")]
    Nanakshahi,
    #[serde(rename = "nepali")]
    Nepali,
    #[serde(rename = "persian")]
    Persian,
    #[serde(rename = "jalali")]
    Jalali,
    #[serde(rename = "taiwan")]
    Taiwan,
    #[serde(rename = "thai")]
    Thai,
    #[serde(rename = "ummalqura")]
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

#[derive(Serialize, Clone, Debug)]
pub enum PlotType {
    #[serde(rename = "scatter")]
    Scatter,
    #[serde(rename = "scattergl")]
    ScatterGL,
    #[serde(rename = "scatter3d")]
    Scatter3D,
    #[serde(rename = "scatterpolar")]
    ScatterPolar,
    #[serde(rename = "scatterpolargl")]
    ScatterPolarGL,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "candlestick")]
    Candlestick,
    #[serde(rename = "contour")]
    Contour,
    #[serde(rename = "heatmap")]
    HeatMap,
    #[serde(rename = "histogram")]
    Histogram,
    #[serde(rename = "histogram2dcontour")]
    Histogram2dContour,
    #[serde(rename = "ohlc")]
    Ohlc,
    #[serde(rename = "surface")]
    Surface,
}

impl Default for PlotType {
    fn default() -> Self {
        PlotType::Scatter
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum Mode {
    #[serde(rename = "lines")]
    Lines,
    #[serde(rename = "markers")]
    Markers,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "lines+markers")]
    LinesMarkers,
    #[serde(rename = "lines+text")]
    LinesText,
    #[serde(rename = "markers+text")]
    MarkersText,
    #[serde(rename = "lines+markers+text")]
    LinesMarkersText,
    #[serde(rename = "none")]
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
pub enum MarkerSymbol {
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "circle-open")]
    CirleOpen,
    #[serde(rename = "circle-dot")]
    CircleDot,
    #[serde(rename = "circle-open-dot")]
    CircleOpenDot,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "square-open")]
    SquareOpen,
    #[serde(rename = "square-dot")]
    SquareDot,
    #[serde(rename = "square-open-dot")]
    SquareOpenDot,
    #[serde(rename = "diamond")]
    Diamond,
    #[serde(rename = "diamond-open")]
    DiamondOpen,
    #[serde(rename = "diamond-dot")]
    DiamondDot,
    #[serde(rename = "diamond-open-dot")]
    DiamondOpenDot,
    #[serde(rename = "cross")]
    Cross,
    #[serde(rename = "cross-open")]
    CrossOpen,
    #[serde(rename = "cross-dot")]
    CrossDot,
    #[serde(rename = "cross-open-dot")]
    CrossOpenDot,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "x-open")]
    XOpen,
    #[serde(rename = "x-dot")]
    XDot,
    #[serde(rename = "x-open-dot")]
    XOpenDot,
    #[serde(rename = "triangle-up")]
    TriangleUp,
    #[serde(rename = "triangle-up-open")]
    TriangleUpOpen,
    #[serde(rename = "triangle-up-dot")]
    TriangleUpDot,
    #[serde(rename = "triangle-up-open-dot")]
    TriangleUpOpenDot,
    #[serde(rename = "triangle-down")]
    TriangleDown,
    #[serde(rename = "triangle-down-open")]
    TriangleDownOpen,
    #[serde(rename = "triangle-down-dot")]
    TriangleDownDot,
    #[serde(rename = "triangle-down-open-dot")]
    TriangleDownOpenDot,
    #[serde(rename = "triangle-left")]
    TriangleLeft,
    #[serde(rename = "triangle-left-open")]
    TriangleLeftOpen,
    #[serde(rename = "triangle-left-dot")]
    TriangleLeftDot,
    #[serde(rename = "triangle-left-open-dot")]
    TriangleLeftOpenDot,
    #[serde(rename = "triangle-right")]
    TriangleRight,
    #[serde(rename = "triangle-right-open")]
    TriangleRightOpen,
    #[serde(rename = "triangle-right-dot")]
    TriangleRightDot,
    #[serde(rename = "triangle-right-open-dot")]
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
    #[serde(rename = "pentagon")]
    Pentagon,
    #[serde(rename = "pentagon-open")]
    PentagonOpen,
    #[serde(rename = "pentagon-dot")]
    PentagonDot,
    #[serde(rename = "pentagon-open-dot")]
    PentagonOpenDot,
    #[serde(rename = "hexagon")]
    Hexagon,
    #[serde(rename = "hexagon-open")]
    HexagonOpen,
    #[serde(rename = "hexagon-dot")]
    HexagonDot,
    #[serde(rename = "hexagon-open-dot")]
    HexagonOpenDot,
    #[serde(rename = "hexagon2")]
    Hexagon2,
    #[serde(rename = "hexagon2-open")]
    Hexagon2Open,
    #[serde(rename = "hexagon2-dot")]
    Hexagon2Dot,
    #[serde(rename = "hexagon2-open-dot")]
    Hexagon2OpenDot,
    #[serde(rename = "octagon")]
    Octagon,
    #[serde(rename = "octagon-open")]
    OctagonOpen,
    #[serde(rename = "octagon-dot")]
    OctagonDot,
    #[serde(rename = "octagon-open-dot")]
    OctagonOpenDot,
    #[serde(rename = "star")]
    Star,
    #[serde(rename = "star-open")]
    StarOpen,
    #[serde(rename = "star-dot")]
    StarDot,
    #[serde(rename = "star-open-dot")]
    StarOpenDot,
    #[serde(rename = "hexagram")]
    Hexagram,
    #[serde(rename = "hexagram-open")]
    HexagramOpen,
    #[serde(rename = "hexagram-dot")]
    HexagramDot,
    #[serde(rename = "hexagram-open-dot")]
    HexagramOpenDot,
    #[serde(rename = "star-triangle-up")]
    StarTriangleUp,
    #[serde(rename = "star-triangle-up-open")]
    StarTriangleUpOpen,
    #[serde(rename = "star-triangle-up-dot")]
    StarTriangleUpDot,
    #[serde(rename = "star-triangle-up-open-dot")]
    StarTriangleUpOpenDot,
    #[serde(rename = "star-triangle-down")]
    StarTriangleDown,
    #[serde(rename = "star-triangle-down-open")]
    StarTriangleDownOpen,
    #[serde(rename = "star-triangle-down-dot")]
    StarTriangleDownDot,
    #[serde(rename = "star-triangle-down-open-dot")]
    StarTriangleDownOpenDot,
    #[serde(rename = "star-square")]
    StarSquare,
    #[serde(rename = "star-square-open")]
    StarSquareOpen,
    #[serde(rename = "star-square-dot")]
    StarSquareDot,
    #[serde(rename = "star-square-open-dot")]
    StarSquareOpenDot,
    #[serde(rename = "star-diamond")]
    StarDiamond,
    #[serde(rename = "star-diamond-open")]
    StarDiamondOpen,
    #[serde(rename = "star-diamond-dot")]
    StarDiamondDot,
    #[serde(rename = "star-diamond-open-dot")]
    StarDiamondOpenDot,
    #[serde(rename = "diamond-tall")]
    DiamondTall,
    #[serde(rename = "diamond-tall-open")]
    DiamondTallOpen,
    #[serde(rename = "diamond-tall-dot")]
    DiamondTallDot,
    #[serde(rename = "diamond-tall-open-dot")]
    DiamondTallOpenDot,
    #[serde(rename = "diamond-wide")]
    DiamondWide,
    #[serde(rename = "diamond-wide-open")]
    DiamondWideOpen,
    #[serde(rename = "diamond-wide-dot")]
    DiamondWideDot,
    #[serde(rename = "diamond-wide-open-dot")]
    DiamondWideOpenDot,
    #[serde(rename = "hourglass")]
    Hourglass,
    #[serde(rename = "hourglass-open")]
    HourglassOpen,
    #[serde(rename = "bowtie")]
    BowTie,
    #[serde(rename = "bowtie-open")]
    BowTieOpen,
    #[serde(rename = "circle-cross")]
    CircleCross,
    #[serde(rename = "circle-cross-open")]
    CircleCrossOpen,
    #[serde(rename = "circle-x")]
    CircleX,
    #[serde(rename = "circle-x-open")]
    CircleXOpen,
    #[serde(rename = "square-cross")]
    SquareCross,
    #[serde(rename = "square-cross-open")]
    SquareCrossOpen,
    #[serde(rename = "square-x")]
    SquareX,
    #[serde(rename = "square-x-open")]
    SquareXOpen,
    #[serde(rename = "diamond-cross")]
    DiamondCross,
    #[serde(rename = "diamond-cross-open")]
    DiamondCrossOpen,
    #[serde(rename = "diamond-x")]
    DiamondX,
    #[serde(rename = "diamond-x-open")]
    DiamondXOpen,
    #[serde(rename = "cross-thin")]
    CrossThin,
    #[serde(rename = "cross-thin-open")]
    CrossThinOpen,
    #[serde(rename = "x-thin")]
    XThin,
    #[serde(rename = "x-thin-open")]
    XThinOpen,
    #[serde(rename = "asterisk")]
    Asterisk,
    #[serde(rename = "asterisk-open")]
    AsteriskOpen,
    #[serde(rename = "hash")]
    Hash,
    #[serde(rename = "hash-open")]
    HashOpen,
    #[serde(rename = "hash-dot")]
    HashDot,
    #[serde(rename = "hash-open-dot")]
    HashOpenDot,
    #[serde(rename = "y-up")]
    YUp,
    #[serde(rename = "y-up-open")]
    YUpOpen,
    #[serde(rename = "y-down")]
    YDown,
    #[serde(rename = "y-down-open")]
    YDownOpen,
    #[serde(rename = "y-left")]
    YLeft,
    #[serde(rename = "y-left-open")]
    YLeftOpen,
    #[serde(rename = "y-right")]
    YRight,
    #[serde(rename = "y-right-open")]
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
pub enum TickMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "array")]
    Array,
}

#[derive(Serialize, Clone, Debug)]
pub enum DashType {
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "dot")]
    Dot,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "longdashdot")]
    LongDash,
    #[serde(rename = "dashdot")]
    DashDot,
    #[serde(rename = "longdashdot")]
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

#[derive(Serialize, Clone, Debug)]
pub enum LineShape {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "spline")]
    Spline,
    #[serde(rename = "hv")]
    Hv,
    #[serde(rename = "vh")]
    Vh,
    #[serde(rename = "hvh")]
    Hvh,
    #[serde(rename = "vhv")]
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
    pub fn new() -> Line {
        Default::default()
    }

    pub fn width(mut self, width: f64) -> Line {
        self.width = Some(width);
        self
    }

    pub fn shape(mut self, shape: LineShape) -> Line {
        self.shape = Some(shape);
        self
    }

    pub fn smoothing(mut self, smoothing: f64) -> Line {
        self.smoothing = Some(smoothing);
        self
    }

    pub fn dash(mut self, dash: DashType) -> Line {
        self.dash = Some(dash);
        self
    }

    pub fn simplify(mut self, simplify: bool) -> Line {
        self.simplify = Some(simplify);
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Line {
        self.color = Some(color.to_color());
        self
    }

    pub fn cauto(mut self, cauto: bool) -> Line {
        self.cauto = Some(cauto);
        self
    }

    pub fn cmin(mut self, cmin: f64) -> Line {
        self.cmin = Some(cmin);
        self
    }

    pub fn cmax(mut self, cmax: f64) -> Line {
        self.cmax = Some(cmax);
        self
    }

    pub fn cmid(mut self, cmid: f64) -> Line {
        self.cmid = Some(cmid);
        self
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Line {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Line {
        self.auto_color_scale = Some(auto_color_scale);
        self
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Line {
        self.reverse_scale = Some(reverse_scale);
        self
    }

    pub fn outlier_color<C: Color>(mut self, outlier_color: C) -> Line {
        self.outlier_color = Some(outlier_color.to_color());
        self
    }

    pub fn outlier_width(mut self, outlier_width: usize) -> Line {
        self.outlier_width = Some(outlier_width);
        self
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum GradientType {
    #[serde(rename = "radial")]
    Radial,
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Clone, Debug)]
pub enum SizeMode {
    #[serde(rename = "diameter")]
    Diameter,
    #[serde(rename = "area")]
    Area,
}

#[derive(Serialize, Clone, Debug)]
pub enum ThicknessMode {
    #[serde(rename = "fraction")]
    Fraction,
    #[serde(rename = "pixels")]
    Pixels,
}

#[derive(Serialize, Clone, Debug)]
pub enum Anchor {
    #[serde(rename = "auto")]
    Auto,
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

#[derive(Serialize, Clone, Debug)]
pub enum TextAnchor {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
}

#[derive(Serialize, Clone, Debug)]
pub enum ExponentFormat {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "e")]
    SmallE,
    #[serde(rename = "E")]
    CapitalE,
    #[serde(rename = "power")]
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
        Self {
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
            thickness_mode: None,
            len_mode: None,
            outline_color: None,
            border_color: None,
            background_color: None,
            tick_mode: None,
            tick0: None,
            dtick: None,
            tick_vals: None,
            tick_text: None,
            ticks: None,
            tick_color: None,
            tick_font: None,
            tick_angle: None,
            tick_format: None,
            tick_format_stops: None,
            tick_prefix: None,
            show_tick_prefix: None,
            tick_suffix: None,
            show_tick_suffix: None,
            exponent_format: None,
            show_exponent: None,
            title: None,
        }
    }
}

impl ColorBar {
    pub fn new() -> ColorBar {
        Default::default()
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
