use plotly_derive::FieldSetter;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum RowOrder {
    #[serde(rename = "top to bottom")]
    TopToBottom,
    #[serde(rename = "bottom to top")]
    BottomToTop,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GridPattern {
    Independent,
    Coupled,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GridXSide {
    Bottom,
    #[serde(rename = "bottom plot")]
    BottomPlot,
    #[serde(rename = "top plot")]
    TopPlot,
    Top,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GridYSide {
    Left,
    #[serde(rename = "left plot")]
    LeftPlot,
    #[serde(rename = "right plot")]
    RightPlot,
    Right,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct GridDomain {
    x: Option<Vec<f64>>,
    y: Option<Vec<f64>>,
}

impl GridDomain {
    pub fn new() -> Self {
        Default::default()
    }
}
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct LayoutGrid {
    rows: Option<usize>,
    #[serde(rename = "roworder")]
    row_order: Option<RowOrder>,
    columns: Option<usize>,
    #[serde(rename = "subplots")]
    sub_plots: Option<Vec<String>>,
    #[serde(rename = "xaxes")]
    x_axes: Option<Vec<String>>,
    #[serde(rename = "yaxes")]
    y_axes: Option<Vec<String>>,
    pattern: Option<GridPattern>,
    #[serde(rename = "xgap")]
    x_gap: Option<f64>,
    #[serde(rename = "ygap")]
    y_gap: Option<f64>,
    domain: Option<GridDomain>,
    #[serde(rename = "xside")]
    x_side: Option<GridXSide>,
    #[serde(rename = "yside")]
    y_side: Option<GridYSide>,
}

impl LayoutGrid {
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
    fn serialize_row_order() {
        assert_eq!(to_value(RowOrder::TopToBottom).unwrap(), json!("top to bottom"));
        assert_eq!(to_value(RowOrder::BottomToTop).unwrap(), json!("bottom to top"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_grid_pattern() {
        assert_eq!(to_value(GridPattern::Independent).unwrap(), json!("independent"));
        assert_eq!(to_value(GridPattern::Coupled).unwrap(), json!("coupled"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_grid_x_side() {
        assert_eq!(to_value(GridXSide::Bottom).unwrap(), json!("bottom"));
        assert_eq!(to_value(GridXSide::BottomPlot).unwrap(), json!("bottom plot"));
        assert_eq!(to_value(GridXSide::Top).unwrap(), json!("top"));
        assert_eq!(to_value(GridXSide::TopPlot).unwrap(), json!("top plot"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_grid_y_side() {
        assert_eq!(to_value(GridYSide::Left).unwrap(), json!("left"));
        assert_eq!(to_value(GridYSide::LeftPlot).unwrap(), json!("left plot"));
        assert_eq!(to_value(GridYSide::Right).unwrap(), json!("right"));
        assert_eq!(to_value(GridYSide::RightPlot).unwrap(), json!("right plot"));
    }

    #[test]
    fn serialize_grid_domain() {
        let grid_domain = GridDomain::new().x(vec![0.0]).y(vec![1.0]);
        let expected = json!({
            "x": [0.0],
            "y": [1.0]
        });

        assert_eq!(to_value(grid_domain).unwrap(), expected);
    }

    #[test]
    fn serialize_layout_grid() {
        let layout_grid = LayoutGrid::new()
            .rows(224)
            .row_order(RowOrder::BottomToTop)
            .columns(501)
            .sub_plots(vec!["subplots".to_string()])
            .x_axes(vec!["xaxes".to_string()])
            .y_axes(vec!["yaxes".to_string()])
            .pattern(GridPattern::Coupled)
            .x_gap(2.2)
            .y_gap(4.4)
            .domain(GridDomain::new())
            .x_side(GridXSide::Top)
            .y_side(GridYSide::Right);

        let expected = json!({
            "rows": 224,
            "roworder": "bottom to top",
            "columns": 501,
            "subplots": ["subplots"],
            "xaxes": ["xaxes"],
            "yaxes": ["yaxes"],
            "pattern": "coupled",
            "xgap": 2.2,
            "ygap": 4.4,
            "domain": {},
            "xside": "top",
            "yside": "right",
        });

        assert_eq!(to_value(layout_grid).unwrap(), expected);
    }
}
