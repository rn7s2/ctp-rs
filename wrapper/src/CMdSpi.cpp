#include "ctp-rs/wrapper/include/CMdSpi.h"
#include "ctp-rs/wrapper/include/Converter.h"

#if defined(__APPLE__)
#include <cstring>
// The pinned darwin SDK lays some structs out shorter than linux (it lacks
// trailing fields linux later added). The dylib hands us a pointer to its
// native-sized struct; reading it as the linux-shaped (larger) struct would
// scan past the dylib's allocation. Copy the darwin-sized bytes into a
// linux-sized buffer with the new tail zeroed before the converter runs.
namespace {
template <typename T>
inline T widen_from_darwin(T* src, std::size_t darwin_size) {
    T widened{};
    if (src != nullptr) {
        std::memcpy(&widened, src, darwin_size);
    }
    return widened;
}
constexpr std::size_t kDarwinRspUserLoginSize = 204;
} // namespace
#endif

CMdSpi::CMdSpi(rust::Box<MdSpi> gateway) : gateway(std::move(gateway)) { }

void CMdSpi::OnFrontConnected() {
    this->gateway->OnFrontConnected(
    );
}

void CMdSpi::OnFrontDisconnected(int32_t nReason) {
    this->gateway->OnFrontDisconnected(
        nReason
    );
}

void CMdSpi::OnHeartBeatWarning(int32_t nTimeLapse) {
    this->gateway->OnHeartBeatWarning(
        nTimeLapse
    );
}

void CMdSpi::OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
#if defined(__APPLE__)
    auto widened = widen_from_darwin(pRspUserLogin, kDarwinRspUserLoginSize);
    CThostFtdcRspUserLoginField* p = (pRspUserLogin != nullptr) ? &widened : nullptr;
#else
    CThostFtdcRspUserLoginField* p = pRspUserLogin;
#endif
    this->gateway->OnRspUserLogin(
        Converter::CThostFtdcRspUserLoginFieldToRust(p),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUserLogout(
        Converter::CThostFtdcUserLogoutFieldToRust(pUserLogout),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField* pMulticastInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryMulticastInstrument(
        Converter::CThostFtdcMulticastInstrumentFieldToRust(pMulticastInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspError(CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspError(
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspSubMarketData(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUnSubMarketData(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspSubForQuoteRsp(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUnSubForQuoteRsp(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRtnDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData) {
    this->gateway->OnRtnDepthMarketData(
        Converter::CThostFtdcDepthMarketDataFieldToRust(pDepthMarketData)
    );
}

void CMdSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) {
    this->gateway->OnRtnForQuoteRsp(
        Converter::CThostFtdcForQuoteRspFieldToRust(pForQuoteRsp)
    );
}
