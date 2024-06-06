//! Module for constructing `IVA` message for communicating with robot

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::geometry::{JointCoord, Transform};
use crate::robot::MotionParam;

/// data structure representing all iva request messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op_code")]
#[serde(rename_all = "snake_case")]
pub enum Instruction {
    Execute {
        #[serde(flatten)]
        robot_command: RobotCommand,
        enter_context: f64,
    },
    Enqueue(RobotCommand),
    Dequeue {
        enter_context: f64,
    },
    Pop,
    Gripper(GripperCommand),
    #[serde(rename = "io")]
    IO {
        target: IOTarget,
        port: u16,
        #[serde(flatten)]
        io_command: IOCommand,
    },
    Get(GetTarget),
    Custom(CustomCommand),
}

impl Instruction {
    pub fn exec(robot_command: RobotCommand) -> Instruction {
        Instruction::Execute {
            robot_command,
            enter_context: 0.0,
        }
    }
    pub fn exec_push(robot_command: RobotCommand) -> Instruction {
        Instruction::Execute {
            robot_command,
            enter_context: 1.0,
        }
    }
    pub fn enqueue(robot_command: RobotCommand) -> Instruction {
        Instruction::Enqueue(robot_command)
    }
    pub fn dequeue() -> Instruction {
        Instruction::Dequeue { enter_context: 0.0 }
    }
    pub fn dequeue_push() -> Instruction {
        Instruction::Dequeue { enter_context: 1.0 }
    }
    pub fn pop() -> Instruction {
        Instruction::Pop
    }

    pub fn get(get_target: GetTarget) -> Instruction {
        Instruction::Get(get_target)
    }

    pub fn gripper(gripper_command: GripperCommand) -> Instruction {
        Instruction::Gripper(gripper_command)
    }

    pub fn io_set(target: IOTarget, port: u16, state: bool) -> Instruction {
        Instruction::IO {
            target,
            port,
            io_command: IOCommand::Set {
                state: if state { 1.0 } else { 0.0 },
            },
        }
    }

    pub fn io_get(target: IOTarget, port: u16) -> Instruction {
        Instruction::IO {
            target,
            port,
            io_command: IOCommand::Get,
        }
    }

    pub fn custom(custom_command: CustomCommand) -> Instruction {
        Instruction::Custom(custom_command)
    }

    pub fn to_json(self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}

/// data structure representing all robot command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum RobotCommand {
    Synchronize,
    Sleep {
        second: f64,
    },
    SetParameter(MotionParam),
    Motion {
        motion_mode: MotionMode,
        #[serde(flatten)]
        target: MotionTarget,
    },
}

impl RobotCommand {
    pub fn sleep(second: f64) -> RobotCommand {
        RobotCommand::Sleep { second }
    }
    pub fn synchorize() -> RobotCommand {
        RobotCommand::Synchronize
    }
    pub fn set_parameter(motion_param: MotionParam) -> RobotCommand {
        RobotCommand::SetParameter(motion_param)
    }
    pub fn linear(target: Transform) -> RobotCommand {
        RobotCommand::Motion {
            motion_mode: MotionMode::Linear,
            target: target.into(),
        }
    }
    pub fn linear_relative(target: Transform) -> RobotCommand {
        RobotCommand::Motion {
            motion_mode: MotionMode::LinearRelative,
            target: target.into(),
        }
    }
    pub fn joint(target: impl Into<MotionTarget>) -> RobotCommand {
        RobotCommand::Motion {
            motion_mode: MotionMode::Joint,
            target: target.into(),
        }
    }
    pub fn joint_relative(target: Transform) -> RobotCommand {
        RobotCommand::Motion {
            motion_mode: MotionMode::JointRelative,
            target: target.into(),
        }
    }
}

/// data structure representing robot motion blend mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionMode {
    Linear,
    LinearRelative,
    Joint,
    JointRelative,
}

/// data structure representing robot motion target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "target")]
#[serde(rename_all = "snake_case")]
pub enum MotionTarget {
    Transform(Transform),
    JointCoord(JointCoord),
}

/// data structure representing robot gripper command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum GripperCommand {
    Activate,
    Get,
    Set { label: String },
}

/// data structure representing psu io target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IOTarget {
    Beckhoff,
    Wrist,
}

/// data structure representing io command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum IOCommand {
    Get,
    Set { state: f64 },
}

/// data structure representing command to get data from robot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "target")]
#[serde(rename_all = "snake_case")]
pub enum GetTarget {
    Transform,
    JointCoord,
    Data { key: String },
}

impl GetTarget {
    pub fn data(key: impl Into<String>) -> GetTarget {
        GetTarget::Data { key: key.into() }
    }
}

/// data structure representing custom command
///
/// the command is a key-value pair with `String` as key and `f64` or `String` as value
///
/// ## Example
/// ```
/// use inovo_rs::iva::*;
///
/// let my_custom_command = CustomCommand::new()
///     .add_string("my_string_key", "my_string_value")
///     .add_float("my_float_key", 69.420);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomCommand(BTreeMap<String, CustomArg>);

impl CustomCommand {
    pub fn new() -> CustomCommand {
        CustomCommand(BTreeMap::default())
    }
    pub fn add_string(mut self, key: impl Into<String>, value: impl Into<String>) -> CustomCommand {
        self.0.insert(key.into(), CustomArg::String(value.into()));
        self
    }
    pub fn add_float(mut self, key: impl Into<String>, value: f64) -> CustomCommand {
        self.0.insert(key.into(), CustomArg::Float(value));
        self
    }
}

/// data structure representing value in custom command
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum CustomArg {
    String(String),
    Float(f64),
}
