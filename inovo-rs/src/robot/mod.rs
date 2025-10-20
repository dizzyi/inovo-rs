//! Module for interacting with inovo robot arm

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};
use tracing::{debug, info};

use crate::context::{Context, ContextGuard};
use crate::iva::*;
use crate::ros_bridge::service::Service;
use crate::ros_bridge::*;
use crate::socket::{self, InovoListener};
use crate::util::InovorsError;
use crate::{geometry::*, ros_bridge};

mod command_sequence;
mod motion_param;

pub mod non_blocking;

pub use command_sequence::CommandSequence;
pub use motion_param::MotionParam;

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
    /// Writer to the tcp stream
    buf_writer: BufWriter<TcpStream>,
    /// Reader of the tcp stream
    buf_reader: BufReader<TcpStream>,
    /// Buffer for reading message
    buffer: String,
}

impl Robot {
    /// construct a new [`Robot`]
    ///
    /// ## Argument
    /// - `tcp_stream : TcpStream` : inner tcp stream connection
    pub fn new(tcp_stream: TcpStream) -> std::io::Result<Self> {
        let buf_writer = BufWriter::new(tcp_stream.try_clone()?);
        let buf_reader = BufReader::new(tcp_stream.try_clone()?);
        let buffer = String::new();

        Ok(Robot {
            buf_writer,
            buf_reader,
            buffer,
        })
    }
    /// get the local socket address of the stream
    pub fn local_addr(&self) -> Result<SocketAddr, io::Error> {
        self.buf_writer.get_ref().local_addr()
    }

    /// get the peer socket address of the stream
    pub fn peer_addr(&self) -> Result<SocketAddr, io::Error> {
        self.buf_writer.get_ref().peer_addr()
    }

    // Accept a new incoming connection from this listener.
    pub fn accept_from(listener: &std::net::TcpListener) -> std::io::Result<Self> {
        let (conn, ip) = listener.accept()?;
        info!("Accept connection from : {ip}");
        Self::new(conn)
    }

    /// write a message ends with `\r\n` to the socket stream
    pub fn write(&mut self, msg: impl Into<String>) -> Result<(), io::Error> {
        let msg: String = format!("{}\r\n", msg.into());
        debug!(">>> {}", msg.trim());
        self.buf_writer.write(msg.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }

    /// read a message ends with `\n` from the socket stream
    pub fn read(&mut self) -> Result<String, io::Error> {
        self.buffer.clear();
        let size = self.buf_reader.read_line(&mut self.buffer)?;
        if size == 0 {
            // return Err(std::io::Error::other("0 input bytes, diconnected"));
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "0 input bytes, disconnected",
            ));
        }
        let msg = self.buffer.clone().trim().to_string();
        debug!("<<< {}", msg);
        Ok(msg)
    }

    /// create a new instance, and call ros bridge run sequence to remotly start
    pub fn new_inovo(port: u16, host: impl Into<String>) -> Result<Self, InovorsError> {
        let listener = socket::new_local_listener(port)?;

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let client =
                    match rosbridge_connect(host, std::time::Duration::from_millis(5000)).await {
                        Ok(c) => c,
                        Err(e) => return Err(InovorsError::RosError(e)),
                    };

                let req = package::commander_msgs::RunSequence {
                    procedure_name: "iva".to_owned(),
                    ..Default::default()
                };
                let response = ros_bridge::service::sequence::Start::call(&client, req).await?;
                if response.success {
                    return Ok(());
                }

                let req = package::commander_msgs::RunSequenceRequest {
                    procedure_name: "iva".to_owned(),
                };
                let response =
                    ros_bridge::service::sequence::StartRequest::call(&client, req).await?;
                if response.success {
                    return Ok(());
                }

                Ok(())
            })?;

        let bot = listener.accept_robot()?;

        Ok(bot)
    }
}

impl IvaRobot for Robot {
    fn instruction(&mut self, inst: &Instruction) -> Result<String, InovorsError> {
        let req = inst.to_iva_request()?.to_string_pretty();
        self.write(req)?;
        let res = self.read()?;
        info!("{:?}", res);
        if res.contains("ERROR") {
            Err(InovorsError::IvaError {
                req: inst.clone(),
                res,
                reason: "response contains `ERROR`".to_owned(),
            })
        } else {
            Ok(res)
        }
    }
}

