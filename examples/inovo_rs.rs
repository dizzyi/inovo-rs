use inovo_rs::geometry::*;
use inovo_rs::logger::Logger;
use inovo_rs::robot::*;

fn main() -> Result<(), RobotError> {
    let mut logger = Logger::default_target("Robot Example");

    logger.info("Creating new robot.");
    let mut bot = Robot::defaut_logger(50003, "192.168.1.121")?;

    let param_1 = MotionParam::new()
        .set_speed(100.0)
        .set_accel(100.0)
        .set_blend_linear(1000.0)
        .set_blend_angular(720.0)
        .set_tcp_speed_linear(1000.0)
        .set_tcp_speed_angular(720.0);

    let param_2 = MotionParam::new()
        .set_speed(10.0)
        .set_accel(10.0)
        .set_blend_linear(1.0)
        .set_blend_angular(1.0)
        .set_tcp_speed_linear(100.0)
        .set_tcp_speed_angular(100.0);

    let param_3 = MotionParam::new()
        .set_speed(50.0)
        .set_accel(50.0)
        // .set_blend_linear(1.0)
        // .set_blend_angular(1.0)
        .set_tcp_speed_linear(100.0)
        .set_tcp_speed_angular(90.0);

    let t = bot.get_current_transform()?;
    let j = bot.get_current_joint()?;

    bot.set_param(param_1.clone())?;

    bot.linear(t.clone().then_x(100.0))?;
    bot.sleep(1.0)?;

    bot.linear_relative(Transform::from_z(100.0))?;
    bot.sleep(1.0)?;

    bot.set_param(param_2.clone())?;

    bot.joint(t.clone().then_y(100.0))?;
    bot.sleep(1.0)?;

    bot.joint_relative(Transform::from_vector([100.0, 100.0, 100.0]))?;
    bot.sleep(1.0)?;

    bot.joint(
        j.clone()
            .then_j1(45.0)
            .then_j2(10.0)
            .then_j3(10.0)
            .then_j4(10.0)
            .then_j5(10.0)
            .then_j6(10.0),
    )?;

    bot.set_param(param_3.clone())?;
    bot.joint(t.clone())?;

    let command_sequence = CommandSequence::new()
        .then_set_param(param_1.clone())
        .then_linear(t.clone())
        .then_sleep(1.0)
        .then_linear(t.clone().then_x(100.0))
        .then_linear_relative(Transform::from_z(100.0))
        .then_set_param(param_2.clone())
        .then_joint(t.clone().then_y(100.0))
        .then_joint_relative(Transform::from_z(100.0))
        .then_set_param(param_3.clone())
        .then_joint(j.clone().then_j1(45.0))
        .then_joint(j.clone());

    bot.sequence(command_sequence.clone())?;

    bot.with_linear(t.clone().then_x(100.0))?;
    bot.with_linear_relative(Transform::from_z(100.0))?;
    bot.with_joint(t.clone().then_y(100.0))?;
    bot.with_joint_relative(Transform::from_rz(45.0))?;
    bot.with_joint(j.then_j1(45.0))?;

    bot.with_linear_relative(Transform::from_x(100.0))?
        .with_linear_relative(Transform::from_y(100.0))?
        .with_linear_relative(Transform::from_z(100.0))?;

    bot.with_sequence(command_sequence)?;

    bot.gripper_activate()?;
    bot.gripper_set("open")?;
    logger.info(format!("gripper get: {}", bot.gripper_get()?));
    bot.gripper_set("close")?;
    logger.info(format!("gripper get: {}", bot.gripper_get()?));

    for i in 0..8 {
        bot.beckhoff_set(i, true)?;
        bot.sleep(1.0)?;

        let b = bot.beckhoff_get(i)?;
        logger.info(format!("Beckhoff Input - port {}, state : {}", i, b));
        bot.sleep(1.0)?;

        bot.beckhoff_set(i, false)?;
        bot.sleep(0.5)?;
    }

    let my_bool: bool = bot.get_data("my bool")?;
    let my_i64: i64 = bot.get_data("my i64")?;
    let my_f64: f64 = bot.get_data("my f64")?;
    let my_string: String = bot.get_data("my string")?;
    let my_joint_coord: JointCoord = bot.get_data("my joint_coord")?;
    let my_transform: Transform = bot.get_data("my transform")?;
    let my_waypoint_j: JointCoord = bot.get_data("my waypoint")?;
    let my_waypoint_t: Transform = bot.get_data("my waypoint")?;

    logger.info(format!("my bool        : {}", my_bool));
    logger.info(format!("my i64         : {}", my_i64));
    logger.info(format!("my f64         : {}", my_f64));
    logger.info(format!("my string      : {}", my_string));
    logger.info(format!("my joint coord : {:?}", my_joint_coord));
    logger.info(format!("my transform   : {:?}", my_transform));
    logger.info(format!("my way point j : {:?}", my_waypoint_j));
    logger.info(format!("my way point t : {:?}", my_waypoint_t));

    Ok(())
}
