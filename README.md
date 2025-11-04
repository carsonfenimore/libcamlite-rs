# libcamlite-rs
This crate brings libcamlite (https://github.com/carsonfenimore/libcamlite) support to rust!

An example using this library is See https://github.com/carsonfenimore/libcamlite-rs-test for sample code. The core of thie sample code is:

    let libcam = LibCamClient::new();
    let lowres = StreamParams{ width: 300, height: 300, format: StreamFormat::STREAM_FORMAT_RGB, framerate: 30};
    libcam.client.setupLowres(&lowres);
    let h264Params = StreamParams{ width: 1920, height: 1080, format:  StreamFormat::STREAM_FORMAT_H264, framerate: 30};
    libcam.client.setupH264(&h264Params, 5, &"main".to_owned(), &"2mbps".to_owned());
    let mycb = Box::new(MyCallback::new(lowres));
    libcam.setCallbacks(mycb);
    libcam.run();

This produces rock steady 30 fps of both rgb and h264:

    low: 31 calls in past 1.0325267 seconds; 30.023436 calls/sec
    h264: 31 calls in past 1.0341291 seconds; 29.976913 calls/sec
    Wrote output.ppm
    low: 31 calls in past 1.0325396 seconds; 30.023062 calls/sec
    h264: 30 calls in past 1.0032188 seconds; 29.903748 calls/sec

All this at the cost of about 70MB of ram a load average of 0.5 on a raspberry pi zero 2w running bullseye aarch64!  Note: you can view the output.h264 file in VLC or individual ppm conversion of the rgb stills using any image viewer.

Versions:
  - 0.1.9: support latest libcamlite, which supports latest libcamera/rpicam-apps
  - 0.1.2: fixes the crate so that it builds the libcamlite dependency automatically.


## Building
You must meet all the requirements of building libcamlite - including building libcamera and rpicam-apps. See https://github.com/carsonfenimore/libcamlite for details.

You can then do the following:

```
sudo apt install -y libclang-dev
git submodule update --remote --recursive
cargo build
```
