use super::super::*;

#[inovo_service("/psu/disable", std_srvs::Trigger)]
pub struct Disable;

#[inovo_service("/psu/enable", std_srvs::Trigger)]
pub struct Enable;

#[inovo_service("/psu/estop/reset", std_srvs::Trigger)]
pub struct EStopReset;

// TODO get_fw_version

#[inovo_service("/psu/reset_fault", std_srvs::Trigger)]
pub struct ResetFault;

#[inovo_service("/psu/safe_stop/reset", std_srvs::Trigger)]
pub struct SafeStopReset;

#[inovo_service("/psu/safe_stop_trip", std_srvs::Trigger)]
pub struct SafeStopTrip;
