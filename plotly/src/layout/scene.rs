use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::Color;
use crate::layout::{Annotation, AspectMode, Axis};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets this scene's axis aspectratio.
/// x, y, z must be positive.
/// Default: {x: 1, y: 1, z: 1}
pub struct AspectRatio {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl AspectRatio {
    pub fn new() -> Self {
        AspectRatio {
            x: Some(1.0),
            y: Some(1.0),
            z: Some(1.0),
        }
    }
}

impl From<(f64, f64, f64)> for AspectRatio {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        AspectRatio {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Container for CameraCenter, Eye, Up, and Projection objects. The camera of a
/// 3D scene.
pub struct Camera {
    center: Option<CameraCenter>,
    eye: Option<Eye>,
    up: Option<Up>,
    projection: Option<Projection>,
}

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets the (x, y, z) components of the 'center' camera vector. This vector
/// determines the translation (x, y, z) space about the center of this scene.
/// Default: {x: 0, y: 0, z: 0} which means that the center of the scene is at
/// the origin.
pub struct CameraCenter {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl CameraCenter {
    pub fn new() -> Self {
        CameraCenter {
            x: Some(0.0),
            y: Some(0.0),
            z: Some(0.0),
        }
    }
}

impl From<(f64, f64, f64)> for CameraCenter {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        CameraCenter {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets the (x, y, z) components of the 'eye' camera vector. This vector
/// determines the view point about the origin of this scene.
/// Default: {x: 1.25, y: 1.25, z: 1.25}
pub struct Eye {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl Eye {
    pub fn new() -> Self {
        Eye {
            x: Some(1.25),
            y: Some(1.25),
            z: Some(1.25),
        }
    }
}

impl From<(f64, f64, f64)> for Eye {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Eye {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Sets the (x, y, z) components of the 'up' camera vector. This vector
/// determines the up direction of this scene with respect to the page. The
/// Default: {x: 0, y: 0, z: 1} which means that the z axis points up.
pub struct Up {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl Up {
    pub fn new() -> Self {
        Up {
            x: Some(0.0),
            y: Some(0.0),
            z: Some(1.0),
        }
    }
}

impl From<(f64, f64, f64)> for Up {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Up {
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }
}

/// Sets the projection type. The projection type could be either "perspective"
/// or "orthographic".
/// Default: "perspective"
#[derive(Default, Serialize, Debug, Clone)]
pub enum ProjectionType {
    #[default]
    #[serde(rename = "perspective")]
    Perspective,
    #[serde(rename = "orthographic")]
    Orthographic,
}

impl From<ProjectionType> for Projection {
    fn from(projection_type: ProjectionType) -> Self {
        Projection {
            projection_type: Some(projection_type),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Container for Projection options.
pub struct Projection {
    #[serde(rename = "type")]
    projection_type: Option<ProjectionType>,
}

impl Projection {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub enum DragMode {
    Zoom,
    Pan,
    Select,
    Lasso,
    DrawClosedPath,
    DrawOpenPath,
    DrawLine,
    DrawRect,
    DrawCircle,
    Orbit,
    Turntable,
    False,
}

impl Serialize for DragMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Zoom => serializer.serialize_str("zoom"),
            Self::Pan => serializer.serialize_str("pan"),
            Self::Select => serializer.serialize_str("select"),
            Self::Lasso => serializer.serialize_str("lasso"),
            Self::DrawClosedPath => serializer.serialize_str("drawclosedpath"),
            Self::DrawOpenPath => serializer.serialize_str("drawopenpath"),
            Self::DrawLine => serializer.serialize_str("drawline"),
            Self::DrawRect => serializer.serialize_str("drawrect"),
            Self::DrawCircle => serializer.serialize_str("drawcircle"),
            Self::Orbit => serializer.serialize_str("orbit"),
            Self::Turntable => serializer.serialize_str("turntable"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Debug, Clone)]
/// Determines the mode of drag interactions.
pub enum DragMode3D {
    Zoom,
    Pan,
    Turntable,
    Orbit,
    False,
}

impl Serialize for DragMode3D {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Zoom => serializer.serialize_str("zoom"),
            Self::Pan => serializer.serialize_str("pan"),
            Self::Turntable => serializer.serialize_str("turntable"),
            Self::Orbit => serializer.serialize_str("orbit"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Debug, Clone)]
pub enum HoverMode {
    X,
    Y,
    Closest,
    False,
    XUnified,
    YUnified,
}

impl Serialize for HoverMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::X => serializer.serialize_str("x"),
            Self::Y => serializer.serialize_str("y"),
            Self::Closest => serializer.serialize_str("closest"),
            Self::False => serializer.serialize_bool(false),
            Self::XUnified => serializer.serialize_str("x unified"),
            Self::YUnified => serializer.serialize_str("y unified"),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// 3D scene layout
pub struct LayoutScene {
    #[serde(rename = "bgcolor")]
    background_color: Option<Box<dyn Color>>,
    camera: Option<Camera>,
    #[serde(rename = "aspectmode")]
    aspect_mode: Option<AspectMode>,
    #[serde(rename = "aspectratio")]
    aspect_ratio: Option<AspectRatio>,
    #[serde(rename = "xaxis")]
    x_axis: Option<Axis>,
    #[serde(rename = "yaxis")]
    y_axis: Option<Axis>,
    #[serde(rename = "zaxis")]
    z_axis: Option<Axis>,
    #[serde(rename = "dragmode")]
    drag_mode: Option<DragMode3D>,
    #[serde(rename = "hovermode")]
    hover_mode: Option<HoverMode>,
    annotations: Option<Vec<Annotation>>,
    // domain: Domain,
    // uirevision: Uirevision,
}

impl LayoutScene {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::layout::Layout;

    #[test]
    fn serialize_hover_mode() {
        assert_eq!(to_value(HoverMode::X).unwrap(), json!("x"));
        assert_eq!(to_value(HoverMode::Y).unwrap(), json!("y"));
        assert_eq!(to_value(HoverMode::Closest).unwrap(), json!("closest"));
        assert_eq!(to_value(HoverMode::False).unwrap(), json!(false));
        assert_eq!(to_value(HoverMode::XUnified).unwrap(), json!("x unified"));
        assert_eq!(to_value(HoverMode::YUnified).unwrap(), json!("y unified"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_drag_mode() {
        assert_eq!(to_value(DragMode::Zoom).unwrap(), json!("zoom"));
        assert_eq!(to_value(DragMode::Pan).unwrap(), json!("pan"));
        assert_eq!(to_value(DragMode::Select).unwrap(), json!("select"));
        assert_eq!(to_value(DragMode::Lasso).unwrap(), json!("lasso"));
        assert_eq!(to_value(DragMode::DrawClosedPath).unwrap(), json!("drawclosedpath"));
        assert_eq!(to_value(DragMode::DrawOpenPath).unwrap(), json!("drawopenpath"));
        assert_eq!(to_value(DragMode::DrawLine).unwrap(), json!("drawline"));
        assert_eq!(to_value(DragMode::DrawRect).unwrap(), json!("drawrect"));
        assert_eq!(to_value(DragMode::DrawCircle).unwrap(), json!("drawcircle"));
        assert_eq!(to_value(DragMode::Orbit).unwrap(), json!("orbit"));
        assert_eq!(to_value(DragMode::Turntable).unwrap(), json!("turntable"));
        assert_eq!(to_value(DragMode::False).unwrap(), json!(false));
    }

    #[test]
    fn serialize_aspect_ratio() {
        let aspect_ratio = AspectRatio::new();

        let expected = json!({
            "x": 1.0,
            "y": 1.0,
            "z": 1.0,
        });

        assert_eq!(to_value(aspect_ratio).unwrap(), expected);

        let aspect_ratio = AspectRatio::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(aspect_ratio).unwrap(), expected);

        let aspect_ratio: AspectRatio = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(aspect_ratio).unwrap(), expected);
    }

    #[test]
    fn serialize_layout_scene() {
        let layout = Layout::new().scene(
            LayoutScene::new()
                .x_axis(Axis::new())
                .y_axis(Axis::new())
                .z_axis(Axis::new())
                .camera(Camera::new())
                .aspect_mode(AspectMode::Auto)
                .hover_mode(HoverMode::Closest)
                .drag_mode(DragMode3D::Turntable)
                .background_color("#FFFFFF")
                .annotations(vec![Annotation::new()]),
        );

        let expected = json!({
            "scene": {
                "xaxis": {},
                "yaxis": {},
                "zaxis": {},
                "camera": {},
                "aspectmode": "auto",
                "hovermode": "closest",
                "dragmode": "turntable",
                "bgcolor": "#FFFFFF",
                "annotations": [{}],
            }
        });

        assert_eq!(to_value(layout).unwrap(), expected);
    }

    #[test]
    fn serialize_eye() {
        let eye = Eye::new();

        assert_eq!(
            to_value(eye).unwrap(),
            json!({
                "x": 1.25,
                "y": 1.25,
                "z": 1.25,
            })
        );

        let eye = Eye::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(eye).unwrap(), expected);

        let eye: Eye = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(eye).unwrap(), expected);
    }

    #[test]
    fn serialize_projection() {
        let projection = Projection::new().projection_type(ProjectionType::default());

        let expected = json!({
            "type": "perspective",
        });

        assert_eq!(to_value(projection).unwrap(), expected);

        let projection = Projection::new().projection_type(ProjectionType::Orthographic);

        let expected = json!({
            "type": "orthographic",
        });

        assert_eq!(to_value(projection).unwrap(), expected);

        let projection: Projection = ProjectionType::Orthographic.into();

        assert_eq!(to_value(projection).unwrap(), expected);
    }

    #[test]
    fn serialize_camera_center() {
        let camera_center = CameraCenter::new();

        let expected = json!({
            "x": 0.0,
            "y": 0.0,
            "z": 0.0,
        });

        assert_eq!(to_value(camera_center).unwrap(), expected);

        let camera_center = CameraCenter::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(camera_center).unwrap(), expected);

        let camera_center: CameraCenter = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(camera_center).unwrap(), expected);
    }

    #[test]
    fn serialize_up() {
        let up = Up::new();

        let expected = json!({
            "x": 0.0,
            "y": 0.0,
            "z": 1.0,
        });

        assert_eq!(to_value(up).unwrap(), expected);

        let up = Up::new().x(1f64).y(2f64).z(3f64);

        let expected = json!({
            "x": 1.0,
            "y": 2.0,
            "z": 3.0,
        });

        assert_eq!(to_value(up).unwrap(), expected);

        let up: Up = (1f64, 2f64, 3f64).into();

        assert_eq!(to_value(up).unwrap(), expected);
    }
}
