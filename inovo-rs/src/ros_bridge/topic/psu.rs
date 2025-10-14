use super::super::*;

#[inovo_topic("/psu/estop/state", psu_msgs::SafetyCircuitState)]
pub struct EStopState;

#[inovo_topic("/psu/safe_stop/state", psu_msgs::SafetyCircuitState)]
pub struct SafeStopState;

#[inovo_topic("/psu/inputs", psu_msgs::Inputs)]
pub struct Inputs;

#[inovo_topic("/psu/status", psu_msgs::Status)]
pub struct Status;
