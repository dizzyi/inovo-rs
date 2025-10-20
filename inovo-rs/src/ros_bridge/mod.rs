use std::fmt::Debug;

pub mod package;
pub mod service;
pub mod topic;

pub use package::*;

use service::*;
use topic::*;

use crate::util::{InovorsError, ToWsUrl};

use arm_msgs::*;
use commander_msgs::*;
use geometry_msgs::*;
use psu_msgs::*;
use std_srvs::*;

pub use roslibrust::rosbridge::{
    ClientHandle, ClientHandleOptions, Publisher, ServiceClient, ServiceHandle, Subscriber,
};
pub use roslibrust::{self, Error, Publish, RosMessageType, RosServiceType, ServiceError};

use inovo_rs_macro::*;

pub async fn rosbridge_connect(
    host: impl Into<String>,
    timeout: std::time::Duration,
) -> Result<ClientHandle, roslibrust::Error> {
    ClientHandle::new_with_options(ClientHandleOptions::new(host.to_ws_url()).timeout(timeout))
        .await
}

pub async fn start_seq(
    host: impl Into<String>,
    procedure_name: impl Into<String>,
) -> Result<(), InovorsError> {
    let procedure_name = procedure_name.into();

    let client = rosbridge_connect(host, std::time::Duration::from_millis(5000)).await?;

    let req = package::commander_msgs::RunSequence {
        procedure_name: procedure_name.clone(),
        ..Default::default()
    };
    let response1 = service::sequence::Start::call(&client, req).await?;
    if response1.success {
        return Ok(());
    }

    let req = package::commander_msgs::RunSequenceRequest {
        procedure_name: procedure_name,
    };
    let response2 = service::sequence::StartRequest::call(&client, req).await?;
    if response2.success {
        return Ok(());
    }

    let msg = format!(
        "start error msg: {}, start-request error msg: {}",
        response1.message, response2.message
    );

    Err(InovorsError::PSUError(msg))
}

#[async_trait::async_trait]
pub trait InovoRosBridge {
    async fn subscribe_inovo<T: topic::Topic>(&self) -> roslibrust::Result<Subscriber<T::Message>>;

    async fn service_inovo<S: service::Service>(&self)
        -> roslibrust::Result<ServiceClient<S::Req>>;

    async fn call_inovo<S: service::Service>(
        &self,
        args: S::Req,
    ) -> roslibrust::Result<<S::Req as RosServiceType>::Response>;

    async fn tcp_speed(&self) -> roslibrust::Result<Subscriber<SpeedStamped>> {
        self.subscribe_inovo::<default_move_group::TcpSpeed>().await
    }
    async fn tcp_pose(&self) -> roslibrust::Result<Subscriber<PoseStamped>> {
        self.subscribe_inovo::<default_move_group::TcpPose>().await
    }
    async fn joint_state(&self) -> roslibrust::Result<Subscriber<sensor_msgs::JointState>> {
        self.subscribe_inovo::<topic::robot::JointStates>().await
    }
    async fn psu_status(&self) -> roslibrust::Result<Subscriber<psu_msgs::Status>> {
        self.subscribe_inovo::<topic::psu::Status>().await
    }
    async fn robot_state(&self) -> roslibrust::Result<Subscriber<RobotState>> {
        self.subscribe_inovo::<topic::robot::RobotState>().await
    }
    async fn estop_state(&self) -> roslibrust::Result<Subscriber<SafetyCircuitState>> {
        self.subscribe_inovo::<topic::psu::EStopState>().await
    }
    async fn safe_stop_state(&self) -> roslibrust::Result<Subscriber<SafetyCircuitState>> {
        self.subscribe_inovo::<topic::psu::SafeStopState>().await
    }
    async fn runtime_state(&self) -> roslibrust::Result<Subscriber<RuntimeState>> {
        self.subscribe_inovo::<topic::sequence::RuntimeState>()
            .await
    }
    async fn jog(&self) -> roslibrust::Result<Subscriber<CartesianJogDemand>> {
        self.subscribe_inovo::<topic::default_move_group::CartesianJog>()
            .await
    }

    // Service

    // psu
    async fn safe_stop_reset(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::psu::SafeStopReset>(Trigger {})
            .await
    }
    async fn estop_reset(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::psu::EStopReset>(Trigger {})
            .await
    }
    async fn power_on(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::psu::Enable>(Trigger {}).await
    }
    async fn power_off(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::psu::Disable>(Trigger {}).await
    }

    // robot
    async fn arm_enable(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::robot::Enable>(Trigger {}).await
    }
    async fn arm_disable(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::robot::Disable>(Trigger {}).await
    }

    // sequence
    async fn sequence_start(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::sequence::StartRequest>(RunSequenceRequest::default())
            .await
    }
    async fn sequence_stop(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::sequence::Stop>(Trigger {}).await
    }
    async fn sequence_pause(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::sequence::Pause>(Trigger {})
            .await
    }
    async fn sequence_step(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::sequence::Step>(Trigger {}).await
    }
    async fn sequence_continue(&self) -> roslibrust::Result<Response> {
        self.call_inovo::<service::sequence::Continue>(Trigger {})
            .await
    }

    async fn sequence_function(
        &self,
        procedure_name: impl Into<String> + Send,
    ) -> roslibrust::Result<Response> {
        self.call_inovo::<service::sequence::StartRequest>(RunSequenceRequest {
            procedure_name: procedure_name.into(),
            ..Default::default()
        })
        .await
    }

    // Publish
    async fn jog_pub(&self) -> roslibrust::Result<Publisher<CartesianJogDemand>>;
}

#[async_trait::async_trait]
impl InovoRosBridge for ClientHandle {
    async fn subscribe_inovo<T: topic::Topic>(&self) -> roslibrust::Result<Subscriber<T::Message>> {
        self.subscribe::<T::Message>(T::NAME).await
    }
    async fn service_inovo<S: service::Service>(
        &self,
    ) -> roslibrust::Result<ServiceClient<S::Req>> {
        S::service(self).await
    }
    async fn call_inovo<S: service::Service>(
        &self,
        args: S::Req,
    ) -> roslibrust::Result<<S::Req as RosServiceType>::Response> {
        S::call(self, args).await
    }
    async fn jog_pub(&self) -> roslibrust::Result<Publisher<CartesianJogDemand>> {
        topic::default_move_group::CartesianJog::advertise(self).await
    }
}
