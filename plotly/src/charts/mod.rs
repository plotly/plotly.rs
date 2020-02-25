use crate::Trace;
use serde::{Serialize, Serializer};

mod basic;
mod financial;
mod scientific;
mod statistical;

pub use basic::Bar;
pub use basic::Scatter;

pub use statistical::BoxPlot;
pub use statistical::BoxMean;
pub use statistical::BoxPoints;
pub use statistical::Density;
pub use statistical::Histogram;
pub use statistical::HistNorm;
pub use statistical::HistFunc;
pub use statistical::Bins;
pub use statistical::Cumulative;

pub use scientific::Contour;
pub use scientific::Contours;
pub use scientific::ContoursType;
pub use scientific::HeatMap;
pub use scientific::Lighting;
pub use scientific::PlaneContours;
pub use scientific::PlaneProject;
pub use scientific::Surface;
pub use scientific::SurfaceContours;

pub use financial::Candlestick;
pub use financial::Ohlc;

fn owned_string_vector<S: AsRef<str>>(s: Vec<S>) -> Vec<String> {
    s.iter()
        .map(|x| x.as_ref().to_string())
        .collect::<Vec<String>>()
}

#[derive(Debug)]
struct TruthyEnum<E> {
    pub e: E,
}

impl<E> Serialize for TruthyEnum<E>
    where E: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let s = serde_json::to_string(&self.e).unwrap().chars().filter(|c| *c != '"').collect::<String>();
        if s == "true" {
            return serializer.serialize_bool(true);
        }
        if s == "false" {
            return serializer.serialize_bool(false);
        }
        serializer.serialize_str(&s)
    }
}

#[derive(Debug)]
pub struct HexColor {
    h: String,
}

impl HexColor {
    pub fn new(h: &str) -> HexColor {
        HexColor { h: h.to_owned() }
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = format!("#{}", self.h);
        serializer.serialize_str(s.as_str())
    }
}

#[derive(Debug)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor { r, g, b }
    }
}

impl Serialize for RgbColor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = format!("rgb({}, {}, {})", self.r, self.g, self.b);
        serializer.serialize_str(s.as_str())
    }
}

#[derive(Debug)]
pub struct RgbaColor {
    r: u8,
    g: u8,
    b: u8,
    a: f64,
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> RgbaColor {
        RgbaColor { r, g, b, a }
    }
}

impl Serialize for RgbaColor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a);
        serializer.serialize_str(s.as_str())
    }
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub enum Orientation {
    #[serde(rename = "v")]
    Vertical,
    #[serde(rename = "h")]
    Horizontal,
}

