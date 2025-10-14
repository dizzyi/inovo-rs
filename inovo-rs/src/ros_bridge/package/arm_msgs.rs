use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::{std_msgs::Header, std_srvs::ResponseBool};

//=======================================
// Topics
//=======================================

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ArmStateFlag {
    #[default]
    Initalizing = 0,
    Disabled = 1,
    Enabling = 2,
    Enabled = 3,
    Disabling = 4,
    Terminating = 5,
}

#[inovo_msg("arm_msgs")]
pub struct ArmState {
    pub header: Header,
    pub enabled: bool,
    pub state: ArmStateFlag,
    pub joint_states: Vec<JointState>,
}

#[inovo_msg("arm_msgs")]
pub struct JointPositionControllerState {
    pub header: Header,
    pub name: Vec<String>,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub effort: Vec<f64>,
    pub target_position: Vec<f64>,
    pub target_velocity: Vec<f64>,
    pub target_effort: Vec<f64>,
    pub expected_effort: Vec<f64>,
    pub effort_error: Vec<f64>,
    pub effort_error_filtered: Vec<f64>,
    pub collision_detected: Vec<bool>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum JointStateFlag {
    #[default]
    Offline = 0,
    Disabled = 1,
    Enabled = 2,
    Fault = 3,
}

#[inovo_msg("arm_msgs")]
pub struct JointState {
    pub state: JointStateFlag,
    pub age: u8,
    pub status: u8,

    pub position: f64,
    pub velocity: f64,
    pub torque: f64,
    pub current: f64,
    pub target_position: f64,

    pub output_gain: f64,
    pub ff_torque: f64,

    pub motor_temp: f64,
    pub drive_temp: f64,
    pub joint_temp: f64,
}

#[inovo_msg("arm_msgs")]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[inovo_msg("arm_msgs")]
pub struct ModularArmConfig {
    pub joints: Vec<ModularJointConfig>,
}

#[inovo_msg("arm_msgs")]
pub struct ModularJointConfig {
    pub name: String,
    pub part_code: String,
    pub calibration: Vec<f32>,
    pub friction_dynamic: f32,
    pub friction_viscous: f32,
    pub upstream_links: Vec<ModularLinkConfig>,
    pub downstream_links: Vec<ModularLinkConfig>,
}

#[inovo_msg("arm_msgs")]
pub struct ModularLinkConfig {
    pub pard_code: String,
    pub calibration: Vec<f32>,
}

#[inovo_msg("arm_msgs")]
pub struct RobotState {
    pub header: Header,
    pub driver_state: String,
    pub drives_powered: bool,
    pub can_enable: bool,
    pub speed_limited: bool,
}

//=======================================
// Services
//=======================================
#[inovo_msg("arm_msgs")]
#[inovo_req(ModularArmConfig)]
pub struct ArmConfig;

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ArmControlCommand {
    #[default]
    Disable = 0,
    Enable = 1,
}

#[inovo_msg("arm_msgs")]
#[inovo_req(ResponseBool)]
pub struct ArmControl {
    pub command: ArmControlCommand,
}

#[inovo_msg("arm_msgs")]
pub struct ArmInfoResponse {
    pub joint_count: u8,
    pub joint_versions: Vec<String>,
    pub values: Vec<KeyValue>,
}

#[inovo_msg("arm_msgs")]
#[inovo_req(ArmInfoResponse)]
pub struct ArmInfo;

// TODO EnqueueJointTrajectory

// TODO JointTrigger

#[inovo_msg("arm_msgs")]
pub struct PollJointStateResponse {
    pub state: JointState,
}

#[inovo_msg("arm_msgs")]
#[inovo_req(PollJointStateResponse)]
pub struct PollJointState;
