#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use cxx::UniquePtr;
pub use ffi::*;
use std::fs::create_dir_all;
use std::mem::forget;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub enum THOST_TE_RESUME_TYPE {
    THOST_TERT_RESTART = 0,
    THOST_TERT_RESUME,
    THOST_TERT_QUICK,
    THOST_TERT_NONE,
}

pub const THOST_FTDC_EXP_Normal: u8 = '0' as u8;
pub const THOST_FTDC_EXP_GenOrderByTrade: u8 = '1' as u8;
pub const THOST_FTDC_ICT_EID: u8 = '0' as u8;
pub const THOST_FTDC_ICT_IDCard: u8 = '1' as u8;
pub const THOST_FTDC_ICT_OfficerIDCard: u8 = '2' as u8;
pub const THOST_FTDC_ICT_PoliceIDCard: u8 = '3' as u8;
pub const THOST_FTDC_ICT_SoldierIDCard: u8 = '4' as u8;
pub const THOST_FTDC_ICT_HouseholdRegister: u8 = '5' as u8;
pub const THOST_FTDC_ICT_Passport: u8 = '6' as u8;
pub const THOST_FTDC_ICT_TaiwanCompatriotIDCard: u8 = '7' as u8;
pub const THOST_FTDC_ICT_HomeComingCard: u8 = '8' as u8;
pub const THOST_FTDC_ICT_LicenseNo: u8 = '9' as u8;
pub const THOST_FTDC_ICT_TaxNo: u8 = 'A' as u8;
pub const THOST_FTDC_ICT_HMMainlandTravelPermit: u8 = 'B' as u8;
pub const THOST_FTDC_ICT_TwMainlandTravelPermit: u8 = 'C' as u8;
pub const THOST_FTDC_ICT_DrivingLicense: u8 = 'D' as u8;
pub const THOST_FTDC_ICT_SocialID: u8 = 'F' as u8;
pub const THOST_FTDC_ICT_LocalID: u8 = 'G' as u8;
pub const THOST_FTDC_ICT_BusinessRegistration: u8 = 'H' as u8;
pub const THOST_FTDC_ICT_HKMCIDCard: u8 = 'I' as u8;
pub const THOST_FTDC_ICT_AccountsPermits: u8 = 'J' as u8;
pub const THOST_FTDC_ICT_FrgPrmtRdCard: u8 = 'K' as u8;
pub const THOST_FTDC_ICT_CptMngPrdLetter: u8 = 'L' as u8;
pub const THOST_FTDC_ICT_HKMCTwResidencePermit: u8 = 'M' as u8;
pub const THOST_FTDC_ICT_UniformSocialCreditCode: u8 = 'N' as u8;
pub const THOST_FTDC_ICT_CorporationCertNo: u8 = 'O' as u8;
pub const THOST_FTDC_ICT_OtherCard: u8 = 'x' as u8;
pub const THOST_FTDC_IR_All: u8 = '1' as u8;
pub const THOST_FTDC_IR_Group: u8 = '2' as u8;
pub const THOST_FTDC_IR_Single: u8 = '3' as u8;
pub const THOST_FTDC_DR_All: u8 = '1' as u8;
pub const THOST_FTDC_DR_Group: u8 = '2' as u8;
pub const THOST_FTDC_DR_Single: u8 = '3' as u8;
pub const THOST_FTDC_DS_Asynchronous: u8 = '1' as u8;
pub const THOST_FTDC_DS_Synchronizing: u8 = '2' as u8;
pub const THOST_FTDC_DS_Synchronized: u8 = '3' as u8;
pub const THOST_FTDC_BDS_Synchronized: u8 = '1' as u8;
pub const THOST_FTDC_BDS_Synchronizing: u8 = '2' as u8;
pub const THOST_FTDC_ECS_NoConnection: u8 = '1' as u8;
pub const THOST_FTDC_ECS_QryInstrumentSent: u8 = '2' as u8;
pub const THOST_FTDC_ECS_GotInformation: u8 = '9' as u8;
pub const THOST_FTDC_TCS_NotConnected: u8 = '1' as u8;
pub const THOST_FTDC_TCS_Connected: u8 = '2' as u8;
pub const THOST_FTDC_TCS_QryInstrumentSent: u8 = '3' as u8;
pub const THOST_FTDC_TCS_SubPrivateFlow: u8 = '4' as u8;
pub const THOST_FTDC_FC_DataAsync: u8 = '1' as u8;
pub const THOST_FTDC_FC_ForceUserLogout: u8 = '2' as u8;
pub const THOST_FTDC_FC_UserPasswordUpdate: u8 = '3' as u8;
pub const THOST_FTDC_FC_BrokerPasswordUpdate: u8 = '4' as u8;
pub const THOST_FTDC_FC_InvestorPasswordUpdate: u8 = '5' as u8;
pub const THOST_FTDC_FC_OrderInsert: u8 = '6' as u8;
pub const THOST_FTDC_FC_OrderAction: u8 = '7' as u8;
pub const THOST_FTDC_FC_SyncSystemData: u8 = '8' as u8;
pub const THOST_FTDC_FC_SyncBrokerData: u8 = '9' as u8;
pub const THOST_FTDC_FC_BachSyncBrokerData: u8 = 'A' as u8;
pub const THOST_FTDC_FC_SuperQuery: u8 = 'B' as u8;
pub const THOST_FTDC_FC_ParkedOrderInsert: u8 = 'C' as u8;
pub const THOST_FTDC_FC_ParkedOrderAction: u8 = 'D' as u8;
pub const THOST_FTDC_FC_SyncOTP: u8 = 'E' as u8;
pub const THOST_FTDC_FC_DeleteOrder: u8 = 'F' as u8;
pub const THOST_FTDC_FC_ExitEmergency: u8 = 'G' as u8;
pub const THOST_FTDC_BFC_ForceUserLogout: u8 = '1' as u8;
pub const THOST_FTDC_BFC_UserPasswordUpdate: u8 = '2' as u8;
pub const THOST_FTDC_BFC_SyncBrokerData: u8 = '3' as u8;
pub const THOST_FTDC_BFC_BachSyncBrokerData: u8 = '4' as u8;
pub const THOST_FTDC_BFC_OrderInsert: u8 = '5' as u8;
pub const THOST_FTDC_BFC_OrderAction: u8 = '6' as u8;
pub const THOST_FTDC_BFC_AllQuery: u8 = '7' as u8;
pub const THOST_FTDC_BFC_log: u8 = 'a' as u8;
pub const THOST_FTDC_BFC_BaseQry: u8 = 'b' as u8;
pub const THOST_FTDC_BFC_TradeQry: u8 = 'c' as u8;
pub const THOST_FTDC_BFC_Trade: u8 = 'd' as u8;
pub const THOST_FTDC_BFC_Virement: u8 = 'e' as u8;
pub const THOST_FTDC_BFC_Risk: u8 = 'f' as u8;
pub const THOST_FTDC_BFC_Session: u8 = 'g' as u8;
pub const THOST_FTDC_BFC_RiskNoticeCtl: u8 = 'h' as u8;
pub const THOST_FTDC_BFC_RiskNotice: u8 = 'i' as u8;
pub const THOST_FTDC_BFC_BrokerDeposit: u8 = 'j' as u8;
pub const THOST_FTDC_BFC_QueryFund: u8 = 'k' as u8;
pub const THOST_FTDC_BFC_QueryOrder: u8 = 'l' as u8;
pub const THOST_FTDC_BFC_QueryTrade: u8 = 'm' as u8;
pub const THOST_FTDC_BFC_QueryPosition: u8 = 'n' as u8;
pub const THOST_FTDC_BFC_QueryMarketData: u8 = 'o' as u8;
pub const THOST_FTDC_BFC_QueryUserEvent: u8 = 'p' as u8;
pub const THOST_FTDC_BFC_QueryRiskNotify: u8 = 'q' as u8;
pub const THOST_FTDC_BFC_QueryFundChange: u8 = 'r' as u8;
pub const THOST_FTDC_BFC_QueryInvestor: u8 = 's' as u8;
pub const THOST_FTDC_BFC_QueryTradingCode: u8 = 't' as u8;
pub const THOST_FTDC_BFC_ForceClose: u8 = 'u' as u8;
pub const THOST_FTDC_BFC_PressTest: u8 = 'v' as u8;
pub const THOST_FTDC_BFC_RemainCalc: u8 = 'w' as u8;
pub const THOST_FTDC_BFC_NetPositionInd: u8 = 'x' as u8;
pub const THOST_FTDC_BFC_RiskPredict: u8 = 'y' as u8;
pub const THOST_FTDC_BFC_DataExport: u8 = 'z' as u8;
pub const THOST_FTDC_BFC_RiskTargetSetup: u8 = 'A' as u8;
pub const THOST_FTDC_BFC_MarketDataWarn: u8 = 'B' as u8;
pub const THOST_FTDC_BFC_QryBizNotice: u8 = 'C' as u8;
pub const THOST_FTDC_BFC_CfgBizNotice: u8 = 'D' as u8;
pub const THOST_FTDC_BFC_SyncOTP: u8 = 'E' as u8;
pub const THOST_FTDC_BFC_SendBizNotice: u8 = 'F' as u8;
pub const THOST_FTDC_BFC_CfgRiskLevelStd: u8 = 'G' as u8;
pub const THOST_FTDC_BFC_TbCommand: u8 = 'H' as u8;
pub const THOST_FTDC_BFC_DeleteOrder: u8 = 'J' as u8;
pub const THOST_FTDC_BFC_ParkedOrderInsert: u8 = 'K' as u8;
pub const THOST_FTDC_BFC_ParkedOrderAction: u8 = 'L' as u8;
pub const THOST_FTDC_BFC_ExecOrderNoCheck: u8 = 'M' as u8;
pub const THOST_FTDC_BFC_Designate: u8 = 'N' as u8;
pub const THOST_FTDC_BFC_StockDisposal: u8 = 'O' as u8;
pub const THOST_FTDC_BFC_BrokerDepositWarn: u8 = 'Q' as u8;
pub const THOST_FTDC_BFC_CoverWarn: u8 = 'S' as u8;
pub const THOST_FTDC_BFC_PreExecOrder: u8 = 'T' as u8;
pub const THOST_FTDC_BFC_ExecOrderRisk: u8 = 'P' as u8;
pub const THOST_FTDC_BFC_PosiLimitWarn: u8 = 'U' as u8;
pub const THOST_FTDC_BFC_QryPosiLimit: u8 = 'V' as u8;
pub const THOST_FTDC_BFC_FBSign: u8 = 'W' as u8;
pub const THOST_FTDC_BFC_FBAccount: u8 = 'X' as u8;
pub const THOST_FTDC_OAS_Submitted: u8 = 'a' as u8;
pub const THOST_FTDC_OAS_Accepted: u8 = 'b' as u8;
pub const THOST_FTDC_OAS_Rejected: u8 = 'c' as u8;
pub const THOST_FTDC_OST_AllTraded: u8 = '0' as u8;
pub const THOST_FTDC_OST_PartTradedQueueing: u8 = '1' as u8;
pub const THOST_FTDC_OST_PartTradedNotQueueing: u8 = '2' as u8;
pub const THOST_FTDC_OST_NoTradeQueueing: u8 = '3' as u8;
pub const THOST_FTDC_OST_NoTradeNotQueueing: u8 = '4' as u8;
pub const THOST_FTDC_OST_Canceled: u8 = '5' as u8;
pub const THOST_FTDC_OST_Unknown: u8 = 'a' as u8;
pub const THOST_FTDC_OST_NotTouched: u8 = 'b' as u8;
pub const THOST_FTDC_OST_Touched: u8 = 'c' as u8;
pub const THOST_FTDC_OSS_InsertSubmitted: u8 = '0' as u8;
pub const THOST_FTDC_OSS_CancelSubmitted: u8 = '1' as u8;
pub const THOST_FTDC_OSS_ModifySubmitted: u8 = '2' as u8;
pub const THOST_FTDC_OSS_Accepted: u8 = '3' as u8;
pub const THOST_FTDC_OSS_InsertRejected: u8 = '4' as u8;
pub const THOST_FTDC_OSS_CancelRejected: u8 = '5' as u8;
pub const THOST_FTDC_OSS_ModifyRejected: u8 = '6' as u8;
pub const THOST_FTDC_PSD_Today: u8 = '1' as u8;
pub const THOST_FTDC_PSD_History: u8 = '2' as u8;
pub const THOST_FTDC_PDT_UseHistory: u8 = '1' as u8;
pub const THOST_FTDC_PDT_NoUseHistory: u8 = '2' as u8;
pub const THOST_FTDC_ER_Broker: u8 = '1' as u8;
pub const THOST_FTDC_ER_Host: u8 = '2' as u8;
pub const THOST_FTDC_ER_Maker: u8 = '3' as u8;
pub const THOST_FTDC_PC_Futures: u8 = '1' as u8;
pub const THOST_FTDC_PC_Options: u8 = '2' as u8;
pub const THOST_FTDC_PC_Combination: u8 = '3' as u8;
pub const THOST_FTDC_PC_Spot: u8 = '4' as u8;
pub const THOST_FTDC_PC_EFP: u8 = '5' as u8;
pub const THOST_FTDC_PC_SpotOption: u8 = '6' as u8;
pub const THOST_FTDC_PC_TAS: u8 = '7' as u8;
pub const THOST_FTDC_PC_MI: u8 = 'I' as u8;
pub const THOST_FTDC_APC_FutureSingle: u8 = '1' as u8;
pub const THOST_FTDC_APC_OptionSingle: u8 = '2' as u8;
pub const THOST_FTDC_APC_Futures: u8 = '3' as u8;
pub const THOST_FTDC_APC_Options: u8 = '4' as u8;
pub const THOST_FTDC_APC_TradingComb: u8 = '5' as u8;
pub const THOST_FTDC_APC_UnTradingComb: u8 = '6' as u8;
pub const THOST_FTDC_APC_AllTrading: u8 = '7' as u8;
pub const THOST_FTDC_APC_All: u8 = '8' as u8;
pub const THOST_FTDC_IP_NotStart: u8 = '0' as u8;
pub const THOST_FTDC_IP_Started: u8 = '1' as u8;
pub const THOST_FTDC_IP_Pause: u8 = '2' as u8;
pub const THOST_FTDC_IP_Expired: u8 = '3' as u8;
pub const THOST_FTDC_D_Buy: u8 = '0' as u8;
pub const THOST_FTDC_D_Sell: u8 = '1' as u8;
pub const THOST_FTDC_PT_Net: u8 = '1' as u8;
pub const THOST_FTDC_PT_Gross: u8 = '2' as u8;
pub const THOST_FTDC_PD_Net: u8 = '1' as u8;
pub const THOST_FTDC_PD_Long: u8 = '2' as u8;
pub const THOST_FTDC_PD_Short: u8 = '3' as u8;
pub const THOST_FTDC_SS_NonActive: u8 = '1' as u8;
pub const THOST_FTDC_SS_Startup: u8 = '2' as u8;
pub const THOST_FTDC_SS_Operating: u8 = '3' as u8;
pub const THOST_FTDC_SS_Settlement: u8 = '4' as u8;
pub const THOST_FTDC_SS_SettlementFinished: u8 = '5' as u8;
pub const THOST_FTDC_RA_Trade: u8 = '0' as u8;
pub const THOST_FTDC_RA_Settlement: u8 = '1' as u8;
pub const THOST_FTDC_HF_Speculation: u8 = '1' as u8;
pub const THOST_FTDC_HF_Arbitrage: u8 = '2' as u8;
pub const THOST_FTDC_HF_Hedge: u8 = '3' as u8;
pub const THOST_FTDC_HF_MarketMaker: u8 = '5' as u8;
pub const THOST_FTDC_HF_SpecHedge: u8 = '6' as u8;
pub const THOST_FTDC_HF_HedgeSpec: u8 = '7' as u8;
pub const THOST_FTDC_BHF_Speculation: u8 = '1' as u8;
pub const THOST_FTDC_BHF_Arbitrage: u8 = '2' as u8;
pub const THOST_FTDC_BHF_Hedge: u8 = '3' as u8;
pub const THOST_FTDC_CIDT_Speculation: u8 = '1' as u8;
pub const THOST_FTDC_CIDT_Arbitrage: u8 = '2' as u8;
pub const THOST_FTDC_CIDT_Hedge: u8 = '3' as u8;
pub const THOST_FTDC_CIDT_MarketMaker: u8 = '5' as u8;
pub const THOST_FTDC_OPT_AnyPrice: u8 = '1' as u8;
pub const THOST_FTDC_OPT_LimitPrice: u8 = '2' as u8;
pub const THOST_FTDC_OPT_BestPrice: u8 = '3' as u8;
pub const THOST_FTDC_OPT_LastPrice: u8 = '4' as u8;
pub const THOST_FTDC_OPT_LastPricePlusOneTicks: u8 = '5' as u8;
pub const THOST_FTDC_OPT_LastPricePlusTwoTicks: u8 = '6' as u8;
pub const THOST_FTDC_OPT_LastPricePlusThreeTicks: u8 = '7' as u8;
pub const THOST_FTDC_OPT_AskPrice1: u8 = '8' as u8;
pub const THOST_FTDC_OPT_AskPrice1PlusOneTicks: u8 = '9' as u8;
pub const THOST_FTDC_OPT_AskPrice1PlusTwoTicks: u8 = 'A' as u8;
pub const THOST_FTDC_OPT_AskPrice1PlusThreeTicks: u8 = 'B' as u8;
pub const THOST_FTDC_OPT_BidPrice1: u8 = 'C' as u8;
pub const THOST_FTDC_OPT_BidPrice1PlusOneTicks: u8 = 'D' as u8;
pub const THOST_FTDC_OPT_BidPrice1PlusTwoTicks: u8 = 'E' as u8;
pub const THOST_FTDC_OPT_BidPrice1PlusThreeTicks: u8 = 'F' as u8;
pub const THOST_FTDC_OPT_FiveLevelPrice: u8 = 'G' as u8;
pub const THOST_FTDC_OF_Open: u8 = '0' as u8;
pub const THOST_FTDC_OF_Close: u8 = '1' as u8;
pub const THOST_FTDC_OF_ForceClose: u8 = '2' as u8;
pub const THOST_FTDC_OF_CloseToday: u8 = '3' as u8;
pub const THOST_FTDC_OF_CloseYesterday: u8 = '4' as u8;
pub const THOST_FTDC_OF_ForceOff: u8 = '5' as u8;
pub const THOST_FTDC_OF_LocalForceClose: u8 = '6' as u8;
pub const THOST_FTDC_FCC_NotForceClose: u8 = '0' as u8;
pub const THOST_FTDC_FCC_LackDeposit: u8 = '1' as u8;
pub const THOST_FTDC_FCC_ClientOverPositionLimit: u8 = '2' as u8;
pub const THOST_FTDC_FCC_MemberOverPositionLimit: u8 = '3' as u8;
pub const THOST_FTDC_FCC_NotMultiple: u8 = '4' as u8;
pub const THOST_FTDC_FCC_Violation: u8 = '5' as u8;
pub const THOST_FTDC_FCC_Other: u8 = '6' as u8;
pub const THOST_FTDC_FCC_PersonDeliv: u8 = '7' as u8;
pub const THOST_FTDC_FCC_Notverifycapital: u8 = '8' as u8;
pub const THOST_FTDC_FCC_LocalLackDeposit: u8 = '9' as u8;
pub const THOST_FTDC_FCC_LocalViolationNocheck: u8 = 'a' as u8;
pub const THOST_FTDC_FCC_LocalViolation: u8 = 'b' as u8;
pub const THOST_FTDC_ORDT_Normal: u8 = '0' as u8;
pub const THOST_FTDC_ORDT_DeriveFromQuote: u8 = '1' as u8;
pub const THOST_FTDC_ORDT_DeriveFromCombination: u8 = '2' as u8;
pub const THOST_FTDC_ORDT_Combination: u8 = '3' as u8;
pub const THOST_FTDC_ORDT_ConditionalOrder: u8 = '4' as u8;
pub const THOST_FTDC_ORDT_Swap: u8 = '5' as u8;
pub const THOST_FTDC_ORDT_DeriveFromBlockTrade: u8 = '6' as u8;
pub const THOST_FTDC_ORDT_DeriveFromEFPTrade: u8 = '7' as u8;
pub const THOST_FTDC_TC_IOC: u8 = '1' as u8;
pub const THOST_FTDC_TC_GFS: u8 = '2' as u8;
pub const THOST_FTDC_TC_GFD: u8 = '3' as u8;
pub const THOST_FTDC_TC_GTD: u8 = '4' as u8;
pub const THOST_FTDC_TC_GTC: u8 = '5' as u8;
pub const THOST_FTDC_TC_GFA: u8 = '6' as u8;
pub const THOST_FTDC_VC_AV: u8 = '1' as u8;
pub const THOST_FTDC_VC_MV: u8 = '2' as u8;
pub const THOST_FTDC_VC_CV: u8 = '3' as u8;
pub const THOST_FTDC_CC_Immediately: u8 = '1' as u8;
pub const THOST_FTDC_CC_Touch: u8 = '2' as u8;
pub const THOST_FTDC_CC_TouchProfit: u8 = '3' as u8;
pub const THOST_FTDC_CC_ParkedOrder: u8 = '4' as u8;
pub const THOST_FTDC_CC_LastPriceGreaterThanStopPrice: u8 = '5' as u8;
pub const THOST_FTDC_CC_LastPriceGreaterEqualStopPrice: u8 = '6' as u8;
pub const THOST_FTDC_CC_LastPriceLesserThanStopPrice: u8 = '7' as u8;
pub const THOST_FTDC_CC_LastPriceLesserEqualStopPrice: u8 = '8' as u8;
pub const THOST_FTDC_CC_AskPriceGreaterThanStopPrice: u8 = '9' as u8;
pub const THOST_FTDC_CC_AskPriceGreaterEqualStopPrice: u8 = 'A' as u8;
pub const THOST_FTDC_CC_AskPriceLesserThanStopPrice: u8 = 'B' as u8;
pub const THOST_FTDC_CC_AskPriceLesserEqualStopPrice: u8 = 'C' as u8;
pub const THOST_FTDC_CC_BidPriceGreaterThanStopPrice: u8 = 'D' as u8;
pub const THOST_FTDC_CC_BidPriceGreaterEqualStopPrice: u8 = 'E' as u8;
pub const THOST_FTDC_CC_BidPriceLesserThanStopPrice: u8 = 'F' as u8;
pub const THOST_FTDC_CC_BidPriceLesserEqualStopPrice: u8 = 'H' as u8;
pub const THOST_FTDC_AF_Delete: u8 = '0' as u8;
pub const THOST_FTDC_AF_Modify: u8 = '3' as u8;
pub const THOST_FTDC_TR_Allow: u8 = '0' as u8;
pub const THOST_FTDC_TR_CloseOnly: u8 = '1' as u8;
pub const THOST_FTDC_TR_Forbidden: u8 = '2' as u8;
pub const THOST_FTDC_OSRC_Participant: u8 = '0' as u8;
pub const THOST_FTDC_OSRC_Administrator: u8 = '1' as u8;
pub const THOST_FTDC_TRDT_SplitCombination: u8 = '#' as u8;
pub const THOST_FTDC_TRDT_Common: u8 = '0' as u8;
pub const THOST_FTDC_TRDT_OptionsExecution: u8 = '1' as u8;
pub const THOST_FTDC_TRDT_OTC: u8 = '2' as u8;
pub const THOST_FTDC_TRDT_EFPDerived: u8 = '3' as u8;
pub const THOST_FTDC_TRDT_CombinationDerived: u8 = '4' as u8;
pub const THOST_FTDC_TRDT_BlockTrade: u8 = '5' as u8;
pub const THOST_FTDC_SPOST_Common: u8 = '#' as u8;
pub const THOST_FTDC_SPOST_Tas: u8 = '0' as u8;
pub const THOST_FTDC_PSRC_LastPrice: u8 = '0' as u8;
pub const THOST_FTDC_PSRC_Buy: u8 = '1' as u8;
pub const THOST_FTDC_PSRC_Sell: u8 = '2' as u8;
pub const THOST_FTDC_PSRC_OTC: u8 = '3' as u8;
pub const THOST_FTDC_IS_BeforeTrading: u8 = '0' as u8;
pub const THOST_FTDC_IS_NoTrading: u8 = '1' as u8;
pub const THOST_FTDC_IS_Continous: u8 = '2' as u8;
pub const THOST_FTDC_IS_AuctionOrdering: u8 = '3' as u8;
pub const THOST_FTDC_IS_AuctionBalance: u8 = '4' as u8;
pub const THOST_FTDC_IS_AuctionMatch: u8 = '5' as u8;
pub const THOST_FTDC_IS_Closed: u8 = '6' as u8;
pub const THOST_FTDC_IS_TransactionProcessing: u8 = '7' as u8;
pub const THOST_FTDC_IER_Automatic: u8 = '1' as u8;
pub const THOST_FTDC_IER_Manual: u8 = '2' as u8;
pub const THOST_FTDC_IER_Fuse: u8 = '3' as u8;
pub const THOST_FTDC_BS_NoUpload: u8 = '1' as u8;
pub const THOST_FTDC_BS_Uploaded: u8 = '2' as u8;
pub const THOST_FTDC_BS_Failed: u8 = '3' as u8;
pub const THOST_FTDC_RS_All: u8 = '1' as u8;
pub const THOST_FTDC_RS_ByProduct: u8 = '2' as u8;
pub const THOST_FTDC_RP_ByVolume: u8 = '1' as u8;
pub const THOST_FTDC_RP_ByFeeOnHand: u8 = '2' as u8;
pub const THOST_FTDC_RL_Level1: u8 = '1' as u8;
pub const THOST_FTDC_RL_Level2: u8 = '2' as u8;
pub const THOST_FTDC_RL_Level3: u8 = '3' as u8;
pub const THOST_FTDC_RL_Level4: u8 = '4' as u8;
pub const THOST_FTDC_RL_Level5: u8 = '5' as u8;
pub const THOST_FTDC_RL_Level6: u8 = '6' as u8;
pub const THOST_FTDC_RL_Level7: u8 = '7' as u8;
pub const THOST_FTDC_RL_Level8: u8 = '8' as u8;
pub const THOST_FTDC_RL_Level9: u8 = '9' as u8;
pub const THOST_FTDC_RSD_ByPeriod: u8 = '1' as u8;
pub const THOST_FTDC_RSD_ByStandard: u8 = '2' as u8;
pub const THOST_FTDC_MT_Out: u8 = '0' as u8;
pub const THOST_FTDC_MT_In: u8 = '1' as u8;
pub const THOST_FTDC_ISPI_MortgageRatio: u8 = '4' as u8;
pub const THOST_FTDC_ISPI_MarginWay: u8 = '5' as u8;
pub const THOST_FTDC_ISPI_BillDeposit: u8 = '9' as u8;
pub const THOST_FTDC_ESPI_MortgageRatio: u8 = '1' as u8;
pub const THOST_FTDC_ESPI_OtherFundItem: u8 = '2' as u8;
pub const THOST_FTDC_ESPI_OtherFundImport: u8 = '3' as u8;
pub const THOST_FTDC_ESPI_CFFEXMinPrepa: u8 = '6' as u8;
pub const THOST_FTDC_ESPI_CZCESettlementType: u8 = '7' as u8;
pub const THOST_FTDC_ESPI_ExchDelivFeeMode: u8 = '9' as u8;
pub const THOST_FTDC_ESPI_DelivFeeMode: u8 = '0' as u8;
pub const THOST_FTDC_ESPI_CZCEComMarginType: u8 = 'A' as u8;
pub const THOST_FTDC_ESPI_DceComMarginType: u8 = 'B' as u8;
pub const THOST_FTDC_ESPI_OptOutDisCountRate: u8 = 'a' as u8;
pub const THOST_FTDC_ESPI_OptMiniGuarantee: u8 = 'b' as u8;
pub const THOST_FTDC_SPI_InvestorIDMinLength: u8 = '1' as u8;
pub const THOST_FTDC_SPI_AccountIDMinLength: u8 = '2' as u8;
pub const THOST_FTDC_SPI_UserRightLogon: u8 = '3' as u8;
pub const THOST_FTDC_SPI_SettlementBillTrade: u8 = '4' as u8;
pub const THOST_FTDC_SPI_TradingCode: u8 = '5' as u8;
pub const THOST_FTDC_SPI_CheckFund: u8 = '6' as u8;
pub const THOST_FTDC_SPI_CommModelRight: u8 = '7' as u8;
pub const THOST_FTDC_SPI_MarginModelRight: u8 = '9' as u8;
pub const THOST_FTDC_SPI_IsStandardActive: u8 = '8' as u8;
pub const THOST_FTDC_SPI_UploadSettlementFile: u8 = 'U' as u8;
pub const THOST_FTDC_SPI_DownloadCSRCFile: u8 = 'D' as u8;
pub const THOST_FTDC_SPI_SettlementBillFile: u8 = 'S' as u8;
pub const THOST_FTDC_SPI_CSRCOthersFile: u8 = 'C' as u8;
pub const THOST_FTDC_SPI_InvestorPhoto: u8 = 'P' as u8;
pub const THOST_FTDC_SPI_CSRCData: u8 = 'R' as u8;
pub const THOST_FTDC_SPI_InvestorPwdModel: u8 = 'I' as u8;
pub const THOST_FTDC_SPI_CFFEXInvestorSettleFile: u8 = 'F' as u8;
pub const THOST_FTDC_SPI_InvestorIDType: u8 = 'a' as u8;
pub const THOST_FTDC_SPI_FreezeMaxReMain: u8 = 'r' as u8;
pub const THOST_FTDC_SPI_IsSync: u8 = 'A' as u8;
pub const THOST_FTDC_SPI_RelieveOpenLimit: u8 = 'O' as u8;
pub const THOST_FTDC_SPI_IsStandardFreeze: u8 = 'X' as u8;
pub const THOST_FTDC_SPI_CZCENormalProductHedge: u8 = 'B' as u8;
pub const THOST_FTDC_TPID_EncryptionStandard: u8 = 'E' as u8;
pub const THOST_FTDC_TPID_RiskMode: u8 = 'R' as u8;
pub const THOST_FTDC_TPID_RiskModeGlobal: u8 = 'G' as u8;
pub const THOST_FTDC_TPID_modeEncode: u8 = 'P' as u8;
pub const THOST_FTDC_TPID_tickMode: u8 = 'T' as u8;
pub const THOST_FTDC_TPID_SingleUserSessionMaxNum: u8 = 'S' as u8;
pub const THOST_FTDC_TPID_LoginFailMaxNum: u8 = 'L' as u8;
pub const THOST_FTDC_TPID_IsAuthForce: u8 = 'A' as u8;
pub const THOST_FTDC_TPID_IsPosiFreeze: u8 = 'F' as u8;
pub const THOST_FTDC_TPID_IsPosiLimit: u8 = 'M' as u8;
pub const THOST_FTDC_TPID_ForQuoteTimeInterval: u8 = 'Q' as u8;
pub const THOST_FTDC_TPID_IsFuturePosiLimit: u8 = 'B' as u8;
pub const THOST_FTDC_TPID_IsFutureOrderFreq: u8 = 'C' as u8;
pub const THOST_FTDC_TPID_IsExecOrderProfit: u8 = 'H' as u8;
pub const THOST_FTDC_TPID_IsCheckBankAcc: u8 = 'I' as u8;
pub const THOST_FTDC_TPID_PasswordDeadLine: u8 = 'J' as u8;
pub const THOST_FTDC_TPID_IsStrongPassword: u8 = 'K' as u8;
pub const THOST_FTDC_TPID_BalanceMorgage: u8 = 'a' as u8;
pub const THOST_FTDC_TPID_MinPwdLen: u8 = 'O' as u8;
pub const THOST_FTDC_TPID_LoginFailMaxNumForIP: u8 = 'U' as u8;
pub const THOST_FTDC_TPID_PasswordPeriod: u8 = 'V' as u8;
pub const THOST_FTDC_TPID_PwdHistoryCmp: u8 = 'X' as u8;
pub const THOST_FTDC_TPID_TranferChkProperty: u8 = 'D' as u8;
pub const THOST_FTDC_FI_SettlementFund: u8 = 'F' as u8;
pub const THOST_FTDC_FI_Trade: u8 = 'T' as u8;
pub const THOST_FTDC_FI_InvestorPosition: u8 = 'P' as u8;
pub const THOST_FTDC_FI_SubEntryFund: u8 = 'O' as u8;
pub const THOST_FTDC_FI_CZCECombinationPos: u8 = 'C' as u8;
pub const THOST_FTDC_FI_CSRCData: u8 = 'R' as u8;
pub const THOST_FTDC_FI_CZCEClose: u8 = 'L' as u8;
pub const THOST_FTDC_FI_CZCENoClose: u8 = 'N' as u8;
pub const THOST_FTDC_FI_PositionDtl: u8 = 'D' as u8;
pub const THOST_FTDC_FI_OptionStrike: u8 = 'S' as u8;
pub const THOST_FTDC_FI_SettlementPriceComparison: u8 = 'M' as u8;
pub const THOST_FTDC_FI_NonTradePosChange: u8 = 'B' as u8;
pub const THOST_FTDC_FUT_Settlement: u8 = '0' as u8;
pub const THOST_FTDC_FUT_Check: u8 = '1' as u8;
pub const THOST_FTDC_FFT_Txt: u8 = '0' as u8;
pub const THOST_FTDC_FFT_Zip: u8 = '1' as u8;
pub const THOST_FTDC_FFT_DBF: u8 = '2' as u8;
pub const THOST_FTDC_FUS_SucceedUpload: u8 = '1' as u8;
pub const THOST_FTDC_FUS_FailedUpload: u8 = '2' as u8;
pub const THOST_FTDC_FUS_SucceedLoad: u8 = '3' as u8;
pub const THOST_FTDC_FUS_PartSucceedLoad: u8 = '4' as u8;
pub const THOST_FTDC_FUS_FailedLoad: u8 = '5' as u8;
pub const THOST_FTDC_TD_Out: u8 = '0' as u8;
pub const THOST_FTDC_TD_In: u8 = '1' as u8;
pub const THOST_FTDC_SC_NoSpecialRule: u8 = '0' as u8;
pub const THOST_FTDC_SC_NoSpringFestival: u8 = '1' as u8;
pub const THOST_FTDC_IPT_LastSettlement: u8 = '1' as u8;
pub const THOST_FTDC_IPT_LaseClose: u8 = '2' as u8;
pub const THOST_FTDC_PLP_Active: u8 = '1' as u8;
pub const THOST_FTDC_PLP_NonActive: u8 = '2' as u8;
pub const THOST_FTDC_PLP_Canceled: u8 = '3' as u8;
pub const THOST_FTDC_DM_CashDeliv: u8 = '1' as u8;
pub const THOST_FTDC_DM_CommodityDeliv: u8 = '2' as u8;
pub const THOST_FTDC_FIOT_FundIO: u8 = '1' as u8;
pub const THOST_FTDC_FIOT_Transfer: u8 = '2' as u8;
pub const THOST_FTDC_FIOT_SwapCurrency: u8 = '3' as u8;
pub const THOST_FTDC_FT_Deposite: u8 = '1' as u8;
pub const THOST_FTDC_FT_ItemFund: u8 = '2' as u8;
pub const THOST_FTDC_FT_Company: u8 = '3' as u8;
pub const THOST_FTDC_FT_InnerTransfer: u8 = '4' as u8;
pub const THOST_FTDC_FD_In: u8 = '1' as u8;
pub const THOST_FTDC_FD_Out: u8 = '2' as u8;
pub const THOST_FTDC_FS_Record: u8 = '1' as u8;
pub const THOST_FTDC_FS_Check: u8 = '2' as u8;
pub const THOST_FTDC_FS_Charge: u8 = '3' as u8;
pub const THOST_FTDC_PS_None: u8 = '1' as u8;
pub const THOST_FTDC_PS_Publishing: u8 = '2' as u8;
pub const THOST_FTDC_PS_Published: u8 = '3' as u8;
pub const THOST_FTDC_ES_NonActive: u8 = '1' as u8;
pub const THOST_FTDC_ES_Startup: u8 = '2' as u8;
pub const THOST_FTDC_ES_Initialize: u8 = '3' as u8;
pub const THOST_FTDC_ES_Initialized: u8 = '4' as u8;
pub const THOST_FTDC_ES_Close: u8 = '5' as u8;
pub const THOST_FTDC_ES_Closed: u8 = '6' as u8;
pub const THOST_FTDC_ES_Settlement: u8 = '7' as u8;
pub const THOST_FTDC_STS_Initialize: u8 = '0' as u8;
pub const THOST_FTDC_STS_Settlementing: u8 = '1' as u8;
pub const THOST_FTDC_STS_Settlemented: u8 = '2' as u8;
pub const THOST_FTDC_STS_Finished: u8 = '3' as u8;
pub const THOST_FTDC_CT_Person: u8 = '0' as u8;
pub const THOST_FTDC_CT_Company: u8 = '1' as u8;
pub const THOST_FTDC_CT_Fund: u8 = '2' as u8;
pub const THOST_FTDC_CT_SpecialOrgan: u8 = '3' as u8;
pub const THOST_FTDC_CT_Asset: u8 = '4' as u8;
pub const THOST_FTDC_BT_Trade: u8 = '0' as u8;
pub const THOST_FTDC_BT_TradeSettle: u8 = '1' as u8;
pub const THOST_FTDC_FAS_Low: u8 = '1' as u8;
pub const THOST_FTDC_FAS_Normal: u8 = '2' as u8;
pub const THOST_FTDC_FAS_Focus: u8 = '3' as u8;
pub const THOST_FTDC_FAS_Risk: u8 = '4' as u8;
pub const THOST_FTDC_FAS_ByTrade: u8 = '1' as u8;
pub const THOST_FTDC_FAS_ByDeliv: u8 = '2' as u8;
pub const THOST_FTDC_FAS_None: u8 = '3' as u8;
pub const THOST_FTDC_FAS_FixFee: u8 = '4' as u8;
pub const THOST_FTDC_PWDT_Trade: u8 = '1' as u8;
pub const THOST_FTDC_PWDT_Account: u8 = '2' as u8;
pub const THOST_FTDC_AG_All: u8 = '1' as u8;
pub const THOST_FTDC_AG_OnlyLost: u8 = '2' as u8;
pub const THOST_FTDC_AG_OnlyGain: u8 = '3' as u8;
pub const THOST_FTDC_AG_None: u8 = '4' as u8;
pub const THOST_FTDC_ICP_Include: u8 = '0' as u8;
pub const THOST_FTDC_ICP_NotInclude: u8 = '2' as u8;
pub const THOST_FTDC_AWT_Enable: u8 = '0' as u8;
pub const THOST_FTDC_AWT_Disable: u8 = '2' as u8;
pub const THOST_FTDC_AWT_NoHoldEnable: u8 = '3' as u8;
pub const THOST_FTDC_FPWD_UnCheck: u8 = '0' as u8;
pub const THOST_FTDC_FPWD_Check: u8 = '1' as u8;
pub const THOST_FTDC_TT_BankToFuture: u8 = '0' as u8;
pub const THOST_FTDC_TT_FutureToBank: u8 = '1' as u8;
pub const THOST_FTDC_TVF_Invalid: u8 = '0' as u8;
pub const THOST_FTDC_TVF_Valid: u8 = '1' as u8;
pub const THOST_FTDC_TVF_Reverse: u8 = '2' as u8;
pub const THOST_FTDC_RN_CD: u8 = '0' as u8;
pub const THOST_FTDC_RN_ZT: u8 = '1' as u8;
pub const THOST_FTDC_RN_QT: u8 = '2' as u8;
pub const THOST_FTDC_SEX_None: u8 = '0' as u8;
pub const THOST_FTDC_SEX_Man: u8 = '1' as u8;
pub const THOST_FTDC_SEX_Woman: u8 = '2' as u8;
pub const THOST_FTDC_UT_Investor: u8 = '0' as u8;
pub const THOST_FTDC_UT_Operator: u8 = '1' as u8;
pub const THOST_FTDC_UT_SuperUser: u8 = '2' as u8;
pub const THOST_FTDC_RATETYPE_MarginRate: u8 = '2' as u8;
pub const THOST_FTDC_NOTETYPE_TradeSettleBill: u8 = '1' as u8;
pub const THOST_FTDC_NOTETYPE_TradeSettleMonth: u8 = '2' as u8;
pub const THOST_FTDC_NOTETYPE_CallMarginNotes: u8 = '3' as u8;
pub const THOST_FTDC_NOTETYPE_ForceCloseNotes: u8 = '4' as u8;
pub const THOST_FTDC_NOTETYPE_TradeNotes: u8 = '5' as u8;
pub const THOST_FTDC_NOTETYPE_DelivNotes: u8 = '6' as u8;
pub const THOST_FTDC_SBS_Day: u8 = '1' as u8;
pub const THOST_FTDC_SBS_Volume: u8 = '2' as u8;
pub const THOST_FTDC_ST_Day: u8 = '0' as u8;
pub const THOST_FTDC_ST_Month: u8 = '1' as u8;
pub const THOST_FTDC_URT_Logon: u8 = '1' as u8;
pub const THOST_FTDC_URT_Transfer: u8 = '2' as u8;
pub const THOST_FTDC_URT_EMail: u8 = '3' as u8;
pub const THOST_FTDC_URT_Fax: u8 = '4' as u8;
pub const THOST_FTDC_URT_ConditionOrder: u8 = '5' as u8;
pub const THOST_FTDC_MPT_PreSettlementPrice: u8 = '1' as u8;
pub const THOST_FTDC_MPT_SettlementPrice: u8 = '2' as u8;
pub const THOST_FTDC_MPT_AveragePrice: u8 = '3' as u8;
pub const THOST_FTDC_MPT_OpenPrice: u8 = '4' as u8;
pub const THOST_FTDC_BGS_None: u8 = '0' as u8;
pub const THOST_FTDC_BGS_NoGenerated: u8 = '1' as u8;
pub const THOST_FTDC_BGS_Generated: u8 = '2' as u8;
pub const THOST_FTDC_AT_HandlePositionAlgo: u8 = '1' as u8;
pub const THOST_FTDC_AT_FindMarginRateAlgo: u8 = '2' as u8;
pub const THOST_FTDC_HPA_Base: u8 = '1' as u8;
pub const THOST_FTDC_HPA_DCE: u8 = '2' as u8;
pub const THOST_FTDC_HPA_CZCE: u8 = '3' as u8;
pub const THOST_FTDC_FMRA_Base: u8 = '1' as u8;
pub const THOST_FTDC_FMRA_DCE: u8 = '2' as u8;
pub const THOST_FTDC_FMRA_CZCE: u8 = '3' as u8;
pub const THOST_FTDC_HTAA_Base: u8 = '1' as u8;
pub const THOST_FTDC_HTAA_DCE: u8 = '2' as u8;
pub const THOST_FTDC_HTAA_CZCE: u8 = '3' as u8;
pub const THOST_FTDC_PST_Order: u8 = '1' as u8;
pub const THOST_FTDC_PST_Open: u8 = '2' as u8;
pub const THOST_FTDC_PST_Fund: u8 = '3' as u8;
pub const THOST_FTDC_PST_Settlement: u8 = '4' as u8;
pub const THOST_FTDC_PST_Company: u8 = '5' as u8;
pub const THOST_FTDC_PST_Corporation: u8 = '6' as u8;
pub const THOST_FTDC_PST_LinkMan: u8 = '7' as u8;
pub const THOST_FTDC_PST_Ledger: u8 = '8' as u8;
pub const THOST_FTDC_PST_Trustee: u8 = '9' as u8;
pub const THOST_FTDC_PST_TrusteeCorporation: u8 = 'A' as u8;
pub const THOST_FTDC_PST_TrusteeOpen: u8 = 'B' as u8;
pub const THOST_FTDC_PST_TrusteeContact: u8 = 'C' as u8;
pub const THOST_FTDC_PST_ForeignerRefer: u8 = 'D' as u8;
pub const THOST_FTDC_PST_CorporationRefer: u8 = 'E' as u8;
pub const THOST_FTDC_QIR_All: u8 = '1' as u8;
pub const THOST_FTDC_QIR_Group: u8 = '2' as u8;
pub const THOST_FTDC_QIR_Single: u8 = '3' as u8;
pub const THOST_FTDC_IRS_Normal: u8 = '1' as u8;
pub const THOST_FTDC_IRS_Warn: u8 = '2' as u8;
pub const THOST_FTDC_IRS_Call: u8 = '3' as u8;
pub const THOST_FTDC_IRS_Force: u8 = '4' as u8;
pub const THOST_FTDC_IRS_Exception: u8 = '5' as u8;
pub const THOST_FTDC_UET_Login: u8 = '1' as u8;
pub const THOST_FTDC_UET_Logout: u8 = '2' as u8;
pub const THOST_FTDC_UET_Trading: u8 = '3' as u8;
pub const THOST_FTDC_UET_TradingError: u8 = '4' as u8;
pub const THOST_FTDC_UET_UpdatePassword: u8 = '5' as u8;
pub const THOST_FTDC_UET_Authenticate: u8 = '6' as u8;
pub const THOST_FTDC_UET_SubmitSysInfo: u8 = '7' as u8;
pub const THOST_FTDC_UET_Transfer: u8 = '8' as u8;
pub const THOST_FTDC_UET_Other: u8 = '9' as u8;
pub const THOST_FTDC_UET_UpdateTradingAccountPassword: u8 = 'a' as u8;
pub const THOST_FTDC_ICS_Close: u8 = '0' as u8;
pub const THOST_FTDC_ICS_CloseToday: u8 = '1' as u8;
pub const THOST_FTDC_SM_Non: u8 = '0' as u8;
pub const THOST_FTDC_SM_Instrument: u8 = '1' as u8;
pub const THOST_FTDC_SM_Product: u8 = '2' as u8;
pub const THOST_FTDC_SM_Investor: u8 = '3' as u8;
pub const THOST_FTDC_PAOS_NotSend: u8 = '1' as u8;
pub const THOST_FTDC_PAOS_Send: u8 = '2' as u8;
pub const THOST_FTDC_PAOS_Deleted: u8 = '3' as u8;
pub const THOST_FTDC_VDS_Dealing: u8 = '1' as u8;
pub const THOST_FTDC_VDS_DeaclSucceed: u8 = '2' as u8;
pub const THOST_FTDC_ORGS_Standard: u8 = '0' as u8;
pub const THOST_FTDC_ORGS_ESunny: u8 = '1' as u8;
pub const THOST_FTDC_ORGS_KingStarV6: u8 = '2' as u8;
pub const THOST_FTDC_VTS_NaturalDeal: u8 = '0' as u8;
pub const THOST_FTDC_VTS_SucceedEnd: u8 = '1' as u8;
pub const THOST_FTDC_VTS_FailedEND: u8 = '2' as u8;
pub const THOST_FTDC_VTS_Exception: u8 = '3' as u8;
pub const THOST_FTDC_VTS_ManualDeal: u8 = '4' as u8;
pub const THOST_FTDC_VTS_MesException: u8 = '5' as u8;
pub const THOST_FTDC_VTS_SysException: u8 = '6' as u8;
pub const THOST_FTDC_VBAT_BankBook: u8 = '1' as u8;
pub const THOST_FTDC_VBAT_BankCard: u8 = '2' as u8;
pub const THOST_FTDC_VBAT_CreditCard: u8 = '3' as u8;
pub const THOST_FTDC_VMS_Natural: u8 = '0' as u8;
pub const THOST_FTDC_VMS_Canceled: u8 = '9' as u8;
pub const THOST_FTDC_VAA_NoAvailAbility: u8 = '0' as u8;
pub const THOST_FTDC_VAA_AvailAbility: u8 = '1' as u8;
pub const THOST_FTDC_VAA_Repeal: u8 = '2' as u8;
pub const THOST_FTDC_GEN_Program: u8 = '0' as u8;
pub const THOST_FTDC_GEN_HandWork: u8 = '1' as u8;
pub const THOST_FTDC_CFMMCKK_REQUEST: u8 = 'R' as u8;
pub const THOST_FTDC_CFMMCKK_AUTO: u8 = 'A' as u8;
pub const THOST_FTDC_CFMMCKK_MANUAL: u8 = 'M' as u8;
pub const THOST_FTDC_CFT_IDCard: u8 = '0' as u8;
pub const THOST_FTDC_CFT_Passport: u8 = '1' as u8;
pub const THOST_FTDC_CFT_OfficerIDCard: u8 = '2' as u8;
pub const THOST_FTDC_CFT_SoldierIDCard: u8 = '3' as u8;
pub const THOST_FTDC_CFT_HomeComingCard: u8 = '4' as u8;
pub const THOST_FTDC_CFT_HouseholdRegister: u8 = '5' as u8;
pub const THOST_FTDC_CFT_LicenseNo: u8 = '6' as u8;
pub const THOST_FTDC_CFT_InstitutionCodeCard: u8 = '7' as u8;
pub const THOST_FTDC_CFT_TempLicenseNo: u8 = '8' as u8;
pub const THOST_FTDC_CFT_NoEnterpriseLicenseNo: u8 = '9' as u8;
pub const THOST_FTDC_CFT_OtherCard: u8 = 'x' as u8;
pub const THOST_FTDC_CFT_SuperDepAgree: u8 = 'a' as u8;
pub const THOST_FTDC_FBC_Others: u8 = '0' as u8;
pub const THOST_FTDC_FBC_TransferDetails: u8 = '1' as u8;
pub const THOST_FTDC_FBC_CustAccStatus: u8 = '2' as u8;
pub const THOST_FTDC_FBC_AccountTradeDetails: u8 = '3' as u8;
pub const THOST_FTDC_FBC_FutureAccountChangeInfoDetails: u8 = '4' as u8;
pub const THOST_FTDC_FBC_CustMoneyDetail: u8 = '5' as u8;
pub const THOST_FTDC_FBC_CustCancelAccountInfo: u8 = '6' as u8;
pub const THOST_FTDC_FBC_CustMoneyResult: u8 = '7' as u8;
pub const THOST_FTDC_FBC_OthersExceptionResult: u8 = '8' as u8;
pub const THOST_FTDC_FBC_CustInterestNetMoneyDetails: u8 = '9' as u8;
pub const THOST_FTDC_FBC_CustMoneySendAndReceiveDetails: u8 = 'a' as u8;
pub const THOST_FTDC_FBC_CorporationMoneyTotal: u8 = 'b' as u8;
pub const THOST_FTDC_FBC_MainbodyMoneyTotal: u8 = 'c' as u8;
pub const THOST_FTDC_FBC_MainPartMonitorData: u8 = 'd' as u8;
pub const THOST_FTDC_FBC_PreparationMoney: u8 = 'e' as u8;
pub const THOST_FTDC_FBC_BankMoneyMonitorData: u8 = 'f' as u8;
pub const THOST_FTDC_CEC_Exchange: u8 = '1' as u8;
pub const THOST_FTDC_CEC_Cash: u8 = '2' as u8;
pub const THOST_FTDC_YNI_Yes: u8 = '0' as u8;
pub const THOST_FTDC_YNI_No: u8 = '1' as u8;
pub const THOST_FTDC_BLT_CurrentMoney: u8 = '0' as u8;
pub const THOST_FTDC_BLT_UsableMoney: u8 = '1' as u8;
pub const THOST_FTDC_BLT_FetchableMoney: u8 = '2' as u8;
pub const THOST_FTDC_BLT_FreezeMoney: u8 = '3' as u8;
pub const THOST_FTDC_GD_Unknown: u8 = '0' as u8;
pub const THOST_FTDC_GD_Male: u8 = '1' as u8;
pub const THOST_FTDC_GD_Female: u8 = '2' as u8;
pub const THOST_FTDC_FPF_BEN: u8 = '0' as u8;
pub const THOST_FTDC_FPF_OUR: u8 = '1' as u8;
pub const THOST_FTDC_FPF_SHA: u8 = '2' as u8;
pub const THOST_FTDC_PWKT_ExchangeKey: u8 = '0' as u8;
pub const THOST_FTDC_PWKT_PassWordKey: u8 = '1' as u8;
pub const THOST_FTDC_PWKT_MACKey: u8 = '2' as u8;
pub const THOST_FTDC_PWKT_MessageKey: u8 = '3' as u8;
pub const THOST_FTDC_PWT_Query: u8 = '0' as u8;
pub const THOST_FTDC_PWT_Fetch: u8 = '1' as u8;
pub const THOST_FTDC_PWT_Transfer: u8 = '2' as u8;
pub const THOST_FTDC_PWT_Trade: u8 = '3' as u8;
pub const THOST_FTDC_EM_NoEncry: u8 = '0' as u8;
pub const THOST_FTDC_EM_DES: u8 = '1' as u8;
pub const THOST_FTDC_EM_3DES: u8 = '2' as u8;
pub const THOST_FTDC_BRF_BankNotNeedRepeal: u8 = '0' as u8;
pub const THOST_FTDC_BRF_BankWaitingRepeal: u8 = '1' as u8;
pub const THOST_FTDC_BRF_BankBeenRepealed: u8 = '2' as u8;
pub const THOST_FTDC_BRORF_BrokerNotNeedRepeal: u8 = '0' as u8;
pub const THOST_FTDC_BRORF_BrokerWaitingRepeal: u8 = '1' as u8;
pub const THOST_FTDC_BRORF_BrokerBeenRepealed: u8 = '2' as u8;
pub const THOST_FTDC_TS_Bank: u8 = '0' as u8;
pub const THOST_FTDC_TS_Future: u8 = '1' as u8;
pub const THOST_FTDC_TS_Store: u8 = '2' as u8;
pub const THOST_FTDC_LF_Yes: u8 = '0' as u8;
pub const THOST_FTDC_LF_No: u8 = '1' as u8;
pub const THOST_FTDC_BAS_Normal: u8 = '0' as u8;
pub const THOST_FTDC_BAS_Freeze: u8 = '1' as u8;
pub const THOST_FTDC_BAS_ReportLoss: u8 = '2' as u8;
pub const THOST_FTDC_MAS_Normal: u8 = '0' as u8;
pub const THOST_FTDC_MAS_Cancel: u8 = '1' as u8;
pub const THOST_FTDC_MSS_Point: u8 = '0' as u8;
pub const THOST_FTDC_MSS_PrePoint: u8 = '1' as u8;
pub const THOST_FTDC_MSS_CancelPoint: u8 = '2' as u8;
pub const THOST_FTDC_SYT_FutureBankTransfer: u8 = '0' as u8;
pub const THOST_FTDC_SYT_StockBankTransfer: u8 = '1' as u8;
pub const THOST_FTDC_SYT_TheThirdPartStore: u8 = '2' as u8;
pub const THOST_FTDC_TEF_NormalProcessing: u8 = '0' as u8;
pub const THOST_FTDC_TEF_Success: u8 = '1' as u8;
pub const THOST_FTDC_TEF_Failed: u8 = '2' as u8;
pub const THOST_FTDC_TEF_Abnormal: u8 = '3' as u8;
pub const THOST_FTDC_TEF_ManualProcessedForException: u8 = '4' as u8;
pub const THOST_FTDC_TEF_CommuFailedNeedManualProcess: u8 = '5' as u8;
pub const THOST_FTDC_TEF_SysErrorNeedManualProcess: u8 = '6' as u8;
pub const THOST_FTDC_PSS_NotProcess: u8 = '0' as u8;
pub const THOST_FTDC_PSS_StartProcess: u8 = '1' as u8;
pub const THOST_FTDC_PSS_Finished: u8 = '2' as u8;
pub const THOST_FTDC_CUSTT_Person: u8 = '0' as u8;
pub const THOST_FTDC_CUSTT_Institution: u8 = '1' as u8;
pub const THOST_FTDC_FBTTD_FromBankToFuture: u8 = '1' as u8;
pub const THOST_FTDC_FBTTD_FromFutureToBank: u8 = '2' as u8;
pub const THOST_FTDC_OOD_Open: u8 = '1' as u8;
pub const THOST_FTDC_OOD_Destroy: u8 = '0' as u8;
pub const THOST_FTDC_AVAF_Invalid: u8 = '0' as u8;
pub const THOST_FTDC_AVAF_Valid: u8 = '1' as u8;
pub const THOST_FTDC_AVAF_Repeal: u8 = '2' as u8;
pub const THOST_FTDC_OT_Bank: u8 = '1' as u8;
pub const THOST_FTDC_OT_Future: u8 = '2' as u8;
pub const THOST_FTDC_OT_PlateForm: u8 = '9' as u8;
pub const THOST_FTDC_OL_HeadQuarters: u8 = '1' as u8;
pub const THOST_FTDC_OL_Branch: u8 = '2' as u8;
pub const THOST_FTDC_PID_FutureProtocal: u8 = '0' as u8;
pub const THOST_FTDC_PID_ICBCProtocal: u8 = '1' as u8;
pub const THOST_FTDC_PID_ABCProtocal: u8 = '2' as u8;
pub const THOST_FTDC_PID_CBCProtocal: u8 = '3' as u8;
pub const THOST_FTDC_PID_CCBProtocal: u8 = '4' as u8;
pub const THOST_FTDC_PID_BOCOMProtocal: u8 = '5' as u8;
pub const THOST_FTDC_PID_FBTPlateFormProtocal: u8 = 'X' as u8;
pub const THOST_FTDC_CM_ShortConnect: u8 = '0' as u8;
pub const THOST_FTDC_CM_LongConnect: u8 = '1' as u8;
pub const THOST_FTDC_SRM_ASync: u8 = '0' as u8;
pub const THOST_FTDC_SRM_Sync: u8 = '1' as u8;
pub const THOST_FTDC_BAT_BankBook: u8 = '1' as u8;
pub const THOST_FTDC_BAT_SavingCard: u8 = '2' as u8;
pub const THOST_FTDC_BAT_CreditCard: u8 = '3' as u8;
pub const THOST_FTDC_FAT_BankBook: u8 = '1' as u8;
pub const THOST_FTDC_FAT_SavingCard: u8 = '2' as u8;
pub const THOST_FTDC_FAT_CreditCard: u8 = '3' as u8;
pub const THOST_FTDC_OS_Ready: u8 = '0' as u8;
pub const THOST_FTDC_OS_CheckIn: u8 = '1' as u8;
pub const THOST_FTDC_OS_CheckOut: u8 = '2' as u8;
pub const THOST_FTDC_OS_CheckFileArrived: u8 = '3' as u8;
pub const THOST_FTDC_OS_CheckDetail: u8 = '4' as u8;
pub const THOST_FTDC_OS_DayEndClean: u8 = '5' as u8;
pub const THOST_FTDC_OS_Invalid: u8 = '9' as u8;
pub const THOST_FTDC_CCBFM_ByAmount: u8 = '1' as u8;
pub const THOST_FTDC_CCBFM_ByMonth: u8 = '2' as u8;
pub const THOST_FTDC_CAPIT_Client: u8 = '1' as u8;
pub const THOST_FTDC_CAPIT_Server: u8 = '2' as u8;
pub const THOST_FTDC_CAPIT_UserApi: u8 = '3' as u8;
pub const THOST_FTDC_LS_Connected: u8 = '1' as u8;
pub const THOST_FTDC_LS_Disconnected: u8 = '2' as u8;
pub const THOST_FTDC_BPWDF_NoCheck: u8 = '0' as u8;
pub const THOST_FTDC_BPWDF_BlankCheck: u8 = '1' as u8;
pub const THOST_FTDC_BPWDF_EncryptCheck: u8 = '2' as u8;
pub const THOST_FTDC_SAT_AccountID: u8 = '1' as u8;
pub const THOST_FTDC_SAT_CardID: u8 = '2' as u8;
pub const THOST_FTDC_SAT_SHStockholderID: u8 = '3' as u8;
pub const THOST_FTDC_SAT_SZStockholderID: u8 = '4' as u8;
pub const THOST_FTDC_TRFS_Normal: u8 = '0' as u8;
pub const THOST_FTDC_TRFS_Repealed: u8 = '1' as u8;
pub const THOST_FTDC_SPTYPE_Broker: u8 = '0' as u8;
pub const THOST_FTDC_SPTYPE_Bank: u8 = '1' as u8;
pub const THOST_FTDC_REQRSP_Request: u8 = '0' as u8;
pub const THOST_FTDC_REQRSP_Response: u8 = '1' as u8;
pub const THOST_FTDC_FBTUET_SignIn: u8 = '0' as u8;
pub const THOST_FTDC_FBTUET_FromBankToFuture: u8 = '1' as u8;
pub const THOST_FTDC_FBTUET_FromFutureToBank: u8 = '2' as u8;
pub const THOST_FTDC_FBTUET_OpenAccount: u8 = '3' as u8;
pub const THOST_FTDC_FBTUET_CancelAccount: u8 = '4' as u8;
pub const THOST_FTDC_FBTUET_ChangeAccount: u8 = '5' as u8;
pub const THOST_FTDC_FBTUET_RepealFromBankToFuture: u8 = '6' as u8;
pub const THOST_FTDC_FBTUET_RepealFromFutureToBank: u8 = '7' as u8;
pub const THOST_FTDC_FBTUET_QueryBankAccount: u8 = '8' as u8;
pub const THOST_FTDC_FBTUET_QueryFutureAccount: u8 = '9' as u8;
pub const THOST_FTDC_FBTUET_SignOut: u8 = 'A' as u8;
pub const THOST_FTDC_FBTUET_SyncKey: u8 = 'B' as u8;
pub const THOST_FTDC_FBTUET_ReserveOpenAccount: u8 = 'C' as u8;
pub const THOST_FTDC_FBTUET_CancelReserveOpenAccount: u8 = 'D' as u8;
pub const THOST_FTDC_FBTUET_ReserveOpenAccountConfirm: u8 = 'E' as u8;
pub const THOST_FTDC_FBTUET_Other: u8 = 'Z' as u8;
pub const THOST_FTDC_DBOP_Insert: u8 = '0' as u8;
pub const THOST_FTDC_DBOP_Update: u8 = '1' as u8;
pub const THOST_FTDC_DBOP_Delete: u8 = '2' as u8;
pub const THOST_FTDC_SYNF_Yes: u8 = '0' as u8;
pub const THOST_FTDC_SYNF_No: u8 = '1' as u8;
pub const THOST_FTDC_SYNT_OneOffSync: u8 = '0' as u8;
pub const THOST_FTDC_SYNT_TimerSync: u8 = '1' as u8;
pub const THOST_FTDC_SYNT_TimerFullSync: u8 = '2' as u8;
pub const THOST_FTDC_FBEDIR_Settlement: u8 = '0' as u8;
pub const THOST_FTDC_FBEDIR_Sale: u8 = '1' as u8;
pub const THOST_FTDC_FBERES_Success: u8 = '0' as u8;
pub const THOST_FTDC_FBERES_InsufficientBalance: u8 = '1' as u8;
pub const THOST_FTDC_FBERES_UnknownTrading: u8 = '8' as u8;
pub const THOST_FTDC_FBERES_Fail: u8 = 'x' as u8;
pub const THOST_FTDC_FBEES_Normal: u8 = '0' as u8;
pub const THOST_FTDC_FBEES_ReExchange: u8 = '1' as u8;
pub const THOST_FTDC_FBEFG_DataPackage: u8 = '0' as u8;
pub const THOST_FTDC_FBEFG_File: u8 = '1' as u8;
pub const THOST_FTDC_FBEAT_NotTrade: u8 = '0' as u8;
pub const THOST_FTDC_FBEAT_Trade: u8 = '1' as u8;
pub const THOST_FTDC_FBEUET_SignIn: u8 = '0' as u8;
pub const THOST_FTDC_FBEUET_Exchange: u8 = '1' as u8;
pub const THOST_FTDC_FBEUET_ReExchange: u8 = '2' as u8;
pub const THOST_FTDC_FBEUET_QueryBankAccount: u8 = '3' as u8;
pub const THOST_FTDC_FBEUET_QueryExchDetial: u8 = '4' as u8;
pub const THOST_FTDC_FBEUET_QueryExchSummary: u8 = '5' as u8;
pub const THOST_FTDC_FBEUET_QueryExchRate: u8 = '6' as u8;
pub const THOST_FTDC_FBEUET_CheckBankAccount: u8 = '7' as u8;
pub const THOST_FTDC_FBEUET_SignOut: u8 = '8' as u8;
pub const THOST_FTDC_FBEUET_Other: u8 = 'Z' as u8;
pub const THOST_FTDC_FBERF_UnProcessed: u8 = '0' as u8;
pub const THOST_FTDC_FBERF_WaitSend: u8 = '1' as u8;
pub const THOST_FTDC_FBERF_SendSuccess: u8 = '2' as u8;
pub const THOST_FTDC_FBERF_SendFailed: u8 = '3' as u8;
pub const THOST_FTDC_FBERF_WaitReSend: u8 = '4' as u8;
pub const THOST_FTDC_NC_NOERROR: u8 = '0' as u8;
pub const THOST_FTDC_NC_Warn: u8 = '1' as u8;
pub const THOST_FTDC_NC_Call: u8 = '2' as u8;
pub const THOST_FTDC_NC_Force: u8 = '3' as u8;
pub const THOST_FTDC_NC_CHUANCANG: u8 = '4' as u8;
pub const THOST_FTDC_NC_Exception: u8 = '5' as u8;
pub const THOST_FTDC_FCT_Manual: u8 = '0' as u8;
pub const THOST_FTDC_FCT_Single: u8 = '1' as u8;
pub const THOST_FTDC_FCT_Group: u8 = '2' as u8;
pub const THOST_FTDC_RNM_System: u8 = '0' as u8;
pub const THOST_FTDC_RNM_SMS: u8 = '1' as u8;
pub const THOST_FTDC_RNM_EMail: u8 = '2' as u8;
pub const THOST_FTDC_RNM_Manual: u8 = '3' as u8;
pub const THOST_FTDC_RNS_NotGen: u8 = '0' as u8;
pub const THOST_FTDC_RNS_Generated: u8 = '1' as u8;
pub const THOST_FTDC_RNS_SendError: u8 = '2' as u8;
pub const THOST_FTDC_RNS_SendOk: u8 = '3' as u8;
pub const THOST_FTDC_RNS_Received: u8 = '4' as u8;
pub const THOST_FTDC_RNS_Confirmed: u8 = '5' as u8;
pub const THOST_FTDC_RUE_ExportData: u8 = '0' as u8;
pub const THOST_FTDC_COST_LastPriceAsc: u8 = '0' as u8;
pub const THOST_FTDC_COST_LastPriceDesc: u8 = '1' as u8;
pub const THOST_FTDC_COST_AskPriceAsc: u8 = '2' as u8;
pub const THOST_FTDC_COST_AskPriceDesc: u8 = '3' as u8;
pub const THOST_FTDC_COST_BidPriceAsc: u8 = '4' as u8;
pub const THOST_FTDC_COST_BidPriceDesc: u8 = '5' as u8;
pub const THOST_FTDC_UOAST_NoSend: u8 = '0' as u8;
pub const THOST_FTDC_UOAST_Sended: u8 = '1' as u8;
pub const THOST_FTDC_UOAST_Generated: u8 = '2' as u8;
pub const THOST_FTDC_UOAST_SendFail: u8 = '3' as u8;
pub const THOST_FTDC_UOAST_Success: u8 = '4' as u8;
pub const THOST_FTDC_UOAST_Fail: u8 = '5' as u8;
pub const THOST_FTDC_UOAST_Cancel: u8 = '6' as u8;
pub const THOST_FTDC_UOACS_NoApply: u8 = '1' as u8;
pub const THOST_FTDC_UOACS_Submited: u8 = '2' as u8;
pub const THOST_FTDC_UOACS_Sended: u8 = '3' as u8;
pub const THOST_FTDC_UOACS_Success: u8 = '4' as u8;
pub const THOST_FTDC_UOACS_Refuse: u8 = '5' as u8;
pub const THOST_FTDC_UOACS_Cancel: u8 = '6' as u8;
pub const THOST_FTDC_QT_Radio: u8 = '1' as u8;
pub const THOST_FTDC_QT_Option: u8 = '2' as u8;
pub const THOST_FTDC_QT_Blank: u8 = '3' as u8;
pub const THOST_FTDC_BT_Request: u8 = '1' as u8;
pub const THOST_FTDC_BT_Response: u8 = '2' as u8;
pub const THOST_FTDC_BT_Notice: u8 = '3' as u8;
pub const THOST_FTDC_CRC_Success: u8 = '0' as u8;
pub const THOST_FTDC_CRC_Working: u8 = '1' as u8;
pub const THOST_FTDC_CRC_InfoFail: u8 = '2' as u8;
pub const THOST_FTDC_CRC_IDCardFail: u8 = '3' as u8;
pub const THOST_FTDC_CRC_OtherFail: u8 = '4' as u8;
pub const THOST_FTDC_CfMMCCT_All: u8 = '0' as u8;
pub const THOST_FTDC_CfMMCCT_Person: u8 = '1' as u8;
pub const THOST_FTDC_CfMMCCT_Company: u8 = '2' as u8;
pub const THOST_FTDC_CfMMCCT_Other: u8 = '3' as u8;
pub const THOST_FTDC_CfMMCCT_SpecialOrgan: u8 = '4' as u8;
pub const THOST_FTDC_CfMMCCT_Asset: u8 = '5' as u8;
pub const THOST_FTDC_EIDT_SHFE: u8 = 'S' as u8;
pub const THOST_FTDC_EIDT_CZCE: u8 = 'Z' as u8;
pub const THOST_FTDC_EIDT_DCE: u8 = 'D' as u8;
pub const THOST_FTDC_EIDT_CFFEX: u8 = 'J' as u8;
pub const THOST_FTDC_EIDT_INE: u8 = 'N' as u8;
pub const THOST_FTDC_ECIDT_Hedge: u8 = '1' as u8;
pub const THOST_FTDC_ECIDT_Arbitrage: u8 = '2' as u8;
pub const THOST_FTDC_ECIDT_Speculation: u8 = '3' as u8;
pub const THOST_FTDC_UF_NoUpdate: u8 = '0' as u8;
pub const THOST_FTDC_UF_Success: u8 = '1' as u8;
pub const THOST_FTDC_UF_Fail: u8 = '2' as u8;
pub const THOST_FTDC_UF_TCSuccess: u8 = '3' as u8;
pub const THOST_FTDC_UF_TCFail: u8 = '4' as u8;
pub const THOST_FTDC_UF_Cancel: u8 = '5' as u8;
pub const THOST_FTDC_AOID_OpenInvestor: u8 = '1' as u8;
pub const THOST_FTDC_AOID_ModifyIDCard: u8 = '2' as u8;
pub const THOST_FTDC_AOID_ModifyNoIDCard: u8 = '3' as u8;
pub const THOST_FTDC_AOID_ApplyTradingCode: u8 = '4' as u8;
pub const THOST_FTDC_AOID_CancelTradingCode: u8 = '5' as u8;
pub const THOST_FTDC_AOID_CancelInvestor: u8 = '6' as u8;
pub const THOST_FTDC_AOID_FreezeAccount: u8 = '8' as u8;
pub const THOST_FTDC_AOID_ActiveFreezeAccount: u8 = '9' as u8;
pub const THOST_FTDC_ASID_NoComplete: u8 = '1' as u8;
pub const THOST_FTDC_ASID_Submited: u8 = '2' as u8;
pub const THOST_FTDC_ASID_Checked: u8 = '3' as u8;
pub const THOST_FTDC_ASID_Refused: u8 = '4' as u8;
pub const THOST_FTDC_ASID_Deleted: u8 = '5' as u8;
pub const THOST_FTDC_UOASM_ByAPI: u8 = '1' as u8;
pub const THOST_FTDC_UOASM_ByFile: u8 = '2' as u8;
pub const THOST_FTDC_EvM_ADD: u8 = '1' as u8;
pub const THOST_FTDC_EvM_UPDATE: u8 = '2' as u8;
pub const THOST_FTDC_EvM_DELETE: u8 = '3' as u8;
pub const THOST_FTDC_EvM_CHECK: u8 = '4' as u8;
pub const THOST_FTDC_EvM_COPY: u8 = '5' as u8;
pub const THOST_FTDC_EvM_CANCEL: u8 = '6' as u8;
pub const THOST_FTDC_EvM_Reverse: u8 = '7' as u8;
pub const THOST_FTDC_UOAA_ASR: u8 = '1' as u8;
pub const THOST_FTDC_UOAA_ASNR: u8 = '2' as u8;
pub const THOST_FTDC_UOAA_NSAR: u8 = '3' as u8;
pub const THOST_FTDC_UOAA_NSR: u8 = '4' as u8;
pub const THOST_FTDC_EvM_InvestorGroupFlow: u8 = '1' as u8;
pub const THOST_FTDC_EvM_InvestorRate: u8 = '2' as u8;
pub const THOST_FTDC_EvM_InvestorCommRateModel: u8 = '3' as u8;
pub const THOST_FTDC_CL_Zero: u8 = '0' as u8;
pub const THOST_FTDC_CL_One: u8 = '1' as u8;
pub const THOST_FTDC_CL_Two: u8 = '2' as u8;
pub const THOST_FTDC_CHS_Init: u8 = '0' as u8;
pub const THOST_FTDC_CHS_Checking: u8 = '1' as u8;
pub const THOST_FTDC_CHS_Checked: u8 = '2' as u8;
pub const THOST_FTDC_CHS_Refuse: u8 = '3' as u8;
pub const THOST_FTDC_CHS_Cancel: u8 = '4' as u8;
pub const THOST_FTDC_CHU_Unused: u8 = '0' as u8;
pub const THOST_FTDC_CHU_Used: u8 = '1' as u8;
pub const THOST_FTDC_CHU_Fail: u8 = '2' as u8;
pub const THOST_FTDC_BAO_ByAccProperty: u8 = '0' as u8;
pub const THOST_FTDC_BAO_ByFBTransfer: u8 = '1' as u8;
pub const THOST_FTDC_MBTS_ByInstrument: u8 = '0' as u8;
pub const THOST_FTDC_MBTS_ByDayInsPrc: u8 = '1' as u8;
pub const THOST_FTDC_MBTS_ByDayIns: u8 = '2' as u8;
pub const THOST_FTDC_OTP_NONE: u8 = '0' as u8;
pub const THOST_FTDC_OTP_TOTP: u8 = '1' as u8;
pub const THOST_FTDC_OTPS_Unused: u8 = '0' as u8;
pub const THOST_FTDC_OTPS_Used: u8 = '1' as u8;
pub const THOST_FTDC_OTPS_Disuse: u8 = '2' as u8;
pub const THOST_FTDC_BUT_Investor: u8 = '1' as u8;
pub const THOST_FTDC_BUT_BrokerUser: u8 = '2' as u8;
pub const THOST_FTDC_FUTT_Commodity: u8 = '1' as u8;
pub const THOST_FTDC_FUTT_Financial: u8 = '2' as u8;
pub const THOST_FTDC_FET_Restriction: u8 = '0' as u8;
pub const THOST_FTDC_FET_TodayRestriction: u8 = '1' as u8;
pub const THOST_FTDC_FET_Transfer: u8 = '2' as u8;
pub const THOST_FTDC_FET_Credit: u8 = '3' as u8;
pub const THOST_FTDC_FET_InvestorWithdrawAlm: u8 = '4' as u8;
pub const THOST_FTDC_FET_BankRestriction: u8 = '5' as u8;
pub const THOST_FTDC_FET_Accountregister: u8 = '6' as u8;
pub const THOST_FTDC_FET_ExchangeFundIO: u8 = '7' as u8;
pub const THOST_FTDC_FET_InvestorFundIO: u8 = '8' as u8;
pub const THOST_FTDC_AST_FBTransfer: u8 = '0' as u8;
pub const THOST_FTDC_AST_ManualEntry: u8 = '1' as u8;
pub const THOST_FTDC_CST_UnifyAccount: u8 = '0' as u8;
pub const THOST_FTDC_CST_ManualEntry: u8 = '1' as u8;
pub const THOST_FTDC_UR_All: u8 = '0' as u8;
pub const THOST_FTDC_UR_Single: u8 = '1' as u8;
pub const THOST_FTDC_BG_Investor: u8 = '2' as u8;
pub const THOST_FTDC_BG_Group: u8 = '1' as u8;
pub const THOST_FTDC_TSSM_Instrument: u8 = '1' as u8;
pub const THOST_FTDC_TSSM_Product: u8 = '2' as u8;
pub const THOST_FTDC_TSSM_Exchange: u8 = '3' as u8;
pub const THOST_FTDC_ESM_Relative: u8 = '1' as u8;
pub const THOST_FTDC_ESM_Typical: u8 = '2' as u8;
pub const THOST_FTDC_RIR_All: u8 = '1' as u8;
pub const THOST_FTDC_RIR_Model: u8 = '2' as u8;
pub const THOST_FTDC_RIR_Single: u8 = '3' as u8;
pub const THOST_FTDC_SDS_Initialize: u8 = '0' as u8;
pub const THOST_FTDC_SDS_Settlementing: u8 = '1' as u8;
pub const THOST_FTDC_SDS_Settlemented: u8 = '2' as u8;
pub const THOST_FTDC_TSRC_NORMAL: u8 = '0' as u8;
pub const THOST_FTDC_TSRC_QUERY: u8 = '1' as u8;
pub const THOST_FTDC_FSM_Product: u8 = '1' as u8;
pub const THOST_FTDC_FSM_Exchange: u8 = '2' as u8;
pub const THOST_FTDC_FSM_All: u8 = '3' as u8;
pub const THOST_FTDC_BIR_Property: u8 = '1' as u8;
pub const THOST_FTDC_BIR_All: u8 = '2' as u8;
pub const THOST_FTDC_PIR_All: u8 = '1' as u8;
pub const THOST_FTDC_PIR_Property: u8 = '2' as u8;
pub const THOST_FTDC_PIR_Single: u8 = '3' as u8;
pub const THOST_FTDC_FIS_NoCreate: u8 = '0' as u8;
pub const THOST_FTDC_FIS_Created: u8 = '1' as u8;
pub const THOST_FTDC_FIS_Failed: u8 = '2' as u8;
pub const THOST_FTDC_FGS_FileTransmit: u8 = '0' as u8;
pub const THOST_FTDC_FGS_FileGen: u8 = '1' as u8;
pub const THOST_FTDC_SoM_Add: u8 = '1' as u8;
pub const THOST_FTDC_SoM_Update: u8 = '2' as u8;
pub const THOST_FTDC_SoM_Delete: u8 = '3' as u8;
pub const THOST_FTDC_SoM_Copy: u8 = '4' as u8;
pub const THOST_FTDC_SoM_AcTive: u8 = '5' as u8;
pub const THOST_FTDC_SoM_CanCel: u8 = '6' as u8;
pub const THOST_FTDC_SoM_ReSet: u8 = '7' as u8;
pub const THOST_FTDC_SoT_UpdatePassword: u8 = '0' as u8;
pub const THOST_FTDC_SoT_UserDepartment: u8 = '1' as u8;
pub const THOST_FTDC_SoT_RoleManager: u8 = '2' as u8;
pub const THOST_FTDC_SoT_RoleFunction: u8 = '3' as u8;
pub const THOST_FTDC_SoT_BaseParam: u8 = '4' as u8;
pub const THOST_FTDC_SoT_SetUserID: u8 = '5' as u8;
pub const THOST_FTDC_SoT_SetUserRole: u8 = '6' as u8;
pub const THOST_FTDC_SoT_UserIpRestriction: u8 = '7' as u8;
pub const THOST_FTDC_SoT_DepartmentManager: u8 = '8' as u8;
pub const THOST_FTDC_SoT_DepartmentCopy: u8 = '9' as u8;
pub const THOST_FTDC_SoT_Tradingcode: u8 = 'A' as u8;
pub const THOST_FTDC_SoT_InvestorStatus: u8 = 'B' as u8;
pub const THOST_FTDC_SoT_InvestorAuthority: u8 = 'C' as u8;
pub const THOST_FTDC_SoT_PropertySet: u8 = 'D' as u8;
pub const THOST_FTDC_SoT_ReSetInvestorPasswd: u8 = 'E' as u8;
pub const THOST_FTDC_SoT_InvestorPersonalityInfo: u8 = 'F' as u8;
pub const THOST_FTDC_CSRCQ_Current: u8 = '0' as u8;
pub const THOST_FTDC_CSRCQ_History: u8 = '1' as u8;
pub const THOST_FTDC_FRS_Normal: u8 = '1' as u8;
pub const THOST_FTDC_FRS_Freeze: u8 = '0' as u8;
pub const THOST_FTDC_STST_Standard: u8 = '0' as u8;
pub const THOST_FTDC_STST_NonStandard: u8 = '1' as u8;
pub const THOST_FTDC_RPT_Freeze: u8 = '1' as u8;
pub const THOST_FTDC_RPT_FreezeActive: u8 = '2' as u8;
pub const THOST_FTDC_RPT_OpenLimit: u8 = '3' as u8;
pub const THOST_FTDC_RPT_RelieveOpenLimit: u8 = '4' as u8;
pub const THOST_FTDC_AMLDS_Normal: u8 = '0' as u8;
pub const THOST_FTDC_AMLDS_Deleted: u8 = '1' as u8;
pub const THOST_FTDC_AMLCHS_Init: u8 = '0' as u8;
pub const THOST_FTDC_AMLCHS_Checking: u8 = '1' as u8;
pub const THOST_FTDC_AMLCHS_Checked: u8 = '2' as u8;
pub const THOST_FTDC_AMLCHS_RefuseReport: u8 = '3' as u8;
pub const THOST_FTDC_AMLDT_DrawDay: u8 = '0' as u8;
pub const THOST_FTDC_AMLDT_TouchDay: u8 = '1' as u8;
pub const THOST_FTDC_AMLCL_CheckLevel0: u8 = '0' as u8;
pub const THOST_FTDC_AMLCL_CheckLevel1: u8 = '1' as u8;
pub const THOST_FTDC_AMLCL_CheckLevel2: u8 = '2' as u8;
pub const THOST_FTDC_AMLCL_CheckLevel3: u8 = '3' as u8;
pub const THOST_FTDC_EFT_CSV: u8 = '0' as u8;
pub const THOST_FTDC_EFT_EXCEL: u8 = '1' as u8;
pub const THOST_FTDC_EFT_DBF: u8 = '2' as u8;
pub const THOST_FTDC_SMT_Before: u8 = '1' as u8;
pub const THOST_FTDC_SMT_Settlement: u8 = '2' as u8;
pub const THOST_FTDC_SMT_After: u8 = '3' as u8;
pub const THOST_FTDC_SMT_Settlemented: u8 = '4' as u8;
pub const THOST_FTDC_SML_Must: u8 = '1' as u8;
pub const THOST_FTDC_SML_Alarm: u8 = '2' as u8;
pub const THOST_FTDC_SML_Prompt: u8 = '3' as u8;
pub const THOST_FTDC_SML_Ignore: u8 = '4' as u8;
pub const THOST_FTDC_SMG_Exhcange: u8 = '1' as u8;
pub const THOST_FTDC_SMG_ASP: u8 = '2' as u8;
pub const THOST_FTDC_SMG_CSRC: u8 = '3' as u8;
pub const THOST_FTDC_LUT_Repeatable: u8 = '1' as u8;
pub const THOST_FTDC_LUT_Unrepeatable: u8 = '2' as u8;
pub const THOST_FTDC_DAR_Settle: u8 = '1' as u8;
pub const THOST_FTDC_DAR_Exchange: u8 = '2' as u8;
pub const THOST_FTDC_DAR_CSRC: u8 = '3' as u8;
pub const THOST_FTDC_MGT_ExchMarginRate: u8 = '0' as u8;
pub const THOST_FTDC_MGT_InstrMarginRate: u8 = '1' as u8;
pub const THOST_FTDC_MGT_InstrMarginRateTrade: u8 = '2' as u8;
pub const THOST_FTDC_ACT_Intraday: u8 = '1' as u8;
pub const THOST_FTDC_ACT_Long: u8 = '2' as u8;
pub const THOST_FTDC_MRT_Exchange: u8 = '1' as u8;
pub const THOST_FTDC_MRT_Investor: u8 = '2' as u8;
pub const THOST_FTDC_MRT_InvestorTrade: u8 = '3' as u8;
pub const THOST_FTDC_BUS_UnBak: u8 = '0' as u8;
pub const THOST_FTDC_BUS_BakUp: u8 = '1' as u8;
pub const THOST_FTDC_BUS_BakUped: u8 = '2' as u8;
pub const THOST_FTDC_BUS_BakFail: u8 = '3' as u8;
pub const THOST_FTDC_SIS_UnInitialize: u8 = '0' as u8;
pub const THOST_FTDC_SIS_Initialize: u8 = '1' as u8;
pub const THOST_FTDC_SIS_Initialized: u8 = '2' as u8;
pub const THOST_FTDC_SRS_NoCreate: u8 = '0' as u8;
pub const THOST_FTDC_SRS_Create: u8 = '1' as u8;
pub const THOST_FTDC_SRS_Created: u8 = '2' as u8;
pub const THOST_FTDC_SRS_CreateFail: u8 = '3' as u8;
pub const THOST_FTDC_SSS_UnSaveData: u8 = '0' as u8;
pub const THOST_FTDC_SSS_SaveDatad: u8 = '1' as u8;
pub const THOST_FTDC_SAS_UnArchived: u8 = '0' as u8;
pub const THOST_FTDC_SAS_Archiving: u8 = '1' as u8;
pub const THOST_FTDC_SAS_Archived: u8 = '2' as u8;
pub const THOST_FTDC_SAS_ArchiveFail: u8 = '3' as u8;
pub const THOST_FTDC_CTPT_Unkown: u8 = '0' as u8;
pub const THOST_FTDC_CTPT_MainCenter: u8 = '1' as u8;
pub const THOST_FTDC_CTPT_BackUp: u8 = '2' as u8;
pub const THOST_FTDC_CDT_Normal: u8 = '0' as u8;
pub const THOST_FTDC_CDT_SpecFirst: u8 = '1' as u8;
pub const THOST_FTDC_MFUR_None: u8 = '0' as u8;
pub const THOST_FTDC_MFUR_Margin: u8 = '1' as u8;
pub const THOST_FTDC_MFUR_All: u8 = '2' as u8;
pub const THOST_FTDC_MFUR_CNY3: u8 = '3' as u8;
pub const THOST_FTDC_SPT_CzceHedge: u8 = '1' as u8;
pub const THOST_FTDC_SPT_IneForeignCurrency: u8 = '2' as u8;
pub const THOST_FTDC_SPT_DceOpenClose: u8 = '3' as u8;
pub const THOST_FTDC_FMT_Mortgage: u8 = '1' as u8;
pub const THOST_FTDC_FMT_Redemption: u8 = '2' as u8;
pub const THOST_FTDC_ASPI_BaseMargin: u8 = '1' as u8;
pub const THOST_FTDC_ASPI_LowestInterest: u8 = '2' as u8;
pub const THOST_FTDC_FMD_In: u8 = '1' as u8;
pub const THOST_FTDC_FMD_Out: u8 = '2' as u8;
pub const THOST_FTDC_BT_Profit: u8 = '0' as u8;
pub const THOST_FTDC_BT_Loss: u8 = '1' as u8;
pub const THOST_FTDC_BT_Other: u8 = 'Z' as u8;
pub const THOST_FTDC_SST_Manual: u8 = '0' as u8;
pub const THOST_FTDC_SST_Automatic: u8 = '1' as u8;
pub const THOST_FTDC_CED_Settlement: u8 = '0' as u8;
pub const THOST_FTDC_CED_Sale: u8 = '1' as u8;
pub const THOST_FTDC_CSS_Entry: u8 = '1' as u8;
pub const THOST_FTDC_CSS_Approve: u8 = '2' as u8;
pub const THOST_FTDC_CSS_Refuse: u8 = '3' as u8;
pub const THOST_FTDC_CSS_Revoke: u8 = '4' as u8;
pub const THOST_FTDC_CSS_Send: u8 = '5' as u8;
pub const THOST_FTDC_CSS_Success: u8 = '6' as u8;
pub const THOST_FTDC_CSS_Failure: u8 = '7' as u8;
pub const THOST_FTDC_REQF_NoSend: u8 = '0' as u8;
pub const THOST_FTDC_REQF_SendSuccess: u8 = '1' as u8;
pub const THOST_FTDC_REQF_SendFailed: u8 = '2' as u8;
pub const THOST_FTDC_REQF_WaitReSend: u8 = '3' as u8;
pub const THOST_FTDC_RESF_Success: u8 = '0' as u8;
pub const THOST_FTDC_RESF_InsuffiCient: u8 = '1' as u8;
pub const THOST_FTDC_RESF_UnKnown: u8 = '8' as u8;
pub const THOST_FTDC_EXS_Before: u8 = '0' as u8;
pub const THOST_FTDC_EXS_After: u8 = '1' as u8;
pub const THOST_FTDC_CR_Domestic: u8 = '1' as u8;
pub const THOST_FTDC_CR_GMT: u8 = '2' as u8;
pub const THOST_FTDC_CR_Foreign: u8 = '3' as u8;
pub const THOST_FTDC_HB_No: u8 = '0' as u8;
pub const THOST_FTDC_HB_Yes: u8 = '1' as u8;
pub const THOST_FTDC_SM_Normal: u8 = '1' as u8;
pub const THOST_FTDC_SM_Emerge: u8 = '2' as u8;
pub const THOST_FTDC_SM_Restore: u8 = '3' as u8;
pub const THOST_FTDC_TPT_Full: u8 = '1' as u8;
pub const THOST_FTDC_TPT_Increment: u8 = '2' as u8;
pub const THOST_FTDC_TPT_BackUp: u8 = '3' as u8;
pub const THOST_FTDC_LM_Trade: u8 = '0' as u8;
pub const THOST_FTDC_LM_Transfer: u8 = '1' as u8;
pub const THOST_FTDC_CPT_Instrument: u8 = '1' as u8;
pub const THOST_FTDC_CPT_Margin: u8 = '2' as u8;
pub const THOST_FTDC_HT_Yes: u8 = '1' as u8;
pub const THOST_FTDC_HT_No: u8 = '0' as u8;
pub const THOST_FTDC_AMT_Bank: u8 = '1' as u8;
pub const THOST_FTDC_AMT_Securities: u8 = '2' as u8;
pub const THOST_FTDC_AMT_Fund: u8 = '3' as u8;
pub const THOST_FTDC_AMT_Insurance: u8 = '4' as u8;
pub const THOST_FTDC_AMT_Trust: u8 = '5' as u8;
pub const THOST_FTDC_AMT_Other: u8 = '9' as u8;
pub const THOST_FTDC_CFIOT_FundIO: u8 = '0' as u8;
pub const THOST_FTDC_CFIOT_SwapCurrency: u8 = '1' as u8;
pub const THOST_FTDC_CAT_Futures: u8 = '1' as u8;
pub const THOST_FTDC_CAT_AssetmgrFuture: u8 = '2' as u8;
pub const THOST_FTDC_CAT_AssetmgrTrustee: u8 = '3' as u8;
pub const THOST_FTDC_CAT_AssetmgrTransfer: u8 = '4' as u8;
pub const THOST_FTDC_LT_Chinese: u8 = '1' as u8;
pub const THOST_FTDC_LT_English: u8 = '2' as u8;
pub const THOST_FTDC_AMCT_Person: u8 = '1' as u8;
pub const THOST_FTDC_AMCT_Organ: u8 = '2' as u8;
pub const THOST_FTDC_AMCT_SpecialOrgan: u8 = '4' as u8;
pub const THOST_FTDC_ASST_Futures: u8 = '3' as u8;
pub const THOST_FTDC_ASST_SpecialOrgan: u8 = '4' as u8;
pub const THOST_FTDC_CIT_HasExch: u8 = '0' as u8;
pub const THOST_FTDC_CIT_HasATP: u8 = '1' as u8;
pub const THOST_FTDC_CIT_HasDiff: u8 = '2' as u8;
pub const THOST_FTDC_DT_HandDeliv: u8 = '1' as u8;
pub const THOST_FTDC_DT_PersonDeliv: u8 = '2' as u8;
pub const THOST_FTDC_MMSA_NO: u8 = '0' as u8;
pub const THOST_FTDC_MMSA_YES: u8 = '1' as u8;
pub const THOST_FTDC_CACT_Person: u8 = '0' as u8;
pub const THOST_FTDC_CACT_Company: u8 = '1' as u8;
pub const THOST_FTDC_CACT_Other: u8 = '2' as u8;
pub const THOST_FTDC_UOAAT_Futures: u8 = '1' as u8;
pub const THOST_FTDC_UOAAT_SpecialOrgan: u8 = '2' as u8;
pub const THOST_FTDC_DEN_Buy: u8 = '0' as u8;
pub const THOST_FTDC_DEN_Sell: u8 = '1' as u8;
pub const THOST_FTDC_OFEN_Open: u8 = '0' as u8;
pub const THOST_FTDC_OFEN_Close: u8 = '1' as u8;
pub const THOST_FTDC_OFEN_ForceClose: u8 = '2' as u8;
pub const THOST_FTDC_OFEN_CloseToday: u8 = '3' as u8;
pub const THOST_FTDC_OFEN_CloseYesterday: u8 = '4' as u8;
pub const THOST_FTDC_OFEN_ForceOff: u8 = '5' as u8;
pub const THOST_FTDC_OFEN_LocalForceClose: u8 = '6' as u8;
pub const THOST_FTDC_HFEN_Speculation: u8 = '1' as u8;
pub const THOST_FTDC_HFEN_Arbitrage: u8 = '2' as u8;
pub const THOST_FTDC_HFEN_Hedge: u8 = '3' as u8;
pub const THOST_FTDC_FIOTEN_FundIO: u8 = '1' as u8;
pub const THOST_FTDC_FIOTEN_Transfer: u8 = '2' as u8;
pub const THOST_FTDC_FIOTEN_SwapCurrency: u8 = '3' as u8;
pub const THOST_FTDC_FTEN_Deposite: u8 = '1' as u8;
pub const THOST_FTDC_FTEN_ItemFund: u8 = '2' as u8;
pub const THOST_FTDC_FTEN_Company: u8 = '3' as u8;
pub const THOST_FTDC_FTEN_InnerTransfer: u8 = '4' as u8;
pub const THOST_FTDC_FDEN_In: u8 = '1' as u8;
pub const THOST_FTDC_FDEN_Out: u8 = '2' as u8;
pub const THOST_FTDC_FMDEN_In: u8 = '1' as u8;
pub const THOST_FTDC_FMDEN_Out: u8 = '2' as u8;
pub const THOST_FTDC_CP_CallOptions: u8 = '1' as u8;
pub const THOST_FTDC_CP_PutOptions: u8 = '2' as u8;
pub const THOST_FTDC_STM_Continental: u8 = '0' as u8;
pub const THOST_FTDC_STM_American: u8 = '1' as u8;
pub const THOST_FTDC_STM_Bermuda: u8 = '2' as u8;
pub const THOST_FTDC_STT_Hedge: u8 = '0' as u8;
pub const THOST_FTDC_STT_Match: u8 = '1' as u8;
pub const THOST_FTDC_APPT_NotStrikeNum: u8 = '4' as u8;
pub const THOST_FTDC_GUDS_Gen: u8 = '0' as u8;
pub const THOST_FTDC_GUDS_Hand: u8 = '1' as u8;
pub const THOST_FTDC_OER_NoExec: u8 = 'n' as u8;
pub const THOST_FTDC_OER_Canceled: u8 = 'c' as u8;
pub const THOST_FTDC_OER_OK: u8 = '0' as u8;
pub const THOST_FTDC_OER_NoPosition: u8 = '1' as u8;
pub const THOST_FTDC_OER_NoDeposit: u8 = '2' as u8;
pub const THOST_FTDC_OER_NoParticipant: u8 = '3' as u8;
pub const THOST_FTDC_OER_NoClient: u8 = '4' as u8;
pub const THOST_FTDC_OER_NoInstrument: u8 = '6' as u8;
pub const THOST_FTDC_OER_NoRight: u8 = '7' as u8;
pub const THOST_FTDC_OER_InvalidVolume: u8 = '8' as u8;
pub const THOST_FTDC_OER_NoEnoughHistoryTrade: u8 = '9' as u8;
pub const THOST_FTDC_OER_Unknown: u8 = 'a' as u8;
pub const THOST_FTDC_COMBT_Future: u8 = '0' as u8;
pub const THOST_FTDC_COMBT_BUL: u8 = '1' as u8;
pub const THOST_FTDC_COMBT_BER: u8 = '2' as u8;
pub const THOST_FTDC_COMBT_STD: u8 = '3' as u8;
pub const THOST_FTDC_COMBT_STG: u8 = '4' as u8;
pub const THOST_FTDC_COMBT_PRT: u8 = '5' as u8;
pub const THOST_FTDC_COMBT_CAS: u8 = '6' as u8;
pub const THOST_FTDC_COMBT_OPL: u8 = '7' as u8;
pub const THOST_FTDC_COMBT_BFO: u8 = '8' as u8;
pub const THOST_FTDC_COMBT_BLS: u8 = '9' as u8;
pub const THOST_FTDC_COMBT_BES: u8 = 'a' as u8;
pub const THOST_FTDC_DCECOMBT_SPL: u8 = '0' as u8;
pub const THOST_FTDC_DCECOMBT_OPL: u8 = '1' as u8;
pub const THOST_FTDC_DCECOMBT_SP: u8 = '2' as u8;
pub const THOST_FTDC_DCECOMBT_SPC: u8 = '3' as u8;
pub const THOST_FTDC_DCECOMBT_BLS: u8 = '4' as u8;
pub const THOST_FTDC_DCECOMBT_BES: u8 = '5' as u8;
pub const THOST_FTDC_DCECOMBT_CAS: u8 = '6' as u8;
pub const THOST_FTDC_DCECOMBT_STD: u8 = '7' as u8;
pub const THOST_FTDC_DCECOMBT_STG: u8 = '8' as u8;
pub const THOST_FTDC_DCECOMBT_BFO: u8 = '9' as u8;
pub const THOST_FTDC_DCECOMBT_SFO: u8 = 'a' as u8;
pub const THOST_FTDC_ORPT_PreSettlementPrice: u8 = '1' as u8;
pub const THOST_FTDC_ORPT_OpenPrice: u8 = '4' as u8;
pub const THOST_FTDC_ORPT_MaxPreSettlementPrice: u8 = '5' as u8;
pub const THOST_FTDC_BLAG_Default: u8 = '1' as u8;
pub const THOST_FTDC_BLAG_IncludeOptValLost: u8 = '2' as u8;
pub const THOST_FTDC_ACTP_Exec: u8 = '1' as u8;
pub const THOST_FTDC_ACTP_Abandon: u8 = '2' as u8;
pub const THOST_FTDC_FQST_Submitted: u8 = 'a' as u8;
pub const THOST_FTDC_FQST_Accepted: u8 = 'b' as u8;
pub const THOST_FTDC_FQST_Rejected: u8 = 'c' as u8;
pub const THOST_FTDC_VM_Absolute: u8 = '0' as u8;
pub const THOST_FTDC_VM_Ratio: u8 = '1' as u8;
pub const THOST_FTDC_EOPF_Reserve: u8 = '0' as u8;
pub const THOST_FTDC_EOPF_UnReserve: u8 = '1' as u8;
pub const THOST_FTDC_EOCF_AutoClose: u8 = '0' as u8;
pub const THOST_FTDC_EOCF_NotToClose: u8 = '1' as u8;
pub const THOST_FTDC_PTE_Futures: u8 = '1' as u8;
pub const THOST_FTDC_PTE_Options: u8 = '2' as u8;
pub const THOST_FTDC_CUFN_CUFN_O: u8 = 'O' as u8;
pub const THOST_FTDC_CUFN_CUFN_T: u8 = 'T' as u8;
pub const THOST_FTDC_CUFN_CUFN_P: u8 = 'P' as u8;
pub const THOST_FTDC_CUFN_CUFN_N: u8 = 'N' as u8;
pub const THOST_FTDC_CUFN_CUFN_L: u8 = 'L' as u8;
pub const THOST_FTDC_CUFN_CUFN_F: u8 = 'F' as u8;
pub const THOST_FTDC_CUFN_CUFN_C: u8 = 'C' as u8;
pub const THOST_FTDC_CUFN_CUFN_M: u8 = 'M' as u8;
pub const THOST_FTDC_DUFN_DUFN_O: u8 = 'O' as u8;
pub const THOST_FTDC_DUFN_DUFN_T: u8 = 'T' as u8;
pub const THOST_FTDC_DUFN_DUFN_P: u8 = 'P' as u8;
pub const THOST_FTDC_DUFN_DUFN_F: u8 = 'F' as u8;
pub const THOST_FTDC_DUFN_DUFN_C: u8 = 'C' as u8;
pub const THOST_FTDC_DUFN_DUFN_D: u8 = 'D' as u8;
pub const THOST_FTDC_DUFN_DUFN_M: u8 = 'M' as u8;
pub const THOST_FTDC_DUFN_DUFN_S: u8 = 'S' as u8;
pub const THOST_FTDC_SUFN_SUFN_O: u8 = 'O' as u8;
pub const THOST_FTDC_SUFN_SUFN_T: u8 = 'T' as u8;
pub const THOST_FTDC_SUFN_SUFN_P: u8 = 'P' as u8;
pub const THOST_FTDC_SUFN_SUFN_F: u8 = 'F' as u8;
pub const THOST_FTDC_CFUFN_SUFN_T: u8 = 'T' as u8;
pub const THOST_FTDC_CFUFN_SUFN_P: u8 = 'P' as u8;
pub const THOST_FTDC_CFUFN_SUFN_F: u8 = 'F' as u8;
pub const THOST_FTDC_CFUFN_SUFN_S: u8 = 'S' as u8;
pub const THOST_FTDC_CMDR_Comb: u8 = '0' as u8;
pub const THOST_FTDC_CMDR_UnComb: u8 = '1' as u8;
pub const THOST_FTDC_CMDR_DelComb: u8 = '2' as u8;
pub const THOST_FTDC_STOV_RealValue: u8 = '1' as u8;
pub const THOST_FTDC_STOV_ProfitValue: u8 = '2' as u8;
pub const THOST_FTDC_STOV_RealRatio: u8 = '3' as u8;
pub const THOST_FTDC_STOV_ProfitRatio: u8 = '4' as u8;
pub const THOST_FTDC_ROAST_Processing: u8 = '0' as u8;
pub const THOST_FTDC_ROAST_Cancelled: u8 = '1' as u8;
pub const THOST_FTDC_ROAST_Opened: u8 = '2' as u8;
pub const THOST_FTDC_ROAST_Invalid: u8 = '3' as u8;
pub const THOST_FTDC_WPSR_Lib: u8 = '1' as u8;
pub const THOST_FTDC_WPSR_Manual: u8 = '2' as u8;
pub const THOST_FTDC_OSCF_CloseSelfOptionPosition: u8 = '1' as u8;
pub const THOST_FTDC_OSCF_ReserveOptionPosition: u8 = '2' as u8;
pub const THOST_FTDC_OSCF_SellCloseSelfFuturePosition: u8 = '3' as u8;
pub const THOST_FTDC_OSCF_ReserveFuturePosition: u8 = '4' as u8;
pub const THOST_FTDC_BZTP_Future: u8 = '1' as u8;
pub const THOST_FTDC_BZTP_Stock: u8 = '2' as u8;
pub const THOST_FTDC_APP_TYPE_Investor: u8 = '1' as u8;
pub const THOST_FTDC_APP_TYPE_InvestorRelay: u8 = '2' as u8;
pub const THOST_FTDC_APP_TYPE_OperatorRelay: u8 = '3' as u8;
pub const THOST_FTDC_APP_TYPE_UnKnown: u8 = '4' as u8;
pub const THOST_FTDC_RV_Right: u8 = '0' as u8;
pub const THOST_FTDC_RV_Refuse: u8 = '1' as u8;
pub const THOST_FTDC_OTC_TRDT_Block: u8 = '0' as u8;
pub const THOST_FTDC_OTC_TRDT_EFP: u8 = '1' as u8;
pub const THOST_FTDC_OTC_MT_DV01: u8 = '1' as u8;
pub const THOST_FTDC_OTC_MT_ParValue: u8 = '2' as u8;
pub const THOST_FTDC_AU_WHITE: u8 = '0' as u8;
pub const THOST_FTDC_AU_BLACK: u8 = '1' as u8;
pub const THOST_FTDC_INS_ALL: u8 = '0' as u8;
pub const THOST_FTDC_INS_FUTURE: u8 = '1' as u8;
pub const THOST_FTDC_INS_OPTION: u8 = '2' as u8;
pub const THOST_FTDC_INS_COMB: u8 = '3' as u8;
pub const THOST_FTDC_TD_ALL: u8 = '0' as u8;
pub const THOST_FTDC_TD_TRADE: u8 = '1' as u8;
pub const THOST_FTDC_TD_UNTRADE: u8 = '2' as u8;
pub const THOST_FTDC_PS_tradeable: u8 = '1' as u8;
pub const THOST_FTDC_PS_untradeable: u8 = '2' as u8;
pub const THOST_FTDC_SDS_Readable: u8 = '1' as u8;
pub const THOST_FTDC_SDS_Reading: u8 = '2' as u8;
pub const THOST_FTDC_SDS_Readend: u8 = '3' as u8;
pub const THOST_FTDC_SDS_OptErr: u8 = 'e' as u8;
pub const THOST_FTDC_ACD_Add: u8 = '1' as u8;
pub const THOST_FTDC_ACD_Del: u8 = '2' as u8;
pub const THOST_FTDC_ACD_Upd: u8 = '3' as u8;
pub const THOST_FTDC_OAC_Balance: u8 = '1' as u8;
pub const THOST_FTDC_OAC_OrigFirst: u8 = '2' as u8;
pub const THOST_FTDC_PLCL_None: u8 = '0' as u8;
pub const THOST_FTDC_PLCL_Product: u8 = '1' as u8;
pub const THOST_FTDC_PLCL_Inst: u8 = '2' as u8;
pub const THOST_FTDC_OFCL_None: u8 = '0' as u8;
pub const THOST_FTDC_OFCL_Product: u8 = '1' as u8;
pub const THOST_FTDC_OFCL_Inst: u8 = '2' as u8;
pub const THOST_FTDC_EBL_False: u8 = '0' as u8;
pub const THOST_FTDC_EBL_True: u8 = '1' as u8;
pub const THOST_FTDC_ETR_USUAL: u8 = '1' as u8;
pub const THOST_FTDC_ETR_FNSP: u8 = '2' as u8;
pub const THOST_FTDC_ETR_BNSP: u8 = '3' as u8;
pub const THOST_FTDC_ETR_SPOT: u8 = '4' as u8;
pub const THOST_FTDC_EPF_None: u8 = '0' as u8;
pub const THOST_FTDC_EPF_SPBM: u8 = '1' as u8;
pub const THOST_FTDC_EPF_RULE: u8 = '2' as u8;
pub const THOST_FTDC_EPF_SPMM: u8 = '3' as u8;
pub const THOST_FTDC_EPF_RCAMS: u8 = '4' as u8;
pub const THOST_FTDC_WDPID_CashIn: u8 = 'C' as u8;
pub const THOST_FTDC_ITR_CloseOnly: u8 = '1' as u8;
pub const THOST_FTDC_ITR_Forbidden: u8 = '2' as u8;
pub const THOST_FTDC_IMID_BothSide: u8 = '1' as u8;
pub const THOST_FTDC_IMID_MMSA: u8 = '2' as u8;
pub const THOST_FTDC_IMID_SPMM: u8 = '3' as u8;
pub const THOST_FTDC_ERComb_BUC: u8 = '0' as u8;
pub const THOST_FTDC_ERComb_BEC: u8 = '1' as u8;
pub const THOST_FTDC_ERComb_BEP: u8 = '2' as u8;
pub const THOST_FTDC_ERComb_BUP: u8 = '3' as u8;
pub const THOST_FTDC_ERComb_CAS: u8 = '4' as u8;
pub const THOST_FTDC_EET_None: u8 = '0' as u8;
pub const THOST_FTDC_EET_SPBM_AddOnHedge: u8 = '1' as u8;
pub const THOST_FTDC_EIC_Usual: u8 = '1' as u8;
pub const THOST_FTDC_EIC_Delivery: u8 = '2' as u8;
pub const THOST_FTDC_EIC_NonComb: u8 = '3' as u8;
pub const THOST_FTDC_PCF_None: u8 = '0' as u8;
pub const THOST_FTDC_PCF_OnlyFrozen: u8 = '1' as u8;
pub const THOST_FTDC_PCF_PositionChange: u8 = '2' as u8;
pub const THOST_FTDC_PRS_Init: u8 = '0' as u8;
pub const THOST_FTDC_PRS_Sync: u8 = '1' as u8;
pub const THOST_FTDC_PRS_UserUpd: u8 = '2' as u8;
pub const THOST_FTDC_PRS_SuperUserUpd: u8 = '3' as u8;
pub const THOST_FTDC_PRS_SecUpd: u8 = '4' as u8;
pub const THOST_FTDC_ASM_Trade: u8 = '0' as u8;
pub const THOST_FTDC_ASM_MarketData: u8 = '1' as u8;
pub const THOST_FTDC_ASM_Other: u8 = '2' as u8;
pub const THOST_FTDC_ADV_V4: u8 = '0' as u8;
pub const THOST_FTDC_ADV_V6: u8 = '1' as u8;
pub const THOST_FTDC_TGQS_QryIdle: u8 = '1' as u8;
pub const THOST_FTDC_TGQS_QryBusy: u8 = '2' as u8;

