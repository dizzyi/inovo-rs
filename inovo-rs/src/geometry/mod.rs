//! Data Structure representing spatial coordinate and robot pose.

mod joint;
mod transform;

use std::f64::consts::PI;

pub use joint::JointCoord;
pub use transform::Pose;
