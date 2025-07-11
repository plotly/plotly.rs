//! Sliders
use plotly_derive::FieldSetter;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{
    color::Color,
    common::{Anchor, Font, Pad},
    layout::{Animation, ControlBuilderError},
    private::NumOrString,
    Relayout, Restyle,
};

/// Sets the Plotly method to be called on for each slider step. If the `skip`
/// method is used, the API updatemenu will function as normal but will perform
/// no API calls and will not bind automatically to state updates. This may be
/// used to create a component interface and attach to updatemenu events
/// manually via JavaScript.
#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SliderMethod {
    /// The restyle method should be used when modifying the data and data
    /// attributes of the graph
    Restyle,
    /// The relayout method should be used when modifying the layout attributes
    /// of the graph.
    Relayout,
    /// The update method should be used when modifying the data and layout
    /// sections of the graph.
    Update,
    /// Animates a sequence of frames
    Animate,
    /// With `skip` method, the API updatemenu will function as normal but will
    /// perform no API calls and will not bind automatically to state updates.
    /// This may be used to create a component interface and attach to
    /// updatemenu events manually via JavaScript.
    Skip,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct SliderStep {
    /// Sets the arguments values to be passed to the Plotly method set in
    /// `method` for each slider step.
    args: Option<Value>,
    /// Sets a 2nd set of `args`, these arguments values are passed to the
    /// Plotly method set in `method` when the slider is moved while in the
    /// active state.
    args2: Option<Value>,
    /// When true, the API method is executed. When false, all other behaviors
    /// are the same and command execution is skipped.
    /// Default: true
    execute: Option<bool>,
    /// Sets the text label to appear on the step.
    label: Option<String>,
    /// Sets the Plotly method to be called on slider step. If the `skip` method
    /// is used, the API updatemenu will function as normal but will perform
    /// no API calls and will not bind automatically to state updates. This
    /// may be used to create a component interface and attach to updatemenu
    /// events manually via JavaScript.
    method: Option<SliderMethod>,
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
    /// Determines whether or not this step is visible.
    visible: Option<bool>,
    /// Sets the value of the slider step, used to determine the current state.
    value: Option<Value>,
}

impl SliderStep {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Builder struct to create slider steps which can do restyles and/or relayouts
#[derive(FieldSetter)]
pub struct SliderStepBuilder {
    label: Option<String>,
    name: Option<String>,
    template_item_name: Option<String>,
    visible: Option<bool>,
    #[field_setter(skip)]
    value: Option<Value>,
    #[field_setter(default = "Map::new()")]
    restyles: Map<String, Value>,
    #[field_setter(default = "Map::new()")]
    relayouts: Map<String, Value>,
    #[field_setter(skip)]
    error: Option<ControlBuilderError>,
    /// Animation configuration
    #[field_setter(skip)]
    animation: Option<Animation>,
}

impl SliderStepBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_restyle(mut self, restyle: impl Restyle) -> Self {
        if self.error.is_none() {
            if let Err(e) = self.try_push_restyle(restyle) {
                self.error = Some(e);
            }
        }
        self
    }

    fn try_push_restyle(&mut self, restyle: impl Restyle) -> Result<(), ControlBuilderError> {
        let restyle_value = serde_json::to_value(&restyle)
            .map_err(|e| ControlBuilderError::RestyleSerializationError(e.to_string()))?;
        let restyle_obj = restyle_value
            .as_object()
            .ok_or_else(|| ControlBuilderError::InvalidRestyleObject(restyle_value.to_string()))?;

        for (k, v) in restyle_obj {
            self.restyles.insert(k.clone(), v.clone());
        }

        Ok(())
    }

    pub fn push_relayout(mut self, relayout: impl Relayout + Serialize) -> Self {
        if self.error.is_none() {
            if let Err(e) = self.try_push_relayout(relayout) {
                self.error = Some(e);
            }
        }

        self
    }

