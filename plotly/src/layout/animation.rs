//! Animation support for Plotly.rs
//!
//! This module provides animation configuration for Plotly.js updatemenu
//! buttons and slider steps, following the Plotly.js animation API
//! specification.

use plotly_derive::FieldSetter;
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

use crate::{Layout, Traces};

/// A frame represents a single state in an animation sequence.
/// Based on Plotly.js frame_attributes.js specification
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, FieldSetter)]
pub struct Frame {
    /// An identifier that specifies the group to which the frame belongs,
    /// used by animate to select a subset of frames
    group: Option<String>,
    /// A label by which to identify the frame
    name: Option<String>,
    /// A list of trace indices that identify the respective traces in the data
    /// attribute
    traces: Option<Vec<usize>>,
    /// The name of the frame into which this frame's properties are merged
    /// before applying. This is used to unify properties and avoid needing
    /// to specify the same values for the same properties in multiple
    /// frames.
    baseframe: Option<String>,
    /// A list of traces this frame modifies. The format is identical to the
    /// normal trace definition.
    data: Option<Traces>,
    /// Layout properties which this frame modifies. The format is identical to
    /// the normal layout definition.
    layout: Option<Layout>,
}

impl Frame {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Represents the animation arguments array for Plotly.js
/// Format: [frameNamesOrNull, animationOptions]
#[derive(Clone, Debug)]
pub struct Animation {
    /// Frames sequence: null, [null], or array of frame names
    frames: FrameListMode,
    /// Animation options/configuration
    options: AnimationOptions,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            frames: FrameListMode::All,
            options: AnimationOptions::default(),
        }
    }
}

impl Animation {
    /// Create a new animation args with default values for options and
    /// FramesMode set to all frames
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a animation for playing all frames (default)    
    pub fn all_frames() -> Self {
        Self::new()
    }

    /// Create a animation setup specifically for pausing a running animation
    pub fn pause() -> Self {
        Self {
            frames: FrameListMode::Pause,
            options: AnimationOptions::new()
                .mode(AnimationMode::Immediate)
                .frame(FrameSettings::new().duration(0).redraw(false))
                .transition(TransitionSettings::new().duration(0)),
        }
    }

    /// Create animation args for specific frames
    pub fn frames(frames: Vec<String>) -> Self {
        Self {
            frames: FrameListMode::Frames(frames),
            ..Default::default()
        }
    }

    /// Set the animation options
    pub fn options(mut self, options: AnimationOptions) -> Self {
        self.options = options;
        self
    }
}

impl Serialize for Animation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.frames)?;
        seq.serialize_element(&self.options)?;
        seq.end()
    }
}

/// First argument in animation args - can be null, [null], or frame names
#[derive(Clone, Debug)]
pub enum FrameListMode {
    /// null - animate all frames
    All,
    /// Array of frame names to animate
    Frames(Vec<String>),
    /// special mode, [null], for pausing an animation
    Pause,
}

impl Serialize for FrameListMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FrameListMode::All => serializer.serialize_unit(),
            FrameListMode::Pause => {
                let arr = vec![serde_json::Value::Null];
                arr.serialize(serializer)
            }
            FrameListMode::Frames(frames) => frames.serialize(serializer),
        }
    }
}

/// Animation configuration options
/// Based on actual Plotly.js animation API from animation_attributes.js
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct AnimationOptions {
    /// Frame animation settings
    frame: Option<FrameSettings>,
    /// Transition animation settings
    transition: Option<TransitionSettings>,
    /// Animation mode
    mode: Option<AnimationMode>,
    /// Animation direction
    direction: Option<AnimationDirection>,
    /// Play frames starting at the current frame instead of the beginning
    fromcurrent: Option<bool>,
}

impl AnimationOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Frame animation settings
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct FrameSettings {
    /// The duration in milliseconds of each frame
    duration: Option<usize>,
    /// Redraw the plot at completion of the transition
    redraw: Option<bool>,
}

impl FrameSettings {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Transition animation settings
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct TransitionSettings {
    /// The duration of the transition, in milliseconds
    duration: Option<usize>,
    /// The easing function used for the transition
    easing: Option<AnimationEasing>,
    /// Determines whether the figure's layout or traces smoothly transitions
    ordering: Option<TransitionOrdering>,
}

impl TransitionSettings {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Animation modes
#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnimationMode {
    Immediate,
    Next,
    AfterAll,
}

/// Animation directions
#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnimationDirection {
    Forward,
    Reverse,
}

