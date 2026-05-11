#include "ctp-rs/wrapper/include/CMdSpi.h"
#include "ctp-rs/wrapper/include/Converter.h"

#if defined(__APPLE__)
#include <cstring>
// The md framework dylib is always sourced from the darwin 6.7.7 SDK on
// macOS (the MdApiDarwinShim path doesn't have a localctp alternative).
// CThostFtdcRspUserLoginField is 204 bytes there vs the linux header's
// 296; copy the dylib's bytes into a linux-sized buffer with the new tail
// zeroed before reading.
namespace {
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
    CThostFtdcRspUserLoginField widened{};
    if (pRspUserLogin != nullptr) {
        std::memcpy(&widened, pRspUserLogin, kDarwinRspUserLoginSize);
        pRspUserLogin = &widened;
    }
#endif
    this->gateway->OnRspUserLogin(
        Converter::CThostFtdcRspUserLoginFieldToRust(pRspUserLogin),
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
