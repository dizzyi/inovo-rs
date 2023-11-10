use std::convert::{TryFrom, TryInto};
use std::ops::{Add, Neg, Sub};

use crate::geometry::IntoPose;
use crate::iva::{Iva, Pose};

/// A structure representing a 6 joint coordinate, in degree
#[derive(Debug, Clone)]
pub struct JointCoord {
    q: [f64; 6],
}

impl JointCoord {
    /// create a new joint coord identity
    pub fn identity() -> Self {
        JointCoord { q: [0_f64; 6] }
    }
    /// create a new joint coord from array
    pub fn new(q: [f64; 6]) -> Self {
        Self { q }
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
        self.q[0] = degree;
        self
    }
    /// set the joint 2 of the joint coord
    pub fn set_j2(mut self, degree: f64) -> Self {
        self.q[1] = degree;
        self
    }
    /// set the joint 3 of the joint coord
    pub fn set_j3(mut self, degree: f64) -> Self {
        self.q[2] = degree;
        self
    }
    /// set the joint 4 of the joint coord
    pub fn set_j4(mut self, degree: f64) -> Self {
        self.q[3] = degree;
        self
    }
    /// set the joint 5 of the joint coord
    pub fn set_j5(mut self, degree: f64) -> Self {
        self.q[4] = degree;
        self
    }
    /// set the joint 6 of the joint coord
    pub fn set_j6(mut self, degree: f64) -> Self {
        self.q[5] = degree;
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

    /// interpolate two joint coord with a parameter t, scale from 0 to 1
    pub fn interpolate(&self, other: &Self, t: f64) -> Self {
        let a: JointCoord = self.q.map(|f| f * (1.0 - t)).into();
        a + other.q.map(|f| f * t).into()
    }
}

impl From<[f64; 6]> for JointCoord {
    fn from(value: [f64; 6]) -> Self {
        Self { q: value }
    }
}

impl From<&[f64; 6]> for JointCoord {
    fn from(value: &[f64; 6]) -> Self {
        Self {
            q: value.to_owned(),
        }
    }
}

impl From<[&f64; 6]> for JointCoord {
    fn from(value: [&f64; 6]) -> Self {
        Self {
            q: value.map(|t| t.to_owned()).into(),
        }
    }
}

impl Neg for JointCoord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            q: self.q.map(|t| t.neg()),
        }
    }
}

impl Add for JointCoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut arr = [0_f64; 6];
        for i in 0..6 {
            arr[i] = self.q[i] + rhs.q[i];
        }
        Self { q: arr }
    }
}

impl Sub for JointCoord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

impl Iva for JointCoord {
    fn tokens(&self) -> Vec<String> {
        self.q.iter().map(|q| format!("{:8.2}", q)).collect()
    }
}

impl IntoPose for JointCoord {
    fn into_pose(self) -> Pose {
        Pose::Joint(self)
    }
}

impl TryFrom<String> for JointCoord {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .replace(&['[', ']'], "")
            .split(",")
            .map(|s| s.parse::<f64>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<f64>, _>>()?
            .try_into()
    }
}

impl TryFrom<Vec<f64>> for JointCoord {
    type Error = String;
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        Ok(Self {
            q: value.try_into().map_err(|e| format!("{:?}", e))?,
        })
    }
}
