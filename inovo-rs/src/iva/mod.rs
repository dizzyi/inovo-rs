//! Module for constructing `IVA` message for communicating with robot

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use crate::geometry::{JointCoord, Pose};
use crate::robot::MotionParam;

/// data structure representing all iva request messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    Execute {
        robot_command: RobotCommand,
        enter_context: f64,
    },
    Enqueue(RobotCommand),
    Dequeue {
        enter_context: f64,
    },
    Pop,
    Gripper(GripperCommand),
    IO {
        target: IOTarget,
        port: u16,
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

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }

    pub fn to_iva_request(&self) -> Result<IvaRequest, IvaMakeRequestError> {
        let mut req = Default::default();
        self.make_iva_request(&mut req)?;
        Ok(req)
    }
}

impl MakeIvaRequest for Instruction {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        let op_code = match self {
            Instruction::Execute {
                robot_command,
                enter_context,
            } => {
                req.make(robot_command)?;
                req.insert("enter_context", *enter_context)?;
                "execute"
            }
            Instruction::Enqueue(robot_command) => {
                req.make(robot_command)?;
                "enqueue"
            }
            Instruction::Dequeue { enter_context } => {
                req.insert("enter_context", *enter_context)?;
                "dequeue"
            }
            Instruction::Pop => "pop",
            Instruction::Gripper(gripper_command) => {
                req.make(gripper_command)?;
                "gripper"
            }
            Instruction::IO {
                target,
                port,
                io_command,
            } => {
                let target = match target {
                    IOTarget::Beckhoff => "beckhoff",
                    IOTarget::Wrist => "wrist",
                };
                req.insert("target", target)?;
                req.insert("port", *port as f64)?;
                req.make(io_command)?;
                "io"
            }
            Instruction::Get(get_target) => {
                req.make(get_target)?;
                "get"
            }
            Instruction::Custom(custom_command) => {
                req.make(custom_command)?;
                "custom"
            }
        };
        req.insert("op_code", op_code)
    }
}

/// data structure representing all robot command
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RobotCommand {
    Synchronize,
    Sleep {
        second: f64,
    },
    SetParameter(MotionParam),
    Motion {
        motion_mode: MotionMode,
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
    pub fn linear(target: Pose) -> RobotCommand {
        RobotCommand::Motion {
            motion_mode: MotionMode::Linear,
            target: target.into(),
        }
    }
    pub fn linear_relative(target: Pose) -> RobotCommand {
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
    pub fn joint_relative(target: Pose) -> RobotCommand {
        RobotCommand::Motion {
            motion_mode: MotionMode::JointRelative,
            target: target.into(),
        }
    }
}

impl MakeIvaRequest for RobotCommand {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        let action = match self {
            RobotCommand::Synchronize => "synchronize",
            RobotCommand::Sleep { second } => {
                req.insert("second", *second)?;
                "sleep"
            }
            RobotCommand::SetParameter(motion_param) => {
                req.make(motion_param)?;
                "set_parameter"
            }
            RobotCommand::Motion {
                motion_mode,
                target,
            } => {
                let mode = match motion_mode {
                    MotionMode::Linear => "linear",
                    MotionMode::LinearRelative => "linear_relative",
                    MotionMode::Joint => "joint",
                    MotionMode::JointRelative => "joint_relative",
                };
                req.insert("motion_mode", mode)?;
                req.make(target)?;
                "motion"
            }
        };
        req.insert("action", action)
    }
}

/// data structure representing robot motion blend mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MotionMode {
    Linear,
    LinearRelative,
    Joint,
    JointRelative,
}

/// data structure representing robot motion target
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MotionTarget {
    Transform(Pose),
    JointCoord(JointCoord),
}

unsafe impl Send for MotionTarget {}

impl MakeIvaRequest for MotionTarget {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        let target = match self {
            MotionTarget::Transform(t) => {
                req.make(t)?;
                "tranform"
            }
            MotionTarget::JointCoord(j) => {
                req.make(j)?;
                "joint_coord"
            }
        };
        req.insert("target", target)
    }
}

/// data structure representing robot gripper command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GripperCommand {
    Activate,
    Get,
    Set { label: String },
}

impl MakeIvaRequest for GripperCommand {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        let action = match self {
            GripperCommand::Activate => "activate",
            GripperCommand::Get => "get",
            GripperCommand::Set { label } => {
                req.insert("label", label)?;
                "set"
            }
        };
        req.insert("action", action)
    }
}

/// data structure representing psu io target
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IOTarget {
    Beckhoff,
    Wrist,
}

