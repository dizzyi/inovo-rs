use super::super::*;

#[inovo_service("/robot/arm_info", arm_msgs::ArmInfo)]
pub struct ArmInfo;

#[inovo_service("/robot/disable", std_srvs::Trigger)]
pub struct Disable;

#[inovo_service("/robot/enable", std_srvs::Trigger)]
pub struct Enable;

// TODO InovoDriverGetLoggers

// TODO InovoDriverSetLoggerLevel

// TODO RefreshNodeIDs

#[inovo_service("/robot/reset", std_srvs::Trigger)]
pub struct Reset;

// TODO SwitchController

// TODO SwitchToPassiveMode
