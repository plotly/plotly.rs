use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::Color;
use crate::common::{Anchor, Font, Orientation, Title};
use crate::layout::VAlign;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TraceOrder {
    Reversed,
    Grouped,
    #[serde(rename = "reversed+grouped")]
    ReversedGrouped,
    Normal,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ItemSizing {
    Trace,
    Constant,
}

#[derive(Debug, Clone)]
pub enum ItemClick {
    Toggle,
    ToggleOthers,
    False,
}

impl Serialize for ItemClick {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Toggle => serializer.serialize_str("toggle"),
            Self::ToggleOthers => serializer.serialize_str("toggleothers"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GroupClick {
    ToggleItem,
    ToggleGroup,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Legend {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    #[serde(rename = "borderwidth")]
    border_width: Option<usize>,
    font: Option<Font>,
    orientation: Option<Orientation>,
    #[serde(rename = "traceorder")]
    trace_order: Option<TraceOrder>,
    #[serde(rename = "tracegroupgap")]
    trace_group_gap: Option<usize>,
    #[serde(rename = "itemsizing")]
    item_sizing: Option<ItemSizing>,
    #[serde(rename = "itemclick")]
    item_click: Option<ItemClick>,
    #[serde(rename = "itemdoubleclick")]
    item_double_click: Option<ItemClick>,
    x: Option<f64>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    y: Option<f64>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    valign: Option<VAlign>,
    title: Option<Title>,
    #[serde(rename = "groupclick")]
    group_click: Option<GroupClick>,
    #[serde(rename = "itemwidth")]
    item_width: Option<usize>,
}

impl Legend {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn serialize_trace_order() {
        assert_eq!(to_value(TraceOrder::Reversed).unwrap(), json!("reversed"));
        assert_eq!(to_value(TraceOrder::Grouped).unwrap(), json!("grouped"));
        assert_eq!(to_value(TraceOrder::ReversedGrouped).unwrap(), json!("reversed+grouped"));
        assert_eq!(to_value(TraceOrder::Normal).unwrap(), json!("normal"));
    }

    #[test]
    fn serialize_item_sizing() {
        assert_eq!(to_value(ItemSizing::Trace).unwrap(), json!("trace"));
        assert_eq!(to_value(ItemSizing::Constant).unwrap(), json!("constant"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_item_click() {
        assert_eq!(to_value(ItemClick::Toggle).unwrap(), json!("toggle"));
        assert_eq!(to_value(ItemClick::ToggleOthers).unwrap(), json!("toggleothers"));
        assert_eq!(to_value(ItemClick::False).unwrap(), json!(false));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_group_click() {
        assert_eq!(to_value(GroupClick::ToggleItem).unwrap(), json!("toggleitem"));
        assert_eq!(to_value(GroupClick::ToggleGroup).unwrap(), json!("togglegroup"));
    }

    #[test]
    fn serialize_legend() {
        let legend = Legend::new()
            .background_color("#123123")
            .border_color("#321321")
            .border_width(500)
            .font(Font::new())
            .orientation(Orientation::Vertical)
            .trace_order(TraceOrder::Normal)
            .trace_group_gap(10)
            .item_sizing(ItemSizing::Trace)
            .item_click(ItemClick::Toggle)
            .item_double_click(ItemClick::False)
            .x(1.0)
            .x_anchor(Anchor::Auto)
            .y(2.0)
            .y_anchor(Anchor::Left)
            .valign(VAlign::Middle)
            .title("title")
            .group_click(GroupClick::ToggleItem)
            .item_width(50);

        let expected = json!({
            "bgcolor": "#123123",
            "bordercolor": "#321321",
            "borderwidth": 500,
            "font": {},
            "orientation": "v",
            "traceorder": "normal",
            "tracegroupgap": 10,
            "itemsizing": "trace",
            "itemclick": "toggle",
            "itemdoubleclick": false,
            "x": 1.0,
            "xanchor": "auto",
            "y": 2.0,
            "yanchor": "left",
            "valign": "middle",
            "title": {"text": "title"},
            "groupclick": "toggleitem",
            "itemwidth": 50
        });

        assert_eq!(to_value(legend).unwrap(), expected)
    }
}
