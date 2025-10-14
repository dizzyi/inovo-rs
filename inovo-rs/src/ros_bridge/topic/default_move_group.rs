use super::super::*;
use super::*;

#[inovo_topic(
    "/default_move_group/cartesian_jog",
    commander_msgs::CartesianJogDemand
)]
pub struct CartesianJog;

// TODO EnableTeachMode

// TODO HiddenMeshesPub

// TODO JointJog

// TODO MoveCancel

#[inovo_topic(
    "/default_move_group/move/feedback",
    commander_msgs::MotionActionFeedback
)]
pub struct MoveFeedback;

#[inovo_topic("/default_move_group/move/goal", commander_msgs::MotionActionGoal)]
pub struct MoveGoal;

#[inovo_topic("/default_move_group/move/result", commander_msgs::MotionActionResult)]
pub struct MoveResult;

#[inovo_topic("/default_move_group/move/status", actionlib_msgs::GoalStatusArray)]
pub struct MoveStatus;

#[inovo_topic("/default_move_group/move_group_info", commander_msgs::MoveGroupInfo)]
pub struct MoveGroupInfo;

// TODO ParameterDescriptions

// TODO ParameterUpdates

#[inovo_topic("/default_move_group/pose", geometry_msgs::PoseStamped)]
pub struct Pose;

#[inovo_topic("/default_move_group/tcp_pose", geometry_msgs::PoseStamped)]
pub struct TcpPose;

#[inovo_topic("/default_move_group/tcp_speed", commander_msgs::SpeedStamped)]
pub struct TcpSpeed;
