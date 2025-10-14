use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ros_bridge::{
    std_msgs::{Header, Time},
    std_srvs::Response,
};

// pub struct FollowJointTrajectoryAction(serde_json::Value);

// TODO control_msgs/FollowJointTrajectoryActionFeedback
#[inovo_msg("control_msgs")]
pub struct FollowJointTrajectoryActionGoal(serde_json::Value);
// TODO control_msgs/FollowJointTrajectoryActionResult
// TODO control_msgs/FollowJointTrajectoryFeedback
// TODO control_msgs/FollowJointTrajectoryGoal
// TODO control_msgs/FollowJointTrajectoryResult
// TODO control_msgs/GripperCommand
// TODO control_msgs/GripperCommandAction
// TODO control_msgs/GripperCommandActionFeedback
// TODO control_msgs/GripperCommandActionGoal
// TODO control_msgs/GripperCommandActionResult
// TODO control_msgs/GripperCommandFeedback
// TODO control_msgs/GripperCommandGoal
// TODO control_msgs/GripperCommandResult
// TODO control_msgs/JointControllerState
// TODO control_msgs/JointJog
// TODO control_msgs/JointTolerance
// TODO control_msgs/JointTrajectoryAction
// TODO control_msgs/JointTrajectoryActionFeedback
// TODO control_msgs/JointTrajectoryActionGoal
// TODO control_msgs/JointTrajectoryActionResult
// TODO control_msgs/JointTrajectoryControllerState
// TODO control_msgs/JointTrajectoryFeedback
// TODO control_msgs/JointTrajectoryGoal
// TODO control_msgs/JointTrajectoryResult
// TODO control_msgs/PidState
// TODO control_msgs/PointHeadAction
// TODO control_msgs/PointHeadActionFeedback
// TODO control_msgs/PointHeadActionGoal
// TODO control_msgs/PointHeadActionResult
// TODO control_msgs/PointHeadFeedback
// TODO control_msgs/PointHeadGoal
// TODO control_msgs/PointHeadResult
// TODO control_msgs/SingleJointPositionAction
// TODO control_msgs/SingleJointPositionActionFeedback
// TODO control_msgs/SingleJointPositionActionGoal
// TODO control_msgs/SingleJointPositionActionResult
// TODO control_msgs/SingleJointPositionFeedback
// TODO control_msgs/SingleJointPositionGoal
// TODO control_msgs/SingleJointPositionResult
