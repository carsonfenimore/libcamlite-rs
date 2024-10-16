#[cxx::bridge(namespace = "libcamlite")]
mod ffi {
    #[repr(i32)]
    enum StreamFormat {
        STREAM_FORMAT_RGB,
        STREAM_FORMAT_H264,
        STREAM_FORMAT_YUV420
    }

    struct StreamParams {
        width: u32,
        height: u32,
        format: StreamFormat,
        framerate: u8,
    }



    extern "Rust" {
        type LibCamWrapCallback;
        unsafe fn callbackH264(&self,bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool );
        unsafe fn callbackLowres(&self, bytes: *mut u8, count: usize);
    }

    unsafe extern "C++" {
        include!("libcamlite-rs/cxx/wrap.h");
        type LibCamWrap;

        fn new_libcamwrap() -> UniquePtr<LibCamWrap>;
        unsafe fn setCallback(&self, obj: Box<LibCamWrapCallback> ) ;
        fn setupLowres(&self, params: StreamParams);
        fn setupH264(&self, params: StreamParams);
        fn run(&self);
    }

    unsafe extern "C++" {
        type StreamFormat;
    }
    unsafe extern "C++" {
        type StreamParams;
    }
}

pub struct LibCamWrapCallback {
}

impl LibCamWrapCallback {
    fn callbackH264(&self,bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool ){
        println!("rust h264 callback of {} bytes with ts {} and keyframe {}\n", count, timestamp_us, keyframe);
    }
    fn callbackLowres(&self, bytes: *mut u8, count: usize){
        println!("rust low res callback of {} bytes\n", count);
    }
}


fn main() {
    use cxx::let_cxx_string;

    println!("Creating client\n");
    let client = ffi::new_libcamwrap();

    println!("Setting callback\n");
    let mut cb = Box::new(LibCamWrapCallback{});
    unsafe { client.setCallback(cb); }

    // Setup streams
    println!("Setup low res stream\n");
    client.setupLowres(ffi::StreamParams{ width: 300, height: 300, format: ffi::StreamFormat::STREAM_FORMAT_RGB, framerate: 30});

    println!("Setup high res stream\n");
    let h264Params = ffi::StreamParams{ width: 1920, height: 1080, format:  ffi::StreamFormat::STREAM_FORMAT_H264, framerate: 30};
    println!("Calling into setupH264\n");
    client.setupH264(h264Params);

    println!("Running...\n");
    client.run();
}
