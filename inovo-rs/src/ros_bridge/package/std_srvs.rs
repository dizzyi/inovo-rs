use derive_more::{Deref, DerefMut};
use roslibrust::{RosMessageType, RosServiceType};
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;

#[inovo_msg("std_srvs")]
pub struct ResponseBool {
    pub success: bool,
}

#[inovo_msg("std_srvs")]
pub struct Response {
    pub success: bool,
    pub message: String,
}

#[inovo_msg("std_srvs")]
#[inovo_req(())]
pub struct Empty;

#[inovo_msg("std_srvs")]
#[derive(Deref, DerefMut)]
pub struct SetBool {
    pub data: bool,
}

#[inovo_msg("std_srvs")]
#[inovo_req(Response)]
pub struct Trigger;
