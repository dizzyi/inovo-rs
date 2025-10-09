use derive_more::{Deref, DerefMut};
use roslibrust::RosMessageType;
use serde::{Deserialize, Serialize};

use inovo_rs_macro::*;

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Bool {
    pub data: bool,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Byte {
    pub data: u8,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct ByteMultiArray {
    #[deref]
    #[deref_mut]
    pub data: Vec<u8>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Char {
    pub data: char,
}

#[inovo_msg("std_msgs")]
pub struct ColorRGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Duration {
    pub data: DurationData,
}

#[inovo_msg("std_msgs")]
pub struct Empty;

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Float32 {
    data: f32,
}

#[inovo_msg("std_msgs")]
pub struct Float32MultiArray {
    pub data: Vec<f32>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Floa64 {
    data: f64,
}

#[inovo_msg("std_msgs")]
pub struct Float64MultiArray {
    pub data: Vec<f64>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
pub struct Header {
    pub seq: u32,
    pub time: DurationData,
    pub frame_id: String,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Int16 {
    data: i16,
}

#[inovo_msg("std_msgs")]
pub struct Int16MultiArray {
    pub data: Vec<i16>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Int32 {
    data: i32,
}

#[inovo_msg("std_msgs")]
pub struct Int32MultiArray {
    pub data: Vec<i32>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Int64 {
    data: i64,
}

#[inovo_msg("std_msgs")]
pub struct Int64MultiArray {
    pub data: Vec<i64>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Int8 {
    data: i8,
}

#[inovo_msg("std_msgs")]
pub struct Int8MultiArray {
    pub data: Vec<i8>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
pub struct MultiArrayDimension {
    pub label: String,
    pub size: u32,
    pub stride: u32,
}

#[inovo_msg("std_msgs")]
pub struct MultiArrayLayout {
    pub data_offset: u32,
    pub dim: Vec<MultiArrayDimension>,
}

#[inovo_msg("std_msgs","String")]
#[derive(Deref, DerefMut)]
pub struct ROSString {
    data: String,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct Time {
    data: DurationData,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct UInt16 {
    data: u16,
}

#[inovo_msg("std_msgs")]
pub struct UInt16MultiArray {
    pub data: Vec<u16>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct UInt32 {
    data: u32,
}

#[inovo_msg("std_msgs")]
pub struct UInt32MultiArray {
    pub data: Vec<u32>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct UInt64 {
    data: u64,
}

#[inovo_msg("std_msgs")]
pub struct UInt64MultiArray {
    pub data: Vec<u64>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
#[derive(Deref, DerefMut)]
pub struct UInt8 {
    data: u8,
}

#[inovo_msg("std_msgs")]
pub struct UInt8MultiArray {
    pub data: Vec<u8>,
    pub layout: MultiArrayLayout,
}

#[inovo_msg("std_msgs")]
pub struct DurationData {
    pub nsecs: u64,
    pub secs: u64,
}
