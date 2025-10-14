use derive_more::{Deref, DerefMut};

use inovo_rs_macro::*;

use crate::ros_bridge::std_msgs::Header;

#[inovo_msg("geometry_msgs")]
pub struct Accel {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct AccelStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub accel: Accel,
}

#[inovo_msg("geometry_msgs")]
pub struct AccelWithCovariance {
    pub accel: Accel,
    pub covariance: Vec<f64>, // TODO assert lenght 36
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct AccelWithCovarianceStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub accel: AccelWithCovariance,
}

#[inovo_msg("geometry_msgs")]
pub struct Inertia {
    pub m: f64,
    pub com: Vector3,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct InertiaStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub inertia: Inertia,
}

#[inovo_msg("geometry_msgs")]
#[derive(Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Point> for nalgebra::Vector3<f64> {
    fn from(val: Point) -> Self {
        nalgebra::Vector3::<f64>::new(val.x, val.y, val.z)
    }
}
impl From<nalgebra::Vector3<f64>> for Point {
    fn from(value: nalgebra::Vector3<f64>) -> Self {
        Point {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Point> for nalgebra::Translation3<f64> {
    fn from(val: Point) -> Self {
        nalgebra::Translation { vector: val.into() }
    }
}
impl From<nalgebra::Translation3<f64>> for Point {
    fn from(value: nalgebra::Translation3<f64>) -> Self {
        value.vector.into()
    }
}

impl Point {
    pub fn into_vector(self) -> nalgebra::Vector3<f64> {
        self.into()
    }
    pub fn from_vector(vector: nalgebra::Vector3<f64>) -> Point {
        vector.into()
    }
    pub fn into_translation(self) -> nalgebra::Translation3<f64> {
        self.into()
    }
    pub fn from_translation(tran: nalgebra::Translation3<f64>) -> Point {
        tran.into()
    }
}

#[inovo_msg("geometry_msgs")]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct PointStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub point: Point,
}

#[inovo_msg("geometry_msgs")]
pub struct Polygon {
    pub points: Vec<Point32>,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct PolygonStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub polygon: Polygon,
}

#[inovo_msg("geometry_msgs")]
#[derive(Copy)]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
}

impl From<Pose> for nalgebra::Isometry3<f64> {
    fn from(val: Pose) -> Self {
        nalgebra::Isometry {
            rotation: val.orientation.into(),
            translation: val.position.into(),
        }
    }
}
impl From<nalgebra::Isometry3<f64>> for Pose {
    fn from(value: nalgebra::Isometry3<f64>) -> Self {
        Pose {
            position: value.translation.into(),
            orientation: value.rotation.into(),
        }
    }
}

impl Pose {
    pub fn into_isometry(self) -> nalgebra::Isometry3<f64> {
        self.into()
    }
    pub fn from_isometry(iso: nalgebra::Isometry3<f64>) -> Pose {
        iso.into()
    }
}

#[inovo_msg("geometry_msgs")]
pub struct Pose2D {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

#[inovo_msg("geometry_msgs")]
pub struct PoseArray {
    pub header: Header,
    pub poses: Vec<Pose>,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct PoseStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub pose: Pose,
}

#[inovo_msg("geometry_msgs")]
pub struct PoseWithCovariance {
    pub pose: Pose,
    pub covariance: Vec<f64>, // TODO assert lenght 36
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct PoseWithCovarianceStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub pose: PoseWithCovariance,
}

#[inovo_msg("geometry_msgs")]
#[derive(Copy)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<Quaternion> for nalgebra::Quaternion<f64> {
    fn from(val: Quaternion) -> Self {
        nalgebra::Quaternion::new(val.w, val.x, val.y, val.z)
    }
}
impl From<Quaternion> for nalgebra::UnitQuaternion<f64> {
    fn from(val: Quaternion) -> Self {
        nalgebra::UnitQuaternion::from_quaternion(val.into())
    }
}
impl From<nalgebra::Quaternion<f64>> for Quaternion {
    fn from(value: nalgebra::Quaternion<f64>) -> Self {
        Quaternion {
            x: value.i,
            y: value.j,
            z: value.k,
            w: value.w,
        }
    }
}
impl From<nalgebra::UnitQuaternion<f64>> for Quaternion {
    fn from(value: nalgebra::UnitQuaternion<f64>) -> Self {
        Quaternion {
            x: value.i,
            y: value.j,
            z: value.k,
            w: value.w,
        }
    }
}

impl Quaternion {
    pub fn into_quaternion(self) -> nalgebra::Quaternion<f64> {
        self.into()
    }
    pub fn from_quaternion(quat: nalgebra::Quaternion<f64>) -> Quaternion {
        quat.into()
    }
    pub fn into_unit_quaternion(self) -> nalgebra::UnitQuaternion<f64> {
        self.into()
    }
    pub fn from_unit_quaternion(unit_quat: nalgebra::UnitQuaternion<f64>) -> Quaternion {
        unit_quat.into()
    }
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct QuaternionStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub quaternion: Quaternion,
}

#[inovo_msg("geometry_msgs")]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct TransformStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub transfrom: Transform,
}

#[inovo_msg("geometry_msgs")]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct TwistStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub twist: Twist,
}

#[inovo_msg("geometry_msgs")]
pub struct TwistWithCovariance {
    pub twist: Twist,
    pub covariance: Vec<f64>, // assert lenght 36
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct TwistWithCovarianceStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub twist: TwistWithCovariance,
}

#[inovo_msg("geometry_msgs")]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Vector3> for nalgebra::Vector3<f64> {
    fn from(val: Vector3) -> Self {
        nalgebra::Vector3::<f64>::new(val.x, val.y, val.z)
    }
}
impl From<nalgebra::Vector3<f64>> for Vector3 {
    fn from(value: nalgebra::Vector3<f64>) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector3> for nalgebra::Translation3<f64> {
    fn from(val: Vector3) -> Self {
        nalgebra::Translation { vector: val.into() }
    }
}
impl From<nalgebra::Translation3<f64>> for Vector3 {
    fn from(value: nalgebra::Translation3<f64>) -> Self {
        value.vector.into()
    }
}

impl Vector3 {
    pub fn into_vector(self) -> nalgebra::Vector3<f64> {
        self.into()
    }
    pub fn from_vector(vector: nalgebra::Vector3<f64>) -> Vector3 {
        vector.into()
    }
    pub fn into_translation(self) -> nalgebra::Translation3<f64> {
        self.into()
    }
    pub fn from_translation(tran: nalgebra::Translation3<f64>) -> Vector3 {
        tran.into()
    }
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct Vector3Stamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub vector: Vector3,
}

#[inovo_msg("geometry_msgs")]
pub struct Wrench {
    pub force: Vector3,
    pub torque: Vector3,
}

#[inovo_msg("geometry_msgs")]
#[derive(Deref, DerefMut)]
pub struct WrenchStamped {
    pub header: Header,
    #[deref]
    #[deref_mut]
    pub wrench: Wrench,
}