#[derive(Serialize, Debug)]
pub enum GroupNorm {
    #[serde(rename = "")]
    Default,
    #[serde(rename = "fraction")]
    Fraction,
    #[serde(rename = "percent")]
    Percent,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Dim<T>
where
    T: Serialize,
{
    Scalar(T),
    Vector(Vec<T>),
}

#[derive(Serialize, Debug)]
pub enum PlotType {
    #[serde(rename = "scatter")]
    Scatter,
    #[serde(rename = "scatter3d")]
    Scatter3D,
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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

// https://www.w3.org/TR/css-color-3/#svg-color
#[derive(Debug)]
pub enum Color {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlancheDalmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornFlowerBlue,
    CornSilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGray,
    DarkGreen,
    DarkGrey,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    Goldenrod,
    Gray,
    Green,
    GreenYellow,
    Grey,
    Honeydew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenrodYellow,
    LightGray,
    LightGreen,
    LightGrey,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquamarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenrod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    Seashell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
    Transparent,
    Hex(HexColor),
    Rgb(RgbColor),
    Rgba(RgbaColor),
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            Color::Rgb(c) => Serialize::serialize(&c, serializer),
            Color::Hex(c) => Serialize::serialize(&c, serializer),
            Color::Rgba(c) => Serialize::serialize(&c, serializer),
            Color::AliceBlue => serializer.serialize_str("aliceblue"),
            Color::AntiqueWhite => serializer.serialize_str("antiquewhite"),
            Color::Aqua => serializer.serialize_str("aqua"),
            Color::Aquamarine => serializer.serialize_str("aquamarine"),
            Color::Azure => serializer.serialize_str("azure"),
            Color::Beige => serializer.serialize_str("beige"),
            Color::Bisque => serializer.serialize_str("bisque"),
            Color::Black => serializer.serialize_str("black"),
            Color::BlancheDalmond => serializer.serialize_str("blanchedalmond"),
            Color::Blue => serializer.serialize_str("blue"),
            Color::BlueViolet => serializer.serialize_str("blueviolet"),
            Color::Brown => serializer.serialize_str("brown"),
            Color::BurlyWood => serializer.serialize_str("burlywood"),
            Color::CadetBlue => serializer.serialize_str("cadetblue"),
            Color::Chartreuse => serializer.serialize_str("chartreuse"),
            Color::Chocolate => serializer.serialize_str("chocolate"),
            Color::Coral => serializer.serialize_str("coral"),
            Color::CornFlowerBlue => serializer.serialize_str("cornflowerblue"),
            Color::CornSilk => serializer.serialize_str("cornsilk"),
            Color::Crimson => serializer.serialize_str("crimson"),
            Color::Cyan => serializer.serialize_str("cyan"),
            Color::DarkBlue => serializer.serialize_str("darkblue"),
            Color::DarkCyan => serializer.serialize_str("darkcyan"),
            Color::DarkGoldenrod => serializer.serialize_str("darkgoldenrod"),
            Color::DarkGray => serializer.serialize_str("darkgray"),
            Color::DarkGreen => serializer.serialize_str("darkgreen"),
            Color::DarkGrey => serializer.serialize_str("darkgrey"),
            Color::DarkKhaki => serializer.serialize_str("darkkhaki"),
            Color::DarkMagenta => serializer.serialize_str("darkmagenta"),
            Color::DarkOliveGreen => serializer.serialize_str("darkolivegreen"),
            Color::DarkOrange => serializer.serialize_str("darkorange"),
            Color::DarkOrchid => serializer.serialize_str("darkorchid"),
            Color::DarkRed => serializer.serialize_str("darkred"),
            Color::DarkSalmon => serializer.serialize_str("darksalmon"),
            Color::DarkSeaGreen => serializer.serialize_str("darkseagreen"),
            Color::DarkSlateBlue => serializer.serialize_str("darkslateblue"),
            Color::DarkSlateGray => serializer.serialize_str("darkslategray"),
            Color::DarkSlateGrey => serializer.serialize_str("darkslategrey"),
            Color::DarkTurquoise => serializer.serialize_str("darkturquoise"),
            Color::DarkViolet => serializer.serialize_str("darkviolet"),
            Color::DeepPink => serializer.serialize_str("deeppink"),
            Color::DeepSkyBlue => serializer.serialize_str("deepskyblue"),
            Color::DimGray => serializer.serialize_str("dimgray"),
            Color::DimGrey => serializer.serialize_str("dimgrey"),
            Color::DodgerBlue => serializer.serialize_str("dodgerblue"),
            Color::FireBrick => serializer.serialize_str("firebrick"),
            Color::FloralWhite => serializer.serialize_str("floralwhite"),
            Color::ForestGreen => serializer.serialize_str("forestgreen"),
            Color::Fuchsia => serializer.serialize_str("fuchsia"),
            Color::Gainsboro => serializer.serialize_str("gainsboro"),
            Color::GhostWhite => serializer.serialize_str("ghostwhite"),
            Color::Gold => serializer.serialize_str("gold"),
            Color::Goldenrod => serializer.serialize_str("goldenrod"),
            Color::Gray => serializer.serialize_str("gray"),
            Color::Green => serializer.serialize_str("green"),
            Color::GreenYellow => serializer.serialize_str("greenyellow"),
            Color::Grey => serializer.serialize_str("grey"),
            Color::Honeydew => serializer.serialize_str("honeydew"),
            Color::HotPink => serializer.serialize_str("hotpink"),
            Color::IndianRed => serializer.serialize_str("indianred"),
            Color::Indigo => serializer.serialize_str("indigo"),
            Color::Ivory => serializer.serialize_str("ivory"),
            Color::Khaki => serializer.serialize_str("khaki"),
            Color::Lavender => serializer.serialize_str("lavender"),
            Color::LavenderBlush => serializer.serialize_str("lavenderblush"),
            Color::LawnGreen => serializer.serialize_str("lawngreen"),
            Color::LemonChiffon => serializer.serialize_str("lemonchiffon"),
            Color::LightBlue => serializer.serialize_str("lightblue"),
            Color::LightCoral => serializer.serialize_str("lightcoral"),
            Color::LightCyan => serializer.serialize_str("lightcyan"),
            Color::LightGoldenrodYellow => serializer.serialize_str("lightgoldenrodyellow"),
            Color::LightGray => serializer.serialize_str("lightgray"),
            Color::LightGreen => serializer.serialize_str("lightgreen"),
            Color::LightGrey => serializer.serialize_str("lightgrey"),
            Color::LightPink => serializer.serialize_str("lightpink"),
            Color::LightSalmon => serializer.serialize_str("lightsalmon"),
            Color::LightSeaGreen => serializer.serialize_str("lightseagreen"),
            Color::LightSkyBlue => serializer.serialize_str("lightskyblue"),
            Color::LightSlateGray => serializer.serialize_str("lightslategray"),
            Color::LightSlateGrey => serializer.serialize_str("lightslategrey"),
            Color::LightSteelBlue => serializer.serialize_str("lightsteelblue"),
            Color::LightYellow => serializer.serialize_str("lightyellow"),
            Color::Lime => serializer.serialize_str("lime"),
            Color::LimeGreen => serializer.serialize_str("limegreen"),
            Color::Linen => serializer.serialize_str("linen"),
            Color::Magenta => serializer.serialize_str("magenta"),
            Color::Maroon => serializer.serialize_str("maroon"),
            Color::MediumAquamarine => serializer.serialize_str("mediumaquamarine"),
            Color::MediumBlue => serializer.serialize_str("mediumblue"),
            Color::MediumOrchid => serializer.serialize_str("mediumorchid"),
            Color::MediumPurple => serializer.serialize_str("mediumpurple"),
            Color::MediumSeaGreen => serializer.serialize_str("mediumseagreen"),
            Color::MediumSlateBlue => serializer.serialize_str("mediumslateblue"),
            Color::MediumSpringGreen => serializer.serialize_str("mediumspringgreen"),
            Color::MediumTurquoise => serializer.serialize_str("mediumturquoise"),
            Color::MediumVioletRed => serializer.serialize_str("mediumvioletred"),
            Color::MidnightBlue => serializer.serialize_str("midnightblue"),
            Color::MintCream => serializer.serialize_str("mintcream"),
            Color::MistyRose => serializer.serialize_str("mistyrose"),
            Color::Moccasin => serializer.serialize_str("moccasin"),
            Color::NavajoWhite => serializer.serialize_str("navajowhite"),
            Color::Navy => serializer.serialize_str("navy"),
            Color::OldLace => serializer.serialize_str("oldlace"),
            Color::Olive => serializer.serialize_str("olive"),
            Color::OliveDrab => serializer.serialize_str("olivedrab"),
            Color::Orange => serializer.serialize_str("orange"),
            Color::OrangeRed => serializer.serialize_str("orangered"),
            Color::Orchid => serializer.serialize_str("orchid"),
            Color::PaleGoldenrod => serializer.serialize_str("palegoldenrod"),
            Color::PaleGreen => serializer.serialize_str("palegreen"),
            Color::PaleTurquoise => serializer.serialize_str("paleturquoise"),
            Color::PaleVioletRed => serializer.serialize_str("palevioletred"),
            Color::PapayaWhip => serializer.serialize_str("papayawhip"),
            Color::PeachPuff => serializer.serialize_str("peachpuff"),
            Color::Peru => serializer.serialize_str("peru"),
            Color::Pink => serializer.serialize_str("pink"),
            Color::Plum => serializer.serialize_str("plum"),
            Color::PowderBlue => serializer.serialize_str("powderblue"),
            Color::Purple => serializer.serialize_str("purple"),
            Color::Red => serializer.serialize_str("red"),
            Color::RosyBrown => serializer.serialize_str("rosybrown"),
            Color::RoyalBlue => serializer.serialize_str("royalblue"),
            Color::SaddleBrown => serializer.serialize_str("saddlebrown"),
            Color::Salmon => serializer.serialize_str("salmon"),
            Color::SandyBrown => serializer.serialize_str("sandybrown"),
            Color::SeaGreen => serializer.serialize_str("seagreen"),
            Color::Seashell => serializer.serialize_str("seashell"),
            Color::Sienna => serializer.serialize_str("sienna"),
            Color::Silver => serializer.serialize_str("silver"),
            Color::SkyBlue => serializer.serialize_str("skyblue"),
            Color::SlateBlue => serializer.serialize_str("slateblue"),
            Color::SlateGray => serializer.serialize_str("slategray"),
            Color::SlateGrey => serializer.serialize_str("slategrey"),
            Color::Snow => serializer.serialize_str("snow"),
            Color::SpringGreen => serializer.serialize_str("springgreen"),
            Color::SteelBlue => serializer.serialize_str("steelblue"),
            Color::Tan => serializer.serialize_str("tan"),
            Color::Teal => serializer.serialize_str("teal"),
            Color::Thistle => serializer.serialize_str("thistle"),
            Color::Tomato => serializer.serialize_str("tomato"),
            Color::Turquoise => serializer.serialize_str("turquoise"),
            Color::Violet => serializer.serialize_str("violet"),
            Color::Wheat => serializer.serialize_str("wheat"),
            Color::White => serializer.serialize_str("white"),
            Color::WhiteSmoke => serializer.serialize_str("whitesmoke"),
            Color::Yellow => serializer.serialize_str("yellow"),
            Color::YellowGreen => serializer.serialize_str("yellowgreen"),
            Color::Transparent => serializer.serialize_str("transparent"),
        }
    }
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub struct ColorScaleElement(f64, Color);

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ColorScale {
    Palette(ColorScalePalette),
    Vector(Vec<ColorScaleElement>),
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

#[derive(Serialize, Debug)]
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
    color: Option<Color>,
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
    outlier_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outlierwidth")]
    outlier_width: Option<usize>,
}

impl Line {
    pub fn new() -> Line {
        Line {
            width: None,
            shape: None,
            smoothing: None,
            dash: None,
            simplify: None,
            color: None,
            cauto: None,
            cmin: None,
            cmax: None,
            cmid: None,
            color_scale: None,
            auto_color_scale: None,
            reverse_scale: None,
            outlier_color: None,
            outlier_width: None,
        }
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

    pub fn color(mut self, color: Color) -> Line {
        self.color = Some(color);
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

    pub fn outlier_color(mut self, outlier_color: Color) -> Line {
        self.outlier_color = Some(outlier_color);
        self
    }

    pub fn outlier_width(mut self, outlier_width: usize) -> Line {
        self.outlier_width = Some(outlier_width);
        self
    }
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub enum SizeMode {
    #[serde(rename = "diameter")]
    Diameter,
    #[serde(rename = "area")]
    Area,
}

#[derive(Serialize, Debug)]
pub enum ThicknessMode {
    #[serde(rename = "fraction")]
    Fraction,
    #[serde(rename = "pixels")]
    Pixels,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub enum TextAnchor {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
}

#[derive(Serialize, Debug)]
pub enum TickMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "array")]
    Array,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub struct Gradient {
    r#type: GradientType,
    color: Dim<Color>,
}

impl Gradient {
    pub fn new(gradient_type: GradientType, color: Dim<Color>) -> Gradient {
        Gradient {
            r#type: gradient_type,
            color,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TickFormatStops {
    enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dtickrange")]
    dtick_range: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl TickFormatStops {
    pub fn new() -> TickFormatStops {
        TickFormatStops {
            enabled: true,
            dtick_range: None,
            value: None,
            name: None,
            template_item_name: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> TickFormatStops {
        self.enabled = enabled;
        self
    }

    pub fn dtick_range(mut self, range: Vec<f64>) -> TickFormatStops {
        self.dtick_range = Some(range);
        self
    }

    pub fn value(mut self, value: &str) -> TickFormatStops {
        self.value = Some(value.to_owned());
        self
    }

    pub fn name(mut self, name: &str) -> TickFormatStops {
        self.name = Some(name.to_owned());
        self
    }

    pub fn template_item_name(mut self, name: &str) -> TickFormatStops {
        self.template_item_name = Some(name.to_owned());
        self
    }
}

#[derive(Serialize, Debug)]
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
    outline_color: Option<Color>,
    #[serde(rename = "outlinewidth")]
    outline_width: usize,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<Color>,
    #[serde(rename = "borderwidth")]
    border_width: usize,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<Color>,
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
    tick_color: Option<Color>,
    #[serde(rename = "showticklabels")]
    show_tick_labels: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickfont")]
    tick_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickangle")]
    tick_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickformat")]
    tick_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tickformatstops")]
    tick_format_stops: Option<TickFormatStops>,
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

impl ColorBar {
    pub fn new() -> ColorBar {
        ColorBar {
            thickness_mode: None,
            thickness: 30,
            len_mode: None,
            len: 1,
            x: 1.02,
            x_anchor: Anchor::Left,
            x_pad: 10.0,
            y: 0.5,
            y_anchor: Anchor::Middle,
            y_pad: 10.0,
            outline_color: None,
            outline_width: 1,
            border_color: None,
            border_width: 0,
            background_color: None,
            tick_mode: None,
            n_ticks: 0,
            tick0: None,
            dtick: None,
            tick_vals: None,
            tick_text: None,
            ticks: None,
            tick_len: 5,
            tick_width: 1,
            tick_color: None,
            show_tick_labels: true,
            tick_font: None,
            tick_angle: None,
            tick_format: None,
            tick_format_stops: None,
            tick_prefix: None,
            show_tick_prefix: None,
            tick_suffix: None,
            show_tick_suffix: None,
            separate_thousands: true,
            exponent_format: None,
            show_exponent: None,
            title: None,
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

    pub fn outline_color(mut self, outline_color: Color) -> ColorBar {
        self.outline_color = Some(outline_color);
        self
    }

    pub fn outline_width(mut self, outline_width: usize) -> ColorBar {
        self.outline_width = outline_width;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> ColorBar {
        self.border_color = Some(border_color);
        self
    }

    pub fn border_width(mut self, border_width: usize) -> ColorBar {
        self.border_width = border_width;
        self
    }

    pub fn background_color(mut self, background_color: Color) -> ColorBar {
        self.background_color = Some(background_color);
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

    pub fn tick_color(mut self, tick_color: Color) -> ColorBar {
        self.tick_color = Some(tick_color);
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

    pub fn tick_format_stops(mut self, tick_format_stops: TickFormatStops) -> ColorBar {
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

#[derive(Serialize, Debug)]
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
    color: Option<Dim<Color>>,
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
    outlier_color: Option<Color>,
}

impl Marker {
    pub fn new() -> Marker {
        Marker {
            symbol: None,
            opacity: None,
            size: None,
            max_displayed: None,
            size_ref: None,
            size_min: None,
            size_mode: None,
            line: None,
            gradient: None,
            color: None,
            cauto: None,
            cmin: None,
            cmax: None,
            cmid: None,
            color_scale: None,
            auto_color_scale: None,
            reverse_scale: None,
            show_scale: None,
            color_bar: None,
            outlier_color: None,
        }
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

    pub fn color(mut self, color: Color) -> Marker {
        self.color = Some(Dim::Scalar(color));
        self
    }

    pub fn color_array(mut self, color: Vec<Color>) -> Marker {
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

    pub fn outlier_color(mut self, outlier_color: Color) -> Marker {
        self.outlier_color = Some(outlier_color);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct Font {
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
}

impl Font {
    pub fn new() -> Font {
        Font {
            family: None,
            size: None,
            color: None,
        }
    }

    pub fn family(mut self, family: &str) -> Font {
        self.family = Some(family.to_owned());
        self
    }

    pub fn size(mut self, size: usize) -> Font {
        self.size = Some(size);
        self
    }

    pub fn color(mut self, color: Color) -> Font {
        self.color = Some(color);
        self
    }
}

#[derive(Serialize, Debug)]
pub enum Reference {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "paper")]
    Paper,
}

#[derive(Serialize, Debug)]
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
pub struct Legend {
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<Color>,
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

    pub fn background_color(mut self, background_color: Color) -> Legend {
        self.background_color = Some(background_color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Legend {
        self.border_color = Some(border_color);
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

impl Title {
    pub fn new(text: &str) -> Title {
        Title {
            text: text.to_owned(),
            font: None,
            side: None,
            x_ref: None,
            y_ref: None,
            x: None,
            y: None,
            x_anchor: None,
            y_anchor: None,
            pad: None,
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
pub enum RangeMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "tozero")]
    ToZero,
    #[serde(rename = "nonnegative")]
    NonNegative,
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
pub struct Axis {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
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
    tick_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showticklabels")]
    show_tick_labels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "automargin")]
    auto_margin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showspikes")]
    show_spikes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spikecolor")]
    spike_color: Option<Color>,
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
    line_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "linewidth")]
    line_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showgrid")]
    show_grid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridcolor")]
    grid_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gridwidth")]
    grid_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zeroline")]
    zero_line: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zerolinecolor")]
    zero_line_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zerolinewidth")]
    zero_line_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showdividers")]
    show_dividers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "dividercolor")]
    divider_color: Option<Color>,
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

    pub fn color(mut self, color: Color) -> Axis {
        self.color = Some(color);
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

    pub fn tick_color(mut self, tick_color: Color) -> Axis {
        self.tick_color = Some(tick_color);
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

    pub fn spike_color(mut self, spike_color: Color) -> Axis {
        self.spike_color = Some(spike_color);
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

    pub fn line_color(mut self, line_color: Color) -> Axis {
        self.line_color = Some(line_color);
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

    pub fn grid_color(mut self, grid_color: Color) -> Axis {
        self.grid_color = Some(grid_color);
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

    pub fn zero_line_color(mut self, zero_line_color: Color) -> Axis {
        self.zero_line_color = Some(zero_line_color);
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

    pub fn divider_color(mut self, divider_color: Color) -> Axis {
        self.divider_color = Some(divider_color);
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
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none", rename = "bgcolor")]
    background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bordercolor")]
    border_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "namelength")]
    name_length: Option<Dim<i32>>,
}

impl Label {
    pub fn new() -> Label {
        Label {
            background_color: None,
            border_color: None,
            font: None,
            align: None,
            name_length: None,
        }
    }

    pub fn background_color(mut self, background_color: Color) -> Label {
        self.background_color = Some(background_color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Label {
        self.border_color = Some(border_color);
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thickness: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
}

impl ErrorData {
    pub fn new(error_type: ErrorType) -> ErrorData {
        ErrorData {
            r#type: error_type,
            array: None,
            visible: None,
            symmetric: None,
            array_minus: None,
            value: None,
            value_minus: None,
            trace_ref: None,
            trace_ref_minus: None,
            copy_ystyle: None,
            color: None,
            thickness: None,
            width: None,
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

    pub fn color(mut self, color: Color) -> ErrorData {
        self.color = Some(color);
        self
    }

    pub fn thickness(mut self, thickness: usize) -> ErrorData {
        self.thickness = Some(thickness);
        self
    }

    pub fn width(mut self, width: usize) -> ErrorData {
        self.width = Some(width);
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

    // uniform_text: Option<UniformText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    separators: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "paper_bgcolor")]
    paper_background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "plot_bgcolor")]
    plot_background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<LayoutColorScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colorway: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "coloraxis")]
    color_axis: Option<ColorAxis>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovermode")]

    // mode_bar: Option<ModeBar>,

    hover_mode: Option<String>,
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

    // grid: Option<LayoutGrid>,

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
    pie_colorway: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extendpiecolors")]
    extend_pie_colors: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "sunburstcolorway")]
    sunburst_colorway: Option<Vec<Color>>,
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
            separators: None,
            paper_background_color: None,
            plot_background_color: None,
            color_scale: None,
            colorway: None,
            color_axis: None,
            hover_mode: None,
            click_mode: None,
            drag_mode: None,
            select_direction: None,
            hover_distance: None,
            spike_distance: None,
            hover_label: None,
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

    pub fn separators(mut self, separators: &str) -> Layout {
        self.separators = Some(separators.to_owned());
        self
    }

    pub fn paper_background_color(mut self, paper_background_color: Color) -> Layout {
        self.paper_background_color = Some(paper_background_color);
        self
    }

    pub fn plot_background_color(mut self, plot_background_color: Color) -> Layout {
        self.plot_background_color = Some(plot_background_color);
        self
    }

    pub fn color_scale(mut self, color_scale: LayoutColorScale) -> Layout {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn colorway(mut self, colorway: Vec<Color>) -> Layout {
        self.colorway = Some(colorway);
        self
    }

    pub fn color_axis(mut self, color_axis: ColorAxis) -> Layout {
        self.color_axis = Some(color_axis);
        self
    }

    pub fn hover_mode(mut self, hover_mode: &str) -> Layout {
        self.hover_mode = Some(hover_mode.to_owned());
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

    pub fn pie_colorway(mut self, pie_colorway: Vec<Color>) -> Layout {
        self.pie_colorway = Some(pie_colorway);
        self
    }

    pub fn extend_pie_colors(mut self, extend_pie_colors: bool) -> Layout {
        self.extend_pie_colors = Some(extend_pie_colors);
        self
    }


    pub fn sunburst_colorway(mut self, sunburst_colorway: Vec<Color>) -> Layout {
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
