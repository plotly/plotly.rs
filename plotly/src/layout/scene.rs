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
    #[serde(rename = "airy")]
    Airy,
    #[serde(rename = "aitoff")]
    Aitoff,
    #[serde(rename = "albers")]
    Albers,
    #[serde(rename = "albers usa")]
    AlbersUsa,
    #[serde(rename = "august")]
    August,
    #[serde(rename = "azimuthal equal area")]
    AzimuthalEqualArea,
    #[serde(rename = "azimuthal equidistant")]
    AzimuthalEquidistant,
    #[serde(rename = "baker")]
    Baker,
    #[serde(rename = "bertin1953")]
    Bertin1953,
    #[serde(rename = "boggs")]
    Boggs,
    #[serde(rename = "bonne")]
    Bonne,
    #[serde(rename = "bottomley")]
    Bottomley,
    #[serde(rename = "bromley")]
    Bromley,
    #[serde(rename = "collignon")]
    Collignon,
    #[serde(rename = "conic conformal")]
    ConicConformal,
    #[serde(rename = "conic equal area")]
    ConicEqualArea,
    #[serde(rename = "conic equidistant")]
    ConicEquidistant,
    #[serde(rename = "craig")]
    Craig,
    #[serde(rename = "craster")]
    Craster,
    #[serde(rename = "cylindrical equal area")]
    CylindricalEqualArea,
    #[serde(rename = "cylindrical stereographic")]
    CylindricalStereographic,
    #[serde(rename = "eckert1")]
    Eckert1,
    #[serde(rename = "eckert2")]
    Eckert2,
    #[serde(rename = "eckert3")]
    Eckert3,
    #[serde(rename = "eckert4")]
    Eckert4,
    #[serde(rename = "eckert5")]
    Eckert5,
    #[serde(rename = "eckert6")]
    Eckert6,
    #[serde(rename = "eisenlohr")]
    Eisenlohr,
    #[serde(rename = "equal earth")]
    EqualEarth,
    #[serde(rename = "equirectangular")]
    Equirectangular,
    #[serde(rename = "fahey")]
    Fahey,
    #[serde(rename = "foucaut")]
    Foucaut,
    #[serde(rename = "foucaut sinusoidal")]
    FoucautSinusoidal,
    #[serde(rename = "ginzburg4")]
    Ginzburg4,
    #[serde(rename = "ginzburg5")]
    Ginzburg5,
    #[serde(rename = "ginzburg6")]
    Ginzburg6,
    #[serde(rename = "ginzburg8")]
    Ginzburg8,
    #[serde(rename = "ginzburg9")]
    Ginzburg9,
    #[serde(rename = "gnomonic")]
    Gnomonic,
    #[serde(rename = "gringorten")]
    Gringorten,
    #[serde(rename = "gringorten quincuncial")]
    GringortenQuincuncial,
    #[serde(rename = "guyou")]
    Guyou,
    #[serde(rename = "hammer")]
    Hammer,
    #[serde(rename = "hill")]
    Hill,
    #[serde(rename = "homolosine")]
    Homolosine,
    #[serde(rename = "hufnagel")]
    Hufnagel,
    #[serde(rename = "hyperelliptical")]
    Hyperelliptical,
    #[serde(rename = "kavrayskiy7")]
    Kavrayskiy7,
    #[serde(rename = "lagrange")]
    Lagrange,
    #[serde(rename = "larrivee")]
    Larrivee,
    #[serde(rename = "laskowski")]
    Laskowski,
    #[serde(rename = "loximuthal")]
    Loximuthal,
    #[serde(rename = "mercator")]
    Mercator,
    #[serde(rename = "miller")]
    Miller,
    #[serde(rename = "mollweide")]
    Mollweide,
    #[serde(rename = "mt flat polar parabolic")]
    MtFlatPolarParabolic,
    #[serde(rename = "mt flat polar quartic")]
    MtFlatPolarQuartic,
    #[serde(rename = "mt flat polar sinusoidal")]
    MtFlatPolarSinusoidal,
    #[serde(rename = "natural earth")]
    NaturalEarth,
    #[serde(rename = "natural earth1")]
    NaturalEarth1,
    #[serde(rename = "natural earth2")]
    NaturalEarth2,
    #[serde(rename = "nell hammer")]
    NellHammer,
    #[serde(rename = "nicolosi")]
    Nicolosi,
    #[serde(rename = "patterson")]
    Patterson,
    #[serde(rename = "peirce quincuncial")]
    PeirceQuincuncial,
    #[serde(rename = "polyconic")]
    Polyconic,
    #[serde(rename = "rectangular polyconic")]
    RectangularPolyconic,
    #[serde(rename = "robinson")]
    Robinson,
    #[serde(rename = "satellite")]
    Satellite,
    #[serde(rename = "sinu mollweide")]
    SinuMollweide,
    #[serde(rename = "sinusoidal")]
    Sinusoidal,
    #[serde(rename = "stereographic")]
    Stereographic,
    #[serde(rename = "times")]
    Times,
    #[serde(rename = "transverse mercator")]
    TransverseMercator,
    #[serde(rename = "van der grinten")]
    VanDerGrinten,
    #[serde(rename = "van der grinten2")]
    VanDerGrinten2,
    #[serde(rename = "van der grinten3")]
    VanDerGrinten3,
    #[serde(rename = "van der grinten4")]
    VanDerGrinten4,
    #[serde(rename = "wagner4")]
    Wagner4,
    #[serde(rename = "wagner6")]
    Wagner6,
    #[serde(rename = "wiechel")]
    Wiechel,
    #[serde(rename = "winkel tripel")]
    WinkelTripel,
    #[serde(rename = "winkel3")]
    Winkel3,
}

