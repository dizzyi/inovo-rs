use inovo_rs::geometry::*;
use inovo_rs::iva::*;
use inovo_rs::robot::*;

fn sub_process(bot: &mut Robot) -> Result<(), String> {
    bot.with_pose(
        MotionType::LinearRelative,
        Transform::from_euler([15.0, 15.0, 15.0]),
    )?;
    Ok(())
}

fn main() -> Result<(), String> {
    let mut robot = Robot::defaut_logger(50003, "psu002")?;

    let home = Transform::from_vector([300.0, 300.0, 400.0]).set_euler([90.0, 0.0, -180.0]);

    let station = home
        .clone()
        .then_vector([-200.0, 200.0, -100.0])
        .then_relative_rx(-60.0);

    robot.linear(home)?;

    let fast = MotionParam::default()
        .set_speed(100.0)
        .set_accel(100.0)
        .set_blend_linear(100.0)
        .set_blend_angular(90.0);

    let slow = MotionParam::default()
        .set_speed(10.0)
        .set_accel(10.0)
        .set_blend_linear(0.0)
        .set_blend_angular(0.0);

    let _ = robot
        .with_param(&fast)?
        .with_pose(MotionType::LinearRelative, Transform::from_x(200.0))?
        .with_pose(MotionType::LinearRelative, Transform::from_y(200.0))?
        .with_param(&slow)?
        .with_pose(MotionType::LinearRelative, Transform::from_z(200.0))?
        //.with_pose(MotionType::Linear, Transform::identity())?
        .with_pose(MotionType::Linear, station)?
        .sleep(5.0)?
        .custom_keyword("LIQUID")?;

    {
        let mut ctx = robot.with_param(&fast)?;

        sub_process(&mut ctx)?;

        let cj = ctx.current_joint()?;
        let seq: CommandSequence = (0..16)
            .map(|i| cj.clone() + JointCoord::from_j6(24.0 * i as f64))
            .map(|j| RobotCommand::Motion(MotionType::Joint, j.into_pose()))
            .collect();

        ctx.sequence(&seq)?;

        let cj = ctx.current_joint()?;
        let seq: CommandSequence = (0..16)
            .map(|i| cj.clone() + JointCoord::from_j4(24.0 * i as f64))
            .map(|j| RobotCommand::Motion(MotionType::Joint, j.into_pose()))
            .collect();

        ctx.sequence(&seq)?;
    }

    {
        let mut ctx1 = robot.with_param(&fast)?;
        let mut ctx2 = ctx1.with_param(&slow)?;
        let mut ctx3 = ctx2.with_pose(MotionType::LinearRelative, Transform::from_z(-100.0))?;

        let cj = ctx3.current_joint()?;
        let seq: CommandSequence = (0..16)
            .map(|i| cj.clone() + JointCoord::from_j4(48.0 * i as f64))
            .map(|j| RobotCommand::Motion(MotionType::Joint, j.into_pose()))
            .collect();

        ctx3.sequence(&seq)?;
    }

    robot
        .set_param(&fast)?
        .linear_relative(Transform::from_z(100.0))?
        .linear_relative(Transform::from_z(-100.0))?
        .linear_relative(Transform::from_x(100.0))?
        .set_param(&slow)?
        .linear_relative(Transform::from_x(-100.0))?
        .linear_relative(Transform::from_y(100.0))?
        .linear_relative(Transform::from_y(-100.0))?;

    let seq = CommandSequence::new()
        .then_set_param(&fast)
        .then_linear_relative(Transform::from_rx(15.0))
        .then_linear_relative(Transform::from_rx(-15.0))
        .then_sync()
        .then_linear_relative(Transform::from_ry(15.0))
        .then_linear_relative(Transform::from_ry(-15.0))
        .then_set_param(&slow)
        .then_sleep(1.0)
        .then_linear_relative(Transform::from_rz(15.0))
        .then_linear_relative(Transform::from_rz(-15.0));

    robot.sequence(&seq)?;

    let current = robot.current_frame()?;
    let center = current.clone().vector_only().then_x(100.0).then_y(100.0);

    println!("{:?}", current);
    println!("{:?}", center);

    let seq: CommandSequence = (0..16)
        .map(|i| Transform::from_rz(24.0 * i as f64))
        .map(move |t| current.clone().then_relative_to(center.clone(), t))
        .map(|t| RobotCommand::Motion(MotionType::Linear, t.into_pose()))
        .collect();

    robot.with_param(&fast)?.sequence(&seq)?;

    Ok(())
}
