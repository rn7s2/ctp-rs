#pragma once

#include "ctp-rs/lib/6.7.9/ThostFtdcMdApi.h"
#include "ctp-rs/lib/6.7.9/ThostFtdcTraderApi.h"

#include <cstdint>

struct MdApi;

class CMdSpi : public CThostFtdcMdSpi
{
public:
    explicit CMdSpi(const MdApi *gateway);

    void OnFrontConnected() override;
    void OnFrontDisconnected(int32_t nReason) override;
    void OnHeartBeatWarning(int32_t nTimeLapse) override;
    void OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspQryMulticastInstrument(CThostFtdcMulticastInstrumentField* pMulticastInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspError(CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspUnSubMarketData(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRspUnSubForQuoteRsp(CThostFtdcSpecificInstrumentField* pSpecificInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) override;
    void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData) override;
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) override;

private:
    const MdApi *gateway;
};
