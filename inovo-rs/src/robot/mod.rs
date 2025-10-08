//! Module for interacting with inovo robot arm

use crate::context::Context;
use crate::iva::*;
use crate::ros_bridge::*;
use crate::socket;

mod command_sequence;
mod iva;
mod motion_param;

pub use command_sequence::*;
pub use iva::IvaRobot;
pub use motion_param::*;

/// A struct of a inovo robot arm
///
/// # Example
/// ```no_run
/// use inovo_rs::iva::CustomCommand;
/// use inovo_rs::robot::*;
/// use inovo_rs::geometry::*;
///
/// fn main() -> Result<(), RobotError>{
///     let mut bot = Robot::defaut_logger(50003, "psu002")?;
///
///     // robot motion
///     bot.linear(Transform::from_vector([100.0,100.0,100.0]))?;
///
///     // robot param
///     bot.set_param(MotionParam::new().set_speed(50.0))?;
///
///     // robot current transform
///     let transform : Transform = bot.get_current_transform()?;
///
///     // sequence command
///     let command_sequence = CommandSequence::new()
///         .then_linear_relative(Transform::from_x(100.0))
///         .then_sleep(1.0);
///     bot.sequence(command_sequence)?;
///
///
///     // gripper command
///     bot.gripper_activate()?;
///     let _ : f64 = bot.gripper_get()?;
///     bot.gripper_set("open")?;
///
///     // get/set digital IO
///     let _ = bot.beckhoff_get(0)?;
///     bot.beckhoff_set(0, true)?;
///
///     // custom command
///     let custom_command = CustomCommand::new()
///         .add_string("foo", "bar")
///         .add_float("meaning of the universe", 42.0);
///     let _ : String = bot.custom(custom_command)?;
///
///     Ok(())
/// }
/// ```
pub struct Robot {
    /// the tcp socket connection with the psu
    stream: socket::Stream,
}

impl Robot {
    /// construct a new [`Robot`]
    pub fn new(stream: socket::Stream) -> Self {
        Self { stream }
    }

    /// create a new instance, and call ros bridge run sequence to remotly start
    pub fn new_inovo(port: u16, host: impl Into<String>) -> Result<Self, RobotError> {
        let host = host.into();

        let mut listener = socket::Listener::new(port)?;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                roslibrust::rosbridge::ClientHandle::new(host.clone())
                    .await
                    .unwrap()
                    .sequence_function("iva")
                    .await
            })
            .unwrap();

        let stream = listener.accept()?;

        Ok(Self::new(stream))
    }
    /// create and run sequence with of inovo arm with default logger
    pub fn defaut_logger(port: u16, host: impl Into<String>) -> Result<Self, RobotError> {
        Self::new_inovo(port, host)
    }

    /// write a message to the socket
    pub fn write(&mut self, msg: impl Into<String>) -> Result<(), RobotError> {
        Ok(self.stream.write(msg)?)
    }
    /// read a message from the socket
    pub fn read(&mut self) -> Result<String, RobotError> {
        Ok(self.stream.read()?)
    }
}

impl IvaRobot for Robot {
    fn instruction(&mut self, inst: Instruction) -> Result<String, RobotError> {
        self.write(inst.to_json()?)?;
        self.read()
    }
}

unsafe impl Send for Robot {}

/// A trait for all data structure that can be deserialize from robot response
pub trait FromRobot: Sized {
    /// parse from robto response string
    fn from_robot(res: String) -> Result<Self, String>;
}

impl FromRobot for f64 {
    fn from_robot(res: String) -> Result<Self, String> {
        res.parse::<f64>().map_err(|e| format!("{}", e))
    }
}
impl FromRobot for i64 {
    fn from_robot(res: String) -> Result<Self, String> {
        res.parse::<i64>().map_err(|e| format!("{}", e))
    }
}
impl FromRobot for bool {
    fn from_robot(res: String) -> Result<Self, String> {
        match res.as_str() {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => Err(format!("unexpected response: {}", res)),
        }
    }
}
impl FromRobot for String {
    fn from_robot(res: String) -> Result<Self, String> {
        Ok(res)
    }
}

/// context representing iva context
///
/// pop a context in iva when exit
pub struct IvaContext;

impl Context<Robot> for IvaContext {
    fn context_enter(&mut self, _: &mut Robot) {}
    fn context_drop(&mut self, machine: &mut Robot) {
        let _ = machine.pop();
    }
}

/// Representing Robot Error
#[derive(Debug, thiserror::Error)]
pub enum RobotError {
    #[error(transparent)]
    SocketError(#[from] std::io::Error),
    // #[error(transparent)]
    // RosBridgeError(#[from] RosBridgeError),
    #[error(transparent)]
    JsonSer(#[from] serde_json::Error),
    #[error("Response Error")]
    ResponseError(String),
}
