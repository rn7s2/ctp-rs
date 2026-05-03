// Compiled on macOS only. Two responsibilities:
//
// 1. Bridge the linux header's 4-arg CreateFtdcMdApi to the darwin archive's
//    3-arg version. lib/darwin/ ships an older header set where the
//    bIsProductionMode parameter doesn't exist; the rest of the wrapper
//    compiles against lib/ headers, so we isolate the darwin call here.
//
// 2. Provide stubs for trader-side symbols the macOS market-data archive
//    references but does not define. The mac archive bundles
//    CThostFtdcUserApiImplBase, which pulls in trader login/handshake code
//    paths that link against encryption hooks and a global version string
//    living in another component of CTP's distribution that's not shipped
//    for macOS. We never reach those code paths — localctp owns the trader
//    side on macOS — so the stubs abort if invoked, surfacing the issue
//    rather than silently misbehaving.

#include "ctp-rs/lib/darwin/ThostFtdcMdApi.h"
#include <cstdio>
#include <cstdlib>

extern "C" void* CtpRsDarwinCreateFtdcMdApi(
    const char* flow_path, bool is_using_udp, bool is_multicast) {
    return CThostFtdcMdApi::CreateFtdcMdApi(flow_path, is_using_udp, is_multicast);
}

extern "C" const char* g_strSupportVersion = "";

namespace {
[[noreturn]] void darwin_stub_abort(const char* sym) {
    std::fprintf(stderr,
        "ctp-rs: %s called on darwin stub — no real CTP trader on macOS, "
        "use localctp\n",
        sym);
    std::abort();
}
}

void EncodeDataUsingAesKey(unsigned char*, unsigned char*, unsigned char*) {
    darwin_stub_abort("EncodeDataUsingAesKey");
}

void ApiEncryptFrontShakeHandData(unsigned char*, int, unsigned char*, int*, const char*) {
    darwin_stub_abort("ApiEncryptFrontShakeHandData");
}

void ApidecryptFrontShakeHandData(unsigned char*, int, unsigned char*, int*, const char*) {
    darwin_stub_abort("ApidecryptFrontShakeHandData");
}
