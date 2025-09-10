//! Table trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::{Color, ColorArray},
    common::{Dim, PlotType, Visible},
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
    /// Specifies the rendered order of the data columns; for example, a value
    /// `2` at position `0`, means that column index `0` in the data will be
    /// rendered as the, third column, as columns have an index base of
    /// zero.
    column_order: Option<Vec<usize>>,
    #[serde(rename = "columnwidth")]
    /// The width of columns expressed as a ratio. Columns fill the available
    /// width, in proportion of their specified column widths.
    column_width: Option<f64>,
    /// Header cell values. `values[m][n]` represents the value of the `n`th
    /// point in column `m`,, therefore the `values[m]` vector length for
    /// all columns must be the same (longer vectors, will be truncated).
    /// Each value must be a finite number or a string.
    header: Option<Header<T>>,
    /// Cell values. `values[m][n]` represents the value of the `n`th point in
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
pub struct Header<T> {
    /// Header cell values. `values[m][n]` represents the value of the `n`th
    /// point in column `m`, therefore the `values[m]` vector length for all
    /// columns must be the same (longer vectors, will be truncated). Each
    /// value must be a finite number or a string.
    values: Option<Vec<T>>,
    /// Prefix for cell values.
    prefix: Option<Dim<String>>,
    /// Suffix for cell values.
    suffix: Option<Dim<String>>,
    height: Option<f64>,
    #[field_setter(skip)]
    align: Option<Dim<Align>>,
    line: Option<Line>,
    /// Sets the cell fill color. It accepts either a specific color,
    /// or an array of colors or a 2D array of colors
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

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(Dim::Scalar(align));
        self
    }

    pub fn align_array(mut self, aligns: Vec<Align>) -> Self {
        self.align = Some(Dim::Vector(aligns));
        self
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
    prefix: Option<Dim<String>>,
    ///Suffix for cell values.
    suffix: Option<Dim<String>>,
    height: Option<f64>,
    #[field_setter(skip)]
    align: Option<Dim<Align>>,
    line: Option<Line>,
    ///Sets the cell fill color. It accepts either a specific color,
    ///or an array of colors or a 2D array of colors
    fill: Option<Fill>,
    #[field_setter(skip)]
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

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(Dim::Scalar(align));
        self
    }

    pub fn align_array(mut self, aligns: Vec<Align>) -> Self {
        self.align = Some(Dim::Vector(aligns));
        self
    }

    pub fn align_matrix(mut self, aligns: Vec<Vec<Align>>) -> Self {
        self.align = Some(Dim::Matrix(aligns));
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Line {
    color: Option<Dim<Box<dyn Color>>>,
    width: Option<Dim<f64>>,
}

impl Line {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(Box::new(color)));
        self
    }

    pub fn color_array<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.color = Some(Dim::Vector(ColorArray(colors).into()));
        self
    }

    pub fn color_matrix<C: Color>(mut self, colors: Vec<Vec<C>>) -> Self {
        let mm = colors
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| Box::new(c) as Box<dyn Color>)
                    .collect()
            })
            .collect();
        self.color = Some(Dim::Matrix(mm));
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(Dim::Scalar(width));
        self
    }

    pub fn width_array(mut self, widths: Vec<f64>) -> Self {
        self.width = Some(Dim::Vector(widths));
        self
    }

    pub fn width_matrix(mut self, widths: Vec<Vec<f64>>) -> Self {
        let mm = widths
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
        self.width = Some(Dim::Matrix(mm));
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Fill {
    color: Option<Dim<Box<dyn Color>>>,
}

impl Fill {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(Box::new(color)));
        self
    }

    pub fn color_array<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.color = Some(Dim::Vector(ColorArray(colors).into()));
        self
    }

    ///  Set a 2D color matrix for cells
    pub fn color_matrix<C: Color>(mut self, m: Vec<Vec<C>>) -> Self {
        let mm = m
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| Box::new(c) as Box<dyn Color>)
                    .collect()
            })
            .collect();
        self.color = Some(Dim::Matrix(mm));
        self
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Clone, Debug, Default)]
pub enum FontStyle {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "italic")]
    Italic,
}

