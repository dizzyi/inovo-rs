use serde::{Deserialize, Serialize};

use crate::ros_bridge::std_msgs::Header;
use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

//=======================================
// Topics
//=======================================

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    #[default]
    RobotArm,
    Gripper,
    VacuumGripper,
    ForceTorque,
    JoyStick,
    PSU,
    EPick,
    OnRobotRg6,
    OnRobotRg2,
    OnRobot2fg7,
    Camera,
    EvoCam2,
    OnRobotScrewdriver,
    GenericModbusDriver,
}

#[inovo_msg("device_msgs")]
pub struct DeviceDescription {
    pub header: Header,
    pub device_type: DeviceType,
    pub device_subtype: String,
    pub ns: String,
    pub driver: String,
    pub descriptive_name: String,
}

#[inovo_msg("device_msgs")]
pub struct DeviceDescriptionArray {
    pub devices: Vec<DeviceDescription>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum DeviceStatusFlag {
    #[default]
    Unknown = -1,
    Offline = 0,
    Inactive = 1,
    Active = 2,
}
#[inovo_msg("device_msgs")]
pub struct DeviceStatus {
    pub status: DeviceStatusFlag,
}
#[inovo_msg("device_msgs")]
pub struct RobotDescription {
    pub urdf: String,
    pub default_tcp_id: String,
    pub joint_prefix: String,
}

//=======================================
// Service
//=======================================

#[inovo_msg("device_msgs")]
pub struct GripperInfoResonse {
    pub gripper_name: String,
    pub max_aperture: f32,
    pub min_aperture: f32,
    pub max_effort: f32,
}

#[inovo_msg("device_msgs")]
#[inovo_req(GripperInfoResonse)]
pub struct GripperInfo;

#[inovo_msg("device_msgs")]
pub struct IOInfoResponse {
    pub io_device_name: String,
    pub num_dout: u32,
    pub num_din: u32,
}

#[inovo_msg("device_msgs")]
#[inovo_req(GripperInfoResonse)]
pub struct IOInfo;

// TODO JoyInfo
