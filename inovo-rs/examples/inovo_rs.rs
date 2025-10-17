use inovo_rs::geometry::*;
use inovo_rs::iva::CustomCommand;
use inovo_rs::robot::*;
use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv()?;

    info!("Creating new robot.");

    // create a new client to the robot
    let mut bot = Robot::new_inovo(50003, std::env::var("DEFAULT_PSU_HOST")?)?;

    // Motion Parameter
    //
    // motion parameter for the robot's movement
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
        // you can leave some of the parameter unset
        // the robot will just not change the value of when recieve command
        // .set_blend_linear(1.0)
        // .set_blend_angular(1.0)
        .set_tcp_speed_linear(100.0)
        .set_tcp_speed_angular(90.0);

    // Getting Current Transform and Joint Coordinate of the robot
    //
    // get the current transform of the robot
    let home_transform = bot.get_current_pose()?;
    // get the current joint coordinate of the robot
    let home_joint_coord = bot.get_current_joint()?;

    // Handling geometry data
    let vz = Pose::from_z(100.0);
    let rz = Pose::from_z(10.0);
    let vxyz = Pose::from_vector([100.0, 100.0, 100.0]);
    let tx = home_transform.then_x(100.0);
    let ty = home_transform.then_y(100.0);
    let j1 = home_joint_coord.clone().then_j1(90.0);
    let j2 = j1.clone() + JointCoord::from([10.0, 10.0, 10.0, 10.0, 10.0, 10.0]);

    // Robot Command
    //
    // set the motion of the robot
    bot.set_param(&param_1.clone())?;
    // perform a linear motion
    bot.linear(&tx)?;
    // sleep
    bot.sleep(1.0)?;
    // you can chain command
    // it will execute on at a time
    bot.linear_relative(&vz)?
        .sleep(1.0)?
        .set_param(&param_2.clone())?
        .joint(ty)?
        .sleep(1.0)?
        .joint_relative(&vxyz)?
        .sleep(1.0)?
        // joint motion can take both `JointCoord` and `Transform` as target
        // while other can only take `Transform` as target
        .joint(j1.clone())?
        .set_param(&param_3)?
        .joint(home_transform)?;

    // you can also create a command sequence for all of the command
    let command_sequence = CommandSequence::new()
        .then_set_param(param_1.clone())
        .then_linear(home_transform)
        .then_sleep(1.0)
        .then_linear(tx)
        .then_linear_relative(vz)
        .then_set_param(param_2.clone())
        .then_joint(ty)
        .then_joint_relative(vz)
        .then_set_param(param_3.clone())
        .then_joint(j2.clone())
        .then_joint(home_joint_coord.clone());
    // in this case the robot will execute all of them before responing
    bot.sequence(&command_sequence)?;

    // Context
    //
    // the `with` keywork denote context manager
    // it will create a RAII guard that reverse the motion automatically
    // after the guard is drop
    bot.with_linear(&tx)?;
    {
        let guard = bot.with_linear_relative(&vz)?;
        // do some other stuff
    } // the robot motion will automatically reverse here
      //
      // you can chain context like this
    bot.with_linear_relative(&Pose::from_x(100.0))?
        .with_linear_relative(&Pose::from_y(100.0))?
        .with_linear_relative(&Pose::from_z(100.0))?;
    //
    //
    //
    {
        let mut guard_1 = bot.with_joint(ty)?;
        let mut guard_2 = guard_1.with_joint_relative(&rz)?;
        let guard_3 = guard_2.with_joint(j1.clone())?;
        // do some other stuff
        //
        // you can early drop the guard and its motion will be reverse
        drop(guard_3)
        //
        // some other stuff
        //
    } // all guard will be reverse here
      //
      //
      // you can even do it with a while sequence
    bot.with_sequence(&command_sequence)?;

    // Gripper interface
    //
    // activating the gripper
    bot.gripper_activate()?;
    // getting/setting the gripper
    bot.gripper_set("open")?;
    info!("gripper get: {}", bot.gripper_get()?);
    bot.gripper_set("close")?;
    info!("gripper get: {}", bot.gripper_get()?);

    // Digital IO
    //
    // getting/setting the digital IO
    for i in 0..8 {
        bot.beckhoff_set(i, true)?;
        bot.sleep(1.0)?;

        let b = bot.beckhoff_get(i)?;
        info!("Beckhoff Input - port {}, state : {}", i, b);
        bot.sleep(1.0)?;

        bot.beckhoff_set(i, false)?;
        bot.sleep(0.5)?;
    }

    // Data
    //
    // getting the data from robot data storeage
    let my_bool: bool = bot.get_data("my bool")?;
    let my_i64: i64 = bot.get_data("my i64")?;
    let my_f64: f64 = bot.get_data("my f64")?;
    let my_string: String = bot.get_data("my string")?;
    let my_joint_coord: JointCoord = bot.get_data("my joint_coord")?;
    let my_transform: Pose = bot.get_data("my transform")?;
    let my_waypoint_j: JointCoord = bot.get_data("my waypoint")?;
    let my_waypoint_t: Pose = bot.get_data("my waypoint")?;
    info!("my bool        : {}", my_bool);
    info!("my i64         : {}", my_i64);
    info!("my f64         : {}", my_f64);
    info!("my string      : {}", my_string);
    info!("my joint coord : {:?}", my_joint_coord);
    info!("my transform   : {:?}", my_transform);
    info!("my way point j : {:?}", my_waypoint_j);
    info!("my way point t : {:?}", my_waypoint_t);

    // Custom Command
    //
    // you need to implement the custom command in block first
    // then you can use it in here
    let custom_command = CustomCommand::new()
        .add_float("my_float", 69.420)
        .add_string("my_string", "this is a string key");

    let response = bot.custom(custom_command)?;
    info!(response);

    Ok(())
}
