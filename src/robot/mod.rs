use std::f64::consts::PI;

use crate::context::*;
use crate::geometry::*;
use crate::iva::*;
use crate::logger::*;
use crate::ros_bridge;
use crate::socket;

mod command_sequence;
mod motion_param;

pub use command_sequence::*;
pub use motion_param::*;

/// A struct of a inovo robot arm
///
/// # Example
/// ```no_run
/// use inovo_rs::iva::*;
/// use inovo_rs::robot::*;
/// use inovo_rs::geometry::*;
///
/// fn main()->Result<(),String>{
///     let mut robot = Robot::defaut_logger(50003, "psu002")?;
///      
///     // perform robot robot command
///     let transform = Transform::from_x(100.0);
///     robot.linear_relative(transform.clone())?;
///
///     // set robot motion parameter
///     let blend = MotionParam::default().set_blend_linear(1.0);
///     let no_blend = MotionParam::default().set_blend_linear(0.0);
///     robot.set_param(&blend)?;
///
///     // get robot current frame
///     let _ = robot.current_frame()?;
///
///     // perform robot sequence
///     let sequence = CommandSequence::new()
///         .then_linear(transform.clone())
///         .then_sleep(10.0);
///     robot.sequence(&sequence)?;
///
///     // get/set digital IO
///     let _ = robot.digital_beckhoff_get(0)?;
///     robot.digital_beckhoff_set(1,IOState::High)?;
///
///     // gripper
///     robot.gripper_activate()?;
///     robot.gripper_set("OPEN")?;
///     let _ = robot.gripper_get()?;
///
///     // custom function
///     robot.custom_keyword("CUSTOM_FUNCTION")?;
///
///     // stacked context
///     robot
///         .with_pose(MotionType::LinearRelative, transform.clone())?
///         .with_param(&no_blend)?
///         .sleep(5.0)?
///         .joint_relative(transform.clone())?;
///
///     // multiple context
///     {
///         let mut guard1 = robot.with_pose(MotionType::LinearRelative, transform.clone())?;
///         //  ^^^^^^
///         //  guard1 can be use as a reference to robot
///         guard1.linear_relative(transform.clone())?;
///
///         let mut guard2 = guard1.with_param(&no_blend)?;
///
///         guard2.sleep(5.0)?;
///         
///         drop(guard2); // <-- when guard2 is dropped, guard1 is released
///                       //     it will reverse the effect of the no_blend context
///
///         guard1.joint_relative(transform.clone())?;
///
///     } // <-- when guard1 is drop, robot is released
///       //     it will reverse the motion of context 1
///     Ok(())
/// }
/// ```
pub struct Robot {
    /// the tcp socket connection with the psu
    stream: socket::Stream,
    /// the logger for the robot arm
    logger: Logger,
    /// a stack of motion param for context management
    param_stack: Vec<MotionParam>,
    /// a stack of context for context machine
    context_stack: Vec<Box<dyn Context<Self>>>,
}

impl Logable for Robot {
    fn get_logger(&mut self) -> &mut Logger {
        &mut self.logger
    }
}
impl InovoRobot for Robot {
    fn construct(stream: socket::Stream, logger: Logger) -> Self {
        Self {
            stream,
            logger,
            param_stack: vec![],
            context_stack: vec![],
        }
    }
    fn stream(&mut self) -> &mut socket::Stream {
        &mut self.stream
    }
    fn param_stack(&mut self) -> &mut Vec<MotionParam> {
        &mut self.param_stack
    }
    fn init(mut self) -> Result<Self, String> {
        self.set_param(&MotionParam::default())?;
        Ok(self)
    }
}
impl IvaRobot for Robot {}

impl ContextMachine for Robot {
    fn context_stack(&mut self) -> &mut Vec<Box<dyn Context<Self>>> {
        &mut self.context_stack
    }
}

/// A trait of inovo robot, for a general connection to psu
pub trait InovoRobot: Sized + Logable + ContextMachine {
    /// construct a new instance
    fn construct(stream: socket::Stream, logger: Logger) -> Self;
    /// get the tcp socket stream to the psu
    fn stream(&mut self) -> &mut socket::Stream;
    /// get the motion parameter stack
    fn param_stack(&mut self) -> &mut Vec<MotionParam>;
    /// initalization of the robot
    fn init(self) -> Result<Self, String>;

