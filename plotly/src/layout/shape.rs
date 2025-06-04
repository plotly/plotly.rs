use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::Color;
use crate::common::DashType;
use crate::private::NumOrString;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShapeType {
    Circle,
    Rect,
    Path,
    Line,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShapeLayer {
    Below,
    Above,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShapeSizeMode {
    Scaled,
    Pixel,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ShapeLine {
    /// Sets the line color.
    color: Option<Box<dyn Color>>,
    /// Sets the line width (in px).
    width: Option<f64>,
    /// Sets the dash style of lines. Set to a dash type string ("solid", "dot",
    /// "dash", "longdash", "dashdot", or "longdashdot") or a dash length
    /// list in px (eg "5px,10px,2px,2px").
    dash: Option<DashType>,
}

impl ShapeLine {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Shape {
    /// Determines whether or not this shape is visible.
    visible: Option<bool>,
    #[field_setter(skip)]
    r#type: Option<ShapeType>,
    /// Specifies whether shapes are drawn below or above traces.
    layer: Option<ShapeLayer>,
    /// Sets the shape's x coordinate axis. If set to an x axis id (e.g. "x" or
    /// "x2"), the `x` position refers to an x coordinate. If set to
    /// "paper", the `x` position refers to the distance from the left side
    /// of the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the left (right) side. If the axis `type` is "log", then
    /// you must take the log of your desired range. If the axis `type` is
    /// "date", then you must convert the date to unix time in milliseconds.
    #[serde(rename = "xref")]
    x_ref: Option<String>,
    /// Sets the shapes's sizing mode along the x axis. If set to "scaled",
    /// `x0`, `x1` and x coordinates within `path` refer to data values on
    /// the x axis or a fraction of the plot area's width (`xref` set to
    /// "paper"). If set to "pixel", `xanchor` specifies the x position
    /// in terms of data or plot fraction but `x0`, `x1` and x coordinates
    /// within `path` are pixels relative to `xanchor`. This way, the shape
    /// can have a fixed width while maintaining a position relative to data
    /// or plot fraction.
    #[serde(rename = "xsizemode")]
    x_size_mode: Option<ShapeSizeMode>,
    /// Only relevant in conjunction with `xsizemode` set to "pixel". Specifies
    /// the anchor point on the x axis to which `x0`, `x1` and x coordinates
    /// within `path` are relative to. E.g. useful to attach a pixel sized
    /// shape to a certain data value. No effect when `xsizemode` not set to
    /// "pixel".
    #[serde(rename = "xanchor")]
    x_anchor: Option<NumOrString>,
    /// Sets the shape's starting x position. See `type` and `xsizemode` for
    /// more info.
    x0: Option<NumOrString>,
    /// Sets the shape's end x position. See `type` and `xsizemode` for more
    /// info.
    x1: Option<NumOrString>,
    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g.
    /// "y" or "y2"), the `y` position refers to an y coordinate If set to
    /// "paper", the `y` position refers to the distance from the bottom of
    /// the plotting area in normalized coordinates where "0" ("1")
    /// corresponds to the bottom (top).
    #[serde(rename = "yref")]
    y_ref: Option<String>,
    /// Sets the shapes's sizing mode along the y axis. If set to "scaled",
    /// `y0`, `y1` and y coordinates within `path` refer to data values on
    /// the y axis or a fraction of the plot area's height (`yref` set to
    /// "paper"). If set to "pixel", `yanchor` specifies the y position
    /// in terms of data or plot fraction but `y0`, `y1` and y coordinates
    /// within `path` are pixels relative to `yanchor`. This way, the shape
    /// can have a fixed height while maintaining a position relative to
    /// data or plot fraction.
    #[serde(rename = "ysizemode")]
    y_size_mode: Option<ShapeSizeMode>,
    /// Only relevant in conjunction with `ysizemode` set to "pixel". Specifies
    /// the anchor point on the y axis to which `y0`, `y1` and y coordinates
    /// within `path` are relative to. E.g. useful to attach a pixel sized
    /// shape to a certain data value. No effect when `ysizemode` not set to
    /// "pixel".
    #[serde(rename = "yanchor")]
    y_anchor: Option<NumOrString>,
    /// Sets the shape's starting y position. See `type` and `ysizemode` for
    /// more info.
    y0: Option<NumOrString>,
    /// Sets the shape's end y position. See `type` and `ysizemode` for more
    /// info.
    y1: Option<NumOrString>,
    /// For `type` "path" - a valid SVG path with the pixel values replaced by
    /// data values in `xsizemode`/`ysizemode` being "scaled" and taken
    /// unmodified as pixels relative to `xanchor` and `yanchor` in case of
    /// "pixel" size mode. There are a few restrictions / quirks
    /// only absolute instructions, not relative. So the allowed segments
    /// are: M, L, H, V, Q, C, T, S, and Z arcs (A) are not allowed because
    /// radius rx and ry are relative. In the future we could consider
    /// supporting relative commands, but we would have to decide on how to
    /// handle date and log axes. Note that even as is, Q and C Bezier paths
    /// that are smooth on linear axes may not be smooth on log, and vice versa.
    /// no chained "polybezier" commands - specify the segment type for each
    /// one. On category axes, values are numbers scaled to the serial
    /// numbers of categories because using the categories themselves
    /// there would be no way to describe fractional positions On data axes:
    /// because space and T are both normal components of path strings, we
    /// can't use either to separate date from time parts. Therefore we'll
    /// use underscore for this purpose: 2015-02-21_13:45:56.789
    path: Option<String>,
    /// Sets the opacity of the shape. Number between or equal to 0 and 1.
    opacity: Option<f64>,
    /// Sets the shape line properties (`color`, `width`, `dash`).
    line: Option<ShapeLine>,
    /// Sets the color filling the shape's interior. Only applies to closed
    /// shapes.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Determines which regions of complex paths constitute the interior. For
    /// more info please visit <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule>
    #[serde(rename = "fillrule")]
    fill_rule: Option<FillRule>,
    /// Determines whether the shape could be activated for edit or not. Has no
    /// effect when the older editable shapes mode is enabled via
    /// `config.editable` or `config.edits.shapePosition`.
    editable: Option<bool>,
    /// When used in a template, named items are created in the output figure in
    /// addition to any items the figure already has in this array. You can
    /// modify these items in the output figure by making your own item with
    /// `templateitemname` matching this `name` alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). Has no
    /// effect outside of a template.
    name: Option<String>,
    /// Used to refer to a named item in this array in the template. Named items
    /// from the template will be created even without a matching item in
    /// the input figure, but you can modify one by making an item with
    /// `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is
    /// no template or no matching item, this item will be hidden unless you
    /// explicitly show it with `visible: true`.
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
}

impl Shape {
    pub fn new() -> Self {
        Default::default()
    }

    /// Specifies the shape type to be drawn. If "line", a line is drawn from
    /// (`x0`,`y0`) to (`x1`,`y1`) with respect to the axes' sizing mode. If
    /// "circle", a circle is drawn from ((`x0`+`x1`)/2, (`y0`+`y1`)/2))
    /// with radius (|(`x0`+`x1`)/2 - `x0`|, |(`y0`+`y1`)/2 -`y0`)|)
    /// with respect to the axes' sizing mode. If "rect", a rectangle is drawn
    /// linking (`x0`,`y0`), (`x1`,`y0`), (`x1`,`y1`), (`x0`,`y1`),
    /// (`x0`,`y0`) with respect to the axes' sizing mode. If "path", draw a
    /// custom SVG path using `path`. with respect to the axes' sizing mode.
    pub fn shape_type(mut self, shape_type: ShapeType) -> Self {
        self.r#type = Some(shape_type);
        self
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DrawDirection {
    Ortho,
    Horizontal,
    Vertical,
    Diagonal,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct NewShape {
    /// Sets the shape line properties (`color`, `width`, `dash`).
    line: Option<ShapeLine>,
    /// Sets the color filling new shapes' interior. Please note that if using a
    /// fillcolor with alpha greater than half, drag inside the active shape
    /// starts moving the shape underneath, otherwise a new shape could be
    /// started over.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Determines the path's interior. For more info please
    /// visit <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule>
    #[serde(rename = "fillrule")]
    fill_rule: Option<FillRule>,
    /// Sets the opacity of new shapes. Number between or equal to 0 and 1.
    opacity: Option<f64>,
    /// Specifies whether new shapes are drawn below or above traces.
    layer: Option<ShapeLayer>,
    /// When `dragmode` is set to "drawrect", "drawline" or "drawcircle" this
    /// limits the drag to be horizontal, vertical or diagonal. Using
    /// "diagonal" there is no limit e.g. in drawing lines in any direction.
    /// "ortho" limits the draw to be either horizontal or vertical.
    /// "horizontal" allows horizontal extend. "vertical" allows vertical
    /// extend.
    #[serde(rename = "drawdirection")]
    draw_direction: Option<DrawDirection>,
}

impl NewShape {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct ActiveShape {
    /// Sets the color filling the active shape' interior.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Sets the opacity of the active shape. Number between or equal to 0 and
    /// 1.
    opacity: Option<f64>,
}

impl ActiveShape {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_shape_type() {
        assert_eq!(to_value(ShapeType::Circle).unwrap(), json!("circle"));
        assert_eq!(to_value(ShapeType::Rect).unwrap(), json!("rect"));
        assert_eq!(to_value(ShapeType::Path).unwrap(), json!("path"));
        assert_eq!(to_value(ShapeType::Line).unwrap(), json!("line"));
    }

    #[test]
    fn serialize_shape_layer() {
        assert_eq!(to_value(ShapeLayer::Below).unwrap(), json!("below"));
        assert_eq!(to_value(ShapeLayer::Above).unwrap(), json!("above"));
    }

    #[test]
    fn serialize_shape_size_mode() {
        assert_eq!(to_value(ShapeSizeMode::Scaled).unwrap(), json!("scaled"));
        assert_eq!(to_value(ShapeSizeMode::Pixel).unwrap(), json!("pixel"));
    }

    #[test]
    fn serialize_fill_rule() {
        assert_eq!(to_value(FillRule::EvenOdd).unwrap(), json!("evenodd"));
        assert_eq!(to_value(FillRule::NonZero).unwrap(), json!("nonzero"));
    }

    #[test]
    fn serialize_shape_line() {
        let shape_line = ShapeLine::new()
            .color("#000FFF")
            .width(100.)
            .dash(DashType::LongDashDot);
        let expected = json!({
            "color": "#000FFF",
            "width": 100.0,
            "dash": "longdashdot",
        });

        assert_eq!(to_value(shape_line).unwrap(), expected);
    }

    #[test]
    fn serialize_shape() {
        let shape = Shape::new()
            .visible(false)
            .shape_type(ShapeType::Circle)
            .layer(ShapeLayer::Above)
            .x_ref("xref")
            .x_size_mode(ShapeSizeMode::Pixel)
            .x_anchor(5)
            .x0(7)
            .x1(8)
            .y_ref("paper")
            .y0(1)
            .y1(2)
            .y_anchor("yanchor")
            .y_size_mode(ShapeSizeMode::Scaled)
            .path("path")
            .opacity(0.2)
            .line(ShapeLine::new())
            .fill_color("#FEFEFE")
            .fill_rule(FillRule::NonZero)
            .editable(true)
            .name("name")
            .template_item_name("templateitemname");

        let expected = json!({
            "visible": false,
            "type": "circle",
            "layer": "above",
            "xref": "xref",
            "xsizemode": "pixel",
            "xanchor": 5,
            "x0": 7,
            "x1": 8,
            "yref": "paper",
            "y0": 1,
            "y1": 2,
            "yanchor": "yanchor",
            "ysizemode": "scaled",
            "path": "path",
            "opacity": 0.2,
            "line": {},
            "fillcolor": "#FEFEFE",
            "fillrule": "nonzero",
            "editable": true,
            "name": "name",
            "templateitemname": "templateitemname"
        });

        assert_eq!(to_value(shape).unwrap(), expected)
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_draw_direction() {
        assert_eq!(to_value(DrawDirection::Ortho).unwrap(), json!("ortho"));
        assert_eq!(to_value(DrawDirection::Horizontal).unwrap(), json!("horizontal"));
        assert_eq!(to_value(DrawDirection::Vertical).unwrap(), json!("vertical"));
        assert_eq!(to_value(DrawDirection::Diagonal).unwrap(), json!("diagonal"));
    }

    #[test]
    fn serialize_new_shape() {
        let new_shape = NewShape::new()
            .line(ShapeLine::new())
            .fill_color("#123ABC")
            .fill_rule(FillRule::EvenOdd)
            .opacity(0.02)
            .layer(ShapeLayer::Below)
            .draw_direction(DrawDirection::Ortho);

        let expected = json!({
            "line": {},
            "fillcolor": "#123ABC",
            "fillrule": "evenodd",
            "opacity": 0.02,
            "layer": "below",
            "drawdirection": "ortho",
        });

        assert_eq!(to_value(new_shape).unwrap(), expected)
    }

    #[test]
    fn serialize_active_shape() {
        let active_shape = ActiveShape::new().fill_color("#123ABC").opacity(0.02);

        let expected = json!({
            "fillcolor": "#123ABC",
            "opacity": 0.02,
        });

        assert_eq!(to_value(active_shape).unwrap(), expected);
    }
}
