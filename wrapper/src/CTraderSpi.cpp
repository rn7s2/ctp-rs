#include "ctp4rs/wrapper/include/CTraderSpi.h"
#include "ctp4rs/wrapper/include/Converter.h"

CTraderSpi::CTraderSpi(const TraderApi* gateway) : gateway(gateway) { }

void CTraderSpi::OnFrontConnected() {
    this->gateway->gateway.OnFrontConnected(
    );
}

void CTraderSpi::OnFrontDisconnected(int32_t nReason) {
    this->gateway->gateway.OnFrontDisconnected(
        nReason
    );
}

void CTraderSpi::OnHeartBeatWarning(int32_t nTimeLapse) {
    this->gateway->gateway.OnHeartBeatWarning(
        nTimeLapse
    );
}

void CTraderSpi::OnRspAuthenticate(CThostFtdcRspAuthenticateField* pRspAuthenticateField, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspAuthenticate(
        Converter::CThostFtdcRspAuthenticateFieldToRust(pRspAuthenticateField),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUserLogin(
        Converter::CThostFtdcRspUserLoginFieldToRust(pRspUserLogin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUserLogout(
        Converter::CThostFtdcUserLogoutFieldToRust(pUserLogout),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserPasswordUpdate(CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUserPasswordUpdate(
        Converter::CThostFtdcUserPasswordUpdateFieldToRust(pUserPasswordUpdate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspTradingAccountPasswordUpdate(CThostFtdcTradingAccountPasswordUpdateField* pTradingAccountPasswordUpdate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspTradingAccountPasswordUpdate(
        Converter::CThostFtdcTradingAccountPasswordUpdateFieldToRust(pTradingAccountPasswordUpdate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField* pRspUserAuthMethod, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspUserAuthMethod(
        Converter::CThostFtdcRspUserAuthMethodFieldToRust(pRspUserAuthMethod),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField* pRspGenUserCaptcha, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspGenUserCaptcha(
        Converter::CThostFtdcRspGenUserCaptchaFieldToRust(pRspGenUserCaptcha),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspGenUserText(CThostFtdcRspGenUserTextField* pRspGenUserText, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspGenUserText(
        Converter::CThostFtdcRspGenUserTextFieldToRust(pRspGenUserText),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspOrderInsert(
        Converter::CThostFtdcInputOrderFieldToRust(pInputOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspParkedOrderInsert(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspParkedOrderInsert(
        Converter::CThostFtdcParkedOrderFieldToRust(pParkedOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspParkedOrderAction(
        Converter::CThostFtdcParkedOrderActionFieldToRust(pParkedOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOrderAction(CThostFtdcInputOrderActionField* pInputOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspOrderAction(
        Converter::CThostFtdcInputOrderActionFieldToRust(pInputOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryMaxOrderVolume(CThostFtdcQryMaxOrderVolumeField* pQryMaxOrderVolume, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryMaxOrderVolume(
        Converter::CThostFtdcQryMaxOrderVolumeFieldToRust(pQryMaxOrderVolume),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspSettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspSettlementInfoConfirm(
        Converter::CThostFtdcSettlementInfoConfirmFieldToRust(pSettlementInfoConfirm),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspRemoveParkedOrder(CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspRemoveParkedOrder(
        Converter::CThostFtdcRemoveParkedOrderFieldToRust(pRemoveParkedOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspRemoveParkedOrderAction(CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspRemoveParkedOrderAction(
        Converter::CThostFtdcRemoveParkedOrderActionFieldToRust(pRemoveParkedOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspExecOrderInsert(
        Converter::CThostFtdcInputExecOrderFieldToRust(pInputExecOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspExecOrderAction(CThostFtdcInputExecOrderActionField* pInputExecOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspExecOrderAction(
        Converter::CThostFtdcInputExecOrderActionFieldToRust(pInputExecOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspForQuoteInsert(
        Converter::CThostFtdcInputForQuoteFieldToRust(pInputForQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQuoteInsert(
        Converter::CThostFtdcInputQuoteFieldToRust(pInputQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQuoteAction(CThostFtdcInputQuoteActionField* pInputQuoteAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQuoteAction(
        Converter::CThostFtdcInputQuoteActionFieldToRust(pInputQuoteAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspBatchOrderAction(CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspBatchOrderAction(
        Converter::CThostFtdcInputBatchOrderActionFieldToRust(pInputBatchOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspOptionSelfCloseInsert(
        Converter::CThostFtdcInputOptionSelfCloseFieldToRust(pInputOptionSelfClose),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOptionSelfCloseAction(CThostFtdcInputOptionSelfCloseActionField* pInputOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspOptionSelfCloseAction(
        Converter::CThostFtdcInputOptionSelfCloseActionFieldToRust(pInputOptionSelfCloseAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspCombActionInsert(
        Converter::CThostFtdcInputCombActionFieldToRust(pInputCombAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOrder(CThostFtdcOrderField* pOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryOrder(
        Converter::CThostFtdcOrderFieldToRust(pOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTrade(CThostFtdcTradeField* pTrade, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTrade(
        Converter::CThostFtdcTradeFieldToRust(pTrade),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPosition(CThostFtdcInvestorPositionField* pInvestorPosition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorPosition(
        Converter::CThostFtdcInvestorPositionFieldToRust(pInvestorPosition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTradingAccount(
        Converter::CThostFtdcTradingAccountFieldToRust(pTradingAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestor(CThostFtdcInvestorField* pInvestor, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestor(
        Converter::CThostFtdcInvestorFieldToRust(pInvestor),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTradingCode(CThostFtdcTradingCodeField* pTradingCode, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTradingCode(
        Converter::CThostFtdcTradingCodeFieldToRust(pTradingCode),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrumentMarginRate(CThostFtdcInstrumentMarginRateField* pInstrumentMarginRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInstrumentMarginRate(
        Converter::CThostFtdcInstrumentMarginRateFieldToRust(pInstrumentMarginRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrumentCommissionRate(CThostFtdcInstrumentCommissionRateField* pInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInstrumentCommissionRate(
        Converter::CThostFtdcInstrumentCommissionRateFieldToRust(pInstrumentCommissionRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchange(CThostFtdcExchangeField* pExchange, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryExchange(
        Converter::CThostFtdcExchangeFieldToRust(pExchange),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryProduct(CThostFtdcProductField* pProduct, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryProduct(
        Converter::CThostFtdcProductFieldToRust(pProduct),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInstrument(
        Converter::CThostFtdcInstrumentFieldToRust(pInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryDepthMarketData(
        Converter::CThostFtdcDepthMarketDataFieldToRust(pDepthMarketData),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTraderOffer(CThostFtdcTraderOfferField* pTraderOffer, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTraderOffer(
        Converter::CThostFtdcTraderOfferFieldToRust(pTraderOffer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySettlementInfo(CThostFtdcSettlementInfoField* pSettlementInfo, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySettlementInfo(
        Converter::CThostFtdcSettlementInfoFieldToRust(pSettlementInfo),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTransferBank(CThostFtdcTransferBankField* pTransferBank, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTransferBank(
        Converter::CThostFtdcTransferBankFieldToRust(pTransferBank),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPositionDetail(CThostFtdcInvestorPositionDetailField* pInvestorPositionDetail, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorPositionDetail(
        Converter::CThostFtdcInvestorPositionDetailFieldToRust(pInvestorPositionDetail),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryNotice(CThostFtdcNoticeField* pNotice, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryNotice(
        Converter::CThostFtdcNoticeFieldToRust(pNotice),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySettlementInfoConfirm(CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySettlementInfoConfirm(
        Converter::CThostFtdcSettlementInfoConfirmFieldToRust(pSettlementInfoConfirm),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPositionCombineDetail(CThostFtdcInvestorPositionCombineDetailField* pInvestorPositionCombineDetail, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorPositionCombineDetail(
        Converter::CThostFtdcInvestorPositionCombineDetailFieldToRust(pInvestorPositionCombineDetail),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCFMMCTradingAccountKey(CThostFtdcCFMMCTradingAccountKeyField* pCFMMCTradingAccountKey, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryCFMMCTradingAccountKey(
        Converter::CThostFtdcCFMMCTradingAccountKeyFieldToRust(pCFMMCTradingAccountKey),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryEWarrantOffset(CThostFtdcEWarrantOffsetField* pEWarrantOffset, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryEWarrantOffset(
        Converter::CThostFtdcEWarrantOffsetFieldToRust(pEWarrantOffset),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProductGroupMargin(CThostFtdcInvestorProductGroupMarginField* pInvestorProductGroupMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorProductGroupMargin(
        Converter::CThostFtdcInvestorProductGroupMarginFieldToRust(pInvestorProductGroupMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchangeMarginRate(CThostFtdcExchangeMarginRateField* pExchangeMarginRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryExchangeMarginRate(
        Converter::CThostFtdcExchangeMarginRateFieldToRust(pExchangeMarginRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchangeMarginRateAdjust(CThostFtdcExchangeMarginRateAdjustField* pExchangeMarginRateAdjust, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryExchangeMarginRateAdjust(
        Converter::CThostFtdcExchangeMarginRateAdjustFieldToRust(pExchangeMarginRateAdjust),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExchangeRate(CThostFtdcExchangeRateField* pExchangeRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryExchangeRate(
        Converter::CThostFtdcExchangeRateFieldToRust(pExchangeRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentACIDMap(CThostFtdcSecAgentACIDMapField* pSecAgentACIDMap, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySecAgentACIDMap(
        Converter::CThostFtdcSecAgentACIDMapFieldToRust(pSecAgentACIDMap),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryProductExchRate(CThostFtdcProductExchRateField* pProductExchRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryProductExchRate(
        Converter::CThostFtdcProductExchRateFieldToRust(pProductExchRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryProductGroup(CThostFtdcProductGroupField* pProductGroup, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryProductGroup(
        Converter::CThostFtdcProductGroupFieldToRust(pProductGroup),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryMMInstrumentCommissionRate(CThostFtdcMMInstrumentCommissionRateField* pMMInstrumentCommissionRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryMMInstrumentCommissionRate(
        Converter::CThostFtdcMMInstrumentCommissionRateFieldToRust(pMMInstrumentCommissionRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryMMOptionInstrCommRate(CThostFtdcMMOptionInstrCommRateField* pMMOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryMMOptionInstrCommRate(
        Converter::CThostFtdcMMOptionInstrCommRateFieldToRust(pMMOptionInstrCommRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInstrumentOrderCommRate(CThostFtdcInstrumentOrderCommRateField* pInstrumentOrderCommRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInstrumentOrderCommRate(
        Converter::CThostFtdcInstrumentOrderCommRateFieldToRust(pInstrumentOrderCommRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentTradingAccount(CThostFtdcTradingAccountField* pTradingAccount, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySecAgentTradingAccount(
        Converter::CThostFtdcTradingAccountFieldToRust(pTradingAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentCheckMode(CThostFtdcSecAgentCheckModeField* pSecAgentCheckMode, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySecAgentCheckMode(
        Converter::CThostFtdcSecAgentCheckModeFieldToRust(pSecAgentCheckMode),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySecAgentTradeInfo(CThostFtdcSecAgentTradeInfoField* pSecAgentTradeInfo, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySecAgentTradeInfo(
        Converter::CThostFtdcSecAgentTradeInfoFieldToRust(pSecAgentTradeInfo),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOptionInstrTradeCost(CThostFtdcOptionInstrTradeCostField* pOptionInstrTradeCost, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryOptionInstrTradeCost(
        Converter::CThostFtdcOptionInstrTradeCostFieldToRust(pOptionInstrTradeCost),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOptionInstrCommRate(CThostFtdcOptionInstrCommRateField* pOptionInstrCommRate, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryOptionInstrCommRate(
        Converter::CThostFtdcOptionInstrCommRateFieldToRust(pOptionInstrCommRate),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryExecOrder(CThostFtdcExecOrderField* pExecOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryExecOrder(
        Converter::CThostFtdcExecOrderFieldToRust(pExecOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryForQuote(CThostFtdcForQuoteField* pForQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryForQuote(
        Converter::CThostFtdcForQuoteFieldToRust(pForQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryQuote(CThostFtdcQuoteField* pQuote, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryQuote(
        Converter::CThostFtdcQuoteFieldToRust(pQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryOptionSelfClose(
        Converter::CThostFtdcOptionSelfCloseFieldToRust(pOptionSelfClose),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestUnit(CThostFtdcInvestUnitField* pInvestUnit, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestUnit(
        Converter::CThostFtdcInvestUnitFieldToRust(pInvestUnit),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombInstrumentGuard(CThostFtdcCombInstrumentGuardField* pCombInstrumentGuard, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryCombInstrumentGuard(
        Converter::CThostFtdcCombInstrumentGuardFieldToRust(pCombInstrumentGuard),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombAction(CThostFtdcCombActionField* pCombAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryCombAction(
        Converter::CThostFtdcCombActionFieldToRust(pCombAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTransferSerial(CThostFtdcTransferSerialField* pTransferSerial, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTransferSerial(
        Converter::CThostFtdcTransferSerialFieldToRust(pTransferSerial),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryAccountregister(CThostFtdcAccountregisterField* pAccountregister, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryAccountregister(
        Converter::CThostFtdcAccountregisterFieldToRust(pAccountregister),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspError(CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspError(
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnOrder(CThostFtdcOrderField* pOrder) {
    this->gateway->gateway.OnRtnOrder(
        Converter::CThostFtdcOrderFieldToRust(pOrder)
    );
}

void CTraderSpi::OnRtnTrade(CThostFtdcTradeField* pTrade) {
    this->gateway->gateway.OnRtnTrade(
        Converter::CThostFtdcTradeFieldToRust(pTrade)
    );
}

void CTraderSpi::OnErrRtnOrderInsert(CThostFtdcInputOrderField* pInputOrder, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnOrderInsert(
        Converter::CThostFtdcInputOrderFieldToRust(pInputOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnOrderAction(CThostFtdcOrderActionField* pOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnOrderAction(
        Converter::CThostFtdcOrderActionFieldToRust(pOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnInstrumentStatus(CThostFtdcInstrumentStatusField* pInstrumentStatus) {
    this->gateway->gateway.OnRtnInstrumentStatus(
        Converter::CThostFtdcInstrumentStatusFieldToRust(pInstrumentStatus)
    );
}

void CTraderSpi::OnRtnBulletin(CThostFtdcBulletinField* pBulletin) {
    this->gateway->gateway.OnRtnBulletin(
        Converter::CThostFtdcBulletinFieldToRust(pBulletin)
    );
}

void CTraderSpi::OnRtnTradingNotice(CThostFtdcTradingNoticeInfoField* pTradingNoticeInfo) {
    this->gateway->gateway.OnRtnTradingNotice(
        Converter::CThostFtdcTradingNoticeInfoFieldToRust(pTradingNoticeInfo)
    );
}

void CTraderSpi::OnRtnErrorConditionalOrder(CThostFtdcErrorConditionalOrderField* pErrorConditionalOrder) {
    this->gateway->gateway.OnRtnErrorConditionalOrder(
        Converter::CThostFtdcErrorConditionalOrderFieldToRust(pErrorConditionalOrder)
    );
}

void CTraderSpi::OnRtnExecOrder(CThostFtdcExecOrderField* pExecOrder) {
    this->gateway->gateway.OnRtnExecOrder(
        Converter::CThostFtdcExecOrderFieldToRust(pExecOrder)
    );
}

void CTraderSpi::OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnExecOrderInsert(
        Converter::CThostFtdcInputExecOrderFieldToRust(pInputExecOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnExecOrderAction(CThostFtdcExecOrderActionField* pExecOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnExecOrderAction(
        Converter::CThostFtdcExecOrderActionFieldToRust(pExecOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnForQuoteInsert(
        Converter::CThostFtdcInputForQuoteFieldToRust(pInputForQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnQuote(CThostFtdcQuoteField* pQuote) {
    this->gateway->gateway.OnRtnQuote(
        Converter::CThostFtdcQuoteFieldToRust(pQuote)
    );
}

void CTraderSpi::OnErrRtnQuoteInsert(CThostFtdcInputQuoteField* pInputQuote, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnQuoteInsert(
        Converter::CThostFtdcInputQuoteFieldToRust(pInputQuote),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnQuoteAction(CThostFtdcQuoteActionField* pQuoteAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnQuoteAction(
        Converter::CThostFtdcQuoteActionFieldToRust(pQuoteAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp) {
    this->gateway->gateway.OnRtnForQuoteRsp(
        Converter::CThostFtdcForQuoteRspFieldToRust(pForQuoteRsp)
    );
}

void CTraderSpi::OnRtnCFMMCTradingAccountToken(CThostFtdcCFMMCTradingAccountTokenField* pCFMMCTradingAccountToken) {
    this->gateway->gateway.OnRtnCFMMCTradingAccountToken(
        Converter::CThostFtdcCFMMCTradingAccountTokenFieldToRust(pCFMMCTradingAccountToken)
    );
}

void CTraderSpi::OnErrRtnBatchOrderAction(CThostFtdcBatchOrderActionField* pBatchOrderAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnBatchOrderAction(
        Converter::CThostFtdcBatchOrderActionFieldToRust(pBatchOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose) {
    this->gateway->gateway.OnRtnOptionSelfClose(
        Converter::CThostFtdcOptionSelfCloseFieldToRust(pOptionSelfClose)
    );
}

void CTraderSpi::OnErrRtnOptionSelfCloseInsert(CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnOptionSelfCloseInsert(
        Converter::CThostFtdcInputOptionSelfCloseFieldToRust(pInputOptionSelfClose),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnOptionSelfCloseAction(CThostFtdcOptionSelfCloseActionField* pOptionSelfCloseAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnOptionSelfCloseAction(
        Converter::CThostFtdcOptionSelfCloseActionFieldToRust(pOptionSelfCloseAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnCombAction(CThostFtdcCombActionField* pCombAction) {
    this->gateway->gateway.OnRtnCombAction(
        Converter::CThostFtdcCombActionFieldToRust(pCombAction)
    );
}

void CTraderSpi::OnErrRtnCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnCombActionInsert(
        Converter::CThostFtdcInputCombActionFieldToRust(pInputCombAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRspQryContractBank(CThostFtdcContractBankField* pContractBank, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryContractBank(
        Converter::CThostFtdcContractBankFieldToRust(pContractBank),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryParkedOrder(CThostFtdcParkedOrderField* pParkedOrder, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryParkedOrder(
        Converter::CThostFtdcParkedOrderFieldToRust(pParkedOrder),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryParkedOrderAction(CThostFtdcParkedOrderActionField* pParkedOrderAction, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryParkedOrderAction(
        Converter::CThostFtdcParkedOrderActionFieldToRust(pParkedOrderAction),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryTradingNotice(CThostFtdcTradingNoticeField* pTradingNotice, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryTradingNotice(
        Converter::CThostFtdcTradingNoticeFieldToRust(pTradingNotice),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryBrokerTradingParams(CThostFtdcBrokerTradingParamsField* pBrokerTradingParams, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryBrokerTradingParams(
        Converter::CThostFtdcBrokerTradingParamsFieldToRust(pBrokerTradingParams),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryBrokerTradingAlgos(CThostFtdcBrokerTradingAlgosField* pBrokerTradingAlgos, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryBrokerTradingAlgos(
        Converter::CThostFtdcBrokerTradingAlgosFieldToRust(pBrokerTradingAlgos),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQueryCFMMCTradingAccountToken(CThostFtdcQueryCFMMCTradingAccountTokenField* pQueryCFMMCTradingAccountToken, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQueryCFMMCTradingAccountToken(
        Converter::CThostFtdcQueryCFMMCTradingAccountTokenFieldToRust(pQueryCFMMCTradingAccountToken),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->gateway.OnRtnFromBankToFutureByBank(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->gateway.OnRtnFromFutureToBankByBank(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnRepealFromBankToFutureByBank(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->gateway.OnRtnRepealFromBankToFutureByBank(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnRepealFromFutureToBankByBank(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->gateway.OnRtnRepealFromFutureToBankByBank(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnFromBankToFutureByFuture(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->gateway.OnRtnFromBankToFutureByFuture(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnFromFutureToBankByFuture(CThostFtdcRspTransferField* pRspTransfer) {
    this->gateway->gateway.OnRtnFromFutureToBankByFuture(
        Converter::CThostFtdcRspTransferFieldToRust(pRspTransfer)
    );
}

void CTraderSpi::OnRtnRepealFromBankToFutureByFutureManual(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->gateway.OnRtnRepealFromBankToFutureByFutureManual(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnRepealFromFutureToBankByFutureManual(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->gateway.OnRtnRepealFromFutureToBankByFutureManual(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnQueryBankBalanceByFuture(CThostFtdcNotifyQueryAccountField* pNotifyQueryAccount) {
    this->gateway->gateway.OnRtnQueryBankBalanceByFuture(
        Converter::CThostFtdcNotifyQueryAccountFieldToRust(pNotifyQueryAccount)
    );
}

void CTraderSpi::OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnBankToFutureByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnFutureToBankByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnRepealBankToFutureByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnRepealBankToFutureByFutureManual(
        Converter::CThostFtdcReqRepealFieldToRust(pReqRepeal),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnRepealFutureToBankByFutureManual(CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnRepealFutureToBankByFutureManual(
        Converter::CThostFtdcReqRepealFieldToRust(pReqRepeal),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnQueryBankBalanceByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnQueryBankBalanceByFuture(
        Converter::CThostFtdcReqQueryAccountFieldToRust(pReqQueryAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRtnRepealFromBankToFutureByFuture(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->gateway.OnRtnRepealFromBankToFutureByFuture(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRtnRepealFromFutureToBankByFuture(CThostFtdcRspRepealField* pRspRepeal) {
    this->gateway->gateway.OnRtnRepealFromFutureToBankByFuture(
        Converter::CThostFtdcRspRepealFieldToRust(pRspRepeal)
    );
}

void CTraderSpi::OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspFromBankToFutureByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspFromFutureToBankByFuture(
        Converter::CThostFtdcReqTransferFieldToRust(pReqTransfer),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQueryBankAccountMoneyByFuture(CThostFtdcReqQueryAccountField* pReqQueryAccount, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQueryBankAccountMoneyByFuture(
        Converter::CThostFtdcReqQueryAccountFieldToRust(pReqQueryAccount),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnOpenAccountByBank(CThostFtdcOpenAccountField* pOpenAccount) {
    this->gateway->gateway.OnRtnOpenAccountByBank(
        Converter::CThostFtdcOpenAccountFieldToRust(pOpenAccount)
    );
}

void CTraderSpi::OnRtnCancelAccountByBank(CThostFtdcCancelAccountField* pCancelAccount) {
    this->gateway->gateway.OnRtnCancelAccountByBank(
        Converter::CThostFtdcCancelAccountFieldToRust(pCancelAccount)
    );
}

void CTraderSpi::OnRtnChangeAccountByBank(CThostFtdcChangeAccountField* pChangeAccount) {
    this->gateway->gateway.OnRtnChangeAccountByBank(
        Converter::CThostFtdcChangeAccountFieldToRust(pChangeAccount)
    );
}

void CTraderSpi::OnRspQryClassifiedInstrument(CThostFtdcInstrumentField* pInstrument, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryClassifiedInstrument(
        Converter::CThostFtdcInstrumentFieldToRust(pInstrument),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombPromotionParam(CThostFtdcCombPromotionParamField* pCombPromotionParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryCombPromotionParam(
        Converter::CThostFtdcCombPromotionParamFieldToRust(pCombPromotionParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRiskSettleInvstPosition(CThostFtdcRiskSettleInvstPositionField* pRiskSettleInvstPosition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRiskSettleInvstPosition(
        Converter::CThostFtdcRiskSettleInvstPositionFieldToRust(pRiskSettleInvstPosition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRiskSettleProductStatus(CThostFtdcRiskSettleProductStatusField* pRiskSettleProductStatus, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRiskSettleProductStatus(
        Converter::CThostFtdcRiskSettleProductStatusFieldToRust(pRiskSettleProductStatus),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMFutureParameter(CThostFtdcSPBMFutureParameterField* pSPBMFutureParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMFutureParameter(
        Converter::CThostFtdcSPBMFutureParameterFieldToRust(pSPBMFutureParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMOptionParameter(CThostFtdcSPBMOptionParameterField* pSPBMOptionParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMOptionParameter(
        Converter::CThostFtdcSPBMOptionParameterFieldToRust(pSPBMOptionParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMIntraParameter(CThostFtdcSPBMIntraParameterField* pSPBMIntraParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMIntraParameter(
        Converter::CThostFtdcSPBMIntraParameterFieldToRust(pSPBMIntraParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMInterParameter(CThostFtdcSPBMInterParameterField* pSPBMInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMInterParameter(
        Converter::CThostFtdcSPBMInterParameterFieldToRust(pSPBMInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMPortfDefinition(CThostFtdcSPBMPortfDefinitionField* pSPBMPortfDefinition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMPortfDefinition(
        Converter::CThostFtdcSPBMPortfDefinitionFieldToRust(pSPBMPortfDefinition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMInvestorPortfDef(CThostFtdcSPBMInvestorPortfDefField* pSPBMInvestorPortfDef, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMInvestorPortfDef(
        Converter::CThostFtdcSPBMInvestorPortfDefFieldToRust(pSPBMInvestorPortfDef),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPortfMarginRatio(CThostFtdcInvestorPortfMarginRatioField* pInvestorPortfMarginRatio, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorPortfMarginRatio(
        Converter::CThostFtdcInvestorPortfMarginRatioFieldToRust(pInvestorPortfMarginRatio),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProdSPBMDetail(CThostFtdcInvestorProdSPBMDetailField* pInvestorProdSPBMDetail, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorProdSPBMDetail(
        Converter::CThostFtdcInvestorProdSPBMDetailFieldToRust(pInvestorProdSPBMDetail),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorCommoditySPMMMargin(CThostFtdcInvestorCommoditySPMMMarginField* pInvestorCommoditySPMMMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorCommoditySPMMMargin(
        Converter::CThostFtdcInvestorCommoditySPMMMarginFieldToRust(pInvestorCommoditySPMMMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorCommodityGroupSPMMMargin(CThostFtdcInvestorCommodityGroupSPMMMarginField* pInvestorCommodityGroupSPMMMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorCommodityGroupSPMMMargin(
        Converter::CThostFtdcInvestorCommodityGroupSPMMMarginFieldToRust(pInvestorCommodityGroupSPMMMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPMMInstParam(CThostFtdcSPMMInstParamField* pSPMMInstParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPMMInstParam(
        Converter::CThostFtdcSPMMInstParamFieldToRust(pSPMMInstParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPMMProductParam(CThostFtdcSPMMProductParamField* pSPMMProductParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPMMProductParam(
        Converter::CThostFtdcSPMMProductParamFieldToRust(pSPMMProductParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQrySPBMAddOnInterParameter(CThostFtdcSPBMAddOnInterParameterField* pSPBMAddOnInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQrySPBMAddOnInterParameter(
        Converter::CThostFtdcSPBMAddOnInterParameterFieldToRust(pSPBMAddOnInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSCombProductInfo(CThostFtdcRCAMSCombProductInfoField* pRCAMSCombProductInfo, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRCAMSCombProductInfo(
        Converter::CThostFtdcRCAMSCombProductInfoFieldToRust(pRCAMSCombProductInfo),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSInstrParameter(CThostFtdcRCAMSInstrParameterField* pRCAMSInstrParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRCAMSInstrParameter(
        Converter::CThostFtdcRCAMSInstrParameterFieldToRust(pRCAMSInstrParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSIntraParameter(CThostFtdcRCAMSIntraParameterField* pRCAMSIntraParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRCAMSIntraParameter(
        Converter::CThostFtdcRCAMSIntraParameterFieldToRust(pRCAMSIntraParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSInterParameter(CThostFtdcRCAMSInterParameterField* pRCAMSInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRCAMSInterParameter(
        Converter::CThostFtdcRCAMSInterParameterFieldToRust(pRCAMSInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSShortOptAdjustParam(CThostFtdcRCAMSShortOptAdjustParamField* pRCAMSShortOptAdjustParam, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRCAMSShortOptAdjustParam(
        Converter::CThostFtdcRCAMSShortOptAdjustParamFieldToRust(pRCAMSShortOptAdjustParam),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRCAMSInvestorCombPosition(CThostFtdcRCAMSInvestorCombPositionField* pRCAMSInvestorCombPosition, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRCAMSInvestorCombPosition(
        Converter::CThostFtdcRCAMSInvestorCombPositionFieldToRust(pRCAMSInvestorCombPosition),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProdRCAMSMargin(CThostFtdcInvestorProdRCAMSMarginField* pInvestorProdRCAMSMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorProdRCAMSMargin(
        Converter::CThostFtdcInvestorProdRCAMSMarginFieldToRust(pInvestorProdRCAMSMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRULEInstrParameter(CThostFtdcRULEInstrParameterField* pRULEInstrParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRULEInstrParameter(
        Converter::CThostFtdcRULEInstrParameterFieldToRust(pRULEInstrParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRULEIntraParameter(CThostFtdcRULEIntraParameterField* pRULEIntraParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRULEIntraParameter(
        Converter::CThostFtdcRULEIntraParameterFieldToRust(pRULEIntraParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryRULEInterParameter(CThostFtdcRULEInterParameterField* pRULEInterParameter, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryRULEInterParameter(
        Converter::CThostFtdcRULEInterParameterFieldToRust(pRULEInterParameter),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorProdRULEMargin(CThostFtdcInvestorProdRULEMarginField* pInvestorProdRULEMargin, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorProdRULEMargin(
        Converter::CThostFtdcInvestorProdRULEMarginFieldToRust(pInvestorProdRULEMargin),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorPortfSetting(CThostFtdcInvestorPortfSettingField* pInvestorPortfSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorPortfSetting(
        Converter::CThostFtdcInvestorPortfSettingFieldToRust(pInvestorPortfSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryInvestorInfoCommRec(CThostFtdcInvestorInfoCommRecField* pInvestorInfoCommRec, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryInvestorInfoCommRec(
        Converter::CThostFtdcInvestorInfoCommRecFieldToRust(pInvestorInfoCommRec),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspQryCombLeg(CThostFtdcCombLegField* pCombLeg, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryCombLeg(
        Converter::CThostFtdcCombLegFieldToRust(pCombLeg),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspOffsetSetting(
        Converter::CThostFtdcInputOffsetSettingFieldToRust(pInputOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRspCancelOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspCancelOffsetSetting(
        Converter::CThostFtdcInputOffsetSettingFieldToRust(pInputOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}

void CTraderSpi::OnRtnOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting) {
    this->gateway->gateway.OnRtnOffsetSetting(
        Converter::CThostFtdcOffsetSettingFieldToRust(pOffsetSetting)
    );
}

void CTraderSpi::OnErrRtnOffsetSetting(CThostFtdcInputOffsetSettingField* pInputOffsetSetting, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnOffsetSetting(
        Converter::CThostFtdcInputOffsetSettingFieldToRust(pInputOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnErrRtnCancelOffsetSetting(CThostFtdcCancelOffsetSettingField* pCancelOffsetSetting, CThostFtdcRspInfoField* pRspInfo) {
    this->gateway->gateway.OnErrRtnCancelOffsetSetting(
        Converter::CThostFtdcCancelOffsetSettingFieldToRust(pCancelOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo)
    );
}

void CTraderSpi::OnRspQryOffsetSetting(CThostFtdcOffsetSettingField* pOffsetSetting, CThostFtdcRspInfoField* pRspInfo, int32_t nRequestID, bool bIsLast) {
    this->gateway->gateway.OnRspQryOffsetSetting(
        Converter::CThostFtdcOffsetSettingFieldToRust(pOffsetSetting),
        Converter::CThostFtdcRspInfoFieldToRust(pRspInfo),
        nRequestID,
        bIsLast
    );
}
