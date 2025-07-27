#include "ctp-rs/wrapper/include/CMdSpi.h"
#include "ctp-rs/wrapper/include/Converter.h"

CMdSpi::CMdSpi(const MdApi* gateway) : gateway(gateway) { }

void CMdSpi::OnFrontConnected() {
    this->gateway->gateway.OnFrontConnected(
    );
}

void CMdSpi::OnFrontDisconnected(int32_t nReason) {
    this->gateway->gateway.OnFrontDisconnected(
        nReason
    );
}

void CMdSpi::OnHeartBeatWarning(int32_t nTimeLapse) {
    this->gateway->gateway.OnHeartBeatWarning(
        nTimeLapse
    );
}

void CMdSpi::OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUserLogin(
        Converter::CThostFtdcRspUserLoginFieldToRust(pRspUserLogin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUserLogout(
        Converter::CThostFtdcUserLogoutFieldToRust(pUserLogout),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField* pMulticastInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryMulticastInstrument(
        Converter::CThostFtdcMulticastInstrumentFieldToRust(pMulticastInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspError(CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspError(
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspSubMarketData(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUnSubMarketData(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspSubForQuoteRsp(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUnSubForQuoteRsp(
        Converter::CThostFtdcSpecificInstrumentFieldToRust(pSpecificInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CMdSpi::OnRtnDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData) {
    this->gateway->gateway.OnRtnDepthMarketData(
        Converter::CThostFtdcDepthMarketDataFieldToRust(pDepthMarketData)
    );
}

void CMdSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) {
    this->gateway->gateway.OnRtnForQuoteRsp(
        Converter::CThostFtdcForQuoteRspFieldToRust(pForQuoteRsp)
    );
}
