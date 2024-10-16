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
  unsigned int bytesTotal = 0;
  unsigned int framesTotal = 0;
  const unsigned int FPS_REPORT_SECS = 2;
  std::chrono::time_point<std::chrono::steady_clock> last =  std::chrono::steady_clock::now();

  unsigned int detectionTotal = 0;
  unsigned int detectionGlobal = 0;
  const unsigned int DETECT_REPORT_SECS = 2;
  std::chrono::time_point<std::chrono::steady_clock> lastDetect =  std::chrono::steady_clock::now();

  void h264Callback(uint8_t* mem, size_t size, int64_t timestamp_us, bool keyframe){
	  callbackObj->callbackH264(mem, size, timestamp_us, keyframe);

	  auto now =  std::chrono::steady_clock::now();
	  auto delta = std::chrono::duration<double, std::milli>(now - last);
	  float deltaSecs = delta.count()/1000.0;
	  framesTotal++;
	  bytesTotal += size;
	  if (deltaSecs > FPS_REPORT_SECS){
		  printf("VidTest: h264 received %.2f fps %d bytes/sec\n", framesTotal / deltaSecs, (int)(bytesTotal / deltaSecs));
		  last = now;
		  bytesTotal = 0;
		  framesTotal = 0;
	  }
  }

  std::string to_zero_lead(const int value, const unsigned precision)
  {
	  std::ostringstream oss;
	  oss << std::setw(precision) << std::setfill('0') << value;
	  return oss.str();
  }

  void lowresCallback(uint8_t* mem, size_t size) {
	  callbackObj->callbackLowres(mem, size);
	  //const unsigned int numChans = 3;
	  //detect->detect(mem, lowresConfig.width, lowresConfig.height, numChans);
	  auto now =  std::chrono::steady_clock::now();
	  auto delta = std::chrono::duration<double, std::milli>(now - lastDetect);
	  float deltaSecs = delta.count()/1000.0;
	  detectionTotal++;
	  detectionGlobal++;
	  if (deltaSecs > DETECT_REPORT_SECS){
		  printf("Vidtest: lowres received %.2f fps\n", detectionTotal / deltaSecs);
		  lastDetect = now;
		  detectionTotal= 0;
	  }
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

void LibCamWrap::setupH264(StreamParams paramsIn) const {
	H264Params params;
	params.stream = paramsIn;
	params.intraPeriod = 2;
	params.profile = "main";
	params.bitrate = "2mbps";
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
