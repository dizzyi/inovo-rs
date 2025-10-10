use super::super::*;
use super::*;

#[inovo_topic("/robot/robot_state", arm_msgs::RobotState)]
pub struct RobotState;

#[inovo_topic("/robot/joint_states", sensor_msgs::JointState)]
pub struct JointStates;
