use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

/// This module provides several user interfaces for describing a color to be
/// used throughout the rest of the library. The easiest way of describing a
/// colour is to use a `&str` or `String`, which is simply serialized as-is and
/// passed on to the underlying `plotly.js` library. `plotly.js` supports [`CSS
/// color formats`], and will fallback to some default color if the color string
/// is malformed.
///
/// For a more type-safe approach, the `RGB` or `RGBA` structs can be used to
/// construct a valid color, which will then get serialized to an appropriate
/// string representation. Cross-browser compatible [`predefined colors`] are
/// supported via the `NamedColor` enum.
///
/// The `Color` trait is public, and so can be implemented for custom colour
/// types. The user can then implement a valid serialization function according
/// to their own requirements. On the whole, that should be largely unnecessary
/// given the functionality already provided within this module.
///
/// [`CSS color formats`]: <https://www.w3schools.com/cssref/css_colors_legal.asp>
/// [`predefined colors`]: <https://www.w3schools.com/cssref/css_colors.asp>
use dyn_clone::DynClone;
use erased_serde::Serialize as ErasedSerialize;
use serde::{de, Deserialize, Deserializer, Serialize};

/// A marker trait allowing several ways to describe a color.
pub trait Color: DynClone + ErasedSerialize + Send + Sync + std::fmt::Debug + 'static {}

dyn_clone::clone_trait_object!(Color);
erased_serde::serialize_trait_object!(Color);

impl Color for NamedColor {}
impl Color for &'static str {}
impl Color for String {}
impl Color for Rgb {}
impl Color for Rgba {}
impl Color for f64 {}
impl Color for f32 {}
impl Color for u64 {}
impl Color for u32 {}
impl Color for u16 {}
impl Color for u8 {}
impl Color for i64 {}
impl Color for i32 {}
impl Color for i16 {}
impl Color for i8 {}
impl Color for usize {}

/// ColorArray is only used internally to provide a helper method for converting
/// Vec<impl Color> to Vec<Box<dyn Color>>, as we would otherwise fall foul of
/// the orphan rules.
pub(crate) struct ColorArray<C: Color>(pub(crate) Vec<C>);

#[allow(clippy::from_over_into)]
impl<C: Color> Into<Vec<Box<dyn Color>>> for ColorArray<C> {
    fn into(self) -> Vec<Box<dyn Color>> {
        self.0
            .into_iter()
            .map(|c| Box::new(c) as Box<dyn Color>)
            .collect()
    }
}

/// A type-safe way of constructing a valid RGB color from constituent R, G and
/// B channels.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rgb {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Rgb {
    /// Create a new Rgb instance.
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("rgb({}, {}, {})", self.r, self.g, self.b))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::new(err.to_string().as_str())
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> ParseError {
        ParseError::new(err.to_string().as_str())
    }
}

impl FromStr for Rgb {
    type Err = ParseError;
    fn from_str(rgb: &str) -> std::result::Result<Self, Self::Err> {
        let prefix: &[_] = &['r', 'g', 'b', 'a', '('];
        let trimmed = rgb.trim_start_matches(prefix).trim_end_matches(')');
        let fields: Vec<&str> = trimmed.split(',').collect();
        if fields.len() != 3 {
            Err(ParseError::new("Invalid string length of for RGB color"))
        } else {
            Ok(Rgb {
                r: u8::from_str(fields[0].trim())?,
                g: u8::from_str(fields[1].trim())?,
                b: u8::from_str(fields[2].trim())?,
            })
        }
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

/// A type-safe way of constructing a valid RGBA color from constituent R, G, B
/// and A channels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: f64,
}

impl Rgba {
    /// Create a new Rgba instance.
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> Rgba {
        Rgba { r, g, b, a }
    }
}

impl Serialize for Rgba {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "rgba({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        ))
    }
}

