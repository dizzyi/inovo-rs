use inovo_rs::ros_bridge::run_sequence;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), String> {
    thread::sleep(Duration::from_secs(10));
    run_sequence("psu002", "iva")?;
    Ok(())
}
