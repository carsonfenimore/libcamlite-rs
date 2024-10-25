#include "stream_info.hpp"

extern "C" {
AVStream* analyze(void* streaminfo, uint8_t* mem, size_t size) {
	libcamlite::StreamInfo* sinfo = (libcamlite::StreamInfo*) streaminfo;
	return sinfo->analyze(mem, size);
}

void* begin_analysis(){
	libcamlite::StreamInfo* sinfo = new libcamlite::StreamInfo;
	return sinfo;
}

}