impl FromStr for Rgba {
    type Err = ParseError;
    fn from_str(rgba: &str) -> std::result::Result<Self, Self::Err> {
        let prefix: &[_] = &['r', 'g', 'b', 'a', '('];
        let trimmed = rgba.trim_start_matches(prefix).trim_end_matches(')');
        let fields: Vec<&str> = trimmed.split(',').collect();
        dbg!(&fields);
        println!("{:?}", &fields);
        if fields.len() != 4 {
            Err(ParseError::new("Invalid string length of for RGBA color"))
        } else {
            Ok(Rgba {
                r: u8::from_str(fields[0].trim())?,
                g: u8::from_str(fields[1].trim())?,
                b: u8::from_str(fields[2].trim())?,
                a: f64::from_str(fields[3].trim())?,
            })
        }
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

/// Cross-browser compatible [`predefined colors`].
///
/// [`predefined colors`]: <https://www.w3schools.com/cssref/css_colors.asp>
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NamedColor {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
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
    RebeccaPurple,
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
}

#[cfg(test)]
mod tests {
    use serde_json::{from_value, json, to_value};

    use super::*;

    #[test]
    fn serialize_rgb() {
        let rgb = Rgb::new(80, 90, 100);
        assert_eq!(to_value(rgb).unwrap(), json!("rgb(80, 90, 100)"));
    }

    #[test]
    fn deserialize_rgb() {
        let rgb = json!("rgb(80, 90, 100)");
        let expected = Rgb::new(80, 90, 100);
        assert_eq!(from_value::<Rgb>(rgb).unwrap(), expected);
    }

    #[test]
    fn serialize_rgba() {
        let rgba = Rgba::new(80, 90, 100, 0.2);
        assert_eq!(to_value(rgba).unwrap(), json!("rgba(80, 90, 100, 0.2)"));
    }

    #[test]
    fn deserialize_rgba() {
        let rgba = json!("rgba(80, 90, 100, 0.2)");
        let expected = Rgba::new(80, 90, 100, 0.2);
        assert_eq!(from_value::<Rgba>(rgba).unwrap(), expected);
    }

    #[test]
    fn serialize_str() {
        let color = "any_arbitrary_string";
        assert_eq!(to_value(color).unwrap(), json!("any_arbitrary_string"));
    }

    #[test]
    fn serialize_string() {
        let color = "any_arbitrary_string".to_string();
        assert_eq!(to_value(color).unwrap(), json!("any_arbitrary_string"));
    }

    #[test]
    fn serialize_numbers() {
        assert_eq!(to_value(1f64).unwrap(), json!(1f64));
        assert_eq!(to_value(1f32).unwrap(), json!(1f32));
        assert_eq!(to_value(1i64).unwrap(), json!(1i64));
        assert_eq!(to_value(1i32).unwrap(), json!(1i32));
        assert_eq!(to_value(1i16).unwrap(), json!(1i16));
        assert_eq!(to_value(1i8).unwrap(), json!(1i8));
        assert_eq!(to_value(1u64).unwrap(), json!(1u64));
        assert_eq!(to_value(1u32).unwrap(), json!(1u32));
        assert_eq!(to_value(1u16).unwrap(), json!(1u16));
        assert_eq!(to_value(1u8).unwrap(), json!(1u8));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_named_color() {
        assert_eq!(to_value(NamedColor::AliceBlue).unwrap(), json!("aliceblue"));
        assert_eq!(to_value(NamedColor::AntiqueWhite).unwrap(), json!("antiquewhite"));
        assert_eq!(to_value(NamedColor::Aqua).unwrap(), json!("aqua"));
        assert_eq!(to_value(NamedColor::Aquamarine).unwrap(), json!("aquamarine"));
        assert_eq!(to_value(NamedColor::Azure).unwrap(), json!("azure"));
        assert_eq!(to_value(NamedColor::Beige).unwrap(), json!("beige"));
        assert_eq!(to_value(NamedColor::Bisque).unwrap(), json!("bisque"));
        assert_eq!(to_value(NamedColor::Black).unwrap(), json!("black"));
        assert_eq!(to_value(NamedColor::BlanchedAlmond).unwrap(), json!("blanchedalmond"));
        assert_eq!(to_value(NamedColor::Blue).unwrap(), json!("blue"));
        assert_eq!(to_value(NamedColor::BlueViolet).unwrap(), json!("blueviolet"));
        assert_eq!(to_value(NamedColor::Brown).unwrap(), json!("brown"));
        assert_eq!(to_value(NamedColor::BurlyWood).unwrap(), json!("burlywood"));
        assert_eq!(to_value(NamedColor::CadetBlue).unwrap(), json!("cadetblue"));
        assert_eq!(to_value(NamedColor::Chartreuse).unwrap(), json!("chartreuse"));
        assert_eq!(to_value(NamedColor::Chocolate).unwrap(), json!("chocolate"));
        assert_eq!(to_value(NamedColor::Coral).unwrap(), json!("coral"));
        assert_eq!(to_value(NamedColor::CornflowerBlue).unwrap(), json!("cornflowerblue"));
        assert_eq!(to_value(NamedColor::CornSilk).unwrap(), json!("cornsilk"));
        assert_eq!(to_value(NamedColor::Crimson).unwrap(), json!("crimson"));
        assert_eq!(to_value(NamedColor::Cyan).unwrap(), json!("cyan"));
        assert_eq!(to_value(NamedColor::DarkBlue).unwrap(), json!("darkblue"));
        assert_eq!(to_value(NamedColor::DarkCyan).unwrap(), json!("darkcyan"));
        assert_eq!(to_value(NamedColor::DarkGoldenrod).unwrap(), json!("darkgoldenrod"));
        assert_eq!(to_value(NamedColor::DarkGray).unwrap(), json!("darkgray"));
        assert_eq!(to_value(NamedColor::DarkGrey).unwrap(), json!("darkgrey"));
        assert_eq!(to_value(NamedColor::DarkGreen).unwrap(), json!("darkgreen"));
        assert_eq!(to_value(NamedColor::DarkOrange).unwrap(), json!("darkorange"));
        assert_eq!(to_value(NamedColor::DarkOrchid).unwrap(), json!("darkorchid"));
        assert_eq!(to_value(NamedColor::DarkRed).unwrap(), json!("darkred"));
        assert_eq!(to_value(NamedColor::DarkSalmon).unwrap(), json!("darksalmon"));
        assert_eq!(to_value(NamedColor::DarkSeaGreen).unwrap(), json!("darkseagreen"));
        assert_eq!(to_value(NamedColor::DarkSlateBlue).unwrap(), json!("darkslateblue"));
        assert_eq!(to_value(NamedColor::DarkSlateGray).unwrap(), json!("darkslategray"));
        assert_eq!(to_value(NamedColor::DarkSlateGrey).unwrap(), json!("darkslategrey"));
        assert_eq!(to_value(NamedColor::DarkTurquoise).unwrap(), json!("darkturquoise"));
        assert_eq!(to_value(NamedColor::DarkViolet).unwrap(), json!("darkviolet"));
        assert_eq!(to_value(NamedColor::DeepPink).unwrap(), json!("deeppink"));
        assert_eq!(to_value(NamedColor::DeepSkyBlue).unwrap(), json!("deepskyblue"));
        assert_eq!(to_value(NamedColor::DimGray).unwrap(), json!("dimgray"));
        assert_eq!(to_value(NamedColor::DimGrey).unwrap(), json!("dimgrey"));
        assert_eq!(to_value(NamedColor::DodgerBlue).unwrap(), json!("dodgerblue"));
        assert_eq!(to_value(NamedColor::FireBrick).unwrap(), json!("firebrick"));
        assert_eq!(to_value(NamedColor::FloralWhite).unwrap(), json!("floralwhite"));
        assert_eq!(to_value(NamedColor::ForestGreen).unwrap(), json!("forestgreen"));
        assert_eq!(to_value(NamedColor::Fuchsia).unwrap(), json!("fuchsia"));
        assert_eq!(to_value(NamedColor::Gainsboro).unwrap(), json!("gainsboro"));
        assert_eq!(to_value(NamedColor::GhostWhite).unwrap(), json!("ghostwhite"));
        assert_eq!(to_value(NamedColor::Gold).unwrap(), json!("gold"));
        assert_eq!(to_value(NamedColor::Goldenrod).unwrap(), json!("goldenrod"));
        assert_eq!(to_value(NamedColor::Gray).unwrap(), json!("gray"));
        assert_eq!(to_value(NamedColor::Grey).unwrap(), json!("grey"));
        assert_eq!(to_value(NamedColor::Green).unwrap(), json!("green"));
        assert_eq!(to_value(NamedColor::GreenYellow).unwrap(), json!("greenyellow"));
        assert_eq!(to_value(NamedColor::Honeydew).unwrap(), json!("honeydew"));
        assert_eq!(to_value(NamedColor::HotPink).unwrap(), json!("hotpink"));
        assert_eq!(to_value(NamedColor::IndianRed).unwrap(), json!("indianred"));
        assert_eq!(to_value(NamedColor::Indigo).unwrap(), json!("indigo"));
        assert_eq!(to_value(NamedColor::Ivory).unwrap(), json!("ivory"));
        assert_eq!(to_value(NamedColor::Khaki).unwrap(), json!("khaki"));
        assert_eq!(to_value(NamedColor::Lavender).unwrap(), json!("lavender"));
        assert_eq!(to_value(NamedColor::LavenderBlush).unwrap(), json!("lavenderblush"));
        assert_eq!(to_value(NamedColor::LawnGreen).unwrap(), json!("lawngreen"));
        assert_eq!(to_value(NamedColor::LemonChiffon).unwrap(), json!("lemonchiffon"));
        assert_eq!(to_value(NamedColor::LightBlue).unwrap(), json!("lightblue"));
        assert_eq!(to_value(NamedColor::LightCoral).unwrap(), json!("lightcoral"));
        assert_eq!(to_value(NamedColor::LightCyan).unwrap(), json!("lightcyan"));
        assert_eq!(to_value(NamedColor::LightGoldenrodYellow).unwrap(), json!("lightgoldenrodyellow"));
        assert_eq!(to_value(NamedColor::LightGray).unwrap(), json!("lightgray"));
        assert_eq!(to_value(NamedColor::LightGrey).unwrap(), json!("lightgrey"));
        assert_eq!(to_value(NamedColor::LightGreen).unwrap(), json!("lightgreen"));
        assert_eq!(to_value(NamedColor::LightPink).unwrap(), json!("lightpink"));
        assert_eq!(to_value(NamedColor::LightSalmon).unwrap(), json!("lightsalmon"));
        assert_eq!(to_value(NamedColor::LightSeaGreen).unwrap(), json!("lightseagreen"));
        assert_eq!(to_value(NamedColor::LightSkyBlue).unwrap(), json!("lightskyblue"));
        assert_eq!(to_value(NamedColor::LightSlateGray).unwrap(), json!("lightslategray"));
        assert_eq!(to_value(NamedColor::LightSlateGrey).unwrap(), json!("lightslategrey"));
        assert_eq!(to_value(NamedColor::LightSteelBlue).unwrap(), json!("lightsteelblue"));
        assert_eq!(to_value(NamedColor::LightYellow).unwrap(), json!("lightyellow"));
        assert_eq!(to_value(NamedColor::Lime).unwrap(), json!("lime"));
        assert_eq!(to_value(NamedColor::LimeGreen).unwrap(), json!("limegreen"));
        assert_eq!(to_value(NamedColor::Linen).unwrap(), json!("linen"));
        assert_eq!(to_value(NamedColor::Magenta).unwrap(), json!("magenta"));
        assert_eq!(to_value(NamedColor::Maroon).unwrap(), json!("maroon"));
        assert_eq!(to_value(NamedColor::MediumAquamarine).unwrap(), json!("mediumaquamarine"));
        assert_eq!(to_value(NamedColor::MediumBlue).unwrap(), json!("mediumblue"));
        assert_eq!(to_value(NamedColor::MediumOrchid).unwrap(), json!("mediumorchid"));
        assert_eq!(to_value(NamedColor::MediumPurple).unwrap(), json!("mediumpurple"));
        assert_eq!(to_value(NamedColor::MediumSeaGreen).unwrap(), json!("mediumseagreen"));
        assert_eq!(to_value(NamedColor::MediumSlateBlue).unwrap(), json!("mediumslateblue"));
        assert_eq!(to_value(NamedColor::MediumSpringGreen).unwrap(), json!("mediumspringgreen"));
        assert_eq!(to_value(NamedColor::MediumTurquoise).unwrap(), json!("mediumturquoise"));
        assert_eq!(to_value(NamedColor::MediumVioletRed).unwrap(), json!("mediumvioletred"));
        assert_eq!(to_value(NamedColor::MidnightBlue).unwrap(), json!("midnightblue"));
        assert_eq!(to_value(NamedColor::MintCream).unwrap(), json!("mintcream"));
        assert_eq!(to_value(NamedColor::MistyRose).unwrap(), json!("mistyrose"));
        assert_eq!(to_value(NamedColor::Moccasin).unwrap(), json!("moccasin"));
        assert_eq!(to_value(NamedColor::NavajoWhite).unwrap(), json!("navajowhite"));
        assert_eq!(to_value(NamedColor::Navy).unwrap(), json!("navy"));
        assert_eq!(to_value(NamedColor::OldLace).unwrap(), json!("oldlace"));
        assert_eq!(to_value(NamedColor::Olive).unwrap(), json!("olive"));
        assert_eq!(to_value(NamedColor::OliveDrab).unwrap(), json!("olivedrab"));
        assert_eq!(to_value(NamedColor::Orange).unwrap(), json!("orange"));
        assert_eq!(to_value(NamedColor::OrangeRed).unwrap(), json!("orangered"));
        assert_eq!(to_value(NamedColor::Orchid).unwrap(), json!("orchid"));
        assert_eq!(to_value(NamedColor::PaleGoldenrod).unwrap(), json!("palegoldenrod"));
        assert_eq!(to_value(NamedColor::PaleGreen).unwrap(), json!("palegreen"));
        assert_eq!(to_value(NamedColor::PaleTurquoise).unwrap(), json!("paleturquoise"));
        assert_eq!(to_value(NamedColor::PaleVioletRed).unwrap(), json!("palevioletred"));
        assert_eq!(to_value(NamedColor::PapayaWhip).unwrap(), json!("papayawhip"));
        assert_eq!(to_value(NamedColor::PeachPuff).unwrap(), json!("peachpuff"));
        assert_eq!(to_value(NamedColor::Peru).unwrap(), json!("peru"));
        assert_eq!(to_value(NamedColor::Pink).unwrap(), json!("pink"));
        assert_eq!(to_value(NamedColor::Plum).unwrap(), json!("plum"));
        assert_eq!(to_value(NamedColor::PowderBlue).unwrap(), json!("powderblue"));
        assert_eq!(to_value(NamedColor::Purple).unwrap(), json!("purple"));
        assert_eq!(to_value(NamedColor::RebeccaPurple).unwrap(), json!("rebeccapurple"));
        assert_eq!(to_value(NamedColor::Red).unwrap(), json!("red"));
        assert_eq!(to_value(NamedColor::RosyBrown).unwrap(), json!("rosybrown"));
        assert_eq!(to_value(NamedColor::RoyalBlue).unwrap(), json!("royalblue"));
        assert_eq!(to_value(NamedColor::SaddleBrown).unwrap(), json!("saddlebrown"));
        assert_eq!(to_value(NamedColor::Salmon).unwrap(), json!("salmon"));
        assert_eq!(to_value(NamedColor::SandyBrown).unwrap(), json!("sandybrown"));
        assert_eq!(to_value(NamedColor::SeaGreen).unwrap(), json!("seagreen"));
        assert_eq!(to_value(NamedColor::Seashell).unwrap(), json!("seashell"));
        assert_eq!(to_value(NamedColor::Sienna).unwrap(), json!("sienna"));
        assert_eq!(to_value(NamedColor::Silver).unwrap(), json!("silver"));
        assert_eq!(to_value(NamedColor::SkyBlue).unwrap(), json!("skyblue"));
        assert_eq!(to_value(NamedColor::SlateBlue).unwrap(), json!("slateblue"));
        assert_eq!(to_value(NamedColor::SlateGray).unwrap(), json!("slategray"));
        assert_eq!(to_value(NamedColor::SlateGrey).unwrap(), json!("slategrey"));
        assert_eq!(to_value(NamedColor::Snow).unwrap(), json!("snow"));
        assert_eq!(to_value(NamedColor::SpringGreen).unwrap(), json!("springgreen"));
        assert_eq!(to_value(NamedColor::SteelBlue).unwrap(), json!("steelblue"));
        assert_eq!(to_value(NamedColor::Tan).unwrap(), json!("tan"));
        assert_eq!(to_value(NamedColor::Teal).unwrap(), json!("teal"));
        assert_eq!(to_value(NamedColor::Thistle).unwrap(), json!("thistle"));
        assert_eq!(to_value(NamedColor::Tomato).unwrap(), json!("tomato"));
        assert_eq!(to_value(NamedColor::Turquoise).unwrap(), json!("turquoise"));
        assert_eq!(to_value(NamedColor::Violet).unwrap(), json!("violet"));
        assert_eq!(to_value(NamedColor::Wheat).unwrap(), json!("wheat"));
        assert_eq!(to_value(NamedColor::White).unwrap(), json!("white"));
        assert_eq!(to_value(NamedColor::WhiteSmoke).unwrap(), json!("whitesmoke"));
        assert_eq!(to_value(NamedColor::Yellow).unwrap(), json!("yellow"));
        assert_eq!(to_value(NamedColor::YellowGreen).unwrap(), json!("yellowgreen"));
        assert_eq!(to_value(NamedColor::Transparent).unwrap(), json!("transparent"));
    }

    #[test]
    #[rustfmt::skip]
    fn deserialize_named_color() {
        assert_eq!(from_value::<NamedColor>(json!("aliceblue")).unwrap(), NamedColor::AliceBlue);
        assert_eq!(from_value::<NamedColor>(json!("antiquewhite")).unwrap(),NamedColor::AntiqueWhite);
        assert_eq!(from_value::<NamedColor>(json!("aqua")).unwrap(),NamedColor::Aqua);
        assert_eq!(from_value::<NamedColor>(json!("aquamarine")).unwrap(),NamedColor::Aquamarine);
        assert_eq!(from_value::<NamedColor>(json!("azure")).unwrap(),NamedColor::Azure);
        assert_eq!(from_value::<NamedColor>(json!("beige")).unwrap(),NamedColor::Beige);
        assert_eq!(from_value::<NamedColor>(json!("bisque")).unwrap(),NamedColor::Bisque);
        assert_eq!(from_value::<NamedColor>(json!("black")).unwrap(),NamedColor::Black);
        assert_eq!(from_value::<NamedColor>(json!("blanchedalmond")).unwrap(),NamedColor::BlanchedAlmond);
        assert_eq!(from_value::<NamedColor>(json!("blue")).unwrap(),NamedColor::Blue);
        assert_eq!(from_value::<NamedColor>(json!("blueviolet")).unwrap(),NamedColor::BlueViolet);
        assert_eq!(from_value::<NamedColor>(json!("brown")).unwrap(),NamedColor::Brown);
        assert_eq!(from_value::<NamedColor>(json!("burlywood")).unwrap(),NamedColor::BurlyWood);
        assert_eq!(from_value::<NamedColor>(json!("cadetblue")).unwrap(),NamedColor::CadetBlue);
        assert_eq!(from_value::<NamedColor>(json!("chartreuse")).unwrap(),NamedColor::Chartreuse);
        assert_eq!(from_value::<NamedColor>(json!("chocolate")).unwrap(),NamedColor::Chocolate);
        assert_eq!(from_value::<NamedColor>(json!("coral")).unwrap(),NamedColor::Coral);
        assert_eq!(from_value::<NamedColor>(json!("cornflowerblue")).unwrap(),NamedColor::CornflowerBlue);
        assert_eq!(from_value::<NamedColor>(json!("cornsilk")).unwrap(),NamedColor::CornSilk);
        assert_eq!(from_value::<NamedColor>(json!("crimson")).unwrap(),NamedColor::Crimson);
        assert_eq!(from_value::<NamedColor>(json!("cyan")).unwrap(),NamedColor::Cyan);
        assert_eq!(from_value::<NamedColor>(json!("darkblue")).unwrap(),NamedColor::DarkBlue);
        assert_eq!(from_value::<NamedColor>(json!("darkcyan")).unwrap(),NamedColor::DarkCyan);
        assert_eq!(from_value::<NamedColor>(json!("darkgoldenrod")).unwrap(),NamedColor::DarkGoldenrod);
        assert_eq!(from_value::<NamedColor>(json!("darkgray")).unwrap(),NamedColor::DarkGray);
        assert_eq!(from_value::<NamedColor>(json!("darkgrey")).unwrap(),NamedColor::DarkGrey);
        assert_eq!(from_value::<NamedColor>(json!("darkgreen")).unwrap(),NamedColor::DarkGreen);
        assert_eq!(from_value::<NamedColor>(json!("darkorange")).unwrap(),NamedColor::DarkOrange);
        assert_eq!(from_value::<NamedColor>(json!("darkorchid")).unwrap(),NamedColor::DarkOrchid);
        assert_eq!(from_value::<NamedColor>(json!("darkred")).unwrap(),NamedColor::DarkRed);
        assert_eq!(from_value::<NamedColor>(json!("darksalmon")).unwrap(),NamedColor::DarkSalmon);
        assert_eq!(from_value::<NamedColor>(json!("darkseagreen")).unwrap(),NamedColor::DarkSeaGreen);
        assert_eq!(from_value::<NamedColor>(json!("darkslateblue")).unwrap(),NamedColor::DarkSlateBlue);
        assert_eq!(from_value::<NamedColor>(json!("darkslategray")).unwrap(),NamedColor::DarkSlateGray);
        assert_eq!(from_value::<NamedColor>(json!("darkslategrey")).unwrap(),NamedColor::DarkSlateGrey);
        assert_eq!(from_value::<NamedColor>(json!("darkturquoise")).unwrap(),NamedColor::DarkTurquoise);
        assert_eq!(from_value::<NamedColor>(json!("darkviolet")).unwrap(),NamedColor::DarkViolet);
        assert_eq!(from_value::<NamedColor>(json!("deeppink")).unwrap(),NamedColor::DeepPink);
        assert_eq!(from_value::<NamedColor>(json!("deepskyblue")).unwrap(),NamedColor::DeepSkyBlue);
        assert_eq!(from_value::<NamedColor>(json!("dimgray")).unwrap(),NamedColor::DimGray);
        assert_eq!(from_value::<NamedColor>(json!("dimgrey")).unwrap(),NamedColor::DimGrey);
        assert_eq!(from_value::<NamedColor>(json!("dodgerblue")).unwrap(),NamedColor::DodgerBlue);
        assert_eq!(from_value::<NamedColor>(json!("firebrick")).unwrap(),NamedColor::FireBrick);
        assert_eq!(from_value::<NamedColor>(json!("floralwhite")).unwrap(),NamedColor::FloralWhite);
        assert_eq!(from_value::<NamedColor>(json!("forestgreen")).unwrap(),NamedColor::ForestGreen);
        assert_eq!(from_value::<NamedColor>(json!("fuchsia")).unwrap(),NamedColor::Fuchsia);
        assert_eq!(from_value::<NamedColor>(json!("gainsboro")).unwrap(),NamedColor::Gainsboro);
        assert_eq!(from_value::<NamedColor>(json!("ghostwhite")).unwrap(),NamedColor::GhostWhite);
        assert_eq!(from_value::<NamedColor>(json!("gold")).unwrap(),NamedColor::Gold);
        assert_eq!(from_value::<NamedColor>(json!("goldenrod")).unwrap(),NamedColor::Goldenrod);
        assert_eq!(from_value::<NamedColor>(json!("gray")).unwrap(),NamedColor::Gray);
        assert_eq!(from_value::<NamedColor>(json!("grey")).unwrap(),NamedColor::Grey);
        assert_eq!(from_value::<NamedColor>(json!("green")).unwrap(),NamedColor::Green);
        assert_eq!(from_value::<NamedColor>(json!("greenyellow")).unwrap(),NamedColor::GreenYellow);
        assert_eq!(from_value::<NamedColor>(json!("honeydew")).unwrap(),NamedColor::Honeydew);
        assert_eq!(from_value::<NamedColor>(json!("hotpink")).unwrap(),NamedColor::HotPink);
        assert_eq!(from_value::<NamedColor>(json!("indianred")).unwrap(),NamedColor::IndianRed);
        assert_eq!(from_value::<NamedColor>(json!("indigo")).unwrap(),NamedColor::Indigo);
        assert_eq!(from_value::<NamedColor>(json!("ivory")).unwrap(),NamedColor::Ivory);
        assert_eq!(from_value::<NamedColor>(json!("khaki")).unwrap(),NamedColor::Khaki);
        assert_eq!(from_value::<NamedColor>(json!("lavender")).unwrap(),NamedColor::Lavender);
        assert_eq!(from_value::<NamedColor>(json!("lavenderblush")).unwrap(),NamedColor::LavenderBlush);
        assert_eq!(from_value::<NamedColor>(json!("lawngreen")).unwrap(),NamedColor::LawnGreen);
        assert_eq!(from_value::<NamedColor>(json!("lemonchiffon")).unwrap(),NamedColor::LemonChiffon);
        assert_eq!(from_value::<NamedColor>(json!("lightblue")).unwrap(),NamedColor::LightBlue);
        assert_eq!(from_value::<NamedColor>(json!("lightcoral")).unwrap(),NamedColor::LightCoral);
        assert_eq!(from_value::<NamedColor>(json!("lightcyan")).unwrap(),NamedColor::LightCyan);
        assert_eq!(from_value::<NamedColor>(json!("lightgoldenrodyellow")).unwrap(),NamedColor::LightGoldenrodYellow);
        assert_eq!(from_value::<NamedColor>(json!("lightgray")).unwrap(),NamedColor::LightGray);
        assert_eq!(from_value::<NamedColor>(json!("lightgrey")).unwrap(),NamedColor::LightGrey);
        assert_eq!(from_value::<NamedColor>(json!("lightgreen")).unwrap(),NamedColor::LightGreen);
        assert_eq!(from_value::<NamedColor>(json!("lightpink")).unwrap(),NamedColor::LightPink);
        assert_eq!(from_value::<NamedColor>(json!("lightsalmon")).unwrap(),NamedColor::LightSalmon);
        assert_eq!(from_value::<NamedColor>(json!("lightseagreen")).unwrap(),NamedColor::LightSeaGreen);
        assert_eq!(from_value::<NamedColor>(json!("lightskyblue")).unwrap(),NamedColor::LightSkyBlue);
        assert_eq!(from_value::<NamedColor>(json!("lightslategray")).unwrap(),NamedColor::LightSlateGray);
        assert_eq!(from_value::<NamedColor>(json!("lightslategrey")).unwrap(),NamedColor::LightSlateGrey);
        assert_eq!(from_value::<NamedColor>(json!("lightsteelblue")).unwrap(),NamedColor::LightSteelBlue);
        assert_eq!(from_value::<NamedColor>(json!("lightyellow")).unwrap(),NamedColor::LightYellow);
        assert_eq!(from_value::<NamedColor>(json!("lime")).unwrap(),NamedColor::Lime);
        assert_eq!(from_value::<NamedColor>(json!("limegreen")).unwrap(),NamedColor::LimeGreen);
        assert_eq!(from_value::<NamedColor>(json!("linen")).unwrap(),NamedColor::Linen);
        assert_eq!(from_value::<NamedColor>(json!("magenta")).unwrap(),NamedColor::Magenta);
        assert_eq!(from_value::<NamedColor>(json!("maroon")).unwrap(),NamedColor::Maroon);
        assert_eq!(from_value::<NamedColor>(json!("mediumaquamarine")).unwrap(),NamedColor::MediumAquamarine);
        assert_eq!(from_value::<NamedColor>(json!("mediumblue")).unwrap(),NamedColor::MediumBlue);
        assert_eq!(from_value::<NamedColor>(json!("mediumorchid")).unwrap(),NamedColor::MediumOrchid);
        assert_eq!(from_value::<NamedColor>(json!("mediumpurple")).unwrap(),NamedColor::MediumPurple);
        assert_eq!(from_value::<NamedColor>(json!("mediumseagreen")).unwrap(),NamedColor::MediumSeaGreen);
        assert_eq!(from_value::<NamedColor>(json!("mediumslateblue")).unwrap(),NamedColor::MediumSlateBlue);
        assert_eq!(from_value::<NamedColor>(json!("mediumspringgreen")).unwrap(),NamedColor::MediumSpringGreen);
        assert_eq!(from_value::<NamedColor>(json!("mediumturquoise")).unwrap(),NamedColor::MediumTurquoise);
        assert_eq!(from_value::<NamedColor>(json!("mediumvioletred")).unwrap(),NamedColor::MediumVioletRed);
        assert_eq!(from_value::<NamedColor>(json!("midnightblue")).unwrap(),NamedColor::MidnightBlue);
        assert_eq!(from_value::<NamedColor>(json!("mintcream")).unwrap(),NamedColor::MintCream);
        assert_eq!(from_value::<NamedColor>(json!("mistyrose")).unwrap(),NamedColor::MistyRose);
        assert_eq!(from_value::<NamedColor>(json!("moccasin")).unwrap(),NamedColor::Moccasin);
        assert_eq!(from_value::<NamedColor>(json!("navajowhite")).unwrap(),NamedColor::NavajoWhite);
        assert_eq!(from_value::<NamedColor>(json!("navy")).unwrap(),NamedColor::Navy);
        assert_eq!(from_value::<NamedColor>(json!("oldlace")).unwrap(),NamedColor::OldLace);
        assert_eq!(from_value::<NamedColor>(json!("olive")).unwrap(),NamedColor::Olive);
        assert_eq!(from_value::<NamedColor>(json!("olivedrab")).unwrap(),NamedColor::OliveDrab);
        assert_eq!(from_value::<NamedColor>(json!("orange")).unwrap(),NamedColor::Orange);
        assert_eq!(from_value::<NamedColor>(json!("orangered")).unwrap(),NamedColor::OrangeRed);
        assert_eq!(from_value::<NamedColor>(json!("orchid")).unwrap(),NamedColor::Orchid);
        assert_eq!(from_value::<NamedColor>(json!("palegoldenrod")).unwrap(),NamedColor::PaleGoldenrod);
        assert_eq!(from_value::<NamedColor>(json!("palegreen")).unwrap(),NamedColor::PaleGreen);
        assert_eq!(from_value::<NamedColor>(json!("paleturquoise")).unwrap(),NamedColor::PaleTurquoise);
        assert_eq!(from_value::<NamedColor>(json!("palevioletred")).unwrap(),NamedColor::PaleVioletRed);
        assert_eq!(from_value::<NamedColor>(json!("papayawhip")).unwrap(),NamedColor::PapayaWhip);
        assert_eq!(from_value::<NamedColor>(json!("peachpuff")).unwrap(),NamedColor::PeachPuff);
        assert_eq!(from_value::<NamedColor>(json!("peru")).unwrap(),NamedColor::Peru);
        assert_eq!(from_value::<NamedColor>(json!("pink")).unwrap(),NamedColor::Pink);
        assert_eq!(from_value::<NamedColor>(json!("plum")).unwrap(),NamedColor::Plum);
        assert_eq!(from_value::<NamedColor>(json!("powderblue")).unwrap(),NamedColor::PowderBlue);
        assert_eq!(from_value::<NamedColor>(json!("purple")).unwrap(),NamedColor::Purple);
        assert_eq!(from_value::<NamedColor>(json!("rebeccapurple")).unwrap(),NamedColor::RebeccaPurple);
        assert_eq!(from_value::<NamedColor>(json!("red")).unwrap(),NamedColor::Red);
        assert_eq!(from_value::<NamedColor>(json!("rosybrown")).unwrap(),NamedColor::RosyBrown);
        assert_eq!(from_value::<NamedColor>(json!("royalblue")).unwrap(),NamedColor::RoyalBlue);
        assert_eq!(from_value::<NamedColor>(json!("saddlebrown")).unwrap(),NamedColor::SaddleBrown);
        assert_eq!(from_value::<NamedColor>(json!("salmon")).unwrap(),NamedColor::Salmon);
        assert_eq!(from_value::<NamedColor>(json!("sandybrown")).unwrap(),NamedColor::SandyBrown);
        assert_eq!(from_value::<NamedColor>(json!("seagreen")).unwrap(),NamedColor::SeaGreen);
        assert_eq!(from_value::<NamedColor>(json!("seashell")).unwrap(),NamedColor::Seashell);
        assert_eq!(from_value::<NamedColor>(json!("sienna")).unwrap(),NamedColor::Sienna);
        assert_eq!(from_value::<NamedColor>(json!("silver")).unwrap(),NamedColor::Silver);
        assert_eq!(from_value::<NamedColor>(json!("skyblue")).unwrap(),NamedColor::SkyBlue);
        assert_eq!(from_value::<NamedColor>(json!("slateblue")).unwrap(),NamedColor::SlateBlue);
        assert_eq!(from_value::<NamedColor>(json!("slategray")).unwrap(),NamedColor::SlateGray);
        assert_eq!(from_value::<NamedColor>(json!("slategrey")).unwrap(),NamedColor::SlateGrey);
        assert_eq!(from_value::<NamedColor>(json!("snow")).unwrap(),NamedColor::Snow);
        assert_eq!(from_value::<NamedColor>(json!("springgreen")).unwrap(),NamedColor::SpringGreen);
        assert_eq!(from_value::<NamedColor>(json!("steelblue")).unwrap(),NamedColor::SteelBlue);
        assert_eq!(from_value::<NamedColor>(json!("tan")).unwrap(),NamedColor::Tan);
        assert_eq!(from_value::<NamedColor>(json!("teal")).unwrap(),NamedColor::Teal);
        assert_eq!(from_value::<NamedColor>(json!("thistle")).unwrap(),NamedColor::Thistle);
        assert_eq!(from_value::<NamedColor>(json!("tomato")).unwrap(),NamedColor::Tomato);
        assert_eq!(from_value::<NamedColor>(json!("turquoise")).unwrap(),NamedColor::Turquoise);
        assert_eq!(from_value::<NamedColor>(json!("violet")).unwrap(),NamedColor::Violet);
        assert_eq!(from_value::<NamedColor>(json!("wheat")).unwrap(),NamedColor::Wheat);
        assert_eq!(from_value::<NamedColor>(json!("white")).unwrap(),NamedColor::White);
        assert_eq!(from_value::<NamedColor>(json!("whitesmoke")).unwrap(),NamedColor::WhiteSmoke);
        assert_eq!(from_value::<NamedColor>(json!("yellow")).unwrap(),NamedColor::Yellow);
        assert_eq!(from_value::<NamedColor>(json!("yellowgreen")).unwrap(),NamedColor::YellowGreen);
        assert_eq!(from_value::<NamedColor>(json!("transparent")).unwrap(),NamedColor::Transparent);
    }
}
