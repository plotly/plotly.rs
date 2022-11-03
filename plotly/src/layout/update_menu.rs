//! Buttons and Dropdowns.

use plotly_derive::FieldSetter;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{
    color::Color,
    common::{Anchor, Font, Pad},
    Relayout, Restyle,
};

/// Sets the Plotly method to be called on click. If the `skip` method is used,
/// the API updatemenu will function as normal but will perform no API calls and
/// will not bind automatically to state updates. This may be used to create a
/// component interface and attach to updatemenu events manually via JavaScript.
#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ButtonMethod {
    /// The restyle method should be used when modifying the data and data
    /// attributes of the graph
    Restyle,
    /// The relayout method should be used when modifying the layout attributes
    /// of the graph.
    Relayout,
    Animate,
    /// The update method should be used when modifying the data and layout
    /// sections of the graph.
    Update,
    Skip,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Button {
    /// Sets the arguments values to be passed to the Plotly method set in
    /// `method` on click.
    args: Option<Value>,
    /// Sets a 2nd set of `args`, these arguments values are passed to the
    /// Plotly method set in `method` when clicking this button while in the
    /// active state. Use this to create toggle buttons.
    args2: Option<Value>,
    /// When true, the API method is executed. When false, all other behaviors
    /// are the same and command execution is skipped. This may be useful
    /// when hooking into, for example, the `plotly_buttonclicked` method
    /// and executing the API command manually without losing the benefit of
    /// the updatemenu automatically binding to the state of the plot through
    /// the specification of `method` and `args`.
    ///
    /// Default: true
    execute: Option<bool>,
    /// Sets the text label to appear on the button.
    label: Option<String>,
    /// Sets the Plotly method to be called on click. If the `skip` method is
    /// used, the API updatemenu will function as normal but will perform no
    /// API calls and will not bind automatically to state updates. This may
    /// be used to create a component interface and attach to updatemenu
    /// events manually via JavaScript.
    method: Option<ButtonMethod>,
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
    /// explicitly show it with `visible: true`
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
    /// Determines whether or not this button is visible.
    visible: Option<bool>,
}

impl Button {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Builder struct to create buttons which can do restyles and/or relayouts
#[derive(FieldSetter)]
pub struct ButtonBuilder {
    label: Option<String>,
    name: Option<String>,
    template_item_name: Option<String>,
    visible: Option<bool>,
    #[field_setter(default = "Map::new()")]
    restyles: Map<String, Value>,
    #[field_setter(default = "Map::new()")]
    relayouts: Map<String, Value>,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn push_restyle(mut self, restyle: impl Restyle + Serialize) -> Self {
        let restyle = serde_json::to_value(&restyle).unwrap();
        for (k, v) in restyle.as_object().unwrap() {
            self.restyles.insert(k.clone(), v.clone());
        }
        self
    }

    pub fn push_relayout(mut self, relayout: impl Relayout + Serialize) -> Self {
        let relayout = serde_json::to_value(&relayout).unwrap();
        for (k, v) in relayout.as_object().unwrap() {
            self.relayouts.insert(k.clone(), v.clone());
        }
        self
    }

    fn method_and_args(
        restyles: Map<String, Value>,
        relayouts: Map<String, Value>,
    ) -> (ButtonMethod, Value) {
        match (restyles.is_empty(), relayouts.is_empty()) {
            (true, true) => (ButtonMethod::Skip, Value::Null),
            (false, true) => (ButtonMethod::Restyle, vec![restyles].into()),
            (true, false) => (ButtonMethod::Relayout, vec![relayouts].into()),
            (false, false) => (ButtonMethod::Update, vec![restyles, relayouts].into()),
        }
    }

