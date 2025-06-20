use serde::{Deserialize, Serialize};

/// Struct representing a rangebreak for Plotly axes.
/// See: https://plotly.com/python/reference/layout/xaxis/#layout-xaxis-rangebreaks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RangeBreak {
    /// Sets the lower and upper bounds for this range break, e.g. ["sat",
    /// "mon"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<[String; 2]>,

    /// Sets the pattern by which this range break is generated, e.g. "day of
    /// week"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Sets the values at which this range break occurs.
    /// See Plotly.js docs for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,

    /// Sets the size of each range break in milliseconds (for time axes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvalue: Option<u64>,

    /// Sets whether this range break is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Default for RangeBreak {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeBreak {
    pub fn new() -> Self {
        Self {
            bounds: None,
            pattern: None,
            values: None,
            dvalue: None,
            enabled: None,
        }
    }

    pub fn bounds(mut self, lower: impl Into<String>, upper: impl Into<String>) -> Self {
        self.bounds = Some([lower.into(), upper.into()]);
        self
    }

    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    pub fn values(mut self, values: Vec<impl Into<String>>) -> Self {
        self.values = Some(values.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn dvalue(mut self, dvalue: u64) -> Self {
        self.dvalue = Some(dvalue);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}
