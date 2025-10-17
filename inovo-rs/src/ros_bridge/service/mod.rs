use roslibrust::rosbridge::{ClientHandle, ServiceClient};
use roslibrust::{Result, RosServiceType};

pub mod beckhoff_io;
pub mod psu;
pub mod robot;
pub mod sequence;

#[async_trait::async_trait]
pub trait Service: Send {
    const NAME: &'static str;
    type Req: RosServiceType<Request = Self::Req>;

    async fn service(client: &ClientHandle) -> Result<ServiceClient<Self::Req>> {
        client.service_client(Self::NAME).await
    }

    async fn call(
        client: &ClientHandle,
        request: Self::Req,
    ) -> Result<<Self::Req as RosServiceType>::Response> {
        client
            .service_client::<Self::Req>(Self::NAME)
            .await?
            .call(request)
            .await
    }
}
