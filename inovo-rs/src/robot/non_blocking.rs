use std::net::SocketAddr;

use crate::geometry::{JointCoord, Pose};
use crate::iva::*;
use crate::ros_bridge::start_seq;
use crate::socket;
use crate::socket::non_blocking::InovoListener;
use crate::util::InovorsError;

use super::{CommandSequence, FromRobot, MotionParam};

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tracing::{debug, info, trace};

pub struct Robot {
    buf_writer: tokio::io::BufWriter<tokio::net::tcp::OwnedWriteHalf>,
    buf_reader: tokio::io::BufReader<tokio::net::tcp::OwnedReadHalf>,
    buffer: String,
}

impl Robot {
    pub fn new(tcp_stream: tokio::net::TcpStream) -> Self {
        let (read_half, write_half) = tcp_stream.into_split();
        let buf_writer = tokio::io::BufWriter::new(write_half);
        let buf_reader = tokio::io::BufReader::new(read_half);
        let buffer = String::new();

        Self {
            buf_writer,
            buf_reader,
            buffer,
        }
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.buf_reader.get_ref().local_addr()
    }
    pub fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        self.buf_reader.get_ref().peer_addr()
    }

    pub async fn accept_from(listener: &tokio::net::TcpListener) -> std::io::Result<Self> {
        let (conn, ip) = listener.accept().await?;
        info!("Accept connection from : {ip}");
        Ok(Self::new(conn))
    }

    pub async fn write(&mut self, msg: impl Into<String>) -> std::io::Result<()> {
        let msg = format!("{}\n", msg.into());
        debug!(">>> {}", msg.trim());
        self.buf_writer.write(msg.as_bytes()).await?;
        self.buf_writer.flush().await?;
        Ok(())
    }
    pub async fn read(&mut self) -> std::io::Result<String> {
        self.buffer.clear();
        debug!("reading line");
        loop {
            let b = self.buf_reader.read_u8().await?;
            trace!("< {:02x} : {} : {}", b, b as char, self.buffer);
            if b == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "detected, disconnected",
                ));
            }
            self.buffer.push(b as char);
            if self.buffer.ends_with("\r\n") {
                break;
            }
        }
        // let size = self.buf_reader.read_line(&mut self.buffer).await?;
        // if self.buffer == 0 {
        //     return Err(std::io::Error::new(
        //         std::io::ErrorKind::UnexpectedEof,
        //         "0 input bytes, disconnected",
        //     ));
        // }
        let msg = self.buffer.clone().trim().to_string();
        debug!("<<< {}", msg);
        Ok(msg)
    }

    pub async fn new_inovo(port: u16, host: impl Into<String>) -> Result<Self, InovorsError> {
        let listener = socket::non_blocking::new_local_listener(port).await?;

        start_seq(host, "iva").await?;

        let bot = listener.accept_robot().await?;

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(bot)
    }
}

