use crate::context::Context;
use crate::iva::{self, MotionType, Pose, RobotCommand};
use crate::robot::IvaRobot;

mod joint;
mod transform;

pub use joint::JointCoord;
pub use transform::Transform;

/// A trait for `JointCoord` and `Transform`,
/// allowing them to have share trait that turn into pose
pub trait IntoPose {
    /// turn it into `iva::Pose`
    fn into_pose(self) -> iva::Pose;
}

/// A context for motion management
///
/// when exit reverse the motion
pub struct MotionContext {
    motion_type: MotionType,
    pose: Pose,
}
impl MotionContext {
    pub fn new(motion_type: MotionType, pose: impl IntoPose) -> Self {
        Self {
            motion_type,
            pose: pose.into_pose(),
        }
    }
}

impl<T> Context<T> for MotionContext
where
    T: IvaRobot,
{
    fn enter_fn(&mut self, t: &mut T) -> Result<(), String> {
        match &self.motion_type {
            MotionType::Joint | MotionType::Linear => {
                let pose = match self.pose {
                    Pose::Joint(_) => Pose::Joint(t.current_joint()?),
                    Pose::Transform(_) => Pose::Transform(t.current_frame()?),
                };
                t.execute(RobotCommand::Motion(
                    self.motion_type.clone(),
                    self.pose.clone(),
                ))?;
                self.pose = pose;
            }
            MotionType::JointRelative | MotionType::LinearRelative => {
                t.execute(RobotCommand::Motion(
                    self.motion_type.clone(),
                    self.pose.clone(),
                ))?;
                self.pose = match self.pose.clone() {
                    Pose::Joint(j) => Pose::Joint(-j),
                    Pose::Transform(t) => Pose::Transform(-t),
                }
            }
        }
        Ok(())
    }
    fn exit_fn(&mut self, t: &mut T) -> Result<(), String> {
        t.execute(RobotCommand::Motion(
            self.motion_type.clone(),
            self.pose.clone(),
        ))?;
        Ok(())
    }
    fn label(&self) -> String {
        format!("Motion Context {:?}, {:?}", self.motion_type, self.pose)
    }
}
