use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::geometry::IntoPose;
use crate::iva::*;

use super::MotionParam;

/// A struct to hold a list of robot commands
/// # Example
/// ```ignore
/// let command_sequence = CommandSequence::new()
///     .then(RobotCommand::Motion(MotionType::Linear, JointCoord::identity()))
///     .then_linear_relative(Transform::from_z(-10.0))
///     .then_set_param(&MotionParam::default())
///     .then_sleep(10.0)
///     .then_sync();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSequence {
    seq: Vec<RobotCommand>,
}

impl CommandSequence {
    /// create a new empty sequence
    pub fn new() -> Self {
        Self { seq: vec![] }
    }

    /// append a new robot command
    pub fn then(mut self, robot_command: RobotCommand) -> Self {
        self.seq.push(robot_command);
        self
    }

    /// append a linear motion with a specified pose
    pub fn then_linear(self, pose: impl IntoPose) -> Self {
        self.then(RobotCommand::Motion(MotionType::Linear, pose.into_pose()))
    }
    /// append a linear relative motion with a specified pose
    pub fn then_linear_relative(self, pose: impl IntoPose) -> Self {
        self.then(RobotCommand::Motion(
            MotionType::LinearRelative,
            pose.into_pose(),
        ))
    }
    /// append a joint motion with a specified pose
    pub fn then_joint(self, pose: impl IntoPose) -> Self {
        self.then(RobotCommand::Motion(MotionType::Joint, pose.into_pose()))
    }
    /// append a joint relative motion with a specified pose
    pub fn then_joint_relative(self, pose: impl IntoPose) -> Self {
        self.then(RobotCommand::Motion(
            MotionType::JointRelative,
            pose.into_pose(),
        ))
    }
    /// append a sleep command
    pub fn then_sleep(self, second: f64) -> Self {
        self.then(RobotCommand::Sleep(second))
    }
    /// append a synchorize command
    pub fn then_sync(self) -> Self {
        self.then(RobotCommand::Sync)
    }
    /// append a set param command
    pub fn then_set_param(self, param: &MotionParam) -> Self {
        self.then(RobotCommand::Param(param.clone()))
    }
}

impl Deref for CommandSequence {
    type Target = Vec<RobotCommand>;
    fn deref(&self) -> &Self::Target {
        &self.seq
    }
}
impl DerefMut for CommandSequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.seq
    }
}

impl FromIterator<RobotCommand> for CommandSequence {
    fn from_iter<T: IntoIterator<Item = RobotCommand>>(iter: T) -> Self {
        Self {
            seq: Vec::from_iter(iter),
        }
    }
}