#[async_trait::async_trait]
impl IvaRobot for Robot {
    async fn instruction(&mut self, inst: &Instruction) -> Result<String, InovorsError> {
        let req = inst.to_iva_request()?.to_string();
        self.write(req).await?;
        let res = self.read().await?;
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

#[async_trait::async_trait]
pub trait IvaRobot {
    async fn instruction(&mut self, inst: &Instruction) -> Result<String, InovorsError>;

    async fn instruction_assert_ok(
        &mut self,
        inst: &Instruction,
    ) -> Result<&mut Self, InovorsError> {
        let res = self.instruction(inst).await?;
        if res.contains("OK") {
            Ok(self)
        } else {
            Err(InovorsError::IvaError {
                req: inst.clone(),
                res,
                reason: "response assert `OK` failed".to_owned(),
            })
        }
    }

    /// send an instruction to the robot and try to parse the response into `T`
    async fn instruction_return<T: FromRobot>(
        &mut self,
        inst: &Instruction,
    ) -> Result<T, InovorsError> {
        let res = self.instruction(inst).await?;
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
    async fn execute(&mut self, robot_command: &RobotCommand) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::exec(*robot_command))
            .await
    }

    /// instruct the robot to sleep
    async fn sleep(&mut self, second: f64) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::Sleep { second }).await
    }

    /// instruct the robot to set the motion param
    async fn set_param(&mut self, motion_param: &MotionParam) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::SetParameter(*motion_param))
            .await
    }

    /// instruct the robot to execute a motion
    async fn motion(
        &mut self,
        mode: &MotionMode,
        target: &Pose,
    ) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::Motion {
            motion_mode: *mode,
            target: (*target).into(),
        })
        .await
    }

    /// instruct the robot to perform a linear move
    async fn linear(&mut self, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.motion(&MotionMode::Linear, target).await
    }
    /// instruct the robot to perform a linear relative move
    async fn linear_relative(&mut self, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.motion(&MotionMode::LinearRelative, target).await
    }
    /// instruct the robot to perform a joint move, can take both [`Transform`] and [`JointCoord`] as target
    async fn joint(
        &mut self,
        target: impl Into<MotionTarget> + Send,
    ) -> Result<&mut Self, InovorsError> {
        self.execute(&RobotCommand::Motion {
            motion_mode: MotionMode::Joint,
            target: target.into(),
        })
        .await
    }
    /// instruct the robot to perform a joint relative move
    async fn joint_relative(&mut self, target: &Pose) -> Result<&mut Self, InovorsError> {
        self.motion(&MotionMode::JointRelative, target).await
    }

    /// instruct the robot to enqueue a [`RobotCommand`]
    async fn enqueue(&mut self, robot_command: &RobotCommand) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::enqueue(*robot_command))
            .await
    }
    /// instruct the robot to dequeue all [`RobotCommand`]
    async fn dequeue(&mut self) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::dequeue()).await
    }
    /// instruct the robot to execute a [`CommandSequence`]
    async fn sequence(
        &mut self,
        command_sequence: &CommandSequence,
    ) -> Result<&mut Self, InovorsError> {
        for robot_command in command_sequence.iter() {
            self.enqueue(robot_command).await?;
        }
        self.dequeue().await
    }

    /// get the current [`Pose`] of the robot
    async fn get_current_pose(&mut self) -> Result<Pose, InovorsError> {
        self.get(GetTarget::Transform).await
    }
    /// get the current [`JointCoord`] of the robot
    async fn get_current_joint(&mut self) -> Result<JointCoord, InovorsError> {
        self.get(GetTarget::JointCoord).await
    }
    /// get data from data dict in robot runtime
    async fn get_data<T: FromRobot>(&mut self, key: String) -> Result<T, InovorsError> {
        self.get(GetTarget::Data { key }).await
    }
    /// get data from robot
    async fn get<T: FromRobot>(&mut self, get_target: GetTarget) -> Result<T, InovorsError> {
        self.instruction_return(&Instruction::Get(get_target)).await
    }

    /// instruct the robot to set digital io
    async fn io_set(
        &mut self,
        io_target: IOTarget,
        port: u16,
        state: bool,
    ) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::io_set(io_target, port, state))
            .await
    }
    /// get the digital io state of the robot
    async fn io_get(&mut self, io_target: IOTarget, port: u16) -> Result<bool, InovorsError> {
        self.instruction_return(&Instruction::io_get(io_target, port))
            .await
    }
    /// set the beckhoff io
    async fn beckhoff_set(&mut self, port: u16, state: bool) -> Result<&mut Self, InovorsError> {
        self.io_set(IOTarget::Beckhoff, port, state).await
    }
    /// set the wrist io
    async fn wrist_set(&mut self, port: u16, state: bool) -> Result<&mut Self, InovorsError> {
        self.io_set(IOTarget::Wrist, port, state).await
    }
    /// get the beckhoff io
    async fn beckhoff_get(&mut self, port: u16) -> Result<bool, InovorsError> {
        self.io_get(IOTarget::Beckhoff, port).await
    }
    /// get the wrist io
    async fn wrist_get(&mut self, port: u16) -> Result<bool, InovorsError> {
        self.io_get(IOTarget::Wrist, port).await
    }

    /// activate the robot gripper
    async fn gripper_activate(&mut self) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::Gripper(GripperCommand::Activate))
            .await
    }
    /// set the robot gripper to a predefined label
    async fn gripper_set(&mut self, label: String) -> Result<&mut Self, InovorsError> {
        self.instruction_assert_ok(&Instruction::gripper(GripperCommand::Set { label }))
            .await
    }
    /// get the robot gripper width
    async fn gripper_get(&mut self) -> Result<f64, InovorsError> {
        self.instruction_return(&Instruction::gripper(GripperCommand::Get))
            .await
    }

    /// instruct the robot to perform a custom command and get the return resposne
    async fn custom(&mut self, custom_command: CustomCommand) -> Result<String, InovorsError> {
        self.instruction(&Instruction::custom(custom_command)).await
    }
}