    /// create a new instance, and call ros bridge run sequence to remotly start
    fn new_inovo(
        port: u16,
        host: impl Into<String>,
        logger: Logger,
        socket_logger: Option<Logger>,
    ) -> Result<Self, String> {
        let host = host.into();
        let mut listener = socket::Listener::new(port)?;

        ros_bridge::run_sequence(&host, "iva")?;

        let stream = listener.accept(Some(host.into()), socket_logger)?;

        Self::construct(stream, logger).init()
    }
    /// create and run sequence with of inovo arm with default logger
    fn defaut_logger(port: u16, host: impl Into<String>) -> Result<Self, String> {
        let host = host.into();
        let logger = Logger::default_target(&host)?;
        Self::new_inovo(port, host, logger, None)
    }
    /// write a message to the socket
    fn write(&mut self, msg: impl Into<String>) -> Result<(), String> {
        self.stream().write(msg)
    }
    /// read a message from the socket
    fn read(&mut self) -> Result<String, String> {
        self.stream().read()
    }
}

/// A trait of inovo robot, for iva protocal
pub trait IvaRobot: InovoRobot {
    /// send a instruction
    fn instruction(&mut self, instruction: Instruction) -> Result<&mut Self, String> {
        self.write(instruction)?;
        Ok(self)
    }
    /// read a message and make sure the response is `OK`
    fn res_is_ok(&mut self) -> Result<&mut Self, String> {
        let res = self.read()?;
        if res.eq("OK") {
            Ok(self)
        } else {
            let msg = format!("Robot response not OK : {:?}", res);
            self.error(&msg)?;
            Err(msg)
        }
    }
    /// send a execute instruction, and make sure response is `OK`
    fn execute(&mut self, robot_command: RobotCommand) -> Result<&mut Self, String> {
        self.instruction(Instruction::Execute(robot_command))?
            .res_is_ok()
    }
    /// perform a linear move to a specified pose
    fn linear(&mut self, pose: impl IntoPose) -> Result<&mut Self, String> {
        self.execute(RobotCommand::Motion(MotionType::Linear, pose.into_pose()))
    }
    /// perform a linear relative move to a specified pose
    fn linear_relative(&mut self, pose: impl IntoPose) -> Result<&mut Self, String> {
        self.execute(RobotCommand::Motion(
            MotionType::LinearRelative,
            pose.into_pose(),
        ))
    }
    /// perform a joint move to a specified pose
    fn joint(&mut self, pose: impl IntoPose) -> Result<&mut Self, String> {
        self.execute(RobotCommand::Motion(MotionType::Joint, pose.into_pose()))
    }
    /// perform a joint relative move to a specified pose
    fn joint_relative(&mut self, pose: impl IntoPose) -> Result<&mut Self, String> {
        self.execute(RobotCommand::Motion(
            MotionType::JointRelative,
            pose.into_pose(),
        ))
    }
    /// sleep for a specified duration
    fn sleep(&mut self, second: f64) -> Result<&mut Self, String> {
        self.execute(RobotCommand::Sleep(second))
    }
    /// set the robot motion parameter
    fn set_param(&mut self, param: &MotionParam) -> Result<&mut Self, String> {
        let _ = self.param_stack().pop();
        self.param_stack().push(param.to_owned());
        self.execute(RobotCommand::Param(param.clone()))
    }
    /// push a robot motion parameter to the stack
    fn push_param(&mut self, param: &MotionParam) -> Result<&mut Self, String> {
        if let None = self.param_stack().last() {
            self.param_stack().push(MotionParam::default())
        }
        self.param_stack().push(param.to_owned());
        self.set_param(param)?;
        Ok(self)
    }
    /// pop a robot motion parameter from the stack
    fn pop_param(&mut self) -> Result<&mut Self, String> {
        let _ = self.param_stack().pop();
        let param = self
            .param_stack()
            .last()
            .cloned()
            .unwrap_or(MotionParam::default());
        self.set_param(&param)
    }
    /// synchronize the robot
    fn sync(&mut self) -> Result<&mut Self, String> {
        self.execute(RobotCommand::Sync)
    }

    /// enter a new context with a motion param
    ///
    /// when exit, pop the motion param
    fn with_param(&mut self, param: &MotionParam) -> Result<ContextGuard<Self>, String> {
        self.with(Box::new(ParamContext::new(param.clone())))
    }

    /// enter a new context with a motion
    ///
    /// when exit, reverse the motion
    fn with_pose(
        &mut self,
        motion_type: MotionType,
        pose: impl IntoPose,
    ) -> Result<ContextGuard<Self>, String> {
        self.with(Box::new(MotionContext::new(motion_type, pose)))
    }

