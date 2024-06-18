//! Table trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{Font, Line, PlotType, Visible},
    Trace,
};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Table<T, N>
where
    T: Serialize + Clone + 'static,
    N: Serialize + Clone + 'static,
{
    #[field_setter(default = "PlotType::Table")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appear as the legend item and on
    /// hover.
    name: Option<String>,
    #[serde(rename = "columnorder")]
    /// Determines whether or not this trace is visible. If
    /// `Visible::LegendOnly`, the trace is not drawn, but can appear as a
    /// legend item (provided that the legend itself is visible).
    visible: Option<Visible>,
    ///Specifies the rendered order of the data columns; for example, a value
    /// `2` at position `0`, means that column index `0` in the data will be
    /// rendered as the, third column, as columns have an index base of
    /// zero.
    column_order: Option<Vec<usize>>,
    #[serde(rename = "columnwidth")]
    ///The width of columns expressed as a ratio. Columns fill the available
    /// width, in proportion of their specified column widths.
    column_width: Option<f64>,
    ///Header cell values. `values[m][n]` represents the value of the `n`th
    /// point in column `m`,, therefore the `values[m]` vector length for
    /// all columns must be the same (longer vectors, will be truncated).
    /// Each value must be a finite number or a string.
    header: Option<Header<T>>,
    ///Cell values. `values[m][n]` represents the value of the `n`th point in
    /// column `m`,, therefore the `values[m]` vector length for all columns
    /// must be the same (longer vectors, will be truncated). Each value
    /// must be a finite number or a string.
    cells: Option<Cells<N>>,
}

impl<T, N> Table<T, N>
where
    T: Serialize + Clone + Default + 'static,
    N: Serialize + Clone + Default + 'static,
{
    pub fn new(header: Header<T>, cells: Cells<N>) -> Box<Self> {
        Box::new(Table {
            header: Some(header),
            cells: Some(cells),
            ..Default::default()
        })
    }
}

impl<T, N> Trace for Table<T, N>
where
    T: Serialize + Clone + 'static,
    N: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Cells<N> {
    ///Cell values. `values[m][n]` represents the value of the `n`th point in
    /// column `m`, therefore the `values[m]` vector length for all columns
    /// must be the same (longer vectors, will be truncated). Each value
    /// must be a finite number or a string
    values: Option<Vec<Vec<N>>>,
    ///Prefix for cell values.
    prefix: Option<String>,
    ///Suffix for cell values.
    suffix: Option<String>,
    height: Option<f64>,
    align: Option<String>,
    line: Option<Line>,
    ///Sets the cell fill color. It accepts either a specific color,
    ///or an array of colors or a 2D array of colors
    fill: Option<Fill>,
    font: Option<Font>,
}

impl<N> Cells<N>
where
    N: Serialize + Clone + Default + 'static,
{
    pub fn new(values: Vec<Vec<N>>) -> Self {
        Cells {
            values: Some(values),
            ..Default::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Header<T> {
    ///Header cell values. `values[m][n]` represents the value of the `n`th
    /// point in column `m`, therefore the `values[m]` vector length for all
    /// columns must be the same (longer vectors, will be truncated). Each
    /// value must be a finite number or a string.
    values: Option<Vec<T>>,
    ///Prefix for cell values.
    prefix: Option<String>,
    ///Suffix for cell values.
    suffix: Option<String>,
    height: Option<f64>,
    align: Option<String>,
    line: Option<Line>,
    ///Sets the cell fill color. It accepts either a specific color,
    ///or an array of colors or a 2D array of colors
    fill: Option<Fill>,
    font: Option<Font>,
}

impl<T> Header<T>
where
    T: Serialize + Clone + Default + 'static,
{
    pub fn new(values: Vec<T>) -> Self {
        Header {
            values: Some(values),
            ..Default::default()
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Fill {
    color: Option<Box<dyn Color>>,
}

impl Fill {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_table() {
        let columns = Header::new(vec![String::from("col1"), String::from("col2")]);
        let values = Cells::new(vec![vec![1, 2], vec![2, 3]]);
        let trace = Table::new(columns, values);

        let expected = json!({
            "type": "table",
            "cells": {
                "values": [[1, 2], [2, 3]],
            },
            "header": {
                "values": ["col1", "col2"],
            },
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
