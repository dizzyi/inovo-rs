use inovo_rs::geometry::*;
use inovo_rs::iva::*;
use inovo_rs::logger::*;
use inovo_rs::robot::MotionParam;

#[test]
pub fn iva_test() {
    let mut logger = Logger::default_target("IVA test");

    let cmds = vec![
        RobotCommand::Synchronize,
        RobotCommand::Sleep { second: 1.0 },
        RobotCommand::SetParameter(MotionParam::new().set_speed(50.0)),
        RobotCommand::Motion {
            motion_mode: MotionMode::Linear,
            target: MotionTarget::Transform(Transform::identity()),
        },
        RobotCommand::Motion {
            motion_mode: MotionMode::JointRelative,
            target: MotionTarget::JointCoord(JointCoord::from_j1(180.0)),
        },
    ];

    let mut insts = vec![];

    for robot_command in cmds.clone().into_iter() {
        let res = Instruction::exec(robot_command);
        insts.push(res);
    }
    for robot_command in cmds.clone().into_iter() {
        let res = Instruction::enqueue(robot_command);
        insts.push(res);
    }

    insts.push(Instruction::dequeue());
    insts.push(Instruction::dequeue_push());
    insts.push(Instruction::pop());

    insts.push(Instruction::gripper(GripperCommand::Activate));
    insts.push(Instruction::gripper(GripperCommand::Get));
    insts.push(Instruction::gripper(GripperCommand::Set {
        label: "open".to_string(),
    }));

    insts.push(Instruction::io_get(IOTarget::Beckhoff, 0));
    insts.push(Instruction::io_get(IOTarget::Wrist, 1));
    insts.push(Instruction::io_set(IOTarget::Beckhoff, 0, true));
    insts.push(Instruction::io_set(IOTarget::Wrist, 2, false));

    insts.push(Instruction::get(GetTarget::Transform));
    insts.push(Instruction::get(GetTarget::JointCoord));
    insts.push(Instruction::get(GetTarget::data("some key")));

    let custom_command = CustomCommand::new()
        .add_float("value", 12.0)
        .add_string("action", "add_limit");

    insts.push(Instruction::custom(custom_command));

    for inst in insts {
        match inst.to_json() {
            Ok(json) => logger.info(format!(
                "{}",
                json.split("\n")
                    .map(|s| format!("{}{}", " ".repeat(0), s))
                    .collect::<Vec<_>>()
                    .join("\n")
            )),
            Err(e) => logger.error(e.to_string()),
        };
    }
}
