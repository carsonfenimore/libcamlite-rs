#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libcamlite_rs::*;
use std::ffi::{CString,CStr};

extern "C" fn myh264Callback(bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool) {
    println!("Callback: h264 data of {} bytes, {} ts\n", count, timestamp_us);
}

extern "C" fn myLowresCallback(bytes: *mut u8, count: usize){
    println!("Callback: raw data of {} bytes\n", count);
}

use crate::root::libcamlite;

fn main() {
    println!("rust\n");
    // setup h264
    let h264Profile = CString::new("main").expect("Error making profile");
    let h264Bitrate = CString::new("2mbit").expect("Error making bitrate");
    let h264Stream = libcamlite::StreamParams{ width: 1920, height: 1080, format: libcamlite::StreamFormat_STREAM_FORMAT_H264, framerate: 30};
    let h264Params= libcamlite::H264Params{ stream: h264Stream, intraPeriod: 2, profile: h264Profile.as_ptr(), bitrate: h264Bitrate.as_ptr(), callback: Some(myh264Callback)  };

    let lowresStream = libcamlite::StreamParams{ width: 300, height: 300, format: libcamlite::StreamFormat_STREAM_FORMAT_RGB, framerate: 30 };
    let lowresParams = libcamlite::LowResParams{ stream: lowresStream, callback: Some(myLowresCallback) };
    unsafe {
        libcamlite::setupH264Stream(h264Params);
        libcamlite::setupLowresStream(lowresParams);
        libcamlite::start();
        libcamlite::stop();
    }
}
