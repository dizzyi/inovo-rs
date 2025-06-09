use inovo_rs::geometry::*;
use inovo_rs::iva::CustomCommand;
use inovo_rs::robot::*;

fn main() -> Result<(), RobotError> {
    let mut bot = Robot::defaut_logger(50003, "psu002")?;

    // robot motion
    bot.linear(Transform::from_vector([100.0, 100.0, 100.0]))?;

    // robot param
    bot.set_param(MotionParam::new().set_speed(50.0))?;

    // robot current transform
    let _: Transform = bot.get_current_transform()?;

    // sequence command
    let command_sequence = CommandSequence::new()
        .then_linear_relative(Transform::from_x(100.0))
        .then_sleep(1.0);
    bot.sequence(command_sequence)?;

    // gripper command
    bot.gripper_activate()?;
    let _: f64 = bot.gripper_get()?;
    bot.gripper_set("open")?;

    // get/set digital IO
    let _ = bot.beckhoff_get(0)?;
    bot.beckhoff_set(0, true)?;

    // custom command
    let custom_command = CustomCommand::new()
        .add_string("foo", "bar")
        .add_float("meaning of the universe", 42.0);
    let _: String = bot.custom(custom_command)?;

    Ok(())
}
