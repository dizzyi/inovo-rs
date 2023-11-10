use inovo_rs::logger;
use std::thread;
use std::time::Duration;

#[test]
fn logger_test() -> Result<(), String> {
    println!("Starting . . .");
    let mut logger1 = logger::Logger::default_target("Test").unwrap();
    let mut i: u128 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 1;
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(1));
        logger1.debug(format!(
            "this is a log message {:>50} {}",
            i,
            " O".repeat(j as usize)
        ))?;
        logger1.info(format!(
            "this is a log message {:>50} {}",
            i,
            " -".repeat(j as usize)
        ))?;
        logger1.warn(format!(
            "this is a log message {:>50} {}",
            i,
            " !".repeat(j as usize)
        ))?;
        i += 1;
        j += k;
        if j <= 0 {
            k = 1;
        }
        if j >= 20 {
            k = -1;
        }
    }
    Ok(())
}
