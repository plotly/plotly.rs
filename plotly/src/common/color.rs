pub trait Color {
    fn to_color_string(&self) -> String;
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
    fn to_color_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
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
    fn to_color_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

// https://www.w3.org/TR/css-color-3/#svg-color
#[derive(Debug)]
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
    fn to_color_string(&self) -> String {
        match self {
            NamedColor::AliceBlue => "aliceblue".to_owned(),
            NamedColor::AntiqueWhite => "antiquewhite".to_owned(),
            NamedColor::Aqua => "aqua".to_owned(),
            NamedColor::Aquamarine => "aquamarine".to_owned(),
            NamedColor::Azure => "azure".to_owned(),
            NamedColor::Beige => "beige".to_owned(),
            NamedColor::Bisque => "bisque".to_owned(),
            NamedColor::Black => "black".to_owned(),
            NamedColor::BlancheDalmond => "blanchedalmond".to_owned(),
            NamedColor::Blue => "blue".to_owned(),
            NamedColor::BlueViolet => "blueviolet".to_owned(),
            NamedColor::Brown => "brown".to_owned(),
            NamedColor::BurlyWood => "burlywood".to_owned(),
            NamedColor::CadetBlue => "cadetblue".to_owned(),
            NamedColor::Chartreuse => "chartreuse".to_owned(),
            NamedColor::Chocolate => "chocolate".to_owned(),
            NamedColor::Coral => "coral".to_owned(),
            NamedColor::CornFlowerBlue => "cornflowerblue".to_owned(),
            NamedColor::CornSilk => "cornsilk".to_owned(),
            NamedColor::Crimson => "crimson".to_owned(),
            NamedColor::Cyan => "cyan".to_owned(),
            NamedColor::DarkBlue => "darkblue".to_owned(),
            NamedColor::DarkCyan => "darkcyan".to_owned(),
            NamedColor::DarkGoldenrod => "darkgoldenrod".to_owned(),
            NamedColor::DarkGray => "darkgray".to_owned(),
            NamedColor::DarkGreen => "darkgreen".to_owned(),
            NamedColor::DarkGrey => "darkgrey".to_owned(),
            NamedColor::DarkKhaki => "darkkhaki".to_owned(),
            NamedColor::DarkMagenta => "darkmagenta".to_owned(),
            NamedColor::DarkOliveGreen => "darkolivegreen".to_owned(),
            NamedColor::DarkOrange => "darkorange".to_owned(),
            NamedColor::DarkOrchid => "darkorchid".to_owned(),
            NamedColor::DarkRed => "darkred".to_owned(),
            NamedColor::DarkSalmon => "darksalmon".to_owned(),
            NamedColor::DarkSeaGreen => "darkseagreen".to_owned(),
            NamedColor::DarkSlateBlue => "darkslateblue".to_owned(),
            NamedColor::DarkSlateGray => "darkslategray".to_owned(),
            NamedColor::DarkSlateGrey => "darkslategrey".to_owned(),
            NamedColor::DarkTurquoise => "darkturquoise".to_owned(),
            NamedColor::DarkViolet => "darkviolet".to_owned(),
            NamedColor::DeepPink => "deeppink".to_owned(),
            NamedColor::DeepSkyBlue => "deepskyblue".to_owned(),
            NamedColor::DimGray => "dimgray".to_owned(),
            NamedColor::DimGrey => "dimgrey".to_owned(),
            NamedColor::DodgerBlue => "dodgerblue".to_owned(),
            NamedColor::FireBrick => "firebrick".to_owned(),
            NamedColor::FloralWhite => "floralwhite".to_owned(),
            NamedColor::ForestGreen => "forestgreen".to_owned(),
            NamedColor::Fuchsia => "fuchsia".to_owned(),
            NamedColor::Gainsboro => "gainsboro".to_owned(),
            NamedColor::GhostWhite => "ghostwhite".to_owned(),
            NamedColor::Gold => "gold".to_owned(),
            NamedColor::Goldenrod => "goldenrod".to_owned(),
            NamedColor::Gray => "gray".to_owned(),
            NamedColor::Green => "green".to_owned(),
            NamedColor::GreenYellow => "greenyellow".to_owned(),
            NamedColor::Grey => "grey".to_owned(),
            NamedColor::Honeydew => "honeydew".to_owned(),
            NamedColor::HotPink => "hotpink".to_owned(),
            NamedColor::IndianRed => "indianred".to_owned(),
            NamedColor::Indigo => "indigo".to_owned(),
            NamedColor::Ivory => "ivory".to_owned(),
            NamedColor::Khaki => "khaki".to_owned(),
            NamedColor::Lavender => "lavender".to_owned(),
            NamedColor::LavenderBlush => "lavenderblush".to_owned(),
            NamedColor::LawnGreen => "lawngreen".to_owned(),
            NamedColor::LemonChiffon => "lemonchiffon".to_owned(),
            NamedColor::LightBlue => "lightblue".to_owned(),
            NamedColor::LightCoral => "lightcoral".to_owned(),
            NamedColor::LightCyan => "lightcyan".to_owned(),
            NamedColor::LightGoldenrodYellow => "lightgoldenrodyellow".to_owned(),
            NamedColor::LightGray => "lightgray".to_owned(),
            NamedColor::LightGreen => "lightgreen".to_owned(),
            NamedColor::LightGrey => "lightgrey".to_owned(),
            NamedColor::LightPink => "lightpink".to_owned(),
            NamedColor::LightSalmon => "lightsalmon".to_owned(),
            NamedColor::LightSeaGreen => "lightseagreen".to_owned(),
            NamedColor::LightSkyBlue => "lightskyblue".to_owned(),
            NamedColor::LightSlateGray => "lightslategray".to_owned(),
            NamedColor::LightSlateGrey => "lightslategrey".to_owned(),
            NamedColor::LightSteelBlue => "lightsteelblue".to_owned(),
            NamedColor::LightYellow => "lightyellow".to_owned(),
            NamedColor::Lime => "lime".to_owned(),
            NamedColor::LimeGreen => "limegreen".to_owned(),
            NamedColor::Linen => "linen".to_owned(),
            NamedColor::Magenta => "magenta".to_owned(),
            NamedColor::Maroon => "maroon".to_owned(),
            NamedColor::MediumAquamarine => "mediumaquamarine".to_owned(),
            NamedColor::MediumBlue => "mediumblue".to_owned(),
            NamedColor::MediumOrchid => "mediumorchid".to_owned(),
            NamedColor::MediumPurple => "mediumpurple".to_owned(),
            NamedColor::MediumSeaGreen => "mediumseagreen".to_owned(),
            NamedColor::MediumSlateBlue => "mediumslateblue".to_owned(),
            NamedColor::MediumSpringGreen => "mediumspringgreen".to_owned(),
            NamedColor::MediumTurquoise => "mediumturquoise".to_owned(),
            NamedColor::MediumVioletRed => "mediumvioletred".to_owned(),
            NamedColor::MidnightBlue => "midnightblue".to_owned(),
            NamedColor::MintCream => "mintcream".to_owned(),
            NamedColor::MistyRose => "mistyrose".to_owned(),
            NamedColor::Moccasin => "moccasin".to_owned(),
            NamedColor::NavajoWhite => "navajowhite".to_owned(),
            NamedColor::Navy => "navy".to_owned(),
            NamedColor::OldLace => "oldlace".to_owned(),
            NamedColor::Olive => "olive".to_owned(),
            NamedColor::OliveDrab => "olivedrab".to_owned(),
            NamedColor::Orange => "orange".to_owned(),
            NamedColor::OrangeRed => "orangered".to_owned(),
            NamedColor::Orchid => "orchid".to_owned(),
            NamedColor::PaleGoldenrod => "palegoldenrod".to_owned(),
            NamedColor::PaleGreen => "palegreen".to_owned(),
            NamedColor::PaleTurquoise => "paleturquoise".to_owned(),
            NamedColor::PaleVioletRed => "palevioletred".to_owned(),
            NamedColor::PapayaWhip => "papayawhip".to_owned(),
            NamedColor::PeachPuff => "peachpuff".to_owned(),
            NamedColor::Peru => "peru".to_owned(),
            NamedColor::Pink => "pink".to_owned(),
            NamedColor::Plum => "plum".to_owned(),
            NamedColor::PowderBlue => "powderblue".to_owned(),
            NamedColor::Purple => "purple".to_owned(),
            NamedColor::Red => "red".to_owned(),
            NamedColor::RosyBrown => "rosybrown".to_owned(),
            NamedColor::RoyalBlue => "royalblue".to_owned(),
            NamedColor::SaddleBrown => "saddlebrown".to_owned(),
            NamedColor::Salmon => "salmon".to_owned(),
            NamedColor::SandyBrown => "sandybrown".to_owned(),
            NamedColor::SeaGreen => "seagreen".to_owned(),
            NamedColor::Seashell => "seashell".to_owned(),
            NamedColor::Sienna => "sienna".to_owned(),
            NamedColor::Silver => "silver".to_owned(),
            NamedColor::SkyBlue => "skyblue".to_owned(),
            NamedColor::SlateBlue => "slateblue".to_owned(),
            NamedColor::SlateGray => "slategray".to_owned(),
            NamedColor::SlateGrey => "slategrey".to_owned(),
            NamedColor::Snow => "snow".to_owned(),
            NamedColor::SpringGreen => "springgreen".to_owned(),
            NamedColor::SteelBlue => "steelblue".to_owned(),
            NamedColor::Tan => "tan".to_owned(),
            NamedColor::Teal => "teal".to_owned(),
            NamedColor::Thistle => "thistle".to_owned(),
            NamedColor::Tomato => "tomato".to_owned(),
            NamedColor::Turquoise => "turquoise".to_owned(),
            NamedColor::Violet => "violet".to_owned(),
            NamedColor::Wheat => "wheat".to_owned(),
            NamedColor::White => "white".to_owned(),
            NamedColor::WhiteSmoke => "whitesmoke".to_owned(),
            NamedColor::Yellow => "yellow".to_owned(),
            NamedColor::YellowGreen => "yellowgreen".to_owned(),
            NamedColor::Transparent => "transparent".to_owned(),
        }
    }
}

