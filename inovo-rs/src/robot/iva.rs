use crate::context::{Context, ContextGuard};
use crate::geometry::{JointCoord, Transform};
use crate::iva::*;
use crate::robot::{CommandSequence, FromRobot, IvaContext, MotionParam, RobotError};

/// A trait of inovo robot, for iva protocal
pub trait IvaRobot
where
    IvaContext: Context<Self>,
{
    /// send an instruction to the robot and read the response
    fn instruction(&mut self, inst: Instruction) -> Result<String, RobotError>;

    /// send an instruction to the robot and assert the response to be `"OK"`, then return self
    fn instruction_assert_ok(&mut self, inst: Instruction) -> Result<&mut Self, RobotError> {
        let res = self.instruction(inst)?;
        match res.as_str() {
            "OK" => Ok(self),
            _ => Err(RobotError::ResponseError(res)),
        }
    }

    /// send an instruction to the robot and try to parse the response into `T`
    fn instruction_return<T: FromRobot>(&mut self, inst: Instruction) -> Result<T, RobotError> {
        let res = self.instruction(inst)?;
        match T::from_robot(res) {
            Ok(t) => Ok(t),
            Err(s) => Err(RobotError::ResponseError(s)),
        }
    }

    /// instruct the robot to execute a [`RobotCommand`]
    fn execute(&mut self, robot_command: RobotCommand) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::exec(robot_command))
    }

    /// instruct the robot to sleep
    fn sleep(&mut self, second: f64) -> Result<&mut Self, RobotError> {
        self.execute(RobotCommand::Sleep { second })
    }

    /// instruct the robot to set the motion param
    fn set_param(&mut self, motion_param: MotionParam) -> Result<&mut Self, RobotError> {
        self.execute(RobotCommand::SetParameter(motion_param))
    }

    /// instruct the robot to execute a motion
    fn motion(&mut self, mode: MotionMode, target: Transform) -> Result<&mut Self, RobotError> {
        self.execute(RobotCommand::Motion {
            motion_mode: mode,
            target: target.into(),
        })
    }

    /// instruct the robot to perform a linear move
    fn linear(&mut self, target: Transform) -> Result<&mut Self, RobotError> {
        self.motion(MotionMode::Linear, target)
    }
    /// instruct the robot to perform a linear relative move
    fn linear_relative(&mut self, target: Transform) -> Result<&mut Self, RobotError> {
        self.motion(MotionMode::LinearRelative, target)
    }
    /// instruct the robot to perform a joint move, can take both [`Transform`] and [`JointCoord`] as target
    fn joint(&mut self, target: impl Into<MotionTarget>) -> Result<&mut Self, RobotError> {
        self.execute(RobotCommand::Motion {
            motion_mode: MotionMode::Joint,
            target: target.into(),
        })
    }
    /// instruct the robot to perform a joint relative move
    fn joint_relative(&mut self, target: Transform) -> Result<&mut Self, RobotError> {
        self.motion(MotionMode::JointRelative, target)
    }

    /// instruct the robot to enter a context with a [`RobotCommand`]
    fn with_execute(
        &mut self,
        robot_command: RobotCommand,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.instruction_assert_ok(Instruction::exec_push(robot_command))?;
        Ok(ContextGuard::new(self, IvaContext))
    }
    /// instruct the robot to enter a context with a sleep
    fn with_sleep(&mut self, second: f64) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_execute(RobotCommand::Sleep { second })
    }
    /// instruct the robot to enter a context with motion param
    fn with_set_param(
        &mut self,
        motion_param: MotionParam,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_execute(RobotCommand::SetParameter(motion_param))
    }
    /// instruct the robot to enter a context with a motion
    fn with_motion(
        &mut self,
        mode: MotionMode,
        target: Transform,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_execute(RobotCommand::Motion {
            motion_mode: mode,
            target: target.into(),
        })
    }
    /// instruct the robot to enter a context with a linear motion
    fn with_linear(
        &mut self,
        target: Transform,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_motion(MotionMode::Linear, target)
    }
    /// instruct the robot to enter a context with a linear relative motion
    fn with_linear_relative(
        &mut self,
        target: Transform,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_motion(MotionMode::LinearRelative, target)
    }
    /// instruct the robot to enter a context with a joint motion, can take both [`Transform`] and [`JointCoord`] as target
    fn with_joint(
        &mut self,
        target: impl Into<MotionTarget>,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_execute(RobotCommand::Motion {
            motion_mode: MotionMode::Joint,
            target: target.into(),
        })
    }
    /// instruct the robot to enter a context with a joint relative motion
    fn with_joint_relative(
        &mut self,
        target: Transform,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.with_motion(MotionMode::JointRelative, target)
    }

    /// instruct the robot to enqueue a [`RobotCommand`]
    fn enqueue(&mut self, robot_command: RobotCommand) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::enqueue(robot_command))
    }
    /// instruct the robot to dequeue all [`RobotCommand`]
    fn dequeue(&mut self) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::dequeue())
    }
    /// instruct the robot to enter a context with by dequeuing all [`RobotCommand`]
    fn with_dequeue(&mut self) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        self.instruction_assert_ok(Instruction::dequeue_push())?;

        Ok(ContextGuard::new(self, IvaContext))
    }

    /// instruct the robot to execute a [`CommandSequence`]
    fn sequence(&mut self, command_sequence: CommandSequence) -> Result<&mut Self, RobotError> {
        for robot_command in command_sequence.into_iter() {
            self.enqueue(robot_command)?;
        }
        self.dequeue()
    }
    /// instruct the robot to enter a context by executing a [`CommandSequence`]
    fn with_sequence(
        &mut self,
        command_sequence: CommandSequence,
    ) -> Result<ContextGuard<Self, IvaContext>, RobotError> {
        for robot_command in command_sequence.into_iter() {
            self.enqueue(robot_command)?;
        }
        self.with_dequeue()
    }

    /// instruct the robot to pop a context
    fn pop(&mut self) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::Pop)
    }

    /// get the current [`Transform`] of the robot
    fn get_current_transform(&mut self) -> Result<Transform, RobotError> {
        self.get(GetTarget::Transform)
    }
    /// get the current [`JointCoord`] of the robot
    fn get_current_joint(&mut self) -> Result<JointCoord, RobotError> {
        self.get(GetTarget::JointCoord)
    }
    /// get data from data dict in robot runtime
    fn get_data<T: FromRobot>(&mut self, key: impl Into<String>) -> Result<T, RobotError> {
        self.get(GetTarget::Data { key: key.into() })
    }
    /// get data from robot
    fn get<T: FromRobot>(&mut self, get_target: GetTarget) -> Result<T, RobotError> {
        self.instruction_return(Instruction::Get(get_target))
    }

    /// instruct the robot to set digital io
    fn io_set(
        &mut self,
        io_target: IOTarget,
        port: u16,
        state: bool,
    ) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::io_set(io_target, port, state))
    }
    /// get the digital io state of the robot
    fn io_get(&mut self, io_target: IOTarget, port: u16) -> Result<bool, RobotError> {
        self.instruction_return(Instruction::io_get(io_target, port))
    }
    /// set the beckhoff io
    fn beckhoff_set(&mut self, port: u16, state: bool) -> Result<&mut Self, RobotError> {
        self.io_set(IOTarget::Beckhoff, port, state)
    }
    /// set the wrist io
    fn wrist_set(&mut self, port: u16, state: bool) -> Result<&mut Self, RobotError> {
        self.io_set(IOTarget::Wrist, port, state)
    }
    /// get the beckhoff io
    fn beckhoff_get(&mut self, port: u16) -> Result<bool, RobotError> {
        self.io_get(IOTarget::Beckhoff, port)
    }
    /// get the wrist io
    fn wrist_get(&mut self, port: u16) -> Result<bool, RobotError> {
        self.io_get(IOTarget::Wrist, port)
    }

    /// activate the robot gripper
    fn gripper_activate(&mut self) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::Gripper(GripperCommand::Activate))
    }
    /// set the robot gripper to a predefined label
    fn gripper_set(&mut self, label: impl Into<String>) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::gripper(GripperCommand::Set {
            label: label.into(),
        }))
    }
    /// get the robot gripper width
    fn gripper_get(&mut self) -> Result<f64, RobotError> {
        self.instruction_return(Instruction::gripper(GripperCommand::Get))
    }

    /// instruct the robot to perform a custom command and get the return resposne
    fn custom(&mut self, custom_command: CustomCommand) -> Result<String, RobotError> {
        self.instruction(Instruction::custom(custom_command))
    }
    /// instruct the robot to perform a custom command and assert the response to be `"OK"`
    fn custom_and(&mut self, custom_command: CustomCommand) -> Result<&mut Self, RobotError> {
        self.instruction_assert_ok(Instruction::custom(custom_command))
    }
}
