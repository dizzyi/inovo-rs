use std::fmt::Debug;
use std::future::Future;

use roslibrust::{
    rosbridge::{ClientHandle, Publisher, Subscriber},
    RosMessageType, RosServiceType, Service,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::*;

use inovo_rs_macro::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InovoMessage<T: Clone + Debug> {
    #[serde(default)]
    pub header: InovoHeader,
    #[serde(default)]
    pub tcp_id: String,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InovoHeader {
    pub frame_id: String,
    pub seq: u64,
    pub stamp: Stamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Stamp {
    pub nsecs: u64,
    pub secs: u64,
}

// pub trait InovoMessageType:
//     Debug + Clone + Serialize + DeserializeOwned + PartialEq + Send + Sync + 'static
// {
//     const INOVO_TYPE_NAME: &'static str;
// }

impl<T> RosMessageType for InovoMessage<T>
where
    T: Debug + Clone + Serialize + DeserializeOwned + RosMessageType + Sync + Send + 'static,
{
    const ROS_TYPE_NAME: &'static str = T::ROS_TYPE_NAME;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Twist {
    pub linear: Vec3,
    pub angular: Vec3,
}

#[inovo_msg("commander_msgs/CartesianJogDemand")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CartesianJogDemand {
    pub twist: Twist,
}

#[inovo_msg("commander_msgs/SpeedStamped")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeedStamped {
    pub speed: Speed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Speed {
    pub linear: f64,
    pub angular: f64,
}

#[inovo_msg("geometry_msgs/PoseStamped")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PoseStamped {
    pose: Pose,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Pose {
    position: Vec3,
    orientation: Vec4,
}

#[inovo_msg("sensor_msgs/JointState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JointState {
    pub effort: Vec<f64>,
    pub name: Vec<String>,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
}

#[inovo_msg("psu_msgs/Status")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PowerState {
    current: f64,
    fault_code: u8,
    state: String,
    voltage: f64,
}

#[inovo_msg("arm_msgs/RobotState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RobotState {
    can_enable: bool,
    driver_state: String,
    drives_powered: bool,
    speed_limited: bool,
}

#[inovo_msg("psu_msgs/SafetyCircuitState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SafetyCircuotState {
    active: bool,
    circuit_complete: bool,
}

#[inovo_msg("commander_msgs/RuntimeState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RuntimeState {
    active_blocks: Vec<String>,
    current_block_progress: f64,
    state: u8,
    variables: Vec<Variable>,
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum RuntimeStatus {
    #[default]
    Idle = 0,
    Running = 1,
    Paused = 2,
    PausedOnError = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Variable {
    name: String,
    #[serde(rename = "type")]
    dtype: String,
    value: String,
}

#[inovo_msg("arm_msgs/ArmState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ArmState {
    enabled: bool,
    state: u8,
    joint_states: Vec<ArmJointState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ArmJointState {
    age: u64,
    current: f64,
    drive_temp: f64,
    ff_torque: f64,

    joint_temp: f64,
    motor_temp: f64,
    output_gain: f64,
    position: f64,

    state: u8,
    status: u64,

    target_position: f64,
    torque: f64,
    velocity: f64,
}

#[inovo_msg("std_srvs/Trigger")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Trigger {}

impl RosServiceType for Trigger {
    type Request = Trigger;
    type Response = Response;
    const ROS_SERVICE_NAME: &'static str = "";
    const MD5SUM: &'static str = "";
}

#[inovo_msg("commander_msgs/RunSequence")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunSequence {
    procedure_name: String,
}

impl RosServiceType for RunSequence {
    type Request = RunSequence;
    type Response = Response;
    const ROS_SERVICE_NAME: &'static str = "";
    const MD5SUM: &'static str = "";
}

