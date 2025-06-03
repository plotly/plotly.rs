use plotly_derive::FieldSetter;
use serde::{Serialize, Serializer};

use crate::color::Color;
use crate::common::{Anchor, Font, Label};
use crate::private::NumOrString;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ArrowSide {
    End,
    Start,
    #[serde(rename = "end+start")]
    StartEnd,
    None,
}

#[derive(Debug, Clone)]
pub enum ClickToShow {
    False,
    OnOff,
    OnOut,
}

impl Serialize for ClickToShow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::False => serializer.serialize_bool(false),
            Self::OnOff => serializer.serialize_str("onoff"),
            Self::OnOut => serializer.serialize_str("onout"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Annotation {
    /// Determines whether or not this annotation is visible.
    visible: Option<bool>,
    /// Sets the text associated with this annotation. Plotly uses a subset of
    /// HTML tags to do things like newline (<br>), bold (<b></b>), italics
    /// (<i></i>), hyperlinks (<a href='...'></a>). Tags <em></em>, <sup></sup>,
    /// <sub></sub> <span></span> are also supported.
    text: Option<String>,
    /// Sets the angle at which the `text` is drawn with respect to the
    /// horizontal.
    #[serde(rename = "textangle")]
    text_angle: Option<f64>,
    /// Sets the annotation text font.
    font: Option<Font>,
    /// Sets an explicit width for the text box. null (default) lets the text
    /// set the box width. Wider text will be clipped. There is no automatic
    /// wrapping; use <br> to start a new line.
    width: Option<f64>,
    /// Sets an explicit height for the text box. null (default) lets the text
    /// set the box height. Taller text will be clipped.
    height: Option<f64>,
    /// Sets the opacity of the annotation (text + arrow).
    opacity: Option<f64>,
    /// Sets the horizontal alignment of the `text` within the box. Has an
    /// effect only if `text` spans two or more lines (i.e. `text` contains
    /// one or more <br> HTML tags) or if an explicit width is set to
    /// override the text width.
    align: Option<HAlign>,
    /// Sets the vertical alignment of the `text` within the box. Has an effect
    /// only if an explicit height is set to override the text height.
    valign: Option<VAlign>,
    /// Sets the background color of the annotation.
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    /// Sets the color of the border enclosing the annotation `text`.
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    /// Sets the padding (in px) between the `text` and the enclosing border.
    #[serde(rename = "borderpad")]
    border_pad: Option<f64>,
    /// Sets the width (in px) of the border enclosing the annotation `text`.
    #[serde(rename = "borderwidth")]
    border_width: Option<f64>,
    /// Determines whether or not the annotation is drawn with an arrow. If
    /// "True", `text` is placed near the arrow's tail. If "False", `text`
    /// lines up with the `x` and `y` provided.
    #[serde(rename = "showarrow")]
    show_arrow: Option<bool>,
    /// Sets the color of the annotation arrow.
    #[serde(rename = "arrowcolor")]
    arrow_color: Option<Box<dyn Color>>,
    /// Sets the end annotation arrow head style. Integer between or equal to 0
    /// and 8.
    #[serde(rename = "arrowhead")]
    arrow_head: Option<u8>,
    /// Sets the start annotation arrow head style. Integer between or equal to
    /// 0 and 8.
    #[serde(rename = "startarrowhead")]
    start_arrow_head: Option<u8>,
    /// Sets the annotation arrow head position.
    #[serde(rename = "arrowside")]
    arrow_side: Option<ArrowSide>,
    /// Sets the size of the end annotation arrow head, relative to
    /// `arrowwidth`. A value of 1 (default) gives a head about 3x as wide
    /// as the line.
    #[serde(rename = "arrowsize")]
    arrow_size: Option<f64>,
    /// Sets the size of the start annotation arrow head, relative to
    /// `arrowwidth`. A value of 1 (default) gives a head about 3x as wide
    /// as the line.
    #[serde(rename = "startarrowsize")]
    start_arrow_size: Option<f64>,
    /// Sets the width (in px) of annotation arrow line.
    #[serde(rename = "arrowwidth")]
    arrow_width: Option<f64>,
    /// Sets a distance, in pixels, to move the end arrowhead away from the
    /// position it is pointing at, for example to point at the edge of a
    /// marker independent of zoom. Note that this shortens the arrow from
    /// the `ax` / `ay` vector, in contrast to `xshift` / `yshift` which
    /// moves everything by this amount.
    #[serde(rename = "standoff")]
    stand_off: Option<f64>,
    /// Sets a distance, in pixels, to move the start arrowhead away from the
    /// position it is pointing at, for example to point at the edge of a
    /// marker independent of zoom. Note that this shortens the arrow from
    /// the `ax` / `ay` vector, in contrast to `xshift` / `yshift`
    /// which moves everything by this amount.
    #[serde(rename = "startstandoff")]
    start_stand_off: Option<f64>,
    /// Sets the x component of the arrow tail about the arrow head. If `axref`
    /// is `pixel`, a positive (negative) component corresponds to an arrow
    /// pointing from right to left (left to right). If `axref` is an axis,
    /// this is an absolute value on that axis, like `x`, NOT a
    /// relative value.
    ax: Option<NumOrString>,
    /// Sets the y component of the arrow tail about the arrow head. If `ayref`
    /// is `pixel`, a positive (negative) component corresponds to an arrow
    /// pointing from bottom to top (top to bottom). If `ayref` is an axis,
    /// this is an absolute value on that axis, like `y`, NOT a
    /// relative value.
    ay: Option<NumOrString>,
    /// Indicates in what terms the tail of the annotation (ax,ay) is specified.
    /// If `pixel`, `ax` is a relative offset in pixels from `x`. If set to
    /// an x axis id (e.g. "x" or "x2"), `ax` is specified in the same terms
    /// as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    #[serde(rename = "axref")]
    ax_ref: Option<String>,
    /// Indicates in what terms the tail of the annotation (ax,ay) is specified.
    /// If `pixel`, `ay` is a relative offset in pixels from `y`. If set to
    /// a y axis id (e.g. "y" or "y2"), `ay` is specified in the same terms
    /// as that axis. This is useful for trendline annotations which
    /// should continue to indicate the correct trend when zoomed.
    #[serde(rename = "ayref")]
    ay_ref: Option<String>,
    /// Sets the annotation's x coordinate axis. If set to an x axis id (e.g.
    /// "x" or "x2"), the `x` position refers to an x coordinate If set to
    /// "paper", the `x` position refers to the distance from the left side
    /// of the plotting area in normalized coordinates where 0 (1)
    /// corresponds to the left (right) side.
    #[serde(rename = "xref")]
    x_ref: Option<String>,
    /// Sets the annotation's x position. If the axis `type` is "log", then you
    /// must take the log of your desired range. If the axis `type` is
    /// "date", it should be date strings, like date data, though Date
    /// objects and unix milliseconds will be accepted and converted to strings.
    /// If the axis `type` is "category", it should be numbers, using the scale
    /// where each category is assigned a serial number from zero in the
    /// order it appears.
    x: Option<NumOrString>,
    /// Sets the text box's horizontal position anchor This anchor binds the `x`
    /// position to the "left", "center" or "right" of the annotation. For
    /// example, if `x` is set to 1, `xref` to "paper" and `xanchor` to
    /// "right" then the right-most portion of the annotation lines up with
    /// the right-most edge of the plotting area. If "auto", the anchor is
    /// equivalent to "center" for data-referenced annotations or if there
    /// is an arrow, whereas for paper-referenced with no arrow, the anchor
    /// picked corresponds to the closest side.
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    /// Shifts the position of the whole annotation and arrow to the right
    /// (positive) or left (negative) by this many pixels.
    #[serde(rename = "xshift")]
    x_shift: Option<f64>,
    /// Sets the annotation's y coordinate axis. If set to an y axis id (e.g.
    /// "y" or "y2"), the `y` position refers to an y coordinate If set to
    /// "paper", the `y` position refers to the distance from the bottom of
    /// the plotting area in normalized coordinates where 0 (1) corresponds
    /// to the bottom (top).
    #[serde(rename = "yref")]
    y_ref: Option<String>,
    /// Sets the annotation's y position. If the axis `type` is "log", then you
    /// must take the log of your desired range. If the axis `type` is
    /// "date", it should be date strings, like date data, though Date
    /// objects and unix milliseconds will be accepted and converted to strings.
    /// If the axis `type` is "category", it should be numbers, using the
    /// scale where each category is assigned a serial number from zero in
    /// the order it appears.
    y: Option<NumOrString>,
    /// Sets the text box's vertical position anchor This anchor binds the `y`
    /// position to the "top", "middle" or "bottom" of the annotation. For
    /// example, if `y` is set to 1, `yref` to "paper" and `yanchor` to
    /// "top" then the top-most portion of the annotation lines up with the
    /// top-most edge of the plotting area. If "auto", the anchor is equivalent
    /// to "middle" for data-referenced annotations or if there is an arrow,
    /// whereas for paper-referenced with no arrow, the anchor picked
    /// corresponds to the closest side.
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    /// Shifts the position of the whole annotation and arrow up (positive) or
    /// down (negative) by this many pixels.
    #[serde(rename = "yshift")]
    y_shift: Option<f64>,
    /// Makes this annotation respond to clicks on the plot. If you click a data
    /// point that exactly matches the `x` and `y` values of this
    /// annotation, and it is hidden (visible: false), it will appear. In
    /// "onoff" mode, you must click the same point again to make it disappear,
    /// so if you click multiple points, you can show multiple annotations.
    /// In "onout" mode, a click anywhere else in the plot (on another data
    /// point or not) will hide this annotation. If you need to show/hide
    /// this annotation in response to different `x` or `y` values, you can set
    /// `xclick` and/or `yclick`. This is useful for example to label the side
    /// of a bar. To label markers though, `standoff` is preferred over
    /// `xclick` and `yclick`.
    #[serde(rename = "clicktoshow")]
    click_to_show: Option<ClickToShow>,
    /// Toggle this annotation when clicking a data point whose `x` value is
    /// `xclick` rather than the annotation's `x` value.
    #[serde(rename = "xclick")]
    x_click: Option<NumOrString>,
    /// Toggle this annotation when clicking a data point whose `y` value is
    /// `yclick` rather than the annotation's `y` value.
    #[serde(rename = "yclick")]
    y_click: Option<NumOrString>,
    /// Sets text to appear when hovering over this annotation. If omitted or
    /// blank, no hover label will appear.
    #[serde(rename = "hovertext")]
    hover_text: Option<String>,
    /// Label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Determines whether the annotation text box captures mouse move and click
    /// events, or allows those events to pass through to data points in the
    /// plot that may be behind the annotation. By default `captureevents`
    /// is "false" unless `hovertext` is provided. If you use the event
    /// `plotly_clickannotation` without `hovertext` you must explicitly enable
    /// `captureevents`.
    #[serde(rename = "captureevents")]
    capture_events: Option<bool>,
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

impl Annotation {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};

    use crate::common::{Anchor, Font, Label};

    #[test]
    fn serialize_valign() {
        assert_eq!(to_value(VAlign::Top).unwrap(), json!("top"));
        assert_eq!(to_value(VAlign::Middle).unwrap(), json!("middle"));
        assert_eq!(to_value(VAlign::Bottom).unwrap(), json!("bottom"));
    }

    #[test]
    fn serialize_halign() {
        assert_eq!(to_value(HAlign::Left).unwrap(), json!("left"));
        assert_eq!(to_value(HAlign::Center).unwrap(), json!("center"));
        assert_eq!(to_value(HAlign::Right).unwrap(), json!("right"));
    }

    #[test]
    fn serialize_click_to_show() {
        assert_eq!(to_value(ClickToShow::False).unwrap(), json!(false));
        assert_eq!(to_value(ClickToShow::OnOff).unwrap(), json!("onoff"));
        assert_eq!(to_value(ClickToShow::OnOut).unwrap(), json!("onout"));
    }

    #[test]
    fn serialize_arrow_side() {
        assert_eq!(to_value(ArrowSide::End).unwrap(), json!("end"));
        assert_eq!(to_value(ArrowSide::Start).unwrap(), json!("start"));
        assert_eq!(to_value(ArrowSide::StartEnd).unwrap(), json!("end+start"));
        assert_eq!(to_value(ArrowSide::None).unwrap(), json!("none"));
    }

    #[test]
    fn serialize_annotation() {
        let annotation = Annotation::new()
            .align(HAlign::Center)
            .arrow_color("#464646")
            .arrow_head(2)
            .arrow_size(123.4)
            .arrow_side(ArrowSide::End)
            .arrow_width(111.1)
            .ax("ax")
            .ax_ref("axref")
            .ay("ay")
            .ay_ref("ayref")
            .background_color("#123456")
            .border_color("#456789")
            .border_pad(500.)
            .border_width(1000.)
            .capture_events(false)
            .click_to_show(ClickToShow::OnOff)
            .font(Font::new())
            .height(6.)
            .hover_label(Label::new())
            .hover_text("hovertext")
            .name("name")
            .opacity(0.01)
            .show_arrow(false)
            .stand_off(999.9)
            .start_arrow_head(0)
            .start_stand_off(8.8)
            .start_arrow_size(456.7)
            .template_item_name("templateitemname")
            .text("text")
            .text_angle(5.)
            .valign(VAlign::Middle)
            .visible(true)
            .width(4.)
            .x_ref("xref")
            .x("x")
            .x_anchor(Anchor::Auto)
            .x_click("xclick")
            .x_shift(4.0)
            .y_ref("yref")
            .y("y")
            .y_anchor(Anchor::Bottom)
            .y_click("yclick")
            .y_shift(6.3);

        let expected = json!({
            "visible": true,
            "text": "text",
            "textangle": 5.0,
            "font": {},
            "width": 4.0,
            "height": 6.0,
            "opacity": 0.01,
            "align": "center",
            "valign": "middle",
            "bgcolor": "#123456",
            "bordercolor": "#456789",
            "borderpad": 500.0,
            "borderwidth": 1000.0,
            "showarrow": false,
            "arrowcolor": "#464646",
            "arrowhead": 2,
            "startarrowhead": 0,
            "arrowside": "end",
            "arrowsize": 123.4,
            "startarrowsize": 456.7,
            "arrowwidth": 111.1,
            "standoff": 999.9,
            "startstandoff": 8.8,
            "ax": "ax",
            "ay": "ay",
            "x": "x",
            "y": "y",
            "axref": "axref",
            "ayref": "ayref",
            "xref": "xref",
            "yref": "yref",
            "xanchor": "auto",
            "yanchor": "bottom",
            "xshift": 4.0,
            "yshift": 6.3,
            "clicktoshow": "onoff",
            "xclick": "xclick",
            "yclick": "yclick",
            "hovertext": "hovertext",
            "hoverlabel": {},
            "captureevents": false,
            "name": "name",
            "templateitemname": "templateitemname",
        });

        assert_eq!(to_value(annotation).unwrap(), expected);
    }
}
