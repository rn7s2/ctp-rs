#include "ctp-rs/wrapper/include/CTraderSpi.h"
#include "ctp-rs/wrapper/include/Converter.h"

CTraderSpi::CTraderSpi(rust::Box<TraderSpi> gateway) : gateway(std::move(gateway)) { }

void CTraderSpi::OnFrontConnected() {
    this->gateway->OnFrontConnected(
    );
}

void CTraderSpi::OnFrontDisconnected(int32_t nReason) {
    this->gateway->OnFrontDisconnected(
        nReason
    );
}

void CTraderSpi::OnHeartBeatWarning(int32_t nTimeLapse) {
    this->gateway->OnHeartBeatWarning(
        nTimeLapse
    );
}

void CTraderSpi::OnRspAuthenticate(CThostFtdcRspAuthenticateField* pRspAuthenticateField, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspAuthenticate(
        Converter::CThostFtdcRspAuthenticateFieldToRust(pRspAuthenticateField),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUserLogin(
        Converter::CThostFtdcRspUserLoginFieldToRust(pRspUserLogin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUserLogout(
        Converter::CThostFtdcUserLogoutFieldToRust(pUserLogout),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUserPasswordUpdate(
        Converter::CThostFtdcUserPasswordUpdateFieldToRust(pUserPasswordUpdate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField* pTradingAccountPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspTradingAccountPasswordUpdate(
        Converter::CThostFtdcTradingAccountPasswordUpdateFieldToRust(pTradingAccountPasswordUpdate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField* pRspUserAuthMethod, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspUserAuthMethod(
        Converter::CThostFtdcRspUserAuthMethodFieldToRust(pRspUserAuthMethod),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField* pRspGenUserCaptcha, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspGenUserCaptcha(
        Converter::CThostFtdcRspGenUserCaptchaFieldToRust(pRspGenUserCaptcha),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspGenUserText(CThostFtdcRspGenUserTextField* pRspGenUserText, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspGenUserText(
        Converter::CThostFtdcRspGenUserTextFieldToRust(pRspGenUserText),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspOrderInsert(
        Converter::CThostFtdcInputOrderFieldToRust(pInputOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspParkedOrderInsert(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspParkedOrderInsert(
        Converter::CThostFtdcParkedOrderFieldToRust(pParkedOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspParkedOrderAction(
        Converter::CThostFtdcParkedOrderActionFieldToRust(pParkedOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOrderAction(CThostFtdcInputOrderActionField* pInputOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspOrderAction(
        Converter::CThostFtdcInputOrderActionFieldToRust(pInputOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField* pQryMaxOrderVolume, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryMaxOrderVolume(
        Converter::CThostFtdcQryMaxOrderVolumeFieldToRust(pQryMaxOrderVolume),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspSettlementInfoConfirm(
        Converter::CThostFtdcSettlementInfoConfirmFieldToRust(pSettlementInfoConfirm),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspRemoveParkedOrder(
        Converter::CThostFtdcRemoveParkedOrderFieldToRust(pRemoveParkedOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspRemoveParkedOrderAction(
        Converter::CThostFtdcRemoveParkedOrderActionFieldToRust(pRemoveParkedOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspExecOrderInsert(
        Converter::CThostFtdcInputExecOrderFieldToRust(pInputExecOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspExecOrderAction(CThostFtdcInputExecOrderActionField* pInputExecOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspExecOrderAction(
        Converter::CThostFtdcInputExecOrderActionFieldToRust(pInputExecOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspForQuoteInsert(
        Converter::CThostFtdcInputForQuoteFieldToRust(pInputForQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQuoteInsert(
        Converter::CThostFtdcInputQuoteFieldToRust(pInputQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQuoteAction(CThostFtdcInputQuoteActionField* pInputQuoteAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQuoteAction(
        Converter::CThostFtdcInputQuoteActionFieldToRust(pInputQuoteAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspBatchOrderAction(
        Converter::CThostFtdcInputBatchOrderActionFieldToRust(pInputBatchOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspOptionSelfCloseInsert(
        Converter::CThostFtdcInputOptionSelfCloseFieldToRust(pInputOptionSelfClose),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField* pInputOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspOptionSelfCloseAction(
        Converter::CThostFtdcInputOptionSelfCloseActionFieldToRust(pInputOptionSelfCloseAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspCombActionInsert(
        Converter::CThostFtdcInputCombActionFieldToRust(pInputCombAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOrder(CThostFtdcOrderField* pOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryOrder(
        Converter::CThostFtdcOrderFieldToRust(pOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTrade(CThostFtdcTradeField* pTrade, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTrade(
        Converter::CThostFtdcTradeFieldToRust(pTrade),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPosition(CThostFtdcInvestorPositionField* pInvestorPosition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorPosition(
        Converter::CThostFtdcInvestorPositionFieldToRust(pInvestorPosition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTradingAccount(
        Converter::CThostFtdcTradingAccountFieldToRust(pTradingAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestor(CThostFtdcInvestorField* pInvestor, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestor(
        Converter::CThostFtdcInvestorFieldToRust(pInvestor),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTradingCode(CThostFtdcTradingCodeField* pTradingCode, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTradingCode(
        Converter::CThostFtdcTradingCodeFieldToRust(pTradingCode),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField* pInstrumentMarginRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInstrumentMarginRate(
        Converter::CThostFtdcInstrumentMarginRateFieldToRust(pInstrumentMarginRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField* pInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInstrumentCommissionRate(
        Converter::CThostFtdcInstrumentCommissionRateFieldToRust(pInstrumentCommissionRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryUserSession(CThostFtdcUserSessionField* pUserSession, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryUserSession(
        Converter::CThostFtdcUserSessionFieldToRust(pUserSession),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchange(CThostFtdcExchangeField* pExchange, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryExchange(
        Converter::CThostFtdcExchangeFieldToRust(pExchange),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryProduct(CThostFtdcProductField* pProduct, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryProduct(
        Converter::CThostFtdcProductFieldToRust(pProduct),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInstrument(
        Converter::CThostFtdcInstrumentFieldToRust(pInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryDepthMarketData(
        Converter::CThostFtdcDepthMarketDataFieldToRust(pDepthMarketData),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTraderOffer(CThostFtdcTraderOfferField* pTraderOffer, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTraderOffer(
        Converter::CThostFtdcTraderOfferFieldToRust(pTraderOffer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySettlementInfo(CThostFtdcSettlementInfoField* pSettlementInfo, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySettlementInfo(
        Converter::CThostFtdcSettlementInfoFieldToRust(pSettlementInfo),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTransferBank(CThostFtdcTransferBankField* pTransferBank, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTransferBank(
        Converter::CThostFtdcTransferBankFieldToRust(pTransferBank),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField* pInvestorPositionDetail, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorPositionDetail(
        Converter::CThostFtdcInvestorPositionDetailFieldToRust(pInvestorPositionDetail),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryNotice(CThostFtdcNoticeField* pNotice, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryNotice(
        Converter::CThostFtdcNoticeFieldToRust(pNotice),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySettlementInfoConfirm(
        Converter::CThostFtdcSettlementInfoConfirmFieldToRust(pSettlementInfoConfirm),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField* pInvestorPositionCombineDetail, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorPositionCombineDetail(
        Converter::CThostFtdcInvestorPositionCombineDetailFieldToRust(pInvestorPositionCombineDetail),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField* pCFMMCTradingAccountKey, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryCFMMCTradingAccountKey(
        Converter::CThostFtdcCFMMCTradingAccountKeyFieldToRust(pCFMMCTradingAccountKey),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField* pEWarrantOffset, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryEWarrantOffset(
        Converter::CThostFtdcEWarrantOffsetFieldToRust(pEWarrantOffset),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField* pInvestorProductGroupMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorProductGroupMargin(
        Converter::CThostFtdcInvestorProductGroupMarginFieldToRust(pInvestorProductGroupMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField* pExchangeMarginRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryExchangeMarginRate(
        Converter::CThostFtdcExchangeMarginRateFieldToRust(pExchangeMarginRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField* pExchangeMarginRateAdjust, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryExchangeMarginRateAdjust(
        Converter::CThostFtdcExchangeMarginRateAdjustFieldToRust(pExchangeMarginRateAdjust),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchangeRate(CThostFtdcExchangeRateField* pExchangeRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryExchangeRate(
        Converter::CThostFtdcExchangeRateFieldToRust(pExchangeRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField* pSecAgentACIDMap, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySecAgentACIDMap(
        Converter::CThostFtdcSecAgentACIDMapFieldToRust(pSecAgentACIDMap),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryProductExchRate(CThostFtdcProductExchRateField* pProductExchRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryProductExchRate(
        Converter::CThostFtdcProductExchRateFieldToRust(pProductExchRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryProductGroup(CThostFtdcProductGroupField* pProductGroup, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryProductGroup(
        Converter::CThostFtdcProductGroupFieldToRust(pProductGroup),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField* pMMInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryMMInstrumentCommissionRate(
        Converter::CThostFtdcMMInstrumentCommissionRateFieldToRust(pMMInstrumentCommissionRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField* pMMOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryMMOptionInstrCommRate(
        Converter::CThostFtdcMMOptionInstrCommRateFieldToRust(pMMOptionInstrCommRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField* pInstrumentOrderCommRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInstrumentOrderCommRate(
        Converter::CThostFtdcInstrumentOrderCommRateFieldToRust(pInstrumentOrderCommRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySecAgentTradingAccount(
        Converter::CThostFtdcTradingAccountFieldToRust(pTradingAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField* pSecAgentCheckMode, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySecAgentCheckMode(
        Converter::CThostFtdcSecAgentCheckModeFieldToRust(pSecAgentCheckMode),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField* pSecAgentTradeInfo, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySecAgentTradeInfo(
        Converter::CThostFtdcSecAgentTradeInfoFieldToRust(pSecAgentTradeInfo),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField* pOptionInstrTradeCost, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryOptionInstrTradeCost(
        Converter::CThostFtdcOptionInstrTradeCostFieldToRust(pOptionInstrTradeCost),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField* pOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryOptionInstrCommRate(
        Converter::CThostFtdcOptionInstrCommRateFieldToRust(pOptionInstrCommRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExecOrder(CThostFtdcExecOrderField* pExecOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryExecOrder(
        Converter::CThostFtdcExecOrderFieldToRust(pExecOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryForQuote(CThostFtdcForQuoteField* pForQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryForQuote(
        Converter::CThostFtdcForQuoteFieldToRust(pForQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryQuote(CThostFtdcQuoteField* pQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryQuote(
        Converter::CThostFtdcQuoteFieldToRust(pQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryOptionSelfClose(
        Converter::CThostFtdcOptionSelfCloseFieldToRust(pOptionSelfClose),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestUnit(CThostFtdcInvestUnitField* pInvestUnit, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestUnit(
        Converter::CThostFtdcInvestUnitFieldToRust(pInvestUnit),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField* pCombInstrumentGuard, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryCombInstrumentGuard(
        Converter::CThostFtdcCombInstrumentGuardFieldToRust(pCombInstrumentGuard),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombAction(CThostFtdcCombActionField* pCombAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryCombAction(
        Converter::CThostFtdcCombActionFieldToRust(pCombAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTransferSerial(CThostFtdcTransferSerialField* pTransferSerial, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTransferSerial(
        Converter::CThostFtdcTransferSerialFieldToRust(pTransferSerial),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryAccountregister(CThostFtdcAccountregisterField* pAccountregister, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryAccountregister(
        Converter::CThostFtdcAccountregisterFieldToRust(pAccountregister),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspError(CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspError(
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnOrder(CThostFtdcOrderField* pOrder) {
    this->gateway->OnRtnOrder(
        Converter::CThostFtdcOrderFieldToRust(pOrder)
    );
}

void CTraderSpi::OnRtnTrade(CThostFtdcTradeField* pTrade) {
    this->gateway->OnRtnTrade(
        Converter::CThostFtdcTradeFieldToRust(pTrade)
    );
}

void CTraderSpi::OnErrRtnOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnOrderInsert(
        Converter::CThostFtdcInputOrderFieldToRust(pInputOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnOrderAction(CThostFtdcOrderActionField* pOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnOrderAction(
        Converter::CThostFtdcOrderActionFieldToRust(pOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField* pInstrumentStatus) {
    this->gateway->OnRtnInstrumentStatus(
        Converter::CThostFtdcInstrumentStatusFieldToRust(pInstrumentStatus)
    );
}

void CTraderSpi::OnRtnBulletin(CThostFtdcBulletinField* pBulletin) {
    this->gateway->OnRtnBulletin(
        Converter::CThostFtdcBulletinFieldToRust(pBulletin)
    );
}

void CTraderSpi::OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField* pTradingNoticeInfo) {
    this->gateway->OnRtnTradingNotice(
        Converter::CThostFtdcTradingNoticeInfoFieldToRust(pTradingNoticeInfo)
    );
}

void CTraderSpi::OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField* pErrorConditionalOrder) {
    this->gateway->OnRtnErrorConditionalOrder(
        Converter::CThostFtdcErrorConditionalOrderFieldToRust(pErrorConditionalOrder)
    );
}

void CTraderSpi::OnRtnExecOrder(CThostFtdcExecOrderField* pExecOrder) {
    this->gateway->OnRtnExecOrder(
        Converter::CThostFtdcExecOrderFieldToRust(pExecOrder)
    );
}

void CTraderSpi::OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnExecOrderInsert(
        Converter::CThostFtdcInputExecOrderFieldToRust(pInputExecOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField* pExecOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnExecOrderAction(
        Converter::CThostFtdcExecOrderActionFieldToRust(pExecOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnForQuoteInsert(
        Converter::CThostFtdcInputForQuoteFieldToRust(pInputForQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnQuote(CThostFtdcQuoteField* pQuote) {
    this->gateway->OnRtnQuote(
        Converter::CThostFtdcQuoteFieldToRust(pQuote)
    );
}

void CTraderSpi::OnErrRtnQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnQuoteInsert(
        Converter::CThostFtdcInputQuoteFieldToRust(pInputQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnQuoteAction(CThostFtdcQuoteActionField* pQuoteAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnQuoteAction(
        Converter::CThostFtdcQuoteActionFieldToRust(pQuoteAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) {
    this->gateway->OnRtnForQuoteRsp(
        Converter::CThostFtdcForQuoteRspFieldToRust(pForQuoteRsp)
    );
}

void CTraderSpi::OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField* pCFMMCTradingAccountToken) {
    this->gateway->OnRtnCFMMCTradingAccountToken(
        Converter::CThostFtdcCFMMCTradingAccountTokenFieldToRust(pCFMMCTradingAccountToken)
    );
}

void CTraderSpi::OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField* pBatchOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnBatchOrderAction(
        Converter::CThostFtdcBatchOrderActionFieldToRust(pBatchOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose) {
    this->gateway->OnRtnOptionSelfClose(
        Converter::CThostFtdcOptionSelfCloseFieldToRust(pOptionSelfClose)
    );
}

void CTraderSpi::OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnOptionSelfCloseInsert(
        Converter::CThostFtdcInputOptionSelfCloseFieldToRust(pInputOptionSelfClose),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField* pOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnOptionSelfCloseAction(
        Converter::CThostFtdcOptionSelfCloseActionFieldToRust(pOptionSelfCloseAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnCombAction(CThostFtdcCombActionField* pCombAction) {
    this->gateway->OnRtnCombAction(
        Converter::CThostFtdcCombActionFieldToRust(pCombAction)
    );
}

void CTraderSpi::OnErrRtnCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnCombActionInsert(
        Converter::CThostFtdcInputCombActionFieldToRust(pInputCombAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRspQryContractBank(CThostFtdcContractBankField* pContractBank, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryContractBank(
        Converter::CThostFtdcContractBankFieldToRust(pContractBank),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryParkedOrder(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryParkedOrder(
        Converter::CThostFtdcParkedOrderFieldToRust(pParkedOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryParkedOrderAction(
        Converter::CThostFtdcParkedOrderActionFieldToRust(pParkedOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTradingNotice(CThostFtdcTradingNoticeField* pTradingNotice, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryTradingNotice(
        Converter::CThostFtdcTradingNoticeFieldToRust(pTradingNotice),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField* pBrokerTradingParams, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryBrokerTradingParams(
        Converter::CThostFtdcBrokerTradingParamsFieldToRust(pBrokerTradingParams),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField* pBrokerTradingAlgos, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryBrokerTradingAlgos(
        Converter::CThostFtdcBrokerTradingAlgosFieldToRust(pBrokerTradingAlgos),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField* pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQueryCFMMCTradingAccountToken(
        Converter::CThostFtdcQueryCFMMCTradingAccountTokenFieldToRust(pQueryCFMMCTradingAccountToken),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->OnRtnFromBankToFutureByBank(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->OnRtnFromFutureToBankByBank(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->OnRtnRepealFromBankToFutureByBank(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->OnRtnRepealFromFutureToBankByBank(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->OnRtnFromBankToFutureByFuture(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->OnRtnFromFutureToBankByFuture(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->OnRtnRepealFromBankToFutureByFutureManual(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->OnRtnRepealFromFutureToBankByFutureManual(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField* pNotifyQueryAccount) {
    this->gateway->OnRtnQueryBankBalanceByFuture(
        Converter::CThostFtdcNotifyQueryAccountFieldToRust(pNotifyQueryAccount)
    );
}

void CTraderSpi::OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnBankToFutureByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnFutureToBankByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnRepealBankToFutureByFutureManual(
        Converter::CThostFtdcReqRepealFieldToRust(pReqRepeal),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnRepealFutureToBankByFutureManual(
        Converter::CThostFtdcReqRepealFieldToRust(pReqRepeal),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnQueryBankBalanceByFuture(
        Converter::CThostFtdcReqQueryAccountFieldToRust(pReqQueryAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->OnRtnRepealFromBankToFutureByFuture(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->OnRtnRepealFromFutureToBankByFuture(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspFromBankToFutureByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspFromFutureToBankByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQueryBankAccountMoneyByFuture(
        Converter::CThostFtdcReqQueryAccountFieldToRust(pReqQueryAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnOpenAccountByBank(CThostFtdcOpenAccountField* pOpenAccount) {
    this->gateway->OnRtnOpenAccountByBank(
        Converter::CThostFtdcOpenAccountFieldToRust(pOpenAccount)
    );
}

void CTraderSpi::OnRtnCancelAccountByBank(CThostFtdcCancelAccountField* pCancelAccount) {
    this->gateway->OnRtnCancelAccountByBank(
        Converter::CThostFtdcCancelAccountFieldToRust(pCancelAccount)
    );
}

void CTraderSpi::OnRtnChangeAccountByBank(CThostFtdcChangeAccountField* pChangeAccount) {
    this->gateway->OnRtnChangeAccountByBank(
        Converter::CThostFtdcChangeAccountFieldToRust(pChangeAccount)
    );
}

void CTraderSpi::OnRspQryClassifiedInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryClassifiedInstrument(
        Converter::CThostFtdcInstrumentFieldToRust(pInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombPromotionParam(CThostFtdcCombPromotionParamField* pCombPromotionParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryCombPromotionParam(
        Converter::CThostFtdcCombPromotionParamFieldToRust(pCombPromotionParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRiskSettleInvstPosition(CThostFtdcRiskSettleInvstPositionField* pRiskSettleInvstPosition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRiskSettleInvstPosition(
        Converter::CThostFtdcRiskSettleInvstPositionFieldToRust(pRiskSettleInvstPosition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRiskSettleProductStatus(CThostFtdcRiskSettleProductStatusField* pRiskSettleProductStatus, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRiskSettleProductStatus(
        Converter::CThostFtdcRiskSettleProductStatusFieldToRust(pRiskSettleProductStatus),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMFutureParameter(CThostFtdcSPBMFutureParameterField* pSPBMFutureParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMFutureParameter(
        Converter::CThostFtdcSPBMFutureParameterFieldToRust(pSPBMFutureParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMOptionParameter(CThostFtdcSPBMOptionParameterField* pSPBMOptionParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMOptionParameter(
        Converter::CThostFtdcSPBMOptionParameterFieldToRust(pSPBMOptionParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMIntraParameter(CThostFtdcSPBMIntraParameterField* pSPBMIntraParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMIntraParameter(
        Converter::CThostFtdcSPBMIntraParameterFieldToRust(pSPBMIntraParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMInterParameter(CThostFtdcSPBMInterParameterField* pSPBMInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMInterParameter(
        Converter::CThostFtdcSPBMInterParameterFieldToRust(pSPBMInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMPortfDefinition(CThostFtdcSPBMPortfDefinitionField* pSPBMPortfDefinition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMPortfDefinition(
        Converter::CThostFtdcSPBMPortfDefinitionFieldToRust(pSPBMPortfDefinition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMInvestorPortfDef(CThostFtdcSPBMInvestorPortfDefField* pSPBMInvestorPortfDef, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMInvestorPortfDef(
        Converter::CThostFtdcSPBMInvestorPortfDefFieldToRust(pSPBMInvestorPortfDef),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPortfMarginRatio(CThostFtdcInvestorPortfMarginRatioField* pInvestorPortfMarginRatio, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorPortfMarginRatio(
        Converter::CThostFtdcInvestorPortfMarginRatioFieldToRust(pInvestorPortfMarginRatio),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProdSPBMDetail(CThostFtdcInvestorProdSPBMDetailField* pInvestorProdSPBMDetail, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorProdSPBMDetail(
        Converter::CThostFtdcInvestorProdSPBMDetailFieldToRust(pInvestorProdSPBMDetail),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorCommoditySPMMMargin(CThostFtdcInvestorCommoditySPMMMarginField* pInvestorCommoditySPMMMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorCommoditySPMMMargin(
        Converter::CThostFtdcInvestorCommoditySPMMMarginFieldToRust(pInvestorCommoditySPMMMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorCommodityGroupSPMMMargin(CThostFtdcInvestorCommodityGroupSPMMMarginField* pInvestorCommodityGroupSPMMMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorCommodityGroupSPMMMargin(
        Converter::CThostFtdcInvestorCommodityGroupSPMMMarginFieldToRust(pInvestorCommodityGroupSPMMMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPMMInstParam(CThostFtdcSPMMInstParamField* pSPMMInstParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPMMInstParam(
        Converter::CThostFtdcSPMMInstParamFieldToRust(pSPMMInstParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPMMProductParam(CThostFtdcSPMMProductParamField* pSPMMProductParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPMMProductParam(
        Converter::CThostFtdcSPMMProductParamFieldToRust(pSPMMProductParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMAddOnInterParameter(CThostFtdcSPBMAddOnInterParameterField* pSPBMAddOnInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQrySPBMAddOnInterParameter(
        Converter::CThostFtdcSPBMAddOnInterParameterFieldToRust(pSPBMAddOnInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSCombProductInfo(CThostFtdcRCAMSCombProductInfoField* pRCAMSCombProductInfo, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRCAMSCombProductInfo(
        Converter::CThostFtdcRCAMSCombProductInfoFieldToRust(pRCAMSCombProductInfo),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSInstrParameter(CThostFtdcRCAMSInstrParameterField* pRCAMSInstrParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRCAMSInstrParameter(
        Converter::CThostFtdcRCAMSInstrParameterFieldToRust(pRCAMSInstrParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSIntraParameter(CThostFtdcRCAMSIntraParameterField* pRCAMSIntraParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRCAMSIntraParameter(
        Converter::CThostFtdcRCAMSIntraParameterFieldToRust(pRCAMSIntraParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSInterParameter(CThostFtdcRCAMSInterParameterField* pRCAMSInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRCAMSInterParameter(
        Converter::CThostFtdcRCAMSInterParameterFieldToRust(pRCAMSInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSShortOptAdjustParam(CThostFtdcRCAMSShortOptAdjustParamField* pRCAMSShortOptAdjustParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRCAMSShortOptAdjustParam(
        Converter::CThostFtdcRCAMSShortOptAdjustParamFieldToRust(pRCAMSShortOptAdjustParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSInvestorCombPosition(CThostFtdcRCAMSInvestorCombPositionField* pRCAMSInvestorCombPosition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRCAMSInvestorCombPosition(
        Converter::CThostFtdcRCAMSInvestorCombPositionFieldToRust(pRCAMSInvestorCombPosition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProdRCAMSMargin(CThostFtdcInvestorProdRCAMSMarginField* pInvestorProdRCAMSMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorProdRCAMSMargin(
        Converter::CThostFtdcInvestorProdRCAMSMarginFieldToRust(pInvestorProdRCAMSMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRULEInstrParameter(CThostFtdcRULEInstrParameterField* pRULEInstrParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRULEInstrParameter(
        Converter::CThostFtdcRULEInstrParameterFieldToRust(pRULEInstrParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRULEIntraParameter(CThostFtdcRULEIntraParameterField* pRULEIntraParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRULEIntraParameter(
        Converter::CThostFtdcRULEIntraParameterFieldToRust(pRULEIntraParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRULEInterParameter(CThostFtdcRULEInterParameterField* pRULEInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryRULEInterParameter(
        Converter::CThostFtdcRULEInterParameterFieldToRust(pRULEInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProdRULEMargin(CThostFtdcInvestorProdRULEMarginField* pInvestorProdRULEMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorProdRULEMargin(
        Converter::CThostFtdcInvestorProdRULEMarginFieldToRust(pInvestorProdRULEMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPortfSetting(CThostFtdcInvestorPortfSettingField* pInvestorPortfSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorPortfSetting(
        Converter::CThostFtdcInvestorPortfSettingFieldToRust(pInvestorPortfSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorInfoCommRec(CThostFtdcInvestorInfoCommRecField* pInvestorInfoCommRec, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryInvestorInfoCommRec(
        Converter::CThostFtdcInvestorInfoCommRecFieldToRust(pInvestorInfoCommRec),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombLeg(CThostFtdcCombLegField* pCombLeg, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryCombLeg(
        Converter::CThostFtdcCombLegFieldToRust(pCombLeg),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspOffsetSetting(
        Converter::CThostFtdcInputOffsetSettingFieldToRust(pInputOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspCancelOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspCancelOffsetSetting(
        Converter::CThostFtdcInputOffsetSettingFieldToRust(pInputOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting) {
    this->gateway->OnRtnOffsetSetting(
        Converter::CThostFtdcOffsetSettingFieldToRust(pOffsetSetting)
    );
}

void CTraderSpi::OnErrRtnOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnOffsetSetting(
        Converter::CThostFtdcInputOffsetSettingFieldToRust(pInputOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnCancelOffsetSetting(CThostFtdcCancelOffsetSettingField* pCancelOffsetSetting, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->OnErrRtnCancelOffsetSetting(
        Converter::CThostFtdcCancelOffsetSettingFieldToRust(pCancelOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRspQryOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->OnRspQryOffsetSetting(
        Converter::CThostFtdcOffsetSettingFieldToRust(pOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}
