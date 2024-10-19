# libcamlite-rs
This crate brings libcamlite support to rust!

Libcamlite add support for accessing libcamera-supported cameras on the raspberry pi.  It provides a simple, efficient API for getting simultaneous H264 and RGB streams.  The H264 stream can be saved to a file or streamed over, say, RTSP (support not included).  The RGB stream is ideal for passing to an object detection library, such as torch or tflite.

As an example, in a few lines we can stream h264/rgb as follows:

    let libcam = LibCamClient::new();

    let lowres = StreamParams{ width: 300, height: 300, format: StreamFormat::STREAM_FORMAT_RGB, framerate: 30};
    libcam.client.setupLowres(&lowres);

    let h264Params = StreamParams{ width: 1920, height: 1080, format:  StreamFormat::STREAM_FORMAT_H264, framerate: 30};
    libcam.client.setupH264(&h264Params, 5, &"main".to_owned(), &"2mbps".to_owned());

    let mycb = Box::new(MyCallback::new(lowres));
    libcam.setCallbacks(mycb);
    libcam.run();

You should then see output on the command line every second or so indicating received frames:

    low: 31 calls in past 1.0325267 seconds; 30.023436 calls/sec
    h264: 31 calls in past 1.0341291 seconds; 29.976913 calls/sec
    Wrote output.ppm
    low: 31 calls in past 1.0325396 seconds; 30.023062 calls/sec
    h264: 30 calls in past 1.0032188 seconds; 29.903748 calls/sec

All in this takes up about 70MB of ram and a load average of 0.5 on a raspberry pi zero 2w, running bullseye aarch64! 

You can view the output.h264 file in VLC.  The PPM should be viewable in any image viewer.

See https://github.com/carsonfenimore/libcamlite-rs-test for sample code.

Note: 0.1.2 fixes the crate so that it builds the libcamlite dependency automatically.