/// A trait of inovo robot, for iva protocal
pub trait IvaRobot
where
    IvaContext: Context<Self>,
{
    /// send an instruction to the robot and read the response
    fn instruction(&mut self, inst: &Instruction) -> Result<String, InovorsError>;

    /// send an instruction to the robot and assert the response to be `"OK"`, then return self
    fn instruction_assert_ok(&mut self, inst: &Instruction) -> Result<&mut Self, InovorsError> {
        let res = self.instruction(inst)?;
        match res.as_str() {
            "OK" => Ok(self),
            _ => Err(InovorsError::IvaError {
                req: inst.clone(),
                res,
                reason: "response assert `OK` failed".to_owned(),
            }),
        }
    }

    /// send an instruction to the robot and try to parse the response into `T`
    fn instruction_return<T: FromRobot>(&mut self, inst: &Instruction) -> Result<T, InovorsError> {
        let res = self.instruction(inst)?;
        match T::from_robot(&res) {
            Ok(t) => Ok(t),
            Err(s) => Err(InovorsError::IvaError {
                req: inst.clone(),
                res,
                reason: format!("Failed to parse from robot due to : {}", s),
            }),
        }
    }

    /// instruct the robot to execute a [`RobotCommand`]
    fn execute(&mut self, robot_command: &RobotCommand) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::exec(*robot_command))
    }

    /// instruct the robot to sleep
    fn sleep(&mut self, second: f64) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::Sleep { second })
    }

    /// instruct the robot to set the motion param
    fn set_param(&mut self, motion_param: &MotionParam) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::SetParameter(*motion_param))
    }

    /// instruct the robot to execute a motion
    fn motion(&mut self, mode: &MotionMode, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::Motion {
            motion_mode: *mode,
            target: (*target).into(),
        })
    }

    /// instruct the robot to perform a linear move
    fn linear(&mut self, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.motion(&MotionMode::Linear, target)
    }
    /// instruct the robot to perform a linear relative move
    fn linear_relative(&mut self, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.motion(&MotionMode::LinearRelative, target)
    }
    /// instruct the robot to perform a joint move, can take both [`Transform`] and [`JointCoord`] as target
    fn joint(&mut self, target: impl Into<MotionTarget>) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::Motion {
            motion_mode: MotionMode::Joint,
            target: target.into(),
        })
    }
    /// instruct the robot to perform a joint relative move
    fn joint_relative(&mut self, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.motion(&MotionMode::JointRelative, target)
    }

    /// instruct the robot to enter a context with a [`RobotCommand`]
    fn with_execute(
        &mut self,
        robot_command: &RobotCommand,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.instruction_assert_ok(&Instruction::exec_push(*robot_command))?;
        Ok(ContextGuard::new(self, IvaContext))
    }
    /// instruct the robot to enter a context with a sleep
    fn with_sleep(&mut self, second: f64) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_execute(&RobotCommand::Sleep { second })
    }
    /// instruct the robot to enter a context with motion param
    fn with_set_param(
        &mut self,
        motion_param: MotionParam,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_execute(&RobotCommand::SetParameter(motion_param))
    }
    /// instruct the robot to enter a context with a motion
    fn with_motion(
        &mut self,
        mode: MotionMode,
        target: &Pose,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_execute(&RobotCommand::Motion {
            motion_mode: mode,
            target: (*target).into(),
        })
    }
    /// instruct the robot to enter a context with a linear motion
    fn with_linear(
        &mut self,
        target: &Pose,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_motion(MotionMode::Linear, target)
    }
    /// instruct the robot to enter a context with a linear relative motion
    fn with_linear_relative(
        &mut self,
        target: &Pose,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_motion(MotionMode::LinearRelative, target)
    }
    /// instruct the robot to enter a context with a joint motion, can take both [`Transform`] and [`JointCoord`] as target
    fn with_joint(
        &mut self,
        target: impl Into<MotionTarget>,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_execute(&RobotCommand::Motion {
            motion_mode: MotionMode::Joint,
            target: target.into(),
        })
    }
    /// instruct the robot to enter a context with a joint relative motion
    fn with_joint_relative(
        &mut self,
        target: &Pose,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.with_motion(MotionMode::JointRelative, target)
    }

    /// instruct the robot to enqueue a [`RobotCommand`]
    fn enqueue(&mut self, robot_command: &RobotCommand) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::enqueue(*robot_command))
    }
    /// instruct the robot to dequeue all [`RobotCommand`]
    fn dequeue(&mut self) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::dequeue())
    }
    /// instruct the robot to enter a context with by dequeuing all [`RobotCommand`]
    fn with_dequeue(&mut self) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        self.instruction_assert_ok(&Instruction::dequeue_push())?;

        Ok(ContextGuard::new(self, IvaContext))
    }

    /// instruct the robot to execute a [`CommandSequence`]
    fn sequence(&mut self, command_sequence: &CommandSequence) -> Result<&mut Self, InovorsError> {
        for robot_command in command_sequence.iter() {
            self.enqueue(robot_command)?;
        }
        self.dequeue()
    }
    /// instruct the robot to enter a context by executing a [`CommandSequence`]
    fn with_sequence(
        &mut self,
        command_sequence: &CommandSequence,
    ) -> Result<ContextGuard<Self, IvaContext>, InovorsError> {
        for robot_command in command_sequence.iter() {
            self.enqueue(robot_command)?;
        }
        self.with_dequeue()
    }

    /// instruct the robot to pop a context
    fn pop(&mut self) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::Pop)
    }

    /// get the current [`Pose`] of the robot
    fn get_current_pose(&mut self) -> Result<Pose, InovorsError> {
        self.get(GetTarget::Transform)
    }
    /// get the current [`JointCoord`] of the robot
    fn get_current_joint(&mut self) -> Result<JointCoord, InovorsError> {
        self.get(GetTarget::JointCoord)
    }
    /// get data from data dict in robot runtime
    fn get_data<T: FromRobot>(&mut self, key: impl Into<String>) -> Result<T, InovorsError> {
        self.get(GetTarget::Data { key: key.into() })
    }
    /// get data from robot
    fn get<T: FromRobot>(&mut self, get_target: GetTarget) -> Result<T, InovorsError> {
        self.instruction_return(&Instruction::Get(get_target))
    }

    /// instruct the robot to set digital io
    fn io_set(
        &mut self,
        io_target: IOTarget,
        port: u16,
        state: bool,
    ) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::io_set(io_target, port, state))
    }
    /// get the digital io state of the robot
    fn io_get(&mut self, io_target: IOTarget, port: u16) -> Result<bool, InovorsError> {
        self.instruction_return(&Instruction::io_get(io_target, port))
    }
    /// set the beckhoff io
    fn beckhoff_set(&mut self, port: u16, state: bool) -> Result<&mut Self, InovorsError> {
        self.io_set(IOTarget::Beckhoff, port, state)
    }
    /// set the wrist io
    fn wrist_set(&mut self, port: u16, state: bool) -> Result<&mut Self, InovorsError> {
        self.io_set(IOTarget::Wrist, port, state)
    }
    /// get the beckhoff io
    fn beckhoff_get(&mut self, port: u16) -> Result<bool, InovorsError> {
        self.io_get(IOTarget::Beckhoff, port)
    }
    /// get the wrist io
    fn wrist_get(&mut self, port: u16) -> Result<bool, InovorsError> {
        self.io_get(IOTarget::Wrist, port)
    }

    /// activate the robot gripper
    fn gripper_activate(&mut self) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::Gripper(GripperCommand::Activate))
    }
    /// set the robot gripper to a predefined label
    fn gripper_set(&mut self, label: impl Into<String>) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::gripper(GripperCommand::Set {
            label: label.into(),
        }))
    }
    /// get the robot gripper width
    fn gripper_get(&mut self) -> Result<f64, InovorsError> {
        self.instruction_return(&Instruction::gripper(GripperCommand::Get))
    }

    /// instruct the robot to perform a custom command and get the return resposne
    fn custom(&mut self, custom_command: CustomCommand) -> Result<String, InovorsError> {
        self.instruction(&Instruction::custom(custom_command))
    }
    /// instruct the robot to perform a custom command and assert the response to be `"OK"`
    fn custom_and(&mut self, custom_command: CustomCommand) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::custom(custom_command))
    }
}

unsafe impl Send for Robot {}

/// A trait for all data structure that can be deserialize from robot response
pub trait FromRobot: Sized {
    /// parse from robto response string
    fn from_robot(res: &String) -> Result<Self, String>;
}

impl FromRobot for f64 {
    fn from_robot(res: &String) -> Result<Self, String> {
        res.parse::<f64>().map_err(|e| format!("{}", e))
    }
}
impl FromRobot for i64 {
    fn from_robot(res: &String) -> Result<Self, String> {
        res.parse::<i64>().map_err(|e| format!("{}", e))
    }
}
impl FromRobot for bool {
    fn from_robot(res: &String) -> Result<Self, String> {
        match res.as_str() {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => Err(format!("unexpected response: {}", res)),
        }
    }
}
impl FromRobot for String {
    fn from_robot(res: &String) -> Result<Self, String> {
        Ok(res.to_owned())
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
