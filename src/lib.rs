
pub trait ExternalCallback {
    unsafe fn callbackH264(&mut self, _bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool );
    unsafe fn callbackLowres(&mut self, _bytes: *mut u8, count: usize);
}

pub struct LibCamWrapCallback {
    pub cb: Box<dyn ExternalCallback> 
}

impl LibCamWrapCallback {
    unsafe fn callbackH264(&mut self, bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool ){
        self.cb.callbackH264(bytes, count, timestamp_us, keyframe);
    }
    unsafe fn callbackLowres(&mut self, bytes: *mut u8, count: usize){
        self.cb.callbackLowres(bytes, count);
    }
}

#[cxx::bridge(namespace = "libcamlite")]
pub mod ffi {
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
        unsafe fn callbackH264(&mut self,bytes: *mut u8, count: usize, timestamp_us: i64, keyframe: bool );
        unsafe fn callbackLowres(&mut self, bytes: *mut u8, count: usize);
    }

    unsafe extern "C++" {
        include!("rslibcamlite/cxx/wrap.h");
        type LibCamWrap;

        fn new_libcamwrap() -> UniquePtr<LibCamWrap>;
        fn setCallback(&self, obj: Box<LibCamWrapCallback> ) ;
        fn setupLowres(&self, params: StreamParams);
        fn setupH264(&self, params: StreamParams, intra: u8, profile: String, bitrate: String);
        fn run(&self);
    }

    unsafe extern "C++" {
        type StreamFormat;
    }
    unsafe extern "C++" {
        type StreamParams;
    }
}


pub struct LibCamClient {
    pub client: cxx::UniquePtr<ffi::LibCamWrap>
}
impl LibCamClient {
    // Helper to make the rust side look a bit cleaner...
    pub fn new() -> Self {
        let s = LibCamClient {
            client: ffi::new_libcamwrap()
        };
        s
    }
    pub fn run(&self) {
        self.client.run();
    }
    pub fn setCallbacks(&self, callbackObj: Box<dyn ExternalCallback>) {
        let cb = Box::new(LibCamWrapCallback{ cb: callbackObj });
        self.client.setCallback(cb); 
    }
}

// Bring some needed wrappers out of ffi
pub use ffi::StreamParams;
pub use ffi::StreamFormat;
