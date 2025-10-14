use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::*;

#[inovo_msg("actionlib_msgs")]
pub struct GoalID {
    pub stamp: std_msgs::DurationData,
    pub id: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GoalStatusFlag {
    #[default]
    Pending = 0,
    Active = 1,
    Preempted = 2,
    Succeeded = 3,
    Aborted = 4,
    Rejected = 5,
    Preempting = 6,
    Recalling = 7,
    Recalled = 8,
    Lost = 9,
}

#[inovo_msg("actionlib_msgs")]
pub struct GoalStatus {
    pub goal_id: GoalID,
    pub status: GoalStatusFlag,
    pub text: String,
}

#[inovo_msg("actionlib_msgs")]
pub struct GoalStatusArray {
    pub header: std_msgs::Header,
    pub status_list: Vec<GoalStatus>,
}
