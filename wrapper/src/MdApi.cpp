#include "ctp-rs/wrapper/include/MdApi.h"
#include "ctp-rs/wrapper/include/Converter.h"

MdApi::MdApi(rust::Box<MdSpi> gateway, rust::String flow_path, bool is_using_udp, bool is_multicast, bool is_production_mode) {
    spi = new CMdSpi(std::move(gateway));
    api = CThostFtdcMdApi::CreateFtdcMdApi(flow_path.c_str(), is_using_udp, is_multicast, is_production_mode);
    api->RegisterSpi(spi);
}

MdApi::~MdApi() {
    if (api) {
        api->Release();
        api = nullptr;
    }
    delete spi;
    spi = nullptr;
}

std::unique_ptr<MdApi> CreateMdApi(rust::Box<MdSpi> gateway, rust::String flow_path, bool is_using_udp, bool is_multicast, bool is_production_mode) {
    return std::make_unique<MdApi>(std::move(gateway), flow_path, is_using_udp, is_multicast, is_production_mode);
}

rust::String MdApi::GetApiVersion() const {
    return api->GetApiVersion(
    );
}

void MdApi::Init() const {
    return api->Init(
    );
}

int32_t MdApi::Join() const {
    return api->Join(
    );
}

rust::String MdApi::GetTradingDay() const {
    return api->GetTradingDay(
    );
}

void MdApi::RegisterFront(rust::String pszFrontAddress) const {
    return api->RegisterFront(
        const_cast<char *>(pszFrontAddress.c_str())
    );
}

void MdApi::RegisterNameServer(rust::String pszNsAddress) const {
    return api->RegisterNameServer(
        const_cast<char *>(pszNsAddress.c_str())
    );
}

void MdApi::RegisterFensUserInfo(FensUserInfoField pFensUserInfo) const {
    CThostFtdcFensUserInfoField req(Converter::FensUserInfoFieldToCpp(pFensUserInfo));
    return api->RegisterFensUserInfo(
        &req
    );
}

int32_t MdApi::SubscribeMarketData(rust::Vec<rust::String> ppInstrumentID) const {
    char **ppInstrumentID_raw = new char *[ppInstrumentID.size()];
    for (int i = 0; i < ppInstrumentID.size(); i++)
        ppInstrumentID_raw[i] = (char *)ppInstrumentID[i].c_str();
    int ret = api->SubscribeMarketData(ppInstrumentID_raw, ppInstrumentID.size());
    delete[] ppInstrumentID_raw;
    return ret;
}

int32_t MdApi::UnSubscribeMarketData(rust::Vec<rust::String> ppInstrumentID) const {
    char **ppInstrumentID_raw = new char *[ppInstrumentID.size()];
    for (int i = 0; i < ppInstrumentID.size(); i++)
        ppInstrumentID_raw[i] = (char *)ppInstrumentID[i].c_str();
    int ret = api->UnSubscribeMarketData(ppInstrumentID_raw, ppInstrumentID.size());
    delete[] ppInstrumentID_raw;
    return ret;
}

int32_t MdApi::SubscribeForQuoteRsp(rust::Vec<rust::String> ppInstrumentID) const {
    char **ppInstrumentID_raw = new char *[ppInstrumentID.size()];
    for (int i = 0; i < ppInstrumentID.size(); i++)
        ppInstrumentID_raw[i] = (char *)ppInstrumentID[i].c_str();
    int ret = api->SubscribeForQuoteRsp(ppInstrumentID_raw, ppInstrumentID.size());
    delete[] ppInstrumentID_raw;
    return ret;
}

int32_t MdApi::UnSubscribeForQuoteRsp(rust::Vec<rust::String> ppInstrumentID) const {
    char **ppInstrumentID_raw = new char *[ppInstrumentID.size()];
    for (int i = 0; i < ppInstrumentID.size(); i++)
        ppInstrumentID_raw[i] = (char *)ppInstrumentID[i].c_str();
    int ret = api->UnSubscribeForQuoteRsp(ppInstrumentID_raw, ppInstrumentID.size());
    delete[] ppInstrumentID_raw;
    return ret;
}

int32_t MdApi::ReqUserLogin(ReqUserLoginField pReqUserLoginField, int32_t nRequestID) const {
    CThostFtdcReqUserLoginField req(Converter::ReqUserLoginFieldToCpp(pReqUserLoginField));
    return api->ReqUserLogin(
        &req,
        nRequestID
    );
}

int32_t MdApi::ReqUserLogout(UserLogoutField pUserLogout, int32_t nRequestID) const {
    CThostFtdcUserLogoutField req(Converter::UserLogoutFieldToCpp(pUserLogout));
    return api->ReqUserLogout(
        &req,
        nRequestID
    );
}

int32_t MdApi::ReqQryMulticastInstrument(QryMulticastInstrumentField pQryMulticastInstrument, int32_t nRequestID) const {
    CThostFtdcQryMulticastInstrumentField req(Converter::QryMulticastInstrumentFieldToCpp(pQryMulticastInstrument));
    return api->ReqQryMulticastInstrument(
        &req,
        nRequestID
    );
}
