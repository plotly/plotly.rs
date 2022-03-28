use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ColorWrapper {
    S(String),
    F(f64),
}

impl PartialEq for ColorWrapper {
    fn eq(&self, other: &Self) -> bool {
        let lhs = match self {
            ColorWrapper::S(v) => v.to_owned(),
            ColorWrapper::F(v) => format!("{}", v),
        };
        let rhs = match other {
            ColorWrapper::S(v) => v.to_owned(),
            ColorWrapper::F(v) => format!("{}", v),
        };
        lhs == rhs
    }
}

pub trait Color {
    fn to_color(&self) -> ColorWrapper;
}

#[derive(Debug)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
}

impl Color for Rgb {
    fn to_color(&self) -> ColorWrapper {
        ColorWrapper::S(format!("rgb({}, {}, {})", self.r, self.g, self.b))
    }
}

#[derive(Debug)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: f64,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> Rgba {
        Rgba { r, g, b, a }
    }
}

impl Color for Rgba {
    fn to_color(&self) -> ColorWrapper {
        ColorWrapper::S(format!(
            "rgba({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        ))
    }
}

// https://www.w3.org/TR/css-color-3/#svg-color
#[derive(Debug, Clone, Copy)]
pub enum NamedColor {
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
}

impl Color for NamedColor {
    fn to_color(&self) -> ColorWrapper {
        match self {
            NamedColor::AliceBlue => ColorWrapper::S("aliceblue".to_owned()),
            NamedColor::AntiqueWhite => ColorWrapper::S("antiquewhite".to_owned()),
            NamedColor::Aqua => ColorWrapper::S("aqua".to_owned()),
            NamedColor::Aquamarine => ColorWrapper::S("aquamarine".to_owned()),
            NamedColor::Azure => ColorWrapper::S("azure".to_owned()),
            NamedColor::Beige => ColorWrapper::S("beige".to_owned()),
            NamedColor::Bisque => ColorWrapper::S("bisque".to_owned()),
            NamedColor::Black => ColorWrapper::S("black".to_owned()),
            NamedColor::BlancheDalmond => ColorWrapper::S("blanchedalmond".to_owned()),
            NamedColor::Blue => ColorWrapper::S("blue".to_owned()),
            NamedColor::BlueViolet => ColorWrapper::S("blueviolet".to_owned()),
            NamedColor::Brown => ColorWrapper::S("brown".to_owned()),
            NamedColor::BurlyWood => ColorWrapper::S("burlywood".to_owned()),
            NamedColor::CadetBlue => ColorWrapper::S("cadetblue".to_owned()),
            NamedColor::Chartreuse => ColorWrapper::S("chartreuse".to_owned()),
            NamedColor::Chocolate => ColorWrapper::S("chocolate".to_owned()),
            NamedColor::Coral => ColorWrapper::S("coral".to_owned()),
            NamedColor::CornFlowerBlue => ColorWrapper::S("cornflowerblue".to_owned()),
            NamedColor::CornSilk => ColorWrapper::S("cornsilk".to_owned()),
            NamedColor::Crimson => ColorWrapper::S("crimson".to_owned()),
            NamedColor::Cyan => ColorWrapper::S("cyan".to_owned()),
            NamedColor::DarkBlue => ColorWrapper::S("darkblue".to_owned()),
            NamedColor::DarkCyan => ColorWrapper::S("darkcyan".to_owned()),
            NamedColor::DarkGoldenrod => ColorWrapper::S("darkgoldenrod".to_owned()),
            NamedColor::DarkGray => ColorWrapper::S("darkgray".to_owned()),
            NamedColor::DarkGreen => ColorWrapper::S("darkgreen".to_owned()),
            NamedColor::DarkGrey => ColorWrapper::S("darkgrey".to_owned()),
            NamedColor::DarkKhaki => ColorWrapper::S("darkkhaki".to_owned()),
            NamedColor::DarkMagenta => ColorWrapper::S("darkmagenta".to_owned()),
            NamedColor::DarkOliveGreen => ColorWrapper::S("darkolivegreen".to_owned()),
            NamedColor::DarkOrange => ColorWrapper::S("darkorange".to_owned()),
            NamedColor::DarkOrchid => ColorWrapper::S("darkorchid".to_owned()),
            NamedColor::DarkRed => ColorWrapper::S("darkred".to_owned()),
            NamedColor::DarkSalmon => ColorWrapper::S("darksalmon".to_owned()),
            NamedColor::DarkSeaGreen => ColorWrapper::S("darkseagreen".to_owned()),
            NamedColor::DarkSlateBlue => ColorWrapper::S("darkslateblue".to_owned()),
            NamedColor::DarkSlateGray => ColorWrapper::S("darkslategray".to_owned()),
            NamedColor::DarkSlateGrey => ColorWrapper::S("darkslategrey".to_owned()),
            NamedColor::DarkTurquoise => ColorWrapper::S("darkturquoise".to_owned()),
            NamedColor::DarkViolet => ColorWrapper::S("darkviolet".to_owned()),
            NamedColor::DeepPink => ColorWrapper::S("deeppink".to_owned()),
            NamedColor::DeepSkyBlue => ColorWrapper::S("deepskyblue".to_owned()),
            NamedColor::DimGray => ColorWrapper::S("dimgray".to_owned()),
            NamedColor::DimGrey => ColorWrapper::S("dimgrey".to_owned()),
            NamedColor::DodgerBlue => ColorWrapper::S("dodgerblue".to_owned()),
            NamedColor::FireBrick => ColorWrapper::S("firebrick".to_owned()),
            NamedColor::FloralWhite => ColorWrapper::S("floralwhite".to_owned()),
            NamedColor::ForestGreen => ColorWrapper::S("forestgreen".to_owned()),
            NamedColor::Fuchsia => ColorWrapper::S("fuchsia".to_owned()),
            NamedColor::Gainsboro => ColorWrapper::S("gainsboro".to_owned()),
            NamedColor::GhostWhite => ColorWrapper::S("ghostwhite".to_owned()),
            NamedColor::Gold => ColorWrapper::S("gold".to_owned()),
            NamedColor::Goldenrod => ColorWrapper::S("goldenrod".to_owned()),
            NamedColor::Gray => ColorWrapper::S("gray".to_owned()),
            NamedColor::Green => ColorWrapper::S("green".to_owned()),
            NamedColor::GreenYellow => ColorWrapper::S("greenyellow".to_owned()),
            NamedColor::Grey => ColorWrapper::S("grey".to_owned()),
            NamedColor::Honeydew => ColorWrapper::S("honeydew".to_owned()),
            NamedColor::HotPink => ColorWrapper::S("hotpink".to_owned()),
            NamedColor::IndianRed => ColorWrapper::S("indianred".to_owned()),
            NamedColor::Indigo => ColorWrapper::S("indigo".to_owned()),
            NamedColor::Ivory => ColorWrapper::S("ivory".to_owned()),
            NamedColor::Khaki => ColorWrapper::S("khaki".to_owned()),
            NamedColor::Lavender => ColorWrapper::S("lavender".to_owned()),
            NamedColor::LavenderBlush => ColorWrapper::S("lavenderblush".to_owned()),
            NamedColor::LawnGreen => ColorWrapper::S("lawngreen".to_owned()),
            NamedColor::LemonChiffon => ColorWrapper::S("lemonchiffon".to_owned()),
            NamedColor::LightBlue => ColorWrapper::S("lightblue".to_owned()),
            NamedColor::LightCoral => ColorWrapper::S("lightcoral".to_owned()),
            NamedColor::LightCyan => ColorWrapper::S("lightcyan".to_owned()),
            NamedColor::LightGoldenrodYellow => ColorWrapper::S("lightgoldenrodyellow".to_owned()),
            NamedColor::LightGray => ColorWrapper::S("lightgray".to_owned()),
            NamedColor::LightGreen => ColorWrapper::S("lightgreen".to_owned()),
            NamedColor::LightGrey => ColorWrapper::S("lightgrey".to_owned()),
            NamedColor::LightPink => ColorWrapper::S("lightpink".to_owned()),
            NamedColor::LightSalmon => ColorWrapper::S("lightsalmon".to_owned()),
            NamedColor::LightSeaGreen => ColorWrapper::S("lightseagreen".to_owned()),
            NamedColor::LightSkyBlue => ColorWrapper::S("lightskyblue".to_owned()),
            NamedColor::LightSlateGray => ColorWrapper::S("lightslategray".to_owned()),
            NamedColor::LightSlateGrey => ColorWrapper::S("lightslategrey".to_owned()),
            NamedColor::LightSteelBlue => ColorWrapper::S("lightsteelblue".to_owned()),
            NamedColor::LightYellow => ColorWrapper::S("lightyellow".to_owned()),
            NamedColor::Lime => ColorWrapper::S("lime".to_owned()),
            NamedColor::LimeGreen => ColorWrapper::S("limegreen".to_owned()),
            NamedColor::Linen => ColorWrapper::S("linen".to_owned()),
            NamedColor::Magenta => ColorWrapper::S("magenta".to_owned()),
            NamedColor::Maroon => ColorWrapper::S("maroon".to_owned()),
            NamedColor::MediumAquamarine => ColorWrapper::S("mediumaquamarine".to_owned()),
            NamedColor::MediumBlue => ColorWrapper::S("mediumblue".to_owned()),
            NamedColor::MediumOrchid => ColorWrapper::S("mediumorchid".to_owned()),
            NamedColor::MediumPurple => ColorWrapper::S("mediumpurple".to_owned()),
            NamedColor::MediumSeaGreen => ColorWrapper::S("mediumseagreen".to_owned()),
            NamedColor::MediumSlateBlue => ColorWrapper::S("mediumslateblue".to_owned()),
            NamedColor::MediumSpringGreen => ColorWrapper::S("mediumspringgreen".to_owned()),
            NamedColor::MediumTurquoise => ColorWrapper::S("mediumturquoise".to_owned()),
            NamedColor::MediumVioletRed => ColorWrapper::S("mediumvioletred".to_owned()),
            NamedColor::MidnightBlue => ColorWrapper::S("midnightblue".to_owned()),
            NamedColor::MintCream => ColorWrapper::S("mintcream".to_owned()),
            NamedColor::MistyRose => ColorWrapper::S("mistyrose".to_owned()),
            NamedColor::Moccasin => ColorWrapper::S("moccasin".to_owned()),
            NamedColor::NavajoWhite => ColorWrapper::S("navajowhite".to_owned()),
            NamedColor::Navy => ColorWrapper::S("navy".to_owned()),
            NamedColor::OldLace => ColorWrapper::S("oldlace".to_owned()),
            NamedColor::Olive => ColorWrapper::S("olive".to_owned()),
            NamedColor::OliveDrab => ColorWrapper::S("olivedrab".to_owned()),
            NamedColor::Orange => ColorWrapper::S("orange".to_owned()),
            NamedColor::OrangeRed => ColorWrapper::S("orangered".to_owned()),
            NamedColor::Orchid => ColorWrapper::S("orchid".to_owned()),
            NamedColor::PaleGoldenrod => ColorWrapper::S("palegoldenrod".to_owned()),
            NamedColor::PaleGreen => ColorWrapper::S("palegreen".to_owned()),
            NamedColor::PaleTurquoise => ColorWrapper::S("paleturquoise".to_owned()),
            NamedColor::PaleVioletRed => ColorWrapper::S("palevioletred".to_owned()),
            NamedColor::PapayaWhip => ColorWrapper::S("papayawhip".to_owned()),
            NamedColor::PeachPuff => ColorWrapper::S("peachpuff".to_owned()),
            NamedColor::Peru => ColorWrapper::S("peru".to_owned()),
            NamedColor::Pink => ColorWrapper::S("pink".to_owned()),
            NamedColor::Plum => ColorWrapper::S("plum".to_owned()),
            NamedColor::PowderBlue => ColorWrapper::S("powderblue".to_owned()),
            NamedColor::Purple => ColorWrapper::S("purple".to_owned()),
            NamedColor::Red => ColorWrapper::S("red".to_owned()),
            NamedColor::RosyBrown => ColorWrapper::S("rosybrown".to_owned()),
            NamedColor::RoyalBlue => ColorWrapper::S("royalblue".to_owned()),
            NamedColor::SaddleBrown => ColorWrapper::S("saddlebrown".to_owned()),
            NamedColor::Salmon => ColorWrapper::S("salmon".to_owned()),
            NamedColor::SandyBrown => ColorWrapper::S("sandybrown".to_owned()),
            NamedColor::SeaGreen => ColorWrapper::S("seagreen".to_owned()),
            NamedColor::Seashell => ColorWrapper::S("seashell".to_owned()),
            NamedColor::Sienna => ColorWrapper::S("sienna".to_owned()),
            NamedColor::Silver => ColorWrapper::S("silver".to_owned()),
            NamedColor::SkyBlue => ColorWrapper::S("skyblue".to_owned()),
            NamedColor::SlateBlue => ColorWrapper::S("slateblue".to_owned()),
            NamedColor::SlateGray => ColorWrapper::S("slategray".to_owned()),
            NamedColor::SlateGrey => ColorWrapper::S("slategrey".to_owned()),
            NamedColor::Snow => ColorWrapper::S("snow".to_owned()),
            NamedColor::SpringGreen => ColorWrapper::S("springgreen".to_owned()),
            NamedColor::SteelBlue => ColorWrapper::S("steelblue".to_owned()),
            NamedColor::Tan => ColorWrapper::S("tan".to_owned()),
            NamedColor::Teal => ColorWrapper::S("teal".to_owned()),
            NamedColor::Thistle => ColorWrapper::S("thistle".to_owned()),
            NamedColor::Tomato => ColorWrapper::S("tomato".to_owned()),
            NamedColor::Turquoise => ColorWrapper::S("turquoise".to_owned()),
            NamedColor::Violet => ColorWrapper::S("violet".to_owned()),
            NamedColor::Wheat => ColorWrapper::S("wheat".to_owned()),
            NamedColor::White => ColorWrapper::S("white".to_owned()),
            NamedColor::WhiteSmoke => ColorWrapper::S("whitesmoke".to_owned()),
            NamedColor::Yellow => ColorWrapper::S("yellow".to_owned()),
            NamedColor::YellowGreen => ColorWrapper::S("yellowgreen".to_owned()),
            NamedColor::Transparent => ColorWrapper::S("transparent".to_owned()),
        }
    }
}

impl Color for f64 {
    fn to_color(&self) -> ColorWrapper {
        ColorWrapper::F(*self)
    }
}

fn string_types_to_color_wrapper<S: AsRef<str> + std::fmt::Display + Sized>(v: S) -> ColorWrapper {
    if v.as_ref().len() < 6 || v.as_ref().len() > 7 {
        panic!("{} is not a valid hex color!", v);
    }
    if v.as_ref().len() == 6 && v.as_ref().starts_with('#') {
        panic!("{} is not a valid hex color!", v);
    }
    if v.as_ref().len() == 7 && !v.as_ref().starts_with('#') {
        panic!("{} is not a valid hex color!", v);
    }
    let valid_characters = "#ABCDEF0123456789";
    let mut s = v.as_ref().to_uppercase();
    if s.len() == 6 {
        s = format!("#{}", s);
    }
    for c in s.chars() {
        if !valid_characters.contains(c) {
            panic!("{} is not a valid hex color!", v);
        }
    }

    ColorWrapper::S(s)
}

// Implementations split intentionally; otherwise there is a conflict with the f64 implementation
// as it `may` implement AsRef<str> in the future.
impl Color for str {
    fn to_color(&self) -> ColorWrapper {
        string_types_to_color_wrapper(self)
    }
}

// Implementations split intentionally; otherwise there is a conflict with the f64 implementation
// as it `may` implement AsRef<str> in the future.
impl Color for &str {
    fn to_color(&self) -> ColorWrapper {
        string_types_to_color_wrapper(self)
    }
}

// Implementations split intentionally; otherwise there is a conflict with the f64 implementation
// as it `may` implement AsRef<str> in the future.
impl Color for String {
    fn to_color(&self) -> ColorWrapper {
        string_types_to_color_wrapper(self)
    }
}

// Implementations split intentionally; otherwise there is a conflict with the f64 implementation
// as it `may` implement AsRef<str> in the future.
impl Color for &String {
    fn to_color(&self) -> ColorWrapper {
        string_types_to_color_wrapper(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_color_normalization_str() {
        assert_eq!("#aabbcc".to_color(), ColorWrapper::S("#AABBCC".to_owned()));
        assert_eq!("aabbcc".to_color(), ColorWrapper::S("#AABBCC".to_owned()));
        assert_eq!("aaBBcc".to_color(), ColorWrapper::S("#AABBCC".to_owned()));
        assert_eq!("FABCDe".to_color(), ColorWrapper::S("#FABCDE".to_owned()));
        assert_eq!("123456".to_color(), ColorWrapper::S("#123456".to_owned()));
        assert_eq!("7890EE".to_color(), ColorWrapper::S("#7890EE".to_owned()));
    }

    #[test]
    fn hex_color_normalization_string() {
        assert_eq!(
            String::from("#aabbcc").to_color(),
            ColorWrapper::S("#AABBCC".to_owned())
        );
        assert_eq!(
            String::from("aabbcc").to_color(),
            ColorWrapper::S("#AABBCC".to_owned())
        );
        assert_eq!(
            String::from("aaBBcc").to_color(),
            ColorWrapper::S("#AABBCC".to_owned())
        );
        assert_eq!(
            String::from("FABCDe").to_color(),
            ColorWrapper::S("#FABCDE".to_owned())
        );
        assert_eq!(
            String::from("123456").to_color(),
            ColorWrapper::S("#123456".to_owned())
        );
        assert_eq!(
            String::from("7890EE").to_color(),
            ColorWrapper::S("#7890EE".to_owned())
        );
    }

    fn color_from_f64() {
        assert_eq!(1.54.to_color(), ColorWrapper::F(1.54));
        assert_eq!(0.1.to_color(), ColorWrapper::F(0.1));
    }

    #[test]
    #[should_panic]
    fn too_short_str() {
        "abc".to_color();
    }

    #[test]
    #[should_panic]
    fn too_short_string() {
        String::from("abc").to_color();
    }

    #[test]
    #[should_panic]
    fn too_long_str() {
        "abcdef0".to_color();
    }

    #[test]
    #[should_panic]
    fn too_long_string() {
        String::from("abcdef0").to_color();
    }

    #[test]
    #[should_panic]
    fn invalid_characters_str() {
        "abdefg".to_color();
    }

    #[test]
    #[should_panic]
    fn invalid_characters_string() {
        String::from("abdefg").to_color();
    }
}
