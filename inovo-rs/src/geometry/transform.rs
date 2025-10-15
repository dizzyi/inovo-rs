use nalgebra::geometry::{Isometry3, UnitQuaternion};
use nalgebra::Translation3;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::iva::{MakeIvaRequest, MotionTarget};
use crate::robot::FromRobot;

pub use crate::ros_bridge::geometry_msgs::Pose;

use crate::ros_bridge::geometry_msgs::{Point, Quaternion};

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
    ///
    /// q is in mm and deg.
    pub fn from_array(q_mm_deg: [f64; 6]) -> Self {
        Self::new(
            q_mm_deg[0],
            q_mm_deg[1],
            q_mm_deg[2],
            q_mm_deg[3],
            q_mm_deg[4],
            q_mm_deg[5],
        )
    }
    /// create a new Pose from vector only
    pub fn from_vector(vector_mm: [f64; 3]) -> Self {
        Pose::new(vector_mm[0], vector_mm[1], vector_mm[2], 0.0, 0.0, 0.0)
    }
    /// create a new Pose from euler only
    pub fn from_euler(euler_degree: [f64; 3]) -> Self {
        Pose::new(
            0.0,
            0.0,
            0.0,
            euler_degree[0],
            euler_degree[1],
            euler_degree[2],
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
    pub fn set_euler(mut self, euler_degree: [f64; 3]) -> Self {
        self.orientation =
            UnitQuaternion::from_euler_angles(euler_degree[0], euler_degree[1], euler_degree[2])
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

    /// set the rx component of the Pose
    pub fn set_rx(mut self, degree: f64) -> Self {
        self.orientation = self.orientation.set_rx(degree);
        self
    }
    /// set the ry component of the Pose
    pub fn set_ry(mut self, degree: f64) -> Self {
        self.orientation = self.orientation.set_ry(degree);
        self
    }
    /// set the rz component of the Pose
    pub fn set_rz(mut self, degree: f64) -> Self {
        self.orientation = self.orientation.set_rz(degree);
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
    pub fn then_euler(self, euler_degree: [f64; 3]) -> Self {
        Self::from_euler(euler_degree) * self
    }

    /// create a new Pose by extracting the vector part
    pub fn vector_only(&self) -> Self {
        // Self::from_vector(self.get_vector().to_owned())
        Pose {
            position: self.position,
            orientation: Default::default(),
        }
    }
    /// create a new Pose by extracting the euler part
    pub fn rotation_only(&self) -> Self {
        // Self::from_euler(self.get_euler().to_owned())
        Pose {
            position: Default::default(),
            orientation: self.orientation,
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
    pub fn then_relative_euler(self, euler_degree: [f64; 3]) -> Self {
        self.then_relative(Self::from_euler(euler_degree))
    }

    /// get the euler rotation in radian
    fn radian_euler(&self) -> [f64; 3] {
        let uni = self.orientation.into_unit_quaternion().euler_angles();
        [uni.0, uni.1, uni.2]
    }
    /// get the vector in `Translation3<f64>`
    fn translation(&self) -> Translation3<f64> {
        Translation3::from(self.position)
    }
    /// get the euler in `UnitQuaterion<f64>`
    fn unit_quaternion(&self) -> UnitQuaternion<f64> {
        self.orientation.into_unit_quaternion()
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

impl From<Pose> for MotionTarget {
    fn from(val: Pose) -> Self {
        MotionTarget::Transform(val)
    }
}

impl FromRobot for Pose {
    fn from_robot(res: &String) -> Result<Self, String> {
        let map = res
            .replace(['{', '}', ' '], "")
            .split(",")
            .filter_map(|term| {
                let t = term.split(':').collect::<Vec<_>>();

                let k = t.first()?.to_string();

                let v = match t.get(1)?.parse::<f64>() {
                    Ok(f) => f,
                    _ => return None,
                };

                Some((k, v))
            })
            .collect::<HashMap<String, f64>>();

        let mut pose = Pose::identity();

        pose.position.x = map.get("x").cloned().unwrap_or_default();
        pose.position.y = map.get("y").cloned().unwrap_or_default();
        pose.position.z = map.get("z").cloned().unwrap_or_default();

        pose.orientation = UnitQuaternion::from_euler_angles(
            map.get("rx").cloned().unwrap_or_default(),
            map.get("ry").cloned().unwrap_or_default(),
            map.get("rz").cloned().unwrap_or_default(),
        )
        .into();

        Ok(pose)
    }
}

impl MakeIvaRequest for Pose {
    fn make_iva_request(
        &self,
        req: &mut crate::iva::IvaRequest,
    ) -> Result<(), crate::iva::IvaMakeRequestError> {
        req.insert("x", self.position.x * 1000.0)?;
        req.insert("y", self.position.y * 1000.0)?;
        req.insert("z", self.position.z * 1000.0)?;
        let euler = self.radian_euler();
        req.insert("rx", euler[0].to_degrees())?;
        req.insert("ry", euler[1].to_degrees())?;
        req.insert("rz", euler[2].to_degrees())?;
        Ok(())
    }
}

// Point
impl Point {
    pub fn new(mut point_mm: [f64; 3]) -> Self {
        for v in &mut point_mm {
            *v /= 1000.0
        }
        Self {
            x: point_mm[0],
            y: point_mm[1],
            z: point_mm[2],
        }
    }
    // Constructor
    pub fn identity() -> Self {
        Self::default()
    }
    pub fn from_x(mm: f64) -> Self {
        Self::new([mm, 0.0, 0.0])
    }
    pub fn from_y(mm: f64) -> Self {
        Self::new([0.0, mm, 0.0])
    }
    pub fn from_z(mm: f64) -> Self {
        Self::new([0.0, 0.0, mm])
    }
    // Builder
    pub fn set_x(mut self, mm: f64) -> Self {
        self.x = mm / 1000.0;
        self
    }
    pub fn set_y(mut self, mm: f64) -> Self {
        self.y = mm / 1000.0;
        self
    }
    pub fn set_z(mut self, mm: f64) -> Self {
        self.z = mm / 1000.0;
        self
    }
    pub fn then(self, offset: Point) -> Self {
        self + offset
    }
    pub fn then_x(mut self, mm: f64) -> Self {
        self.x += mm / 1000.0;
        self
    }
    pub fn then_y(mut self, mm: f64) -> Self {
        self.y += mm / 1000.0;
        self
    }
    pub fn then_z(mut self, mm: f64) -> Self {
        self.z += mm / 1000.0;
        self
    }

    pub fn lenght(&self) -> f64 {
        self.into_vector().norm()
    }
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

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, mut rhs: Point) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;
        rhs
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        -1.0_f64 * self
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        self + -rhs
    }
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

// Quaternion
impl Quaternion {
    pub fn from_euler(mut euler_deg: [f64; 3]) -> Self {
        for v in &mut euler_deg {
            *v = v.to_radians();
        }
        nalgebra::UnitQuaternion::from_euler_angles(euler_deg[0], euler_deg[1], euler_deg[2]).into()
    }
    // Constructor
    pub fn identity() -> Self {
        Self::from_euler([0.0, 0.0, 0.0])
    }
    pub fn from_rx(deg: f64) -> Self {
        Self::from_euler([deg, 0.0, 0.0])
    }
    pub fn from_ry(deg: f64) -> Self {
        Self::from_euler([0.0, deg, 0.0])
    }
    pub fn from_rz(deg: f64) -> Self {
        Self::from_euler([0.0, 0.0, deg])
    }

    // Builder
    pub fn set_rx(self, deg: f64) -> Self {
        let mut euler = self.into_unit_quaternion().euler_angles();
        euler.0 = deg.to_radians();
        UnitQuaternion::from_euler_angles(euler.0, euler.1, euler.2).into()
    }
    pub fn set_ry(self, deg: f64) -> Self {
        let mut euler = self.into_unit_quaternion().euler_angles();
        euler.1 = deg.to_radians();
        UnitQuaternion::from_euler_angles(euler.0, euler.1, euler.2).into()
    }
    pub fn set_rz(self, deg: f64) -> Self {
        let mut euler = self.into_unit_quaternion().euler_angles();
        euler.2 = deg.to_radians();
        UnitQuaternion::from_euler_angles(euler.0, euler.1, euler.2).into()
    }

    pub fn then(self, rot: Self) -> Self {
        rot * self
    }
    pub fn then_rx(self, deg: f64) -> Self {
        Self::from_rx(deg) * self
    }
    pub fn then_ry(self, deg: f64) -> Self {
        Self::from_rx(deg) * self
    }
    pub fn then_rz(self, deg: f64) -> Self {
        Self::from_rx(deg) * self
    }

    pub fn angle(&self) -> f64 {
        self.into_unit_quaternion().angle()
    }
    pub fn inverse(&self) -> Self {
        self.into_unit_quaternion().inverse().into()
    }
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

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        (self.into_unit_quaternion() * rhs.into_unit_quaternion()).into()
    }
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
