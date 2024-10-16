#include "wrap.h"
#include "libcamlite-rs/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <chrono>
#include <sstream>
#include <iomanip>

// for std::bind _1, _2, etc...
using namespace std::placeholders;  

namespace libcamlite {

class LibCamWrap::Impl {

  Impl();
  friend LibCamWrap;


  LibCamLite libcam;
  rust::Box<LibCamWrapCallback> callbackObj;

  void h264Callback(uint8_t* mem, size_t size, int64_t timestamp_us, bool keyframe){
	  callbackObj->callbackH264(mem, size, timestamp_us, keyframe);
  }

  void lowresCallback(uint8_t* mem, size_t size) {
	  callbackObj->callbackLowres(mem, size);
  }
};

LibCamWrap::LibCamWrap() : impl(new class LibCamWrap::Impl) {}

void LibCamWrap::setCallback(rust::Box<LibCamWrapCallback> obj) const {
	impl->callbackObj = std::move(obj);
}

void LibCamWrap::setupLowres(StreamParams params) const {
	printf("Setup low res %dx%d\n", params.width, params.height);
	libcamlite::LowResParams lowres;
	lowres.stream = params;
	impl->libcam.setupLowresStream(lowres, std::bind(&LibCamWrap::Impl::lowresCallback, impl.get(), _1, _2));
}

void LibCamWrap::setupH264(StreamParams paramsIn, uint8_t intra, rust::String profile, rust::String bitrate) const {
	H264Params params;
	params.stream = paramsIn;
	params.intraPeriod = intra;
	params.profile = std::string(profile);
	params.bitrate = std::string(bitrate);
	printf("Setup H264 with res %dx%d\n", params.stream.width, params.stream.height);
	impl->libcam.setupH264Stream(params, std::bind(&LibCamWrap::Impl::h264Callback, impl.get(), _1, _2, _3, _4));
}

LibCamWrap::Impl::Impl():callbackObj(rust::Box<LibCamWrapCallback>::from_raw(NULL)) {
}

void LibCamWrap::run() const{
	impl->libcam.start();
}

std::unique_ptr<LibCamWrap> new_libcamwrap() {
  return std::make_unique<LibCamWrap>();
}


}
