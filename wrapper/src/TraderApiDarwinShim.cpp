// Compiled on macOS only when ctp-rs is built without the `localctp` feature.
//
// Mirrors MdApiDarwinShim.cpp for the trader-side framework dylib. The bytes
// live between _ctp_rs_trader_dylib_start and _ctp_rs_trader_dylib_end (see
// build.rs's trader_dylib_blob.cpp). At first call we extract the bytes to
// a temp file, dlopen() it, and dlsym() the two static entry points the
// header advertises — CreateFtdcTraderApi and GetApiVersion. Every other
// method is virtual, so once we own a CThostFtdcTraderApi* the rest of the
// wrapper calls through the vtable that the dylib already populated.
//
// Like the mdapi case, the darwin trader's CreateFtdcTraderApi takes one
// fewer arg than the linux header — bIsProductionMode is absent on darwin
// (verified with `nm -gU`: the exported symbol is
// _ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc, no trailing `b`).
// We silently drop is_production_mode at the bridge.

#include "ctp-rs/wrapper/include/DarwinDylibLoader.h"

#include <cstdio>
#include <cstdlib>
#include <dlfcn.h>
#include <mutex>

extern "C" const unsigned char ctp_rs_trader_dylib_start[];
extern "C" const unsigned char ctp_rs_trader_dylib_end[];

namespace {

using CreateFn = void* (*)(const char*);
using VersionFn = const char* (*)();

CreateFn g_create = nullptr;
VersionFn g_version = nullptr;

bool ensure_loaded() {
    static std::once_flag once;
    static bool ok = false;
    std::call_once(once, [] {
        void* h = ctp_rs::darwin::extract_and_dlopen(
            "thosttraderapi_se",
            ctp_rs_trader_dylib_start, ctp_rs_trader_dylib_end);
        if (!h) return;

        // Itanium-mangled names for:
        //   CThostFtdcTraderApi::CreateFtdcTraderApi(char const*)
        //   CThostFtdcTraderApi::GetApiVersion()
        g_create = reinterpret_cast<CreateFn>(
            ::dlsym(h, "_ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc"));
        g_version = reinterpret_cast<VersionFn>(
            ::dlsym(h, "_ZN19CThostFtdcTraderApi13GetApiVersionEv"));
        if (!g_create || !g_version) {
            const char* err = ::dlerror();
            std::fprintf(stderr,
                "ctp-rs: dlsym traderapi entry points failed: %s\n",
                err ? err : "<no dlerror>");
            return;
        }
        ok = true;
    });
    return ok;
}

[[noreturn]] void abort_unloaded(const char* what) {
    std::fprintf(stderr,
        "ctp-rs: %s called before embedded traderapi dylib could be "
        "loaded; aborting\n",
        what);
    std::abort();
}

} // namespace

extern "C" void* CtpRsDarwinCreateFtdcTraderApi(
    const char* flow_path, bool is_production_mode) {
    if (!ensure_loaded()) abort_unloaded("CreateFtdcTraderApi");
    (void)is_production_mode; // not present in the darwin SDK
    return g_create(flow_path);
}

extern "C" const char* CtpRsDarwinGetTraderApiVersion() {
    if (!ensure_loaded()) abort_unloaded("GetApiVersion");
    return g_version();
}
