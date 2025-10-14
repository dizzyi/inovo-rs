use inovo_rs_macro::*;

// TODO sensor_msgs/BatteryState
// TODO sensor_msgs/CameraInfo
// TODO sensor_msgs/ChannelFloat32
// TODO sensor_msgs/CompressedImage
// TODO sensor_msgs/FluidPressure
// TODO sensor_msgs/Illuminance
// TODO sensor_msgs/Image
// TODO sensor_msgs/Imu

#[inovo_msg("sensor_msgs")]
pub struct JointState {
    pub effort: Vec<f64>,
    pub name: Vec<String>,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
}

// TODO sensor_msgs/Joy
// TODO sensor_msgs/JoyFeedback
// TODO sensor_msgs/JoyFeedbackArray
// TODO sensor_msgs/LaserEcho
// TODO sensor_msgs/LaserScan
// TODO sensor_msgs/MagneticField
// TODO sensor_msgs/MultiDOFJointState
// TODO sensor_msgs/MultiEchoLaserScan
// TODO sensor_msgs/NavSatFix
// TODO sensor_msgs/NavSatStatus
// TODO sensor_msgs/PointCloud
// TODO sensor_msgs/PointCloud2
// TODO sensor_msgs/PointField
// TODO sensor_msgs/Range
// TODO sensor_msgs/RegionOfInterest
// TODO sensor_msgs/RelativeHumidity
// TODO sensor_msgs/Temperature
// TODO sensor_msgs/TimeReference
