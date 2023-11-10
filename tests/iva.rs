use inovo_rs::geometry::*;
use inovo_rs::iva::*;
use inovo_rs::robot::*;

#[test]
pub fn iva_test() {
    let _ = CommandSequence::new()
        .then(RobotCommand::Motion(
            MotionType::Linear,
            Pose::Joint(JointCoord::identity()),
        ))
        .then_linear_relative(Transform::from_z(-10.0))
        .then_set_param(&MotionParam::default())
        .then_sleep(10.0)
        .then_sync();

    let a = Instruction::Execute(RobotCommand::Motion(
        MotionType::JointRelative,
        Pose::Transform(Transform::from_x(100.0)),
    ));

    println!("{}", a);

    let transform = Transform::from_rx(180.0).then_ry(90.0);

    let a = Instruction::Execute(RobotCommand::Motion(
        MotionType::JointRelative,
        Pose::Transform(transform),
    ));
    println!("{}", a);

    let transform =
        Transform::from_x(1.0).then_relative_to(Transform::from_y(2.0), Transform::from_rz(90.0));

    let a = Instruction::Execute(RobotCommand::Motion(
        MotionType::JointRelative,
        Pose::Transform(transform),
    ));
    println!("{}", a);

    let a = Instruction::Execute(RobotCommand::Motion(
        MotionType::JointRelative,
        Pose::Joint(JointCoord::identity().set_j1(180.0)),
    ));
    println!("{}", a);
    let a = Instruction::Enqueue(RobotCommand::Sleep(12f64));
    println!("{}", a);
    let a = Instruction::Enqueue(RobotCommand::Sync);
    println!("{}", a);
    let a = Instruction::Enqueue(RobotCommand::Param(MotionParam::default()));
    println!("{}", a);
    let a = Instruction::Dequeue;
    println!("{}", a);
    let a = Instruction::Digital(IOSource::Beckhoff, 1, IOType::Input);
    println!("{}", a);
    let a = Instruction::Digital(IOSource::Wrist, 1, IOType::Output(IOState::High));
    println!("{}", a);
    let a = Instruction::Gripper(GripperCommand::Activate);
    println!("{}", a);
    let a = Instruction::Gripper(GripperCommand::Set("OPEN".to_string()));
    println!("{}", a);
    let a = Instruction::Current(PoseType::Frame);
    println!("{}", a);
}
