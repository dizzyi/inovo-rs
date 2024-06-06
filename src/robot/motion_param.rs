use serde::{Deserialize, Serialize};

use crate::geometry::deg_to_rad;

/// Data structure representing robot's motion parameter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MotionParam {
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    accel: f64,
    #[serde(default)]
    blend_linear: f64,
    #[serde(default)]
    blend_angular: f64,
    #[serde(default)]
    tcp_speed_linear: f64,
    #[serde(default)]
    tcp_speed_angular: f64,
}

impl MotionParam {
    pub const MIN_PRECENT: f64 = 1.0;
    pub const MAX_PRECENT: f64 = 100.0;
    pub const MIN_LENGHT: f64 = 1.0;
    pub const MAX_LENGHT: f64 = 1000.0;
    pub const MIN_ANGLE: f64 = 1.0;
    pub const MAX_ANGLE: f64 = 720.0;

    /// create a new with all field unset.
    pub fn new() -> MotionParam {
        MotionParam::default()
    }
    /// set speed with percent, clamp to [`MotionParam::MIN_PRECENT`] and [`MotionParam::MAX_PRECENT`]
    pub fn set_speed(mut self, percent: f64) -> MotionParam {
        self.speed = percent.clamp(MotionParam::MIN_PRECENT, MotionParam::MAX_PRECENT) / 100.0;
        self
    }
    /// set accel with percent, clamp to [`MotionParam::MIN_PRECENT`] and [`MotionParam::MAX_PRECENT`]
    pub fn set_accel(mut self, percent: f64) -> MotionParam {
        self.accel = percent.clamp(MotionParam::MIN_PRECENT, MotionParam::MAX_PRECENT) / 100.0;
        self
    }
    /// set linear blend with percent, clamp to [`MotionParam::MIN_LENGHT`] and [`MotionParam::MAX_LENGHT`]
    pub fn set_blend_linear(mut self, mm: f64) -> MotionParam {
        self.blend_linear = mm.clamp(MotionParam::MIN_LENGHT, MotionParam::MAX_LENGHT) / 1000.0;
        self
    }
    /// set angular blend with percent, clamp to [`MotionParam::MIN_ANGLE`] and [`MotionParam::MAX_ANGLE]
    pub fn set_blend_angular(mut self, deg: f64) -> MotionParam {
        self.blend_angular = deg_to_rad(deg.clamp(MotionParam::MIN_ANGLE, MotionParam::MAX_ANGLE));
        self
    }
    /// set linear tcp speed limit with percent, clamp to [`MotionParam::MIN_LENGHT`] and [`MotionParam::MAX_LENGHT`]
    pub fn set_tcp_speed_linear(mut self, mm: f64) -> MotionParam {
        self.tcp_speed_linear = mm.clamp(MotionParam::MIN_LENGHT, MotionParam::MAX_LENGHT) / 1000.0;
        self
    }
    /// set linear blend with percent, clamp to [`MotionParam::MIN_ANGLE`] and [`MotionParam::MAX_ANGLE]
    pub fn set_tcp_speed_angular(mut self, deg: f64) -> MotionParam {
        self.tcp_speed_angular =
            deg_to_rad(deg.clamp(MotionParam::MIN_ANGLE, MotionParam::MAX_ANGLE));
        self
    }
}