unsafe impl Send for MdApi {}
unsafe impl Sync for MdApi {}

impl MdApi {
    pub fn CreateMdApiAndSpi(tx: Sender<MdSpiMsg>, flow_path: String) -> UniquePtr<MdApi> {
        if !Path::new(&flow_path).exists() {
            create_dir_all(&flow_path).unwrap();
        }
        let spi = Arc::pin(MdSpi { tx });
        let api = CreateMdApi(&spi, flow_path);
        forget(spi); // 让 spi 一直存在，防止被释放
        api
    }
}

#[derive(Debug, Clone)]
pub enum MdSpiMsg {
    OnFrontConnected,
    OnFrontDisconnected(i32),
    OnHeartBeatWarning(i32),
    OnRspUserLogin(RspUserLoginField, RspInfoField, i32, bool),
    OnRspUserLogout(UserLogoutField, RspInfoField, i32, bool),
    OnRspQryMulticastInstrument(MulticastInstrumentField, RspInfoField, i32, bool),
    OnRspError(RspInfoField, i32, bool),
    OnRspSubMarketData(SpecificInstrumentField, RspInfoField, i32, bool),
    OnRspUnSubMarketData(SpecificInstrumentField, RspInfoField, i32, bool),
    OnRspSubForQuoteRsp(SpecificInstrumentField, RspInfoField, i32, bool),
    OnRspUnSubForQuoteRsp(SpecificInstrumentField, RspInfoField, i32, bool),
    OnRtnDepthMarketData(DepthMarketDataField),
    OnRtnForQuoteRsp(ForQuoteRspField),
}

