use crate::geometry::IntoPose;
use crate::iva::Iva;

use nalgebra::geometry::{Isometry3, UnitQuaternion};
use nalgebra::Translation3;
use std::f64::consts::PI;
use std::ops::{Div, Mul, Neg};

/// A structure representing a 3D Transformation
#[derive(Debug, Clone)]
pub struct Transform {
    vector: [f64; 3],
    euler: [f64; 3],
}

impl Transform {
    /// create a new identity transform
    pub fn identity() -> Self {
        Transform {
            vector: [0_f64; 3],
            euler: [0_f64; 3],
        }
    }
    /// create a new transform from vector and euler angle
    pub fn new(vector_mm: [f64; 3], euler_degree: [f64; 3]) -> Self {
        Self {
            vector: vector_mm,
            euler: euler_degree,
        }
    }
    /// create a new transform from an array containing vector and euler angle
    pub fn from(q: [f64; 6]) -> Self {
        Self::new([q[0], q[1], q[2]], [q[3], q[4], q[5]])
    }
    /// create a new transform from vector only
    pub fn from_vector(vector_mm: [f64; 3]) -> Self {
        Self {
            vector: vector_mm,
            euler: [0_f64; 3],
        }
    }
    /// create a new transform from euler only
    pub fn from_euler(eular_degree: [f64; 3]) -> Self {
        Self {
            vector: [0_f64; 3],
            euler: eular_degree,
        }
    }
    /// create a new transform from x component
    pub fn from_x(mm: f64) -> Self {
        Self::identity().set_x(mm)
    }
    /// create a new transform from y component
    pub fn from_y(mm: f64) -> Self {
        Self::identity().set_y(mm)
    }
    /// create a new transform from z component
    pub fn from_z(mm: f64) -> Self {
        Self::identity().set_z(mm)
    }
    /// create a new transform from rx component
    pub fn from_rx(degree: f64) -> Self {
        Self::identity().set_rx(degree)
    }
    /// create a new transform from ry component
    pub fn from_ry(degree: f64) -> Self {
        Self::identity().set_ry(degree)
    }
    /// create a new transform from rz component
    pub fn from_rz(degree: f64) -> Self {
        Self::identity().set_rz(degree)
    }

    /// get the vector of the transform
    pub fn get_vector(&self) -> &[f64; 3] {
        &self.vector
    }
    /// get the euler of the transform
    pub fn get_euler(&self) -> &[f64; 3] {
        &self.euler
    }
    /// get the x component of the transform
    pub fn get_x(&self) -> f64 {
        self.vector[0]
    }
    /// get the y component of the transform
    pub fn get_y(&self) -> f64 {
        self.vector[1]
    }
    /// get the z component of the transform
    pub fn get_z(&self) -> f64 {
        self.vector[2]
    }
    /// get the rx component of the transform
    pub fn get_rx(&self) -> f64 {
        self.euler[0]
    }
    /// get the ry component of the transform
    pub fn get_ry(&self) -> f64 {
        self.euler[1]
    }
    /// get the rz component of the transform
    pub fn get_rz(&self) -> f64 {
        self.euler[2]
    }
    /// set the vector of the transform
    pub fn set_vector(mut self, vector_mm: [f64; 3]) -> Self {
        self.vector = vector_mm;
        self
    }
    /// set the euler of the transform
    pub fn set_euler(mut self, eular_degree: [f64; 3]) -> Self {
        self.euler = eular_degree;
        self
    }
    /// set the x component of the transform
    pub fn set_x(mut self, mm: f64) -> Self {
        self.vector[0] = mm;
        self
    }
    /// set the y component of the transform
    pub fn set_y(mut self, mm: f64) -> Self {
        self.vector[1] = mm;
        self
    }
    /// set the z component of the transform
    pub fn set_z(mut self, mm: f64) -> Self {
        self.vector[2] = mm;
        self
    }
    /// set the rx component of the transform
    pub fn set_rx(mut self, degree: f64) -> Self {
        self.euler[0] = degree;
        self
    }
    /// set the ry component of the transform
    pub fn set_ry(mut self, degree: f64) -> Self {
        self.euler[1] = degree;
        self
    }
    /// set the rz component of the transform
    pub fn set_rz(mut self, degree: f64) -> Self {
        self.euler[2] = degree;
        self
    }

    /// create a new transform from inovo robot message
    pub fn from_robot(value: String) -> Result<Self, String> {
        value
            .replace(&['{', '}', ' '], "")
            .split(",")
            .map(|kv| kv.split(":").collect::<Vec<&str>>())
            .collect::<Vec<_>>()
            .split_last()
            .ok_or(format!("error spliting last"))?
            .1
            .iter()
            .fold(Ok(Transform::identity()), |acc, kv| {
                if let Ok(t) = acc {
                    match (kv.get(0), kv.get(1)) {
                        (Some(&k), Some(&v)) => {
                            if let Ok(f) = v.parse::<f64>() {
                                match k {
                                    "x" => Ok(t.set_x(f * 1000.0)),
                                    "y" => Ok(t.set_y(f * 1000.0)),
                                    "z" => Ok(t.set_z(f * 1000.0)),
                                    "rx" => Ok(t.set_rx(f / PI * 180.0)),
                                    "ry" => Ok(t.set_ry(f / PI * 180.0)),
                                    "rz" => Ok(t.set_rz(f / PI * 180.0)),
                                    _ => Err(format!("unknown key {}", k)),
                                }
                            } else {
                                Err(format!("cannot parse {} to f64", v))
                            }
                        }
                        _ => Err(format!("kv value error {:?}", kv)),
                    }
                } else {
                    acc
                }
            })
    }