#[inovo_msg("")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Response {
    message: String,
    success: bool,
}

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
    ) -> Subscriber<InovoMessage<T>>;

    async fn call_inovo<S: RosServiceType>(
        &self,
        service: impl AsRef<str> + Send,
        args: S::Request,
    ) -> S::Response;

    async fn tcp_speed(&self) -> Subscriber<InovoMessage<SpeedStamped>> {
        self.subscribe_inovo(Self::TOPIC_TCP_SPEED).await
    }
    async fn tcp_pose(&self) -> Subscriber<InovoMessage<PoseStamped>> {
        self.subscribe_inovo(Self::TOPIC_TCP_POSE).await
    }
    async fn joint_state(&self) -> Subscriber<InovoMessage<JointState>> {
        self.subscribe_inovo(Self::TOPIC_JOINT_STATE).await
    }
    async fn power_state(&self) -> Subscriber<InovoMessage<PowerState>> {
        self.subscribe_inovo(Self::TOPIC_POWER_STATE).await
    }
    async fn robot_state(&self) -> Subscriber<InovoMessage<RobotState>> {
        self.subscribe_inovo(Self::TOPIC_ROBOT_STATE).await
    }
    async fn estop_state(&self) -> Subscriber<InovoMessage<SafetyCircuotState>> {
        self.subscribe_inovo(Self::TOPIC_ESTOP_STATE).await
    }
    async fn safe_stop_state(&self) -> Subscriber<InovoMessage<SafetyCircuotState>> {
        self.subscribe_inovo(Self::TOPIC_SAFE_STOP_STATE).await
    }
    async fn runtime_state(&self) -> Subscriber<InovoMessage<RuntimeState>> {
        self.subscribe_inovo(Self::TOPIC_RUNTIME_STATE).await
    }
    async fn arm_state(&self) -> Subscriber<InovoMessage<ArmState>> {
        self.subscribe_inovo(Self::TOPIC_ARM_STATE).await
    }
    async fn jog(&self) -> Subscriber<InovoMessage<CartesianJogDemand>> {
        self.subscribe_inovo(Self::TOPIC_CARTESIAN_JOG).await
    }

    // Service

    // psu
    async fn safe_stop_reset(&self) -> Response {
        self.call_inovo::<Trigger>("/psu/safe_stop/reset", Trigger {})
            .await
    }
    async fn estop_reset(&self) -> Response {
        self.call_inovo::<Trigger>("/psu/estop/reset", Trigger {})
            .await
    }
    async fn power_on(&self) -> Response {
        self.call_inovo::<Trigger>("/psu/enable", Trigger {}).await
    }
    async fn power_off(&self) -> Response {
        self.call_inovo::<Trigger>("/psu/disable", Trigger {}).await
    }

    // robot
    async fn arm_enable(&self) -> Response {
        self.call_inovo::<Trigger>("/robot/enable", Trigger {})
            .await
    }
    async fn arm_disable(&self) -> Response {
        self.call_inovo::<Trigger>("/robot/disable", Trigger {})
            .await
    }

    // sequence
    async fn sequence_start(&self) -> Response {
        self.call_inovo::<Trigger>("/sequence/start", Trigger {})
            .await
    }
    async fn sequence_stop(&self) -> Response {
        self.call_inovo::<Trigger>("/sequence/stop", Trigger {})
            .await
    }
    async fn sequence_pause(&self) -> Response {
        self.call_inovo::<Trigger>("/sequence/pause", Trigger {})
            .await
    }
    async fn sequence_step(&self) -> Response {
        self.call_inovo::<Trigger>("/sequence/step", Trigger {})
            .await
    }
    async fn sequence_debug(&self) -> Response {
        self.call_inovo::<Trigger>("/sequence/debug", Trigger {})
            .await
    }
    async fn sequence_continue(&self) -> Response {
        self.call_inovo::<Trigger>("/sequence/continue", Trigger {})
            .await
    }

    async fn sequence_function(&self, procedure_name: impl Into<String> + Send) -> Response {
        self.call_inovo::<RunSequence>(
            "/sequence/start",
            RunSequence {
                procedure_name: procedure_name.into(),
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
    ) -> Subscriber<InovoMessage<T>> {
        println!("subscribing: {} - {}", topic.as_ref(), T::ROS_TYPE_NAME);
        self.subscribe(topic.as_ref()).await.unwrap()
    }
    async fn call_inovo<S: RosServiceType>(
        &self,
        service: impl AsRef<str> + Send,
        args: S::Request,
    ) -> S::Response {
        self.call_service::<S>(service.as_ref(), args)
            .await
            .unwrap()
    }
    async fn jog_pub(&self) -> Publisher<CartesianJogDemand> {
        self.advertise(Self::TOPIC_CARTESIAN_JOG).await.unwrap()
    }
}