#[derive(Serialize, Clone, Debug, Default)]
pub enum TextCase {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "word caps")]
    WordCaps,
    #[serde(rename = "upper")]
    Upper,
    #[serde(rename = "lower")]
    Lower,
}

#[derive(Serialize, Clone, Debug, Default)]
pub enum TextVariant {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "small-caps")]
    SmallCaps,
    #[serde(rename = "all-small-caps")]
    AllSmallCaps,
    #[serde(rename = "all-petite-caps")]
    AllPetiteCaps,
}

#[derive(Serialize, Clone, Debug, Default)]
pub enum LinePosition {
    #[default]
    #[serde(rename = "under")]
    Under,
    #[serde(rename = "over")]
    Over,
    #[serde(rename = "through")]
    Through,
    #[serde(rename = "under+over")]
    UnderOver,
    #[serde(rename = "under+through")]
    UnderThrough,
    #[serde(rename = "over+through")]
    OverThrough,
    #[serde(rename = "under+over+through")]
    UnderOverThrough,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct Font {
    color: Option<Dim<Box<dyn Color>>>,
    family: Option<Dim<String>>,
    size: Option<Dim<f64>>,
    style: Option<Dim<FontStyle>>,
    textcase: Option<Dim<TextCase>>,
    variant: Option<Dim<TextVariant>>,
    weight: Option<Dim<f64>>,
    lineposition: Option<Dim<LinePosition>>,
}

impl Font {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn family(mut self, family: &str) -> Self {
        self.family = Some(Dim::Scalar(family.to_owned()));
        self
    }

    pub fn family_array(mut self, families: Vec<String>) -> Self {
        self.family = Some(Dim::Vector(families));
        self
    }

    pub fn family_matrix(mut self, families: Vec<Vec<String>>) -> Self {
        self.family = Some(Dim::Matrix(families));
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(Dim::Scalar(size));
        self
    }

    pub fn size_array(mut self, sizes: Vec<f64>) -> Self {
        self.size = Some(Dim::Vector(sizes));
        self
    }

    pub fn size_matrix(mut self, sizes: Vec<Vec<f64>>) -> Self {
        self.size = Some(Dim::Matrix(sizes));
        self
    }

    pub fn weight(mut self, weight: f64) -> Self {
        self.weight = Some(Dim::Scalar(weight));
        self
    }

    pub fn weight_array(mut self, weights: Vec<f64>) -> Self {
        self.weight = Some(Dim::Vector(weights));
        self
    }

    pub fn weight_matrix(mut self, weights: Vec<Vec<f64>>) -> Self {
        self.weight = Some(Dim::Matrix(weights));
        self
    }

    pub fn style(mut self, style: FontStyle) -> Self {
        self.style = Some(Dim::Scalar(style));
        self
    }

    pub fn style_array(mut self, styles: Vec<FontStyle>) -> Self {
        self.style = Some(Dim::Vector(styles));
        self
    }

    pub fn style_matrix(mut self, styles: Vec<Vec<FontStyle>>) -> Self {
        self.style = Some(Dim::Matrix(styles));
        self
    }

    pub fn textcase(mut self, textcase: TextCase) -> Self {
        self.textcase = Some(Dim::Scalar(textcase));
        self
    }

    pub fn textcase_array(mut self, textcases: Vec<TextCase>) -> Self {
        self.textcase = Some(Dim::Vector(textcases));
        self
    }

    pub fn textcase_matrix(mut self, textcases: Vec<Vec<TextCase>>) -> Self {
        self.textcase = Some(Dim::Matrix(textcases));
        self
    }

