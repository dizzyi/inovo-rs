//! Data Structure representing spatial coordinate and robot pose.

mod joint;
mod transform;

use std::f64::consts::PI;

pub use joint::JointCoord;
pub use transform::Transform;

/// convert degree to radian
pub fn deg_to_rad(deg: f64) -> f64 {
    deg / 180.0 * PI
}
/// convert radian to degree
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}