pub struct MdSpi {
    tx: Sender<MdSpiMsg>,
}

impl MdSpi {
pub fn OnFrontConnected(&self) { self.tx.send(MdSpiMsg::OnFrontConnected).expect("sending MdSpiMsg failed"); }
pub fn OnFrontDisconnected(&self, nReason: i32) { self.tx.send(MdSpiMsg::OnFrontDisconnected(nReason)).expect("sending MdSpiMsg failed"); }
pub fn OnHeartBeatWarning(&self, nTimeLapse: i32) { self.tx.send(MdSpiMsg::OnHeartBeatWarning(nTimeLapse)).expect("sending MdSpiMsg failed"); }
pub fn OnRspUserLogin(&self, pRspUserLogin: RspUserLoginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUserLogin(pRspUserLogin, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspUserLogout(&self, pUserLogout: UserLogoutField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUserLogout(pUserLogout, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspQryMulticastInstrument(&self, pMulticastInstrument: MulticastInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspQryMulticastInstrument(pMulticastInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspError(&self, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspError(pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspSubMarketData(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspSubMarketData(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspUnSubMarketData(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUnSubMarketData(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspSubForQuoteRsp(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspSubForQuoteRsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRspUnSubForQuoteRsp(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUnSubForQuoteRsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending MdSpiMsg failed"); }
pub fn OnRtnDepthMarketData(&self, pDepthMarketData: DepthMarketDataField) { self.tx.send(MdSpiMsg::OnRtnDepthMarketData(pDepthMarketData)).expect("sending MdSpiMsg failed"); }
pub fn OnRtnForQuoteRsp(&self, pForQuoteRsp: ForQuoteRspField) { self.tx.send(MdSpiMsg::OnRtnForQuoteRsp(pForQuoteRsp)).expect("sending MdSpiMsg failed"); }
}

unsafe impl Send for TraderApi {}
unsafe impl Sync for TraderApi {}

impl TraderApi {
    pub fn CreateTraderApiAndSpi(tx: Sender<TraderSpiMsg>, flow_path: String) -> UniquePtr<TraderApi> {
        if !Path::new(&flow_path).exists() {
            create_dir_all(&flow_path).unwrap();
        }
        let spi = Arc::pin(TraderSpi { tx });
        let api = CreateTraderApi(&spi, flow_path);
        forget(spi); // 让 spi 一直存在，防止被释放
        api
    }
}

#[derive(Debug, Clone)]
pub enum TraderSpiMsg {
    OnFrontConnected,
    OnFrontDisconnected(i32),
    OnHeartBeatWarning(i32),
    OnRspAuthenticate(RspAuthenticateField, RspInfoField, i32, bool),
    OnRspUserLogin(RspUserLoginField, RspInfoField, i32, bool),
    OnRspUserLogout(UserLogoutField, RspInfoField, i32, bool),
    OnRspUserPasswordUpdate(UserPasswordUpdateField, RspInfoField, i32, bool),
    OnRspTradingAccountPasswordUpdate(TradingAccountPasswordUpdateField, RspInfoField, i32, bool),
    OnRspUserAuthMethod(RspUserAuthMethodField, RspInfoField, i32, bool),
    OnRspGenUserCaptcha(RspGenUserCaptchaField, RspInfoField, i32, bool),
    OnRspGenUserText(RspGenUserTextField, RspInfoField, i32, bool),
    OnRspOrderInsert(InputOrderField, RspInfoField, i32, bool),
    OnRspParkedOrderInsert(ParkedOrderField, RspInfoField, i32, bool),
    OnRspParkedOrderAction(ParkedOrderActionField, RspInfoField, i32, bool),
    OnRspOrderAction(InputOrderActionField, RspInfoField, i32, bool),
    OnRspQryMaxOrderVolume(QryMaxOrderVolumeField, RspInfoField, i32, bool),
    OnRspSettlementInfoConfirm(SettlementInfoConfirmField, RspInfoField, i32, bool),
    OnRspRemoveParkedOrder(RemoveParkedOrderField, RspInfoField, i32, bool),
    OnRspRemoveParkedOrderAction(RemoveParkedOrderActionField, RspInfoField, i32, bool),
    OnRspExecOrderInsert(InputExecOrderField, RspInfoField, i32, bool),
    OnRspExecOrderAction(InputExecOrderActionField, RspInfoField, i32, bool),
    OnRspForQuoteInsert(InputForQuoteField, RspInfoField, i32, bool),
    OnRspQuoteInsert(InputQuoteField, RspInfoField, i32, bool),
    OnRspQuoteAction(InputQuoteActionField, RspInfoField, i32, bool),
    OnRspBatchOrderAction(InputBatchOrderActionField, RspInfoField, i32, bool),
    OnRspOptionSelfCloseInsert(InputOptionSelfCloseField, RspInfoField, i32, bool),
    OnRspOptionSelfCloseAction(InputOptionSelfCloseActionField, RspInfoField, i32, bool),
    OnRspCombActionInsert(InputCombActionField, RspInfoField, i32, bool),
    OnRspQryOrder(OrderField, RspInfoField, i32, bool),
    OnRspQryTrade(TradeField, RspInfoField, i32, bool),
    OnRspQryInvestorPosition(InvestorPositionField, RspInfoField, i32, bool),
    OnRspQryTradingAccount(TradingAccountField, RspInfoField, i32, bool),
    OnRspQryInvestor(InvestorField, RspInfoField, i32, bool),
    OnRspQryTradingCode(TradingCodeField, RspInfoField, i32, bool),
    OnRspQryInstrumentMarginRate(InstrumentMarginRateField, RspInfoField, i32, bool),
    OnRspQryInstrumentCommissionRate(InstrumentCommissionRateField, RspInfoField, i32, bool),
    OnRspQryExchange(ExchangeField, RspInfoField, i32, bool),
    OnRspQryProduct(ProductField, RspInfoField, i32, bool),
    OnRspQryInstrument(InstrumentField, RspInfoField, i32, bool),
    OnRspQryDepthMarketData(DepthMarketDataField, RspInfoField, i32, bool),
    OnRspQryTraderOffer(TraderOfferField, RspInfoField, i32, bool),
    OnRspQrySettlementInfo(SettlementInfoField, RspInfoField, i32, bool),
    OnRspQryTransferBank(TransferBankField, RspInfoField, i32, bool),
    OnRspQryInvestorPositionDetail(InvestorPositionDetailField, RspInfoField, i32, bool),
    OnRspQryNotice(NoticeField, RspInfoField, i32, bool),
    OnRspQrySettlementInfoConfirm(SettlementInfoConfirmField, RspInfoField, i32, bool),
    OnRspQryInvestorPositionCombineDetail(InvestorPositionCombineDetailField, RspInfoField, i32, bool),
    OnRspQryCFMMCTradingAccountKey(CFMMCTradingAccountKeyField, RspInfoField, i32, bool),
    OnRspQryEWarrantOffset(EWarrantOffsetField, RspInfoField, i32, bool),
    OnRspQryInvestorProductGroupMargin(InvestorProductGroupMarginField, RspInfoField, i32, bool),
    OnRspQryExchangeMarginRate(ExchangeMarginRateField, RspInfoField, i32, bool),
    OnRspQryExchangeMarginRateAdjust(ExchangeMarginRateAdjustField, RspInfoField, i32, bool),
    OnRspQryExchangeRate(ExchangeRateField, RspInfoField, i32, bool),
    OnRspQrySecAgentACIDMap(SecAgentACIDMapField, RspInfoField, i32, bool),
    OnRspQryProductExchRate(ProductExchRateField, RspInfoField, i32, bool),
    OnRspQryProductGroup(ProductGroupField, RspInfoField, i32, bool),
    OnRspQryMMInstrumentCommissionRate(MMInstrumentCommissionRateField, RspInfoField, i32, bool),
    OnRspQryMMOptionInstrCommRate(MMOptionInstrCommRateField, RspInfoField, i32, bool),
    OnRspQryInstrumentOrderCommRate(InstrumentOrderCommRateField, RspInfoField, i32, bool),
    OnRspQrySecAgentTradingAccount(TradingAccountField, RspInfoField, i32, bool),
    OnRspQrySecAgentCheckMode(SecAgentCheckModeField, RspInfoField, i32, bool),
    OnRspQrySecAgentTradeInfo(SecAgentTradeInfoField, RspInfoField, i32, bool),
    OnRspQryOptionInstrTradeCost(OptionInstrTradeCostField, RspInfoField, i32, bool),
    OnRspQryOptionInstrCommRate(OptionInstrCommRateField, RspInfoField, i32, bool),
    OnRspQryExecOrder(ExecOrderField, RspInfoField, i32, bool),
    OnRspQryForQuote(ForQuoteField, RspInfoField, i32, bool),
    OnRspQryQuote(QuoteField, RspInfoField, i32, bool),
    OnRspQryOptionSelfClose(OptionSelfCloseField, RspInfoField, i32, bool),
    OnRspQryInvestUnit(InvestUnitField, RspInfoField, i32, bool),
    OnRspQryCombInstrumentGuard(CombInstrumentGuardField, RspInfoField, i32, bool),
    OnRspQryCombAction(CombActionField, RspInfoField, i32, bool),
    OnRspQryTransferSerial(TransferSerialField, RspInfoField, i32, bool),
    OnRspQryAccountregister(AccountregisterField, RspInfoField, i32, bool),
    OnRspError(RspInfoField, i32, bool),
    OnRtnOrder(OrderField),
    OnRtnTrade(TradeField),
    OnErrRtnOrderInsert(InputOrderField, RspInfoField),
    OnErrRtnOrderAction(OrderActionField, RspInfoField),
    OnRtnInstrumentStatus(InstrumentStatusField),
    OnRtnBulletin(BulletinField),
    OnRtnTradingNotice(TradingNoticeInfoField),
    OnRtnErrorConditionalOrder(ErrorConditionalOrderField),
    OnRtnExecOrder(ExecOrderField),
    OnErrRtnExecOrderInsert(InputExecOrderField, RspInfoField),
    OnErrRtnExecOrderAction(ExecOrderActionField, RspInfoField),
    OnErrRtnForQuoteInsert(InputForQuoteField, RspInfoField),
    OnRtnQuote(QuoteField),
    OnErrRtnQuoteInsert(InputQuoteField, RspInfoField),
    OnErrRtnQuoteAction(QuoteActionField, RspInfoField),
    OnRtnForQuoteRsp(ForQuoteRspField),
    OnRtnCFMMCTradingAccountToken(CFMMCTradingAccountTokenField),
    OnErrRtnBatchOrderAction(BatchOrderActionField, RspInfoField),
    OnRtnOptionSelfClose(OptionSelfCloseField),
    OnErrRtnOptionSelfCloseInsert(InputOptionSelfCloseField, RspInfoField),
    OnErrRtnOptionSelfCloseAction(OptionSelfCloseActionField, RspInfoField),
    OnRtnCombAction(CombActionField),
    OnErrRtnCombActionInsert(InputCombActionField, RspInfoField),
    OnRspQryContractBank(ContractBankField, RspInfoField, i32, bool),
    OnRspQryParkedOrder(ParkedOrderField, RspInfoField, i32, bool),
    OnRspQryParkedOrderAction(ParkedOrderActionField, RspInfoField, i32, bool),
    OnRspQryTradingNotice(TradingNoticeField, RspInfoField, i32, bool),
    OnRspQryBrokerTradingParams(BrokerTradingParamsField, RspInfoField, i32, bool),
    OnRspQryBrokerTradingAlgos(BrokerTradingAlgosField, RspInfoField, i32, bool),
    OnRspQueryCFMMCTradingAccountToken(QueryCFMMCTradingAccountTokenField, RspInfoField, i32, bool),
    OnRtnFromBankToFutureByBank(RspTransferField),
    OnRtnFromFutureToBankByBank(RspTransferField),
    OnRtnRepealFromBankToFutureByBank(RspRepealField),
    OnRtnRepealFromFutureToBankByBank(RspRepealField),
    OnRtnFromBankToFutureByFuture(RspTransferField),
    OnRtnFromFutureToBankByFuture(RspTransferField),
    OnRtnRepealFromBankToFutureByFutureManual(RspRepealField),
    OnRtnRepealFromFutureToBankByFutureManual(RspRepealField),
    OnRtnQueryBankBalanceByFuture(NotifyQueryAccountField),
    OnErrRtnBankToFutureByFuture(ReqTransferField, RspInfoField),
    OnErrRtnFutureToBankByFuture(ReqTransferField, RspInfoField),
    OnErrRtnRepealBankToFutureByFutureManual(ReqRepealField, RspInfoField),
    OnErrRtnRepealFutureToBankByFutureManual(ReqRepealField, RspInfoField),
    OnErrRtnQueryBankBalanceByFuture(ReqQueryAccountField, RspInfoField),
    OnRtnRepealFromBankToFutureByFuture(RspRepealField),
    OnRtnRepealFromFutureToBankByFuture(RspRepealField),
    OnRspFromBankToFutureByFuture(ReqTransferField, RspInfoField, i32, bool),
    OnRspFromFutureToBankByFuture(ReqTransferField, RspInfoField, i32, bool),
    OnRspQueryBankAccountMoneyByFuture(ReqQueryAccountField, RspInfoField, i32, bool),
    OnRtnOpenAccountByBank(OpenAccountField),
    OnRtnCancelAccountByBank(CancelAccountField),
    OnRtnChangeAccountByBank(ChangeAccountField),
    OnRspQryClassifiedInstrument(InstrumentField, RspInfoField, i32, bool),
    OnRspQryCombPromotionParam(CombPromotionParamField, RspInfoField, i32, bool),
    OnRspQryRiskSettleInvstPosition(RiskSettleInvstPositionField, RspInfoField, i32, bool),
    OnRspQryRiskSettleProductStatus(RiskSettleProductStatusField, RspInfoField, i32, bool),
    OnRspQrySPBMFutureParameter(SPBMFutureParameterField, RspInfoField, i32, bool),
    OnRspQrySPBMOptionParameter(SPBMOptionParameterField, RspInfoField, i32, bool),
    OnRspQrySPBMIntraParameter(SPBMIntraParameterField, RspInfoField, i32, bool),
    OnRspQrySPBMInterParameter(SPBMInterParameterField, RspInfoField, i32, bool),
    OnRspQrySPBMPortfDefinition(SPBMPortfDefinitionField, RspInfoField, i32, bool),
    OnRspQrySPBMInvestorPortfDef(SPBMInvestorPortfDefField, RspInfoField, i32, bool),
    OnRspQryInvestorPortfMarginRatio(InvestorPortfMarginRatioField, RspInfoField, i32, bool),
    OnRspQryInvestorProdSPBMDetail(InvestorProdSPBMDetailField, RspInfoField, i32, bool),
    OnRspQryInvestorCommoditySPMMMargin(InvestorCommoditySPMMMarginField, RspInfoField, i32, bool),
    OnRspQryInvestorCommodityGroupSPMMMargin(InvestorCommodityGroupSPMMMarginField, RspInfoField, i32, bool),
    OnRspQrySPMMInstParam(SPMMInstParamField, RspInfoField, i32, bool),
    OnRspQrySPMMProductParam(SPMMProductParamField, RspInfoField, i32, bool),
    OnRspQrySPBMAddOnInterParameter(SPBMAddOnInterParameterField, RspInfoField, i32, bool),
    OnRspQryRCAMSCombProductInfo(RCAMSCombProductInfoField, RspInfoField, i32, bool),
    OnRspQryRCAMSInstrParameter(RCAMSInstrParameterField, RspInfoField, i32, bool),
    OnRspQryRCAMSIntraParameter(RCAMSIntraParameterField, RspInfoField, i32, bool),
    OnRspQryRCAMSInterParameter(RCAMSInterParameterField, RspInfoField, i32, bool),
    OnRspQryRCAMSShortOptAdjustParam(RCAMSShortOptAdjustParamField, RspInfoField, i32, bool),
    OnRspQryRCAMSInvestorCombPosition(RCAMSInvestorCombPositionField, RspInfoField, i32, bool),
    OnRspQryInvestorProdRCAMSMargin(InvestorProdRCAMSMarginField, RspInfoField, i32, bool),
    OnRspQryRULEInstrParameter(RULEInstrParameterField, RspInfoField, i32, bool),
    OnRspQryRULEIntraParameter(RULEIntraParameterField, RspInfoField, i32, bool),
    OnRspQryRULEInterParameter(RULEInterParameterField, RspInfoField, i32, bool),
    OnRspQryInvestorProdRULEMargin(InvestorProdRULEMarginField, RspInfoField, i32, bool),
    OnRspQryInvestorPortfSetting(InvestorPortfSettingField, RspInfoField, i32, bool),
    OnRspQryInvestorInfoCommRec(InvestorInfoCommRecField, RspInfoField, i32, bool),
}

pub struct TraderSpi {
    tx: Sender<TraderSpiMsg>,
}

impl TraderSpi {
pub fn OnFrontConnected(&self) { self.tx.send(TraderSpiMsg::OnFrontConnected).expect("sending TraderSpiMsg failed"); }
pub fn OnFrontDisconnected(&self, nReason: i32) { self.tx.send(TraderSpiMsg::OnFrontDisconnected(nReason)).expect("sending TraderSpiMsg failed"); }
pub fn OnHeartBeatWarning(&self, nTimeLapse: i32) { self.tx.send(TraderSpiMsg::OnHeartBeatWarning(nTimeLapse)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspAuthenticate(&self, pRspAuthenticateField: RspAuthenticateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspAuthenticate(pRspAuthenticateField, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspUserLogin(&self, pRspUserLogin: RspUserLoginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserLogin(pRspUserLogin, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspUserLogout(&self, pUserLogout: UserLogoutField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserLogout(pUserLogout, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspUserPasswordUpdate(&self, pUserPasswordUpdate: UserPasswordUpdateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserPasswordUpdate(pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspTradingAccountPasswordUpdate(&self, pTradingAccountPasswordUpdate: TradingAccountPasswordUpdateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspTradingAccountPasswordUpdate(pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspUserAuthMethod(&self, pRspUserAuthMethod: RspUserAuthMethodField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserAuthMethod(pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspGenUserCaptcha(&self, pRspGenUserCaptcha: RspGenUserCaptchaField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspGenUserCaptcha(pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspGenUserText(&self, pRspGenUserText: RspGenUserTextField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspGenUserText(pRspGenUserText, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspOrderInsert(&self, pInputOrder: InputOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOrderInsert(pInputOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspParkedOrderInsert(&self, pParkedOrder: ParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspParkedOrderInsert(pParkedOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspParkedOrderAction(pParkedOrderAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspOrderAction(&self, pInputOrderAction: InputOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOrderAction(pInputOrderAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryMaxOrderVolume(&self, pQryMaxOrderVolume: QryMaxOrderVolumeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryMaxOrderVolume(pQryMaxOrderVolume, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspSettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspSettlementInfoConfirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspRemoveParkedOrder(&self, pRemoveParkedOrder: RemoveParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspRemoveParkedOrder(pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspRemoveParkedOrderAction(&self, pRemoveParkedOrderAction: RemoveParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspRemoveParkedOrderAction(pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspExecOrderInsert(pInputExecOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspExecOrderAction(&self, pInputExecOrderAction: InputExecOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspExecOrderAction(pInputExecOrderAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspForQuoteInsert(&self, pInputForQuote: InputForQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspForQuoteInsert(pInputForQuote, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQuoteInsert(&self, pInputQuote: InputQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQuoteInsert(pInputQuote, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQuoteAction(&self, pInputQuoteAction: InputQuoteActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQuoteAction(pInputQuoteAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspBatchOrderAction(&self, pInputBatchOrderAction: InputBatchOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspBatchOrderAction(pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOptionSelfCloseInsert(pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspOptionSelfCloseAction(&self, pInputOptionSelfCloseAction: InputOptionSelfCloseActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOptionSelfCloseAction(pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspCombActionInsert(&self, pInputCombAction: InputCombActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspCombActionInsert(pInputCombAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryOrder(&self, pOrder: OrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOrder(pOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTrade(&self, pTrade: TradeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTrade(pTrade, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorPosition(&self, pInvestorPosition: InvestorPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPosition(pInvestorPosition, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTradingAccount(&self, pTradingAccount: TradingAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTradingAccount(pTradingAccount, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestor(&self, pInvestor: InvestorField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestor(pInvestor, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTradingCode(&self, pTradingCode: TradingCodeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTradingCode(pTradingCode, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInstrumentMarginRate(&self, pInstrumentMarginRate: InstrumentMarginRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrumentMarginRate(pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInstrumentCommissionRate(&self, pInstrumentCommissionRate: InstrumentCommissionRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrumentCommissionRate(pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryExchange(&self, pExchange: ExchangeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchange(pExchange, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryProduct(&self, pProduct: ProductField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryProduct(pProduct, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInstrument(&self, pInstrument: InstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrument(pInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryDepthMarketData(&self, pDepthMarketData: DepthMarketDataField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryDepthMarketData(pDepthMarketData, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTraderOffer(&self, pTraderOffer: TraderOfferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTraderOffer(pTraderOffer, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySettlementInfo(&self, pSettlementInfo: SettlementInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySettlementInfo(pSettlementInfo, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTransferBank(&self, pTransferBank: TransferBankField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTransferBank(pTransferBank, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorPositionDetail(&self, pInvestorPositionDetail: InvestorPositionDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPositionDetail(pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryNotice(&self, pNotice: NoticeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryNotice(pNotice, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySettlementInfoConfirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorPositionCombineDetail(&self, pInvestorPositionCombineDetail: InvestorPositionCombineDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPositionCombineDetail(pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryCFMMCTradingAccountKey(&self, pCFMMCTradingAccountKey: CFMMCTradingAccountKeyField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCFMMCTradingAccountKey(pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryEWarrantOffset(&self, pEWarrantOffset: EWarrantOffsetField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryEWarrantOffset(pEWarrantOffset, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorProductGroupMargin(&self, pInvestorProductGroupMargin: InvestorProductGroupMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProductGroupMargin(pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryExchangeMarginRate(&self, pExchangeMarginRate: ExchangeMarginRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchangeMarginRate(pExchangeMarginRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryExchangeMarginRateAdjust(&self, pExchangeMarginRateAdjust: ExchangeMarginRateAdjustField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchangeMarginRateAdjust(pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryExchangeRate(&self, pExchangeRate: ExchangeRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchangeRate(pExchangeRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySecAgentACIDMap(&self, pSecAgentACIDMap: SecAgentACIDMapField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentACIDMap(pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryProductExchRate(&self, pProductExchRate: ProductExchRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryProductExchRate(pProductExchRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryProductGroup(&self, pProductGroup: ProductGroupField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryProductGroup(pProductGroup, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryMMInstrumentCommissionRate(&self, pMMInstrumentCommissionRate: MMInstrumentCommissionRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryMMInstrumentCommissionRate(pMMInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryMMOptionInstrCommRate(&self, pMMOptionInstrCommRate: MMOptionInstrCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryMMOptionInstrCommRate(pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInstrumentOrderCommRate(&self, pInstrumentOrderCommRate: InstrumentOrderCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrumentOrderCommRate(pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySecAgentTradingAccount(&self, pTradingAccount: TradingAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentTradingAccount(pTradingAccount, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySecAgentCheckMode(&self, pSecAgentCheckMode: SecAgentCheckModeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentCheckMode(pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySecAgentTradeInfo(&self, pSecAgentTradeInfo: SecAgentTradeInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentTradeInfo(pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryOptionInstrTradeCost(&self, pOptionInstrTradeCost: OptionInstrTradeCostField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOptionInstrTradeCost(pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryOptionInstrCommRate(&self, pOptionInstrCommRate: OptionInstrCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOptionInstrCommRate(pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryExecOrder(&self, pExecOrder: ExecOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExecOrder(pExecOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryForQuote(&self, pForQuote: ForQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryForQuote(pForQuote, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryQuote(&self, pQuote: QuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryQuote(pQuote, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryOptionSelfClose(&self, pOptionSelfClose: OptionSelfCloseField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOptionSelfClose(pOptionSelfClose, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestUnit(&self, pInvestUnit: InvestUnitField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestUnit(pInvestUnit, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryCombInstrumentGuard(&self, pCombInstrumentGuard: CombInstrumentGuardField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombInstrumentGuard(pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryCombAction(&self, pCombAction: CombActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombAction(pCombAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTransferSerial(&self, pTransferSerial: TransferSerialField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTransferSerial(pTransferSerial, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryAccountregister(&self, pAccountregister: AccountregisterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryAccountregister(pAccountregister, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspError(&self, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspError(pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnOrder(&self, pOrder: OrderField) { self.tx.send(TraderSpiMsg::OnRtnOrder(pOrder)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnTrade(&self, pTrade: TradeField) { self.tx.send(TraderSpiMsg::OnRtnTrade(pTrade)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnOrderInsert(&self, pInputOrder: InputOrderField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOrderInsert(pInputOrder, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnOrderAction(&self, pOrderAction: OrderActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOrderAction(pOrderAction, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnInstrumentStatus(&self, pInstrumentStatus: InstrumentStatusField) { self.tx.send(TraderSpiMsg::OnRtnInstrumentStatus(pInstrumentStatus)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnBulletin(&self, pBulletin: BulletinField) { self.tx.send(TraderSpiMsg::OnRtnBulletin(pBulletin)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnTradingNotice(&self, pTradingNoticeInfo: TradingNoticeInfoField) { self.tx.send(TraderSpiMsg::OnRtnTradingNotice(pTradingNoticeInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnErrorConditionalOrder(&self, pErrorConditionalOrder: ErrorConditionalOrderField) { self.tx.send(TraderSpiMsg::OnRtnErrorConditionalOrder(pErrorConditionalOrder)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnExecOrder(&self, pExecOrder: ExecOrderField) { self.tx.send(TraderSpiMsg::OnRtnExecOrder(pExecOrder)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnExecOrderInsert(pInputExecOrder, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnExecOrderAction(&self, pExecOrderAction: ExecOrderActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnExecOrderAction(pExecOrderAction, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnForQuoteInsert(&self, pInputForQuote: InputForQuoteField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnForQuoteInsert(pInputForQuote, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnQuote(&self, pQuote: QuoteField) { self.tx.send(TraderSpiMsg::OnRtnQuote(pQuote)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnQuoteInsert(&self, pInputQuote: InputQuoteField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnQuoteInsert(pInputQuote, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnQuoteAction(&self, pQuoteAction: QuoteActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnQuoteAction(pQuoteAction, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnForQuoteRsp(&self, pForQuoteRsp: ForQuoteRspField) { self.tx.send(TraderSpiMsg::OnRtnForQuoteRsp(pForQuoteRsp)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnCFMMCTradingAccountToken(&self, pCFMMCTradingAccountToken: CFMMCTradingAccountTokenField) { self.tx.send(TraderSpiMsg::OnRtnCFMMCTradingAccountToken(pCFMMCTradingAccountToken)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnBatchOrderAction(&self, pBatchOrderAction: BatchOrderActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnBatchOrderAction(pBatchOrderAction, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnOptionSelfClose(&self, pOptionSelfClose: OptionSelfCloseField) { self.tx.send(TraderSpiMsg::OnRtnOptionSelfClose(pOptionSelfClose)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOptionSelfCloseInsert(pInputOptionSelfClose, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnOptionSelfCloseAction(&self, pOptionSelfCloseAction: OptionSelfCloseActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOptionSelfCloseAction(pOptionSelfCloseAction, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnCombAction(&self, pCombAction: CombActionField) { self.tx.send(TraderSpiMsg::OnRtnCombAction(pCombAction)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnCombActionInsert(&self, pInputCombAction: InputCombActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnCombActionInsert(pInputCombAction, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryContractBank(&self, pContractBank: ContractBankField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryContractBank(pContractBank, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryParkedOrder(&self, pParkedOrder: ParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryParkedOrder(pParkedOrder, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryParkedOrderAction(pParkedOrderAction, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryTradingNotice(&self, pTradingNotice: TradingNoticeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTradingNotice(pTradingNotice, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryBrokerTradingParams(&self, pBrokerTradingParams: BrokerTradingParamsField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryBrokerTradingParams(pBrokerTradingParams, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryBrokerTradingAlgos(&self, pBrokerTradingAlgos: BrokerTradingAlgosField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryBrokerTradingAlgos(pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQueryCFMMCTradingAccountToken(&self, pQueryCFMMCTradingAccountToken: QueryCFMMCTradingAccountTokenField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQueryCFMMCTradingAccountToken(pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnFromBankToFutureByBank(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromBankToFutureByBank(pRspTransfer)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnFromFutureToBankByBank(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromFutureToBankByBank(pRspTransfer)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnRepealFromBankToFutureByBank(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromBankToFutureByBank(pRspRepeal)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnRepealFromFutureToBankByBank(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromFutureToBankByBank(pRspRepeal)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnFromBankToFutureByFuture(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromBankToFutureByFuture(pRspTransfer)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnFromFutureToBankByFuture(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromFutureToBankByFuture(pRspTransfer)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnRepealFromBankToFutureByFutureManual(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromBankToFutureByFutureManual(pRspRepeal)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnRepealFromFutureToBankByFutureManual(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromFutureToBankByFutureManual(pRspRepeal)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnQueryBankBalanceByFuture(&self, pNotifyQueryAccount: NotifyQueryAccountField) { self.tx.send(TraderSpiMsg::OnRtnQueryBankBalanceByFuture(pNotifyQueryAccount)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnBankToFutureByFuture(pReqTransfer, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnFutureToBankByFuture(pReqTransfer, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnRepealBankToFutureByFutureManual(&self, pReqRepeal: ReqRepealField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnRepealBankToFutureByFutureManual(pReqRepeal, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnRepealFutureToBankByFutureManual(&self, pReqRepeal: ReqRepealField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnRepealFutureToBankByFutureManual(pReqRepeal, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnErrRtnQueryBankBalanceByFuture(&self, pReqQueryAccount: ReqQueryAccountField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnQueryBankBalanceByFuture(pReqQueryAccount, pRspInfo)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnRepealFromBankToFutureByFuture(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromBankToFutureByFuture(pRspRepeal)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnRepealFromFutureToBankByFuture(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromFutureToBankByFuture(pRspRepeal)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspFromBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspFromBankToFutureByFuture(pReqTransfer, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspFromFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspFromFutureToBankByFuture(pReqTransfer, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQueryBankAccountMoneyByFuture(&self, pReqQueryAccount: ReqQueryAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQueryBankAccountMoneyByFuture(pReqQueryAccount, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnOpenAccountByBank(&self, pOpenAccount: OpenAccountField) { self.tx.send(TraderSpiMsg::OnRtnOpenAccountByBank(pOpenAccount)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnCancelAccountByBank(&self, pCancelAccount: CancelAccountField) { self.tx.send(TraderSpiMsg::OnRtnCancelAccountByBank(pCancelAccount)).expect("sending TraderSpiMsg failed"); }
pub fn OnRtnChangeAccountByBank(&self, pChangeAccount: ChangeAccountField) { self.tx.send(TraderSpiMsg::OnRtnChangeAccountByBank(pChangeAccount)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryClassifiedInstrument(&self, pInstrument: InstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryClassifiedInstrument(pInstrument, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryCombPromotionParam(&self, pCombPromotionParam: CombPromotionParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombPromotionParam(pCombPromotionParam, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRiskSettleInvstPosition(&self, pRiskSettleInvstPosition: RiskSettleInvstPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRiskSettleInvstPosition(pRiskSettleInvstPosition, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRiskSettleProductStatus(&self, pRiskSettleProductStatus: RiskSettleProductStatusField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRiskSettleProductStatus(pRiskSettleProductStatus, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMFutureParameter(&self, pSPBMFutureParameter: SPBMFutureParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMFutureParameter(pSPBMFutureParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMOptionParameter(&self, pSPBMOptionParameter: SPBMOptionParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMOptionParameter(pSPBMOptionParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMIntraParameter(&self, pSPBMIntraParameter: SPBMIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMIntraParameter(pSPBMIntraParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMInterParameter(&self, pSPBMInterParameter: SPBMInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMInterParameter(pSPBMInterParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMPortfDefinition(&self, pSPBMPortfDefinition: SPBMPortfDefinitionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMPortfDefinition(pSPBMPortfDefinition, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMInvestorPortfDef(&self, pSPBMInvestorPortfDef: SPBMInvestorPortfDefField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMInvestorPortfDef(pSPBMInvestorPortfDef, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorPortfMarginRatio(&self, pInvestorPortfMarginRatio: InvestorPortfMarginRatioField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPortfMarginRatio(pInvestorPortfMarginRatio, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorProdSPBMDetail(&self, pInvestorProdSPBMDetail: InvestorProdSPBMDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProdSPBMDetail(pInvestorProdSPBMDetail, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorCommoditySPMMMargin(&self, pInvestorCommoditySPMMMargin: InvestorCommoditySPMMMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorCommoditySPMMMargin(pInvestorCommoditySPMMMargin, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorCommodityGroupSPMMMargin(&self, pInvestorCommodityGroupSPMMMargin: InvestorCommodityGroupSPMMMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorCommodityGroupSPMMMargin(pInvestorCommodityGroupSPMMMargin, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPMMInstParam(&self, pSPMMInstParam: SPMMInstParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPMMInstParam(pSPMMInstParam, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPMMProductParam(&self, pSPMMProductParam: SPMMProductParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPMMProductParam(pSPMMProductParam, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQrySPBMAddOnInterParameter(&self, pSPBMAddOnInterParameter: SPBMAddOnInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMAddOnInterParameter(pSPBMAddOnInterParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRCAMSCombProductInfo(&self, pRCAMSCombProductInfo: RCAMSCombProductInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSCombProductInfo(pRCAMSCombProductInfo, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRCAMSInstrParameter(&self, pRCAMSInstrParameter: RCAMSInstrParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSInstrParameter(pRCAMSInstrParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRCAMSIntraParameter(&self, pRCAMSIntraParameter: RCAMSIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSIntraParameter(pRCAMSIntraParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRCAMSInterParameter(&self, pRCAMSInterParameter: RCAMSInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSInterParameter(pRCAMSInterParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRCAMSShortOptAdjustParam(&self, pRCAMSShortOptAdjustParam: RCAMSShortOptAdjustParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSShortOptAdjustParam(pRCAMSShortOptAdjustParam, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRCAMSInvestorCombPosition(&self, pRCAMSInvestorCombPosition: RCAMSInvestorCombPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSInvestorCombPosition(pRCAMSInvestorCombPosition, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorProdRCAMSMargin(&self, pInvestorProdRCAMSMargin: InvestorProdRCAMSMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProdRCAMSMargin(pInvestorProdRCAMSMargin, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRULEInstrParameter(&self, pRULEInstrParameter: RULEInstrParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRULEInstrParameter(pRULEInstrParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRULEIntraParameter(&self, pRULEIntraParameter: RULEIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRULEIntraParameter(pRULEIntraParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryRULEInterParameter(&self, pRULEInterParameter: RULEInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRULEInterParameter(pRULEInterParameter, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorProdRULEMargin(&self, pInvestorProdRULEMargin: InvestorProdRULEMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProdRULEMargin(pInvestorProdRULEMargin, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorPortfSetting(&self, pInvestorPortfSetting: InvestorPortfSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPortfSetting(pInvestorPortfSetting, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
pub fn OnRspQryInvestorInfoCommRec(&self, pInvestorInfoCommRec: InvestorInfoCommRecField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorInfoCommRec(pInvestorInfoCommRec, pRspInfo, nRequestID, bIsLast)).expect("sending TraderSpiMsg failed"); }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MdSpi;

        pub fn OnFrontConnected(&self);
        pub fn OnFrontDisconnected(&self, nReason: i32);
        pub fn OnHeartBeatWarning(&self, nTimeLapse: i32);
        pub fn OnRspUserLogin(&self, pRspUserLogin: RspUserLoginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUserLogout(&self, pUserLogout: UserLogoutField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryMulticastInstrument(&self, pMulticastInstrument: MulticastInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspError(&self, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspSubMarketData(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUnSubMarketData(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspSubForQuoteRsp(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUnSubForQuoteRsp(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRtnDepthMarketData(&self, pDepthMarketData: DepthMarketDataField);
        pub fn OnRtnForQuoteRsp(&self, pForQuoteRsp: ForQuoteRspField);
    }

    unsafe extern "C++" {
        include!("ctp4rs/wrapper/include/MdApi.h");
        type MdApi;

        fn CreateMdApi(spi: &MdSpi, flow_path: String) -> UniquePtr<MdApi>;

        fn GetApiVersion(&self)-> String;
        fn Init(&self);
        fn Join(&self)-> i32;
        fn GetTradingDay(&self)-> String;
        fn RegisterFront(&self, pszFrontAddress: String);
        fn RegisterNameServer(&self, pszNsAddress: String);
        fn RegisterFensUserInfo(&self, pFensUserInfo: FensUserInfoField);
        fn SubscribeMarketData(&self, ppInstrumentID: Vec<String>, nCount: i32)-> i32;
        fn UnSubscribeMarketData(&self, ppInstrumentID: Vec<String>, nCount: i32)-> i32;
        fn SubscribeForQuoteRsp(&self, ppInstrumentID: Vec<String>, nCount: i32)-> i32;
        fn UnSubscribeForQuoteRsp(&self, ppInstrumentID: Vec<String>, nCount: i32)-> i32;
        fn ReqUserLogin(&self, pReqUserLoginField: ReqUserLoginField, nRequestID: i32)-> i32;
        fn ReqUserLogout(&self, pUserLogout: UserLogoutField, nRequestID: i32)-> i32;
        fn ReqQryMulticastInstrument(&self, pQryMulticastInstrument: QryMulticastInstrumentField, nRequestID: i32)-> i32;
    }

    extern "Rust" {
        type TraderSpi;

        pub fn OnFrontConnected(&self);
        pub fn OnFrontDisconnected(&self, nReason: i32);
        pub fn OnHeartBeatWarning(&self, nTimeLapse: i32);
        pub fn OnRspAuthenticate(&self, pRspAuthenticateField: RspAuthenticateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUserLogin(&self, pRspUserLogin: RspUserLoginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUserLogout(&self, pUserLogout: UserLogoutField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUserPasswordUpdate(&self, pUserPasswordUpdate: UserPasswordUpdateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspTradingAccountPasswordUpdate(&self, pTradingAccountPasswordUpdate: TradingAccountPasswordUpdateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspUserAuthMethod(&self, pRspUserAuthMethod: RspUserAuthMethodField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspGenUserCaptcha(&self, pRspGenUserCaptcha: RspGenUserCaptchaField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspGenUserText(&self, pRspGenUserText: RspGenUserTextField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspOrderInsert(&self, pInputOrder: InputOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspParkedOrderInsert(&self, pParkedOrder: ParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspOrderAction(&self, pInputOrderAction: InputOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryMaxOrderVolume(&self, pQryMaxOrderVolume: QryMaxOrderVolumeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspSettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspRemoveParkedOrder(&self, pRemoveParkedOrder: RemoveParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspRemoveParkedOrderAction(&self, pRemoveParkedOrderAction: RemoveParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspExecOrderAction(&self, pInputExecOrderAction: InputExecOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspForQuoteInsert(&self, pInputForQuote: InputForQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQuoteInsert(&self, pInputQuote: InputQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQuoteAction(&self, pInputQuoteAction: InputQuoteActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspBatchOrderAction(&self, pInputBatchOrderAction: InputBatchOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspOptionSelfCloseAction(&self, pInputOptionSelfCloseAction: InputOptionSelfCloseActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspCombActionInsert(&self, pInputCombAction: InputCombActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryOrder(&self, pOrder: OrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTrade(&self, pTrade: TradeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorPosition(&self, pInvestorPosition: InvestorPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTradingAccount(&self, pTradingAccount: TradingAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestor(&self, pInvestor: InvestorField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTradingCode(&self, pTradingCode: TradingCodeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInstrumentMarginRate(&self, pInstrumentMarginRate: InstrumentMarginRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInstrumentCommissionRate(&self, pInstrumentCommissionRate: InstrumentCommissionRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryExchange(&self, pExchange: ExchangeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryProduct(&self, pProduct: ProductField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInstrument(&self, pInstrument: InstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryDepthMarketData(&self, pDepthMarketData: DepthMarketDataField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTraderOffer(&self, pTraderOffer: TraderOfferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySettlementInfo(&self, pSettlementInfo: SettlementInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTransferBank(&self, pTransferBank: TransferBankField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorPositionDetail(&self, pInvestorPositionDetail: InvestorPositionDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryNotice(&self, pNotice: NoticeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorPositionCombineDetail(&self, pInvestorPositionCombineDetail: InvestorPositionCombineDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryCFMMCTradingAccountKey(&self, pCFMMCTradingAccountKey: CFMMCTradingAccountKeyField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryEWarrantOffset(&self, pEWarrantOffset: EWarrantOffsetField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorProductGroupMargin(&self, pInvestorProductGroupMargin: InvestorProductGroupMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryExchangeMarginRate(&self, pExchangeMarginRate: ExchangeMarginRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryExchangeMarginRateAdjust(&self, pExchangeMarginRateAdjust: ExchangeMarginRateAdjustField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryExchangeRate(&self, pExchangeRate: ExchangeRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySecAgentACIDMap(&self, pSecAgentACIDMap: SecAgentACIDMapField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryProductExchRate(&self, pProductExchRate: ProductExchRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryProductGroup(&self, pProductGroup: ProductGroupField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryMMInstrumentCommissionRate(&self, pMMInstrumentCommissionRate: MMInstrumentCommissionRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryMMOptionInstrCommRate(&self, pMMOptionInstrCommRate: MMOptionInstrCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInstrumentOrderCommRate(&self, pInstrumentOrderCommRate: InstrumentOrderCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySecAgentTradingAccount(&self, pTradingAccount: TradingAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySecAgentCheckMode(&self, pSecAgentCheckMode: SecAgentCheckModeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySecAgentTradeInfo(&self, pSecAgentTradeInfo: SecAgentTradeInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryOptionInstrTradeCost(&self, pOptionInstrTradeCost: OptionInstrTradeCostField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryOptionInstrCommRate(&self, pOptionInstrCommRate: OptionInstrCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryExecOrder(&self, pExecOrder: ExecOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryForQuote(&self, pForQuote: ForQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryQuote(&self, pQuote: QuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryOptionSelfClose(&self, pOptionSelfClose: OptionSelfCloseField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestUnit(&self, pInvestUnit: InvestUnitField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryCombInstrumentGuard(&self, pCombInstrumentGuard: CombInstrumentGuardField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryCombAction(&self, pCombAction: CombActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTransferSerial(&self, pTransferSerial: TransferSerialField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryAccountregister(&self, pAccountregister: AccountregisterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspError(&self, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRtnOrder(&self, pOrder: OrderField);
        pub fn OnRtnTrade(&self, pTrade: TradeField);
        pub fn OnErrRtnOrderInsert(&self, pInputOrder: InputOrderField, pRspInfo: RspInfoField);
        pub fn OnErrRtnOrderAction(&self, pOrderAction: OrderActionField, pRspInfo: RspInfoField);
        pub fn OnRtnInstrumentStatus(&self, pInstrumentStatus: InstrumentStatusField);
        pub fn OnRtnBulletin(&self, pBulletin: BulletinField);
        pub fn OnRtnTradingNotice(&self, pTradingNoticeInfo: TradingNoticeInfoField);
        pub fn OnRtnErrorConditionalOrder(&self, pErrorConditionalOrder: ErrorConditionalOrderField);
        pub fn OnRtnExecOrder(&self, pExecOrder: ExecOrderField);
        pub fn OnErrRtnExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, pRspInfo: RspInfoField);
        pub fn OnErrRtnExecOrderAction(&self, pExecOrderAction: ExecOrderActionField, pRspInfo: RspInfoField);
        pub fn OnErrRtnForQuoteInsert(&self, pInputForQuote: InputForQuoteField, pRspInfo: RspInfoField);
        pub fn OnRtnQuote(&self, pQuote: QuoteField);
        pub fn OnErrRtnQuoteInsert(&self, pInputQuote: InputQuoteField, pRspInfo: RspInfoField);
        pub fn OnErrRtnQuoteAction(&self, pQuoteAction: QuoteActionField, pRspInfo: RspInfoField);
        pub fn OnRtnForQuoteRsp(&self, pForQuoteRsp: ForQuoteRspField);
        pub fn OnRtnCFMMCTradingAccountToken(&self, pCFMMCTradingAccountToken: CFMMCTradingAccountTokenField);
        pub fn OnErrRtnBatchOrderAction(&self, pBatchOrderAction: BatchOrderActionField, pRspInfo: RspInfoField);
        pub fn OnRtnOptionSelfClose(&self, pOptionSelfClose: OptionSelfCloseField);
        pub fn OnErrRtnOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, pRspInfo: RspInfoField);
        pub fn OnErrRtnOptionSelfCloseAction(&self, pOptionSelfCloseAction: OptionSelfCloseActionField, pRspInfo: RspInfoField);
        pub fn OnRtnCombAction(&self, pCombAction: CombActionField);
        pub fn OnErrRtnCombActionInsert(&self, pInputCombAction: InputCombActionField, pRspInfo: RspInfoField);
        pub fn OnRspQryContractBank(&self, pContractBank: ContractBankField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryParkedOrder(&self, pParkedOrder: ParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryTradingNotice(&self, pTradingNotice: TradingNoticeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryBrokerTradingParams(&self, pBrokerTradingParams: BrokerTradingParamsField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryBrokerTradingAlgos(&self, pBrokerTradingAlgos: BrokerTradingAlgosField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQueryCFMMCTradingAccountToken(&self, pQueryCFMMCTradingAccountToken: QueryCFMMCTradingAccountTokenField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRtnFromBankToFutureByBank(&self, pRspTransfer: RspTransferField);
        pub fn OnRtnFromFutureToBankByBank(&self, pRspTransfer: RspTransferField);
        pub fn OnRtnRepealFromBankToFutureByBank(&self, pRspRepeal: RspRepealField);
        pub fn OnRtnRepealFromFutureToBankByBank(&self, pRspRepeal: RspRepealField);
        pub fn OnRtnFromBankToFutureByFuture(&self, pRspTransfer: RspTransferField);
        pub fn OnRtnFromFutureToBankByFuture(&self, pRspTransfer: RspTransferField);
        pub fn OnRtnRepealFromBankToFutureByFutureManual(&self, pRspRepeal: RspRepealField);
        pub fn OnRtnRepealFromFutureToBankByFutureManual(&self, pRspRepeal: RspRepealField);
        pub fn OnRtnQueryBankBalanceByFuture(&self, pNotifyQueryAccount: NotifyQueryAccountField);
        pub fn OnErrRtnBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField);
        pub fn OnErrRtnFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField);
        pub fn OnErrRtnRepealBankToFutureByFutureManual(&self, pReqRepeal: ReqRepealField, pRspInfo: RspInfoField);
        pub fn OnErrRtnRepealFutureToBankByFutureManual(&self, pReqRepeal: ReqRepealField, pRspInfo: RspInfoField);
        pub fn OnErrRtnQueryBankBalanceByFuture(&self, pReqQueryAccount: ReqQueryAccountField, pRspInfo: RspInfoField);
        pub fn OnRtnRepealFromBankToFutureByFuture(&self, pRspRepeal: RspRepealField);
        pub fn OnRtnRepealFromFutureToBankByFuture(&self, pRspRepeal: RspRepealField);
        pub fn OnRspFromBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspFromFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQueryBankAccountMoneyByFuture(&self, pReqQueryAccount: ReqQueryAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRtnOpenAccountByBank(&self, pOpenAccount: OpenAccountField);
        pub fn OnRtnCancelAccountByBank(&self, pCancelAccount: CancelAccountField);
        pub fn OnRtnChangeAccountByBank(&self, pChangeAccount: ChangeAccountField);
        pub fn OnRspQryClassifiedInstrument(&self, pInstrument: InstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryCombPromotionParam(&self, pCombPromotionParam: CombPromotionParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRiskSettleInvstPosition(&self, pRiskSettleInvstPosition: RiskSettleInvstPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRiskSettleProductStatus(&self, pRiskSettleProductStatus: RiskSettleProductStatusField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMFutureParameter(&self, pSPBMFutureParameter: SPBMFutureParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMOptionParameter(&self, pSPBMOptionParameter: SPBMOptionParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMIntraParameter(&self, pSPBMIntraParameter: SPBMIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMInterParameter(&self, pSPBMInterParameter: SPBMInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMPortfDefinition(&self, pSPBMPortfDefinition: SPBMPortfDefinitionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMInvestorPortfDef(&self, pSPBMInvestorPortfDef: SPBMInvestorPortfDefField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorPortfMarginRatio(&self, pInvestorPortfMarginRatio: InvestorPortfMarginRatioField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorProdSPBMDetail(&self, pInvestorProdSPBMDetail: InvestorProdSPBMDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorCommoditySPMMMargin(&self, pInvestorCommoditySPMMMargin: InvestorCommoditySPMMMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorCommodityGroupSPMMMargin(&self, pInvestorCommodityGroupSPMMMargin: InvestorCommodityGroupSPMMMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPMMInstParam(&self, pSPMMInstParam: SPMMInstParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPMMProductParam(&self, pSPMMProductParam: SPMMProductParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQrySPBMAddOnInterParameter(&self, pSPBMAddOnInterParameter: SPBMAddOnInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRCAMSCombProductInfo(&self, pRCAMSCombProductInfo: RCAMSCombProductInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRCAMSInstrParameter(&self, pRCAMSInstrParameter: RCAMSInstrParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRCAMSIntraParameter(&self, pRCAMSIntraParameter: RCAMSIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRCAMSInterParameter(&self, pRCAMSInterParameter: RCAMSInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRCAMSShortOptAdjustParam(&self, pRCAMSShortOptAdjustParam: RCAMSShortOptAdjustParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRCAMSInvestorCombPosition(&self, pRCAMSInvestorCombPosition: RCAMSInvestorCombPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorProdRCAMSMargin(&self, pInvestorProdRCAMSMargin: InvestorProdRCAMSMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRULEInstrParameter(&self, pRULEInstrParameter: RULEInstrParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRULEIntraParameter(&self, pRULEIntraParameter: RULEIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryRULEInterParameter(&self, pRULEInterParameter: RULEInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorProdRULEMargin(&self, pInvestorProdRULEMargin: InvestorProdRULEMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorPortfSetting(&self, pInvestorPortfSetting: InvestorPortfSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspQryInvestorInfoCommRec(&self, pInvestorInfoCommRec: InvestorInfoCommRecField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
    }

    unsafe extern "C++" {
        include!("ctp4rs/wrapper/include/TraderApi.h");
        type TraderApi;

        fn CreateTraderApi(spi: &TraderSpi, flow_path: String) -> UniquePtr<TraderApi>;
        fn GetFrontInfo(&self) -> FrontInfoField;

        fn GetApiVersion(&self)-> String;
        fn Init(&self);
        fn Join(&self)-> i32;
        fn GetTradingDay(&self)-> String;
        fn RegisterFront(&self, pszFrontAddress: String);
        fn RegisterNameServer(&self, pszNsAddress: String);
        fn RegisterFensUserInfo(&self, pFensUserInfo: FensUserInfoField);
        fn SubscribePrivateTopic(&self, nResumeType: i32);
        fn SubscribePublicTopic(&self, nResumeType: i32);
        fn ReqAuthenticate(&self, pReqAuthenticateField: ReqAuthenticateField, nRequestID: i32)-> i32;
        fn RegisterUserSystemInfo(&self, pUserSystemInfo: UserSystemInfoField)-> i32;
        fn SubmitUserSystemInfo(&self, pUserSystemInfo: UserSystemInfoField)-> i32;
        fn ReqUserLogin(&self, pReqUserLoginField: ReqUserLoginField, nRequestID: i32)-> i32;
        fn ReqUserLogout(&self, pUserLogout: UserLogoutField, nRequestID: i32)-> i32;
        fn ReqUserPasswordUpdate(&self, pUserPasswordUpdate: UserPasswordUpdateField, nRequestID: i32)-> i32;
        fn ReqTradingAccountPasswordUpdate(&self, pTradingAccountPasswordUpdate: TradingAccountPasswordUpdateField, nRequestID: i32)-> i32;
        fn ReqUserAuthMethod(&self, pReqUserAuthMethod: ReqUserAuthMethodField, nRequestID: i32)-> i32;
        fn ReqGenUserCaptcha(&self, pReqGenUserCaptcha: ReqGenUserCaptchaField, nRequestID: i32)-> i32;
        fn ReqGenUserText(&self, pReqGenUserText: ReqGenUserTextField, nRequestID: i32)-> i32;
        fn ReqUserLoginWithCaptcha(&self, pReqUserLoginWithCaptcha: ReqUserLoginWithCaptchaField, nRequestID: i32)-> i32;
        fn ReqUserLoginWithText(&self, pReqUserLoginWithText: ReqUserLoginWithTextField, nRequestID: i32)-> i32;
        fn ReqUserLoginWithOTP(&self, pReqUserLoginWithOTP: ReqUserLoginWithOTPField, nRequestID: i32)-> i32;
        fn ReqOrderInsert(&self, pInputOrder: InputOrderField, nRequestID: i32)-> i32;
        fn ReqParkedOrderInsert(&self, pParkedOrder: ParkedOrderField, nRequestID: i32)-> i32;
        fn ReqParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, nRequestID: i32)-> i32;
        fn ReqOrderAction(&self, pInputOrderAction: InputOrderActionField, nRequestID: i32)-> i32;
        fn ReqQryMaxOrderVolume(&self, pQryMaxOrderVolume: QryMaxOrderVolumeField, nRequestID: i32)-> i32;
        fn ReqSettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, nRequestID: i32)-> i32;
        fn ReqRemoveParkedOrder(&self, pRemoveParkedOrder: RemoveParkedOrderField, nRequestID: i32)-> i32;
        fn ReqRemoveParkedOrderAction(&self, pRemoveParkedOrderAction: RemoveParkedOrderActionField, nRequestID: i32)-> i32;
        fn ReqExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, nRequestID: i32)-> i32;
        fn ReqExecOrderAction(&self, pInputExecOrderAction: InputExecOrderActionField, nRequestID: i32)-> i32;
        fn ReqForQuoteInsert(&self, pInputForQuote: InputForQuoteField, nRequestID: i32)-> i32;
        fn ReqQuoteInsert(&self, pInputQuote: InputQuoteField, nRequestID: i32)-> i32;
        fn ReqQuoteAction(&self, pInputQuoteAction: InputQuoteActionField, nRequestID: i32)-> i32;
        fn ReqBatchOrderAction(&self, pInputBatchOrderAction: InputBatchOrderActionField, nRequestID: i32)-> i32;
        fn ReqOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, nRequestID: i32)-> i32;
        fn ReqOptionSelfCloseAction(&self, pInputOptionSelfCloseAction: InputOptionSelfCloseActionField, nRequestID: i32)-> i32;
        fn ReqCombActionInsert(&self, pInputCombAction: InputCombActionField, nRequestID: i32)-> i32;
        fn ReqQryOrder(&self, pQryOrder: QryOrderField, nRequestID: i32)-> i32;
        fn ReqQryTrade(&self, pQryTrade: QryTradeField, nRequestID: i32)-> i32;
        fn ReqQryInvestorPosition(&self, pQryInvestorPosition: QryInvestorPositionField, nRequestID: i32)-> i32;
        fn ReqQryTradingAccount(&self, pQryTradingAccount: QryTradingAccountField, nRequestID: i32)-> i32;
        fn ReqQryInvestor(&self, pQryInvestor: QryInvestorField, nRequestID: i32)-> i32;
        fn ReqQryTradingCode(&self, pQryTradingCode: QryTradingCodeField, nRequestID: i32)-> i32;
        fn ReqQryInstrumentMarginRate(&self, pQryInstrumentMarginRate: QryInstrumentMarginRateField, nRequestID: i32)-> i32;
        fn ReqQryInstrumentCommissionRate(&self, pQryInstrumentCommissionRate: QryInstrumentCommissionRateField, nRequestID: i32)-> i32;
        fn ReqQryExchange(&self, pQryExchange: QryExchangeField, nRequestID: i32)-> i32;
        fn ReqQryProduct(&self, pQryProduct: QryProductField, nRequestID: i32)-> i32;
        fn ReqQryInstrument(&self, pQryInstrument: QryInstrumentField, nRequestID: i32)-> i32;
        fn ReqQryDepthMarketData(&self, pQryDepthMarketData: QryDepthMarketDataField, nRequestID: i32)-> i32;
        fn ReqQryTraderOffer(&self, pQryTraderOffer: QryTraderOfferField, nRequestID: i32)-> i32;
        fn ReqQrySettlementInfo(&self, pQrySettlementInfo: QrySettlementInfoField, nRequestID: i32)-> i32;
        fn ReqQryTransferBank(&self, pQryTransferBank: QryTransferBankField, nRequestID: i32)-> i32;
        fn ReqQryInvestorPositionDetail(&self, pQryInvestorPositionDetail: QryInvestorPositionDetailField, nRequestID: i32)-> i32;
        fn ReqQryNotice(&self, pQryNotice: QryNoticeField, nRequestID: i32)-> i32;
        fn ReqQrySettlementInfoConfirm(&self, pQrySettlementInfoConfirm: QrySettlementInfoConfirmField, nRequestID: i32)-> i32;
        fn ReqQryInvestorPositionCombineDetail(&self, pQryInvestorPositionCombineDetail: QryInvestorPositionCombineDetailField, nRequestID: i32)-> i32;
        fn ReqQryCFMMCTradingAccountKey(&self, pQryCFMMCTradingAccountKey: QryCFMMCTradingAccountKeyField, nRequestID: i32)-> i32;
        fn ReqQryEWarrantOffset(&self, pQryEWarrantOffset: QryEWarrantOffsetField, nRequestID: i32)-> i32;
        fn ReqQryInvestorProductGroupMargin(&self, pQryInvestorProductGroupMargin: QryInvestorProductGroupMarginField, nRequestID: i32)-> i32;
        fn ReqQryExchangeMarginRate(&self, pQryExchangeMarginRate: QryExchangeMarginRateField, nRequestID: i32)-> i32;
        fn ReqQryExchangeMarginRateAdjust(&self, pQryExchangeMarginRateAdjust: QryExchangeMarginRateAdjustField, nRequestID: i32)-> i32;
        fn ReqQryExchangeRate(&self, pQryExchangeRate: QryExchangeRateField, nRequestID: i32)-> i32;
        fn ReqQrySecAgentACIDMap(&self, pQrySecAgentACIDMap: QrySecAgentACIDMapField, nRequestID: i32)-> i32;
        fn ReqQryProductExchRate(&self, pQryProductExchRate: QryProductExchRateField, nRequestID: i32)-> i32;
        fn ReqQryProductGroup(&self, pQryProductGroup: QryProductGroupField, nRequestID: i32)-> i32;
        fn ReqQryMMInstrumentCommissionRate(&self, pQryMMInstrumentCommissionRate: QryMMInstrumentCommissionRateField, nRequestID: i32)-> i32;
        fn ReqQryMMOptionInstrCommRate(&self, pQryMMOptionInstrCommRate: QryMMOptionInstrCommRateField, nRequestID: i32)-> i32;
        fn ReqQryInstrumentOrderCommRate(&self, pQryInstrumentOrderCommRate: QryInstrumentOrderCommRateField, nRequestID: i32)-> i32;
        fn ReqQrySecAgentTradingAccount(&self, pQryTradingAccount: QryTradingAccountField, nRequestID: i32)-> i32;
        fn ReqQrySecAgentCheckMode(&self, pQrySecAgentCheckMode: QrySecAgentCheckModeField, nRequestID: i32)-> i32;
        fn ReqQrySecAgentTradeInfo(&self, pQrySecAgentTradeInfo: QrySecAgentTradeInfoField, nRequestID: i32)-> i32;
        fn ReqQryOptionInstrTradeCost(&self, pQryOptionInstrTradeCost: QryOptionInstrTradeCostField, nRequestID: i32)-> i32;
        fn ReqQryOptionInstrCommRate(&self, pQryOptionInstrCommRate: QryOptionInstrCommRateField, nRequestID: i32)-> i32;
        fn ReqQryExecOrder(&self, pQryExecOrder: QryExecOrderField, nRequestID: i32)-> i32;
        fn ReqQryForQuote(&self, pQryForQuote: QryForQuoteField, nRequestID: i32)-> i32;
        fn ReqQryQuote(&self, pQryQuote: QryQuoteField, nRequestID: i32)-> i32;
        fn ReqQryOptionSelfClose(&self, pQryOptionSelfClose: QryOptionSelfCloseField, nRequestID: i32)-> i32;
        fn ReqQryInvestUnit(&self, pQryInvestUnit: QryInvestUnitField, nRequestID: i32)-> i32;
        fn ReqQryCombInstrumentGuard(&self, pQryCombInstrumentGuard: QryCombInstrumentGuardField, nRequestID: i32)-> i32;
        fn ReqQryCombAction(&self, pQryCombAction: QryCombActionField, nRequestID: i32)-> i32;
        fn ReqQryTransferSerial(&self, pQryTransferSerial: QryTransferSerialField, nRequestID: i32)-> i32;
        fn ReqQryAccountregister(&self, pQryAccountregister: QryAccountregisterField, nRequestID: i32)-> i32;
        fn ReqQryContractBank(&self, pQryContractBank: QryContractBankField, nRequestID: i32)-> i32;
        fn ReqQryParkedOrder(&self, pQryParkedOrder: QryParkedOrderField, nRequestID: i32)-> i32;
        fn ReqQryParkedOrderAction(&self, pQryParkedOrderAction: QryParkedOrderActionField, nRequestID: i32)-> i32;
        fn ReqQryTradingNotice(&self, pQryTradingNotice: QryTradingNoticeField, nRequestID: i32)-> i32;
        fn ReqQryBrokerTradingParams(&self, pQryBrokerTradingParams: QryBrokerTradingParamsField, nRequestID: i32)-> i32;
        fn ReqQryBrokerTradingAlgos(&self, pQryBrokerTradingAlgos: QryBrokerTradingAlgosField, nRequestID: i32)-> i32;
        fn ReqQueryCFMMCTradingAccountToken(&self, pQueryCFMMCTradingAccountToken: QueryCFMMCTradingAccountTokenField, nRequestID: i32)-> i32;
        fn ReqFromBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, nRequestID: i32)-> i32;
        fn ReqFromFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, nRequestID: i32)-> i32;
        fn ReqQueryBankAccountMoneyByFuture(&self, pReqQueryAccount: ReqQueryAccountField, nRequestID: i32)-> i32;
        fn ReqQryClassifiedInstrument(&self, pQryClassifiedInstrument: QryClassifiedInstrumentField, nRequestID: i32)-> i32;
        fn ReqQryCombPromotionParam(&self, pQryCombPromotionParam: QryCombPromotionParamField, nRequestID: i32)-> i32;
        fn ReqQryRiskSettleInvstPosition(&self, pQryRiskSettleInvstPosition: QryRiskSettleInvstPositionField, nRequestID: i32)-> i32;
        fn ReqQryRiskSettleProductStatus(&self, pQryRiskSettleProductStatus: QryRiskSettleProductStatusField, nRequestID: i32)-> i32;
        fn ReqQrySPBMFutureParameter(&self, pQrySPBMFutureParameter: QrySPBMFutureParameterField, nRequestID: i32)-> i32;
        fn ReqQrySPBMOptionParameter(&self, pQrySPBMOptionParameter: QrySPBMOptionParameterField, nRequestID: i32)-> i32;
        fn ReqQrySPBMIntraParameter(&self, pQrySPBMIntraParameter: QrySPBMIntraParameterField, nRequestID: i32)-> i32;
        fn ReqQrySPBMInterParameter(&self, pQrySPBMInterParameter: QrySPBMInterParameterField, nRequestID: i32)-> i32;
        fn ReqQrySPBMPortfDefinition(&self, pQrySPBMPortfDefinition: QrySPBMPortfDefinitionField, nRequestID: i32)-> i32;
        fn ReqQrySPBMInvestorPortfDef(&self, pQrySPBMInvestorPortfDef: QrySPBMInvestorPortfDefField, nRequestID: i32)-> i32;
        fn ReqQryInvestorPortfMarginRatio(&self, pQryInvestorPortfMarginRatio: QryInvestorPortfMarginRatioField, nRequestID: i32)-> i32;
        fn ReqQryInvestorProdSPBMDetail(&self, pQryInvestorProdSPBMDetail: QryInvestorProdSPBMDetailField, nRequestID: i32)-> i32;
        fn ReqQryInvestorCommoditySPMMMargin(&self, pQryInvestorCommoditySPMMMargin: QryInvestorCommoditySPMMMarginField, nRequestID: i32)-> i32;
        fn ReqQryInvestorCommodityGroupSPMMMargin(&self, pQryInvestorCommodityGroupSPMMMargin: QryInvestorCommodityGroupSPMMMarginField, nRequestID: i32)-> i32;
        fn ReqQrySPMMInstParam(&self, pQrySPMMInstParam: QrySPMMInstParamField, nRequestID: i32)-> i32;
        fn ReqQrySPMMProductParam(&self, pQrySPMMProductParam: QrySPMMProductParamField, nRequestID: i32)-> i32;
        fn ReqQrySPBMAddOnInterParameter(&self, pQrySPBMAddOnInterParameter: QrySPBMAddOnInterParameterField, nRequestID: i32)-> i32;
        fn ReqQryRCAMSCombProductInfo(&self, pQryRCAMSCombProductInfo: QryRCAMSCombProductInfoField, nRequestID: i32)-> i32;
        fn ReqQryRCAMSInstrParameter(&self, pQryRCAMSInstrParameter: QryRCAMSInstrParameterField, nRequestID: i32)-> i32;
        fn ReqQryRCAMSIntraParameter(&self, pQryRCAMSIntraParameter: QryRCAMSIntraParameterField, nRequestID: i32)-> i32;
        fn ReqQryRCAMSInterParameter(&self, pQryRCAMSInterParameter: QryRCAMSInterParameterField, nRequestID: i32)-> i32;
        fn ReqQryRCAMSShortOptAdjustParam(&self, pQryRCAMSShortOptAdjustParam: QryRCAMSShortOptAdjustParamField, nRequestID: i32)-> i32;
        fn ReqQryRCAMSInvestorCombPosition(&self, pQryRCAMSInvestorCombPosition: QryRCAMSInvestorCombPositionField, nRequestID: i32)-> i32;
        fn ReqQryInvestorProdRCAMSMargin(&self, pQryInvestorProdRCAMSMargin: QryInvestorProdRCAMSMarginField, nRequestID: i32)-> i32;
        fn ReqQryRULEInstrParameter(&self, pQryRULEInstrParameter: QryRULEInstrParameterField, nRequestID: i32)-> i32;
        fn ReqQryRULEIntraParameter(&self, pQryRULEIntraParameter: QryRULEIntraParameterField, nRequestID: i32)-> i32;
        fn ReqQryRULEInterParameter(&self, pQryRULEInterParameter: QryRULEInterParameterField, nRequestID: i32)-> i32;
        fn ReqQryInvestorProdRULEMargin(&self, pQryInvestorProdRULEMargin: QryInvestorProdRULEMarginField, nRequestID: i32)-> i32;
        fn ReqQryInvestorPortfSetting(&self, pQryInvestorPortfSetting: QryInvestorPortfSettingField, nRequestID: i32)-> i32;
        fn ReqQryInvestorInfoCommRec(&self, pQryInvestorInfoCommRec: QryInvestorInfoCommRecField, nRequestID: i32)-> i32;
    }

    #[derive(Debug, Clone, Default)]
    struct DisseminationField {
        is_null: bool,
        SequenceSeries: u16,
        SequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
        Password: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        MacAddress: String,
        OneTimePassword: String,
        LoginRemark: String,
        ClientIPPort: i32,
        ClientIPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspUserLoginField {
        is_null: bool,
        TradingDay: String,
        LoginTime: String,
        BrokerID: String,
        UserID: String,
        SystemName: String,
        FrontID: i32,
        SessionID: i32,
        MaxOrderRef: String,
        SHFETime: String,
        DCETime: String,
        CZCETime: String,
        FFEXTime: String,
        INETime: String,
        SysVersion: String,
        GFEXTime: String,
        LoginDRIdentityID: i32,
        UserDRIdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct UserLogoutField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ForceUserLogoutField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqAuthenticateField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserProductInfo: String,
        AuthCode: String,
        AppID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspAuthenticateField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserProductInfo: String,
        AppID: String,
        AppType: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct AuthenticationInfoField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserProductInfo: String,
        AuthInfo: String,
        IsResult: i32,
        AppID: String,
        AppType: u8,
        ClientIPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspUserLogin2Field {
        is_null: bool,
        TradingDay: String,
        LoginTime: String,
        BrokerID: String,
        UserID: String,
        SystemName: String,
        FrontID: i32,
        SessionID: i32,
        MaxOrderRef: String,
        SHFETime: String,
        DCETime: String,
        CZCETime: String,
        FFEXTime: String,
        INETime: String,
        RandomString: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferHeaderField {
        is_null: bool,
        Version: String,
        TradeCode: String,
        TradeDate: String,
        TradeTime: String,
        TradeSerial: String,
        FutureID: String,
        BankID: String,
        BankBrchID: String,
        OperNo: String,
        DeviceID: String,
        RecordNum: Vec<u8>,
        SessionID: i32,
        RequestID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferBankToFutureReqField {
        is_null: bool,
        FutureAccount: String,
        FuturePwdFlag: u8,
        FutureAccPwd: String,
        TradeAmt: f64,
        CustFee: f64,
        CurrencyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferBankToFutureRspField {
        is_null: bool,
        RetCode: String,
        RetInfo: String,
        FutureAccount: String,
        TradeAmt: f64,
        CustFee: f64,
        CurrencyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferFutureToBankReqField {
        is_null: bool,
        FutureAccount: String,
        FuturePwdFlag: u8,
        FutureAccPwd: String,
        TradeAmt: f64,
        CustFee: f64,
        CurrencyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferFutureToBankRspField {
        is_null: bool,
        RetCode: String,
        RetInfo: String,
        FutureAccount: String,
        TradeAmt: f64,
        CustFee: f64,
        CurrencyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferQryBankReqField {
        is_null: bool,
        FutureAccount: String,
        FuturePwdFlag: u8,
        FutureAccPwd: String,
        CurrencyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferQryBankRspField {
        is_null: bool,
        RetCode: String,
        RetInfo: String,
        FutureAccount: String,
        TradeAmt: f64,
        UseAmt: f64,
        FetchAmt: f64,
        CurrencyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferQryDetailReqField {
        is_null: bool,
        FutureAccount: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferQryDetailRspField {
        is_null: bool,
        TradeDate: String,
        TradeTime: String,
        TradeCode: String,
        FutureSerial: i32,
        FutureID: String,
        FutureAccount: String,
        BankSerial: i32,
        BankID: String,
        BankBrchID: String,
        BankAccount: String,
        CertCode: String,
        CurrencyCode: String,
        TxAmount: f64,
        Flag: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct RspInfoField {
        is_null: bool,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeField {
        is_null: bool,
        ExchangeID: String,
        ExchangeName: String,
        ExchangeProperty: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct ProductField {
        is_null: bool,
        ProductName: String,
        ExchangeID: String,
        ProductClass: u8,
        VolumeMultiple: i32,
        PriceTick: f64,
        MaxMarketOrderVolume: i32,
        MinMarketOrderVolume: i32,
        MaxLimitOrderVolume: i32,
        MinLimitOrderVolume: i32,
        PositionType: u8,
        PositionDateType: u8,
        CloseDealType: u8,
        TradeCurrencyID: String,
        MortgageFundUseRange: u8,
        UnderlyingMultiple: f64,
        ProductID: String,
        ExchangeProductID: String,
        OpenLimitControlLevel: u8,
        OrderFreqControlLevel: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentField {
        is_null: bool,
        ExchangeID: String,
        InstrumentName: String,
        ProductClass: u8,
        DeliveryYear: i32,
        DeliveryMonth: i32,
        MaxMarketOrderVolume: i32,
        MinMarketOrderVolume: i32,
        MaxLimitOrderVolume: i32,
        MinLimitOrderVolume: i32,
        VolumeMultiple: i32,
        PriceTick: f64,
        CreateDate: String,
        OpenDate: String,
        ExpireDate: String,
        StartDelivDate: String,
        EndDelivDate: String,
        InstLifePhase: u8,
        IsTrading: i32,
        PositionType: u8,
        PositionDateType: u8,
        LongMarginRatio: f64,
        ShortMarginRatio: f64,
        MaxMarginSideAlgorithm: u8,
        StrikePrice: f64,
        OptionsType: u8,
        UnderlyingMultiple: f64,
        CombinationType: u8,
        InstrumentID: String,
        ExchangeInstID: String,
        ProductID: String,
        UnderlyingInstrID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerField {
        is_null: bool,
        BrokerID: String,
        BrokerAbbr: String,
        BrokerName: String,
        IsActive: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct TraderField {
        is_null: bool,
        ExchangeID: String,
        TraderID: String,
        ParticipantID: String,
        Password: String,
        InstallCount: i32,
        BrokerID: String,
        OrderCancelAlg: u8,
        TradeInstallCount: i32,
        MDInstallCount: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorField {
        is_null: bool,
        InvestorID: String,
        BrokerID: String,
        InvestorGroupID: String,
        InvestorName: String,
        IdentifiedCardType: u8,
        IdentifiedCardNo: String,
        IsActive: i32,
        Telephone: String,
        Address: String,
        OpenDate: String,
        Mobile: String,
        CommModelID: String,
        MarginModelID: String,
        IsOrderFreq: u8,
        IsOpenVolLimit: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingCodeField {
        is_null: bool,
        InvestorID: String,
        BrokerID: String,
        ExchangeID: String,
        ClientID: String,
        IsActive: i32,
        ClientIDType: u8,
        BranchID: String,
        BizType: u8,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct PartBrokerField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        ParticipantID: String,
        IsActive: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SuperUserField {
        is_null: bool,
        UserID: String,
        UserName: String,
        Password: String,
        IsActive: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SuperUserFunctionField {
        is_null: bool,
        UserID: String,
        FunctionCode: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorGroupField {
        is_null: bool,
        BrokerID: String,
        InvestorGroupID: String,
        InvestorGroupName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingAccountField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        PreMortgage: f64,
        PreCredit: f64,
        PreDeposit: f64,
        PreBalance: f64,
        PreMargin: f64,
        InterestBase: f64,
        Interest: f64,
        Deposit: f64,
        Withdraw: f64,
        FrozenMargin: f64,
        FrozenCash: f64,
        FrozenCommission: f64,
        CurrMargin: f64,
        CashIn: f64,
        Commission: f64,
        CloseProfit: f64,
        PositionProfit: f64,
        Balance: f64,
        Available: f64,
        WithdrawQuota: f64,
        Reserve: f64,
        TradingDay: String,
        SettlementID: i32,
        Credit: f64,
        Mortgage: f64,
        ExchangeMargin: f64,
        DeliveryMargin: f64,
        ExchangeDeliveryMargin: f64,
        ReserveBalance: f64,
        CurrencyID: String,
        PreFundMortgageIn: f64,
        PreFundMortgageOut: f64,
        FundMortgageIn: f64,
        FundMortgageOut: f64,
        FundMortgageAvailable: f64,
        MortgageableFund: f64,
        SpecProductMargin: f64,
        SpecProductFrozenMargin: f64,
        SpecProductCommission: f64,
        SpecProductFrozenCommission: f64,
        SpecProductPositionProfit: f64,
        SpecProductCloseProfit: f64,
        SpecProductPositionProfitByAlg: f64,
        SpecProductExchangeMargin: f64,
        BizType: u8,
        FrozenSwap: f64,
        RemainSwap: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorPositionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        PosiDirection: u8,
        HedgeFlag: u8,
        PositionDate: u8,
        YdPosition: i32,
        Position: i32,
        LongFrozen: i32,
        ShortFrozen: i32,
        LongFrozenAmount: f64,
        ShortFrozenAmount: f64,
        OpenVolume: i32,
        CloseVolume: i32,
        OpenAmount: f64,
        CloseAmount: f64,
        PositionCost: f64,
        PreMargin: f64,
        UseMargin: f64,
        FrozenMargin: f64,
        FrozenCash: f64,
        FrozenCommission: f64,
        CashIn: f64,
        Commission: f64,
        CloseProfit: f64,
        PositionProfit: f64,
        PreSettlementPrice: f64,
        SettlementPrice: f64,
        TradingDay: String,
        SettlementID: i32,
        OpenCost: f64,
        ExchangeMargin: f64,
        CombPosition: i32,
        CombLongFrozen: i32,
        CombShortFrozen: i32,
        CloseProfitByDate: f64,
        CloseProfitByTrade: f64,
        TodayPosition: i32,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        StrikeFrozen: i32,
        StrikeFrozenAmount: f64,
        AbandonFrozen: i32,
        ExchangeID: String,
        YdStrikeFrozen: i32,
        InvestUnitID: String,
        PositionCostOffset: f64,
        TasPosition: i32,
        TasPositionCost: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentMarginRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        IsRelative: i32,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentCommissionRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        ExchangeID: String,
        BizType: u8,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct DepthMarketDataField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        LastPrice: f64,
        PreSettlementPrice: f64,
        PreClosePrice: f64,
        PreOpenInterest: f64,
        OpenPrice: f64,
        HighestPrice: f64,
        LowestPrice: f64,
        Volume: i32,
        Turnover: f64,
        OpenInterest: f64,
        ClosePrice: f64,
        SettlementPrice: f64,
        UpperLimitPrice: f64,
        LowerLimitPrice: f64,
        PreDelta: f64,
        CurrDelta: f64,
        UpdateTime: String,
        UpdateMillisec: i32,
        BidPrice1: f64,
        BidVolume1: i32,
        AskPrice1: f64,
        AskVolume1: i32,
        BidPrice2: f64,
        BidVolume2: i32,
        AskPrice2: f64,
        AskVolume2: i32,
        BidPrice3: f64,
        BidVolume3: i32,
        AskPrice3: f64,
        AskVolume3: i32,
        BidPrice4: f64,
        BidVolume4: i32,
        AskPrice4: f64,
        AskVolume4: i32,
        BidPrice5: f64,
        BidVolume5: i32,
        AskPrice5: f64,
        AskVolume5: i32,
        AveragePrice: f64,
        ActionDay: String,
        InstrumentID: String,
        ExchangeInstID: String,
        BandingUpperPrice: f64,
        BandingLowerPrice: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentTradingRightField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        TradingRight: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerUserField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserName: String,
        UserType: u8,
        IsActive: i32,
        IsUsingOTP: i32,
        IsAuthForce: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerUserPasswordField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        Password: String,
        LastUpdateTime: String,
        LastLoginTime: String,
        ExpireDate: String,
        WeakExpireDate: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerUserFunctionField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        BrokerFunctionCode: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct TraderOfferField {
        is_null: bool,
        ExchangeID: String,
        TraderID: String,
        ParticipantID: String,
        Password: String,
        InstallID: i32,
        OrderLocalID: String,
        TraderConnectStatus: u8,
        ConnectRequestDate: String,
        ConnectRequestTime: String,
        LastReportDate: String,
        LastReportTime: String,
        ConnectDate: String,
        ConnectTime: String,
        StartDate: String,
        StartTime: String,
        TradingDay: String,
        BrokerID: String,
        MaxTradeID: String,
        MaxOrderMessageReference: Vec<u8>,
        OrderCancelAlg: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct SettlementInfoField {
        is_null: bool,
        TradingDay: String,
        SettlementID: i32,
        BrokerID: String,
        InvestorID: String,
        SequenceNo: i32,
        Content: Vec<u8>,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentMarginRateAdjustField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        IsRelative: i32,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeMarginRateField {
        is_null: bool,
        BrokerID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeMarginRateAdjustField {
        is_null: bool,
        BrokerID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        ExchLongMarginRatioByMoney: f64,
        ExchLongMarginRatioByVolume: f64,
        ExchShortMarginRatioByMoney: f64,
        ExchShortMarginRatioByVolume: f64,
        NoLongMarginRatioByMoney: f64,
        NoLongMarginRatioByVolume: f64,
        NoShortMarginRatioByMoney: f64,
        NoShortMarginRatioByVolume: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeRateField {
        is_null: bool,
        BrokerID: String,
        FromCurrencyID: String,
        FromCurrencyUnit: f64,
        ToCurrencyID: String,
        ExchangeRate: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct SettlementRefField {
        is_null: bool,
        TradingDay: String,
        SettlementID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct CurrentTimeField {
        is_null: bool,
        CurrDate: String,
        CurrTime: String,
        CurrMillisec: i32,
        ActionDay: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CommPhaseField {
        is_null: bool,
        TradingDay: String,
        CommPhaseNo: u16,
        SystemID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct LoginInfoField {
        is_null: bool,
        FrontID: i32,
        SessionID: i32,
        BrokerID: String,
        UserID: String,
        LoginDate: String,
        LoginTime: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        SystemName: String,
        PasswordDeprecated: Vec<u8>,
        MaxOrderRef: String,
        SHFETime: String,
        DCETime: String,
        CZCETime: String,
        FFEXTime: String,
        MacAddress: String,
        OneTimePassword: String,
        INETime: String,
        IsQryControl: i32,
        LoginRemark: String,
        Password: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct LogoutAllField {
        is_null: bool,
        FrontID: i32,
        SessionID: i32,
        SystemName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct FrontStatusField {
        is_null: bool,
        FrontID: i32,
        LastReportDate: String,
        LastReportTime: String,
        IsActive: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct UserPasswordUpdateField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        OldPassword: String,
        NewPassword: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderRef: String,
        UserID: String,
        OrderPriceType: u8,
        Direction: u8,
        CombOffsetFlag: String,
        CombHedgeFlag: String,
        LimitPrice: f64,
        VolumeTotalOriginal: i32,
        TimeCondition: u8,
        GTDDate: String,
        VolumeCondition: u8,
        MinVolume: i32,
        ContingentCondition: u8,
        StopPrice: f64,
        ForceCloseReason: u8,
        IsAutoSuspend: i32,
        BusinessUnit: String,
        RequestID: i32,
        UserForceClose: i32,
        IsSwapOrder: i32,
        ExchangeID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        ClientID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct OrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderRef: String,
        UserID: String,
        OrderPriceType: u8,
        Direction: u8,
        CombOffsetFlag: String,
        CombHedgeFlag: String,
        LimitPrice: f64,
        VolumeTotalOriginal: i32,
        TimeCondition: u8,
        GTDDate: String,
        VolumeCondition: u8,
        MinVolume: i32,
        ContingentCondition: u8,
        StopPrice: f64,
        ForceCloseReason: u8,
        IsAutoSuspend: i32,
        BusinessUnit: String,
        RequestID: i32,
        OrderLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        OrderSysID: String,
        OrderSource: u8,
        OrderStatus: u8,
        OrderType: u8,
        VolumeTraded: i32,
        VolumeTotal: i32,
        InsertDate: String,
        InsertTime: String,
        ActiveTime: String,
        SuspendTime: String,
        UpdateTime: String,
        CancelTime: String,
        ActiveTraderID: String,
        ClearingPartID: String,
        SequenceNo: i32,
        FrontID: i32,
        SessionID: i32,
        UserProductInfo: String,
        StatusMsg: String,
        UserForceClose: i32,
        ActiveUserID: String,
        BrokerOrderSeq: i32,
        RelativeOrderSysID: String,
        ZCETotalTradedVolume: i32,
        IsSwapOrder: i32,
        BranchID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        MacAddress: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderField {
        is_null: bool,
        OrderPriceType: u8,
        Direction: u8,
        CombOffsetFlag: String,
        CombHedgeFlag: String,
        LimitPrice: f64,
        VolumeTotalOriginal: i32,
        TimeCondition: u8,
        GTDDate: String,
        VolumeCondition: u8,
        MinVolume: i32,
        ContingentCondition: u8,
        StopPrice: f64,
        ForceCloseReason: u8,
        IsAutoSuspend: i32,
        BusinessUnit: String,
        RequestID: i32,
        OrderLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        OrderSysID: String,
        OrderSource: u8,
        OrderStatus: u8,
        OrderType: u8,
        VolumeTraded: i32,
        VolumeTotal: i32,
        InsertDate: String,
        InsertTime: String,
        ActiveTime: String,
        SuspendTime: String,
        UpdateTime: String,
        CancelTime: String,
        ActiveTraderID: String,
        ClearingPartID: String,
        SequenceNo: i32,
        BranchID: String,
        MacAddress: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderInsertErrorField {
        is_null: bool,
        ExchangeID: String,
        ParticipantID: String,
        TraderID: String,
        InstallID: i32,
        OrderLocalID: String,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderActionRef: i32,
        OrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        OrderSysID: String,
        ActionFlag: u8,
        LimitPrice: f64,
        VolumeChange: i32,
        UserID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct OrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderActionRef: i32,
        OrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        OrderSysID: String,
        ActionFlag: u8,
        LimitPrice: f64,
        VolumeChange: i32,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        OrderLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        StatusMsg: String,
        BranchID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderActionField {
        is_null: bool,
        ExchangeID: String,
        OrderSysID: String,
        ActionFlag: u8,
        LimitPrice: f64,
        VolumeChange: i32,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        OrderLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        BranchID: String,
        MacAddress: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderActionErrorField {
        is_null: bool,
        ExchangeID: String,
        OrderSysID: String,
        TraderID: String,
        InstallID: i32,
        OrderLocalID: String,
        ActionLocalID: String,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeTradeField {
        is_null: bool,
        ExchangeID: String,
        TradeID: String,
        Direction: u8,
        OrderSysID: String,
        ParticipantID: String,
        ClientID: String,
        TradingRole: u8,
        OffsetFlag: u8,
        HedgeFlag: u8,
        Price: f64,
        Volume: i32,
        TradeDate: String,
        TradeTime: String,
        TradeType: u8,
        PriceSource: u8,
        TraderID: String,
        OrderLocalID: String,
        ClearingPartID: String,
        BusinessUnit: String,
        SequenceNo: i32,
        TradeSource: u8,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradeField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderRef: String,
        UserID: String,
        ExchangeID: String,
        TradeID: String,
        Direction: u8,
        OrderSysID: String,
        ParticipantID: String,
        ClientID: String,
        TradingRole: u8,
        OffsetFlag: u8,
        HedgeFlag: u8,
        Price: f64,
        Volume: i32,
        TradeDate: String,
        TradeTime: String,
        TradeType: u8,
        PriceSource: u8,
        TraderID: String,
        OrderLocalID: String,
        ClearingPartID: String,
        BusinessUnit: String,
        SequenceNo: i32,
        TradingDay: String,
        SettlementID: i32,
        BrokerOrderSeq: i32,
        TradeSource: u8,
        InvestUnitID: String,
        InstrumentID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct UserSessionField {
        is_null: bool,
        FrontID: i32,
        SessionID: i32,
        BrokerID: String,
        UserID: String,
        LoginDate: String,
        LoginTime: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        MacAddress: String,
        LoginRemark: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMaxOrderVolumeField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        Direction: u8,
        OffsetFlag: u8,
        HedgeFlag: u8,
        MaxVolume: i32,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SettlementInfoConfirmField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ConfirmDate: String,
        ConfirmTime: String,
        SettlementID: i32,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDepositField {
        is_null: bool,
        DepositSeqNo: Vec<u8>,
        BrokerID: String,
        InvestorID: String,
        Deposit: f64,
        IsForce: i32,
        CurrencyID: String,
        IsFromSopt: i32,
        TradingPassword: String,
        IsSecAgentTranfer: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncFundMortgageField {
        is_null: bool,
        MortgageSeqNo: Vec<u8>,
        BrokerID: String,
        InvestorID: String,
        FromCurrencyID: String,
        MortgageAmount: f64,
        ToCurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerSyncField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingInvestorField {
        is_null: bool,
        InvestorID: String,
        BrokerID: String,
        InvestorGroupID: String,
        InvestorName: String,
        IdentifiedCardType: u8,
        IdentifiedCardNo: String,
        IsActive: i32,
        Telephone: String,
        Address: String,
        OpenDate: String,
        Mobile: String,
        CommModelID: String,
        MarginModelID: String,
        IsOrderFreq: u8,
        IsOpenVolLimit: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingTradingCodeField {
        is_null: bool,
        InvestorID: String,
        BrokerID: String,
        ExchangeID: String,
        ClientID: String,
        IsActive: i32,
        ClientIDType: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingInvestorGroupField {
        is_null: bool,
        BrokerID: String,
        InvestorGroupID: String,
        InvestorGroupName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingTradingAccountField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        PreMortgage: f64,
        PreCredit: f64,
        PreDeposit: f64,
        PreBalance: f64,
        PreMargin: f64,
        InterestBase: f64,
        Interest: f64,
        Deposit: f64,
        Withdraw: f64,
        FrozenMargin: f64,
        FrozenCash: f64,
        FrozenCommission: f64,
        CurrMargin: f64,
        CashIn: f64,
        Commission: f64,
        CloseProfit: f64,
        PositionProfit: f64,
        Balance: f64,
        Available: f64,
        WithdrawQuota: f64,
        Reserve: f64,
        TradingDay: String,
        SettlementID: i32,
        Credit: f64,
        Mortgage: f64,
        ExchangeMargin: f64,
        DeliveryMargin: f64,
        ExchangeDeliveryMargin: f64,
        ReserveBalance: f64,
        CurrencyID: String,
        PreFundMortgageIn: f64,
        PreFundMortgageOut: f64,
        FundMortgageIn: f64,
        FundMortgageOut: f64,
        FundMortgageAvailable: f64,
        MortgageableFund: f64,
        SpecProductMargin: f64,
        SpecProductFrozenMargin: f64,
        SpecProductCommission: f64,
        SpecProductFrozenCommission: f64,
        SpecProductPositionProfit: f64,
        SpecProductCloseProfit: f64,
        SpecProductPositionProfitByAlg: f64,
        SpecProductExchangeMargin: f64,
        FrozenSwap: f64,
        RemainSwap: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingInvestorPositionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        PosiDirection: u8,
        HedgeFlag: u8,
        PositionDate: u8,
        YdPosition: i32,
        Position: i32,
        LongFrozen: i32,
        ShortFrozen: i32,
        LongFrozenAmount: f64,
        ShortFrozenAmount: f64,
        OpenVolume: i32,
        CloseVolume: i32,
        OpenAmount: f64,
        CloseAmount: f64,
        PositionCost: f64,
        PreMargin: f64,
        UseMargin: f64,
        FrozenMargin: f64,
        FrozenCash: f64,
        FrozenCommission: f64,
        CashIn: f64,
        Commission: f64,
        CloseProfit: f64,
        PositionProfit: f64,
        PreSettlementPrice: f64,
        SettlementPrice: f64,
        TradingDay: String,
        SettlementID: i32,
        OpenCost: f64,
        ExchangeMargin: f64,
        CombPosition: i32,
        CombLongFrozen: i32,
        CombShortFrozen: i32,
        CloseProfitByDate: f64,
        CloseProfitByTrade: f64,
        TodayPosition: i32,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        StrikeFrozen: i32,
        StrikeFrozenAmount: f64,
        AbandonFrozen: i32,
        ExchangeID: String,
        YdStrikeFrozen: i32,
        InvestUnitID: String,
        PositionCostOffset: f64,
        TasPosition: i32,
        TasPositionCost: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingInstrumentMarginRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        IsRelative: i32,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingInstrumentCommissionRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncingInstrumentTradingRightField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        TradingRight: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        OrderSysID: String,
        InsertTimeStart: String,
        InsertTimeEnd: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTradeField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        TradeID: String,
        TradeTimeStart: String,
        TradeTimeEnd: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPositionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTradingAccountField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CurrencyID: String,
        BizType: u8,
        AccountID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTradingCodeField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        ClientID: String,
        ClientIDType: u8,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorGroupField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentMarginRateField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentCommissionRateField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentTradingRightField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBrokerField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTraderField {
        is_null: bool,
        ExchangeID: String,
        ParticipantID: String,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySuperUserFunctionField {
        is_null: bool,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryUserSessionField {
        is_null: bool,
        FrontID: i32,
        SessionID: i32,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryPartBrokerField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        ParticipantID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryFrontStatusField {
        is_null: bool,
        FrontID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeOrderField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeOrderActionField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySuperUserField {
        is_null: bool,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeField {
        is_null: bool,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryProductField {
        is_null: bool,
        ProductClass: u8,
        ExchangeID: String,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        ExchangeInstID: String,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryDepthMarketDataField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        ProductClass: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBrokerUserField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBrokerUserFunctionField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTraderOfferField {
        is_null: bool,
        ExchangeID: String,
        ParticipantID: String,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySyncDepositField {
        is_null: bool,
        BrokerID: String,
        DepositSeqNo: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySettlementInfoField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        TradingDay: String,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeMarginRateField {
        is_null: bool,
        BrokerID: String,
        HedgeFlag: u8,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeMarginRateAdjustField {
        is_null: bool,
        BrokerID: String,
        HedgeFlag: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeRateField {
        is_null: bool,
        BrokerID: String,
        FromCurrencyID: String,
        ToCurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySyncFundMortgageField {
        is_null: bool,
        BrokerID: String,
        MortgageSeqNo: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct QryHisOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        OrderSysID: String,
        InsertTimeStart: String,
        InsertTimeEnd: String,
        TradingDay: String,
        SettlementID: i32,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionInstrMiniMarginField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        MinMargin: f64,
        ValueMethod: u8,
        IsRelative: i32,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionInstrMarginAdjustField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        SShortMarginRatioByMoney: f64,
        SShortMarginRatioByVolume: f64,
        HShortMarginRatioByMoney: f64,
        HShortMarginRatioByVolume: f64,
        AShortMarginRatioByMoney: f64,
        AShortMarginRatioByVolume: f64,
        IsRelative: i32,
        MShortMarginRatioByMoney: f64,
        MShortMarginRatioByVolume: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionInstrCommRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        StrikeRatioByMoney: f64,
        StrikeRatioByVolume: f64,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionInstrTradeCostField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        FixedMargin: f64,
        MiniMargin: f64,
        Royalty: f64,
        ExchFixedMargin: f64,
        ExchMiniMargin: f64,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOptionInstrTradeCostField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        InputPrice: f64,
        UnderlyingPrice: f64,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOptionInstrCommRateField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct IndexPriceField {
        is_null: bool,
        BrokerID: String,
        ClosePrice: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputExecOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExecOrderRef: String,
        UserID: String,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        OffsetFlag: u8,
        HedgeFlag: u8,
        ActionType: u8,
        PosiDirection: u8,
        ReservePositionFlag: u8,
        CloseFlag: u8,
        ExchangeID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        ClientID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputExecOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExecOrderActionRef: i32,
        ExecOrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        ExecOrderSysID: String,
        ActionFlag: u8,
        UserID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExecOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExecOrderRef: String,
        UserID: String,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        OffsetFlag: u8,
        HedgeFlag: u8,
        ActionType: u8,
        PosiDirection: u8,
        ReservePositionFlag: u8,
        CloseFlag: u8,
        ExecOrderLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        ExecOrderSysID: String,
        InsertDate: String,
        InsertTime: String,
        CancelTime: String,
        ExecResult: u8,
        ClearingPartID: String,
        SequenceNo: i32,
        FrontID: i32,
        SessionID: i32,
        UserProductInfo: String,
        StatusMsg: String,
        ActiveUserID: String,
        BrokerExecOrderSeq: i32,
        BranchID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        MacAddress: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExecOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExecOrderActionRef: i32,
        ExecOrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        ExecOrderSysID: String,
        ActionFlag: u8,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        ExecOrderLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        ActionType: u8,
        StatusMsg: String,
        BranchID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExecOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        ExecOrderSysID: String,
        InsertTimeStart: String,
        InsertTimeEnd: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeExecOrderField {
        is_null: bool,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        OffsetFlag: u8,
        HedgeFlag: u8,
        ActionType: u8,
        PosiDirection: u8,
        ReservePositionFlag: u8,
        CloseFlag: u8,
        ExecOrderLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        ExecOrderSysID: String,
        InsertDate: String,
        InsertTime: String,
        CancelTime: String,
        ExecResult: u8,
        ClearingPartID: String,
        SequenceNo: i32,
        BranchID: String,
        MacAddress: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeExecOrderField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExecOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeExecOrderActionField {
        is_null: bool,
        ExchangeID: String,
        ExecOrderSysID: String,
        ActionFlag: u8,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        ExecOrderLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        ActionType: u8,
        BranchID: String,
        MacAddress: String,
        Volume: i32,
        IPAddress: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeExecOrderActionField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ErrExecOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExecOrderRef: String,
        UserID: String,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        OffsetFlag: u8,
        HedgeFlag: u8,
        ActionType: u8,
        PosiDirection: u8,
        ReservePositionFlag: u8,
        CloseFlag: u8,
        ExchangeID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        ClientID: String,
        MacAddress: String,
        ErrorID: i32,
        ErrorMsg: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryErrExecOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ErrExecOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExecOrderActionRef: i32,
        ExecOrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        ExecOrderSysID: String,
        ActionFlag: u8,
        UserID: String,
        InvestUnitID: String,
        MacAddress: String,
        ErrorID: i32,
        ErrorMsg: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryErrExecOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionInstrTradingRightField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        Direction: u8,
        TradingRight: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOptionInstrTradingRightField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        Direction: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputForQuoteField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ForQuoteRef: String,
        UserID: String,
        ExchangeID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ForQuoteField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ForQuoteRef: String,
        UserID: String,
        ForQuoteLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        InsertDate: String,
        InsertTime: String,
        ForQuoteStatus: u8,
        FrontID: i32,
        SessionID: i32,
        StatusMsg: String,
        ActiveUserID: String,
        BrokerForQutoSeq: i32,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryForQuoteField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InsertTimeStart: String,
        InsertTimeEnd: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeForQuoteField {
        is_null: bool,
        ForQuoteLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        InsertDate: String,
        InsertTime: String,
        ForQuoteStatus: u8,
        MacAddress: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeForQuoteField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputQuoteField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        QuoteRef: String,
        UserID: String,
        AskPrice: f64,
        BidPrice: f64,
        AskVolume: i32,
        BidVolume: i32,
        RequestID: i32,
        BusinessUnit: String,
        AskOffsetFlag: u8,
        BidOffsetFlag: u8,
        AskHedgeFlag: u8,
        BidHedgeFlag: u8,
        AskOrderRef: String,
        BidOrderRef: String,
        ForQuoteSysID: String,
        ExchangeID: String,
        InvestUnitID: String,
        ClientID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        ReplaceSysID: String,
        TimeCondition: u8,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct InputQuoteActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        QuoteActionRef: i32,
        QuoteRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        QuoteSysID: String,
        ActionFlag: u8,
        UserID: String,
        InvestUnitID: String,
        ClientID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QuoteField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        QuoteRef: String,
        UserID: String,
        AskPrice: f64,
        BidPrice: f64,
        AskVolume: i32,
        BidVolume: i32,
        RequestID: i32,
        BusinessUnit: String,
        AskOffsetFlag: u8,
        BidOffsetFlag: u8,
        AskHedgeFlag: u8,
        BidHedgeFlag: u8,
        QuoteLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        NotifySequence: i32,
        OrderSubmitStatus: u8,
        TradingDay: String,
        SettlementID: i32,
        QuoteSysID: String,
        InsertDate: String,
        InsertTime: String,
        CancelTime: String,
        QuoteStatus: u8,
        ClearingPartID: String,
        SequenceNo: i32,
        AskOrderSysID: String,
        BidOrderSysID: String,
        FrontID: i32,
        SessionID: i32,
        UserProductInfo: String,
        StatusMsg: String,
        ActiveUserID: String,
        BrokerQuoteSeq: i32,
        AskOrderRef: String,
        BidOrderRef: String,
        ForQuoteSysID: String,
        BranchID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        MacAddress: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
        ReplaceSysID: String,
        TimeCondition: u8,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QuoteActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        QuoteActionRef: i32,
        QuoteRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        QuoteSysID: String,
        ActionFlag: u8,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        QuoteLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        StatusMsg: String,
        BranchID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryQuoteField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        QuoteSysID: String,
        InsertTimeStart: String,
        InsertTimeEnd: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeQuoteField {
        is_null: bool,
        AskPrice: f64,
        BidPrice: f64,
        AskVolume: i32,
        BidVolume: i32,
        RequestID: i32,
        BusinessUnit: String,
        AskOffsetFlag: u8,
        BidOffsetFlag: u8,
        AskHedgeFlag: u8,
        BidHedgeFlag: u8,
        QuoteLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        NotifySequence: i32,
        OrderSubmitStatus: u8,
        TradingDay: String,
        SettlementID: i32,
        QuoteSysID: String,
        InsertDate: String,
        InsertTime: String,
        CancelTime: String,
        QuoteStatus: u8,
        ClearingPartID: String,
        SequenceNo: i32,
        AskOrderSysID: String,
        BidOrderSysID: String,
        ForQuoteSysID: String,
        BranchID: String,
        MacAddress: String,
        ExchangeInstID: String,
        IPAddress: String,
        TimeCondition: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeQuoteField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryQuoteActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeQuoteActionField {
        is_null: bool,
        ExchangeID: String,
        QuoteSysID: String,
        ActionFlag: u8,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        QuoteLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        MacAddress: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeQuoteActionField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionInstrDeltaField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        Delta: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ForQuoteRspField {
        is_null: bool,
        TradingDay: String,
        ForQuoteSysID: String,
        ForQuoteTime: String,
        ActionDay: String,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct StrikeOffsetField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        Offset: f64,
        OffsetType: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryStrikeOffsetField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputBatchOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderActionRef: i32,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        UserID: String,
        InvestUnitID: String,
        MacAddress: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BatchOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderActionRef: i32,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        StatusMsg: String,
        InvestUnitID: String,
        MacAddress: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeBatchOrderActionField {
        is_null: bool,
        ExchangeID: String,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        MacAddress: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBatchOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CombInstrumentGuardField {
        is_null: bool,
        BrokerID: String,
        GuarantRatio: f64,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCombInstrumentGuardField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputCombActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CombActionRef: String,
        UserID: String,
        Direction: u8,
        Volume: i32,
        CombDirection: u8,
        HedgeFlag: u8,
        ExchangeID: String,
        MacAddress: String,
        InvestUnitID: String,
        FrontID: i32,
        SessionID: i32,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CombActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CombActionRef: String,
        UserID: String,
        Direction: u8,
        Volume: i32,
        CombDirection: u8,
        HedgeFlag: u8,
        ActionLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        ActionStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        SequenceNo: i32,
        FrontID: i32,
        SessionID: i32,
        UserProductInfo: String,
        StatusMsg: String,
        MacAddress: String,
        ComTradeID: String,
        BranchID: String,
        InvestUnitID: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCombActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeCombActionField {
        is_null: bool,
        Direction: u8,
        Volume: i32,
        CombDirection: u8,
        HedgeFlag: u8,
        ActionLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        ActionStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        SequenceNo: i32,
        MacAddress: String,
        ComTradeID: String,
        BranchID: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeCombActionField {
        is_null: bool,
        ParticipantID: String,
        ClientID: String,
        ExchangeID: String,
        TraderID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ProductExchRateField {
        is_null: bool,
        QuoteCurrencyID: String,
        ExchangeRate: f64,
        ExchangeID: String,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryProductExchRateField {
        is_null: bool,
        ExchangeID: String,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryForQuoteParamField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ForQuoteParamField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        LastPrice: f64,
        PriceInterval: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MMOptionInstrCommRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        StrikeRatioByMoney: f64,
        StrikeRatioByVolume: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMMOptionInstrCommRateField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MMInstrumentCommissionRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMMInstrumentCommissionRateField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentOrderCommRateField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        OrderCommByVolume: f64,
        OrderActionCommByVolume: f64,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
        OrderCommByTrade: f64,
        OrderActionCommByTrade: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentOrderCommRateField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradeParamField {
        is_null: bool,
        BrokerID: String,
        TradeParamID: u8,
        TradeParamValue: Vec<u8>,
        Memo: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentMarginRateULField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct FutureLimitPosiParamField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        SpecOpenVolume: i32,
        ArbiOpenVolume: i32,
        OpenVolume: i32,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct LoginForbiddenIPField {
        is_null: bool,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct IPListField {
        is_null: bool,
        IsWhite: i32,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputOptionSelfCloseField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OptionSelfCloseRef: String,
        UserID: String,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        HedgeFlag: u8,
        OptSelfCloseFlag: u8,
        ExchangeID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        ClientID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InputOptionSelfCloseActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OptionSelfCloseActionRef: i32,
        OptionSelfCloseRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        OptionSelfCloseSysID: String,
        ActionFlag: u8,
        UserID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionSelfCloseField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OptionSelfCloseRef: String,
        UserID: String,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        HedgeFlag: u8,
        OptSelfCloseFlag: u8,
        OptionSelfCloseLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        OptionSelfCloseSysID: String,
        InsertDate: String,
        InsertTime: String,
        CancelTime: String,
        ExecResult: u8,
        ClearingPartID: String,
        SequenceNo: i32,
        FrontID: i32,
        SessionID: i32,
        UserProductInfo: String,
        StatusMsg: String,
        ActiveUserID: String,
        BrokerOptionSelfCloseSeq: i32,
        BranchID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        MacAddress: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OptionSelfCloseActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OptionSelfCloseActionRef: i32,
        OptionSelfCloseRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        OptionSelfCloseSysID: String,
        ActionFlag: u8,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        OptionSelfCloseLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        StatusMsg: String,
        BranchID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOptionSelfCloseField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        OptionSelfCloseSysID: String,
        InsertTimeStart: String,
        InsertTimeEnd: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeOptionSelfCloseField {
        is_null: bool,
        Volume: i32,
        RequestID: i32,
        BusinessUnit: String,
        HedgeFlag: u8,
        OptSelfCloseFlag: u8,
        OptionSelfCloseLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        OptionSelfCloseSysID: String,
        InsertDate: String,
        InsertTime: String,
        CancelTime: String,
        ExecResult: u8,
        ClearingPartID: String,
        SequenceNo: i32,
        BranchID: String,
        MacAddress: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryOptionSelfCloseActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeOptionSelfCloseActionField {
        is_null: bool,
        ExchangeID: String,
        OptionSelfCloseSysID: String,
        ActionFlag: u8,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        OptionSelfCloseLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        BranchID: String,
        MacAddress: String,
        OptSelfCloseFlag: u8,
        IPAddress: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDelaySwapField {
        is_null: bool,
        DelaySwapSeqNo: Vec<u8>,
        BrokerID: String,
        InvestorID: String,
        FromCurrencyID: String,
        FromAmount: f64,
        FromFrozenSwap: f64,
        FromRemainSwap: f64,
        ToCurrencyID: String,
        ToAmount: f64,
        IsManualSwap: i32,
        IsAllRemainSetZero: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySyncDelaySwapField {
        is_null: bool,
        BrokerID: String,
        DelaySwapSeqNo: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestUnitField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InvestUnitID: String,
        InvestorUnitName: String,
        InvestorGroupID: String,
        CommModelID: String,
        MarginModelID: String,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestUnitField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SecAgentCheckModeField {
        is_null: bool,
        InvestorID: String,
        BrokerID: String,
        CurrencyID: String,
        BrokerSecAgentID: String,
        CheckSelfAccount: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SecAgentTradeInfoField {
        is_null: bool,
        BrokerID: String,
        BrokerSecAgentID: String,
        InvestorID: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        LastPrice: f64,
        PreSettlementPrice: f64,
        PreClosePrice: f64,
        PreOpenInterest: f64,
        OpenPrice: f64,
        HighestPrice: f64,
        LowestPrice: f64,
        Volume: i32,
        Turnover: f64,
        OpenInterest: f64,
        ClosePrice: f64,
        SettlementPrice: f64,
        UpperLimitPrice: f64,
        LowerLimitPrice: f64,
        PreDelta: f64,
        CurrDelta: f64,
        UpdateTime: String,
        UpdateMillisec: i32,
        ActionDay: String,
        InstrumentID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataBaseField {
        is_null: bool,
        TradingDay: String,
        PreSettlementPrice: f64,
        PreClosePrice: f64,
        PreOpenInterest: f64,
        PreDelta: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataStaticField {
        is_null: bool,
        OpenPrice: f64,
        HighestPrice: f64,
        LowestPrice: f64,
        ClosePrice: f64,
        UpperLimitPrice: f64,
        LowerLimitPrice: f64,
        SettlementPrice: f64,
        CurrDelta: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataLastMatchField {
        is_null: bool,
        LastPrice: f64,
        Volume: i32,
        Turnover: f64,
        OpenInterest: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataBestPriceField {
        is_null: bool,
        BidPrice1: f64,
        BidVolume1: i32,
        AskPrice1: f64,
        AskVolume1: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataBid23Field {
        is_null: bool,
        BidPrice2: f64,
        BidVolume2: i32,
        BidPrice3: f64,
        BidVolume3: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataAsk23Field {
        is_null: bool,
        AskPrice2: f64,
        AskVolume2: i32,
        AskPrice3: f64,
        AskVolume3: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataBid45Field {
        is_null: bool,
        BidPrice4: f64,
        BidVolume4: i32,
        BidPrice5: f64,
        BidVolume5: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataAsk45Field {
        is_null: bool,
        AskPrice4: f64,
        AskVolume4: i32,
        AskPrice5: f64,
        AskVolume5: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataUpdateTimeField {
        is_null: bool,
        UpdateTime: String,
        UpdateMillisec: i32,
        ActionDay: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataBandingPriceField {
        is_null: bool,
        BandingUpperPrice: f64,
        BandingLowerPrice: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataExchangeField {
        is_null: bool,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SpecificInstrumentField {
        is_null: bool,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InstrumentStatusField {
        is_null: bool,
        ExchangeID: String,
        SettlementGroupID: String,
        InstrumentStatus: u8,
        TradingSegmentSN: i32,
        EnterTime: String,
        EnterReason: u8,
        ExchangeInstID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentStatusField {
        is_null: bool,
        ExchangeID: String,
        ExchangeInstID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorAccountField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct PositionProfitAlgorithmField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        Algorithm: u8,
        Memo: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct DiscountField {
        is_null: bool,
        BrokerID: String,
        InvestorRange: u8,
        InvestorID: String,
        Discount: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTransferBankField {
        is_null: bool,
        BankID: String,
        BankBrchID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferBankField {
        is_null: bool,
        BankID: String,
        BankBrchID: String,
        BankName: String,
        IsActive: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPositionDetailField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorPositionDetailField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        Direction: u8,
        OpenDate: String,
        TradeID: String,
        Volume: i32,
        OpenPrice: f64,
        TradingDay: String,
        SettlementID: i32,
        TradeType: u8,
        ExchangeID: String,
        CloseProfitByDate: f64,
        CloseProfitByTrade: f64,
        PositionProfitByDate: f64,
        PositionProfitByTrade: f64,
        Margin: f64,
        ExchMargin: f64,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        LastSettlementPrice: f64,
        SettlementPrice: f64,
        CloseVolume: i32,
        CloseAmount: f64,
        TimeFirstVolume: i32,
        InvestUnitID: String,
        SpecPosiType: u8,
        InstrumentID: String,
        CombInstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        Password: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MDTraderOfferField {
        is_null: bool,
        ExchangeID: String,
        TraderID: String,
        ParticipantID: String,
        Password: String,
        InstallID: i32,
        OrderLocalID: String,
        TraderConnectStatus: u8,
        ConnectRequestDate: String,
        ConnectRequestTime: String,
        LastReportDate: String,
        LastReportTime: String,
        ConnectDate: String,
        ConnectTime: String,
        StartDate: String,
        StartTime: String,
        TradingDay: String,
        BrokerID: String,
        MaxTradeID: String,
        MaxOrderMessageReference: Vec<u8>,
        OrderCancelAlg: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMDTraderOfferField {
        is_null: bool,
        ExchangeID: String,
        ParticipantID: String,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryNoticeField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct NoticeField {
        is_null: bool,
        BrokerID: String,
        Content: Vec<u8>,
        SequenceLabel: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct UserRightField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserRightType: u8,
        IsForbidden: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySettlementInfoConfirmField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct LoadSettlementInfoField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerWithdrawAlgorithmField {
        is_null: bool,
        BrokerID: String,
        WithdrawAlgorithm: u8,
        UsingRatio: f64,
        IncludeCloseProfit: u8,
        AllWithoutTrade: u8,
        AvailIncludeCloseProfit: u8,
        IsBrokerUserEvent: i32,
        CurrencyID: String,
        FundMortgageRatio: f64,
        BalanceAlgorithm: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordUpdateV1Field {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OldPassword: String,
        NewPassword: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordUpdateField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        OldPassword: String,
        NewPassword: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCombinationLegField {
        is_null: bool,
        LegID: i32,
        CombInstrumentID: String,
        LegInstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySyncStatusField {
        is_null: bool,
        TradingDay: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CombinationLegField {
        is_null: bool,
        LegID: i32,
        Direction: u8,
        LegMultiple: i32,
        ImplyLevel: i32,
        CombInstrumentID: String,
        LegInstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncStatusField {
        is_null: bool,
        TradingDay: String,
        DataSyncStatus: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct QryLinkManField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct LinkManField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        PersonType: u8,
        IdentifiedCardType: u8,
        IdentifiedCardNo: String,
        PersonName: String,
        Telephone: String,
        Address: String,
        ZipCode: String,
        Priority: i32,
        UOAZipCode: String,
        PersonFullName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBrokerUserEventField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserEventType: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerUserEventField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        UserEventType: u8,
        EventSequenceNo: i32,
        EventDate: String,
        EventTime: String,
        UserEventInfo: String,
        InvestorID: String,
        InstrumentID: String,
        DRIdentityID: i32,
        TradingDay: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryContractBankField {
        is_null: bool,
        BrokerID: String,
        BankID: String,
        BankBrchID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ContractBankField {
        is_null: bool,
        BrokerID: String,
        BankID: String,
        BankBrchID: String,
        BankName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorPositionCombineDetailField {
        is_null: bool,
        TradingDay: String,
        OpenDate: String,
        ExchangeID: String,
        SettlementID: i32,
        BrokerID: String,
        InvestorID: String,
        ComTradeID: String,
        TradeID: String,
        HedgeFlag: u8,
        Direction: u8,
        TotalAmt: i32,
        Margin: f64,
        ExchMargin: f64,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        LegID: i32,
        LegMultiple: i32,
        TradeGroupID: i32,
        InvestUnitID: String,
        InstrumentID: String,
        CombInstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ParkedOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderRef: String,
        UserID: String,
        OrderPriceType: u8,
        Direction: u8,
        CombOffsetFlag: String,
        CombHedgeFlag: String,
        LimitPrice: f64,
        VolumeTotalOriginal: i32,
        TimeCondition: u8,
        GTDDate: String,
        VolumeCondition: u8,
        MinVolume: i32,
        ContingentCondition: u8,
        StopPrice: f64,
        ForceCloseReason: u8,
        IsAutoSuspend: i32,
        BusinessUnit: String,
        RequestID: i32,
        UserForceClose: i32,
        ExchangeID: String,
        ParkedOrderID: String,
        UserType: u8,
        Status: u8,
        ErrorID: i32,
        ErrorMsg: String,
        IsSwapOrder: i32,
        AccountID: String,
        CurrencyID: String,
        ClientID: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ParkedOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderActionRef: i32,
        OrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        OrderSysID: String,
        ActionFlag: u8,
        LimitPrice: f64,
        VolumeChange: i32,
        UserID: String,
        ParkedOrderActionID: String,
        UserType: u8,
        Status: u8,
        ErrorID: i32,
        ErrorMsg: String,
        InvestUnitID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryParkedOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryParkedOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RemoveParkedOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ParkedOrderID: String,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RemoveParkedOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ParkedOrderActionID: String,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorWithdrawAlgorithmField {
        is_null: bool,
        BrokerID: String,
        InvestorRange: u8,
        InvestorID: String,
        UsingRatio: f64,
        CurrencyID: String,
        FundMortgageRatio: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPositionCombineDetailField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        CombInstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MarketDataAveragePriceField {
        is_null: bool,
        AveragePrice: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct VerifyInvestorPasswordField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        Password: String,
    }
    #[derive(Debug, Clone, Default)]
    struct UserIPField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        MacAddress: String,
        IPAddress: String,
        IPMask: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingNoticeInfoField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        SendTime: String,
        FieldContent: Vec<u8>,
        SequenceSeries: u16,
        SequenceNo: i32,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingNoticeField {
        is_null: bool,
        BrokerID: String,
        InvestorRange: u8,
        InvestorID: String,
        SequenceSeries: u16,
        UserID: String,
        SendTime: String,
        SequenceNo: i32,
        FieldContent: Vec<u8>,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTradingNoticeField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryErrOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ErrOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderRef: String,
        UserID: String,
        OrderPriceType: u8,
        Direction: u8,
        CombOffsetFlag: String,
        CombHedgeFlag: String,
        LimitPrice: f64,
        VolumeTotalOriginal: i32,
        TimeCondition: u8,
        GTDDate: String,
        VolumeCondition: u8,
        MinVolume: i32,
        ContingentCondition: u8,
        StopPrice: f64,
        ForceCloseReason: u8,
        IsAutoSuspend: i32,
        BusinessUnit: String,
        RequestID: i32,
        UserForceClose: i32,
        ErrorID: i32,
        ErrorMsg: String,
        IsSwapOrder: i32,
        ExchangeID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        ClientID: String,
        MacAddress: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ErrorConditionalOrderField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderRef: String,
        UserID: String,
        OrderPriceType: u8,
        Direction: u8,
        CombOffsetFlag: String,
        CombHedgeFlag: String,
        LimitPrice: f64,
        VolumeTotalOriginal: i32,
        TimeCondition: u8,
        GTDDate: String,
        VolumeCondition: u8,
        MinVolume: i32,
        ContingentCondition: u8,
        StopPrice: f64,
        ForceCloseReason: u8,
        IsAutoSuspend: i32,
        BusinessUnit: String,
        RequestID: i32,
        OrderLocalID: String,
        ExchangeID: String,
        ParticipantID: String,
        ClientID: String,
        TraderID: String,
        InstallID: i32,
        OrderSubmitStatus: u8,
        NotifySequence: i32,
        TradingDay: String,
        SettlementID: i32,
        OrderSysID: String,
        OrderSource: u8,
        OrderStatus: u8,
        OrderType: u8,
        VolumeTraded: i32,
        VolumeTotal: i32,
        InsertDate: String,
        InsertTime: String,
        ActiveTime: String,
        SuspendTime: String,
        UpdateTime: String,
        CancelTime: String,
        ActiveTraderID: String,
        ClearingPartID: String,
        SequenceNo: i32,
        FrontID: i32,
        SessionID: i32,
        UserProductInfo: String,
        StatusMsg: String,
        UserForceClose: i32,
        ActiveUserID: String,
        BrokerOrderSeq: i32,
        RelativeOrderSysID: String,
        ZCETotalTradedVolume: i32,
        ErrorID: i32,
        ErrorMsg: String,
        IsSwapOrder: i32,
        BranchID: String,
        InvestUnitID: String,
        AccountID: String,
        CurrencyID: String,
        MacAddress: String,
        InstrumentID: String,
        ExchangeInstID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryErrOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ErrOrderActionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        OrderActionRef: i32,
        OrderRef: String,
        RequestID: i32,
        FrontID: i32,
        SessionID: i32,
        ExchangeID: String,
        OrderSysID: String,
        ActionFlag: u8,
        LimitPrice: f64,
        VolumeChange: i32,
        ActionDate: String,
        ActionTime: String,
        TraderID: String,
        InstallID: i32,
        OrderLocalID: String,
        ActionLocalID: String,
        ParticipantID: String,
        ClientID: String,
        BusinessUnit: String,
        OrderActionStatus: u8,
        UserID: String,
        StatusMsg: String,
        BranchID: String,
        InvestUnitID: String,
        MacAddress: String,
        ErrorID: i32,
        ErrorMsg: String,
        InstrumentID: String,
        IPAddress: String,
        OrderMemo: String,
        SessionReqSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryExchangeSequenceField {
        is_null: bool,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ExchangeSequenceField {
        is_null: bool,
        ExchangeID: String,
        SequenceNo: i32,
        MarketStatus: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMaxOrderVolumeWithPriceField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        Direction: u8,
        OffsetFlag: u8,
        HedgeFlag: u8,
        MaxVolume: i32,
        Price: f64,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBrokerTradingParamsField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CurrencyID: String,
        AccountID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerTradingParamsField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        MarginPriceType: u8,
        Algorithm: u8,
        AvailIncludeCloseProfit: u8,
        CurrencyID: String,
        OptionRoyaltyPriceType: u8,
        AccountID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBrokerTradingAlgosField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerTradingAlgosField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        HandlePositionAlgoID: u8,
        FindMarginRateAlgoID: u8,
        HandleTradingAccountAlgoID: u8,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QueryBrokerDepositField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerDepositField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        ParticipantID: String,
        ExchangeID: String,
        PreBalance: f64,
        CurrMargin: f64,
        CloseProfit: f64,
        Balance: f64,
        Deposit: f64,
        Withdraw: f64,
        Available: f64,
        Reserve: f64,
        FrozenMargin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCFMMCBrokerKeyField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CFMMCBrokerKeyField {
        is_null: bool,
        BrokerID: String,
        ParticipantID: String,
        CreateDate: String,
        CreateTime: String,
        KeyID: i32,
        CurrentKey: Vec<u8>,
        KeyKind: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct CFMMCTradingAccountKeyField {
        is_null: bool,
        BrokerID: String,
        ParticipantID: String,
        AccountID: String,
        KeyID: i32,
        CurrentKey: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCFMMCTradingAccountKeyField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerUserOTPParamField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        OTPVendorsID: String,
        SerialNumber: Vec<u8>,
        AuthKey: Vec<u8>,
        LastDrift: i32,
        LastSuccess: i32,
        OTPType: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct ManualSyncBrokerUserOTPField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        OTPType: u8,
        FirstOTP: Vec<u8>,
        SecondOTP: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct CommRateModelField {
        is_null: bool,
        BrokerID: String,
        CommModelID: String,
        CommModelName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCommRateModelField {
        is_null: bool,
        BrokerID: String,
        CommModelID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct MarginModelField {
        is_null: bool,
        BrokerID: String,
        MarginModelID: String,
        MarginModelName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMarginModelField {
        is_null: bool,
        BrokerID: String,
        MarginModelID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct EWarrantOffsetField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        Direction: u8,
        HedgeFlag: u8,
        Volume: i32,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryEWarrantOffsetField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InvestUnitID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProductGroupMarginField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        ExchangeID: String,
        InvestUnitID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorProductGroupMarginField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        TradingDay: String,
        SettlementID: i32,
        FrozenMargin: f64,
        LongFrozenMargin: f64,
        ShortFrozenMargin: f64,
        UseMargin: f64,
        LongUseMargin: f64,
        ShortUseMargin: f64,
        ExchMargin: f64,
        LongExchMargin: f64,
        ShortExchMargin: f64,
        CloseProfit: f64,
        FrozenCommission: f64,
        Commission: f64,
        FrozenCash: f64,
        CashIn: f64,
        PositionProfit: f64,
        OffsetAmount: f64,
        LongOffsetAmount: f64,
        ShortOffsetAmount: f64,
        ExchOffsetAmount: f64,
        LongExchOffsetAmount: f64,
        ShortExchOffsetAmount: f64,
        HedgeFlag: u8,
        ExchangeID: String,
        InvestUnitID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QueryCFMMCTradingAccountTokenField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InvestUnitID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CFMMCTradingAccountTokenField {
        is_null: bool,
        BrokerID: String,
        ParticipantID: String,
        AccountID: String,
        KeyID: i32,
        Token: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct QryProductGroupField {
        is_null: bool,
        ExchangeID: String,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ProductGroupField {
        is_null: bool,
        ExchangeID: String,
        ProductID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct BulletinField {
        is_null: bool,
        ExchangeID: String,
        TradingDay: String,
        BulletinID: i32,
        SequenceNo: i32,
        NewsType: Vec<u8>,
        NewsUrgency: u8,
        SendTime: String,
        Abstract: Vec<u8>,
        ComeFrom: Vec<u8>,
        Content: Vec<u8>,
        URLLink: Vec<u8>,
        MarketID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryBulletinField {
        is_null: bool,
        ExchangeID: String,
        BulletinID: i32,
        SequenceNo: i32,
        NewsType: Vec<u8>,
        NewsUrgency: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct MulticastInstrumentField {
        is_null: bool,
        TopicID: i32,
        InstrumentNo: i32,
        CodePrice: f64,
        VolumeMultiple: i32,
        PriceTick: f64,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryMulticastInstrumentField {
        is_null: bool,
        TopicID: i32,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct AppIDAuthAssignField {
        is_null: bool,
        BrokerID: String,
        AppID: String,
        DRIdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqOpenAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        CashExchangeCode: u8,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        TID: i32,
        UserID: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqCancelAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        CashExchangeCode: u8,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        TID: i32,
        UserID: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqChangeAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        NewBankAccount: String,
        NewBankPassWord: String,
        AccountID: String,
        Password: String,
        BankAccType: u8,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        BrokerIDByBank: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        TID: i32,
        Digest: Vec<u8>,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqTransferField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        FutureSerial: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        TradeAmount: f64,
        FutureFetchAmount: f64,
        FeePayFlag: u8,
        CustFee: f64,
        BrokerFee: f64,
        Message: Vec<u8>,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        TransferStatus: u8,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspTransferField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        FutureSerial: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        TradeAmount: f64,
        FutureFetchAmount: f64,
        FeePayFlag: u8,
        CustFee: f64,
        BrokerFee: f64,
        Message: Vec<u8>,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        TransferStatus: u8,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqRepealField {
        is_null: bool,
        RepealTimeInterval: i32,
        RepealedTimes: i32,
        BankRepealFlag: u8,
        BrokerRepealFlag: u8,
        PlateRepealSerial: i32,
        BankRepealSerial: String,
        FutureRepealSerial: i32,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        FutureSerial: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        TradeAmount: f64,
        FutureFetchAmount: f64,
        FeePayFlag: u8,
        CustFee: f64,
        BrokerFee: f64,
        Message: Vec<u8>,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        TransferStatus: u8,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspRepealField {
        is_null: bool,
        RepealTimeInterval: i32,
        RepealedTimes: i32,
        BankRepealFlag: u8,
        BrokerRepealFlag: u8,
        PlateRepealSerial: i32,
        BankRepealSerial: String,
        FutureRepealSerial: i32,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        FutureSerial: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        TradeAmount: f64,
        FutureFetchAmount: f64,
        FeePayFlag: u8,
        CustFee: f64,
        BrokerFee: f64,
        Message: Vec<u8>,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        TransferStatus: u8,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqQueryAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        FutureSerial: i32,
        InstallID: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspQueryAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        FutureSerial: i32,
        InstallID: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        BankUseAmount: f64,
        BankFetchAmount: f64,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct FutureSignIOField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Digest: Vec<u8>,
        CurrencyID: String,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RspFutureSignInField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Digest: Vec<u8>,
        CurrencyID: String,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        ErrorID: i32,
        ErrorMsg: String,
        PinKey: Vec<u8>,
        MacKey: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqFutureSignOutField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Digest: Vec<u8>,
        CurrencyID: String,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RspFutureSignOutField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Digest: Vec<u8>,
        CurrencyID: String,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqQueryTradeResultBySerialField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        Reference: i32,
        RefrenceIssureType: u8,
        RefrenceIssure: Vec<u8>,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        CurrencyID: String,
        TradeAmount: f64,
        Digest: Vec<u8>,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspQueryTradeResultBySerialField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        ErrorID: i32,
        ErrorMsg: String,
        Reference: i32,
        RefrenceIssureType: u8,
        RefrenceIssure: Vec<u8>,
        OriginReturnCode: String,
        OriginDescrInfoForReturnCode: String,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        CurrencyID: String,
        TradeAmount: f64,
        Digest: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqDayEndFileReadyField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        FileBusinessCode: u8,
        Digest: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct ReturnResultField {
        is_null: bool,
        ReturnCode: String,
        DescrInfoForReturnCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct VerifyFuturePasswordField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        AccountID: String,
        Password: String,
        BankAccount: String,
        BankPassWord: String,
        InstallID: i32,
        TID: i32,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct VerifyCustInfoField {
        is_null: bool,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct VerifyFuturePasswordAndCustInfoField {
        is_null: bool,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        AccountID: String,
        Password: String,
        CurrencyID: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct DepositResultInformField {
        is_null: bool,
        DepositSeqNo: Vec<u8>,
        BrokerID: String,
        InvestorID: String,
        Deposit: f64,
        RequestID: i32,
        ReturnCode: String,
        DescrInfoForReturnCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqSyncKeyField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Message: Vec<u8>,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RspSyncKeyField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Message: Vec<u8>,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct NotifyQueryAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        FutureSerial: i32,
        InstallID: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        BankUseAmount: f64,
        BankFetchAmount: f64,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TransferSerialField {
        is_null: bool,
        PlateSerial: i32,
        TradeDate: String,
        TradingDay: String,
        TradeTime: String,
        TradeCode: String,
        SessionID: i32,
        BankID: String,
        BankBranchID: String,
        BankAccType: u8,
        BankAccount: String,
        BankSerial: String,
        BrokerID: String,
        BrokerBranchID: String,
        FutureAccType: u8,
        AccountID: String,
        InvestorID: String,
        FutureSerial: i32,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CurrencyID: String,
        TradeAmount: f64,
        CustFee: f64,
        BrokerFee: f64,
        AvailabilityFlag: u8,
        OperatorCode: String,
        BankNewAccount: String,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTransferSerialField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        BankID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct NotifyFutureSignInField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Digest: Vec<u8>,
        CurrencyID: String,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        ErrorID: i32,
        ErrorMsg: String,
        PinKey: Vec<u8>,
        MacKey: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct NotifyFutureSignOutField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Digest: Vec<u8>,
        CurrencyID: String,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct NotifySyncKeyField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        InstallID: i32,
        UserID: String,
        Message: Vec<u8>,
        DeviceID: String,
        BrokerIDByBank: Vec<u8>,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryAccountregisterField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        BankID: String,
        BankBranchID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct AccountregisterField {
        is_null: bool,
        TradeDay: String,
        BankID: String,
        BankBranchID: String,
        BankAccount: String,
        BrokerID: String,
        BrokerBranchID: String,
        AccountID: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustomerName: String,
        CurrencyID: String,
        OpenOrDestroy: u8,
        RegDate: String,
        OutDate: String,
        TID: i32,
        CustType: u8,
        BankAccType: u8,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct OpenAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        CashExchangeCode: u8,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        TID: i32,
        UserID: String,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CancelAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        CashExchangeCode: u8,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        TID: i32,
        UserID: String,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ChangeAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        NewBankAccount: String,
        NewBankPassWord: String,
        AccountID: String,
        Password: String,
        BankAccType: u8,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        BrokerIDByBank: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        TID: i32,
        Digest: Vec<u8>,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SecAgentACIDMapField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        AccountID: String,
        CurrencyID: String,
        BrokerSecAgentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySecAgentACIDMapField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        AccountID: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct UserRightsAssignField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        DRIdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct BrokerUserRightAssignField {
        is_null: bool,
        BrokerID: String,
        DRIdentityID: i32,
        Tradeable: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct DRTransferField {
        is_null: bool,
        OrigDRIdentityID: i32,
        DestDRIdentityID: i32,
        OrigBrokerID: String,
        DestBrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct FensUserInfoField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        LoginMode: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct CurrTransferIdentityField {
        is_null: bool,
        IdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct LoginForbiddenUserField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryLoginForbiddenUserField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingAccountReserveField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        Reserve: f64,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryLoginForbiddenIPField {
        is_null: bool,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryIPListField {
        is_null: bool,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryUserRightsAssignField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReserveOpenAccountConfirmField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        BrokerIDByBank: Vec<u8>,
        TID: i32,
        AccountID: String,
        Password: String,
        BankReserveOpenSeq: Vec<u8>,
        BookDate: String,
        BookPsw: Vec<u8>,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReserveOpenAccountField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        Gender: u8,
        CountryCode: String,
        CustType: u8,
        Address: String,
        ZipCode: String,
        Telephone: String,
        MobilePhone: String,
        Fax: String,
        EMail: String,
        MoneyAccountStatus: u8,
        BankAccount: String,
        BankPassWord: String,
        InstallID: i32,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        BrokerIDByBank: Vec<u8>,
        TID: i32,
        ReserveOpenAccStas: u8,
        ErrorID: i32,
        ErrorMsg: String,
    }
    #[derive(Debug, Clone, Default)]
    struct AccountPropertyField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        BankID: String,
        BankAccount: String,
        OpenName: String,
        OpenBank: Vec<u8>,
        IsActive: i32,
        AccountSourceType: u8,
        OpenDate: String,
        CancelDate: String,
        OperatorID: String,
        OperateDate: String,
        OperateTime: String,
        CurrencyID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCurrDRIdentityField {
        is_null: bool,
        DRIdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct CurrDRIdentityField {
        is_null: bool,
        DRIdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySecAgentCheckModeField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySecAgentTradeInfoField {
        is_null: bool,
        BrokerID: String,
        BrokerSecAgentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqUserAuthMethodField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspUserAuthMethodField {
        is_null: bool,
        UsableAuthMethod: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqGenUserCaptchaField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspGenUserCaptchaField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        CaptchaInfoLen: i32,
        CaptchaInfo: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqGenUserTextField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspGenUserTextField {
        is_null: bool,
        UserTextSeq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginWithCaptchaField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
        Password: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        MacAddress: String,
        LoginRemark: String,
        Captcha: Vec<u8>,
        ClientIPPort: i32,
        ClientIPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginWithTextField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
        Password: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        MacAddress: String,
        LoginRemark: String,
        Text: Vec<u8>,
        ClientIPPort: i32,
        ClientIPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginWithOTPField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
        Password: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        MacAddress: String,
        LoginRemark: String,
        OTPPassword: String,
        ClientIPPort: i32,
        ClientIPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqApiHandshakeField {
        is_null: bool,
        CryptoKeyVersion: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RspApiHandshakeField {
        is_null: bool,
        FrontHandshakeDataLen: i32,
        FrontHandshakeData: Vec<u8>,
        IsApiAuthEnabled: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqVerifyApiKeyField {
        is_null: bool,
        ApiHandshakeDataLen: i32,
        ApiHandshakeData: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct DepartmentUserField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        InvestorRange: u8,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QueryFreqField {
        is_null: bool,
        QueryFreq: i32,
        FTDPkgFreq: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct AuthForbiddenIPField {
        is_null: bool,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryAuthForbiddenIPField {
        is_null: bool,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDelaySwapFrozenField {
        is_null: bool,
        DelaySwapSeqNo: Vec<u8>,
        BrokerID: String,
        InvestorID: String,
        FromCurrencyID: String,
        FromRemainSwap: f64,
        IsManualSwap: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct UserSystemInfoField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        ClientSystemInfoLen: i32,
        ClientSystemInfo: String,
        ClientIPPort: i32,
        ClientLoginTime: String,
        ClientAppID: String,
        ClientPublicIP: String,
        ClientLoginRemark: String,
    }
    #[derive(Debug, Clone, Default)]
    struct AuthUserIDField {
        is_null: bool,
        BrokerID: String,
        AppID: String,
        UserID: String,
        AuthType: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct AuthIPField {
        is_null: bool,
        BrokerID: String,
        AppID: String,
        IPAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryClassifiedInstrumentField {
        is_null: bool,
        InstrumentID: String,
        ExchangeID: String,
        ExchangeInstID: String,
        ProductID: String,
        TradingType: u8,
        ClassType: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct QryCombPromotionParamField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct CombPromotionParamField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        CombHedgeFlag: String,
        Xparameter: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginSMField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        UserID: String,
        Password: String,
        UserProductInfo: String,
        InterfaceProductInfo: String,
        ProtocolInfo: String,
        MacAddress: String,
        OneTimePassword: String,
        LoginRemark: String,
        ClientIPPort: i32,
        ClientIPAddress: String,
        BrokerName: String,
        AuthCode: String,
        AppID: String,
        PIN: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRiskSettleInvstPositionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRiskSettleProductStatusField {
        is_null: bool,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RiskSettleInvstPositionField {
        is_null: bool,
        InstrumentID: String,
        BrokerID: String,
        InvestorID: String,
        PosiDirection: u8,
        HedgeFlag: u8,
        PositionDate: u8,
        YdPosition: i32,
        Position: i32,
        LongFrozen: i32,
        ShortFrozen: i32,
        LongFrozenAmount: f64,
        ShortFrozenAmount: f64,
        OpenVolume: i32,
        CloseVolume: i32,
        OpenAmount: f64,
        CloseAmount: f64,
        PositionCost: f64,
        PreMargin: f64,
        UseMargin: f64,
        FrozenMargin: f64,
        FrozenCash: f64,
        FrozenCommission: f64,
        CashIn: f64,
        Commission: f64,
        CloseProfit: f64,
        PositionProfit: f64,
        PreSettlementPrice: f64,
        SettlementPrice: f64,
        TradingDay: String,
        SettlementID: i32,
        OpenCost: f64,
        ExchangeMargin: f64,
        CombPosition: i32,
        CombLongFrozen: i32,
        CombShortFrozen: i32,
        CloseProfitByDate: f64,
        CloseProfitByTrade: f64,
        TodayPosition: i32,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        StrikeFrozen: i32,
        StrikeFrozenAmount: f64,
        AbandonFrozen: i32,
        ExchangeID: String,
        YdStrikeFrozen: i32,
        InvestUnitID: String,
        PositionCostOffset: f64,
        TasPosition: i32,
        TasPositionCost: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct RiskSettleProductStatusField {
        is_null: bool,
        ExchangeID: String,
        ProductID: String,
        ProductStatus: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInfoField {
        is_null: bool,
        SyncDeltaSequenceNo: i32,
        SyncDeltaStatus: u8,
        SyncDescription: Vec<u8>,
        IsOnlyTrdDelta: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaProductStatusField {
        is_null: bool,
        SyncDeltaSequenceNo: i32,
        ExchangeID: String,
        ProductID: String,
        ProductStatus: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstPosDtlField {
        is_null: bool,
        InstrumentID: String,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        Direction: u8,
        OpenDate: String,
        TradeID: String,
        Volume: i32,
        OpenPrice: f64,
        TradingDay: String,
        SettlementID: i32,
        TradeType: u8,
        CombInstrumentID: String,
        ExchangeID: String,
        CloseProfitByDate: f64,
        CloseProfitByTrade: f64,
        PositionProfitByDate: f64,
        PositionProfitByTrade: f64,
        Margin: f64,
        ExchMargin: f64,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        LastSettlementPrice: f64,
        SettlementPrice: f64,
        CloseVolume: i32,
        CloseAmount: f64,
        TimeFirstVolume: i32,
        SpecPosiType: u8,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstPosCombDtlField {
        is_null: bool,
        TradingDay: String,
        OpenDate: String,
        ExchangeID: String,
        SettlementID: i32,
        BrokerID: String,
        InvestorID: String,
        ComTradeID: String,
        TradeID: String,
        InstrumentID: String,
        HedgeFlag: u8,
        Direction: u8,
        TotalAmt: i32,
        Margin: f64,
        ExchMargin: f64,
        MarginRateByMoney: f64,
        MarginRateByVolume: f64,
        LegID: i32,
        LegMultiple: i32,
        TradeGroupID: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaTradingAccountField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        PreMortgage: f64,
        PreCredit: f64,
        PreDeposit: f64,
        PreBalance: f64,
        PreMargin: f64,
        InterestBase: f64,
        Interest: f64,
        Deposit: f64,
        Withdraw: f64,
        FrozenMargin: f64,
        FrozenCash: f64,
        FrozenCommission: f64,
        CurrMargin: f64,
        CashIn: f64,
        Commission: f64,
        CloseProfit: f64,
        PositionProfit: f64,
        Balance: f64,
        Available: f64,
        WithdrawQuota: f64,
        Reserve: f64,
        TradingDay: String,
        SettlementID: i32,
        Credit: f64,
        Mortgage: f64,
        ExchangeMargin: f64,
        DeliveryMargin: f64,
        ExchangeDeliveryMargin: f64,
        ReserveBalance: f64,
        CurrencyID: String,
        PreFundMortgageIn: f64,
        PreFundMortgageOut: f64,
        FundMortgageIn: f64,
        FundMortgageOut: f64,
        FundMortgageAvailable: f64,
        MortgageableFund: f64,
        SpecProductMargin: f64,
        SpecProductFrozenMargin: f64,
        SpecProductCommission: f64,
        SpecProductFrozenCommission: f64,
        SpecProductPositionProfit: f64,
        SpecProductCloseProfit: f64,
        SpecProductPositionProfitByAlg: f64,
        SpecProductExchangeMargin: f64,
        FrozenSwap: f64,
        RemainSwap: f64,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInitInvstMarginField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        LastRiskTotalInvstMargin: f64,
        LastRiskTotalExchMargin: f64,
        ThisSyncInvstMargin: f64,
        ThisSyncExchMargin: f64,
        RemainRiskInvstMargin: f64,
        RemainRiskExchMargin: f64,
        LastRiskSpecTotalInvstMargin: f64,
        LastRiskSpecTotalExchMargin: f64,
        ThisSyncSpecInvstMargin: f64,
        ThisSyncSpecExchMargin: f64,
        RemainRiskSpecInvstMargin: f64,
        RemainRiskSpecExchMargin: f64,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaDceCombInstrumentField {
        is_null: bool,
        CombInstrumentID: String,
        ExchangeID: String,
        ExchangeInstID: String,
        TradeGroupID: i32,
        CombHedgeFlag: u8,
        CombinationType: u8,
        Direction: u8,
        ProductID: String,
        Xparameter: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstMarginRateField {
        is_null: bool,
        InstrumentID: String,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        IsRelative: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaExchMarginRateField {
        is_null: bool,
        BrokerID: String,
        InstrumentID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaOptExchMarginField {
        is_null: bool,
        BrokerID: String,
        InstrumentID: String,
        SShortMarginRatioByMoney: f64,
        SShortMarginRatioByVolume: f64,
        HShortMarginRatioByMoney: f64,
        HShortMarginRatioByVolume: f64,
        AShortMarginRatioByMoney: f64,
        AShortMarginRatioByVolume: f64,
        MShortMarginRatioByMoney: f64,
        MShortMarginRatioByVolume: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaOptInvstMarginField {
        is_null: bool,
        InstrumentID: String,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        SShortMarginRatioByMoney: f64,
        SShortMarginRatioByVolume: f64,
        HShortMarginRatioByMoney: f64,
        HShortMarginRatioByVolume: f64,
        AShortMarginRatioByMoney: f64,
        AShortMarginRatioByVolume: f64,
        IsRelative: i32,
        MShortMarginRatioByMoney: f64,
        MShortMarginRatioByVolume: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstMarginRateULField {
        is_null: bool,
        InstrumentID: String,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        LongMarginRatioByMoney: f64,
        LongMarginRatioByVolume: f64,
        ShortMarginRatioByMoney: f64,
        ShortMarginRatioByVolume: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaOptInvstCommRateField {
        is_null: bool,
        InstrumentID: String,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        StrikeRatioByMoney: f64,
        StrikeRatioByVolume: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstCommRateField {
        is_null: bool,
        InstrumentID: String,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        OpenRatioByMoney: f64,
        OpenRatioByVolume: f64,
        CloseRatioByMoney: f64,
        CloseRatioByVolume: f64,
        CloseTodayRatioByMoney: f64,
        CloseTodayRatioByVolume: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaProductExchRateField {
        is_null: bool,
        ProductID: String,
        QuoteCurrencyID: String,
        ExchangeRate: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaDepthMarketDataField {
        is_null: bool,
        TradingDay: String,
        InstrumentID: String,
        ExchangeID: String,
        ExchangeInstID: String,
        LastPrice: f64,
        PreSettlementPrice: f64,
        PreClosePrice: f64,
        PreOpenInterest: f64,
        OpenPrice: f64,
        HighestPrice: f64,
        LowestPrice: f64,
        Volume: i32,
        Turnover: f64,
        OpenInterest: f64,
        ClosePrice: f64,
        SettlementPrice: f64,
        UpperLimitPrice: f64,
        LowerLimitPrice: f64,
        PreDelta: f64,
        CurrDelta: f64,
        UpdateTime: String,
        UpdateMillisec: i32,
        BidPrice1: f64,
        BidVolume1: i32,
        AskPrice1: f64,
        AskVolume1: i32,
        BidPrice2: f64,
        BidVolume2: i32,
        AskPrice2: f64,
        AskVolume2: i32,
        BidPrice3: f64,
        BidVolume3: i32,
        AskPrice3: f64,
        AskVolume3: i32,
        BidPrice4: f64,
        BidVolume4: i32,
        AskPrice4: f64,
        AskVolume4: i32,
        BidPrice5: f64,
        BidVolume5: i32,
        AskPrice5: f64,
        AskVolume5: i32,
        AveragePrice: f64,
        ActionDay: String,
        BandingUpperPrice: f64,
        BandingLowerPrice: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaIndexPriceField {
        is_null: bool,
        BrokerID: String,
        InstrumentID: String,
        ClosePrice: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaEWarrantOffsetField {
        is_null: bool,
        TradingDay: String,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        InstrumentID: String,
        Direction: u8,
        HedgeFlag: u8,
        Volume: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMFutureParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        InstrumentID: String,
        ProdFamilyCode: String,
        Cvf: i32,
        TimeRange: u8,
        MarginRate: f64,
        LockRateX: f64,
        AddOnRate: f64,
        PreSettlementPrice: f64,
        AddOnLockRateX2: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMOptionParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        InstrumentID: String,
        ProdFamilyCode: String,
        Cvf: i32,
        DownPrice: f64,
        Delta: f64,
        SlimiDelta: f64,
        PreSettlementPrice: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMIntraParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProdFamilyCode: String,
        IntraRateY: f64,
        AddOnIntraRateY2: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        SpreadId: i32,
        InterRateZ: f64,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncSPBMParameterEndField {
        is_null: bool,
        TradingDay: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMFutureParameterField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMOptionParameterField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMIntraParameterField {
        is_null: bool,
        ExchangeID: String,
        ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMInterParameterField {
        is_null: bool,
        ExchangeID: String,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMPortfDefinitionField {
        is_null: bool,
        ExchangeID: String,
        PortfolioDefID: i32,
        ProdFamilyCode: String,
        IsSPBM: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMInvestorPortfDefField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        PortfolioDefID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorPortfMarginRatioField {
        is_null: bool,
        InvestorRange: u8,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        MarginRatio: f64,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMPortfDefinitionField {
        is_null: bool,
        ExchangeID: String,
        PortfolioDefID: i32,
        ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMInvestorPortfDefField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPortfMarginRatioField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ExchangeID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorProdSPBMDetailField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        ProdFamilyCode: String,
        IntraInstrMargin: f64,
        BCollectingMargin: f64,
        SCollectingMargin: f64,
        IntraProdMargin: f64,
        NetMargin: f64,
        InterProdMargin: f64,
        SingleMargin: f64,
        AddOnMargin: f64,
        DeliveryMargin: f64,
        CallOptionMinRisk: f64,
        PutOptionMinRisk: f64,
        OptionMinRisk: f64,
        OptionValueOffset: f64,
        OptionRoyalty: f64,
        RealOptionValueOffset: f64,
        Margin: f64,
        ExchMargin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProdSPBMDetailField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct PortfTradeParamSettingField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        Portfolio: u8,
        IsActionVerify: i32,
        IsCloseVerify: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorTradingRightField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InvstTradingRight: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct MortgageParamField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        MortgageBalance: f64,
        CheckMortgageRatio: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct WithDrawParamField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        WithDrawParamID: u8,
        WithDrawParamValue: Vec<u8>,
    }
    #[derive(Debug, Clone, Default)]
    struct ThostUserFunctionField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        ThostFunctionCode: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryThostUserFunctionField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SPBMAddOnInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        SpreadId: i32,
        AddOnInterRateZ2: f64,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPBMAddOnInterParameterField {
        is_null: bool,
        ExchangeID: String,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorCommoditySPMMMarginField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CommodityID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorCommodityGroupSPMMMarginField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CommodityGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPMMInstParamField {
        is_null: bool,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QrySPMMProductParamField {
        is_null: bool,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorCommoditySPMMMarginField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        CommodityID: String,
        MarginBeforeDiscount: f64,
        MarginNoDiscount: f64,
        LongPosRisk: f64,
        LongOpenFrozenRisk: f64,
        LongCloseFrozenRisk: f64,
        ShortPosRisk: f64,
        ShortOpenFrozenRisk: f64,
        ShortCloseFrozenRisk: f64,
        IntraCommodityRate: f64,
        OptionDiscountRate: f64,
        PosDiscount: f64,
        OpenFrozenDiscount: f64,
        NetRisk: f64,
        CloseFrozenMargin: f64,
        FrozenCommission: f64,
        Commission: f64,
        FrozenCash: f64,
        CashIn: f64,
        StrikeFrozenMargin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorCommodityGroupSPMMMarginField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        CommodityGroupID: String,
        MarginBeforeDiscount: f64,
        MarginNoDiscount: f64,
        LongRisk: f64,
        ShortRisk: f64,
        CloseFrozenMargin: f64,
        InterCommodityRate: f64,
        MiniMarginRatio: f64,
        AdjustRatio: f64,
        IntraCommodityDiscount: f64,
        InterCommodityDiscount: f64,
        ExchMargin: f64,
        InvestorMargin: f64,
        FrozenCommission: f64,
        Commission: f64,
        FrozenCash: f64,
        CashIn: f64,
        StrikeFrozenMargin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct SPMMInstParamField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        InstMarginCalID: u8,
        CommodityID: String,
        CommodityGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct SPMMProductParamField {
        is_null: bool,
        ExchangeID: String,
        ProductID: String,
        CommodityID: String,
        CommodityGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTraderAssignField {
        is_null: bool,
        TraderID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TraderAssignField {
        is_null: bool,
        BrokerID: String,
        ExchangeID: String,
        TraderID: String,
        ParticipantID: String,
        DRIdentityID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorInfoCntSettingField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        ProductID: String,
        IsCalInfoComm: i32,
        IsLimitInfoMax: i32,
        InfoMaxLimit: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RCAMSCombProductInfoField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProductID: String,
        CombProductID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RCAMSInstrParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProductID: String,
        HedgeRate: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct RCAMSIntraParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        CombProductID: String,
        HedgeRate: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct RCAMSInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProductGroupID: String,
        Priority: i32,
        CreditRate: f64,
        CombProduct1: String,
        CombProduct2: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RCAMSShortOptAdjustParamField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        CombProductID: String,
        HedgeFlag: u8,
        AdjustValue: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct RCAMSInvestorCombPositionField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
        HedgeFlag: u8,
        PosiDirection: u8,
        CombInstrumentID: String,
        LegID: i32,
        ExchangeInstID: String,
        TotalAmt: i32,
        ExchMargin: f64,
        Margin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorProdRCAMSMarginField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        CombProductID: String,
        HedgeFlag: u8,
        ProductGroupID: String,
        RiskBeforeDiscount: f64,
        IntraInstrRisk: f64,
        BPosRisk: f64,
        SPosRisk: f64,
        IntraProdRisk: f64,
        NetRisk: f64,
        InterProdRisk: f64,
        ShortOptRiskAdj: f64,
        OptionRoyalty: f64,
        MMSACloseFrozenMargin: f64,
        CloseCombFrozenMargin: f64,
        CloseFrozenMargin: f64,
        MMSAOpenFrozenMargin: f64,
        DeliveryOpenFrozenMargin: f64,
        OpenFrozenMargin: f64,
        UseFrozenMargin: f64,
        MMSAExchMargin: f64,
        DeliveryExchMargin: f64,
        CombExchMargin: f64,
        ExchMargin: f64,
        UseMargin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSCombProductInfoField {
        is_null: bool,
        ProductID: String,
        CombProductID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSInstrParameterField {
        is_null: bool,
        ProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSIntraParameterField {
        is_null: bool,
        CombProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSInterParameterField {
        is_null: bool,
        ProductGroupID: String,
        CombProduct1: String,
        CombProduct2: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSShortOptAdjustParamField {
        is_null: bool,
        CombProductID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSInvestorCombPositionField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
        CombInstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProdRCAMSMarginField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        CombProductID: String,
        ProductGroupID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct RULEInstrParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        InstrumentID: String,
        InstrumentClass: u8,
        StdInstrumentID: String,
        BSpecRatio: f64,
        SSpecRatio: f64,
        BHedgeRatio: f64,
        SHedgeRatio: f64,
        BAddOnMargin: f64,
        SAddOnMargin: f64,
        CommodityGroupID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RULEIntraParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProdFamilyCode: String,
        StdInstrumentID: String,
        StdInstrMargin: f64,
        UsualIntraRate: f64,
        DeliveryIntraRate: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct RULEInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        SpreadId: i32,
        InterRate: f64,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
        Leg1PropFactor: i32,
        Leg2PropFactor: i32,
        CommodityGroupID: i32,
        CommodityGroupName: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRULEInstrParameterField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRULEIntraParameterField {
        is_null: bool,
        ExchangeID: String,
        ProdFamilyCode: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryRULEInterParameterField {
        is_null: bool,
        ExchangeID: String,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
        CommodityGroupID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorProdRULEMarginField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        ProdFamilyCode: String,
        InstrumentClass: u8,
        CommodityGroupID: i32,
        BStdPosition: f64,
        SStdPosition: f64,
        BStdOpenFrozen: f64,
        SStdOpenFrozen: f64,
        BStdCloseFrozen: f64,
        SStdCloseFrozen: f64,
        IntraProdStdPosition: f64,
        NetStdPosition: f64,
        InterProdStdPosition: f64,
        SingleStdPosition: f64,
        IntraProdMargin: f64,
        InterProdMargin: f64,
        SingleMargin: f64,
        NonCombMargin: f64,
        AddOnMargin: f64,
        ExchMargin: f64,
        AddOnFrozenMargin: f64,
        OpenFrozenMargin: f64,
        CloseFrozenMargin: f64,
        Margin: f64,
        FrozenMargin: f64,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProdRULEMarginField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        ProdFamilyCode: String,
        CommodityGroupID: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMPortfDefinitionField {
        is_null: bool,
        ExchangeID: String,
        PortfolioDefID: i32,
        ProdFamilyCode: String,
        IsSPBM: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMInvstPortfDefField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        PortfolioDefID: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMFutureParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        InstrumentID: String,
        ProdFamilyCode: String,
        Cvf: i32,
        TimeRange: u8,
        MarginRate: f64,
        LockRateX: f64,
        AddOnRate: f64,
        PreSettlementPrice: f64,
        AddOnLockRateX2: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMOptionParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        InstrumentID: String,
        ProdFamilyCode: String,
        Cvf: i32,
        DownPrice: f64,
        Delta: f64,
        SlimiDelta: f64,
        PreSettlementPrice: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMIntraParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProdFamilyCode: String,
        IntraRateY: f64,
        AddOnIntraRateY2: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        SpreadId: i32,
        InterRateZ: f64,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMAddOnInterParamField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        SpreadId: i32,
        AddOnInterRateZ2: f64,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPMMInstParamField {
        is_null: bool,
        ExchangeID: String,
        InstrumentID: String,
        InstMarginCalID: u8,
        CommodityID: String,
        CommodityGroupID: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPMMProductParamField {
        is_null: bool,
        ExchangeID: String,
        ProductID: String,
        CommodityID: String,
        CommodityGroupID: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvestorSPMMModelField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        SPMMModelID: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPMMModelParamField {
        is_null: bool,
        ExchangeID: String,
        SPMMModelID: String,
        CommodityGroupID: String,
        IntraCommodityRate: f64,
        InterCommodityRate: f64,
        OptionDiscountRate: f64,
        MiniMarginRatio: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSCombProdInfoField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProductID: String,
        CombProductID: String,
        ProductGroupID: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSInstrParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProductID: String,
        HedgeRate: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSIntraParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        CombProductID: String,
        HedgeRate: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProductGroupID: String,
        Priority: i32,
        CreditRate: f64,
        CombProduct1: String,
        CombProduct2: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSSOptAdjParamField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        CombProductID: String,
        HedgeFlag: u8,
        AdjustValue: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSCombRuleDtlField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProdGroup: Vec<u8>,
        RuleId: Vec<u8>,
        Priority: i32,
        HedgeFlag: u8,
        CombMargin: f64,
        ExchangeInstID: String,
        LegID: i32,
        LegInstrumentID: String,
        Direction: u8,
        LegMultiple: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSInvstCombPosField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
        HedgeFlag: u8,
        PosiDirection: u8,
        CombInstrumentID: String,
        LegID: i32,
        ExchangeInstID: String,
        TotalAmt: i32,
        ExchMargin: f64,
        Margin: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRULEInstrParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        InstrumentID: String,
        InstrumentClass: u8,
        StdInstrumentID: String,
        BSpecRatio: f64,
        SSpecRatio: f64,
        BHedgeRatio: f64,
        SHedgeRatio: f64,
        BAddOnMargin: f64,
        SAddOnMargin: f64,
        CommodityGroupID: i32,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRULEIntraParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        ProdFamilyCode: String,
        StdInstrumentID: String,
        StdInstrMargin: f64,
        UsualIntraRate: f64,
        DeliveryIntraRate: f64,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRULEInterParameterField {
        is_null: bool,
        TradingDay: String,
        ExchangeID: String,
        SpreadId: i32,
        InterRate: f64,
        Leg1ProdFamilyCode: String,
        Leg2ProdFamilyCode: String,
        Leg1PropFactor: i32,
        Leg2PropFactor: i32,
        CommodityGroupID: i32,
        CommodityGroupName: String,
        ActionDirection: u8,
        SyncDeltaSequenceNo: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct IpAddrParamField {
        is_null: bool,
        BrokerID: String,
        Address: String,
        DRIdentityID: i32,
        DRIdentityName: String,
        AddrSrvMode: u8,
        AddrVer: u8,
        AddrNo: i32,
        AddrName: String,
        IsSM: i32,
        IsLocalAddr: i32,
        Remark: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryIpAddrParamField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TGIpAddrParamField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        Address: String,
        DRIdentityID: i32,
        DRIdentityName: String,
        AddrSrvMode: u8,
        AddrVer: u8,
        AddrNo: i32,
        AddrName: String,
        IsSM: i32,
        IsLocalAddr: i32,
        Remark: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryTGIpAddrParamField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct TGSessionQryStatusField {
        is_null: bool,
        LastQryFreq: i32,
        QryStatus: u8,
    }
    #[derive(Debug, Clone, Default)]
    struct LocalAddrConfigField {
        is_null: bool,
        BrokerID: String,
        PeerAddr: String,
        NetMask: Vec<u8>,
        DRIdentityID: i32,
        LocalAddress: String,
    }
    #[derive(Debug, Clone, Default)]
    struct QryLocalAddrConfigField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqQueryBankAccountBySecField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        FutureSerial: i32,
        InstallID: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        LongCustomerName: String,
        DRIdentityID: i32,
        SecFutureSerial: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RspQueryBankAccountBySecField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        FutureSerial: i32,
        InstallID: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        BankUseAmount: f64,
        BankFetchAmount: f64,
        LongCustomerName: String,
        DRIdentityID: i32,
        SecFutureSerial: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ReqTransferBySecField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        FutureSerial: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        TradeAmount: f64,
        FutureFetchAmount: f64,
        FeePayFlag: u8,
        CustFee: f64,
        BrokerFee: f64,
        Message: Vec<u8>,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        TransferStatus: u8,
        LongCustomerName: String,
        DRIdentityID: i32,
        SecFutureSerial: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RspTransferBySecField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        InstallID: i32,
        FutureSerial: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        TradeAmount: f64,
        FutureFetchAmount: f64,
        FeePayFlag: u8,
        CustFee: f64,
        BrokerFee: f64,
        Message: Vec<u8>,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        TransferStatus: u8,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
        DRIdentityID: i32,
        SecFutureSerial: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct NotifyQueryFutureAccountBySecField {
        is_null: bool,
        TradeCode: String,
        BankID: String,
        BankBranchID: String,
        BrokerID: String,
        BrokerBranchID: String,
        TradeDate: String,
        TradeTime: String,
        BankSerial: String,
        TradingDay: String,
        PlateSerial: i32,
        LastFragment: u8,
        SessionID: i32,
        CustomerName: String,
        IdCardType: u8,
        IdentifiedCardNo: String,
        CustType: u8,
        BankAccount: String,
        BankPassWord: String,
        AccountID: String,
        Password: String,
        FutureSerial: i32,
        InstallID: i32,
        UserID: String,
        VerifyCertNoFlag: u8,
        CurrencyID: String,
        Digest: Vec<u8>,
        BankAccType: u8,
        DeviceID: String,
        BankSecuAccType: u8,
        BrokerIDByBank: Vec<u8>,
        BankSecuAcc: Vec<u8>,
        BankPwdFlag: u8,
        SecuPwdFlag: u8,
        OperNo: String,
        RequestID: i32,
        TID: i32,
        BankUseAmount: f64,
        BankFetchAmount: f64,
        ErrorID: i32,
        ErrorMsg: String,
        LongCustomerName: String,
        DRIdentityID: i32,
        SecFutureSerial: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct ExitEmergencyField {
        is_null: bool,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorPortfMarginModelField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        MarginModelID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorPortfSettingField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        HedgeFlag: u8,
        UsePortf: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPortfSettingField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct UserPasswordUpdateFromSecField {
        is_null: bool,
        BrokerID: String,
        UserID: String,
        OldPassword: String,
        NewPassword: String,
        FromSec: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct SettlementInfoConfirmFromSecField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        ConfirmDate: String,
        ConfirmTime: String,
        FromSec: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordUpdateFromSecField {
        is_null: bool,
        BrokerID: String,
        AccountID: String,
        OldPassword: String,
        NewPassword: String,
        CurrencyID: String,
        FromSec: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct RiskForbiddenRightField {
        is_null: bool,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
        UserID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct InvestorInfoCommRecField {
        is_null: bool,
        ExchangeID: String,
        BrokerID: String,
        InvestorID: String,
        InstrumentID: String,
        OrderCount: i32,
        OrderActionCount: i32,
        ForQuoteCnt: i32,
        InfoComm: f64,
        IsOptSeries: i32,
        ProductID: String,
        InfoCnt: i32,
    }
    #[derive(Debug, Clone, Default)]
    struct QryInvestorInfoCommRecField {
        is_null: bool,
        InvestorID: String,
        InstrumentID: String,
        BrokerID: String,
    }
    #[derive(Debug, Clone, Default)]
    struct FrontInfoField {
        is_null: bool,
        FrontAddr: String,
        QryFreq: i32,
        FTDPkgFreq: i32,
    }
}