    pub fn color<C: Color>(mut self, color: C) -> Self {
        self.color = Some(Dim::Scalar(Box::new(color)));
        self
    }

    pub fn color_array<C: Color>(mut self, colors: Vec<C>) -> Self {
        self.color = Some(Dim::Vector(ColorArray(colors).into()));
        self
    }

    pub fn color_matrix<C: Color>(mut self, m: Vec<Vec<C>>) -> Self {
        let mm = m
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| Box::new(c) as Box<dyn Color>)
                    .collect()
            })
            .collect();
        self.color = Some(Dim::Matrix(mm));
        self
    }

    pub fn lineposition(mut self, lineposition: LinePosition) -> Self {
        self.lineposition = Some(Dim::Scalar(lineposition));
        self
    }

    pub fn lineposition_array(mut self, linepositions: Vec<LinePosition>) -> Self {
        self.lineposition = Some(Dim::Vector(linepositions));
        self
    }

    pub fn lineposition_matrix(mut self, linepositions: Vec<Vec<LinePosition>>) -> Self {
        self.lineposition = Some(Dim::Matrix(linepositions));
        self
    }

    pub fn variant(mut self, variant: TextVariant) -> Self {
        self.variant = Some(Dim::Scalar(variant));
        self
    }

    pub fn variant_array(mut self, variants: Vec<TextVariant>) -> Self {
        self.variant = Some(Dim::Vector(variants));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::color::NamedColor;
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_simple_table() {
        let columns = Header::new(vec![String::from("col1"), String::from("col2")])
            .line(Line::new().color(NamedColor::Black).width(0.5));
        let values = Cells::new(vec![vec![1, 2], vec![2, 3]]);
        let trace = Table::new(columns, values);

        let expected = json!({
            "type": "table",
            "cells": {
                "values": [[1, 2], [2, 3]],
            },
            "header": {
                "values": ["col1", "col2"],
                "line": {
                    "color": "black",
                    "width": 0.5,
                },
            },
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_table_with_color_array() {
        let columns = Header::new(vec![String::from("col1"), String::from("col2")])
            .fill(Fill::new().color_array(vec![NamedColor::Blue, NamedColor::Red]))
            .font(Font::new().color_array(vec![NamedColor::Black, NamedColor::Blue]));
        let values = Cells::new(vec![vec![1, 2], vec![2, 3]])
            .fill(Fill::new().color_array(vec![NamedColor::Blue, NamedColor::Red]))
            .line(
                Line::new()
                    .color_array(vec![NamedColor::Black, NamedColor::Blue])
                    .width_array(vec![0.5, 0.3]),
            );
        let trace = Table::new(columns, values);

        let expected = json!({
            "type": "table",
            "cells": {
                "values": [[1, 2], [2, 3]],
                "fill": {
                    "color": ["blue", "red"],
                },
                "line": {
                    "color": ["black", "blue"],
                    "width": [0.5, 0.3],
                },
            },
            "header": {
                "values": ["col1", "col2"],
                "fill": {
                    "color": ["blue", "red"],
                },
                "font": {
                    "color": ["black", "blue"],
                },
            },
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_table_with_color_matrix() {
        let columns = Header::new(vec![String::from("col1"), String::from("col2")])
            .fill(Fill::new().color_array(vec![NamedColor::Blue, NamedColor::Red]));
        let values = Cells::new(vec![vec![1, 2], vec![2, 3]]).fill(Fill::new().color_matrix(vec![
            vec![NamedColor::Blue, NamedColor::Red],
            vec![NamedColor::Red, NamedColor::Blue],
        ]));
        let trace = Table::new(columns, values);

        let expected = json!({
            "type": "table",
            "cells": {
                "values": [[1, 2], [2, 3]],
                "fill": {
                    "color": [["blue", "red"], ["red", "blue"]],
                },
            },
            "header": {
                "values": ["col1", "col2"],
                "fill": {
                    "color": ["blue", "red"],
                },
            },
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
