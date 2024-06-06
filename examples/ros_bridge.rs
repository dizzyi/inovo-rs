use inovo_rs::ros_bridge::RosBridge;

fn main() -> Result<(), String> {
    RosBridge::new("psu002", 1000).run_sequence("iva").unwrap();
    Ok(())
}
