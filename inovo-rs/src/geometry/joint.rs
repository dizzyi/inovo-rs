use std::ops::{Add, Neg, Sub};

use serde::{Deserialize, Serialize};

use crate::iva::MotionTarget;
use crate::robot::FromRobot;

/// A structure representing a 6 joint coordinate, in degree
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JointCoord {
    j1: f64,
    j2: f64,
    j3: f64,
    j4: f64,
    j5: f64,
    j6: f64,
}

impl JointCoord {
    /// create a new joint coord identity
    pub fn identity() -> Self {
        JointCoord::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
    /// create a new joint coord from array
    pub fn new(
        j1_deg: f64,
        j2_deg: f64,
        j3_deg: f64,
        j4_deg: f64,
        j5_deg: f64,
        j6_deg: f64,
    ) -> Self {
        JointCoord {
            j1: j1_deg,
            j2: j2_deg,
            j3: j3_deg,
            j4: j4_deg,
            j5: j5_deg,
            j6: j6_deg,
        }
    }

    /// create a new joint coord from joint 1
    pub fn from_j1(degree: f64) -> Self {
        JointCoord::identity().set_j1(degree)
    }
    /// create a new joint coord from joint 2
    pub fn from_j2(degree: f64) -> Self {
        JointCoord::identity().set_j2(degree)
    }
    /// create a new joint coord from joint 3
    pub fn from_j3(degree: f64) -> Self {
        JointCoord::identity().set_j3(degree)
    }
    /// create a new joint coord from joint 4
    pub fn from_j4(degree: f64) -> Self {
        JointCoord::identity().set_j4(degree)
    }
    /// create a new joint coord from joint 5
    pub fn from_j5(degree: f64) -> Self {
        JointCoord::identity().set_j5(degree)
    }
    /// create a new joint coord from joint 6
    pub fn from_j6(degree: f64) -> Self {
        JointCoord::identity().set_j6(degree)
    }

    /// set the joint 1 of the joint coord
    pub fn set_j1(mut self, degree: f64) -> Self {
        self.j1 = degree;
        self
    }
    /// set the joint 2 of the joint coord
    pub fn set_j2(mut self, degree: f64) -> Self {
        self.j2 = degree;
        self
    }
    /// set the joint 3 of the joint coord
    pub fn set_j3(mut self, degree: f64) -> Self {
        self.j3 = degree;
        self
    }
    /// set the joint 4 of the joint coord
    pub fn set_j4(mut self, degree: f64) -> Self {
        self.j4 = degree;
        self
    }
    /// set the joint 5 of the joint coord
    pub fn set_j5(mut self, degree: f64) -> Self {
        self.j5 = degree;
        self
    }
    /// set the joint 6 of the joint coord
    pub fn set_j6(mut self, degree: f64) -> Self {
        self.j6 = degree;
        self
    }

    /// append rotation on joint 1
    pub fn then_j1(self, degree: f64) -> Self {
        self + JointCoord::from_j1(degree)
    }
    /// append rotation on joint 2
    pub fn then_j2(self, degree: f64) -> Self {
        self + JointCoord::from_j2(degree)
    }
    /// append rotation on joint 3
    pub fn then_j3(self, degree: f64) -> Self {
        self + JointCoord::from_j3(degree)
    }
    /// append rotation on joint 4
    pub fn then_j4(self, degree: f64) -> Self {
        self + JointCoord::from_j4(degree)
    }
    /// append rotation on joint 5
    pub fn then_j5(self, degree: f64) -> Self {
        self + JointCoord::from_j5(degree)
    }
    /// append rotation on joint 6
    pub fn then_j6(self, degree: f64) -> Self {
        self + JointCoord::from_j6(degree)
    }

    pub fn into_array(self) -> [f64; 6] {
        self.into()
    }

    pub fn scale(&self, factor: f64) -> JointCoord {
        self.clone().into_array().map(|v| v * factor).into()
    }

    /// interpolate two joint coord with a parameter t, scale from 0 to 1
    pub fn interpolate(&self, other: &Self, t: f64) -> Self {
        self.scale(1.0 - t) + other.scale(t)
    }
}

impl From<[f64; 6]> for JointCoord {
    fn from(value: [f64; 6]) -> Self {
        JointCoord::new(value[0], value[1], value[2], value[3], value[4], value[5])
    }
}

impl From<&[f64; 6]> for JointCoord {
    fn from(value: &[f64; 6]) -> Self {
        JointCoord::from(value.to_owned())
    }
}

impl Into<[f64; 6]> for JointCoord {
    fn into(self) -> [f64; 6] {
        [self.j1, self.j2, self.j3, self.j4, self.j5, self.j6]
    }
}

impl Neg for JointCoord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.into_array().map(|v| v.neg()).into()
    }
}

impl Add for JointCoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut arr = self.into_array();
        let other_arr = rhs.into_array();
        for i in 0..6 {
            arr[i] += other_arr[i];
        }
        arr.into()
    }
}

impl Sub for JointCoord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

impl From<String> for JointCoord {
    fn from(value: String) -> JointCoord {
        value
            .chars()
            .skip_while(|&c| c != '[')
            .take_while(|&c| c != ']')
            .collect::<String>()
            .replace(&['[', ']', ' '][..], "")
            .split(",")
            .filter_map(|s| s.parse::<f64>().ok())
            .map(|f| crate::geometry::rad_to_deg(f))
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<Vec<f64>> for JointCoord {
    fn from(value: Vec<f64>) -> JointCoord {
        [
            value.get(0).cloned().unwrap_or_default(),
            value.get(1).cloned().unwrap_or_default(),
            value.get(2).cloned().unwrap_or_default(),
            value.get(3).cloned().unwrap_or_default(),
            value.get(4).cloned().unwrap_or_default(),
            value.get(5).cloned().unwrap_or_default(),
        ]
        .into()
    }
}

impl Into<MotionTarget> for JointCoord {
    fn into(self) -> MotionTarget {
        MotionTarget::JointCoord(self)
    }
}

impl FromRobot for JointCoord {
    fn from_robot(res: String) -> Result<Self, String> {
        Ok(res.into())
    }
}
