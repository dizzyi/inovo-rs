use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use crate::ros_bridge::{std_msgs::Header, std_srvs};
use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

//=======================================
// Topics
//=======================================
#[inovo_msg("gpio_msgs")]
pub struct IOState {
    pub header: Header,
    pub digital_inputs: Vec<i8>,
    pub digital_outputs: Vec<i8>,
    pub analog_inputs: Vec<f32>,
    pub analog_outputs: Vec<f32>,
}

//=======================================
// Service
//=======================================
#[inovo_msg("gpio_msgs")]
#[inovo_req(std_srvs::Response)]
pub struct AnalogWrite {
    pub pin: i32,
    pub value: f32,
}
#[inovo_msg("gpio_msgs")]
pub struct DigitalReadResponse {
    pub value: i8,
    pub success: bool,
    pub message: String,
}

#[inovo_msg("gpio_msgs")]
#[inovo_req(DigitalReadResponse)]
pub struct DigitalRead {
    pub pin: i8,
}

#[inovo_msg("gpio_msgs")]
#[inovo_req(std_srvs::Response)]
pub struct DigitalWrite {
    pub pin: i8,
    pub value: i8,
}
