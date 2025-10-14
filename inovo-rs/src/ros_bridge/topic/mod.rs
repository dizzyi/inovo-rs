use inovo_rs_macro::inovo_topic;
use roslibrust::rosbridge::{ClientHandle, Subscriber};
use roslibrust::{Result, RosMessageType};

use crate::ros_bridge::*;

pub mod default_move_group;
pub mod psu;
pub mod robot;
pub mod sequence;

pub mod beckhoff_io {
    use super::*;
    #[inovo_topic("/beckhoff_io/io", gpio_msgs::IOState)]
    pub struct IO;
}

#[async_trait::async_trait]
pub trait Topic {
    const NAME: &'static str;
    type Message: RosMessageType;

    async fn subscribe(client: &ClientHandle) -> Result<Subscriber<Self::Message>> {
        client.subscribe(Self::NAME).await
    }
}

#[inovo_topic("/rosout", rosgraph_msgs::Log)]
pub struct RosOut;
