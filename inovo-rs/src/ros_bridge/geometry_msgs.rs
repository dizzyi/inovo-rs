use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::std_msgs::Header;

#[inovo_msg("geometry_msgs")]
pub struct Accel {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[inovo_msg("geometry_msgs")]
pub struct AccelStamped {
    pub header: Header,
    pub accel: Accel,
}

#[inovo_msg("geometry_msgs")]
pub struct AccelWithCovariance {
    pub accel: Accel,
    pub covariance: Vec<f64>, // TODO assert lenght 36
}

#[inovo_msg("geometry_msgs")]
pub struct AccelWithCovarianceStamped {
    pub header: Header,
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
pub struct InertiaStamped {
    pub header: Header,
    pub inertia: Inertia,
}

#[inovo_msg("geometry_msgs")]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[inovo_msg("geometry_msgs")]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[inovo_msg("geometry_msgs")]
pub struct PointStamped {
    pub header: Header,
    pub point: Point,
}

#[inovo_msg("geometry_msgs")]
pub struct Polygon {
    pub points: Vec<Point32>,
}

#[inovo_msg("geometry_msgs")]
pub struct PolygonStamped {
    pub header: Header,
    pub polygon: Polygon,
}

#[inovo_msg("geometry_msgs")]
pub struct Pose {
    pub position: Point,
    pub orientation: Quaternion,
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
pub struct PoseStamped {
    pub header: Header,
    pub pose: Pose,
}

#[inovo_msg("geometry_msgs")]
pub struct PoseWithCovariance {
    pub pose: Pose,
    pub covariance: Vec<f64>, // TODO assert lenght 36
}

#[inovo_msg("geometry_msgs")]
pub struct PoseWithCovarianceStamped {
    pub header: Header,
    pub pose: PoseWithCovariance,
}

#[inovo_msg("geometry_msgs")]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[inovo_msg("geometry_msgs")]
pub struct QuaternionStamped {
    pub header: Header,
    pub quaternion: Quaternion,
}

#[inovo_msg("geometry_msgs")]
pub struct Transform {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

#[inovo_msg("geometry_msgs")]
pub struct TransformStamped {
    pub header: Header,
    pub transfrom: Transform,
}

#[inovo_msg("geometry_msgs")]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}

#[inovo_msg("geometry_msgs")]
pub struct TwistStamped {
    pub header: Header,
    pub twist: Twist,
}

#[inovo_msg("geometry_msgs")]
pub struct TwistWithCovariance {
    pub twist: Twist,
    pub covariance: Vec<f64>, // assert lenght 36
}

#[inovo_msg("geometry_msgs")]
pub struct TwistWithCovarianceStamped {
    pub header: Header,
    pub twist: TwistWithCovariance,
}

#[inovo_msg("geometry_msgs")]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[inovo_msg("geometry_msgs")]
pub struct Vector3Stamped {
    pub header: Header,
    pub vector: Vector3,
}

#[inovo_msg("geometry_msgs")]
pub struct Wrench {
    pub force: Vector3,
    pub torque: Vector3,
}

#[inovo_msg("geometry_msgs")]
pub struct WrenchStamped {
    pub header: Header,
    pub wrench: Wrench,
}
