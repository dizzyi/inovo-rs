use std::{fmt::Debug, vec};

use roslibrust::{
    rosbridge::{ClientHandle, Publisher, Subscriber},
    RosMessageType, RosServiceType,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Stamp {
    pub secs: u64,
    pub nsecs: u64,
}

impl Into<u128> for Stamp {
    fn into(self) -> u128 {
        self.nsecs as u128 + 1_000_000_000 * self.secs as u128
    }
}

impl<T> RosMessageType for InovoMessage<T>
where
    T: Debug + Clone + Serialize + DeserializeOwned + RosMessageType + Sync + Send + 'static,
{
    const ROS_TYPE_NAME: &'static str = T::ROS_TYPE_NAME;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Into<nalgebra::Vector3<f64>> for Vec3 {
    fn into(self) -> nalgebra::Vector3<f64> {
        nalgebra::Vector3::<f64>::new(self.x, self.y, self.z)
    }
}
impl From<nalgebra::Vector3<f64>> for Vec3 {
    fn from(value: nalgebra::Vector3<f64>) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl Into<nalgebra::Translation3<f64>> for Vec3 {
    fn into(self) -> nalgebra::Translation3<f64> {
        nalgebra::Translation {
            vector: self.into(),
        }
    }
}
impl From<nalgebra::Translation3<f64>> for Vec3 {
    fn from(value: nalgebra::Translation3<f64>) -> Self {
        value.vector.into()
    }
}

impl Vec3 {
    pub fn into_vector(self) -> nalgebra::Vector3<f64> {
        self.into()
    }
    pub fn from_vector(vector: nalgebra::Vector3<f64>) -> Vec3 {
        vector.into()
    }
    pub fn into_translation(self) -> nalgebra::Translation3<f64> {
        self.into()
    }
    pub fn from_translation(tran: nalgebra::Translation3<f64>) -> Vec3 {
        tran.into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Into<nalgebra::SVector<f64, 4>> for Vec4 {
    fn into(self) -> nalgebra::SVector<f64, 4> {
        nalgebra::SVector::<f64, 4>::new(self.x, self.y, self.z, self.w)
    }
}
impl From<nalgebra::SVector<f64, 4>> for Vec4 {
    fn from(value: nalgebra::SVector<f64, 4>) -> Self {
        Vec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl Into<nalgebra::Quaternion<f64>> for Vec4 {
    fn into(self) -> nalgebra::Quaternion<f64> {
        nalgebra::Quaternion::new(self.w, self.x, self.y, self.z)
    }
}
impl Into<nalgebra::UnitQuaternion<f64>> for Vec4 {
    fn into(self) -> nalgebra::UnitQuaternion<f64> {
        nalgebra::UnitQuaternion::from_quaternion(self.into())
    }
}
impl From<nalgebra::Quaternion<f64>> for Vec4 {
    fn from(value: nalgebra::Quaternion<f64>) -> Self {
        Vec4 {
            x: value.i,
            y: value.j,
            z: value.k,
            w: value.w,
        }
    }
}
impl From<nalgebra::UnitQuaternion<f64>> for Vec4 {
    fn from(value: nalgebra::UnitQuaternion<f64>) -> Self {
        Vec4 {
            x: value.i,
            y: value.j,
            z: value.k,
            w: value.w,
        }
    }
}

impl Vec4 {
    pub fn into_quaternion(self) -> nalgebra::Quaternion<f64> {
        self.into()
    }
    pub fn from_quaternion(quat: nalgebra::Quaternion<f64>) -> Vec4 {
        quat.into()
    }
    pub fn into_unit_quaternion(self) -> nalgebra::UnitQuaternion<f64> {
        self.into()
    }
    pub fn from_unit_quaternion(unit_quat: nalgebra::UnitQuaternion<f64>) -> Vec4 {
        unit_quat.into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Twist {
    pub linear: Vec3,
    pub angular: Vec3,
}

#[inovo_msg("commander_msgs/CartesianJogDemand")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct CartesianJogDemand {
    pub twist: Twist,
}

#[inovo_msg("commander_msgs/SpeedStamped")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeedStamped {
    pub speed: Speed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Speed {
    pub linear: f64,
    pub angular: f64,
}

#[inovo_msg("geometry_msgs/PoseStamped")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct PoseStamped {
    pub pose: Pose,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Pose {
    pub position: Vec3,
    pub orientation: Vec4,
}

impl Into<nalgebra::Isometry3<f64>> for Pose {
    fn into(self) -> nalgebra::Isometry3<f64> {
        nalgebra::Isometry {
            rotation: self.orientation.into(),
            translation: self.position.into(),
        }
    }
}
impl From<nalgebra::Isometry3<f64>> for Pose {
    fn from(value: nalgebra::Isometry3<f64>) -> Self {
        Pose {
            position: value.translation.into(),
            orientation: value.rotation.into(),
        }
    }
}

impl Pose {
    pub fn into_isometry(self) -> nalgebra::Isometry3<f64> {
        self.into()
    }
    pub fn from_isometry(iso: nalgebra::Isometry3<f64>) -> Pose {
        iso.into()
    }
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
    pub current: f64,
    pub fault_code: u8,
    pub state: String,
    pub voltage: f64,
}

#[inovo_msg("arm_msgs/RobotState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RobotState {
    pub can_enable: bool,
    pub driver_state: String,
    pub drives_powered: bool,
    pub speed_limited: bool,
}

#[inovo_msg("psu_msgs/SafetyCircuitState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SafetyCircuotState {
    pub active: bool,
    pub circuit_complete: bool,
}

#[inovo_msg("commander_msgs/RuntimeState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RuntimeState {
    pub active_blocks: Vec<String>,
    pub current_block_progress: f64,
    pub state: u8,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Serialize_repr)]
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
    pub name: String,
    #[serde(rename = "type")]
    pub dtype: String,
    pub value: String,
}

#[inovo_msg("arm_msgs/ArmState")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ArmState {
    pub enabled: bool,
    pub state: u8,
    pub joint_states: Vec<ArmJointState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ArmJointState {
    pub age: u64,
    pub current: f64,
    pub drive_temp: f64,
    pub ff_torque: f64,

    pub joint_temp: f64,
    pub motor_temp: f64,
    pub output_gain: f64,
    pub position: f64,

    pub state: u8,
    pub status: u64,

    pub target_position: f64,
    pub torque: f64,
    pub velocity: f64,
}

#[inovo_msg("std_srvs/Trigger")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Trigger {}

#[inovo_msg("commander_msgs/RunSequence")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunSequence {
    pub procedure_name: String,
}

impl RosServiceType for Trigger {
    type Request = Trigger;
    type Response = InovoResponse;
    const ROS_SERVICE_NAME: &'static str = "";
    const MD5SUM: &'static str = "";
}

impl RosServiceType for RunSequence {
    type Request = RunSequence;
    type Response = InovoResponse;
    const ROS_SERVICE_NAME: &'static str = "";
    const MD5SUM: &'static str = "";
}

#[inovo_msg("")]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InovoResponse {
    pub message: String,
    pub success: bool,
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

    async fn call_inovo<S: RosServiceType<Response = InovoResponse>>(
        &self,
        service: impl AsRef<str> + Send,
        args: S::Request,
    ) -> Result<(), String>;

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
    async fn safe_stop_reset(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/psu/safe_stop/reset", Trigger {})
            .await
    }
    async fn estop_reset(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/psu/estop/reset", Trigger {})
            .await
    }
    async fn power_on(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/psu/enable", Trigger {}).await
    }
    async fn power_off(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/psu/disable", Trigger {}).await
    }

    // robot
    async fn arm_enable(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/robot/enable", Trigger {})
            .await
    }
    async fn arm_disable(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/robot/disable", Trigger {})
            .await
    }

    // sequence
    async fn sequence_start(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/sequence/start", Trigger {})
            .await
    }
    async fn sequence_stop(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/sequence/stop", Trigger {})
            .await
    }
    async fn sequence_pause(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/sequence/pause", Trigger {})
            .await
    }
    async fn sequence_step(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/sequence/step", Trigger {})
            .await
    }
    async fn sequence_debug(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/sequence/debug", Trigger {})
            .await
    }
    async fn sequence_continue(&self) -> Result<(), String> {
        self.call_inovo::<Trigger>("/sequence/continue", Trigger {})
            .await
    }

    async fn sequence_function(
        &self,
        procedure_name: impl Into<String> + Send,
    ) -> Result<(), String> {
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
        self.subscribe(topic.as_ref()).await.unwrap()
    }
    async fn call_inovo<S: RosServiceType<Response = InovoResponse>>(
        &self,
        service: impl AsRef<str> + Send,
        args: S::Request,
    ) -> Result<(), String> {
        let res = self
            .call_service::<S>(service.as_ref(), args)
            .await
            .unwrap();

        if res.success {
            Ok(())
        } else {
            Err(res.message)
        }
    }
    async fn jog_pub(&self) -> Publisher<CartesianJogDemand> {
        self.advertise(Self::TOPIC_CARTESIAN_JOG).await.unwrap()
    }
}