/// Transition ordering options
#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransitionOrdering {
    #[serde(rename = "layout first")]
    LayoutFirst,
    #[serde(rename = "traces first")]
    TracesFirst,
}

/// Easing functions for animation transitions
#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnimationEasing {
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
    use crate::Scatter;

    #[test]
    fn serialize_animation_easing() {
        let test_cases = [
            (AnimationEasing::Linear, "linear"),
            (AnimationEasing::Cubic, "cubic"),
            (AnimationEasing::CubicInOut, "cubic-in-out"),
            (AnimationEasing::ElasticInOut, "elastic-in-out"),
        ];

        for (easing, expected) in test_cases {
            assert_eq!(
                to_value(easing).unwrap(),
                json!(expected),
                "Failed for {:?}",
                easing
            );
        }
    }

    #[test]
    fn serialize_animation_mode() {
        let test_cases = [
            (AnimationMode::Immediate, "immediate"),
            (AnimationMode::Next, "next"),
            (AnimationMode::AfterAll, "afterall"),
        ];

        for (mode, expected) in test_cases {
            assert_eq!(
                to_value(mode).unwrap(),
                json!(expected),
                "Failed for {:?}",
                mode
            );
        }
    }

    #[test]
    fn serialize_animation_direction() {
        let test_cases = [
            (AnimationDirection::Forward, "forward"),
            (AnimationDirection::Reverse, "reverse"),
        ];

        for (direction, expected) in test_cases {
            assert_eq!(
                to_value(direction).unwrap(),
                json!(expected),
                "Failed for {:?}",
                direction
            );
        }
    }

    #[test]
    fn serialize_transition_ordering() {
        let test_cases = [
            (TransitionOrdering::LayoutFirst, "layout first"),
            (TransitionOrdering::TracesFirst, "traces first"),
        ];

        for (ordering, expected) in test_cases {
            assert_eq!(
                to_value(ordering).unwrap(),
                json!(expected),
                "Failed for {:?}",
                ordering
            );
        }
    }

    #[test]
    fn serialize_frame() {
        let frame = Frame::new()
            .name("test_frame")
            .group("test_group")
            .baseframe("base_frame");

        let expected = json!({
            "name": "test_frame",
            "group": "test_group",
            "baseframe": "base_frame"
        });

        assert_eq!(to_value(frame).unwrap(), expected);
    }

    #[test]
    fn serialize_frame_with_data() {
        let trace = Scatter::new(vec![1, 2, 3], vec![1, 2, 3]);
        let mut traces = Traces::new();
        traces.push(trace);

        let frame = Frame::new().name("frame_with_data").data(traces);

        let expected = json!({
            "name": "frame_with_data",
            "data": [
                {
                    "type": "scatter",
                    "x": [1, 2, 3],
                    "y": [1, 2, 3]
                }
            ]
        });

        assert_eq!(to_value(frame).unwrap(), expected);
    }

    #[test]
    fn serialize_animation() {
        let test_cases = [
            (
                Animation::all_frames(),
                json!(null),
                "all frames should serialize to null",
            ),
            (
                Animation::pause(),
                json!([null]),
                "pause should serialize to [null]",
            ),
            (
                Animation::frames(vec!["frame1".to_string(), "frame2".to_string()]),
                json!(["frame1", "frame2"]),
                "specific frames should serialize to frame names array",
            ),
        ];

        for (animation, expected_frames, description) in test_cases {
            let json = to_value(animation).unwrap();
            assert_eq!(json[0], expected_frames, "{}", description);
            assert!(json[1].is_object(), "Second element should be an object");
        }
    }

    #[test]
    fn serialize_animation_options_defaults() {
        let options = AnimationOptions::new();
        assert_eq!(to_value(options).unwrap(), json!({}));
    }

    #[test]
    fn serialize_animation_options() {
        let options = AnimationOptions::new()
            .mode(AnimationMode::Immediate)
            .direction(AnimationDirection::Forward)
            .fromcurrent(false)
            .frame(FrameSettings::new().duration(500).redraw(true))
            .transition(
                TransitionSettings::new()
                    .duration(300)
                    .easing(AnimationEasing::CubicInOut)
                    .ordering(TransitionOrdering::LayoutFirst),
            );

        let expected = json!({
            "mode": "immediate",
            "direction": "forward",
            "fromcurrent": false,
            "frame": {
                "duration": 500,
                "redraw": true
            },
            "transition": {
                "duration": 300,
                "easing": "cubic-in-out",
                "ordering": "layout first"
            }
        });

        assert_eq!(to_value(options).unwrap(), expected);
    }
}
