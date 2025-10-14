use super::super::*;

// TODO AnalogWrite
// TODO BeckhoffIODriverGetLoggers
// TODO BeckhoffIODriverSetLoggerLevel

#[inovo_service("/beckhoff_io/digital_write", gpio_msgs::DigitalWrite)]
pub struct DigitalWrite;

#[inovo_service("/beckhoff_io/io_info", device_msgs::IOInfo)]
pub struct IOInfo;
