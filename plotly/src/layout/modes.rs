use serde::{Serialize, Serializer};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BoxMode {
    Group,
    Overlay,
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
pub enum ClickMode {
    Event,
    Select,
    #[serde(rename = "event+select")]
    EventAndSelect,
    None,
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

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn serialize_click_mode() {
        assert_eq!(to_value(ClickMode::Event).unwrap(), json!("event"));
        assert_eq!(to_value(ClickMode::Select).unwrap(), json!("select"));
        assert_eq!(to_value(ClickMode::EventAndSelect).unwrap(), json!("event+select"));
        assert_eq!(to_value(ClickMode::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_uniform_text_mode() {
        assert_eq!(to_value(UniformTextMode::False).unwrap(), json!(false));
        assert_eq!(to_value(UniformTextMode::Hide).unwrap(), json!("hide"));
        assert_eq!(to_value(UniformTextMode::Show).unwrap(), json!("show"));
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
}