    fn try_push_relayout(
        &mut self,
        relayout: impl Relayout + Serialize,
    ) -> Result<(), ControlBuilderError> {
        let relayout_value = serde_json::to_value(&relayout)
            .map_err(|e| ControlBuilderError::RelayoutSerializationError(e.to_string()))?;
        let relayout_obj = relayout_value.as_object().ok_or_else(|| {
            ControlBuilderError::InvalidRelayoutObject(relayout_value.to_string())
        })?;

        for (k, v) in relayout_obj {
            self.relayouts.insert(k.clone(), v.clone());
        }

        Ok(())
    }

    /// Sets the value of the slider step using any type that can be converted
    /// into NumOrString.
    pub fn value<T: Into<NumOrString>>(mut self, value: T) -> Self {
        if self.error.is_none() {
            if let Err(e) = self.try_set_value(value) {
                self.error = Some(e);
            }
        }

        self
    }

    fn try_set_value<T: Into<NumOrString>>(&mut self, value: T) -> Result<(), ControlBuilderError> {
        let value_json = serde_json::to_value(value.into())
            .map_err(|e| ControlBuilderError::ValueSerializationError(e.to_string()))?;
        self.value = Some(value_json);
        Ok(())
    }

    /// Set the animation configuration for this slider step
    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }
    pub fn build(self) -> Result<SliderStep, ControlBuilderError> {
        if let Some(error) = self.error {
            return Err(error);
        }

        let (method, args) = match (
            self.animation,
            self.restyles.is_empty(),
            self.relayouts.is_empty(),
        ) {
            // Animation takes precedence
            (Some(animation), _, _) => (
                SliderMethod::Animate,
                serde_json::to_value(animation)
                    .map_err(|e| ControlBuilderError::AnimationSerializationError(e.to_string()))?,
            ),
            // Regular restyle/relayout combinations
            (None, true, true) => (SliderMethod::Skip, Value::Null),
            (None, false, true) => (SliderMethod::Restyle, vec![self.restyles].into()),
            (None, true, false) => (SliderMethod::Relayout, vec![self.relayouts].into()),
            (None, false, false) => (
                SliderMethod::Update,
                vec![self.restyles, self.relayouts].into(),
            ),
        };

        Ok(SliderStep {
            label: self.label,
            args: Some(args),
            method: Some(method),
            name: self.name,
            template_item_name: self.template_item_name,
            visible: self.visible,
            value: self.value,
            ..Default::default()
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct Slider {
    /// Determines which slider step (by index starting from 0) is considered
    /// active.
    active: Option<i32>,
    /// Sets the background color of the slider grip while dragging.
    #[serde(rename = "activebgcolor")]
    active_background_color: Option<Box<dyn Color>>,
    /// Sets the background color of the slider.
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    /// Sets the color of the border enclosing the slider.
    #[serde(rename = "bordercolor")]
    border_color: Option<Box<dyn Color>>,
    /// Sets the width (in px) of the border enclosing the slider.
    #[serde(rename = "borderwidth")]
    border_width: Option<usize>,
    /// Sets the current value display configuration.
    #[serde(rename = "currentvalue")]
    current_value: Option<SliderCurrentValue>,
    /// Sets the font of the slider step text.
    font: Option<Font>,
    /// Sets the length of the slider.
    #[serde(rename = "len")]
    length: Option<f64>,
    /// Sets the length in pixels of minor tick marks.
    #[serde(rename = "minorticklen")]
    minor_tick_length: Option<usize>,
    /// When used in a template, named items are created in the output figure in
    /// addition to any items the figure already has in this array.
    name: Option<String>,
    /// Sets the padding around the slider.
    pad: Option<Pad>,
    /// Sets the slider steps.
    steps: Option<Vec<SliderStep>>,
    /// Used to refer to a named item in this array in the template.
    #[serde(rename = "templateitemname")]
    template_item_name: Option<String>,
    /// Sets the color of the border enclosing the slider.
    #[serde(rename = "tickcolor")]
    tick_color: Option<Box<dyn Color>>,
    /// Sets the length in pixels of step tick marks.
    #[serde(rename = "ticklen")]
    tick_length: Option<usize>,
    /// Sets the tick width (in px).
    #[serde(rename = "tickwidth")]
    tick_width: Option<usize>,
    /// Sets the transition configuration.
    transition: Option<SliderTransition>,
    /// Determines whether or not the slider is visible.
    visible: Option<bool>,
    /// Sets the x position (in normalized coordinates) of the slider.
    x: Option<f64>,
    /// Sets the slider's horizontal position anchor.
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    /// Sets the y position (in normalized coordinates) of the slider.
    y: Option<f64>,
    /// Sets the slider's vertical position anchor.
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
}

impl Slider {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configuration for the current value display of a slider.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct SliderCurrentValue {
    /// Sets the font of the current value label text.
    font: Option<Font>,
    /// The amount of space, in pixels, between the current value label and the
    /// slider.
    offset: Option<usize>,
    /// When currentvalue.visible is True, this sets the prefix of the label.
    prefix: Option<String>,
    /// When currentvalue.visible is True, this sets the suffix of the label.
    suffix: Option<String>,
    /// Shows the currently-selected value above the slider.
    visible: Option<bool>,
    /// The alignment of the value readout relative to the length of the slider.
    #[serde(rename = "xanchor")]
    x_anchor: Option<SliderCurrentValueXAnchor>,
}

impl SliderCurrentValue {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Configuration for slider transitions.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, FieldSetter, Clone)]
pub struct SliderTransition {
    /// Sets the duration of the slider transition.
    duration: Option<usize>,
    /// Sets the easing function of the slider transition.
    easing: Option<SliderTransitionEasing>,
}

impl SliderTransition {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SliderCurrentValueXAnchor {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SliderTransitionEasing {
    Linear,
    Quad,
    Cubic,
    Sin,
    Exp,
    Circle,
    Elastic,
    Back,
    Bounce,
    #[serde(rename = "linear-in")]
    LinearIn,
    #[serde(rename = "quad-in")]
    QuadIn,
    #[serde(rename = "cubic-in")]
    CubicIn,
    #[serde(rename = "sin-in")]
    SinIn,
    #[serde(rename = "exp-in")]
    ExpIn,
    #[serde(rename = "circle-in")]
    CircleIn,
    #[serde(rename = "elastic-in")]
    ElasticIn,
    #[serde(rename = "back-in")]
    BackIn,
    #[serde(rename = "bounce-in")]
    BounceIn,
    #[serde(rename = "linear-out")]
    LinearOut,
    #[serde(rename = "quad-out")]
    QuadOut,
    #[serde(rename = "cubic-out")]
    CubicOut,
    #[serde(rename = "sin-out")]
    SinOut,
    #[serde(rename = "exp-out")]
    ExpOut,
    #[serde(rename = "circle-out")]
    CircleOut,
    #[serde(rename = "elastic-out")]
    ElasticOut,
    #[serde(rename = "back-out")]
    BackOut,
    #[serde(rename = "bounce-out")]
    BounceOut,
    #[serde(rename = "linear-in-out")]
    LinearInOut,
    #[serde(rename = "quad-in-out")]
    QuadInOut,
    #[serde(rename = "cubic-in-out")]
    CubicInOut,
    #[serde(rename = "sin-in-out")]
    SinInOut,
    #[serde(rename = "exp-in-out")]
    ExpInOut,
    #[serde(rename = "circle-in-out")]
    CircleInOut,
    #[serde(rename = "elastic-in-out")]
    ElasticInOut,
    #[serde(rename = "back-in-out")]
    BackInOut,
    #[serde(rename = "bounce-in-out")]
    BounceInOut,
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::{Anchor, Visible};
    use crate::layout::{Animation, AnimationMode, FrameSettings, TransitionSettings};

    #[test]
    fn serialize_slider_method() {
        let test_cases = [
            (SliderMethod::Restyle, "restyle"),
            (SliderMethod::Relayout, "relayout"),
            (SliderMethod::Animate, "animate"),
            (SliderMethod::Update, "update"),
            (SliderMethod::Skip, "skip"),
        ];

        for (method, expected) in test_cases {
            assert_eq!(
                to_value(method).unwrap(),
                json!(expected),
                "Failed for {:?}",
                method
            );
        }
    }

    #[test]
    fn serialize_slider_step() {
        let slider_step = SliderStep::new()
            .args(json!([
                { "visible": [true, false] },
                { "width": 20},
            ]))
            .args2(json!([]))
            .execute(true)
            .label("Step Label")
            .method(SliderMethod::Update)
            .name("Step Name")
            .template_item_name("Template")
            .visible(true)
            .value(json!("step_value"));

        let expected = json!({
            "args": [
                { "visible": [true, false] },
                { "width": 20},
            ],
            "args2": [],
            "execute": true,
            "label": "Step Label",
            "method": "update",
            "name": "Step Name",
            "templateitemname": "Template",
            "visible": true,
            "value": "step_value",
        });

        assert_eq!(to_value(slider_step).unwrap(), expected);
    }

    #[test]
    fn serialize_slider() {
        let slider = Slider::new()
            .active(0)
            .active_background_color("#FF0000")
            .background_color("#123ABC")
            .border_color("#ABC123")
            .border_width(2)
            .current_value(
                SliderCurrentValue::new()
                    .font(Font::new().size(12))
                    .offset(10)
                    .prefix("Value: ")
                    .suffix(" units")
                    .visible(true)
                    .x_anchor(SliderCurrentValueXAnchor::Left),
            )
            .font(Font::new())
            .length(0.8)
            .minor_tick_length(3)
            .name("Test Slider")
            .pad(Pad::new(10, 10, 10))
            .steps(vec![SliderStep::new()])
            .template_item_name("Template")
            .tick_color("#333")
            .tick_length(7)
            .tick_width(1)
            .transition(
                SliderTransition::new()
                    .duration(150)
                    .easing(SliderTransitionEasing::CubicInOut),
            )
            .visible(true)
            .x(0.1)
            .x_anchor(Anchor::Left)
            .y(0.9)
            .y_anchor(Anchor::Top);

        let expected = json!({
            "active": 0,
            "activebgcolor": "#FF0000",
            "bgcolor": "#123ABC",
            "bordercolor": "#ABC123",
            "borderwidth": 2,
            "currentvalue": {
                "font": {"size": 12},
                "offset": 10,
                "prefix": "Value: ",
                "suffix": " units",
                "visible": true,
                "xanchor": "left"
            },
            "font": {},
            "len": 0.8,
            "minorticklen": 3,
            "name": "Test Slider",
            "pad": {"t": 10, "b": 10, "l": 10},
            "steps": [{}],
            "templateitemname": "Template",
            "tickcolor": "#333",
            "ticklen": 7,
            "tickwidth": 1,
            "transition": {
                "duration": 150,
                "easing": "cubic-in-out"
            },
            "visible": true,
            "x": 0.1,
            "xanchor": "left",
            "y": 0.9,
            "yanchor": "top",
        });

        assert_eq!(to_value(slider).unwrap(), expected);
    }

    #[test]
    fn serialize_slider_current_value_x_anchor() {
        let test_cases = [
            (SliderCurrentValueXAnchor::Left, "left"),
            (SliderCurrentValueXAnchor::Center, "center"),
            (SliderCurrentValueXAnchor::Right, "right"),
        ];

        for (anchor, expected) in test_cases {
            assert_eq!(
                to_value(&anchor).unwrap(),
                json!(expected),
                "Failed for {:?}",
                anchor
            );
        }
    }

    #[test]
    fn serialize_slider_transition_easing() {
        let test_cases = [
            (SliderTransitionEasing::Linear, "linear"),
            (SliderTransitionEasing::CubicInOut, "cubic-in-out"),
            (SliderTransitionEasing::BounceIn, "bounce-in"),
        ];

        for (easing, expected) in test_cases {
            assert_eq!(
                to_value(&easing).unwrap(),
                json!(expected),
                "Failed for {:?}",
                easing
            );
        }
    }

    #[test]
    fn serialize_slider_step_builder_default() {
        let slider_step = SliderStepBuilder::new().build().unwrap();

        let expected = json!({
            "args": null,
            "method": "skip",
        });

        assert_eq!(to_value(slider_step).unwrap(), expected);
    }

    #[test]
    fn serialize_slider_step_builder() {
        let slider_step = SliderStepBuilder::new()
            .label("Test Step")
            .name("test_step")
            .template_item_name("template")
            .visible(true)
            .build()
            .unwrap();

        let expected = json!({
            "args": null,
            "method": "skip",
            "label": "Test Step",
            "name": "test_step",
            "templateitemname": "template",
            "visible": true,
        });

        assert_eq!(to_value(slider_step).unwrap(), expected);
    }

    #[test]
    fn serialize_valid_push_restyle() {
        let slider_step = SliderStepBuilder::new()
            .push_restyle(crate::Bar::<i32, i32>::modify_visible(vec![
                Visible::True,
                Visible::False,
            ]))
            .build()
            .unwrap();

        let expected = json!({
            "args": [{"visible": [true, false]}],
            "method": "restyle",
        });

        assert_eq!(to_value(slider_step).unwrap(), expected);
    }

    #[test]
    fn serialize_valid_push_relayout() {
        let slider_step = SliderStepBuilder::new()
            .push_relayout(crate::layout::Layout::modify_title("Hello"))
            .build()
            .unwrap();

        let expected = json!({
            "args": [{"title": {"text": "Hello"}}],
            "method": "relayout",
        });

        assert_eq!(to_value(slider_step).unwrap(), expected);
    }

    #[test]
    fn invalid_push_restyle() {
        // Dummy type that implements Restyle but serializes to a non-object
        #[derive(serde::Serialize)]
        struct InvalidJsonObject;
        impl Restyle for InvalidJsonObject {}

        let builder = SliderStepBuilder::new().push_restyle(InvalidJsonObject);
        let err = builder.build().unwrap_err();
        match err {
            ControlBuilderError::InvalidRestyleObject(s) => {
                assert!(s.contains("null"));
            }
            _ => panic!("Expected InvalidRestyleObject error"),
        }
    }

    #[test]
    fn invalid_push_relayout() {
        // Dummy type that implements Relayout but serializes to a non-object
        #[derive(serde::Serialize)]
        struct InvalidJsonObject;
        impl Relayout for InvalidJsonObject {}

        let result = SliderStepBuilder::new()
            .label("Test")
            .push_relayout(InvalidJsonObject)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            ControlBuilderError::InvalidRelayoutObject(_) => {}
            _ => panic!("Expected InvalidRelayoutObject error"),
        }
    }

    #[test]
    fn serialize_slider_step_builder_with_animation() {
        let animation = Animation::frames(vec!["frame1".to_string()]).options(
            crate::layout::AnimationOptions::new()
                .mode(AnimationMode::Immediate)
                .frame(FrameSettings::new().duration(300).redraw(false))
                .transition(TransitionSettings::new().duration(300)),
        );

        let slider_step = SliderStepBuilder::new()
            .label("Animate to frame1")
            .value("frame1")
            .animation(animation)
            .build()
            .unwrap();

        let expected = json!({
            "args": [
                ["frame1"],
                {
                    "mode": "immediate",
                    "transition": {"duration": 300},
                    "frame": {"duration": 300, "redraw": false}
                }
            ],
            "method": "animate",
            "label": "Animate to frame1",
            "value": "frame1"
        });

        assert_eq!(to_value(slider_step).unwrap(), expected);
    }
}
