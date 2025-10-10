use std::fmt::Debug;

pub mod package;
pub mod topic;

pub use package::*;
pub use topic::*;

use arm_msgs::*;
use commander_msgs::*;
use geometry_msgs::*;
use psu_msgs::*;
use std_srvs::*;

use roslibrust::rosbridge::{ClientHandle, Publisher, Subscriber};
use roslibrust::{RosMessageType, RosServiceType};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::*;

use inovo_rs_macro::*;

type InovoRosResult<T> = Result<T, roslibrust::Error>;

#[async_trait::async_trait]
pub trait InovoRosBridge {
    const TOPIC_ROBOT_STATE: &'static str = "/robot/robot_state";
    const TOPIC_POWER_STATE: &'static str = "/psu/status";
    const TOPIC_JOINT_STATE: &'static str = "/robot/joint_states";
    const TOPIC_TCP_POSE: &'static str = "/default_move_group/tcp_pose";
    const TOPIC_TCP_SPEED: &'static str = "/default_move_group/tcp_speed";
    const TOPIC_CARTESIAN_JOG: &'static str = "/default_move_group/cartesian_jog";
    const TOPIC_ESTOP_STATE: &'static str = "/psu/estop/state";
    const TOPIC_SAFE_STOP_STATE: &'static str = "/psu/safe_stop/state";
    const TOPIC_ARM_STATE: &'static str = "/robot/arm_state";
    const TOPIC_RUNTIME_STATE: &'static str = "/sequence/runtime_state";

    async fn subscribe_inovo<T: RosMessageType>(
        &self,
        topic: impl AsRef<str> + Send,
    ) -> InovoRosResult<Subscriber<T>>;

    async fn call_inovo<S: RosServiceType>(
        &self,
        service: impl AsRef<str> + Send,
        args: S::Request,
    ) -> InovoRosResult<S::Response>;

    async fn tcp_speed(&self) -> InovoRosResult<Subscriber<SpeedStamped>> {
        self.subscribe_inovo(Self::TOPIC_TCP_SPEED).await
    }
    async fn tcp_pose(&self) -> InovoRosResult<Subscriber<PoseStamped>> {
        self.subscribe_inovo(Self::TOPIC_TCP_POSE).await
    }
    async fn joint_state(&self) -> InovoRosResult<Subscriber<JointState>> {
        self.subscribe_inovo(Self::TOPIC_JOINT_STATE).await
    }
    async fn power_state(&self) -> InovoRosResult<Subscriber<psu_msgs::Status>> {
        self.subscribe_inovo(Self::TOPIC_POWER_STATE).await
    }
    async fn robot_state(&self) -> InovoRosResult<Subscriber<RobotState>> {
        self.subscribe_inovo(Self::TOPIC_ROBOT_STATE).await
    }
    async fn estop_state(&self) -> InovoRosResult<Subscriber<SafetyCircuitState>> {
        self.subscribe_inovo(Self::TOPIC_ESTOP_STATE).await
    }
    async fn safe_stop_state(&self) -> InovoRosResult<Subscriber<SafetyCircuitState>> {
        self.subscribe_inovo(Self::TOPIC_SAFE_STOP_STATE).await
    }
    async fn runtime_state(&self) -> InovoRosResult<Subscriber<RuntimeState>> {
        self.subscribe_inovo(Self::TOPIC_RUNTIME_STATE).await
    }
    async fn arm_state(&self) -> InovoRosResult<Subscriber<ArmState>> {
        self.subscribe_inovo(Self::TOPIC_ARM_STATE).await
    }
    async fn jog(&self) -> InovoRosResult<Subscriber<CartesianJogDemand>> {
        self.subscribe_inovo(Self::TOPIC_CARTESIAN_JOG).await
    }

    // Service

    // psu
    async fn safe_stop_reset(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/psu/safe_stop/reset", Trigger {})
            .await
    }
    async fn estop_reset(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/psu/estop/reset", Trigger {})
            .await
    }
    async fn power_on(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/psu/enable", Trigger {}).await
    }
    async fn power_off(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/psu/disable", Trigger {}).await
    }

    // robot
    async fn arm_enable(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/robot/enable", Trigger {})
            .await
    }
    async fn arm_disable(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/robot/disable", Trigger {})
            .await
    }

    // sequence
    async fn sequence_start(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/sequence/start", Trigger {})
            .await
    }
    async fn sequence_stop(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/sequence/stop", Trigger {})
            .await
    }
    async fn sequence_pause(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/sequence/pause", Trigger {})
            .await
    }
    async fn sequence_step(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/sequence/step", Trigger {})
            .await
    }
    async fn sequence_debug(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/sequence/debug", Trigger {})
            .await
    }
    async fn sequence_continue(&self) -> InovoRosResult<Response> {
        self.call_inovo::<Trigger>("/sequence/continue", Trigger {})
            .await
    }

    async fn sequence_function(
        &self,
        procedure_name: impl Into<String> + Send,
    ) -> InovoRosResult<Response> {
        self.call_inovo::<RunSequence>(
            "/sequence/start",
            RunSequence {
                procedure_name: procedure_name.into(),
                ..Default::default()
            },
        )
        .await
    }

    // Publish
    async fn jog_pub(&self) -> Publisher<CartesianJogDemand>;
}

#[async_trait::async_trait]
impl InovoRosBridge for ClientHandle {
    async fn subscribe_inovo<T: RosMessageType>(
        &self,
        topic: impl AsRef<str> + Send,
    ) -> InovoRosResult<Subscriber<T>> {
        Ok(self.subscribe(topic.as_ref()).await?)
    }
    async fn call_inovo<S: RosServiceType>(
        &self,
        service: impl AsRef<str> + Send,
        args: S::Request,
    ) -> InovoRosResult<S::Response> {
        self.call_service::<S>(service.as_ref(), args).await
    }
    async fn jog_pub(&self) -> Publisher<CartesianJogDemand> {
        self.advertise(Self::TOPIC_CARTESIAN_JOG).await.unwrap()
    }
}
