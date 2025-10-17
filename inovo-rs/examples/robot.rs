use inovo_rs::geometry::*;
use inovo_rs::iva::CustomCommand;
use inovo_rs::robot::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    dotenv::dotenv()?;

    let mut bot = Robot::new_inovo(50003, std::env::var("DEFAULT_PSU_HOST")?)?;

    // robot motion
    bot.linear(&Pose::from_vector([100.0, 100.0, 100.0]))?;

    // robot param
    bot.set_param(&MotionParam::new().set_speed(50.0))?;

    // robot current Pose
    let _: Pose = bot.get_current_pose()?;

    // sequence command
    let command_sequence = CommandSequence::new()
        .then_linear_relative(Pose::from_x(100.0))
        .then_sleep(1.0);
    bot.sequence(&command_sequence)?;

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
