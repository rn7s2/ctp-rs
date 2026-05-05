#pragma once

// Shared helper for the macOS shims that embed CTP framework dylibs into the
// final binary via .incbin. See MdApiDarwinShim.cpp / TraderApiDarwinShim.cpp
// for callers, and build.rs for the build-time embedding step.

namespace ctp_rs::darwin {

// Writes [blob_start, blob_end) to a per-process temp file named after `name`
// (e.g. "thostmduserapi_se" → $TMPDIR/ctp-rs.<pid>.thostmduserapi_se.dylib),
// then dlopen()s it with RTLD_NOW|RTLD_LOCAL. Returns the dlopen handle on
// success, or nullptr on failure (the failure has been logged to stderr; the
// temp file is removed on dlopen failure but kept on extraction failure to
// aid post-mortem inspection).
void* extract_and_dlopen(const char* name,
                         const unsigned char* blob_start,
                         const unsigned char* blob_end);

} // namespace ctp_rs::darwin