impl Color for str {
    fn to_color_string(&self) -> String {
        String::from(self).to_color_string()
    }
}

impl<T> Color for T
where
    T: AsRef<str> + std::fmt::Display + Sized,
{
    fn to_color_string(&self) -> String {
        if self.as_ref().len() < 6 || self.as_ref().len() > 7 {
            panic!(format!("{} is not a valid hex color!", self));
        }
        if self.as_ref().len() == 6 && self.as_ref().starts_with('#') {
            panic!(format!("{} is not a valid hex color!", self));
        }
        if self.as_ref().len() == 7 && !self.as_ref().starts_with('#') {
            panic!(format!("{} is not a valid hex color!", self));
        }
        let valid_characters = "#ABCDEF0123456789";
        let mut s = self.as_ref().to_uppercase();
        if s.len() == 6 {
            s = format!("#{}", s);
        }
        for c in s.chars() {
            if !valid_characters.contains(c) {
                panic!(format!("{} is not a valid hex color!", self));
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_color_normalization_str() {
        assert_eq!("#aabbcc".to_color_string(), "#AABBCC");
        assert_eq!("aabbcc".to_color_string(), "#AABBCC");
        assert_eq!("aaBBcc".to_color_string(), "#AABBCC");
        assert_eq!("FABCDe".to_color_string(), "#FABCDE");
        assert_eq!("123456".to_color_string(), "#123456");
        assert_eq!("7890EE".to_color_string(), "#7890EE");
    }

    #[test]
    fn hex_color_normalization_string() {
        assert_eq!(String::from("#aabbcc").to_color_string(), "#AABBCC");
        assert_eq!(String::from("aabbcc".to_color_string()), "#AABBCC");
        assert_eq!(String::from("aaBBcc".to_color_string()), "#AABBCC");
        assert_eq!(String::from("FABCDe".to_color_string()), "#FABCDE");
        assert_eq!(String::from("123456".to_color_string()), "#123456");
        assert_eq!(String::from("7890EE".to_color_string()), "#7890EE");
    }

    #[test]
    #[should_panic]
    fn too_short_str() {
        "abc".to_color_string();
    }

    #[test]
    #[should_panic]
    fn too_short_string() {
        String::from("abc").to_color_string();
    }

    #[test]
    #[should_panic]
    fn too_long_str() {
        "abcdef0".to_color_string();
    }

    #[test]
    #[should_panic]
    fn too_long_string() {
        String::from("abcdef0").to_color_string();
    }

    #[test]
    #[should_panic]
    fn invalid_characters_str() {
        "abdefg".to_color_string();
    }

    #[test]
    #[should_panic]
    fn invalid_characters_string() {
        String::from("abdefg").to_color_string();
    }
}
