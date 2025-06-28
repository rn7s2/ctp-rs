#include "ctp-rs/wrapper/include/TraderApi.h"
#include "ctp-rs/wrapper/include/Converter.h"

TraderApi::TraderApi(const TraderSpi &gateway, rust::String flow_path) : gateway(gateway) {
    spi = new CTraderSpi(this);
    api = CThostFtdcTraderApi::CreateFtdcTraderApi(flow_path.c_str());
    api->RegisterSpi(spi);
}

std::unique_ptr<TraderApi> CreateTraderApi(const TraderSpi &gateway, rust::String flow_path) {
    return std::make_unique<TraderApi>(gateway, flow_path);
}

FrontInfoField TraderApi::GetFrontInfo() const {
    CThostFtdcFrontInfoField req{};
    memset(&req, 0, sizeof(req));
    api->GetFrontInfo(&req);
    return Converter::CThostFtdcFrontInfoFieldToRust(&req);
}

rust::String TraderApi::GetApiVersion() const {
    return api->GetApiVersion(
    );
}

void TraderApi::Init() const {
    return api->Init(
    );
}

int32_t TraderApi::Join() const {
    return api->Join(
    );
}

rust::String TraderApi::GetTradingDay() const {
    return api->GetTradingDay(
    );
}

void TraderApi::RegisterFront(rust::String pszFrontAddress) const {
    return api->RegisterFront(
        const_cast<char *>(pszFrontAddress.c_str())
    );
}

void TraderApi::RegisterNameServer(rust::String pszNsAddress) const {
    return api->RegisterNameServer(
        const_cast<char *>(pszNsAddress.c_str())
    );
}

void TraderApi::RegisterFensUserInfo(FensUserInfoField pFensUserInfo) const {
    CThostFtdcFensUserInfoField req(Converter::FensUserInfoFieldToCpp(pFensUserInfo));
    return api->RegisterFensUserInfo(
        &req
    );
}

void TraderApi::SubscribePrivateTopic(int32_t nResumeType) const {
    return api->SubscribePrivateTopic(
        (THOST_TE_RESUME_TYPE)nResumeType
    );
}

void TraderApi::SubscribePublicTopic(int32_t nResumeType) const {
    return api->SubscribePublicTopic(
        (THOST_TE_RESUME_TYPE)nResumeType
    );
}

