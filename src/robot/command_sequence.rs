use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::geometry::*;
use crate::iva::*;
use crate::robot::MotionParam;

/// A struct to hold a list of robot commands
/// # Example
/// ```
/// use inovo_rs::robot::*;
/// use inovo_rs::iva::*;
/// use inovo_rs::geometry::*;
///
/// let command_sequence = CommandSequence::new()
///     .then(RobotCommand::joint(JointCoord::identity()))
///     .then_linear_relative(Transform::from_z(-10.0))
///     .then_set_param(MotionParam::default())
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

    /// append a linear motion with a specified target
    pub fn then_linear(self, target: Transform) -> Self {
        self.then(RobotCommand::linear(target))
    }
    /// append a linear relative motion with a specified target
    pub fn then_linear_relative(self, target: Transform) -> Self {
        self.then(RobotCommand::linear_relative(target))
    }
    /// append a joint motion with a specified target
    pub fn then_joint(self, target: impl Into<MotionTarget>) -> Self {
        self.then(RobotCommand::joint(target))
    }
    /// append a joint relative motion with a specified target
    pub fn then_joint_relative(self, target: Transform) -> Self {
        self.then(RobotCommand::joint_relative(target))
    }
    /// append a sleep command
    pub fn then_sleep(self, second: f64) -> Self {
        self.then(RobotCommand::Sleep { second })
    }
    /// append a synchorize command
    pub fn then_sync(self) -> Self {
        self.then(RobotCommand::Synchronize)
    }
    /// append a set param command
    pub fn then_set_param(self, param: MotionParam) -> Self {
        self.then(RobotCommand::SetParameter(param))
    }
}

impl IntoIterator for CommandSequence {
    type Item = RobotCommand;
    type IntoIter = std::vec::IntoIter<RobotCommand>;
    fn into_iter(self) -> Self::IntoIter {
        self.seq.into_iter()
    }
}

impl Deref for CommandSequence {
    type Target = Vec<RobotCommand>;
    fn deref(&self) -> &Self::Target {
        &self.seq
    }
}

impl FromIterator<RobotCommand> for CommandSequence {
    fn from_iter<T: IntoIterator<Item = RobotCommand>>(iter: T) -> Self {
        Self {
            seq: Vec::from_iter(iter),
        }
    }
}