/// data structure representing io command
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IOCommand {
    Get,
    Set { state: f64 },
}

impl MakeIvaRequest for IOCommand {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        let action = match self {
            IOCommand::Get => "get",
            IOCommand::Set { state } => {
                req.insert("state", *state)?;
                "set"
            }
        };
        req.insert("action", action)
    }
}

/// data structure representing command to get data from robot
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl MakeIvaRequest for GetTarget {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        let target = match self {
            GetTarget::Transform => "transform",
            GetTarget::JointCoord => "joint_coord",
            GetTarget::Data { key } => {
                req.insert("key", key)?;
                "data"
            }
        };
        req.insert("target", target)
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
pub struct CustomCommand(BTreeMap<String, IvaArg>);

impl Default for CustomCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomCommand {
    pub fn new() -> CustomCommand {
        CustomCommand(BTreeMap::default())
    }
    pub fn add_string(mut self, key: impl Into<String>, value: impl Into<String>) -> CustomCommand {
        self.0.insert(key.into(), IvaArg::String(value.into()));
        self
    }
    pub fn add_float(mut self, key: impl Into<String>, value: f64) -> CustomCommand {
        self.0.insert(key.into(), IvaArg::Float(value));
        self
    }
}

impl MakeIvaRequest for CustomCommand {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError> {
        for (k, v) in self.0.iter() {
            match v {
                IvaArg::Float(f) => req.insert(k, *f)?,
                IvaArg::String(s) => req.insert(k, s)?,
                IvaArg::Bool(b) => req.insert(k, *b)?,
                IvaArg::UInt(u) => req.insert(k, *u)?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IvaArg {
    String(String),
    UInt(u32),
    Float(f64),
    Bool(bool),
}

impl IvaArg {
    fn to_value(&self) -> serde_json::Value {
        match self {
            IvaArg::String(s) => serde_json::Value::from(s.clone()),
            IvaArg::UInt(u) => serde_json::Value::from(*u),
            IvaArg::Float(f) => serde_json::Value::from(*f),
            IvaArg::Bool(b) => serde_json::Value::from(*b),
        }
    }
}

impl<'a> From<&'a str> for IvaArg {
    fn from(value: &'a str) -> Self {
        IvaArg::String(value.to_string())
    }
}

impl From<String> for IvaArg {
    fn from(value: String) -> Self {
        IvaArg::String(value)
    }
}
impl From<&String> for IvaArg {
    fn from(value: &String) -> Self {
        IvaArg::String(value.clone())
    }
}

impl From<f64> for IvaArg {
    fn from(value: f64) -> Self {
        IvaArg::Float(value)
    }
}
impl From<u32> for IvaArg {
    fn from(value: u32) -> Self {
        IvaArg::UInt(value)
    }
}
impl From<bool> for IvaArg {
    fn from(value: bool) -> Self {
        IvaArg::Float(if value { 1.0 } else { 0.0 })
    }
}

#[derive(Debug, Clone, Default)]
pub struct IvaRequest(pub HashMap<String, IvaArg>);

impl IvaRequest {
    pub(crate) fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<IvaArg>,
    ) -> Result<(), IvaMakeRequestError> {
        let k = key.into();
        let v = value.into();
        if let Some(v_exist) = self.0.insert(k.clone(), v.clone()) {
            return Err(IvaMakeRequestError::DuplicatedKey {
                key: k,
                v_insert: v,
                v_exist,
            });
        }
        Ok(())
    }
    fn make(&mut self, obj: &impl MakeIvaRequest) -> Result<(), IvaMakeRequestError> {
        obj.make_iva_request(self)
    }
    pub fn to_string_pretty(&self) -> String {
        let dict = self
            .0
            .iter()
            .map(|(k, v)| (k, v.to_value()))
            .collect::<serde_json::Value>();
        serde_json::to_string_pretty(&dict).unwrap()
    }

    pub fn to_string(&self) -> String {
        let dict = self
            .0
            .iter()
            .map(|(k, v)| (k, v.to_value()))
            .collect::<serde_json::Value>();
        serde_json::to_string(&dict).unwrap()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum IvaMakeRequestError {
    #[error("Encounter duplicated key `{key}` while making request (inserting {v_insert:?}, existing {v_exist:?})")]
    DuplicatedKey {
        key: String,
        v_insert: IvaArg,
        v_exist: IvaArg,
    },
}

pub trait MakeIvaRequest {
    fn make_iva_request(&self, req: &mut IvaRequest) -> Result<(), IvaMakeRequestError>;
}
