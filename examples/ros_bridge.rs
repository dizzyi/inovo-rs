use inovo_rs::ros_bridge::RosBridge;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), String> {
    thread::sleep(Duration::from_secs(10));
    RosBridge::new("psu002", 1000)?.run_sequence("iva")?;
    Ok(())
}
