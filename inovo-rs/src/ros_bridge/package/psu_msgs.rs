use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::std_msgs::Header;

#[inovo_msg("psu_msgs")]
pub struct Inputs {
    pub config_in_1: bool,
    pub config_in_2: bool,
}

#[inovo_msg("psu_msgs")]
pub struct SafetyCircuitState {
    pub active: bool,
    pub circuit_complete: bool,
}

#[inovo_msg("psu_msgs")]
pub struct Status {
    pub header: Header,
    pub voltage: f32,
    pub current: f32,
    pub state: String,
    pub fault_code: i32,
}

#[inovo_msg("psu_msgs")]
pub struct GetStringResponse {
    pub success: bool,
    pub data: String,
}

#[inovo_msg("psu_msgs")]
#[inovo_srv(GetStringResponse)]
pub struct GetString;
