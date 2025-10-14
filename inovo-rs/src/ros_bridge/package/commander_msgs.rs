use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::{
    actionlib_msgs, geometry_msgs,
    std_msgs::{Header, Time},
    std_srvs::Response,
};

//=======================================
// Topics
//=======================================

#[inovo_msg("commander_msgs")]
pub struct BlockError {
    pub header: Header,
    pub block_id: String,
    pub message: String,
}

#[inovo_msg("commander_msgs")]
pub struct BlockLog {
    pub header: Header,
    pub block_id: String,
    pub message: String,
}

#[inovo_msg("commander_msgs")]
pub struct Blockly {
    pub token: String,
    pub name: String,
    pub saved: bool,
    pub blockly: String,
}

#[inovo_msg("commander_msgs")]
pub struct CartesianJogDemand {
    pub header: Header,
    pub twist: super::geometry_msgs::Twist,
    pub tcp_id: String,
}

#[inovo_msg("commander_msgs")]
pub struct LinAng {
    pub linear: f64,
    pub angular: f64,
}

#[inovo_msg("commander_msgs")]
pub struct MotionAction {
    pub action_goal: MotionActionGoal,
    pub action_result: MotionActionResult,
    pub action_feedback: MotionActionFeedback,
}

#[inovo_msg("commander_msgs")]
pub struct MotionActionFeedback {
    pub header: Header,
    pub status: actionlib_msgs::GoalStatus,
    pub feedback: MotionFeedback,
}

#[inovo_msg("commander_msgs")]
pub struct MotionActionGoal {
    pub header: Header,
    pub goal_id: actionlib_msgs::GoalID,
    pub goal: MotionGoal,
}

#[inovo_msg("commander_msgs")]
pub struct MotionActionResult {
    pub header: Header,
    pub status: actionlib_msgs::GoalStatus,
    pub result: MotionResult,
}

#[inovo_msg("commander_msgs")]
pub struct MotionFeedback {
    pub progress: f64,
    pub index: i32,
    pub time_from_start: f64,
}

#[inovo_msg("commander_msgs")]
pub struct MotionGoal {
    pub motion_sequence: Vec<MotionSequencePoint>,
    pub end_effector: String,
    pub ignore_scaling: bool,
}

#[inovo_msg("commander_msgs")]
pub struct MotionResult {
    pub success: bool,
    pub message: String,
    pub index: i32,
}

#[inovo_msg("commander_msgs")]
pub struct MotionSequencePoint {
    pub pose: geometry_msgs::Pose,
    pub frame_id: String,
    pub tcp_id: String,
    pub relative: bool,
    pub use_joint_space_target: bool,
    pub use_nearest_joint_space_target: bool,
    pub joint_names: Vec<String>,
    pub joint_angles: Vec<f64>,
    pub unwind_joint_target: bool,
    pub max_velocity: LinAng,
    pub max_joint_velocity: f64,
    pub max_joint_acceleration: f64,
    pub blend: LinAng,
    pub cartesian: bool,
}

#[inovo_msg("commander_msgs")]
pub struct MoveGroupInfo {
    pub controlled_joints: Vec<String>,
    pub tcp_link_name: String,
}

#[inovo_msg("commander_msgs")]
pub struct ProjectMeta {
    pub name: String,
    pub modified: Time,
    pub file_size_bytes: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PromptInputType {
    #[default]
    None = 0,
    Text = 1,
    Number = 2,
}

#[inovo_msg("commander_msgs")]
pub struct Prompt {
    pub header: Header,
    pub prompt_id: String,
    pub input: PromptInputType,
    pub block_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RuntimeStatus {
    #[default]
    Idle = 0,
    Running = 1,
    Paused = 2,
    PausedOnError = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub dtype: String,
    pub value: String,
}

#[inovo_msg("commander_msgs")]
pub struct RuntimeState {
    pub active_blocks: Vec<String>,
    pub current_block_progress: f64,
    pub state: RuntimeStatus,
    pub variables: Vec<Variable>,
}

#[inovo_msg("commander_msgs")]
pub struct Speed {
    pub linear: f64,
    pub angular: f64,
}

#[inovo_msg("commander_msgs")]
pub struct SpeedStamped {
    pub header: Header,
    pub speed: Speed,
}

#[inovo_msg("commander_msgs")]
pub struct Update {
    pub header: Header,
    pub token: String,
    pub blockly: String,
}

//=======================================
// Services
//=======================================

#[inovo_msg("commander_msgs")]
#[inovo_req(Response)]
pub struct DeleteVariable {
    pub name: String,
}

#[inovo_msg("commander_msgs")]
pub struct DownloadResponse {
    pub name: String,
    pub blockly: String,
}

#[inovo_msg("commander_msgs")]
#[inovo_req(DownloadResponse)]
pub struct Download;

#[inovo_msg("commander_msgs")]
pub struct GetVariableResponse {
    pub value: String,
    pub success: bool,
    pub message: String,
}

#[inovo_msg("commander_msgs")]
#[inovo_req(GetVariableResponse)]
pub struct GetVariable {
    pub name: String,
}

// TODO Insert

#[inovo_msg("commander_msgs")]
#[inovo_req(ProjectMeta)]
pub struct ListProject;

// TODO NewProject

#[inovo_msg("commander_msgs")]
// pub struct ProjectResponse {
//     pub success: bool,
//     pub reason: String,
//     pub message: String
// }
pub struct ProjectResponse(serde_json::Value); // TODO

pub struct Project {
    pub name: String,
    pub force: bool,
}

// TODO PromptResponse

#[inovo_msg("commander_msgs")]
#[inovo_req(Response)]
pub struct RunSequence {
    pub procedure_name: String,
    pub variables_names: Vec<String>,
    pub variables_values: Vec<String>,
}

// TODO SetCursor

#[inovo_msg("commander_msgs")]
#[inovo_req(Response)]
pub struct SetVariable {
    pub name: String,
    pub value: String,
}

// TODO UpdateSavedConfiguration

// TODO Upload