int32_t TraderApi::ReqAuthenticate(ReqAuthenticateField pReqAuthenticateField, int32_t nRequestID) const {
    CThostFtdcReqAuthenticateField req(Converter::ReqAuthenticateFieldToCpp(pReqAuthenticateField));
    return api->ReqAuthenticate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::RegisterUserSystemInfo(UserSystemInfoField pUserSystemInfo) const {
    CThostFtdcUserSystemInfoField req(Converter::UserSystemInfoFieldToCpp(pUserSystemInfo));
    return api->RegisterUserSystemInfo(
        &req
    );
}

int32_t TraderApi::SubmitUserSystemInfo(UserSystemInfoField pUserSystemInfo) const {
    CThostFtdcUserSystemInfoField req(Converter::UserSystemInfoFieldToCpp(pUserSystemInfo));
    return api->SubmitUserSystemInfo(
        &req
    );
}

int32_t TraderApi::ReqUserLogin(ReqUserLoginField pReqUserLoginField, int32_t nRequestID) const {
    CThostFtdcReqUserLoginField req(Converter::ReqUserLoginFieldToCpp(pReqUserLoginField));
    return api->ReqUserLogin(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqUserLogout(UserLogoutField pUserLogout, int32_t nRequestID) const {
    CThostFtdcUserLogoutField req(Converter::UserLogoutFieldToCpp(pUserLogout));
    return api->ReqUserLogout(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqUserPasswordUpdate(UserPasswordUpdateField pUserPasswordUpdate, int32_t nRequestID) const {
    CThostFtdcUserPasswordUpdateField req(Converter::UserPasswordUpdateFieldToCpp(pUserPasswordUpdate));
    return api->ReqUserPasswordUpdate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqTradingAccountPasswordUpdate(TradingAccountPasswordUpdateField pTradingAccountPasswordUpdate, int32_t nRequestID) const {
    CThostFtdcTradingAccountPasswordUpdateField req(Converter::TradingAccountPasswordUpdateFieldToCpp(pTradingAccountPasswordUpdate));
    return api->ReqTradingAccountPasswordUpdate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqUserAuthMethod(ReqUserAuthMethodField pReqUserAuthMethod, int32_t nRequestID) const {
    CThostFtdcReqUserAuthMethodField req(Converter::ReqUserAuthMethodFieldToCpp(pReqUserAuthMethod));
    return api->ReqUserAuthMethod(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqGenUserCaptcha(ReqGenUserCaptchaField pReqGenUserCaptcha, int32_t nRequestID) const {
    CThostFtdcReqGenUserCaptchaField req(Converter::ReqGenUserCaptchaFieldToCpp(pReqGenUserCaptcha));
    return api->ReqGenUserCaptcha(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqGenUserText(ReqGenUserTextField pReqGenUserText, int32_t nRequestID) const {
    CThostFtdcReqGenUserTextField req(Converter::ReqGenUserTextFieldToCpp(pReqGenUserText));
    return api->ReqGenUserText(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqUserLoginWithCaptcha(ReqUserLoginWithCaptchaField pReqUserLoginWithCaptcha, int32_t nRequestID) const {
    CThostFtdcReqUserLoginWithCaptchaField req(Converter::ReqUserLoginWithCaptchaFieldToCpp(pReqUserLoginWithCaptcha));
    return api->ReqUserLoginWithCaptcha(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqUserLoginWithText(ReqUserLoginWithTextField pReqUserLoginWithText, int32_t nRequestID) const {
    CThostFtdcReqUserLoginWithTextField req(Converter::ReqUserLoginWithTextFieldToCpp(pReqUserLoginWithText));
    return api->ReqUserLoginWithText(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqUserLoginWithOTP(ReqUserLoginWithOTPField pReqUserLoginWithOTP, int32_t nRequestID) const {
    CThostFtdcReqUserLoginWithOTPField req(Converter::ReqUserLoginWithOTPFieldToCpp(pReqUserLoginWithOTP));
    return api->ReqUserLoginWithOTP(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqOrderInsert(InputOrderField pInputOrder, int32_t nRequestID) const {
    CThostFtdcInputOrderField req(Converter::InputOrderFieldToCpp(pInputOrder));
    return api->ReqOrderInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqParkedOrderInsert(ParkedOrderField pParkedOrder, int32_t nRequestID) const {
    CThostFtdcParkedOrderField req(Converter::ParkedOrderFieldToCpp(pParkedOrder));
    return api->ReqParkedOrderInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqParkedOrderAction(ParkedOrderActionField pParkedOrderAction, int32_t nRequestID) const {
    CThostFtdcParkedOrderActionField req(Converter::ParkedOrderActionFieldToCpp(pParkedOrderAction));
    return api->ReqParkedOrderAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqOrderAction(InputOrderActionField pInputOrderAction, int32_t nRequestID) const {
    CThostFtdcInputOrderActionField req(Converter::InputOrderActionFieldToCpp(pInputOrderAction));
    return api->ReqOrderAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryMaxOrderVolume(QryMaxOrderVolumeField pQryMaxOrderVolume, int32_t nRequestID) const {
    CThostFtdcQryMaxOrderVolumeField req(Converter::QryMaxOrderVolumeFieldToCpp(pQryMaxOrderVolume));
    return api->ReqQryMaxOrderVolume(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqSettlementInfoConfirm(SettlementInfoConfirmField pSettlementInfoConfirm, int32_t nRequestID) const {
    CThostFtdcSettlementInfoConfirmField req(Converter::SettlementInfoConfirmFieldToCpp(pSettlementInfoConfirm));
    return api->ReqSettlementInfoConfirm(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqRemoveParkedOrder(RemoveParkedOrderField pRemoveParkedOrder, int32_t nRequestID) const {
    CThostFtdcRemoveParkedOrderField req(Converter::RemoveParkedOrderFieldToCpp(pRemoveParkedOrder));
    return api->ReqRemoveParkedOrder(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqRemoveParkedOrderAction(RemoveParkedOrderActionField pRemoveParkedOrderAction, int32_t nRequestID) const {
    CThostFtdcRemoveParkedOrderActionField req(Converter::RemoveParkedOrderActionFieldToCpp(pRemoveParkedOrderAction));
    return api->ReqRemoveParkedOrderAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqExecOrderInsert(InputExecOrderField pInputExecOrder, int32_t nRequestID) const {
    CThostFtdcInputExecOrderField req(Converter::InputExecOrderFieldToCpp(pInputExecOrder));
    return api->ReqExecOrderInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqExecOrderAction(InputExecOrderActionField pInputExecOrderAction, int32_t nRequestID) const {
    CThostFtdcInputExecOrderActionField req(Converter::InputExecOrderActionFieldToCpp(pInputExecOrderAction));
    return api->ReqExecOrderAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqForQuoteInsert(InputForQuoteField pInputForQuote, int32_t nRequestID) const {
    CThostFtdcInputForQuoteField req(Converter::InputForQuoteFieldToCpp(pInputForQuote));
    return api->ReqForQuoteInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQuoteInsert(InputQuoteField pInputQuote, int32_t nRequestID) const {
    CThostFtdcInputQuoteField req(Converter::InputQuoteFieldToCpp(pInputQuote));
    return api->ReqQuoteInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQuoteAction(InputQuoteActionField pInputQuoteAction, int32_t nRequestID) const {
    CThostFtdcInputQuoteActionField req(Converter::InputQuoteActionFieldToCpp(pInputQuoteAction));
    return api->ReqQuoteAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqBatchOrderAction(InputBatchOrderActionField pInputBatchOrderAction, int32_t nRequestID) const {
    CThostFtdcInputBatchOrderActionField req(Converter::InputBatchOrderActionFieldToCpp(pInputBatchOrderAction));
    return api->ReqBatchOrderAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqOptionSelfCloseInsert(InputOptionSelfCloseField pInputOptionSelfClose, int32_t nRequestID) const {
    CThostFtdcInputOptionSelfCloseField req(Converter::InputOptionSelfCloseFieldToCpp(pInputOptionSelfClose));
    return api->ReqOptionSelfCloseInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqOptionSelfCloseAction(InputOptionSelfCloseActionField pInputOptionSelfCloseAction, int32_t nRequestID) const {
    CThostFtdcInputOptionSelfCloseActionField req(Converter::InputOptionSelfCloseActionFieldToCpp(pInputOptionSelfCloseAction));
    return api->ReqOptionSelfCloseAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqCombActionInsert(InputCombActionField pInputCombAction, int32_t nRequestID) const {
    CThostFtdcInputCombActionField req(Converter::InputCombActionFieldToCpp(pInputCombAction));
    return api->ReqCombActionInsert(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryOrder(QryOrderField pQryOrder, int32_t nRequestID) const {
    CThostFtdcQryOrderField req(Converter::QryOrderFieldToCpp(pQryOrder));
    return api->ReqQryOrder(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTrade(QryTradeField pQryTrade, int32_t nRequestID) const {
    CThostFtdcQryTradeField req(Converter::QryTradeFieldToCpp(pQryTrade));
    return api->ReqQryTrade(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorPosition(QryInvestorPositionField pQryInvestorPosition, int32_t nRequestID) const {
    CThostFtdcQryInvestorPositionField req(Converter::QryInvestorPositionFieldToCpp(pQryInvestorPosition));
    return api->ReqQryInvestorPosition(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTradingAccount(QryTradingAccountField pQryTradingAccount, int32_t nRequestID) const {
    CThostFtdcQryTradingAccountField req(Converter::QryTradingAccountFieldToCpp(pQryTradingAccount));
    return api->ReqQryTradingAccount(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestor(QryInvestorField pQryInvestor, int32_t nRequestID) const {
    CThostFtdcQryInvestorField req(Converter::QryInvestorFieldToCpp(pQryInvestor));
    return api->ReqQryInvestor(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTradingCode(QryTradingCodeField pQryTradingCode, int32_t nRequestID) const {
    CThostFtdcQryTradingCodeField req(Converter::QryTradingCodeFieldToCpp(pQryTradingCode));
    return api->ReqQryTradingCode(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInstrumentMarginRate(QryInstrumentMarginRateField pQryInstrumentMarginRate, int32_t nRequestID) const {
    CThostFtdcQryInstrumentMarginRateField req(Converter::QryInstrumentMarginRateFieldToCpp(pQryInstrumentMarginRate));
    return api->ReqQryInstrumentMarginRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInstrumentCommissionRate(QryInstrumentCommissionRateField pQryInstrumentCommissionRate, int32_t nRequestID) const {
    CThostFtdcQryInstrumentCommissionRateField req(Converter::QryInstrumentCommissionRateFieldToCpp(pQryInstrumentCommissionRate));
    return api->ReqQryInstrumentCommissionRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryExchange(QryExchangeField pQryExchange, int32_t nRequestID) const {
    CThostFtdcQryExchangeField req(Converter::QryExchangeFieldToCpp(pQryExchange));
    return api->ReqQryExchange(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryProduct(QryProductField pQryProduct, int32_t nRequestID) const {
    CThostFtdcQryProductField req(Converter::QryProductFieldToCpp(pQryProduct));
    return api->ReqQryProduct(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInstrument(QryInstrumentField pQryInstrument, int32_t nRequestID) const {
    CThostFtdcQryInstrumentField req(Converter::QryInstrumentFieldToCpp(pQryInstrument));
    return api->ReqQryInstrument(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryDepthMarketData(QryDepthMarketDataField pQryDepthMarketData, int32_t nRequestID) const {
    CThostFtdcQryDepthMarketDataField req(Converter::QryDepthMarketDataFieldToCpp(pQryDepthMarketData));
    return api->ReqQryDepthMarketData(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTraderOffer(QryTraderOfferField pQryTraderOffer, int32_t nRequestID) const {
    CThostFtdcQryTraderOfferField req(Converter::QryTraderOfferFieldToCpp(pQryTraderOffer));
    return api->ReqQryTraderOffer(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySettlementInfo(QrySettlementInfoField pQrySettlementInfo, int32_t nRequestID) const {
    CThostFtdcQrySettlementInfoField req(Converter::QrySettlementInfoFieldToCpp(pQrySettlementInfo));
    return api->ReqQrySettlementInfo(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTransferBank(QryTransferBankField pQryTransferBank, int32_t nRequestID) const {
    CThostFtdcQryTransferBankField req(Converter::QryTransferBankFieldToCpp(pQryTransferBank));
    return api->ReqQryTransferBank(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorPositionDetail(QryInvestorPositionDetailField pQryInvestorPositionDetail, int32_t nRequestID) const {
    CThostFtdcQryInvestorPositionDetailField req(Converter::QryInvestorPositionDetailFieldToCpp(pQryInvestorPositionDetail));
    return api->ReqQryInvestorPositionDetail(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryNotice(QryNoticeField pQryNotice, int32_t nRequestID) const {
    CThostFtdcQryNoticeField req(Converter::QryNoticeFieldToCpp(pQryNotice));
    return api->ReqQryNotice(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySettlementInfoConfirm(QrySettlementInfoConfirmField pQrySettlementInfoConfirm, int32_t nRequestID) const {
    CThostFtdcQrySettlementInfoConfirmField req(Converter::QrySettlementInfoConfirmFieldToCpp(pQrySettlementInfoConfirm));
    return api->ReqQrySettlementInfoConfirm(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorPositionCombineDetail(QryInvestorPositionCombineDetailField pQryInvestorPositionCombineDetail, int32_t nRequestID) const {
    CThostFtdcQryInvestorPositionCombineDetailField req(Converter::QryInvestorPositionCombineDetailFieldToCpp(pQryInvestorPositionCombineDetail));
    return api->ReqQryInvestorPositionCombineDetail(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryCFMMCTradingAccountKey(QryCFMMCTradingAccountKeyField pQryCFMMCTradingAccountKey, int32_t nRequestID) const {
    CThostFtdcQryCFMMCTradingAccountKeyField req(Converter::QryCFMMCTradingAccountKeyFieldToCpp(pQryCFMMCTradingAccountKey));
    return api->ReqQryCFMMCTradingAccountKey(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryEWarrantOffset(QryEWarrantOffsetField pQryEWarrantOffset, int32_t nRequestID) const {
    CThostFtdcQryEWarrantOffsetField req(Converter::QryEWarrantOffsetFieldToCpp(pQryEWarrantOffset));
    return api->ReqQryEWarrantOffset(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorProductGroupMargin(QryInvestorProductGroupMarginField pQryInvestorProductGroupMargin, int32_t nRequestID) const {
    CThostFtdcQryInvestorProductGroupMarginField req(Converter::QryInvestorProductGroupMarginFieldToCpp(pQryInvestorProductGroupMargin));
    return api->ReqQryInvestorProductGroupMargin(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryExchangeMarginRate(QryExchangeMarginRateField pQryExchangeMarginRate, int32_t nRequestID) const {
    CThostFtdcQryExchangeMarginRateField req(Converter::QryExchangeMarginRateFieldToCpp(pQryExchangeMarginRate));
    return api->ReqQryExchangeMarginRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryExchangeMarginRateAdjust(QryExchangeMarginRateAdjustField pQryExchangeMarginRateAdjust, int32_t nRequestID) const {
    CThostFtdcQryExchangeMarginRateAdjustField req(Converter::QryExchangeMarginRateAdjustFieldToCpp(pQryExchangeMarginRateAdjust));
    return api->ReqQryExchangeMarginRateAdjust(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryExchangeRate(QryExchangeRateField pQryExchangeRate, int32_t nRequestID) const {
    CThostFtdcQryExchangeRateField req(Converter::QryExchangeRateFieldToCpp(pQryExchangeRate));
    return api->ReqQryExchangeRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySecAgentACIDMap(QrySecAgentACIDMapField pQrySecAgentACIDMap, int32_t nRequestID) const {
    CThostFtdcQrySecAgentACIDMapField req(Converter::QrySecAgentACIDMapFieldToCpp(pQrySecAgentACIDMap));
    return api->ReqQrySecAgentACIDMap(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryProductExchRate(QryProductExchRateField pQryProductExchRate, int32_t nRequestID) const {
    CThostFtdcQryProductExchRateField req(Converter::QryProductExchRateFieldToCpp(pQryProductExchRate));
    return api->ReqQryProductExchRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryProductGroup(QryProductGroupField pQryProductGroup, int32_t nRequestID) const {
    CThostFtdcQryProductGroupField req(Converter::QryProductGroupFieldToCpp(pQryProductGroup));
    return api->ReqQryProductGroup(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryMMInstrumentCommissionRate(QryMMInstrumentCommissionRateField pQryMMInstrumentCommissionRate, int32_t nRequestID) const {
    CThostFtdcQryMMInstrumentCommissionRateField req(Converter::QryMMInstrumentCommissionRateFieldToCpp(pQryMMInstrumentCommissionRate));
    return api->ReqQryMMInstrumentCommissionRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryMMOptionInstrCommRate(QryMMOptionInstrCommRateField pQryMMOptionInstrCommRate, int32_t nRequestID) const {
    CThostFtdcQryMMOptionInstrCommRateField req(Converter::QryMMOptionInstrCommRateFieldToCpp(pQryMMOptionInstrCommRate));
    return api->ReqQryMMOptionInstrCommRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInstrumentOrderCommRate(QryInstrumentOrderCommRateField pQryInstrumentOrderCommRate, int32_t nRequestID) const {
    CThostFtdcQryInstrumentOrderCommRateField req(Converter::QryInstrumentOrderCommRateFieldToCpp(pQryInstrumentOrderCommRate));
    return api->ReqQryInstrumentOrderCommRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySecAgentTradingAccount(QryTradingAccountField pQryTradingAccount, int32_t nRequestID) const {
    CThostFtdcQryTradingAccountField req(Converter::QryTradingAccountFieldToCpp(pQryTradingAccount));
    return api->ReqQrySecAgentTradingAccount(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySecAgentCheckMode(QrySecAgentCheckModeField pQrySecAgentCheckMode, int32_t nRequestID) const {
    CThostFtdcQrySecAgentCheckModeField req(Converter::QrySecAgentCheckModeFieldToCpp(pQrySecAgentCheckMode));
    return api->ReqQrySecAgentCheckMode(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySecAgentTradeInfo(QrySecAgentTradeInfoField pQrySecAgentTradeInfo, int32_t nRequestID) const {
    CThostFtdcQrySecAgentTradeInfoField req(Converter::QrySecAgentTradeInfoFieldToCpp(pQrySecAgentTradeInfo));
    return api->ReqQrySecAgentTradeInfo(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryOptionInstrTradeCost(QryOptionInstrTradeCostField pQryOptionInstrTradeCost, int32_t nRequestID) const {
    CThostFtdcQryOptionInstrTradeCostField req(Converter::QryOptionInstrTradeCostFieldToCpp(pQryOptionInstrTradeCost));
    return api->ReqQryOptionInstrTradeCost(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryOptionInstrCommRate(QryOptionInstrCommRateField pQryOptionInstrCommRate, int32_t nRequestID) const {
    CThostFtdcQryOptionInstrCommRateField req(Converter::QryOptionInstrCommRateFieldToCpp(pQryOptionInstrCommRate));
    return api->ReqQryOptionInstrCommRate(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryExecOrder(QryExecOrderField pQryExecOrder, int32_t nRequestID) const {
    CThostFtdcQryExecOrderField req(Converter::QryExecOrderFieldToCpp(pQryExecOrder));
    return api->ReqQryExecOrder(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryForQuote(QryForQuoteField pQryForQuote, int32_t nRequestID) const {
    CThostFtdcQryForQuoteField req(Converter::QryForQuoteFieldToCpp(pQryForQuote));
    return api->ReqQryForQuote(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryQuote(QryQuoteField pQryQuote, int32_t nRequestID) const {
    CThostFtdcQryQuoteField req(Converter::QryQuoteFieldToCpp(pQryQuote));
    return api->ReqQryQuote(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryOptionSelfClose(QryOptionSelfCloseField pQryOptionSelfClose, int32_t nRequestID) const {
    CThostFtdcQryOptionSelfCloseField req(Converter::QryOptionSelfCloseFieldToCpp(pQryOptionSelfClose));
    return api->ReqQryOptionSelfClose(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestUnit(QryInvestUnitField pQryInvestUnit, int32_t nRequestID) const {
    CThostFtdcQryInvestUnitField req(Converter::QryInvestUnitFieldToCpp(pQryInvestUnit));
    return api->ReqQryInvestUnit(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryCombInstrumentGuard(QryCombInstrumentGuardField pQryCombInstrumentGuard, int32_t nRequestID) const {
    CThostFtdcQryCombInstrumentGuardField req(Converter::QryCombInstrumentGuardFieldToCpp(pQryCombInstrumentGuard));
    return api->ReqQryCombInstrumentGuard(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryCombAction(QryCombActionField pQryCombAction, int32_t nRequestID) const {
    CThostFtdcQryCombActionField req(Converter::QryCombActionFieldToCpp(pQryCombAction));
    return api->ReqQryCombAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTransferSerial(QryTransferSerialField pQryTransferSerial, int32_t nRequestID) const {
    CThostFtdcQryTransferSerialField req(Converter::QryTransferSerialFieldToCpp(pQryTransferSerial));
    return api->ReqQryTransferSerial(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryAccountregister(QryAccountregisterField pQryAccountregister, int32_t nRequestID) const {
    CThostFtdcQryAccountregisterField req(Converter::QryAccountregisterFieldToCpp(pQryAccountregister));
    return api->ReqQryAccountregister(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryContractBank(QryContractBankField pQryContractBank, int32_t nRequestID) const {
    CThostFtdcQryContractBankField req(Converter::QryContractBankFieldToCpp(pQryContractBank));
    return api->ReqQryContractBank(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryParkedOrder(QryParkedOrderField pQryParkedOrder, int32_t nRequestID) const {
    CThostFtdcQryParkedOrderField req(Converter::QryParkedOrderFieldToCpp(pQryParkedOrder));
    return api->ReqQryParkedOrder(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryParkedOrderAction(QryParkedOrderActionField pQryParkedOrderAction, int32_t nRequestID) const {
    CThostFtdcQryParkedOrderActionField req(Converter::QryParkedOrderActionFieldToCpp(pQryParkedOrderAction));
    return api->ReqQryParkedOrderAction(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryTradingNotice(QryTradingNoticeField pQryTradingNotice, int32_t nRequestID) const {
    CThostFtdcQryTradingNoticeField req(Converter::QryTradingNoticeFieldToCpp(pQryTradingNotice));
    return api->ReqQryTradingNotice(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryBrokerTradingParams(QryBrokerTradingParamsField pQryBrokerTradingParams, int32_t nRequestID) const {
    CThostFtdcQryBrokerTradingParamsField req(Converter::QryBrokerTradingParamsFieldToCpp(pQryBrokerTradingParams));
    return api->ReqQryBrokerTradingParams(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryBrokerTradingAlgos(QryBrokerTradingAlgosField pQryBrokerTradingAlgos, int32_t nRequestID) const {
    CThostFtdcQryBrokerTradingAlgosField req(Converter::QryBrokerTradingAlgosFieldToCpp(pQryBrokerTradingAlgos));
    return api->ReqQryBrokerTradingAlgos(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQueryCFMMCTradingAccountToken(QueryCFMMCTradingAccountTokenField pQueryCFMMCTradingAccountToken, int32_t nRequestID) const {
    CThostFtdcQueryCFMMCTradingAccountTokenField req(Converter::QueryCFMMCTradingAccountTokenFieldToCpp(pQueryCFMMCTradingAccountToken));
    return api->ReqQueryCFMMCTradingAccountToken(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqFromBankToFutureByFuture(ReqTransferField pReqTransfer, int32_t nRequestID) const {
    CThostFtdcReqTransferField req(Converter::ReqTransferFieldToCpp(pReqTransfer));
    return api->ReqFromBankToFutureByFuture(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqFromFutureToBankByFuture(ReqTransferField pReqTransfer, int32_t nRequestID) const {
    CThostFtdcReqTransferField req(Converter::ReqTransferFieldToCpp(pReqTransfer));
    return api->ReqFromFutureToBankByFuture(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQueryBankAccountMoneyByFuture(ReqQueryAccountField pReqQueryAccount, int32_t nRequestID) const {
    CThostFtdcReqQueryAccountField req(Converter::ReqQueryAccountFieldToCpp(pReqQueryAccount));
    return api->ReqQueryBankAccountMoneyByFuture(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryClassifiedInstrument(QryClassifiedInstrumentField pQryClassifiedInstrument, int32_t nRequestID) const {
    CThostFtdcQryClassifiedInstrumentField req(Converter::QryClassifiedInstrumentFieldToCpp(pQryClassifiedInstrument));
    return api->ReqQryClassifiedInstrument(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryCombPromotionParam(QryCombPromotionParamField pQryCombPromotionParam, int32_t nRequestID) const {
    CThostFtdcQryCombPromotionParamField req(Converter::QryCombPromotionParamFieldToCpp(pQryCombPromotionParam));
    return api->ReqQryCombPromotionParam(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRiskSettleInvstPosition(QryRiskSettleInvstPositionField pQryRiskSettleInvstPosition, int32_t nRequestID) const {
    CThostFtdcQryRiskSettleInvstPositionField req(Converter::QryRiskSettleInvstPositionFieldToCpp(pQryRiskSettleInvstPosition));
    return api->ReqQryRiskSettleInvstPosition(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRiskSettleProductStatus(QryRiskSettleProductStatusField pQryRiskSettleProductStatus, int32_t nRequestID) const {
    CThostFtdcQryRiskSettleProductStatusField req(Converter::QryRiskSettleProductStatusFieldToCpp(pQryRiskSettleProductStatus));
    return api->ReqQryRiskSettleProductStatus(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMFutureParameter(QrySPBMFutureParameterField pQrySPBMFutureParameter, int32_t nRequestID) const {
    CThostFtdcQrySPBMFutureParameterField req(Converter::QrySPBMFutureParameterFieldToCpp(pQrySPBMFutureParameter));
    return api->ReqQrySPBMFutureParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMOptionParameter(QrySPBMOptionParameterField pQrySPBMOptionParameter, int32_t nRequestID) const {
    CThostFtdcQrySPBMOptionParameterField req(Converter::QrySPBMOptionParameterFieldToCpp(pQrySPBMOptionParameter));
    return api->ReqQrySPBMOptionParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMIntraParameter(QrySPBMIntraParameterField pQrySPBMIntraParameter, int32_t nRequestID) const {
    CThostFtdcQrySPBMIntraParameterField req(Converter::QrySPBMIntraParameterFieldToCpp(pQrySPBMIntraParameter));
    return api->ReqQrySPBMIntraParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMInterParameter(QrySPBMInterParameterField pQrySPBMInterParameter, int32_t nRequestID) const {
    CThostFtdcQrySPBMInterParameterField req(Converter::QrySPBMInterParameterFieldToCpp(pQrySPBMInterParameter));
    return api->ReqQrySPBMInterParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMPortfDefinition(QrySPBMPortfDefinitionField pQrySPBMPortfDefinition, int32_t nRequestID) const {
    CThostFtdcQrySPBMPortfDefinitionField req(Converter::QrySPBMPortfDefinitionFieldToCpp(pQrySPBMPortfDefinition));
    return api->ReqQrySPBMPortfDefinition(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMInvestorPortfDef(QrySPBMInvestorPortfDefField pQrySPBMInvestorPortfDef, int32_t nRequestID) const {
    CThostFtdcQrySPBMInvestorPortfDefField req(Converter::QrySPBMInvestorPortfDefFieldToCpp(pQrySPBMInvestorPortfDef));
    return api->ReqQrySPBMInvestorPortfDef(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorPortfMarginRatio(QryInvestorPortfMarginRatioField pQryInvestorPortfMarginRatio, int32_t nRequestID) const {
    CThostFtdcQryInvestorPortfMarginRatioField req(Converter::QryInvestorPortfMarginRatioFieldToCpp(pQryInvestorPortfMarginRatio));
    return api->ReqQryInvestorPortfMarginRatio(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorProdSPBMDetail(QryInvestorProdSPBMDetailField pQryInvestorProdSPBMDetail, int32_t nRequestID) const {
    CThostFtdcQryInvestorProdSPBMDetailField req(Converter::QryInvestorProdSPBMDetailFieldToCpp(pQryInvestorProdSPBMDetail));
    return api->ReqQryInvestorProdSPBMDetail(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorCommoditySPMMMargin(QryInvestorCommoditySPMMMarginField pQryInvestorCommoditySPMMMargin, int32_t nRequestID) const {
    CThostFtdcQryInvestorCommoditySPMMMarginField req(Converter::QryInvestorCommoditySPMMMarginFieldToCpp(pQryInvestorCommoditySPMMMargin));
    return api->ReqQryInvestorCommoditySPMMMargin(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorCommodityGroupSPMMMargin(QryInvestorCommodityGroupSPMMMarginField pQryInvestorCommodityGroupSPMMMargin, int32_t nRequestID) const {
    CThostFtdcQryInvestorCommodityGroupSPMMMarginField req(Converter::QryInvestorCommodityGroupSPMMMarginFieldToCpp(pQryInvestorCommodityGroupSPMMMargin));
    return api->ReqQryInvestorCommodityGroupSPMMMargin(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPMMInstParam(QrySPMMInstParamField pQrySPMMInstParam, int32_t nRequestID) const {
    CThostFtdcQrySPMMInstParamField req(Converter::QrySPMMInstParamFieldToCpp(pQrySPMMInstParam));
    return api->ReqQrySPMMInstParam(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPMMProductParam(QrySPMMProductParamField pQrySPMMProductParam, int32_t nRequestID) const {
    CThostFtdcQrySPMMProductParamField req(Converter::QrySPMMProductParamFieldToCpp(pQrySPMMProductParam));
    return api->ReqQrySPMMProductParam(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQrySPBMAddOnInterParameter(QrySPBMAddOnInterParameterField pQrySPBMAddOnInterParameter, int32_t nRequestID) const {
    CThostFtdcQrySPBMAddOnInterParameterField req(Converter::QrySPBMAddOnInterParameterFieldToCpp(pQrySPBMAddOnInterParameter));
    return api->ReqQrySPBMAddOnInterParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRCAMSCombProductInfo(QryRCAMSCombProductInfoField pQryRCAMSCombProductInfo, int32_t nRequestID) const {
    CThostFtdcQryRCAMSCombProductInfoField req(Converter::QryRCAMSCombProductInfoFieldToCpp(pQryRCAMSCombProductInfo));
    return api->ReqQryRCAMSCombProductInfo(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRCAMSInstrParameter(QryRCAMSInstrParameterField pQryRCAMSInstrParameter, int32_t nRequestID) const {
    CThostFtdcQryRCAMSInstrParameterField req(Converter::QryRCAMSInstrParameterFieldToCpp(pQryRCAMSInstrParameter));
    return api->ReqQryRCAMSInstrParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRCAMSIntraParameter(QryRCAMSIntraParameterField pQryRCAMSIntraParameter, int32_t nRequestID) const {
    CThostFtdcQryRCAMSIntraParameterField req(Converter::QryRCAMSIntraParameterFieldToCpp(pQryRCAMSIntraParameter));
    return api->ReqQryRCAMSIntraParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRCAMSInterParameter(QryRCAMSInterParameterField pQryRCAMSInterParameter, int32_t nRequestID) const {
    CThostFtdcQryRCAMSInterParameterField req(Converter::QryRCAMSInterParameterFieldToCpp(pQryRCAMSInterParameter));
    return api->ReqQryRCAMSInterParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRCAMSShortOptAdjustParam(QryRCAMSShortOptAdjustParamField pQryRCAMSShortOptAdjustParam, int32_t nRequestID) const {
    CThostFtdcQryRCAMSShortOptAdjustParamField req(Converter::QryRCAMSShortOptAdjustParamFieldToCpp(pQryRCAMSShortOptAdjustParam));
    return api->ReqQryRCAMSShortOptAdjustParam(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRCAMSInvestorCombPosition(QryRCAMSInvestorCombPositionField pQryRCAMSInvestorCombPosition, int32_t nRequestID) const {
    CThostFtdcQryRCAMSInvestorCombPositionField req(Converter::QryRCAMSInvestorCombPositionFieldToCpp(pQryRCAMSInvestorCombPosition));
    return api->ReqQryRCAMSInvestorCombPosition(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorProdRCAMSMargin(QryInvestorProdRCAMSMarginField pQryInvestorProdRCAMSMargin, int32_t nRequestID) const {
    CThostFtdcQryInvestorProdRCAMSMarginField req(Converter::QryInvestorProdRCAMSMarginFieldToCpp(pQryInvestorProdRCAMSMargin));
    return api->ReqQryInvestorProdRCAMSMargin(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRULEInstrParameter(QryRULEInstrParameterField pQryRULEInstrParameter, int32_t nRequestID) const {
    CThostFtdcQryRULEInstrParameterField req(Converter::QryRULEInstrParameterFieldToCpp(pQryRULEInstrParameter));
    return api->ReqQryRULEInstrParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRULEIntraParameter(QryRULEIntraParameterField pQryRULEIntraParameter, int32_t nRequestID) const {
    CThostFtdcQryRULEIntraParameterField req(Converter::QryRULEIntraParameterFieldToCpp(pQryRULEIntraParameter));
    return api->ReqQryRULEIntraParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryRULEInterParameter(QryRULEInterParameterField pQryRULEInterParameter, int32_t nRequestID) const {
    CThostFtdcQryRULEInterParameterField req(Converter::QryRULEInterParameterFieldToCpp(pQryRULEInterParameter));
    return api->ReqQryRULEInterParameter(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorProdRULEMargin(QryInvestorProdRULEMarginField pQryInvestorProdRULEMargin, int32_t nRequestID) const {
    CThostFtdcQryInvestorProdRULEMarginField req(Converter::QryInvestorProdRULEMarginFieldToCpp(pQryInvestorProdRULEMargin));
    return api->ReqQryInvestorProdRULEMargin(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorPortfSetting(QryInvestorPortfSettingField pQryInvestorPortfSetting, int32_t nRequestID) const {
    CThostFtdcQryInvestorPortfSettingField req(Converter::QryInvestorPortfSettingFieldToCpp(pQryInvestorPortfSetting));
    return api->ReqQryInvestorPortfSetting(
        &req,
        nRequestID
    );
}

int32_t TraderApi::ReqQryInvestorInfoCommRec(QryInvestorInfoCommRecField pQryInvestorInfoCommRec, int32_t nRequestID) const {
    CThostFtdcQryInvestorInfoCommRecField req(Converter::QryInvestorInfoCommRecFieldToCpp(pQryInvestorInfoCommRec));
    return api->ReqQryInvestorInfoCommRec(
        &req,
        nRequestID
    );
}
