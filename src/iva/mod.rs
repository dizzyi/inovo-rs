use std::fmt::Display;
use std::vec;

use crate::geometry::{JointCoord, Transform};
use crate::robot::MotionParam;

/// Trait for generating iva tokens from an instruction.
pub trait Iva {
    /// compile the instruction to a vec of String
    fn tokens(&self) -> Vec<String>;
}

/// Enum representing different types of instructions.
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Execute a robot command immediately.
    Execute(RobotCommand),
    /// Enqueue a robot command for later execution.
    Enqueue(RobotCommand),
    /// Dequeue and execute all the enqueued command
    Dequeue,
    /// Control the robot's gripper.
    Gripper(GripperCommand),
    /// Control digital input/output.
    Digital(IOSource, u8, IOType),
    // Get the current pose.
    Current(PoseType),
    // Custom instruction with a list of strings.
    Custom(Vec<String>),
}

impl Instruction {
    const EXECUTE: &'static str = "EXECUTE";
    const ENQUEUE: &'static str = "ENQUEUE";
    const DEQUEUE: &'static str = "DEQUEUE";
    const GRIPPER: &'static str = "GRIPPER";
    const DIGITAL: &'static str = "DIGITAL";
    const CURRENT: &'static str = "CURRENT";
    const CUSTOM: &'static str = "CUSTOM";
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tokens()
                .iter()
                .map(|t| format!("{:>10}", t))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl Into<String> for Instruction {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Iva for Instruction {
    fn tokens(&self) -> Vec<String> {
        match self {
            Instruction::Execute(robot_command) => std::iter::once(Instruction::EXECUTE.to_owned())
                .chain(robot_command.tokens())
                .collect(),
            Instruction::Enqueue(robot_command) => std::iter::once(Instruction::ENQUEUE.to_owned())
                .chain(robot_command.tokens())
                .collect(),
            Instruction::Dequeue => {
                vec![Instruction::DEQUEUE.to_owned()]
            }
            Instruction::Gripper(gripper_command) => {
                std::iter::once(Instruction::GRIPPER.to_owned())
                    .chain(gripper_command.tokens())
                    .collect()
            }
            Instruction::Digital(io_source, port, io_type) => {
                std::iter::once(Instruction::DIGITAL.to_string())
                    .chain(io_source.tokens())
                    .chain(std::iter::once(port.to_string()))
                    .chain(io_type.tokens())
                    .collect()
            }
            Instruction::Current(pose_type) => std::iter::once(Instruction::CURRENT.to_string())
                .chain(pose_type.tokens())
                .collect(),
            Instruction::Custom(s) => std::iter::once(Instruction::CUSTOM.to_string())
                .chain(s.into_iter().cloned())
                .collect(),
        }
    }
}

/// Enum representing different types of robot commands.
#[derive(Debug, Clone)]
pub enum RobotCommand {
    /// Perform a motion with a specific pose and motion type.
    Motion(MotionType, Pose),
    /// Set the motion parameter.
    Param(MotionParam),
    /// Sleep for a specified duration.
    Sleep(f64),
    /// Synchronize the robot.
    Sync,
}

impl RobotCommand {
    const MOTION: &'static str = "MOTION";
    const PARAM: &'static str = "PARAM";
    const SYNC: &'static str = "SYNC";
    const SLEEP: &'static str = "SLEEP";
}

impl Iva for RobotCommand {
    fn tokens(&self) -> Vec<String> {
        match self {
            RobotCommand::Motion(motion_type, pose) => {
                std::iter::once(RobotCommand::MOTION.to_string())
                    .chain(motion_type.tokens())
                    .chain(pose.tokens())
                    .collect()
            }
            RobotCommand::Param(param) => std::iter::once(RobotCommand::PARAM.to_string())
                .chain(param.tokens())
                .collect(),
            RobotCommand::Sync => {
                vec![RobotCommand::SYNC.to_string()]
            }
            RobotCommand::Sleep(second) => {
                vec![RobotCommand::SLEEP.to_string(), format!("{:9.3}", second)]
            }
        }
    }
}

/// Enum representing different types motion
#[derive(Debug, Clone)]
pub enum MotionType {
    Linear,
    LinearRelative,
    Joint,
    JointRelative,
}

impl MotionType {
    const L: &'static str = "L";
    const LR: &'static str = "LR";
    const J: &'static str = "J";
    const JR: &'static str = "JR";
}

impl Iva for MotionType {
    fn tokens(&self) -> Vec<String> {
        match self {
            MotionType::Linear => vec![MotionType::L.to_string()],
            MotionType::LinearRelative => vec![MotionType::LR.to_string()],
            MotionType::Joint => vec![MotionType::J.to_string()],
            MotionType::JointRelative => vec![MotionType::JR.to_string()],
        }
    }
}

/// Enum representing different types of pose with data
#[derive(Debug, Clone)]
pub enum Pose {
    Joint(JointCoord),
    Transform(Transform),
}

impl Pose {
    const JOINT: &str = "JOINT";
    const TRANSFORM: &str = "TRANSFORM";
}

impl Iva for Pose {
    fn tokens(&self) -> Vec<String> {
        match self {
            Pose::Joint(joint) => std::iter::once(Pose::JOINT.to_string())
                .chain(joint.tokens())
                .collect(),
            Pose::Transform(transform) => std::iter::once(Pose::TRANSFORM.to_string())
                .chain(transform.tokens())
                .collect(),
        }
    }
}

/// Enum representing different types of gripper command
#[derive(Debug, Clone)]
pub enum GripperCommand {
    /// activate the gripper
    Activate,
    /// get the current position of the gripper
    Get,
    /// set the position of the gripper to its associated label
    Set(String),
}

impl GripperCommand {
    const ACTIVATE: &'static str = "ACTIVATE";
    const GET: &'static str = "GET";
    const SET: &'static str = "SET";
}

impl Iva for GripperCommand {
    fn tokens(&self) -> Vec<String> {
        match self {
            GripperCommand::Activate => {
                vec![GripperCommand::ACTIVATE.to_string()]
            }
            GripperCommand::Get => {
                vec![GripperCommand::GET.to_string()]
            }
            GripperCommand::Set(s) => {
                vec![GripperCommand::SET.to_string(), s.to_string()]
            }
        }
    }
}

/// Enum representing different types of digital Input/Output source
#[derive(Debug, Clone)]
pub enum IOSource {
    Beckhoff,
    Wrist,
}

impl IOSource {
    const BECKHOFF: &'static str = "BECKHOFF";
    const WRIST: &'static str = "WRIST";
}

impl Iva for IOSource {
    fn tokens(&self) -> Vec<String> {
        match self {
            IOSource::Beckhoff => {
                vec![IOSource::BECKHOFF.to_string()]
            }
            IOSource::Wrist => {
                vec![IOSource::WRIST.to_string()]
            }
        }
    }
}

/// Enum representing different types of digital Input/Output
#[derive(Debug, Clone)]
pub enum IOType {
    Input,
    Output(IOState),
}

impl IOType {
    const INPUT: &'static str = "INPUT";
    const OUTPUT: &'static str = "OUTPUT";
}

impl Iva for IOType {
    fn tokens(&self) -> Vec<String> {
        match self {
            IOType::Input => {
                vec![IOType::INPUT.to_string()]
            }
            IOType::Output(state) => std::iter::once(IOType::OUTPUT.to_string())
                .chain(state.tokens())
                .collect(),
        }
    }
}

/// Enum representing different types of Digital Input/Output state
#[derive(Debug, Clone)]
pub enum IOState {
    High,
    Low,
}

impl IOState {
    const HIGH: &'static str = "HIGH";
    const LOW: &'static str = "LOW";
}

impl Iva for IOState {
    fn tokens(&self) -> Vec<String> {
        match self {
            IOState::High => vec![IOState::HIGH.to_string()],
            IOState::Low => vec![IOState::LOW.to_string()],
        }
    }
}

/// Enum representing different types of pose
#[derive(Debug, Clone)]
pub enum PoseType {
    Frame,
    Joint,
}

impl PoseType {
    const FRAME: &'static str = "FRAME";
    const JOINT: &'static str = "JOINT";
}

impl Iva for PoseType {
    fn tokens(&self) -> Vec<String> {
        match self {
            PoseType::Frame => vec![PoseType::FRAME.to_string()],
            PoseType::Joint => vec![PoseType::JOINT.to_string()],
        }
    }
}
