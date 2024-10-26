#pragma once

#include "../submodules/libcamlite/src/libcamlite.hpp"
#include "../submodules/libcamlite/src/stream_info.hpp"

#include "rust/cxx.h"
#include <memory>
#include <functional>

namespace libcamlite {

struct LibCamWrapCallback;

class LibCamWrap {
public:
  LibCamWrap();
  void setCallback(rust::Box<LibCamWrapCallback> obj) const;
  void setupLowres(const StreamParams& params) const;
  void setupH264(const StreamParams& paramsIn, uint8_t intra, const rust::String& profile, const rust::String& bitrate) const;
  void start(bool detach) const;
private:
  class Impl;
  std::shared_ptr<Impl> impl;

};

std::unique_ptr<LibCamWrap> new_libcamwrap();


std::unique_ptr<StreamInfo> new_streaminfo();

}

