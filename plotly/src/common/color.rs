//! This module provides several user interfaces for describing a color to be used throughout the rest of the library.
//! The easiest way of describing a colour is to use a `&str` or `String`, which is simply serialized as-is and
//! passed on to the underlying `plotly.js` library. `plotly.js` supports [`CSS color formats`], and will fallback
//! to some default color if the color string is malformed.
//!
//! For a more type-safe approach, the `RGB` or `RGBA` structs can be used to construct a valid color, which will then
//! get serialized to an appropriate string representation. Cross-browser compatible [`predefined colors`] are
//! supported via the `NamedColor` enum.
//!
//! The `Color` trait is public, and so can be implemented for custom colour types. The user can then implement
//! a valid serialization function according to their own requirements. On the whole, that should be largely
//! unnecessary given the functionality already provided within this module.
//!
//! [`CSS color formats`]: https://www.w3schools.com/cssref/css_colors_legal.asp
//! [`predefined colors`]: https://www.w3schools.com/cssref/css_colors.asp

use dyn_clone::DynClone;
use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;

/// A marker trait allowing several ways to describe a color.
pub trait Color: DynClone + ErasedSerialize + Send + Sync + std::fmt::Debug + 'static {}

dyn_clone::clone_trait_object!(Color);
erased_serde::serialize_trait_object!(Color);

impl Color for NamedColor {}
impl Color for &'static str {}
impl Color for String {}
impl Color for Rgb {}
impl Color for Rgba {}

/// ColorArray is only used internally to provide a helper method for converting Vec<impl Color2> to
/// Vec<Box<dyn Color2>>, as we would otherwise fall foul of the orphan rules.
pub(crate) struct ColorArray<C: Color>(pub(crate) Vec<C>);

impl<C: Color> Into<Vec<Box<dyn Color>>> for ColorArray<C> {
    fn into(self) -> Vec<Box<dyn Color>> {
        self.0
            .into_iter()
            .map(|c| Box::new(c) as Box<dyn Color>)
            .collect()
    }
}

/// A type-safe way of constructing a valid RGB color from constituent R, G and B channels.
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
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

/// A type-safe way of constructing a valid RGBA color from constituent R, G, B and A channels.
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: f64,
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

/// Cross-browser compatible [`predefined colors`].
///
/// [`predefined colors`]: https://www.w3schools.com/cssref/css_colors.asp
#[derive(Debug, Clone, Copy, Serialize)]
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
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_rgb() {
        let rgb = Rgb::new(80, 90, 100);
        assert_eq!(to_value(rgb).unwrap(), json!("rgb(80, 90, 100)"));
    }

    #[test]
    fn test_serialize_rgba() {
        let rgb = Rgba::new(80, 90, 100, 0.2);
        assert_eq!(to_value(rgb).unwrap(), json!("rgba(80, 90, 100, 0.2)"));
    }

    #[test]
    fn test_serialize_str() {
        let color = "any_arbitrary_string";
        assert_eq!(to_value(color).unwrap(), json!("any_arbitrary_string"));
    }

    #[test]
    fn test_serialize_string() {
        let color = "any_arbitrary_string".to_string();
        assert_eq!(to_value(color).unwrap(), json!("any_arbitrary_string"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_named_color() {
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
}
