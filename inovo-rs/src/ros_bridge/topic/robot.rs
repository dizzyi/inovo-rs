use super::super::*;

// TODO CartesianVelocityControllerCmd

// TODO Config

// TODO JointPositionControllerCmd
// TODO JointPositionControllerJointTargets

#[inovo_topic("/robot/joint_states", sensor_msgs::JointState)]
pub struct JointStates;

pub mod joint_trajectory_controller {
    use super::*;

    // TODO Cancel
    // TODO Feedback
    #[inovo_topic(
        "/robot/joint_trajectory_controller/follow_joint_trajectory/goal",
        control_msgs::FollowJointTrajectoryActionGoal
    )]
    pub struct Goal;
    // TODO Result
    // TODO Status
}

// TODO joint velocity cmd
// TODO proctime
// TODO robot descriptions

#[inovo_topic("/robot/robot_state", arm_msgs::RobotState)]
pub struct RobotState;

pub mod scaled_joint_trajectory_controller {
    use super::*;

    // TODO Cancel
    // TODO Feedback

    #[inovo_topic(
        "/robot/scaled_joint_trajectory_controller/follow_joint_trajectory/goal",
        control_msgs::FollowJointTrajectoryActionGoal
    )]
    pub struct Goal;
    // TODO Result
    // TODO Status
}

// TODO spherical velocity cmd

#[inovo_topic("/robot/robot_status", device_msgs::DeviceStatus)]
pub struct RobotStatus;

#[inovo_topic("/robot/wrist/color", std_msgs::ColorRGBA)]
pub struct WristColor;
