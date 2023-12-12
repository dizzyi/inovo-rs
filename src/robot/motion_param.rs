use std::f64::consts::PI;

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::iva::Iva;

use super::IvaRobot;

/// A struct represent the Motion Param of the robot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionParam {
    speed: f64,
    accel: f64,
    blend_linear: f64,
    blend_angular: f64,
    tcp_speed_linear: f64,
    tcp_speed_angular: f64,
}

impl MotionParam {
    const DEFAULT_SPEED: f64 = 50.0;
    const DEFAULT_ACCEL: f64 = 50.0;
    const DEFAULT_BLEND_LINEAR: f64 = 0.01;
    const DEFAULT_BLEND_ANGULAR: f64 = 0.01;
    const DEFAULT_TCP_SPEED_LINEAR: f64 = 1000.0;
    const DEFAULT_TCP_SPEED_ANGULAR: f64 = 720.0;

    const MIN_PERCENT: f64 = 0.01;
    const MAX_PERCENT: f64 = 100.0;
    const MIN_LENGHT: f64 = 0.01;
    const MAX_LENGHT: f64 = 1000.0;
    const MIN_ANGLE: f64 = 0.001;
    const MAX_ANGLE: f64 = 720.0;

    /// constructor for `MotionParam`
    pub fn new(
        speed: f64,
        accel: f64,
        blend_linear: f64,
        blend_angular: f64,
        tcp_speed_linear: f64,
        tcp_speed_angular: f64,
    ) -> Self {
        Self {
            speed,
            accel,
            blend_linear,
            blend_angular,
            tcp_speed_linear,
            tcp_speed_angular,
        }
        .clamp()
    }

    /// clamp the value of its parameter
    pub fn clamp(mut self) -> Self {
        self.speed = self
            .speed
            .clamp(MotionParam::MIN_PERCENT, MotionParam::MAX_PERCENT);

        self.accel = self
            .accel
            .clamp(MotionParam::MIN_PERCENT, MotionParam::MAX_PERCENT);

        self.blend_linear = self
            .blend_linear
            .clamp(MotionParam::MIN_LENGHT, MotionParam::MAX_LENGHT);

        self.blend_angular = self
            .blend_angular
            .clamp(MotionParam::MIN_ANGLE, MotionParam::MAX_ANGLE);

        self.tcp_speed_linear = self
            .tcp_speed_linear
            .clamp(MotionParam::MIN_LENGHT, MotionParam::MAX_LENGHT);

        self.tcp_speed_angular = self
            .tcp_speed_angular
            .clamp(MotionParam::MIN_ANGLE, MotionParam::MAX_ANGLE);

        self
    }

    /// set the speed parameter, and clamp values
    pub fn set_speed(mut self, percent: f64) -> Self {
        self.speed = percent;
        self.clamp()
    }
    /// set the accel parameter, and clamp values
    pub fn set_accel(mut self, percent: f64) -> Self {
        self.accel = percent;
        self.clamp()
    }
    /// set the blend linear parameter, and clamp values
    pub fn set_blend_linear(mut self, mm: f64) -> Self {
        self.blend_linear = mm;
        self.clamp()
    }
    /// set the blend angular parameter, and clamp values
    pub fn set_blend_angular(mut self, degree: f64) -> Self {
        self.blend_angular = degree;
        self.clamp()
    }
    /// set the tcp speed linear speed limit parameter, and clamp values
    pub fn set_tcp_speed_linear(mut self, mm: f64) -> Self {
        self.tcp_speed_linear = mm;
        self.clamp()
    }
    /// set the tcp speed angular speed limit parameter, and clamp values
    pub fn set_tcp_speed_angular(mut self, degree: f64) -> Self {
        self.tcp_speed_angular = degree;
        self.clamp()
    }

    /// getter for the speed
    pub fn get_speed(&self) -> f64 {
        self.speed
    }
    /// getter for the accel
    pub fn get_accel(&self) -> f64 {
        self.accel
    }
    /// getter for the blend linear
    pub fn get_blend_linear(&self) -> f64 {
        self.blend_linear
    }
    /// getter for the blend angular
    pub fn get_blend_angular(&self) -> f64 {
        self.blend_angular
    }
    /// getter for the tcp speed linear
    pub fn get_tcp_speed_linear(&self) -> f64 {
        self.tcp_speed_linear
    }
    /// getter for the tcp speed angular
    pub fn get_tcp_speed_angular(&self) -> f64 {
        self.tcp_speed_angular
    }
}

impl Default for MotionParam {
    fn default() -> Self {
        MotionParam::new(
            MotionParam::DEFAULT_SPEED,
            MotionParam::DEFAULT_ACCEL,
            MotionParam::DEFAULT_BLEND_LINEAR,
            MotionParam::DEFAULT_BLEND_ANGULAR,
            MotionParam::DEFAULT_TCP_SPEED_LINEAR,
            MotionParam::DEFAULT_TCP_SPEED_ANGULAR,
        )
    }
}

impl Iva for MotionParam {
    fn tokens(&self) -> Vec<String> {
        vec![
            self.speed / 100.0,
            self.accel / 100.0,
            self.blend_linear / 1000.0,
            self.blend_angular / 180.0 * PI,
            self.tcp_speed_linear / 1000.0,
            self.tcp_speed_angular / 180.0 * PI,
        ]
        .into_iter()
        .map(|v| format!("{:8.5}", v))
        .collect()
    }
}

/// A context for motion parameter management
///
/// when enter, push a motion parameter to robot
///
/// when exit, pop a motion parameter from robot
pub struct ParamContext {
    param: MotionParam,
}
impl ParamContext {
    pub fn new(param: MotionParam) -> Self {
        Self { param }
    }
}

impl Into<ParamContext> for MotionParam {
    fn into(self) -> ParamContext {
        ParamContext::new(self)
    }
}

impl<T> Context<T> for ParamContext
where
    T: IvaRobot,
{
    fn enter_fn(&mut self, t: &mut T) -> Result<(), String> {
        t.push_param(&self.param)?;
        Ok(())
    }
    fn exit_fn(&mut self, t: &mut T) -> Result<(), String> {
        t.pop_param()?;
        Ok(())
    }
    fn label(&self) -> String {
        format!("Motion Param Context {:?} ", self.param)
    }
}
