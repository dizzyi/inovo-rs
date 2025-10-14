use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::*;

#[inovo_msg("rosgraph_msgs")]
pub struct Clock {
    pub clock: std_msgs::DurationData,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LogLevel {
    #[default]
    Debug = 1,
    Info = 2,
    Warn = 4,
    Error = 8,
    Fatal = 16,
}

#[inovo_msg("rosgraph_msgs")]
pub struct Log {
    pub header: std_msgs::Header,
    pub level: LogLevel,
    pub name: String,
    pub msg: String,
    pub file: String,
    pub function: String,
    pub line: u32,
    pub topics: Vec<String>,
}

#[inovo_msg("rosgraph_msgs")]
pub struct TopicStatistics {
    pub topic: String,
    pub node_pub: String,
    pub node_sub: String,
    pub window_start: std_msgs::DurationData,
    pub window_stop: std_msgs::DurationData,
    pub deliverd_msgs: i32,
    pub dropped_msgs: i32,
    pub traffic: i32,
    pub peroid_mean: std_msgs::DurationData,
    pub peroid_stddev: std_msgs::DurationData,
    pub peroid_max: std_msgs::DurationData,
    pub stamp_age_mean: std_msgs::DurationData,
    pub stamp_age_stddev: std_msgs::DurationData,
    pub stamp_age_max: std_msgs::DurationData,
}
