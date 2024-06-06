use inovo_rs::logger;
use std::thread;
use std::time::Duration;

#[test]
fn logger_test() -> Result<(), String> {
    println!("Starting . . .");
    let mut logger1 = logger::Logger::default_target("Test");
    let mut i: u128 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 1;
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(1));
        logger1.debug(format!(
            "this is a log message {:>50} {}",
            i,
            " O".repeat(j as usize)
        ));
        logger1.info(format!(
            "this is a log message {:>50} {}",
            i,
            " -".repeat(j as usize)
        ));
        logger1.warn(format!(
            "this is a log message {:>50} {}",
            i,
            " !".repeat(j as usize)
        ));
        logger1.error(format!(
            "this is a log message {:>50} {}",
            i,
            " X".repeat(j as usize)
        ));
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

#[test]
fn multi_logger() {
    let sentence = "THIS IS A LOGGER";
    let mut name = String::new();
    let mut loggers = vec![];
    for word in sentence.split(" ") {
        if name.len() != 0 {
            name.push_str(" ");
        }
        name.push_str(word);
        let mut logger = logger::Logger::default_target(name.clone());
        logger.info("a message");

        loggers.push(logger);

        for logger in loggers.iter_mut() {
            logger.info("---- another message");
        }
    }
}