    /// enqueue a robotcommand to the robot
    fn enqueue(&mut self, robot_command: RobotCommand) -> Result<&mut Self, String> {
        self.instruction(Instruction::Enqueue(robot_command))?
            .res_is_ok()
    }
    /// dequeue and perform all enqueueed robot command
    fn dequeue(&mut self) -> Result<&mut Self, String> {
        self.instruction(Instruction::Dequeue)?.res_is_ok()
    }
    /// perform a sequence
    fn sequence(&mut self, sequence: &CommandSequence) -> Result<&mut Self, String> {
        for robot_command in sequence.iter().cloned() {
            self.enqueue(robot_command)?;
        }
        self.dequeue()
    }
    /// activate the gripper
    fn gripper_activate(&mut self) -> Result<&mut Self, String> {
        self.instruction(Instruction::Gripper(GripperCommand::Activate))?
            .res_is_ok()
    }
    /// set the gripper position to a specified label
    fn gripper_set(&mut self, label: impl Into<String>) -> Result<&mut Self, String> {
        self.instruction(Instruction::Gripper(GripperCommand::Set(label.into())))?
            .res_is_ok()
    }
    /// get the gripper current position
    fn gripper_get(&mut self) -> Result<f64, String> {
        self.instruction(Instruction::Gripper(GripperCommand::Get))?
            .read()?
            .parse::<f64>()
            .map_err(|e| e.to_string())
    }

    /// set the digital output of a specified channel
    fn digital_set(
        &mut self,
        io_source: IOSource,
        port: u8,
        io_state: IOState,
    ) -> Result<&mut Self, String> {
        self.instruction(Instruction::Digital(
            io_source,
            port,
            IOType::Output(io_state),
        ))?
        .res_is_ok()
    }
    /// get the digital input of a specified channel
    fn digital_get(&mut self, io_source: IOSource, port: u8) -> Result<bool, String> {
        self.instruction(Instruction::Digital(io_source, port, IOType::Input))?
            .read()
            .map(|t| t.eq("True"))
    }

    /// get the beckhoff digital input of specified port
    fn digital_beckhoff_get(&mut self, port: u8) -> Result<bool, String> {
        self.digital_get(IOSource::Beckhoff, port)
    }
    /// set the beckhoff digital output of specified port
    fn digital_beckhoff_set(&mut self, port: u8, io_state: IOState) -> Result<&mut Self, String> {
        self.digital_set(IOSource::Beckhoff, port, io_state)
    }

    /// get the wrist digital input of specified port
    fn digital_wrist_get(&mut self, port: u8) -> Result<bool, String> {
        self.digital_get(IOSource::Wrist, port)
    }
    /// set the wrist digital output of specified port
    fn digital_wrist_set(&mut self, port: u8, io_state: IOState) -> Result<&mut Self, String> {
        self.digital_set(IOSource::Wrist, port, io_state)
    }

    /// get the current frame of the robot
    fn current_frame(&mut self) -> Result<Transform, String> {
        Transform::from_robot(
            self.instruction(Instruction::Current(PoseType::Frame))?
                .read()?,
        )
    }
    /// get the current joint coord of the robot
    fn current_joint(&mut self) -> Result<JointCoord, String> {
        self.instruction(Instruction::Current(PoseType::Joint))?
            .read()?
            .replace(&['[', ']', ' '], "")
            .split(",")
            .collect::<Vec<_>>()
            .split_last()
            .ok_or(format!("error spliting last"))?
            .1
            .iter()
            .map(|s| s.parse::<f64>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<f64>, _>>()?
            .into_iter()
            .map(|f| f / PI * 180.0)
            .collect::<Vec<_>>()
            .try_into()
    }

    /// send a custom instruction
    fn custom(
        &mut self,
        tokens: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<&mut Self, String> {
        self.instruction(Instruction::Custom(
            tokens.into_iter().map(|a| a.into()).collect::<Vec<_>>(),
        ))
    }
    /// send a custom instruction and make sure the response are `OK`
    fn custom_function(
        &mut self,
        tokens: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<&mut Self, String> {
        self.custom(tokens)?.res_is_ok()
    }
    /// send a custom instruction with only one keyword, and make sure the response are `OK`
    fn custom_keyword(&mut self, func: impl Into<String>) -> Result<&mut Self, String> {
        self.custom(vec![func.into()])?.res_is_ok()
    }
}

unsafe impl Send for Robot {}