impl From<ProjectionType> for Projection {
    fn from(projection_type: ProjectionType) -> Self {
        Projection {
            projection_type: Some(projection_type),
            rotation: None,
        }
    }
}

/// Sets the rotation of the map projection.
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Rotation {
    /// Rotates the map along meridians (in degrees North).
    lat: Option<f64>,
    /// Rotates the map along parallels (in degrees East).
    lon: Option<f64>,
    /// Roll the map (in degrees). For example, a roll of "180" makes the map
    /// appear upside down.
    roll: Option<f64>,
}

impl Rotation {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
/// Container for Projection options.
pub struct Projection {
    #[serde(rename = "type")]
    projection_type: Option<ProjectionType>,
    // Sets the rotation of the map projection. See https://plotly.com/python/reference/layout/geo/#layout-geo-projection-rotation
    #[serde(rename = "rotation")]
    rotation: Option<Rotation>,
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
        assert_eq!(
            to_value(projection).unwrap(),
            json!({"type": "perspective"})
        );

        let projection: Projection = ProjectionType::Orthographic.into();
        assert_eq!(
            to_value(projection).unwrap(),
            json!({ "type": "orthographic" })
        );

        let projections = vec![
            (ProjectionType::Orthographic, "orthographic"),
            (ProjectionType::Perspective, "perspective"),
            (ProjectionType::Airy, "airy"),
            (ProjectionType::Aitoff, "aitoff"),
            (ProjectionType::Albers, "albers"),
            (ProjectionType::AlbersUsa, "albers usa"),
            (ProjectionType::August, "august"),
            (ProjectionType::AzimuthalEqualArea, "azimuthal equal area"),
            (
                ProjectionType::AzimuthalEquidistant,
                "azimuthal equidistant",
            ),
            (ProjectionType::Baker, "baker"),
            (ProjectionType::Bertin1953, "bertin1953"),
            (ProjectionType::Boggs, "boggs"),
            (ProjectionType::Bonne, "bonne"),
            (ProjectionType::Bottomley, "bottomley"),
            (ProjectionType::Bromley, "bromley"),
            (ProjectionType::Collignon, "collignon"),
            (ProjectionType::ConicConformal, "conic conformal"),
            (ProjectionType::ConicEqualArea, "conic equal area"),
            (ProjectionType::ConicEquidistant, "conic equidistant"),
            (ProjectionType::Craig, "craig"),
            (ProjectionType::Craster, "craster"),
            (
                ProjectionType::CylindricalEqualArea,
                "cylindrical equal area",
            ),
            (
                ProjectionType::CylindricalStereographic,
                "cylindrical stereographic",
            ),
            (ProjectionType::Eckert1, "eckert1"),
            (ProjectionType::Eckert2, "eckert2"),
            (ProjectionType::Eckert3, "eckert3"),
            (ProjectionType::Eckert4, "eckert4"),
            (ProjectionType::Eckert5, "eckert5"),
            (ProjectionType::Eckert6, "eckert6"),
            (ProjectionType::Eisenlohr, "eisenlohr"),
            (ProjectionType::EqualEarth, "equal earth"),
            (ProjectionType::Equirectangular, "equirectangular"),
            (ProjectionType::Fahey, "fahey"),
            (ProjectionType::Foucaut, "foucaut"),
            (ProjectionType::FoucautSinusoidal, "foucaut sinusoidal"),
            (ProjectionType::Ginzburg4, "ginzburg4"),
            (ProjectionType::Ginzburg5, "ginzburg5"),
            (ProjectionType::Ginzburg6, "ginzburg6"),
            (ProjectionType::Ginzburg8, "ginzburg8"),
            (ProjectionType::Ginzburg9, "ginzburg9"),
            (ProjectionType::Gnomonic, "gnomonic"),
            (ProjectionType::Gringorten, "gringorten"),
            (
                ProjectionType::GringortenQuincuncial,
                "gringorten quincuncial",
            ),
            (ProjectionType::Guyou, "guyou"),
            (ProjectionType::Hammer, "hammer"),
            (ProjectionType::Hill, "hill"),
            (ProjectionType::Homolosine, "homolosine"),
            (ProjectionType::Hufnagel, "hufnagel"),
            (ProjectionType::Hyperelliptical, "hyperelliptical"),
            (ProjectionType::Kavrayskiy7, "kavrayskiy7"),
            (ProjectionType::Lagrange, "lagrange"),
            (ProjectionType::Larrivee, "larrivee"),
            (ProjectionType::Laskowski, "laskowski"),
            (ProjectionType::Loximuthal, "loximuthal"),
            (ProjectionType::Mercator, "mercator"),
            (ProjectionType::Miller, "miller"),
            (ProjectionType::Mollweide, "mollweide"),
            (
                ProjectionType::MtFlatPolarParabolic,
                "mt flat polar parabolic",
            ),
            (ProjectionType::MtFlatPolarQuartic, "mt flat polar quartic"),
            (
                ProjectionType::MtFlatPolarSinusoidal,
                "mt flat polar sinusoidal",
            ),
            (ProjectionType::NaturalEarth, "natural earth"),
            (ProjectionType::NaturalEarth1, "natural earth1"),
            (ProjectionType::NaturalEarth2, "natural earth2"),
            (ProjectionType::NellHammer, "nell hammer"),
            (ProjectionType::Nicolosi, "nicolosi"),
            (ProjectionType::Patterson, "patterson"),
            (ProjectionType::PeirceQuincuncial, "peirce quincuncial"),
            (ProjectionType::Polyconic, "polyconic"),
            (
                ProjectionType::RectangularPolyconic,
                "rectangular polyconic",
            ),
            (ProjectionType::Robinson, "robinson"),
            (ProjectionType::Satellite, "satellite"),
            (ProjectionType::SinuMollweide, "sinu mollweide"),
            (ProjectionType::Sinusoidal, "sinusoidal"),
            (ProjectionType::Stereographic, "stereographic"),
            (ProjectionType::Times, "times"),
            (ProjectionType::TransverseMercator, "transverse mercator"),
            (ProjectionType::VanDerGrinten, "van der grinten"),
            (ProjectionType::VanDerGrinten2, "van der grinten2"),
            (ProjectionType::VanDerGrinten3, "van der grinten3"),
            (ProjectionType::VanDerGrinten4, "van der grinten4"),
            (ProjectionType::Wagner4, "wagner4"),
            (ProjectionType::Wagner6, "wagner6"),
            (ProjectionType::Wiechel, "wiechel"),
            (ProjectionType::WinkelTripel, "winkel tripel"),
            (ProjectionType::Winkel3, "winkel3"),
        ];
        for (variant, name) in projections {
            let projection = Projection::new().projection_type(variant.clone());
            assert_eq!(to_value(projection).unwrap(), json!({"type": name}));
        }
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

    #[test]
    fn serialize_projection_with_rotation() {
        let projection = Projection {
            projection_type: Some(ProjectionType::Mercator),
            rotation: Some(Rotation {
                lat: Some(10.0),
                lon: Some(20.0),
                roll: Some(30.0),
            }),
        };
        let expected = json!({
            "type": "mercator",
            "rotation": {"lat": 10.0, "lon": 20.0, "roll": 30.0}
        });
        assert_eq!(to_value(projection).unwrap(), expected);
    }
}
