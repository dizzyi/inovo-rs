use nalgebra::geometry::{Isometry3, UnitQuaternion};
use nalgebra::Translation3;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::ops::{Div, Mul, Neg};

use serde::{Deserialize, Serialize};

use crate::geometry::deg_to_rad;
use crate::iva::MotionTarget;
use crate::robot::FromRobot;

pub use crate::ros_bridge::geometry_msgs::Pose;

use crate::ros_bridge::geometry_msgs::{Point, Quaternion};

/// A structure representing a 3D Poseation
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub struct Pose {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
//     pub rx: f64,
//     pub ry: f64,
//     pub rz: f64,
// }

impl Pose {
    /// create a new Pose from vector and euler angle
    pub fn new(x_mm: f64, y_mm: f64, z_mm: f64, rx_deg: f64, ry_deg: f64, rz_deg: f64) -> Self {
        let position = Point {
            x: x_mm / 1000.0,
            y: y_mm / 1000.0,
            z: z_mm / 1000.0,
        };
        let orientation = UnitQuaternion::from_euler_angles(
            rx_deg.to_radians(),
            ry_deg.to_radians(),
            rz_deg.to_radians(),
        )
        .into();
        Self {
            position,
            orientation,
        }
    }
    /// create a new identity Pose
    pub fn identity() -> Self {
        Pose::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
    /// create a new Pose from an array containing vector and euler angle
    pub fn from_array(q: [f64; 6]) -> Self {
        Self::new(q[0], q[1], q[2], q[3], q[4], q[5])
    }
    /// create a new Pose from vector only
    pub fn from_vector(vector_mm: [f64; 3]) -> Self {
        Pose::new(vector_mm[0], vector_mm[1], vector_mm[2], 0.0, 0.0, 0.0)
    }
    /// create a new Pose from euler only
    pub fn from_euler(eular_degree: [f64; 3]) -> Self {
        Pose::new(
            0.0,
            0.0,
            0.0,
            eular_degree[0],
            eular_degree[1],
            eular_degree[2],
        )
    }
    /// create a new Pose from x component
    pub fn from_x(mm: f64) -> Self {
        Self::identity().set_x(mm)
    }
    /// create a new Pose from y component
    pub fn from_y(mm: f64) -> Self {
        Self::identity().set_y(mm)
    }
    /// create a new Pose from z component
    pub fn from_z(mm: f64) -> Self {
        Self::identity().set_z(mm)
    }
    /// create a new Pose from rx component
    pub fn from_rx(degree: f64) -> Self {
        Self::identity().set_rx(degree)
    }
    /// create a new Pose from ry component
    pub fn from_ry(degree: f64) -> Self {
        Self::identity().set_ry(degree)
    }
    /// create a new Pose from rz component
    pub fn from_rz(degree: f64) -> Self {
        Self::identity().set_rz(degree)
    }

    /// set the vector of the Pose
    pub fn set_vector(mut self, vector_mm: [f64; 3]) -> Self {
        self.position = Point {
            x: vector_mm[0],
            y: vector_mm[1],
            z: vector_mm[2],
        };
        self
    }
    /// set the euler of the Pose
    pub fn set_euler(mut self, eular_degree: [f64; 3]) -> Self {
        self.orientation =
            UnitQuaternion::from_euler_angles(eular_degree[0], eular_degree[1], eular_degree[2])
                .into();
        self
    }
    /// set the x component of the Pose
    pub fn set_x(mut self, mm: f64) -> Self {
        self.position.x = mm / 1000.0;
        self
    }
    /// set the y component of the Pose
    pub fn set_y(mut self, mm: f64) -> Self {
        self.position.y = mm / 1000.0;
        self
    }
    /// set the z component of the Pose
    pub fn set_z(mut self, mm: f64) -> Self {
        self.position.z = mm / 1000.0;
        self
    }

    // TODO
    /// set the rx component of the Pose
    pub fn set_rx(mut self, degree: f64) -> Self {
        let mut euler = self.orientation.into_unit_quaternion().euler_angles();
        euler.0 = degree.to_radians();
        self.orientation = UnitQuaternion::from_euler_angles(euler.0, euler.1, euler.2).into();
        self
    }
    /// set the ry component of the Pose
    pub fn set_ry(mut self, degree: f64) -> Self {
        let mut euler = self.orientation.into_unit_quaternion().euler_angles();
        euler.1 = degree.to_radians();
        self.orientation = UnitQuaternion::from_euler_angles(euler.0, euler.1, euler.2).into();
        self
    }
    /// set the rz component of the Pose
    pub fn set_rz(mut self, degree: f64) -> Self {
        let mut euler = self.orientation.into_unit_quaternion().euler_angles();
        euler.2 = degree.to_radians();
        self.orientation = UnitQuaternion::from_euler_angles(euler.0, euler.1, euler.2).into();
        self
    }

    /// append a new Pose to the original Pose
    pub fn then(self, pose: Self) -> Self {
        pose * self
    }
    /// append x translation to the original Pose
    pub fn then_x(self, mm: f64) -> Self {
        Self::from_x(mm) * self
    }
    /// append y translation to the original Pose
    pub fn then_y(self, mm: f64) -> Self {
        Self::from_y(mm) * self
    }
    /// append z translation to the original Pose
    pub fn then_z(self, mm: f64) -> Self {
        Self::from_z(mm) * self
    }
    /// append vector translation to the original Pose
    pub fn then_vector(self, vector_mm: [f64; 3]) -> Self {
        Self::from_vector(vector_mm) * self
    }

    /// append rx rotation to the original Pose
    pub fn then_rx(self, degree: f64) -> Self {
        Self::from_rx(degree) * self
    }
    /// append ry rotation to the original Pose
    pub fn then_ry(self, degree: f64) -> Self {
        Self::from_ry(degree) * self
    }
    /// append rz rotation to the original Pose
    pub fn then_rz(self, degree: f64) -> Self {
        Self::from_rz(degree) * self
    }
    /// append euler rotation to the original Pose
    pub fn then_euler(self, eular_degree: [f64; 3]) -> Self {
        Self::from_euler(eular_degree) * self
    }

    /// create a new Pose by extracting the vector part
    pub fn vector_only(&self) -> Self {
        // Self::from_vector(self.get_vector().to_owned())
        Pose {
            position: self.position.clone(),
            orientation: Default::default(),
        }
    }
    /// create a new Pose by extracting the euler part
    pub fn eular_only(&self) -> Self {
        // Self::from_euler(self.get_euler().to_owned())
        Pose {
            position: Default::default(),
            orientation: self.orientation.clone(),
        }
    }

    /// append relative Pose to the original Pose, relative to a reference
    pub fn then_relative_to(mut self, reference: Self, pose: Self) -> Self {
        self = reference.clone().inverse() * self;
        self = pose * self;
        reference * self
    }
    /// append relative Pose to the original Pose, relatice to the vector part of original Pose
    pub fn then_relative(self, pose: Self) -> Self {
        let reference = self.vector_only();
        self.then_relative_to(reference, pose)
    }
    /// append relative x translation to the original Pose
    pub fn then_relative_x(self, mm: f64) -> Self {
        self.then_relative(Self::from_x(mm))
    }
    /// append relative y translation to the original Pose
    pub fn then_relative_y(self, mm: f64) -> Self {
        self.then_relative(Self::from_y(mm))
    }
    /// append relative z translation to the original Pose
    pub fn then_relative_z(self, mm: f64) -> Self {
        self.then_relative(Self::from_z(mm))
    }

    /// append relative vector translation to the original Pose
    pub fn then_relative_vector(self, vector_mm: [f64; 3]) -> Self {
        self.then_relative(Self::from_vector(vector_mm))
    }

    /// append relative rx rotation to the original Pose
    pub fn then_relative_rx(self, degree: f64) -> Self {
        self.then_relative(Self::from_rx(degree))
    }
    /// append relative ry rotation to the original Pose
    pub fn then_relative_ry(self, degree: f64) -> Self {
        self.then_relative(Self::from_ry(degree))
    }
    /// append relative rz rotation to the original Pose
    pub fn then_relative_rz(self, degree: f64) -> Self {
        self.then_relative(Self::from_rz(degree))
    }
    /// append relative euler rotation to the original Pose
    pub fn then_relative_euler(self, eular_degree: [f64; 3]) -> Self {
        self.then_relative(Self::from_euler(eular_degree))
    }

    /// get the euler rotation in radian
    fn radian_euler(&self) -> [f64; 3] {
        let uni = UnitQuaternion::from_quaternion(self.orientation.clone().into_quaternion())
            .euler_angles();
        [uni.0, uni.1, uni.2]
    }
    /// get the vector in `Translation3<f64>`
    fn translation(&self) -> Translation3<f64> {
        Translation3::from(self.position.clone())
    }
    /// get the euler in `UnitQuaterion<f64>`
    fn unit_quaternion(&self) -> UnitQuaternion<f64> {
        let euler = self.radian_euler();
        UnitQuaternion::from_euler_angles(euler[0], euler[1], euler[2])
    }
    /// get the Pose in `Isometry<f64>`
    fn isometry(&self) -> Isometry3<f64> {
        let translation = self.translation();
        let rotation = self.unit_quaternion();
        Isometry3::from_parts(translation, rotation)
    }
    /// compute the inverse of the  Pose
    pub fn inverse(&self) -> Self {
        self.isometry().inverse().into()
    }
    /// interpolate two Pose with a parameter t, scale from 0 to 1
    pub fn interpolate(&self, other: &Self, t: f64) -> Option<Self> {
        self.isometry()
            .try_lerp_slerp(&other.isometry(), t, f64::EPSILON)
            .map(|i| i.into())
    }
}

impl From<String> for Pose {
    fn from(value: String) -> Self {
        value
            .chars()
            .skip_while(|&c| c != 'r')
            .take_while(|&c| c != '}')
            .collect::<String>()
            .replace(['{', '}', ' '], "")
            .split(",")
            .filter_map(|term| {
                let t = term.split(':').collect::<Vec<_>>();

                let k = t.first()?.to_string();

                let v = match t.get(1)?.parse::<f64>() {
                    Ok(f) => f,
                    _ => return None,
                };

                let v = if k.contains('r') {
                    crate::geometry::rad_to_deg(v)
                } else {
                    v * 1000.0
                };

                Some((k, v))
            })
            .collect::<HashMap<String, f64>>()
            .into()
    }
}

impl From<HashMap<String, f64>> for Pose {
    fn from(value: HashMap<String, f64>) -> Pose {
        Pose::identity()
            .set_x(value.get("x").cloned().unwrap_or_default())
            .set_y(value.get("y").cloned().unwrap_or_default())
            .set_z(value.get("z").cloned().unwrap_or_default())
            .set_rx(value.get("rx").cloned().unwrap_or_default())
            .set_ry(value.get("ry").cloned().unwrap_or_default())
            .set_rz(value.get("rz").cloned().unwrap_or_default())
    }
}

impl Mul for Pose {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.isometry() * rhs.isometry()).into()
    }
}

impl Div for Pose {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        rhs.inverse() * self
    }
}

impl Neg for Pose {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.inverse()
    }
}

impl From<Pose> for MotionTarget {
    fn from(val: Pose) -> Self {
        MotionTarget::Transform(val)
    }
}

impl FromRobot for Pose {
    fn from_robot(res: String) -> Result<Self, String> {
        Ok(res.into())
    }
}
