use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::private::NumOrString;

/// Struct representing a rangebreak for Plotly axes.
/// See: https://plotly.com/python/reference/layout/xaxis/#layout-xaxis-rangebreaks
#[derive(Debug, Clone, Serialize, PartialEq, FieldSetter)]
pub struct RangeBreak {
    /// Sets the lower and upper bounds for this range break, e.g. ["sat",
    /// "mon"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[field_setter(skip)]
    pub bounds: Option<[NumOrString; 2]>,

    /// Sets the pattern by which this range break is generated, e.g. "day of
    /// week"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<NumOrString>,

    /// Sets the values at which this range break occurs.
    /// See Plotly.js docs for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[field_setter(skip)]
    pub values: Option<Vec<NumOrString>>,

    /// Sets the size of each range break in milliseconds (for time axes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvalue: Option<u64>,

    /// Sets whether this range break is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl RangeBreak {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bounds<T: Into<NumOrString>>(mut self, lower: T, upper: T) -> Self {
        self.bounds = Some([lower.into(), upper.into()]);
        self
    }

    pub fn values<T: Into<NumOrString>>(mut self, values: Vec<T>) -> Self {
        self.values = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_range_break_default() {
        let range_break = RangeBreak::new();

        let expected = json!({});

        assert_eq!(to_value(range_break).unwrap(), expected);
    }

    #[test]
    fn serialize_range_break() {
        let range_break = RangeBreak::new()
            .pattern("day of week")
            .bounds("sat", "mon")
            .values(vec!["sat", "sun"])
            .dvalue(86400000)
            .enabled(true);

        let expected = json!({
            "pattern": "day of week",
            "bounds": ["sat", "mon"],
            "values": ["sat", "sun"],
            "dvalue": 86400000,
            "enabled": true
        });

        assert_eq!(to_value(range_break).unwrap(), expected);
    }

    #[test]
    fn seriealize_range_break_with_mixed_values() {
        let range_break = RangeBreak::new().values(vec![
            NumOrString::S("sat".to_string()),
            NumOrString::I(0),
            NumOrString::S("sun".to_string()),
            NumOrString::I(1),
        ]);

        let expected = json!({
            "values": ["sat", 0, "sun", 1]
        });

        assert_eq!(to_value(range_break).unwrap(), expected);
    }
}
