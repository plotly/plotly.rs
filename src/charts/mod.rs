use crate::TraceSerialize;
use serde::{Serialize, Serializer};

mod basic;
mod financial;
mod scientific;
mod statistical;

pub use basic::Bar;
pub use basic::Scatter;

pub use statistical::BoxPlot;
pub use statistical::Density;
pub use statistical::Histogram;

pub use scientific::Contour;
pub use scientific::HeatMap;
pub use scientific::ParallelCoordinates;

pub use financial::Candlestick;
pub use financial::Ohlc;

#[derive(Debug)]
pub struct HexColor {
    r: u8,
    g: u8,
    b: u8,
}

impl HexColor {
    pub fn new(r: u8, g: u8, b: u8) -> HexColor {
        HexColor { r, g, b }
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = format!("#{:X}{:X}{:X}", self.r, self.g, self.b);
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
pub enum PlotType {
    #[serde(rename = "scatter")]
    Scatter,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "histogram")]
    Histogram,
    #[serde(rename = "histogram2dcontour")]
    Histogram2dContour,
    #[serde(rename = "contour")]
    Contour,
    #[serde(rename = "candlestick")]
    Candlestick,
    #[serde(rename = "ohlc")]
    Ohlc,
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
pub enum Color {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "grey")]
    Grey,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "maroon")]
    Maroon,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "fuchsia")]
    Fuchsia,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "lime")]
    Lime,
    #[serde(rename = "olive")]
    Olive,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "navy")]
    Navy,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "teal")]
    Teal,
    #[serde(rename = "aqua")]
    Aqua,
    Hex(HexColor),
    Rgb(RgbColor),
    Rgba(RgbaColor),
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
pub struct LineSegment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

impl LineSegment {
    fn new() -> LineSegment {
        LineSegment {
            width: None,
            color: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Marker {
    pub symbol: MarkerSymbol,
    pub opacity: f64,
    pub size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineSegment>,
}

impl Marker {
    fn new() -> Marker {
        Marker {
            symbol: MarkerSymbol::Circle,
            opacity: 1.0,
            size: 10,
            line: Some(LineSegment::new()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Font {
    pub family: String,
    pub size: usize,
    pub color: Color,
}

impl Font {
    pub fn new(family: String, size: usize, color: Color) -> Font {
        Font {
            family,
            size,
            color,
        }
    }

    pub fn default() -> Font {
        Font {
            family: String::from("Arial"),
            size: 12,
            color: Color::Black,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Title {
    pub text: String,
    pub font: Font,
}

impl Title {
    pub fn new(text: String) -> Title {
        Title {
            text,
            font: Font::default(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Axis {
    pub title: Title,
    pub r#type: String,
    pub autorange: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<f64>>,
}

impl Axis {
    pub fn new(title: &str) -> Axis {
        Axis {
            title: Title::new(title.to_owned()),
            r#type: String::from("-"),
            autorange: true,
            range: None,
        }
    }

    pub fn default_x() -> Axis {
        Axis::new("x")
    }

    pub fn default_y() -> Axis {
        Axis::new("y")
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
    pub r#type: ErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<f64>>,
    pub visible: bool,
    pub symmetric: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrayminus: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueminus: Option<f64>,
}

impl ErrorData {
    pub fn new(error_type: ErrorType) -> ErrorData {
        ErrorData {
            r#type: error_type,
            array: None,
            visible: true,
            symmetric: true,
            arrayminus: None,
            value: None,
            valueminus: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Layout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xaxis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaxis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barmode: Option<BarMode>,
}

impl Layout {
    pub fn new(title: &str) -> Layout {
        Layout {
            title: Some(Title::new(title.to_owned())),
            font: None,
            xaxis: Some(Axis::default_x()),
            yaxis: Some(Axis::default_y()),
            template: None,
            barmode: Some(BarMode::Group),
        }
    }
}

impl TraceSerialize for Layout {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
