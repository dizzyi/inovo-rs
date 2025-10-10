use roslibrust::rosbridge::{ClientHandle, Subscriber};
use roslibrust::{Result, RosMessageType};

pub mod robot;

#[async_trait::async_trait]
pub trait Topic {
    const NAME: &'static str;
    type Message: RosMessageType;

    async fn subscribe(client: &ClientHandle) -> Result<Subscriber<Self::Message>> {
        client.subscribe(Self::NAME).await
    }
}