    /// append a new transform to the original transform
    pub fn then(self, transform: Self) -> Self {
        transform * self
    }
    /// append x translation to the original transform
    pub fn then_x(self, mm: f64) -> Self {
        Self::from_x(mm) * self
    }
    /// append y translation to the original transform
    pub fn then_y(self, mm: f64) -> Self {
        Self::from_y(mm) * self
    }
    /// append z translation to the original transform
    pub fn then_z(self, mm: f64) -> Self {
        Self::from_z(mm) * self
    }
    /// append vector translation to the original transform
    pub fn then_vector(self, vector_mm: [f64; 3]) -> Self {
        Self::from_vector(vector_mm) * self
    }

    /// append rx rotation to the original transform
    pub fn then_rx(self, degree: f64) -> Self {
        Self::from_rx(degree) * self
    }
    /// append ry rotation to the original transform
    pub fn then_ry(self, degree: f64) -> Self {
        Self::from_ry(degree) * self
    }
    /// append rz rotation to the original transform
    pub fn then_rz(self, degree: f64) -> Self {
        Self::from_rz(degree) * self
    }
    /// append euler rotation to the original transform
    pub fn then_euler(self, eular_degree: [f64; 3]) -> Self {
        Self::from_euler(eular_degree) * self
    }

    /// create a new transform by extracting the vector part
    pub fn vector_only(&self) -> Self {
        Self::from_vector(self.get_vector().to_owned())
    }
    /// create a new transform by extracting the euler part
    pub fn eular_only(&self) -> Self {
        Self::from_euler(self.get_euler().to_owned())
    }

    /// append relative transform to the original transform, relative to a reference
    pub fn then_relative_to(mut self, reference: Self, transform: Self) -> Self {
        self = reference.clone().inverse() * self;
        self = transform * self;
        reference * self
    }
    /// append relative transform to the original transform, relatice to the vector part of original transform
    pub fn then_relative(self, transform: Self) -> Self {
        self.clone().then_relative_to(self.vector_only(), transform)
    }
    /// append relative x translation to the original transform
    pub fn then_relative_x(self, mm: f64) -> Self {
        self.then_relative(Self::from_x(mm))
    }
    /// append relative y translation to the original transform
    pub fn then_relative_y(self, mm: f64) -> Self {
        self.then_relative(Self::from_y(mm))
    }
    /// append relative z translation to the original transform
    pub fn then_relative_z(self, mm: f64) -> Self {
        self.then_relative(Self::from_z(mm))
    }

    /// append relative vector translation to the original transform
    pub fn then_relative_vector(self, vector_mm: [f64; 3]) -> Self {
        self.then_relative(Self::from_vector(vector_mm))
    }

    /// append relative rx rotation to the original transform
    pub fn then_relative_rx(self, degree: f64) -> Self {
        self.then_relative(Self::from_rx(degree))
    }
    /// append relative ry rotation to the original transform
    pub fn then_relative_ry(self, degree: f64) -> Self {
        self.then_relative(Self::from_ry(degree))
    }
    /// append relative rz rotation to the original transform
    pub fn then_relative_rz(self, degree: f64) -> Self {
        self.then_relative(Self::from_rz(degree))
    }
    /// append relative euler rotation to the original transform
    pub fn then_relative_euler(self, eular_degree: [f64; 3]) -> Self {
        self.then_relative(Self::from_euler(eular_degree))
    }

    /// get the euler rotation in radian
    fn radian_euler(&self) -> [f64; 3] {
        self.euler.map(|p| p / 180.0 * PI)
    }
    /// get the vector in `Translation3<f64>`
    fn translation(&self) -> Translation3<f64> {
        Translation3::from(self.vector)
    }
    /// get the euler in `UnitQuaterion<f64>`
    fn unit_quaternion(&self) -> UnitQuaternion<f64> {
        let euler = self.radian_euler();
        UnitQuaternion::from_euler_angles(euler[0], euler[1], euler[2])
    }
    /// get the transform in `Isometry<f64>`
    fn isometry(&self) -> Isometry3<f64> {
        let translation = self.translation();
        let rotation = self.unit_quaternion();
        Isometry3::from_parts(translation, rotation)
    }
    /// compute the inverse of the  transform
    pub fn inverse(&self) -> Self {
        self.isometry().inverse().into()
    }
    /// interpolate two transform with a parameter t, scale from 0 to 1
    pub fn interpolate(&self, other: &Self, t: f64) -> Option<Self> {
        self.isometry()
            .try_lerp_slerp(&other.isometry(), t, f64::EPSILON)
            .map(|i| i.into())
    }
}

impl From<Isometry3<f64>> for Transform {
    fn from(value: Isometry3<f64>) -> Self {
        let vector = value.translation.vector.into();
        let euler = value.rotation.euler_angles();
        let euler = [euler.0, euler.1, euler.2].map(|p| p / PI * 180.0);
        Self { vector, euler }
    }
}

impl Mul for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.isometry() * rhs.isometry()).into()
    }
}

impl Div for Transform {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        rhs.inverse() * self
    }
}

impl Neg for Transform {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.inverse()
    }
}

impl Iva for Transform {
    fn tokens(&self) -> Vec<String> {
        self.vector
            .map(|p| format!("{:8.2}", p))
            .iter()
            .chain(&self.euler.map(|p| format!("{:8.2}", p)))
            .cloned()
            .collect()
    }
}

impl IntoPose for Transform {
    fn into_pose(self) -> crate::iva::Pose {
        crate::iva::Pose::Transform(self)
    }
}