    pub fn build(self) -> Button {
        let (method, args) = Self::method_and_args(self.restyles, self.relayouts);
        Button {
            label: self.label,
            args: Some(args),
            method: Some(method),
            name: self.name,
            template_item_name: self.template_item_name,
            visible: self.visible,
            ..Default::default()
        }
    }
}

/// Determines whether the buttons are accessible via a dropdown menu or whether
/// the buttons are stacked horizontally or vertically
///
/// Default: "dropdown"
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UpdateMenuType {
    Dropdown,
    Buttons,
}

/// Determines the direction in which the buttons are laid out, whether in a
/// dropdown menu or a row/column of buttons. For `left` and `up`, the buttons
/// will still appear in left-to-right or top-to-bottom order respectively.
///
/// Default: "down"
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UpdateMenuDirection {
    Left,
    Right,
    Up,
    Down,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct UpdateMenu {
    /// Determines which button (by index starting from 0) is considered active.
    active: Option<i32>,
    /// Sets the background color of the update menu buttons.
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    /// Sets the color of the border enclosing the update menu.
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    /// Sets the width (in px) of the border enclosing the update menu.
    #[serde(rename = "borderwidth")]
    border_width: Option<usize>,
    buttons: Option<Vec<Button>>,
    /// Determines the direction in which the buttons are laid out, whether in
    /// a dropdown menu or a row/column of buttons. For `left` and `up`,
    /// the buttons will still appear in left-to-right or top-to-bottom order
    /// respectively.
    direction: Option<UpdateMenuDirection>,
    /// Sets the font of the update menu button text.
    font: Option<Font>,
    /// When used in a template, named items are created in the output figure in
    /// addition to any items the figure already has in this array. You can
    /// modify these items in the output figure by making your own item with
    /// `templateitemname` matching this `name` alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). Has no
    /// effect outside of a template.
    name: Option<String>,
    /// Sets the padding around the buttons or dropdown menu.
    pad: Option<Pad>,
    /// Highlights active dropdown item or active button if true.
    #[serde(rename = "showactive")]
    show_active: Option<bool>,
    /// Used to refer to a named item in this array in the template. Named items
    /// from the template will be created even without a matching item in
    /// the input figure, but you can modify one by making an item with
    /// `templateitemname` matching its `name`, alongside your modifications
    /// (including `visible: false` or `enabled: false` to hide it). If there is
    /// no template or no matching item, this item will be hidden unless you
    /// explicitly show it with `visible: true`.
    template_item_name: Option<String>,
    /// Determines whether the buttons are accessible via a dropdown menu or
    /// whether the buttons are stacked horizontally or vertically
    #[serde(rename = "type")]
    ty: Option<UpdateMenuType>,
    /// Determines whether or not the update menu is visible.
    visible: Option<bool>,
    /// Type: number between or equal to -2 and 3
    /// Default: -0.05
    /// Sets the x position (in normalized coordinates) of the update menu.
    x: Option<f64>,
    /// Sets the update menu's horizontal position anchor. This anchor binds the
    /// `x` position to the "left", "center" or "right" of the range
    /// selector. Default: "right"
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    /// Type: number between or equal to -2 and 3
    /// Default: 1
    /// Sets the y position (in normalized coordinates) of the update menu.
    y: Option<f64>,
    /// Sets the update menu's vertical position anchor This anchor binds the
    /// `y` position to the "top", "middle" or "bottom" of the range
    /// selector. Default: "top"
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
}

impl UpdateMenu {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::{
        common::{Title, Visible},
        Layout,
    };

    #[test]
    fn test_serialize_button_method() {
        assert_eq!(to_value(ButtonMethod::Restyle).unwrap(), json!("restyle"));
        assert_eq!(to_value(ButtonMethod::Relayout).unwrap(), json!("relayout"));
        assert_eq!(to_value(ButtonMethod::Animate).unwrap(), json!("animate"));
        assert_eq!(to_value(ButtonMethod::Update).unwrap(), json!("update"));
        assert_eq!(to_value(ButtonMethod::Skip).unwrap(), json!("skip"));
    }

    #[test]
    fn test_serialize_button() {
        let button = Button::new()
            .args(json!([
                { "visible": [true, false] },
                { "width": 20},
            ]))
            .args2(json!([]))
            .execute(true)
            .label("Label")
            .method(ButtonMethod::Update)
            .name("Name")
            .template_item_name("Template")
            .visible(true);

        let expected = json!({
            "args": [
                { "visible": [true, false] },
                { "width": 20},
            ],
            "args2": [],
            "execute": true,
            "label": "Label",
            "method": "update",
            "name": "Name",
            "templateitemname": "Template",
            "visible": true,
        });

        assert_eq!(to_value(button).unwrap(), expected);
    }

    #[test]
    fn test_button_builder() {
        let expected = json!({
            "args": [
                { "visible": [true, false] },
                { "title": {"text": "Hello"}, "width": 20},
            ],
            "label": "Label",
            "method": "update",
            "name": "Name",
            "templateitemname": "Template",
            "visible": true,
        });

        let button = ButtonBuilder::new()
            .label("Label")
            .name("Name")
            .template_item_name("Template")
            .visible(true)
            .push_restyle(crate::Bar::<i32, i32>::modify_visible(vec![
                Visible::True,
                Visible::False,
            ]))
            .push_relayout(Layout::modify_title(Title::new("Hello")))
            .push_relayout(Layout::modify_width(20))
            .build();

        assert_eq!(to_value(button).unwrap(), expected);
    }
}
