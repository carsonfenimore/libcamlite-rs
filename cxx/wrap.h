#pragma once

#include "../submodules/libcamlite/src/libcamlite.hpp"

#include "rust/cxx.h"
#include <memory>
#include <functional>

namespace libcamlite {

struct LibCamWrapCallback;

class LibCamWrap {
public:
  LibCamWrap();
  void setCallback(rust::Box<LibCamWrapCallback> obj) const;
  void setupLowres(StreamParams params) const;
  void setupH264(StreamParams paramsIn, uint8_t intra, rust::String profile, rust::String bitrate) const;
  void run() const;
private:
  class Impl;
  std::shared_ptr<Impl> impl;

};

std::unique_ptr<LibCamWrap> new_libcamwrap();

}
