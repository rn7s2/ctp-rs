//! CTP（综合交易平台）C++ 接口的 Rust 绑定。
//!
//! 由官方 CTP 头文件生成；结构体字段名称与 CTP 原始命名保持一致。
//!
//! # [`MdApi`] — 行情接口
//!
//! 通过 [`MdApi::CreateMdApiAndSpi`] 创建实例，传入一个
//! [`std::sync::mpsc::Sender`] 用于接收 [`MdSpiMsg`] 回调消息。
//!
//! # [`TraderApi`] — 交易接口
//!
//! 通过 [`TraderApi::CreateTraderApiAndSpi`] 创建实例，传入一个
//! [`std::sync::mpsc::Sender`] 用于接收 [`TraderSpiMsg`] 回调消息。

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub use ffi::*;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::mpsc::Sender;

pub type UniquePtr<T> = cxx::UniquePtr<T>;

pub enum THOST_TE_RESUME_TYPE {
    THOST_TERT_RESTART = 0,
    THOST_TERT_RESUME,
    THOST_TERT_QUICK,
    THOST_TERT_NONE,
}

/// TFtdcExchangePropertyType是一个交易所属性类型
///
/// 正常
pub const THOST_FTDC_EXP_Normal: u8 = '0' as u8;
/// TFtdcExchangePropertyType是一个交易所属性类型
///
/// 根据成交生成报单
pub const THOST_FTDC_EXP_GenOrderByTrade: u8 = '1' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 组织机构代码
pub const THOST_FTDC_ICT_EID: u8 = '0' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 中国公民身份证
pub const THOST_FTDC_ICT_IDCard: u8 = '1' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 军官证
pub const THOST_FTDC_ICT_OfficerIDCard: u8 = '2' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 警官证
pub const THOST_FTDC_ICT_PoliceIDCard: u8 = '3' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 士兵证
pub const THOST_FTDC_ICT_SoldierIDCard: u8 = '4' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 户口簿
pub const THOST_FTDC_ICT_HouseholdRegister: u8 = '5' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 护照
pub const THOST_FTDC_ICT_Passport: u8 = '6' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 台胞证
pub const THOST_FTDC_ICT_TaiwanCompatriotIDCard: u8 = '7' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 回乡证
pub const THOST_FTDC_ICT_HomeComingCard: u8 = '8' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 营业执照号
pub const THOST_FTDC_ICT_LicenseNo: u8 = '9' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 税务登记号/当地纳税ID
pub const THOST_FTDC_ICT_TaxNo: u8 = 'A' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 港澳居民来往内地通行证
pub const THOST_FTDC_ICT_HMMainlandTravelPermit: u8 = 'B' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 台湾居民来往大陆通行证
pub const THOST_FTDC_ICT_TwMainlandTravelPermit: u8 = 'C' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 驾照
pub const THOST_FTDC_ICT_DrivingLicense: u8 = 'D' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 当地社保ID
pub const THOST_FTDC_ICT_SocialID: u8 = 'F' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 当地身份证
pub const THOST_FTDC_ICT_LocalID: u8 = 'G' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 商业登记证
pub const THOST_FTDC_ICT_BusinessRegistration: u8 = 'H' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 港澳永久性居民身份证
pub const THOST_FTDC_ICT_HKMCIDCard: u8 = 'I' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 人行开户许可证
pub const THOST_FTDC_ICT_AccountsPermits: u8 = 'J' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 外国人永久居留证
pub const THOST_FTDC_ICT_FrgPrmtRdCard: u8 = 'K' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 资管产品备案函
pub const THOST_FTDC_ICT_CptMngPrdLetter: u8 = 'L' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 港澳台居民居住证
pub const THOST_FTDC_ICT_HKMCTwResidencePermit: u8 = 'M' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 统一社会信用代码
pub const THOST_FTDC_ICT_UniformSocialCreditCode: u8 = 'N' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 机构成立证明文件
pub const THOST_FTDC_ICT_CorporationCertNo: u8 = 'O' as u8;
/// TFtdcIdCardTypeType是一个证件类型类型
///
/// 其他证件
pub const THOST_FTDC_ICT_OtherCard: u8 = 'x' as u8;
/// TFtdcInvestorRangeType是一个投资者范围类型
///
/// 所有
pub const THOST_FTDC_IR_All: u8 = '1' as u8;
/// TFtdcInvestorRangeType是一个投资者范围类型
///
/// 投资者组
pub const THOST_FTDC_IR_Group: u8 = '2' as u8;
/// TFtdcInvestorRangeType是一个投资者范围类型
///
/// 单一投资者
pub const THOST_FTDC_IR_Single: u8 = '3' as u8;
/// TFtdcDepartmentRangeType是一个投资者范围类型
///
/// 所有
pub const THOST_FTDC_DR_All: u8 = '1' as u8;
/// TFtdcDepartmentRangeType是一个投资者范围类型
///
/// 组织架构
pub const THOST_FTDC_DR_Group: u8 = '2' as u8;
/// TFtdcDepartmentRangeType是一个投资者范围类型
///
/// 单一投资者
pub const THOST_FTDC_DR_Single: u8 = '3' as u8;
/// TFtdcDataSyncStatusType是一个数据同步状态类型
///
/// 未同步
pub const THOST_FTDC_DS_Asynchronous: u8 = '1' as u8;
/// TFtdcDataSyncStatusType是一个数据同步状态类型
///
/// 同步中
pub const THOST_FTDC_DS_Synchronizing: u8 = '2' as u8;
/// TFtdcDataSyncStatusType是一个数据同步状态类型
///
/// 已同步
pub const THOST_FTDC_DS_Synchronized: u8 = '3' as u8;
/// TFtdcBrokerDataSyncStatusType是一个经纪公司数据同步状态类型
///
/// 已同步
pub const THOST_FTDC_BDS_Synchronized: u8 = '1' as u8;
/// TFtdcBrokerDataSyncStatusType是一个经纪公司数据同步状态类型
///
/// 同步中
pub const THOST_FTDC_BDS_Synchronizing: u8 = '2' as u8;
/// TFtdcExchangeConnectStatusType是一个交易所连接状态类型
///
/// 没有任何连接
pub const THOST_FTDC_ECS_NoConnection: u8 = '1' as u8;
/// TFtdcExchangeConnectStatusType是一个交易所连接状态类型
///
/// 已经发出合约查询请求
pub const THOST_FTDC_ECS_QryInstrumentSent: u8 = '2' as u8;
/// TFtdcExchangeConnectStatusType是一个交易所连接状态类型
///
/// 已经获取信息
pub const THOST_FTDC_ECS_GotInformation: u8 = '9' as u8;
/// TFtdcTraderConnectStatusType是一个交易所交易员连接状态类型
///
/// 没有任何连接
pub const THOST_FTDC_TCS_NotConnected: u8 = '1' as u8;
/// TFtdcTraderConnectStatusType是一个交易所交易员连接状态类型
///
/// 已经连接
pub const THOST_FTDC_TCS_Connected: u8 = '2' as u8;
/// TFtdcTraderConnectStatusType是一个交易所交易员连接状态类型
///
/// 已经发出合约查询请求
pub const THOST_FTDC_TCS_QryInstrumentSent: u8 = '3' as u8;
/// TFtdcTraderConnectStatusType是一个交易所交易员连接状态类型
///
/// 订阅私有流
pub const THOST_FTDC_TCS_SubPrivateFlow: u8 = '4' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 数据异步化
pub const THOST_FTDC_FC_DataAsync: u8 = '1' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 强制用户登出
pub const THOST_FTDC_FC_ForceUserLogout: u8 = '2' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 变更管理用户口令
pub const THOST_FTDC_FC_UserPasswordUpdate: u8 = '3' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 变更经纪公司口令
pub const THOST_FTDC_FC_BrokerPasswordUpdate: u8 = '4' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 变更投资者口令
pub const THOST_FTDC_FC_InvestorPasswordUpdate: u8 = '5' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 报单插入
pub const THOST_FTDC_FC_OrderInsert: u8 = '6' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 报单操作
pub const THOST_FTDC_FC_OrderAction: u8 = '7' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 同步系统数据
pub const THOST_FTDC_FC_SyncSystemData: u8 = '8' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 同步经纪公司数据
pub const THOST_FTDC_FC_SyncBrokerData: u8 = '9' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 批量同步经纪公司数据
pub const THOST_FTDC_FC_BachSyncBrokerData: u8 = 'A' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 超级查询
pub const THOST_FTDC_FC_SuperQuery: u8 = 'B' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 预埋报单插入
pub const THOST_FTDC_FC_ParkedOrderInsert: u8 = 'C' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 预埋报单操作
pub const THOST_FTDC_FC_ParkedOrderAction: u8 = 'D' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 同步动态令牌
pub const THOST_FTDC_FC_SyncOTP: u8 = 'E' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 删除未知单
pub const THOST_FTDC_FC_DeleteOrder: u8 = 'F' as u8;
/// TFtdcFunctionCodeType是一个功能代码类型
///
/// 退出紧急状态
pub const THOST_FTDC_FC_ExitEmergency: u8 = 'G' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 强制用户登出
pub const THOST_FTDC_BFC_ForceUserLogout: u8 = '1' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 变更用户口令
pub const THOST_FTDC_BFC_UserPasswordUpdate: u8 = '2' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 同步经纪公司数据
pub const THOST_FTDC_BFC_SyncBrokerData: u8 = '3' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 批量同步经纪公司数据
pub const THOST_FTDC_BFC_BachSyncBrokerData: u8 = '4' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 报单插入
pub const THOST_FTDC_BFC_OrderInsert: u8 = '5' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 报单操作
pub const THOST_FTDC_BFC_OrderAction: u8 = '6' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 全部查询
pub const THOST_FTDC_BFC_AllQuery: u8 = '7' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 系统功能：登入/登出/修改密码等
pub const THOST_FTDC_BFC_log: u8 = 'a' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 基本查询：查询基础数据，如合约，交易所等常量
pub const THOST_FTDC_BFC_BaseQry: u8 = 'b' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 交易查询：如查成交，委托
pub const THOST_FTDC_BFC_TradeQry: u8 = 'c' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 交易功能：报单，撤单
pub const THOST_FTDC_BFC_Trade: u8 = 'd' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 银期转账
pub const THOST_FTDC_BFC_Virement: u8 = 'e' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风险监控
pub const THOST_FTDC_BFC_Risk: u8 = 'f' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 查询/管理：查询会话，踢人等
pub const THOST_FTDC_BFC_Session: u8 = 'g' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风控通知控制
pub const THOST_FTDC_BFC_RiskNoticeCtl: u8 = 'h' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风控通知发送
pub const THOST_FTDC_BFC_RiskNotice: u8 = 'i' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 察看经纪公司资金权限
pub const THOST_FTDC_BFC_BrokerDeposit: u8 = 'j' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 资金查询
pub const THOST_FTDC_BFC_QueryFund: u8 = 'k' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 报单查询
pub const THOST_FTDC_BFC_QueryOrder: u8 = 'l' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 成交查询
pub const THOST_FTDC_BFC_QueryTrade: u8 = 'm' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 持仓查询
pub const THOST_FTDC_BFC_QueryPosition: u8 = 'n' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 行情查询
pub const THOST_FTDC_BFC_QueryMarketData: u8 = 'o' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 用户事件查询
pub const THOST_FTDC_BFC_QueryUserEvent: u8 = 'p' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风险通知查询
pub const THOST_FTDC_BFC_QueryRiskNotify: u8 = 'q' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 出入金查询
pub const THOST_FTDC_BFC_QueryFundChange: u8 = 'r' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 投资者信息查询
pub const THOST_FTDC_BFC_QueryInvestor: u8 = 's' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 交易编码查询
pub const THOST_FTDC_BFC_QueryTradingCode: u8 = 't' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 强平
pub const THOST_FTDC_BFC_ForceClose: u8 = 'u' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 压力测试
pub const THOST_FTDC_BFC_PressTest: u8 = 'v' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 权益反算
pub const THOST_FTDC_BFC_RemainCalc: u8 = 'w' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 净持仓保证金指标
pub const THOST_FTDC_BFC_NetPositionInd: u8 = 'x' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风险预算
pub const THOST_FTDC_BFC_RiskPredict: u8 = 'y' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 数据导出
pub const THOST_FTDC_BFC_DataExport: u8 = 'z' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风控指标设置
pub const THOST_FTDC_BFC_RiskTargetSetup: u8 = 'A' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 行情预警
pub const THOST_FTDC_BFC_MarketDataWarn: u8 = 'B' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 业务通知查询
pub const THOST_FTDC_BFC_QryBizNotice: u8 = 'C' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 业务通知模板设置
pub const THOST_FTDC_BFC_CfgBizNotice: u8 = 'D' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 同步动态令牌
pub const THOST_FTDC_BFC_SyncOTP: u8 = 'E' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 发送业务通知
pub const THOST_FTDC_BFC_SendBizNotice: u8 = 'F' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 风险级别标准设置
pub const THOST_FTDC_BFC_CfgRiskLevelStd: u8 = 'G' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 交易终端应急功能
pub const THOST_FTDC_BFC_TbCommand: u8 = 'H' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 删除未知单
pub const THOST_FTDC_BFC_DeleteOrder: u8 = 'J' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 预埋报单插入
pub const THOST_FTDC_BFC_ParkedOrderInsert: u8 = 'K' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 预埋报单操作
pub const THOST_FTDC_BFC_ParkedOrderAction: u8 = 'L' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 资金不够仍允许行权
pub const THOST_FTDC_BFC_ExecOrderNoCheck: u8 = 'M' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 指定
pub const THOST_FTDC_BFC_Designate: u8 = 'N' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 证券处置
pub const THOST_FTDC_BFC_StockDisposal: u8 = 'O' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 席位资金预警
pub const THOST_FTDC_BFC_BrokerDepositWarn: u8 = 'Q' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 备兑不足预警
pub const THOST_FTDC_BFC_CoverWarn: u8 = 'S' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 行权试算
pub const THOST_FTDC_BFC_PreExecOrder: u8 = 'T' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 行权交收风险
pub const THOST_FTDC_BFC_ExecOrderRisk: u8 = 'P' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 持仓限额预警
pub const THOST_FTDC_BFC_PosiLimitWarn: u8 = 'U' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 持仓限额查询
pub const THOST_FTDC_BFC_QryPosiLimit: u8 = 'V' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 银期签到签退
pub const THOST_FTDC_BFC_FBSign: u8 = 'W' as u8;
/// TFtdcBrokerFunctionCodeType是一个经纪公司功能代码类型
///
/// 银期签约解约
pub const THOST_FTDC_BFC_FBAccount: u8 = 'X' as u8;
/// TFtdcOrderActionStatusType是一个报单操作状态类型
///
/// 已经提交
pub const THOST_FTDC_OAS_Submitted: u8 = 'a' as u8;
/// TFtdcOrderActionStatusType是一个报单操作状态类型
///
/// 已经接受
pub const THOST_FTDC_OAS_Accepted: u8 = 'b' as u8;
/// TFtdcOrderActionStatusType是一个报单操作状态类型
///
/// 已经被拒绝
pub const THOST_FTDC_OAS_Rejected: u8 = 'c' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 全部成交
pub const THOST_FTDC_OST_AllTraded: u8 = '0' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 部分成交还在队列中
pub const THOST_FTDC_OST_PartTradedQueueing: u8 = '1' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 部分成交不在队列中
pub const THOST_FTDC_OST_PartTradedNotQueueing: u8 = '2' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 未成交还在队列中
pub const THOST_FTDC_OST_NoTradeQueueing: u8 = '3' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 未成交不在队列中
pub const THOST_FTDC_OST_NoTradeNotQueueing: u8 = '4' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 撤单
pub const THOST_FTDC_OST_Canceled: u8 = '5' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 未知
pub const THOST_FTDC_OST_Unknown: u8 = 'a' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 尚未触发
pub const THOST_FTDC_OST_NotTouched: u8 = 'b' as u8;
/// TFtdcOrderStatusType是一个报单状态类型
///
/// 已触发
pub const THOST_FTDC_OST_Touched: u8 = 'c' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 已经提交
pub const THOST_FTDC_OSS_InsertSubmitted: u8 = '0' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 撤单已经提交
pub const THOST_FTDC_OSS_CancelSubmitted: u8 = '1' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 修改已经提交
pub const THOST_FTDC_OSS_ModifySubmitted: u8 = '2' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 已经接受
pub const THOST_FTDC_OSS_Accepted: u8 = '3' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 报单已经被拒绝
pub const THOST_FTDC_OSS_InsertRejected: u8 = '4' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 撤单已经被拒绝
pub const THOST_FTDC_OSS_CancelRejected: u8 = '5' as u8;
/// TFtdcOrderSubmitStatusType是一个报单提交状态类型
///
/// 改单已经被拒绝
pub const THOST_FTDC_OSS_ModifyRejected: u8 = '6' as u8;
/// TFtdcPositionDateType是一个持仓日期类型
///
/// 今日持仓
pub const THOST_FTDC_PSD_Today: u8 = '1' as u8;
/// TFtdcPositionDateType是一个持仓日期类型
///
/// 历史持仓
pub const THOST_FTDC_PSD_History: u8 = '2' as u8;
/// TFtdcPositionDateTypeType是一个持仓日期类型类型
///
/// 使用历史持仓
pub const THOST_FTDC_PDT_UseHistory: u8 = '1' as u8;
/// TFtdcPositionDateTypeType是一个持仓日期类型类型
///
/// 不使用历史持仓
pub const THOST_FTDC_PDT_NoUseHistory: u8 = '2' as u8;
/// TFtdcTradingRoleType是一个交易角色类型
///
/// 代理
pub const THOST_FTDC_ER_Broker: u8 = '1' as u8;
/// TFtdcTradingRoleType是一个交易角色类型
///
/// 自营
pub const THOST_FTDC_ER_Host: u8 = '2' as u8;
/// TFtdcTradingRoleType是一个交易角色类型
///
/// 做市商
pub const THOST_FTDC_ER_Maker: u8 = '3' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 期货
pub const THOST_FTDC_PC_Futures: u8 = '1' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 期货期权
pub const THOST_FTDC_PC_Options: u8 = '2' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 组合
pub const THOST_FTDC_PC_Combination: u8 = '3' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 即期
pub const THOST_FTDC_PC_Spot: u8 = '4' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 期转现
pub const THOST_FTDC_PC_EFP: u8 = '5' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 现货期权
pub const THOST_FTDC_PC_SpotOption: u8 = '6' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// TAS合约
pub const THOST_FTDC_PC_TAS: u8 = '7' as u8;
/// TFtdcProductClassType是一个产品类型类型
///
/// 金属指数
pub const THOST_FTDC_PC_MI: u8 = 'I' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 期货单一合约
pub const THOST_FTDC_APC_FutureSingle: u8 = '1' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 期权单一合约
pub const THOST_FTDC_APC_OptionSingle: u8 = '2' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 可交易期货(含期货组合和期货单一合约)
pub const THOST_FTDC_APC_Futures: u8 = '3' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 可交易期权(含期权组合和期权单一合约)
pub const THOST_FTDC_APC_Options: u8 = '4' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 可下单套利组合
pub const THOST_FTDC_APC_TradingComb: u8 = '5' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 可申请的组合（可以申请的组合合约 包含可以交易的合约）
pub const THOST_FTDC_APC_UnTradingComb: u8 = '6' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 所有可以交易合约
pub const THOST_FTDC_APC_AllTrading: u8 = '7' as u8;
/// TFtdcAPIProductClassType是一个产品类型类型
///
/// 所有合约（包含不能交易合约 慎用）
pub const THOST_FTDC_APC_All: u8 = '8' as u8;
/// TFtdcInstLifePhaseType是一个合约生命周期状态类型
///
/// 未上市
pub const THOST_FTDC_IP_NotStart: u8 = '0' as u8;
/// TFtdcInstLifePhaseType是一个合约生命周期状态类型
///
/// 上市
pub const THOST_FTDC_IP_Started: u8 = '1' as u8;
/// TFtdcInstLifePhaseType是一个合约生命周期状态类型
///
/// 停牌
pub const THOST_FTDC_IP_Pause: u8 = '2' as u8;
/// TFtdcInstLifePhaseType是一个合约生命周期状态类型
///
/// 到期
pub const THOST_FTDC_IP_Expired: u8 = '3' as u8;
/// TFtdcDirectionType是一个买卖方向类型
///
/// 买
pub const THOST_FTDC_D_Buy: u8 = '0' as u8;
/// TFtdcDirectionType是一个买卖方向类型
///
/// 卖
pub const THOST_FTDC_D_Sell: u8 = '1' as u8;
/// TFtdcPositionTypeType是一个持仓类型类型
///
/// 净持仓
pub const THOST_FTDC_PT_Net: u8 = '1' as u8;
/// TFtdcPositionTypeType是一个持仓类型类型
///
/// 综合持仓
pub const THOST_FTDC_PT_Gross: u8 = '2' as u8;
/// TFtdcPosiDirectionType是一个持仓多空方向类型
///
/// 净
pub const THOST_FTDC_PD_Net: u8 = '1' as u8;
/// TFtdcPosiDirectionType是一个持仓多空方向类型
///
/// 多头
pub const THOST_FTDC_PD_Long: u8 = '2' as u8;
/// TFtdcPosiDirectionType是一个持仓多空方向类型
///
/// 空头
pub const THOST_FTDC_PD_Short: u8 = '3' as u8;
/// TFtdcSysSettlementStatusType是一个系统结算状态类型
///
/// 不活跃
pub const THOST_FTDC_SS_NonActive: u8 = '1' as u8;
/// TFtdcSysSettlementStatusType是一个系统结算状态类型
///
/// 启动
pub const THOST_FTDC_SS_Startup: u8 = '2' as u8;
/// TFtdcSysSettlementStatusType是一个系统结算状态类型
///
/// 操作
pub const THOST_FTDC_SS_Operating: u8 = '3' as u8;
/// TFtdcSysSettlementStatusType是一个系统结算状态类型
///
/// 结算
pub const THOST_FTDC_SS_Settlement: u8 = '4' as u8;
/// TFtdcSysSettlementStatusType是一个系统结算状态类型
///
/// 结算完成
pub const THOST_FTDC_SS_SettlementFinished: u8 = '5' as u8;
/// TFtdcRatioAttrType是一个费率属性类型
///
/// 交易费率
pub const THOST_FTDC_RA_Trade: u8 = '0' as u8;
/// TFtdcRatioAttrType是一个费率属性类型
///
/// 结算费率
pub const THOST_FTDC_RA_Settlement: u8 = '1' as u8;
/// TFtdcHedgeFlagType是一个投机套保标志类型
///
/// 投机
pub const THOST_FTDC_HF_Speculation: u8 = '1' as u8;
/// TFtdcHedgeFlagType是一个投机套保标志类型
///
/// 套利
pub const THOST_FTDC_HF_Arbitrage: u8 = '2' as u8;
/// TFtdcHedgeFlagType是一个投机套保标志类型
///
/// 套保
pub const THOST_FTDC_HF_Hedge: u8 = '3' as u8;
/// TFtdcHedgeFlagType是一个投机套保标志类型
///
/// 做市商
pub const THOST_FTDC_HF_MarketMaker: u8 = '5' as u8;
/// TFtdcHedgeFlagType是一个投机套保标志类型
///
/// 第一腿投机第二腿套保
pub const THOST_FTDC_HF_SpecHedge: u8 = '6' as u8;
/// TFtdcHedgeFlagType是一个投机套保标志类型
///
/// 第一腿套保第二腿投机
pub const THOST_FTDC_HF_HedgeSpec: u8 = '7' as u8;
/// TFtdcBillHedgeFlagType是一个投机套保标志类型
///
/// 投机
pub const THOST_FTDC_BHF_Speculation: u8 = '1' as u8;
/// TFtdcBillHedgeFlagType是一个投机套保标志类型
///
/// 套利
pub const THOST_FTDC_BHF_Arbitrage: u8 = '2' as u8;
/// TFtdcBillHedgeFlagType是一个投机套保标志类型
///
/// 套保
pub const THOST_FTDC_BHF_Hedge: u8 = '3' as u8;
/// TFtdcClientIDTypeType是一个交易编码类型类型
///
/// 投机
pub const THOST_FTDC_CIDT_Speculation: u8 = '1' as u8;
/// TFtdcClientIDTypeType是一个交易编码类型类型
///
/// 套利
pub const THOST_FTDC_CIDT_Arbitrage: u8 = '2' as u8;
/// TFtdcClientIDTypeType是一个交易编码类型类型
///
/// 套保
pub const THOST_FTDC_CIDT_Hedge: u8 = '3' as u8;
/// TFtdcClientIDTypeType是一个交易编码类型类型
///
/// 做市商
pub const THOST_FTDC_CIDT_MarketMaker: u8 = '5' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 任意价
pub const THOST_FTDC_OPT_AnyPrice: u8 = '1' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 限价
pub const THOST_FTDC_OPT_LimitPrice: u8 = '2' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 最优价
pub const THOST_FTDC_OPT_BestPrice: u8 = '3' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 最新价
pub const THOST_FTDC_OPT_LastPrice: u8 = '4' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 最新价浮动上浮1个ticks
pub const THOST_FTDC_OPT_LastPricePlusOneTicks: u8 = '5' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 最新价浮动上浮2个ticks
pub const THOST_FTDC_OPT_LastPricePlusTwoTicks: u8 = '6' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 最新价浮动上浮3个ticks
pub const THOST_FTDC_OPT_LastPricePlusThreeTicks: u8 = '7' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 卖一价
pub const THOST_FTDC_OPT_AskPrice1: u8 = '8' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 卖一价浮动上浮1个ticks
pub const THOST_FTDC_OPT_AskPrice1PlusOneTicks: u8 = '9' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 卖一价浮动上浮2个ticks
pub const THOST_FTDC_OPT_AskPrice1PlusTwoTicks: u8 = 'A' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 卖一价浮动上浮3个ticks
pub const THOST_FTDC_OPT_AskPrice1PlusThreeTicks: u8 = 'B' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 买一价
pub const THOST_FTDC_OPT_BidPrice1: u8 = 'C' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 买一价浮动上浮1个ticks
pub const THOST_FTDC_OPT_BidPrice1PlusOneTicks: u8 = 'D' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 买一价浮动上浮2个ticks
pub const THOST_FTDC_OPT_BidPrice1PlusTwoTicks: u8 = 'E' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 买一价浮动上浮3个ticks
pub const THOST_FTDC_OPT_BidPrice1PlusThreeTicks: u8 = 'F' as u8;
/// TFtdcOrderPriceTypeType是一个报单价格条件类型
///
/// 五档价
pub const THOST_FTDC_OPT_FiveLevelPrice: u8 = 'G' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 开仓
pub const THOST_FTDC_OF_Open: u8 = '0' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 平仓
pub const THOST_FTDC_OF_Close: u8 = '1' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 强平
pub const THOST_FTDC_OF_ForceClose: u8 = '2' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 平今
pub const THOST_FTDC_OF_CloseToday: u8 = '3' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 平昨
pub const THOST_FTDC_OF_CloseYesterday: u8 = '4' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 强减
pub const THOST_FTDC_OF_ForceOff: u8 = '5' as u8;
/// TFtdcOffsetFlagType是一个开平标志类型
///
/// 本地强平
pub const THOST_FTDC_OF_LocalForceClose: u8 = '6' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 非强平
pub const THOST_FTDC_FCC_NotForceClose: u8 = '0' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 资金不足
pub const THOST_FTDC_FCC_LackDeposit: u8 = '1' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 客户超仓
pub const THOST_FTDC_FCC_ClientOverPositionLimit: u8 = '2' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 会员超仓
pub const THOST_FTDC_FCC_MemberOverPositionLimit: u8 = '3' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 持仓非整数倍
pub const THOST_FTDC_FCC_NotMultiple: u8 = '4' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 违规
pub const THOST_FTDC_FCC_Violation: u8 = '5' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 其它
pub const THOST_FTDC_FCC_Other: u8 = '6' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 自然人临近交割
pub const THOST_FTDC_FCC_PersonDeliv: u8 = '7' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 本地强平资金不足忽略敞口
pub const THOST_FTDC_FCC_Notverifycapital: u8 = '8' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 本地强平资金不足
pub const THOST_FTDC_FCC_LocalLackDeposit: u8 = '9' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 本地强平违规持仓忽略敞口
pub const THOST_FTDC_FCC_LocalViolationNocheck: u8 = 'a' as u8;
/// TFtdcForceCloseReasonType是一个强平原因类型
///
/// 本地强平违规持仓
pub const THOST_FTDC_FCC_LocalViolation: u8 = 'b' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 正常
pub const THOST_FTDC_ORDT_Normal: u8 = '0' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 报价衍生
pub const THOST_FTDC_ORDT_DeriveFromQuote: u8 = '1' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 组合衍生
pub const THOST_FTDC_ORDT_DeriveFromCombination: u8 = '2' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 组合报单
pub const THOST_FTDC_ORDT_Combination: u8 = '3' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 条件单
pub const THOST_FTDC_ORDT_ConditionalOrder: u8 = '4' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 互换单
pub const THOST_FTDC_ORDT_Swap: u8 = '5' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 大宗交易成交衍生
pub const THOST_FTDC_ORDT_DeriveFromBlockTrade: u8 = '6' as u8;
/// TFtdcOrderTypeType是一个报单类型类型
///
/// 期转现成交衍生
pub const THOST_FTDC_ORDT_DeriveFromEFPTrade: u8 = '7' as u8;
/// TFtdcTimeConditionType是一个有效期类型类型
///
/// 立即完成，否则撤销
pub const THOST_FTDC_TC_IOC: u8 = '1' as u8;
/// TFtdcTimeConditionType是一个有效期类型类型
///
/// 本节有效
pub const THOST_FTDC_TC_GFS: u8 = '2' as u8;
/// TFtdcTimeConditionType是一个有效期类型类型
///
/// 当日有效
pub const THOST_FTDC_TC_GFD: u8 = '3' as u8;
/// TFtdcTimeConditionType是一个有效期类型类型
///
/// 指定日期前有效
pub const THOST_FTDC_TC_GTD: u8 = '4' as u8;
/// TFtdcTimeConditionType是一个有效期类型类型
///
/// 撤销前有效
pub const THOST_FTDC_TC_GTC: u8 = '5' as u8;
/// TFtdcTimeConditionType是一个有效期类型类型
///
/// 集合竞价有效
pub const THOST_FTDC_TC_GFA: u8 = '6' as u8;
/// TFtdcVolumeConditionType是一个成交量类型类型
///
/// 任何数量
pub const THOST_FTDC_VC_AV: u8 = '1' as u8;
/// TFtdcVolumeConditionType是一个成交量类型类型
///
/// 最小数量
pub const THOST_FTDC_VC_MV: u8 = '2' as u8;
/// TFtdcVolumeConditionType是一个成交量类型类型
///
/// 全部数量
pub const THOST_FTDC_VC_CV: u8 = '3' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 立即
pub const THOST_FTDC_CC_Immediately: u8 = '1' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 止损
pub const THOST_FTDC_CC_Touch: u8 = '2' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 止赢
pub const THOST_FTDC_CC_TouchProfit: u8 = '3' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 预埋单
pub const THOST_FTDC_CC_ParkedOrder: u8 = '4' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 最新价大于条件价
pub const THOST_FTDC_CC_LastPriceGreaterThanStopPrice: u8 = '5' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 最新价大于等于条件价
pub const THOST_FTDC_CC_LastPriceGreaterEqualStopPrice: u8 = '6' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 最新价小于条件价
pub const THOST_FTDC_CC_LastPriceLesserThanStopPrice: u8 = '7' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 最新价小于等于条件价
pub const THOST_FTDC_CC_LastPriceLesserEqualStopPrice: u8 = '8' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 卖一价大于条件价
pub const THOST_FTDC_CC_AskPriceGreaterThanStopPrice: u8 = '9' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 卖一价大于等于条件价
pub const THOST_FTDC_CC_AskPriceGreaterEqualStopPrice: u8 = 'A' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 卖一价小于条件价
pub const THOST_FTDC_CC_AskPriceLesserThanStopPrice: u8 = 'B' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 卖一价小于等于条件价
pub const THOST_FTDC_CC_AskPriceLesserEqualStopPrice: u8 = 'C' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 买一价大于条件价
pub const THOST_FTDC_CC_BidPriceGreaterThanStopPrice: u8 = 'D' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 买一价大于等于条件价
pub const THOST_FTDC_CC_BidPriceGreaterEqualStopPrice: u8 = 'E' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 买一价小于条件价
pub const THOST_FTDC_CC_BidPriceLesserThanStopPrice: u8 = 'F' as u8;
/// TFtdcContingentConditionType是一个触发条件类型
///
/// 买一价小于等于条件价
pub const THOST_FTDC_CC_BidPriceLesserEqualStopPrice: u8 = 'H' as u8;
/// TFtdcActionFlagType是一个操作标志类型
///
/// 删除
pub const THOST_FTDC_AF_Delete: u8 = '0' as u8;
/// TFtdcActionFlagType是一个操作标志类型
///
/// 修改
pub const THOST_FTDC_AF_Modify: u8 = '3' as u8;
/// TFtdcTradingRightType是一个交易权限类型
///
/// 可以交易
pub const THOST_FTDC_TR_Allow: u8 = '0' as u8;
/// TFtdcTradingRightType是一个交易权限类型
///
/// 只能平仓
pub const THOST_FTDC_TR_CloseOnly: u8 = '1' as u8;
/// TFtdcTradingRightType是一个交易权限类型
///
/// 不能交易
pub const THOST_FTDC_TR_Forbidden: u8 = '2' as u8;
/// TFtdcOrderSourceType是一个报单来源类型
///
/// 来自参与者
pub const THOST_FTDC_OSRC_Participant: u8 = '0' as u8;
/// TFtdcOrderSourceType是一个报单来源类型
///
/// 来自管理员
pub const THOST_FTDC_OSRC_Administrator: u8 = '1' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// 组合持仓拆分为单一持仓,初始化不应包含该类型的持仓
pub const THOST_FTDC_TRDT_SplitCombination: u8 = '#' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// 普通成交
pub const THOST_FTDC_TRDT_Common: u8 = '0' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// 期权执行
pub const THOST_FTDC_TRDT_OptionsExecution: u8 = '1' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// OTC成交
pub const THOST_FTDC_TRDT_OTC: u8 = '2' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// 期转现衍生成交
pub const THOST_FTDC_TRDT_EFPDerived: u8 = '3' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// 组合衍生成交
pub const THOST_FTDC_TRDT_CombinationDerived: u8 = '4' as u8;
/// TFtdcTradeTypeType是一个成交类型类型
///
/// 大宗交易成交
pub const THOST_FTDC_TRDT_BlockTrade: u8 = '5' as u8;
/// TFtdcSpecPosiTypeType是一个特殊持仓明细标识类型
///
/// 普通持仓明细
pub const THOST_FTDC_SPOST_Common: u8 = '#' as u8;
/// TFtdcSpecPosiTypeType是一个特殊持仓明细标识类型
///
/// TAS合约成交产生的标的合约持仓明细
pub const THOST_FTDC_SPOST_Tas: u8 = '0' as u8;
/// TFtdcPriceSourceType是一个成交价来源类型
///
/// 前成交价
pub const THOST_FTDC_PSRC_LastPrice: u8 = '0' as u8;
/// TFtdcPriceSourceType是一个成交价来源类型
///
/// 买委托价
pub const THOST_FTDC_PSRC_Buy: u8 = '1' as u8;
/// TFtdcPriceSourceType是一个成交价来源类型
///
/// 卖委托价
pub const THOST_FTDC_PSRC_Sell: u8 = '2' as u8;
/// TFtdcPriceSourceType是一个成交价来源类型
///
/// 场外成交价
pub const THOST_FTDC_PSRC_OTC: u8 = '3' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 开盘前
pub const THOST_FTDC_IS_BeforeTrading: u8 = '0' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 非交易
pub const THOST_FTDC_IS_NoTrading: u8 = '1' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 连续交易
pub const THOST_FTDC_IS_Continous: u8 = '2' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 集合竞价报单
pub const THOST_FTDC_IS_AuctionOrdering: u8 = '3' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 集合竞价价格平衡
pub const THOST_FTDC_IS_AuctionBalance: u8 = '4' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 集合竞价撮合
pub const THOST_FTDC_IS_AuctionMatch: u8 = '5' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 收盘
pub const THOST_FTDC_IS_Closed: u8 = '6' as u8;
/// TFtdcInstrumentStatusType是一个合约交易状态类型
///
/// 交易业务处理
pub const THOST_FTDC_IS_TransactionProcessing: u8 = '7' as u8;
/// TFtdcInstStatusEnterReasonType是一个品种进入交易状态原因类型
///
/// 自动切换
pub const THOST_FTDC_IER_Automatic: u8 = '1' as u8;
/// TFtdcInstStatusEnterReasonType是一个品种进入交易状态原因类型
///
/// 手动切换
pub const THOST_FTDC_IER_Manual: u8 = '2' as u8;
/// TFtdcInstStatusEnterReasonType是一个品种进入交易状态原因类型
///
/// 熔断
pub const THOST_FTDC_IER_Fuse: u8 = '3' as u8;
/// TFtdcBatchStatusType是一个处理状态类型
///
/// 未上传
pub const THOST_FTDC_BS_NoUpload: u8 = '1' as u8;
/// TFtdcBatchStatusType是一个处理状态类型
///
/// 已上传
pub const THOST_FTDC_BS_Uploaded: u8 = '2' as u8;
/// TFtdcBatchStatusType是一个处理状态类型
///
/// 审核失败
pub const THOST_FTDC_BS_Failed: u8 = '3' as u8;
/// TFtdcReturnStyleType是一个按品种返还方式类型
///
/// 按所有品种
pub const THOST_FTDC_RS_All: u8 = '1' as u8;
/// TFtdcReturnStyleType是一个按品种返还方式类型
///
/// 按品种
pub const THOST_FTDC_RS_ByProduct: u8 = '2' as u8;
/// TFtdcReturnPatternType是一个返还模式类型
///
/// 按成交手数
pub const THOST_FTDC_RP_ByVolume: u8 = '1' as u8;
/// TFtdcReturnPatternType是一个返还模式类型
///
/// 按留存手续费
pub const THOST_FTDC_RP_ByFeeOnHand: u8 = '2' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别1
pub const THOST_FTDC_RL_Level1: u8 = '1' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别2
pub const THOST_FTDC_RL_Level2: u8 = '2' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别3
pub const THOST_FTDC_RL_Level3: u8 = '3' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别4
pub const THOST_FTDC_RL_Level4: u8 = '4' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别5
pub const THOST_FTDC_RL_Level5: u8 = '5' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别6
pub const THOST_FTDC_RL_Level6: u8 = '6' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别7
pub const THOST_FTDC_RL_Level7: u8 = '7' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别8
pub const THOST_FTDC_RL_Level8: u8 = '8' as u8;
/// TFtdcReturnLevelType是一个返还级别类型
///
/// 级别9
pub const THOST_FTDC_RL_Level9: u8 = '9' as u8;
/// TFtdcReturnStandardType是一个返还标准类型
///
/// 分阶段返还
pub const THOST_FTDC_RSD_ByPeriod: u8 = '1' as u8;
/// TFtdcReturnStandardType是一个返还标准类型
///
/// 按某一标准
pub const THOST_FTDC_RSD_ByStandard: u8 = '2' as u8;
/// TFtdcMortgageTypeType是一个质押类型类型
///
/// 质出
pub const THOST_FTDC_MT_Out: u8 = '0' as u8;
/// TFtdcMortgageTypeType是一个质押类型类型
///
/// 质入
pub const THOST_FTDC_MT_In: u8 = '1' as u8;
/// TFtdcInvestorSettlementParamIDType是一个投资者结算参数代码类型
///
/// 质押比例
pub const THOST_FTDC_ISPI_MortgageRatio: u8 = '4' as u8;
/// TFtdcInvestorSettlementParamIDType是一个投资者结算参数代码类型
///
/// 保证金算法
pub const THOST_FTDC_ISPI_MarginWay: u8 = '5' as u8;
/// TFtdcInvestorSettlementParamIDType是一个投资者结算参数代码类型
///
/// 结算单结存是否包含质押
pub const THOST_FTDC_ISPI_BillDeposit: u8 = '9' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 质押比例
pub const THOST_FTDC_ESPI_MortgageRatio: u8 = '1' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 分项资金导入项
pub const THOST_FTDC_ESPI_OtherFundItem: u8 = '2' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 分项资金入交易所出入金
pub const THOST_FTDC_ESPI_OtherFundImport: u8 = '3' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 中金所开户最低可用金额
pub const THOST_FTDC_ESPI_CFFEXMinPrepa: u8 = '6' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 郑商所结算方式
pub const THOST_FTDC_ESPI_CZCESettlementType: u8 = '7' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 交易所交割手续费收取方式
pub const THOST_FTDC_ESPI_ExchDelivFeeMode: u8 = '9' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 投资者交割手续费收取方式
pub const THOST_FTDC_ESPI_DelivFeeMode: u8 = '0' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 郑商所组合持仓保证金收取方式
pub const THOST_FTDC_ESPI_CZCEComMarginType: u8 = 'A' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 大商所套利保证金是否优惠
pub const THOST_FTDC_ESPI_DceComMarginType: u8 = 'B' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 虚值期权保证金优惠比率
pub const THOST_FTDC_ESPI_OptOutDisCountRate: u8 = 'a' as u8;
/// TFtdcExchangeSettlementParamIDType是一个交易所结算参数代码类型
///
/// 最低保障系数
pub const THOST_FTDC_ESPI_OptMiniGuarantee: u8 = 'b' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者代码最小长度
pub const THOST_FTDC_SPI_InvestorIDMinLength: u8 = '1' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者帐号代码最小长度
pub const THOST_FTDC_SPI_AccountIDMinLength: u8 = '2' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者开户默认登录权限
pub const THOST_FTDC_SPI_UserRightLogon: u8 = '3' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者交易结算单成交汇总方式
pub const THOST_FTDC_SPI_SettlementBillTrade: u8 = '4' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 统一开户更新交易编码方式
pub const THOST_FTDC_SPI_TradingCode: u8 = '5' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 结算是否判断存在未复核的出入金和分项资金
pub const THOST_FTDC_SPI_CheckFund: u8 = '6' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 是否启用手续费模板数据权限
pub const THOST_FTDC_SPI_CommModelRight: u8 = '7' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 是否启用保证金率模板数据权限
pub const THOST_FTDC_SPI_MarginModelRight: u8 = '9' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 是否规范用户才能激活
pub const THOST_FTDC_SPI_IsStandardActive: u8 = '8' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 上传的交易所结算文件路径
pub const THOST_FTDC_SPI_UploadSettlementFile: u8 = 'U' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 上报保证金监控中心文件路径
pub const THOST_FTDC_SPI_DownloadCSRCFile: u8 = 'D' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 生成的结算单文件路径
pub const THOST_FTDC_SPI_SettlementBillFile: u8 = 'S' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 证监会文件标识
pub const THOST_FTDC_SPI_CSRCOthersFile: u8 = 'C' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者照片路径
pub const THOST_FTDC_SPI_InvestorPhoto: u8 = 'P' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 全结经纪公司上传文件路径
pub const THOST_FTDC_SPI_CSRCData: u8 = 'R' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 开户密码录入方式
pub const THOST_FTDC_SPI_InvestorPwdModel: u8 = 'I' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者中金所结算文件下载路径
pub const THOST_FTDC_SPI_CFFEXInvestorSettleFile: u8 = 'F' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 投资者代码编码方式
pub const THOST_FTDC_SPI_InvestorIDType: u8 = 'a' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 休眠户最高权益
pub const THOST_FTDC_SPI_FreezeMaxReMain: u8 = 'r' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 手续费相关操作实时上场开关
pub const THOST_FTDC_SPI_IsSync: u8 = 'A' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 解除开仓权限限制
pub const THOST_FTDC_SPI_RelieveOpenLimit: u8 = 'O' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 是否规范用户才能休眠
pub const THOST_FTDC_SPI_IsStandardFreeze: u8 = 'X' as u8;
/// TFtdcSystemParamIDType是一个系统参数代码类型
///
/// 郑商所是否开放所有品种套保交易
pub const THOST_FTDC_SPI_CZCENormalProductHedge: u8 = 'B' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 系统加密算法
pub const THOST_FTDC_TPID_EncryptionStandard: u8 = 'E' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 系统风险算法
pub const THOST_FTDC_TPID_RiskMode: u8 = 'R' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 系统风险算法是否全局 0-否 1-是
pub const THOST_FTDC_TPID_RiskModeGlobal: u8 = 'G' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 密码加密算法
pub const THOST_FTDC_TPID_modeEncode: u8 = 'P' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 价格小数位数参数
pub const THOST_FTDC_TPID_tickMode: u8 = 'T' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 用户最大会话数
pub const THOST_FTDC_TPID_SingleUserSessionMaxNum: u8 = 'S' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 最大连续登录失败数
pub const THOST_FTDC_TPID_LoginFailMaxNum: u8 = 'L' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 是否强制认证
pub const THOST_FTDC_TPID_IsAuthForce: u8 = 'A' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 是否冻结证券持仓
pub const THOST_FTDC_TPID_IsPosiFreeze: u8 = 'F' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 是否限仓
pub const THOST_FTDC_TPID_IsPosiLimit: u8 = 'M' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 郑商所询价时间间隔
pub const THOST_FTDC_TPID_ForQuoteTimeInterval: u8 = 'Q' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 是否期货限仓
pub const THOST_FTDC_TPID_IsFuturePosiLimit: u8 = 'B' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 是否期货下单频率限制
pub const THOST_FTDC_TPID_IsFutureOrderFreq: u8 = 'C' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 行权冻结是否计算盈利
pub const THOST_FTDC_TPID_IsExecOrderProfit: u8 = 'H' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 银期开户是否验证开户银行卡号是否是预留银行账户
pub const THOST_FTDC_TPID_IsCheckBankAcc: u8 = 'I' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 弱密码最后修改日期
pub const THOST_FTDC_TPID_PasswordDeadLine: u8 = 'J' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 强密码校验
pub const THOST_FTDC_TPID_IsStrongPassword: u8 = 'K' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 自有资金质押比
pub const THOST_FTDC_TPID_BalanceMorgage: u8 = 'a' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 最小密码长度
pub const THOST_FTDC_TPID_MinPwdLen: u8 = 'O' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// IP当日最大登陆失败次数
pub const THOST_FTDC_TPID_LoginFailMaxNumForIP: u8 = 'U' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 密码有效期
pub const THOST_FTDC_TPID_PasswordPeriod: u8 = 'V' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 历史密码重复限制次数
pub const THOST_FTDC_TPID_PwdHistoryCmp: u8 = 'X' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 转账是否验证预留银行账户
pub const THOST_FTDC_TPID_TranferChkProperty: u8 = 'i' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 非交易时间异常报单校验参数
pub const THOST_FTDC_TPID_TradeChkPhase: u8 = 'j' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 其他异常报单校验参数（价格和手数）
pub const THOST_FTDC_TPID_TradeChkPriceVol: u8 = 'k' as u8;
/// TFtdcTradeParamIDType是一个交易系统参数代码类型
///
/// 卖出垂直价差组合新算法
pub const THOST_FTDC_TPID_NewBESMarginAlgo: u8 = 'l' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 资金数据
pub const THOST_FTDC_FI_SettlementFund: u8 = 'F' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 成交数据
pub const THOST_FTDC_FI_Trade: u8 = 'T' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 投资者持仓数据
pub const THOST_FTDC_FI_InvestorPosition: u8 = 'P' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 投资者分项资金数据
pub const THOST_FTDC_FI_SubEntryFund: u8 = 'O' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 组合持仓数据
pub const THOST_FTDC_FI_CZCECombinationPos: u8 = 'C' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 上报保证金监控中心数据
pub const THOST_FTDC_FI_CSRCData: u8 = 'R' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 郑商所平仓了结数据
pub const THOST_FTDC_FI_CZCEClose: u8 = 'L' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 郑商所非平仓了结数据
pub const THOST_FTDC_FI_CZCENoClose: u8 = 'N' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 持仓明细数据
pub const THOST_FTDC_FI_PositionDtl: u8 = 'D' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 期权执行文件
pub const THOST_FTDC_FI_OptionStrike: u8 = 'S' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 结算价比对文件
pub const THOST_FTDC_FI_SettlementPriceComparison: u8 = 'M' as u8;
/// TFtdcFileIDType是一个文件标识类型
///
/// 上期所非持仓变动明细
pub const THOST_FTDC_FI_NonTradePosChange: u8 = 'B' as u8;
/// TFtdcFileTypeType是一个文件上传类型类型
///
/// 结算
pub const THOST_FTDC_FUT_Settlement: u8 = '0' as u8;
/// TFtdcFileTypeType是一个文件上传类型类型
///
/// 核对
pub const THOST_FTDC_FUT_Check: u8 = '1' as u8;
/// TFtdcFileFormatType是一个文件格式类型
///
/// 文本文件(.txt)
pub const THOST_FTDC_FFT_Txt: u8 = '0' as u8;
/// TFtdcFileFormatType是一个文件格式类型
///
/// 压缩文件(.zip)
pub const THOST_FTDC_FFT_Zip: u8 = '1' as u8;
/// TFtdcFileFormatType是一个文件格式类型
///
/// DBF文件(.dbf)
pub const THOST_FTDC_FFT_DBF: u8 = '2' as u8;
/// TFtdcFileUploadStatusType是一个文件状态类型
///
/// 上传成功
pub const THOST_FTDC_FUS_SucceedUpload: u8 = '1' as u8;
/// TFtdcFileUploadStatusType是一个文件状态类型
///
/// 上传失败
pub const THOST_FTDC_FUS_FailedUpload: u8 = '2' as u8;
/// TFtdcFileUploadStatusType是一个文件状态类型
///
/// 导入成功
pub const THOST_FTDC_FUS_SucceedLoad: u8 = '3' as u8;
/// TFtdcFileUploadStatusType是一个文件状态类型
///
/// 导入部分成功
pub const THOST_FTDC_FUS_PartSucceedLoad: u8 = '4' as u8;
/// TFtdcFileUploadStatusType是一个文件状态类型
///
/// 导入失败
pub const THOST_FTDC_FUS_FailedLoad: u8 = '5' as u8;
/// TFtdcTransferDirectionType是一个移仓方向类型
///
/// 移出
pub const THOST_FTDC_TD_Out: u8 = '0' as u8;
/// TFtdcTransferDirectionType是一个移仓方向类型
///
/// 移入
pub const THOST_FTDC_TD_In: u8 = '1' as u8;
/// TFtdcSpecialCreateRuleType是一个特殊的创建规则类型
///
/// 没有特殊创建规则
pub const THOST_FTDC_SC_NoSpecialRule: u8 = '0' as u8;
/// TFtdcSpecialCreateRuleType是一个特殊的创建规则类型
///
/// 不包含春节
pub const THOST_FTDC_SC_NoSpringFestival: u8 = '1' as u8;
/// TFtdcBasisPriceTypeType是一个挂牌基准价类型类型
///
/// 上一合约结算价
pub const THOST_FTDC_IPT_LastSettlement: u8 = '1' as u8;
/// TFtdcBasisPriceTypeType是一个挂牌基准价类型类型
///
/// 上一合约收盘价
pub const THOST_FTDC_IPT_LaseClose: u8 = '2' as u8;
/// TFtdcProductLifePhaseType是一个产品生命周期状态类型
///
/// 活跃
pub const THOST_FTDC_PLP_Active: u8 = '1' as u8;
/// TFtdcProductLifePhaseType是一个产品生命周期状态类型
///
/// 不活跃
pub const THOST_FTDC_PLP_NonActive: u8 = '2' as u8;
/// TFtdcProductLifePhaseType是一个产品生命周期状态类型
///
/// 注销
pub const THOST_FTDC_PLP_Canceled: u8 = '3' as u8;
/// TFtdcDeliveryModeType是一个交割方式类型
///
/// 现金交割
pub const THOST_FTDC_DM_CashDeliv: u8 = '1' as u8;
/// TFtdcDeliveryModeType是一个交割方式类型
///
/// 实物交割
pub const THOST_FTDC_DM_CommodityDeliv: u8 = '2' as u8;
/// TFtdcFundIOTypeType是一个出入金类型类型
///
/// 出入金
pub const THOST_FTDC_FIOT_FundIO: u8 = '1' as u8;
/// TFtdcFundIOTypeType是一个出入金类型类型
///
/// 银期转帐
pub const THOST_FTDC_FIOT_Transfer: u8 = '2' as u8;
/// TFtdcFundIOTypeType是一个出入金类型类型
///
/// 银期换汇
pub const THOST_FTDC_FIOT_SwapCurrency: u8 = '3' as u8;
/// TFtdcFundTypeType是一个资金类型类型
///
/// 银行存款
pub const THOST_FTDC_FT_Deposite: u8 = '1' as u8;
/// TFtdcFundTypeType是一个资金类型类型
///
/// 分项资金
pub const THOST_FTDC_FT_ItemFund: u8 = '2' as u8;
/// TFtdcFundTypeType是一个资金类型类型
///
/// 公司调整
pub const THOST_FTDC_FT_Company: u8 = '3' as u8;
/// TFtdcFundTypeType是一个资金类型类型
///
/// 资金内转
pub const THOST_FTDC_FT_InnerTransfer: u8 = '4' as u8;
/// TFtdcFundDirectionType是一个出入金方向类型
///
/// 入金
pub const THOST_FTDC_FD_In: u8 = '1' as u8;
/// TFtdcFundDirectionType是一个出入金方向类型
///
/// 出金
pub const THOST_FTDC_FD_Out: u8 = '2' as u8;
/// TFtdcFundStatusType是一个资金状态类型
///
/// 已录入
pub const THOST_FTDC_FS_Record: u8 = '1' as u8;
/// TFtdcFundStatusType是一个资金状态类型
///
/// 已复核
pub const THOST_FTDC_FS_Check: u8 = '2' as u8;
/// TFtdcFundStatusType是一个资金状态类型
///
/// 已冲销
pub const THOST_FTDC_FS_Charge: u8 = '3' as u8;
/// TFtdcPublishStatusType是一个发布状态类型
///
/// 未发布
pub const THOST_FTDC_PS_None: u8 = '1' as u8;
/// TFtdcPublishStatusType是一个发布状态类型
///
/// 正在发布
pub const THOST_FTDC_PS_Publishing: u8 = '2' as u8;
/// TFtdcPublishStatusType是一个发布状态类型
///
/// 已发布
pub const THOST_FTDC_PS_Published: u8 = '3' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 不活跃
pub const THOST_FTDC_ES_NonActive: u8 = '1' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 启动
pub const THOST_FTDC_ES_Startup: u8 = '2' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 交易开始初始化
pub const THOST_FTDC_ES_Initialize: u8 = '3' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 交易完成初始化
pub const THOST_FTDC_ES_Initialized: u8 = '4' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 收市开始
pub const THOST_FTDC_ES_Close: u8 = '5' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 收市完成
pub const THOST_FTDC_ES_Closed: u8 = '6' as u8;
/// TFtdcSystemStatusType是一个系统状态类型
///
/// 结算
pub const THOST_FTDC_ES_Settlement: u8 = '7' as u8;
/// TFtdcSettlementStatusType是一个结算状态类型
///
/// 初始
pub const THOST_FTDC_STS_Initialize: u8 = '0' as u8;
/// TFtdcSettlementStatusType是一个结算状态类型
///
/// 结算中
pub const THOST_FTDC_STS_Settlementing: u8 = '1' as u8;
/// TFtdcSettlementStatusType是一个结算状态类型
///
/// 已结算
pub const THOST_FTDC_STS_Settlemented: u8 = '2' as u8;
/// TFtdcSettlementStatusType是一个结算状态类型
///
/// 结算完成
pub const THOST_FTDC_STS_Finished: u8 = '3' as u8;
/// TFtdcInvestorTypeType是一个投资者类型类型
///
/// 自然人
pub const THOST_FTDC_CT_Person: u8 = '0' as u8;
/// TFtdcInvestorTypeType是一个投资者类型类型
///
/// 法人
pub const THOST_FTDC_CT_Company: u8 = '1' as u8;
/// TFtdcInvestorTypeType是一个投资者类型类型
///
/// 投资基金
pub const THOST_FTDC_CT_Fund: u8 = '2' as u8;
/// TFtdcInvestorTypeType是一个投资者类型类型
///
/// 特殊法人
pub const THOST_FTDC_CT_SpecialOrgan: u8 = '3' as u8;
/// TFtdcInvestorTypeType是一个投资者类型类型
///
/// 资管户
pub const THOST_FTDC_CT_Asset: u8 = '4' as u8;
/// TFtdcBrokerTypeType是一个经纪公司类型类型
///
/// 交易会员
pub const THOST_FTDC_BT_Trade: u8 = '0' as u8;
/// TFtdcBrokerTypeType是一个经纪公司类型类型
///
/// 交易结算会员
pub const THOST_FTDC_BT_TradeSettle: u8 = '1' as u8;
/// TFtdcRiskLevelType是一个风险等级类型
///
/// 低风险客户
pub const THOST_FTDC_FAS_Low: u8 = '1' as u8;
/// TFtdcRiskLevelType是一个风险等级类型
///
/// 普通客户
pub const THOST_FTDC_FAS_Normal: u8 = '2' as u8;
/// TFtdcRiskLevelType是一个风险等级类型
///
/// 关注客户
pub const THOST_FTDC_FAS_Focus: u8 = '3' as u8;
/// TFtdcRiskLevelType是一个风险等级类型
///
/// 风险客户
pub const THOST_FTDC_FAS_Risk: u8 = '4' as u8;
/// TFtdcFeeAcceptStyleType是一个手续费收取方式类型
///
/// 按交易收取
pub const THOST_FTDC_FAS_ByTrade: u8 = '1' as u8;
/// TFtdcFeeAcceptStyleType是一个手续费收取方式类型
///
/// 按交割收取
pub const THOST_FTDC_FAS_ByDeliv: u8 = '2' as u8;
/// TFtdcFeeAcceptStyleType是一个手续费收取方式类型
///
/// 不收
pub const THOST_FTDC_FAS_None: u8 = '3' as u8;
/// TFtdcFeeAcceptStyleType是一个手续费收取方式类型
///
/// 按指定手续费收取
pub const THOST_FTDC_FAS_FixFee: u8 = '4' as u8;
/// TFtdcPasswordTypeType是一个密码类型类型
///
/// 交易密码
pub const THOST_FTDC_PWDT_Trade: u8 = '1' as u8;
/// TFtdcPasswordTypeType是一个密码类型类型
///
/// 资金密码
pub const THOST_FTDC_PWDT_Account: u8 = '2' as u8;
/// TFtdcAlgorithmType是一个盈亏算法类型
///
/// 浮盈浮亏都计算
pub const THOST_FTDC_AG_All: u8 = '1' as u8;
/// TFtdcAlgorithmType是一个盈亏算法类型
///
/// 浮盈不计，浮亏计
pub const THOST_FTDC_AG_OnlyLost: u8 = '2' as u8;
/// TFtdcAlgorithmType是一个盈亏算法类型
///
/// 浮盈计，浮亏不计
pub const THOST_FTDC_AG_OnlyGain: u8 = '3' as u8;
/// TFtdcAlgorithmType是一个盈亏算法类型
///
/// 浮盈浮亏都不计算
pub const THOST_FTDC_AG_None: u8 = '4' as u8;
/// TFtdcIncludeCloseProfitType是一个是否包含平仓盈利类型
///
/// 包含平仓盈利
pub const THOST_FTDC_ICP_Include: u8 = '0' as u8;
/// TFtdcIncludeCloseProfitType是一个是否包含平仓盈利类型
///
/// 不包含平仓盈利
pub const THOST_FTDC_ICP_NotInclude: u8 = '2' as u8;
/// TFtdcAllWithoutTradeType是一个是否受可提比例限制类型
///
/// 无仓无成交不受可提比例限制
pub const THOST_FTDC_AWT_Enable: u8 = '0' as u8;
/// TFtdcAllWithoutTradeType是一个是否受可提比例限制类型
///
/// 受可提比例限制
pub const THOST_FTDC_AWT_Disable: u8 = '2' as u8;
/// TFtdcAllWithoutTradeType是一个是否受可提比例限制类型
///
/// 无仓不受可提比例限制
pub const THOST_FTDC_AWT_NoHoldEnable: u8 = '3' as u8;
/// TFtdcFuturePwdFlagType是一个资金密码核对标志类型
///
/// 不核对
pub const THOST_FTDC_FPWD_UnCheck: u8 = '0' as u8;
/// TFtdcFuturePwdFlagType是一个资金密码核对标志类型
///
/// 核对
pub const THOST_FTDC_FPWD_Check: u8 = '1' as u8;
/// TFtdcTransferTypeType是一个银期转账类型类型
///
/// 银行转期货
pub const THOST_FTDC_TT_BankToFuture: u8 = '0' as u8;
/// TFtdcTransferTypeType是一个银期转账类型类型
///
/// 期货转银行
pub const THOST_FTDC_TT_FutureToBank: u8 = '1' as u8;
/// TFtdcTransferValidFlagType是一个转账有效标志类型
///
/// 无效或失败
pub const THOST_FTDC_TVF_Invalid: u8 = '0' as u8;
/// TFtdcTransferValidFlagType是一个转账有效标志类型
///
/// 有效
pub const THOST_FTDC_TVF_Valid: u8 = '1' as u8;
/// TFtdcTransferValidFlagType是一个转账有效标志类型
///
/// 冲正
pub const THOST_FTDC_TVF_Reverse: u8 = '2' as u8;
/// TFtdcReasonType是一个事由类型
///
/// 错单
pub const THOST_FTDC_RN_CD: u8 = '0' as u8;
/// TFtdcReasonType是一个事由类型
///
/// 资金在途
pub const THOST_FTDC_RN_ZT: u8 = '1' as u8;
/// TFtdcReasonType是一个事由类型
///
/// 其它
pub const THOST_FTDC_RN_QT: u8 = '2' as u8;
/// TFtdcSexType是一个性别类型
///
/// 未知
pub const THOST_FTDC_SEX_None: u8 = '0' as u8;
/// TFtdcSexType是一个性别类型
///
/// 男
pub const THOST_FTDC_SEX_Man: u8 = '1' as u8;
/// TFtdcSexType是一个性别类型
///
/// 女
pub const THOST_FTDC_SEX_Woman: u8 = '2' as u8;
/// TFtdcUserTypeType是一个用户类型类型
///
/// 投资者
pub const THOST_FTDC_UT_Investor: u8 = '0' as u8;
/// TFtdcUserTypeType是一个用户类型类型
///
/// 操作员
pub const THOST_FTDC_UT_Operator: u8 = '1' as u8;
/// TFtdcUserTypeType是一个用户类型类型
///
/// 管理员
pub const THOST_FTDC_UT_SuperUser: u8 = '2' as u8;
/// TFtdcRateTypeType是一个费率类型类型
///
/// 保证金率
pub const THOST_FTDC_RATETYPE_MarginRate: u8 = '2' as u8;
/// TFtdcNoteTypeType是一个通知类型类型
///
/// 交易结算单
pub const THOST_FTDC_NOTETYPE_TradeSettleBill: u8 = '1' as u8;
/// TFtdcNoteTypeType是一个通知类型类型
///
/// 交易结算月报
pub const THOST_FTDC_NOTETYPE_TradeSettleMonth: u8 = '2' as u8;
/// TFtdcNoteTypeType是一个通知类型类型
///
/// 追加保证金通知书
pub const THOST_FTDC_NOTETYPE_CallMarginNotes: u8 = '3' as u8;
/// TFtdcNoteTypeType是一个通知类型类型
///
/// 强行平仓通知书
pub const THOST_FTDC_NOTETYPE_ForceCloseNotes: u8 = '4' as u8;
/// TFtdcNoteTypeType是一个通知类型类型
///
/// 成交通知书
pub const THOST_FTDC_NOTETYPE_TradeNotes: u8 = '5' as u8;
/// TFtdcNoteTypeType是一个通知类型类型
///
/// 交割通知书
pub const THOST_FTDC_NOTETYPE_DelivNotes: u8 = '6' as u8;
/// TFtdcSettlementStyleType是一个结算单方式类型
///
/// 逐日盯市
pub const THOST_FTDC_SBS_Day: u8 = '1' as u8;
/// TFtdcSettlementStyleType是一个结算单方式类型
///
/// 逐笔对冲
pub const THOST_FTDC_SBS_Volume: u8 = '2' as u8;
/// TFtdcSettlementBillTypeType是一个结算单类型类型
///
/// 日报
pub const THOST_FTDC_ST_Day: u8 = '0' as u8;
/// TFtdcSettlementBillTypeType是一个结算单类型类型
///
/// 月报
pub const THOST_FTDC_ST_Month: u8 = '1' as u8;
/// TFtdcUserRightTypeType是一个客户权限类型类型
///
/// 登录
pub const THOST_FTDC_URT_Logon: u8 = '1' as u8;
/// TFtdcUserRightTypeType是一个客户权限类型类型
///
/// 银期转帐
pub const THOST_FTDC_URT_Transfer: u8 = '2' as u8;
/// TFtdcUserRightTypeType是一个客户权限类型类型
///
/// 邮寄结算单
pub const THOST_FTDC_URT_EMail: u8 = '3' as u8;
/// TFtdcUserRightTypeType是一个客户权限类型类型
///
/// 传真结算单
pub const THOST_FTDC_URT_Fax: u8 = '4' as u8;
/// TFtdcUserRightTypeType是一个客户权限类型类型
///
/// 条件单
pub const THOST_FTDC_URT_ConditionOrder: u8 = '5' as u8;
/// TFtdcMarginPriceTypeType是一个保证金价格类型类型
///
/// 昨结算价
pub const THOST_FTDC_MPT_PreSettlementPrice: u8 = '1' as u8;
/// TFtdcMarginPriceTypeType是一个保证金价格类型类型
///
/// 最新价
pub const THOST_FTDC_MPT_SettlementPrice: u8 = '2' as u8;
/// TFtdcMarginPriceTypeType是一个保证金价格类型类型
///
/// 成交均价
pub const THOST_FTDC_MPT_AveragePrice: u8 = '3' as u8;
/// TFtdcMarginPriceTypeType是一个保证金价格类型类型
///
/// 开仓价
pub const THOST_FTDC_MPT_OpenPrice: u8 = '4' as u8;
/// TFtdcBillGenStatusType是一个结算单生成状态类型
///
/// 未生成
pub const THOST_FTDC_BGS_None: u8 = '0' as u8;
/// TFtdcBillGenStatusType是一个结算单生成状态类型
///
/// 生成中
pub const THOST_FTDC_BGS_NoGenerated: u8 = '1' as u8;
/// TFtdcBillGenStatusType是一个结算单生成状态类型
///
/// 已生成
pub const THOST_FTDC_BGS_Generated: u8 = '2' as u8;
/// TFtdcAlgoTypeType是一个算法类型类型
///
/// 持仓处理算法
pub const THOST_FTDC_AT_HandlePositionAlgo: u8 = '1' as u8;
/// TFtdcAlgoTypeType是一个算法类型类型
///
/// 寻找保证金率算法
pub const THOST_FTDC_AT_FindMarginRateAlgo: u8 = '2' as u8;
/// TFtdcHandlePositionAlgoIDType是一个持仓处理算法编号类型
///
/// 基本
pub const THOST_FTDC_HPA_Base: u8 = '1' as u8;
/// TFtdcHandlePositionAlgoIDType是一个持仓处理算法编号类型
///
/// 大连商品交易所
pub const THOST_FTDC_HPA_DCE: u8 = '2' as u8;
/// TFtdcHandlePositionAlgoIDType是一个持仓处理算法编号类型
///
/// 郑州商品交易所
pub const THOST_FTDC_HPA_CZCE: u8 = '3' as u8;
/// TFtdcFindMarginRateAlgoIDType是一个寻找保证金率算法编号类型
///
/// 基本
pub const THOST_FTDC_FMRA_Base: u8 = '1' as u8;
/// TFtdcFindMarginRateAlgoIDType是一个寻找保证金率算法编号类型
///
/// 大连商品交易所
pub const THOST_FTDC_FMRA_DCE: u8 = '2' as u8;
/// TFtdcFindMarginRateAlgoIDType是一个寻找保证金率算法编号类型
///
/// 郑州商品交易所
pub const THOST_FTDC_FMRA_CZCE: u8 = '3' as u8;
/// TFtdcHandleTradingAccountAlgoIDType是一个资金处理算法编号类型
///
/// 基本
pub const THOST_FTDC_HTAA_Base: u8 = '1' as u8;
/// TFtdcHandleTradingAccountAlgoIDType是一个资金处理算法编号类型
///
/// 大连商品交易所
pub const THOST_FTDC_HTAA_DCE: u8 = '2' as u8;
/// TFtdcHandleTradingAccountAlgoIDType是一个资金处理算法编号类型
///
/// 郑州商品交易所
pub const THOST_FTDC_HTAA_CZCE: u8 = '3' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 指定下单人
pub const THOST_FTDC_PST_Order: u8 = '1' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 开户授权人
pub const THOST_FTDC_PST_Open: u8 = '2' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 资金调拨人
pub const THOST_FTDC_PST_Fund: u8 = '3' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 结算单确认人
pub const THOST_FTDC_PST_Settlement: u8 = '4' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 法人
pub const THOST_FTDC_PST_Company: u8 = '5' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 法人代表
pub const THOST_FTDC_PST_Corporation: u8 = '6' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 投资者联系人
pub const THOST_FTDC_PST_LinkMan: u8 = '7' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 分户管理资产负责人
pub const THOST_FTDC_PST_Ledger: u8 = '8' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 托（保）管人
pub const THOST_FTDC_PST_Trustee: u8 = '9' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 托（保）管机构法人代表
pub const THOST_FTDC_PST_TrusteeCorporation: u8 = 'A' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 托（保）管机构开户授权人
pub const THOST_FTDC_PST_TrusteeOpen: u8 = 'B' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 托（保）管机构联系人
pub const THOST_FTDC_PST_TrusteeContact: u8 = 'C' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 境外自然人参考证件
pub const THOST_FTDC_PST_ForeignerRefer: u8 = 'D' as u8;
/// TFtdcPersonTypeType是一个联系人类型类型
///
/// 法人代表参考证件
pub const THOST_FTDC_PST_CorporationRefer: u8 = 'E' as u8;
/// TFtdcQueryInvestorRangeType是一个查询范围类型
///
/// 所有
pub const THOST_FTDC_QIR_All: u8 = '1' as u8;
/// TFtdcQueryInvestorRangeType是一个查询范围类型
///
/// 查询分类
pub const THOST_FTDC_QIR_Group: u8 = '2' as u8;
/// TFtdcQueryInvestorRangeType是一个查询范围类型
///
/// 单一投资者
pub const THOST_FTDC_QIR_Single: u8 = '3' as u8;
/// TFtdcInvestorRiskStatusType是一个投资者风险状态类型
///
/// 正常
pub const THOST_FTDC_IRS_Normal: u8 = '1' as u8;
/// TFtdcInvestorRiskStatusType是一个投资者风险状态类型
///
/// 警告
pub const THOST_FTDC_IRS_Warn: u8 = '2' as u8;
/// TFtdcInvestorRiskStatusType是一个投资者风险状态类型
///
/// 追保
pub const THOST_FTDC_IRS_Call: u8 = '3' as u8;
/// TFtdcInvestorRiskStatusType是一个投资者风险状态类型
///
/// 强平
pub const THOST_FTDC_IRS_Force: u8 = '4' as u8;
/// TFtdcInvestorRiskStatusType是一个投资者风险状态类型
///
/// 异常
pub const THOST_FTDC_IRS_Exception: u8 = '5' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 登录
pub const THOST_FTDC_UET_Login: u8 = '1' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 登出
pub const THOST_FTDC_UET_Logout: u8 = '2' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// CTP校验通过
pub const THOST_FTDC_UET_Trading: u8 = '3' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// CTP校验失败
pub const THOST_FTDC_UET_TradingError: u8 = '4' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 修改密码
pub const THOST_FTDC_UET_UpdatePassword: u8 = '5' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 客户端认证
pub const THOST_FTDC_UET_Authenticate: u8 = '6' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 终端信息上报
pub const THOST_FTDC_UET_SubmitSysInfo: u8 = '7' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 转账
pub const THOST_FTDC_UET_Transfer: u8 = '8' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 其他
pub const THOST_FTDC_UET_Other: u8 = '9' as u8;
/// TFtdcUserEventTypeType是一个用户事件类型类型
///
/// 修改资金密码
pub const THOST_FTDC_UET_UpdateTradingAccountPassword: u8 = 'a' as u8;
/// TFtdcCloseStyleType是一个平仓方式类型
///
/// 先开先平
pub const THOST_FTDC_ICS_Close: u8 = '0' as u8;
/// TFtdcCloseStyleType是一个平仓方式类型
///
/// 先平今再平昨
pub const THOST_FTDC_ICS_CloseToday: u8 = '1' as u8;
/// TFtdcStatModeType是一个统计方式类型
///
/// ----
pub const THOST_FTDC_SM_Non: u8 = '0' as u8;
/// TFtdcStatModeType是一个统计方式类型
///
/// 按合约统计
pub const THOST_FTDC_SM_Instrument: u8 = '1' as u8;
/// TFtdcStatModeType是一个统计方式类型
///
/// 按产品统计
pub const THOST_FTDC_SM_Product: u8 = '2' as u8;
/// TFtdcStatModeType是一个统计方式类型
///
/// 按投资者统计
pub const THOST_FTDC_SM_Investor: u8 = '3' as u8;
/// TFtdcParkedOrderStatusType是一个预埋单状态类型
///
/// 未发送
pub const THOST_FTDC_PAOS_NotSend: u8 = '1' as u8;
/// TFtdcParkedOrderStatusType是一个预埋单状态类型
///
/// 已发送
pub const THOST_FTDC_PAOS_Send: u8 = '2' as u8;
/// TFtdcParkedOrderStatusType是一个预埋单状态类型
///
/// 已删除
pub const THOST_FTDC_PAOS_Deleted: u8 = '3' as u8;
/// TFtdcVirDealStatusType是一个处理状态类型
///
/// 正在处理
pub const THOST_FTDC_VDS_Dealing: u8 = '1' as u8;
/// TFtdcVirDealStatusType是一个处理状态类型
///
/// 处理成功
pub const THOST_FTDC_VDS_DeaclSucceed: u8 = '2' as u8;
/// TFtdcOrgSystemIDType是一个原有系统代码类型
///
/// 综合交易平台
pub const THOST_FTDC_ORGS_Standard: u8 = '0' as u8;
/// TFtdcOrgSystemIDType是一个原有系统代码类型
///
/// 易盛系统
pub const THOST_FTDC_ORGS_ESunny: u8 = '1' as u8;
/// TFtdcOrgSystemIDType是一个原有系统代码类型
///
/// 金仕达V6系统
pub const THOST_FTDC_ORGS_KingStarV6: u8 = '2' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 正常处理中
pub const THOST_FTDC_VTS_NaturalDeal: u8 = '0' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 成功结束
pub const THOST_FTDC_VTS_SucceedEnd: u8 = '1' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 失败结束
pub const THOST_FTDC_VTS_FailedEND: u8 = '2' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 异常中
pub const THOST_FTDC_VTS_Exception: u8 = '3' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 已人工异常处理
pub const THOST_FTDC_VTS_ManualDeal: u8 = '4' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 通讯异常 ，请人工处理
pub const THOST_FTDC_VTS_MesException: u8 = '5' as u8;
/// TFtdcVirTradeStatusType是一个交易状态类型
///
/// 系统出错，请人工处理
pub const THOST_FTDC_VTS_SysException: u8 = '6' as u8;
/// TFtdcVirBankAccTypeType是一个银行帐户类型类型
///
/// 存折
pub const THOST_FTDC_VBAT_BankBook: u8 = '1' as u8;
/// TFtdcVirBankAccTypeType是一个银行帐户类型类型
///
/// 储蓄卡
pub const THOST_FTDC_VBAT_BankCard: u8 = '2' as u8;
/// TFtdcVirBankAccTypeType是一个银行帐户类型类型
///
/// 信用卡
pub const THOST_FTDC_VBAT_CreditCard: u8 = '3' as u8;
/// TFtdcVirementStatusType是一个银行帐户类型类型
///
/// 正常
pub const THOST_FTDC_VMS_Natural: u8 = '0' as u8;
/// TFtdcVirementStatusType是一个银行帐户类型类型
///
/// 销户
pub const THOST_FTDC_VMS_Canceled: u8 = '9' as u8;
/// TFtdcVirementAvailAbilityType是一个有效标志类型
///
/// 未确认
pub const THOST_FTDC_VAA_NoAvailAbility: u8 = '0' as u8;
/// TFtdcVirementAvailAbilityType是一个有效标志类型
///
/// 有效
pub const THOST_FTDC_VAA_AvailAbility: u8 = '1' as u8;
/// TFtdcVirementAvailAbilityType是一个有效标志类型
///
/// 冲正
pub const THOST_FTDC_VAA_Repeal: u8 = '2' as u8;
/// TFtdcAMLGenStatusType是一个Aml生成方式类型
///
/// 程序生成
pub const THOST_FTDC_GEN_Program: u8 = '0' as u8;
/// TFtdcAMLGenStatusType是一个Aml生成方式类型
///
/// 人工生成
pub const THOST_FTDC_GEN_HandWork: u8 = '1' as u8;
/// TFtdcCFMMCKeyKindType是一个动态密钥类别(保证金监管)类型
///
/// 主动请求更新
pub const THOST_FTDC_CFMMCKK_REQUEST: u8 = 'R' as u8;
/// TFtdcCFMMCKeyKindType是一个动态密钥类别(保证金监管)类型
///
/// CFMMC自动更新
pub const THOST_FTDC_CFMMCKK_AUTO: u8 = 'A' as u8;
/// TFtdcCFMMCKeyKindType是一个动态密钥类别(保证金监管)类型
///
/// CFMMC手动更新
pub const THOST_FTDC_CFMMCKK_MANUAL: u8 = 'M' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 身份证
pub const THOST_FTDC_CFT_IDCard: u8 = '0' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 护照
pub const THOST_FTDC_CFT_Passport: u8 = '1' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 军官证
pub const THOST_FTDC_CFT_OfficerIDCard: u8 = '2' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 士兵证
pub const THOST_FTDC_CFT_SoldierIDCard: u8 = '3' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 回乡证
pub const THOST_FTDC_CFT_HomeComingCard: u8 = '4' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 户口簿
pub const THOST_FTDC_CFT_HouseholdRegister: u8 = '5' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 营业执照号
pub const THOST_FTDC_CFT_LicenseNo: u8 = '6' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 组织机构代码证
pub const THOST_FTDC_CFT_InstitutionCodeCard: u8 = '7' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 临时营业执照号
pub const THOST_FTDC_CFT_TempLicenseNo: u8 = '8' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 民办非企业登记证书
pub const THOST_FTDC_CFT_NoEnterpriseLicenseNo: u8 = '9' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 其他证件
pub const THOST_FTDC_CFT_OtherCard: u8 = 'x' as u8;
/// TFtdcCertificationTypeType是一个证件类型类型
///
/// 主管部门批文
pub const THOST_FTDC_CFT_SuperDepAgree: u8 = 'a' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 其他
pub const THOST_FTDC_FBC_Others: u8 = '0' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 转账交易明细对账
pub const THOST_FTDC_FBC_TransferDetails: u8 = '1' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 客户账户状态对账
pub const THOST_FTDC_FBC_CustAccStatus: u8 = '2' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 账户类交易明细对账
pub const THOST_FTDC_FBC_AccountTradeDetails: u8 = '3' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 期货账户信息变更明细对账
pub const THOST_FTDC_FBC_FutureAccountChangeInfoDetails: u8 = '4' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 客户资金台账余额明细对账
pub const THOST_FTDC_FBC_CustMoneyDetail: u8 = '5' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 客户销户结息明细对账
pub const THOST_FTDC_FBC_CustCancelAccountInfo: u8 = '6' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 客户资金余额对账结果
pub const THOST_FTDC_FBC_CustMoneyResult: u8 = '7' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 其它对账异常结果文件
pub const THOST_FTDC_FBC_OthersExceptionResult: u8 = '8' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 客户结息净额明细
pub const THOST_FTDC_FBC_CustInterestNetMoneyDetails: u8 = '9' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 客户资金交收明细
pub const THOST_FTDC_FBC_CustMoneySendAndReceiveDetails: u8 = 'a' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 法人存管银行资金交收汇总
pub const THOST_FTDC_FBC_CorporationMoneyTotal: u8 = 'b' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 主体间资金交收汇总
pub const THOST_FTDC_FBC_MainbodyMoneyTotal: u8 = 'c' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 总分平衡监管数据
pub const THOST_FTDC_FBC_MainPartMonitorData: u8 = 'd' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 存管银行备付金余额
pub const THOST_FTDC_FBC_PreparationMoney: u8 = 'e' as u8;
/// TFtdcFileBusinessCodeType是一个文件业务功能类型
///
/// 协办存管银行资金监管数据
pub const THOST_FTDC_FBC_BankMoneyMonitorData: u8 = 'f' as u8;
/// TFtdcCashExchangeCodeType是一个汇钞标志类型
///
/// 汇
pub const THOST_FTDC_CEC_Exchange: u8 = '1' as u8;
/// TFtdcCashExchangeCodeType是一个汇钞标志类型
///
/// 钞
pub const THOST_FTDC_CEC_Cash: u8 = '2' as u8;
/// TFtdcYesNoIndicatorType是一个是或否标识类型
///
/// 是
pub const THOST_FTDC_YNI_Yes: u8 = '0' as u8;
/// TFtdcYesNoIndicatorType是一个是或否标识类型
///
/// 否
pub const THOST_FTDC_YNI_No: u8 = '1' as u8;
/// TFtdcBanlanceTypeType是一个余额类型类型
///
/// 当前余额
pub const THOST_FTDC_BLT_CurrentMoney: u8 = '0' as u8;
/// TFtdcBanlanceTypeType是一个余额类型类型
///
/// 可用余额
pub const THOST_FTDC_BLT_UsableMoney: u8 = '1' as u8;
/// TFtdcBanlanceTypeType是一个余额类型类型
///
/// 可取余额
pub const THOST_FTDC_BLT_FetchableMoney: u8 = '2' as u8;
/// TFtdcBanlanceTypeType是一个余额类型类型
///
/// 冻结余额
pub const THOST_FTDC_BLT_FreezeMoney: u8 = '3' as u8;
/// TFtdcGenderType是一个性别类型
///
/// 未知状态
pub const THOST_FTDC_GD_Unknown: u8 = '0' as u8;
/// TFtdcGenderType是一个性别类型
///
/// 男
pub const THOST_FTDC_GD_Male: u8 = '1' as u8;
/// TFtdcGenderType是一个性别类型
///
/// 女
pub const THOST_FTDC_GD_Female: u8 = '2' as u8;
/// TFtdcFeePayFlagType是一个费用支付标志类型
///
/// 由受益方支付费用
pub const THOST_FTDC_FPF_BEN: u8 = '0' as u8;
/// TFtdcFeePayFlagType是一个费用支付标志类型
///
/// 由发送方支付费用
pub const THOST_FTDC_FPF_OUR: u8 = '1' as u8;
/// TFtdcFeePayFlagType是一个费用支付标志类型
///
/// 由发送方支付发起的费用，受益方支付接受的费用
pub const THOST_FTDC_FPF_SHA: u8 = '2' as u8;
/// TFtdcPassWordKeyTypeType是一个密钥类型类型
///
/// 交换密钥
pub const THOST_FTDC_PWKT_ExchangeKey: u8 = '0' as u8;
/// TFtdcPassWordKeyTypeType是一个密钥类型类型
///
/// 密码密钥
pub const THOST_FTDC_PWKT_PassWordKey: u8 = '1' as u8;
/// TFtdcPassWordKeyTypeType是一个密钥类型类型
///
/// MAC密钥
pub const THOST_FTDC_PWKT_MACKey: u8 = '2' as u8;
/// TFtdcPassWordKeyTypeType是一个密钥类型类型
///
/// 报文密钥
pub const THOST_FTDC_PWKT_MessageKey: u8 = '3' as u8;
/// TFtdcFBTPassWordTypeType是一个密码类型类型
///
/// 查询
pub const THOST_FTDC_PWT_Query: u8 = '0' as u8;
/// TFtdcFBTPassWordTypeType是一个密码类型类型
///
/// 取款
pub const THOST_FTDC_PWT_Fetch: u8 = '1' as u8;
/// TFtdcFBTPassWordTypeType是一个密码类型类型
///
/// 转帐
pub const THOST_FTDC_PWT_Transfer: u8 = '2' as u8;
/// TFtdcFBTPassWordTypeType是一个密码类型类型
///
/// 交易
pub const THOST_FTDC_PWT_Trade: u8 = '3' as u8;
/// TFtdcFBTEncryModeType是一个加密方式类型
///
/// 不加密
pub const THOST_FTDC_EM_NoEncry: u8 = '0' as u8;
/// TFtdcFBTEncryModeType是一个加密方式类型
///
/// DES
pub const THOST_FTDC_EM_DES: u8 = '1' as u8;
/// TFtdcFBTEncryModeType是一个加密方式类型
///
/// 3DES
pub const THOST_FTDC_EM_3DES: u8 = '2' as u8;
/// TFtdcBankRepealFlagType是一个银行冲正标志类型
///
/// 银行无需自动冲正
pub const THOST_FTDC_BRF_BankNotNeedRepeal: u8 = '0' as u8;
/// TFtdcBankRepealFlagType是一个银行冲正标志类型
///
/// 银行待自动冲正
pub const THOST_FTDC_BRF_BankWaitingRepeal: u8 = '1' as u8;
/// TFtdcBankRepealFlagType是一个银行冲正标志类型
///
/// 银行已自动冲正
pub const THOST_FTDC_BRF_BankBeenRepealed: u8 = '2' as u8;
/// TFtdcBrokerRepealFlagType是一个期商冲正标志类型
///
/// 期商无需自动冲正
pub const THOST_FTDC_BRORF_BrokerNotNeedRepeal: u8 = '0' as u8;
/// TFtdcBrokerRepealFlagType是一个期商冲正标志类型
///
/// 期商待自动冲正
pub const THOST_FTDC_BRORF_BrokerWaitingRepeal: u8 = '1' as u8;
/// TFtdcBrokerRepealFlagType是一个期商冲正标志类型
///
/// 期商已自动冲正
pub const THOST_FTDC_BRORF_BrokerBeenRepealed: u8 = '2' as u8;
/// TFtdcInstitutionTypeType是一个机构类别类型
///
/// 银行
pub const THOST_FTDC_TS_Bank: u8 = '0' as u8;
/// TFtdcInstitutionTypeType是一个机构类别类型
///
/// 期商
pub const THOST_FTDC_TS_Future: u8 = '1' as u8;
/// TFtdcInstitutionTypeType是一个机构类别类型
///
/// 券商
pub const THOST_FTDC_TS_Store: u8 = '2' as u8;
/// TFtdcLastFragmentType是一个最后分片标志类型
///
/// 是最后分片
pub const THOST_FTDC_LF_Yes: u8 = '0' as u8;
/// TFtdcLastFragmentType是一个最后分片标志类型
///
/// 不是最后分片
pub const THOST_FTDC_LF_No: u8 = '1' as u8;
/// TFtdcBankAccStatusType是一个银行账户状态类型
///
/// 正常
pub const THOST_FTDC_BAS_Normal: u8 = '0' as u8;
/// TFtdcBankAccStatusType是一个银行账户状态类型
///
/// 冻结
pub const THOST_FTDC_BAS_Freeze: u8 = '1' as u8;
/// TFtdcBankAccStatusType是一个银行账户状态类型
///
/// 挂失
pub const THOST_FTDC_BAS_ReportLoss: u8 = '2' as u8;
/// TFtdcMoneyAccountStatusType是一个资金账户状态类型
///
/// 正常
pub const THOST_FTDC_MAS_Normal: u8 = '0' as u8;
/// TFtdcMoneyAccountStatusType是一个资金账户状态类型
///
/// 销户
pub const THOST_FTDC_MAS_Cancel: u8 = '1' as u8;
/// TFtdcManageStatusType是一个存管状态类型
///
/// 指定存管
pub const THOST_FTDC_MSS_Point: u8 = '0' as u8;
/// TFtdcManageStatusType是一个存管状态类型
///
/// 预指定
pub const THOST_FTDC_MSS_PrePoint: u8 = '1' as u8;
/// TFtdcManageStatusType是一个存管状态类型
///
/// 撤销指定
pub const THOST_FTDC_MSS_CancelPoint: u8 = '2' as u8;
/// TFtdcSystemTypeType是一个应用系统类型类型
///
/// 银期转帐
pub const THOST_FTDC_SYT_FutureBankTransfer: u8 = '0' as u8;
/// TFtdcSystemTypeType是一个应用系统类型类型
///
/// 银证转帐
pub const THOST_FTDC_SYT_StockBankTransfer: u8 = '1' as u8;
/// TFtdcSystemTypeType是一个应用系统类型类型
///
/// 第三方存管
pub const THOST_FTDC_SYT_TheThirdPartStore: u8 = '2' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 正常处理中
pub const THOST_FTDC_TEF_NormalProcessing: u8 = '0' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 成功结束
pub const THOST_FTDC_TEF_Success: u8 = '1' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 失败结束
pub const THOST_FTDC_TEF_Failed: u8 = '2' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 异常中
pub const THOST_FTDC_TEF_Abnormal: u8 = '3' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 已人工异常处理
pub const THOST_FTDC_TEF_ManualProcessedForException: u8 = '4' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 通讯异常 ，请人工处理
pub const THOST_FTDC_TEF_CommuFailedNeedManualProcess: u8 = '5' as u8;
/// TFtdcTxnEndFlagType是一个银期转帐划转结果标志类型
///
/// 系统出错，请人工处理
pub const THOST_FTDC_TEF_SysErrorNeedManualProcess: u8 = '6' as u8;
/// TFtdcProcessStatusType是一个银期转帐服务处理状态类型
///
/// 未处理
pub const THOST_FTDC_PSS_NotProcess: u8 = '0' as u8;
/// TFtdcProcessStatusType是一个银期转帐服务处理状态类型
///
/// 开始处理
pub const THOST_FTDC_PSS_StartProcess: u8 = '1' as u8;
/// TFtdcProcessStatusType是一个银期转帐服务处理状态类型
///
/// 处理完成
pub const THOST_FTDC_PSS_Finished: u8 = '2' as u8;
/// TFtdcCustTypeType是一个客户类型类型
///
/// 自然人
pub const THOST_FTDC_CUSTT_Person: u8 = '0' as u8;
/// TFtdcCustTypeType是一个客户类型类型
///
/// 机构户
pub const THOST_FTDC_CUSTT_Institution: u8 = '1' as u8;
/// TFtdcFBTTransferDirectionType是一个银期转帐方向类型
///
/// 入金，银行转期货
pub const THOST_FTDC_FBTTD_FromBankToFuture: u8 = '1' as u8;
/// TFtdcFBTTransferDirectionType是一个银期转帐方向类型
///
/// 出金，期货转银行
pub const THOST_FTDC_FBTTD_FromFutureToBank: u8 = '2' as u8;
/// TFtdcOpenOrDestroyType是一个开销户类别类型
///
/// 开户
pub const THOST_FTDC_OOD_Open: u8 = '1' as u8;
/// TFtdcOpenOrDestroyType是一个开销户类别类型
///
/// 销户
pub const THOST_FTDC_OOD_Destroy: u8 = '0' as u8;
/// TFtdcAvailabilityFlagType是一个有效标志类型
///
/// 未确认
pub const THOST_FTDC_AVAF_Invalid: u8 = '0' as u8;
/// TFtdcAvailabilityFlagType是一个有效标志类型
///
/// 有效
pub const THOST_FTDC_AVAF_Valid: u8 = '1' as u8;
/// TFtdcAvailabilityFlagType是一个有效标志类型
///
/// 冲正
pub const THOST_FTDC_AVAF_Repeal: u8 = '2' as u8;
/// TFtdcOrganTypeType是一个机构类型类型
///
/// 银行代理
pub const THOST_FTDC_OT_Bank: u8 = '1' as u8;
/// TFtdcOrganTypeType是一个机构类型类型
///
/// 交易前置
pub const THOST_FTDC_OT_Future: u8 = '2' as u8;
/// TFtdcOrganTypeType是一个机构类型类型
///
/// 银期转帐平台管理
pub const THOST_FTDC_OT_PlateForm: u8 = '9' as u8;
/// TFtdcOrganLevelType是一个机构级别类型
///
/// 银行总行或期商总部
pub const THOST_FTDC_OL_HeadQuarters: u8 = '1' as u8;
/// TFtdcOrganLevelType是一个机构级别类型
///
/// 银行分中心或期货公司营业部
pub const THOST_FTDC_OL_Branch: u8 = '2' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 期商协议
pub const THOST_FTDC_PID_FutureProtocal: u8 = '0' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 工行协议
pub const THOST_FTDC_PID_ICBCProtocal: u8 = '1' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 农行协议
pub const THOST_FTDC_PID_ABCProtocal: u8 = '2' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 中国银行协议
pub const THOST_FTDC_PID_CBCProtocal: u8 = '3' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 建行协议
pub const THOST_FTDC_PID_CCBProtocal: u8 = '4' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 交行协议
pub const THOST_FTDC_PID_BOCOMProtocal: u8 = '5' as u8;
/// TFtdcProtocalIDType是一个协议类型类型
///
/// 银期转帐平台协议
pub const THOST_FTDC_PID_FBTPlateFormProtocal: u8 = 'X' as u8;
/// TFtdcConnectModeType是一个套接字连接方式类型
///
/// 短连接
pub const THOST_FTDC_CM_ShortConnect: u8 = '0' as u8;
/// TFtdcConnectModeType是一个套接字连接方式类型
///
/// 长连接
pub const THOST_FTDC_CM_LongConnect: u8 = '1' as u8;
/// TFtdcSyncModeType是一个套接字通信方式类型
///
/// 异步
pub const THOST_FTDC_SRM_ASync: u8 = '0' as u8;
/// TFtdcSyncModeType是一个套接字通信方式类型
///
/// 同步
pub const THOST_FTDC_SRM_Sync: u8 = '1' as u8;
/// TFtdcBankAccTypeType是一个银行帐号类型类型
///
/// 银行存折
pub const THOST_FTDC_BAT_BankBook: u8 = '1' as u8;
/// TFtdcBankAccTypeType是一个银行帐号类型类型
///
/// 储蓄卡
pub const THOST_FTDC_BAT_SavingCard: u8 = '2' as u8;
/// TFtdcBankAccTypeType是一个银行帐号类型类型
///
/// 信用卡
pub const THOST_FTDC_BAT_CreditCard: u8 = '3' as u8;
/// TFtdcFutureAccTypeType是一个期货公司帐号类型类型
///
/// 银行存折
pub const THOST_FTDC_FAT_BankBook: u8 = '1' as u8;
/// TFtdcFutureAccTypeType是一个期货公司帐号类型类型
///
/// 储蓄卡
pub const THOST_FTDC_FAT_SavingCard: u8 = '2' as u8;
/// TFtdcFutureAccTypeType是一个期货公司帐号类型类型
///
/// 信用卡
pub const THOST_FTDC_FAT_CreditCard: u8 = '3' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 启用
pub const THOST_FTDC_OS_Ready: u8 = '0' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 签到
pub const THOST_FTDC_OS_CheckIn: u8 = '1' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 签退
pub const THOST_FTDC_OS_CheckOut: u8 = '2' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 对帐文件到达
pub const THOST_FTDC_OS_CheckFileArrived: u8 = '3' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 对帐
pub const THOST_FTDC_OS_CheckDetail: u8 = '4' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 日终清理
pub const THOST_FTDC_OS_DayEndClean: u8 = '5' as u8;
/// TFtdcOrganStatusType是一个接入机构状态类型
///
/// 注销
pub const THOST_FTDC_OS_Invalid: u8 = '9' as u8;
/// TFtdcCCBFeeModeType是一个建行收费模式类型
///
/// 按金额扣收
pub const THOST_FTDC_CCBFM_ByAmount: u8 = '1' as u8;
/// TFtdcCCBFeeModeType是一个建行收费模式类型
///
/// 按月扣收
pub const THOST_FTDC_CCBFM_ByMonth: u8 = '2' as u8;
/// TFtdcCommApiTypeType是一个通讯API类型类型
///
/// 客户端
pub const THOST_FTDC_CAPIT_Client: u8 = '1' as u8;
/// TFtdcCommApiTypeType是一个通讯API类型类型
///
/// 服务端
pub const THOST_FTDC_CAPIT_Server: u8 = '2' as u8;
/// TFtdcCommApiTypeType是一个通讯API类型类型
///
/// 交易系统的UserApi
pub const THOST_FTDC_CAPIT_UserApi: u8 = '3' as u8;
/// TFtdcLinkStatusType是一个连接状态类型
///
/// 已经连接
pub const THOST_FTDC_LS_Connected: u8 = '1' as u8;
/// TFtdcLinkStatusType是一个连接状态类型
///
/// 没有连接
pub const THOST_FTDC_LS_Disconnected: u8 = '2' as u8;
/// TFtdcPwdFlagType是一个密码核对标志类型
///
/// 不核对
pub const THOST_FTDC_BPWDF_NoCheck: u8 = '0' as u8;
/// TFtdcPwdFlagType是一个密码核对标志类型
///
/// 明文核对
pub const THOST_FTDC_BPWDF_BlankCheck: u8 = '1' as u8;
/// TFtdcPwdFlagType是一个密码核对标志类型
///
/// 密文核对
pub const THOST_FTDC_BPWDF_EncryptCheck: u8 = '2' as u8;
/// TFtdcSecuAccTypeType是一个期货帐号类型类型
///
/// 资金帐号
pub const THOST_FTDC_SAT_AccountID: u8 = '1' as u8;
/// TFtdcSecuAccTypeType是一个期货帐号类型类型
///
/// 资金卡号
pub const THOST_FTDC_SAT_CardID: u8 = '2' as u8;
/// TFtdcSecuAccTypeType是一个期货帐号类型类型
///
/// 上海股东帐号
pub const THOST_FTDC_SAT_SHStockholderID: u8 = '3' as u8;
/// TFtdcSecuAccTypeType是一个期货帐号类型类型
///
/// 深圳股东帐号
pub const THOST_FTDC_SAT_SZStockholderID: u8 = '4' as u8;
/// TFtdcTransferStatusType是一个转账交易状态类型
///
/// 正常
pub const THOST_FTDC_TRFS_Normal: u8 = '0' as u8;
/// TFtdcTransferStatusType是一个转账交易状态类型
///
/// 被冲正
pub const THOST_FTDC_TRFS_Repealed: u8 = '1' as u8;
/// TFtdcSponsorTypeType是一个发起方类型
///
/// 期商
pub const THOST_FTDC_SPTYPE_Broker: u8 = '0' as u8;
/// TFtdcSponsorTypeType是一个发起方类型
///
/// 银行
pub const THOST_FTDC_SPTYPE_Bank: u8 = '1' as u8;
/// TFtdcReqRspTypeType是一个请求响应类别类型
///
/// 请求
pub const THOST_FTDC_REQRSP_Request: u8 = '0' as u8;
/// TFtdcReqRspTypeType是一个请求响应类别类型
///
/// 响应
pub const THOST_FTDC_REQRSP_Response: u8 = '1' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 签到
pub const THOST_FTDC_FBTUET_SignIn: u8 = '0' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 银行转期货
pub const THOST_FTDC_FBTUET_FromBankToFuture: u8 = '1' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 期货转银行
pub const THOST_FTDC_FBTUET_FromFutureToBank: u8 = '2' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 开户
pub const THOST_FTDC_FBTUET_OpenAccount: u8 = '3' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 销户
pub const THOST_FTDC_FBTUET_CancelAccount: u8 = '4' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 变更银行账户
pub const THOST_FTDC_FBTUET_ChangeAccount: u8 = '5' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 冲正银行转期货
pub const THOST_FTDC_FBTUET_RepealFromBankToFuture: u8 = '6' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 冲正期货转银行
pub const THOST_FTDC_FBTUET_RepealFromFutureToBank: u8 = '7' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 查询银行账户
pub const THOST_FTDC_FBTUET_QueryBankAccount: u8 = '8' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 查询期货账户
pub const THOST_FTDC_FBTUET_QueryFutureAccount: u8 = '9' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 签退
pub const THOST_FTDC_FBTUET_SignOut: u8 = 'A' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 密钥同步
pub const THOST_FTDC_FBTUET_SyncKey: u8 = 'B' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 预约开户
pub const THOST_FTDC_FBTUET_ReserveOpenAccount: u8 = 'C' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 撤销预约开户
pub const THOST_FTDC_FBTUET_CancelReserveOpenAccount: u8 = 'D' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 预约开户确认
pub const THOST_FTDC_FBTUET_ReserveOpenAccountConfirm: u8 = 'E' as u8;
/// TFtdcFBTUserEventTypeType是一个银期转帐用户事件类型类型
///
/// 其他
pub const THOST_FTDC_FBTUET_Other: u8 = 'Z' as u8;
/// TFtdcDBOperationType是一个记录操作类型类型
///
/// 插入
pub const THOST_FTDC_DBOP_Insert: u8 = '0' as u8;
/// TFtdcDBOperationType是一个记录操作类型类型
///
/// 更新
pub const THOST_FTDC_DBOP_Update: u8 = '1' as u8;
/// TFtdcDBOperationType是一个记录操作类型类型
///
/// 删除
pub const THOST_FTDC_DBOP_Delete: u8 = '2' as u8;
/// TFtdcSyncFlagType是一个同步标记类型
///
/// 已同步
pub const THOST_FTDC_SYNF_Yes: u8 = '0' as u8;
/// TFtdcSyncFlagType是一个同步标记类型
///
/// 未同步
pub const THOST_FTDC_SYNF_No: u8 = '1' as u8;
/// TFtdcSyncTypeType是一个同步类型类型
///
/// 一次同步
pub const THOST_FTDC_SYNT_OneOffSync: u8 = '0' as u8;
/// TFtdcSyncTypeType是一个同步类型类型
///
/// 定时同步
pub const THOST_FTDC_SYNT_TimerSync: u8 = '1' as u8;
/// TFtdcSyncTypeType是一个同步类型类型
///
/// 定时完全同步
pub const THOST_FTDC_SYNT_TimerFullSync: u8 = '2' as u8;
/// TFtdcExDirectionType是一个换汇方向类型
///
/// 结汇
pub const THOST_FTDC_FBEDIR_Settlement: u8 = '0' as u8;
/// TFtdcExDirectionType是一个换汇方向类型
///
/// 售汇
pub const THOST_FTDC_FBEDIR_Sale: u8 = '1' as u8;
/// TFtdcFBEResultFlagType是一个换汇成功标志类型
///
/// 成功
pub const THOST_FTDC_FBERES_Success: u8 = '0' as u8;
/// TFtdcFBEResultFlagType是一个换汇成功标志类型
///
/// 账户余额不足
pub const THOST_FTDC_FBERES_InsufficientBalance: u8 = '1' as u8;
/// TFtdcFBEResultFlagType是一个换汇成功标志类型
///
/// 交易结果未知
pub const THOST_FTDC_FBERES_UnknownTrading: u8 = '8' as u8;
/// TFtdcFBEResultFlagType是一个换汇成功标志类型
///
/// 失败
pub const THOST_FTDC_FBERES_Fail: u8 = 'x' as u8;
/// TFtdcFBEExchStatusType是一个换汇交易状态类型
///
/// 正常
pub const THOST_FTDC_FBEES_Normal: u8 = '0' as u8;
/// TFtdcFBEExchStatusType是一个换汇交易状态类型
///
/// 交易重发
pub const THOST_FTDC_FBEES_ReExchange: u8 = '1' as u8;
/// TFtdcFBEFileFlagType是一个换汇文件标志类型
///
/// 数据包
pub const THOST_FTDC_FBEFG_DataPackage: u8 = '0' as u8;
/// TFtdcFBEFileFlagType是一个换汇文件标志类型
///
/// 文件
pub const THOST_FTDC_FBEFG_File: u8 = '1' as u8;
/// TFtdcFBEAlreadyTradeType是一个换汇已交易标志类型
///
/// 未交易
pub const THOST_FTDC_FBEAT_NotTrade: u8 = '0' as u8;
/// TFtdcFBEAlreadyTradeType是一个换汇已交易标志类型
///
/// 已交易
pub const THOST_FTDC_FBEAT_Trade: u8 = '1' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 签到
pub const THOST_FTDC_FBEUET_SignIn: u8 = '0' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 换汇
pub const THOST_FTDC_FBEUET_Exchange: u8 = '1' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 换汇重发
pub const THOST_FTDC_FBEUET_ReExchange: u8 = '2' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 银行账户查询
pub const THOST_FTDC_FBEUET_QueryBankAccount: u8 = '3' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 换汇明细查询
pub const THOST_FTDC_FBEUET_QueryExchDetial: u8 = '4' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 换汇汇总查询
pub const THOST_FTDC_FBEUET_QueryExchSummary: u8 = '5' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 换汇汇率查询
pub const THOST_FTDC_FBEUET_QueryExchRate: u8 = '6' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 对账文件通知
pub const THOST_FTDC_FBEUET_CheckBankAccount: u8 = '7' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 签退
pub const THOST_FTDC_FBEUET_SignOut: u8 = '8' as u8;
/// TFtdcFBEUserEventTypeType是一个银期换汇用户事件类型类型
///
/// 其他
pub const THOST_FTDC_FBEUET_Other: u8 = 'Z' as u8;
/// TFtdcFBEReqFlagType是一个换汇发送标志类型
///
/// 未处理
pub const THOST_FTDC_FBERF_UnProcessed: u8 = '0' as u8;
/// TFtdcFBEReqFlagType是一个换汇发送标志类型
///
/// 等待发送
pub const THOST_FTDC_FBERF_WaitSend: u8 = '1' as u8;
/// TFtdcFBEReqFlagType是一个换汇发送标志类型
///
/// 发送成功
pub const THOST_FTDC_FBERF_SendSuccess: u8 = '2' as u8;
/// TFtdcFBEReqFlagType是一个换汇发送标志类型
///
/// 发送失败
pub const THOST_FTDC_FBERF_SendFailed: u8 = '3' as u8;
/// TFtdcFBEReqFlagType是一个换汇发送标志类型
///
/// 等待重发
pub const THOST_FTDC_FBERF_WaitReSend: u8 = '4' as u8;
/// TFtdcNotifyClassType是一个风险通知类型类型
///
/// 正常
pub const THOST_FTDC_NC_NOERROR: u8 = '0' as u8;
/// TFtdcNotifyClassType是一个风险通知类型类型
///
/// 警示
pub const THOST_FTDC_NC_Warn: u8 = '1' as u8;
/// TFtdcNotifyClassType是一个风险通知类型类型
///
/// 追保
pub const THOST_FTDC_NC_Call: u8 = '2' as u8;
/// TFtdcNotifyClassType是一个风险通知类型类型
///
/// 强平
pub const THOST_FTDC_NC_Force: u8 = '3' as u8;
/// TFtdcNotifyClassType是一个风险通知类型类型
///
/// 穿仓
pub const THOST_FTDC_NC_CHUANCANG: u8 = '4' as u8;
/// TFtdcNotifyClassType是一个风险通知类型类型
///
/// 异常
pub const THOST_FTDC_NC_Exception: u8 = '5' as u8;
/// TFtdcForceCloseTypeType是一个强平单类型类型
///
/// 手工强平
pub const THOST_FTDC_FCT_Manual: u8 = '0' as u8;
/// TFtdcForceCloseTypeType是一个强平单类型类型
///
/// 单一投资者辅助强平
pub const THOST_FTDC_FCT_Single: u8 = '1' as u8;
/// TFtdcForceCloseTypeType是一个强平单类型类型
///
/// 批量投资者辅助强平
pub const THOST_FTDC_FCT_Group: u8 = '2' as u8;
/// TFtdcRiskNotifyMethodType是一个风险通知途径类型
///
/// 系统通知
pub const THOST_FTDC_RNM_System: u8 = '0' as u8;
/// TFtdcRiskNotifyMethodType是一个风险通知途径类型
///
/// 短信通知
pub const THOST_FTDC_RNM_SMS: u8 = '1' as u8;
/// TFtdcRiskNotifyMethodType是一个风险通知途径类型
///
/// 邮件通知
pub const THOST_FTDC_RNM_EMail: u8 = '2' as u8;
/// TFtdcRiskNotifyMethodType是一个风险通知途径类型
///
/// 人工通知
pub const THOST_FTDC_RNM_Manual: u8 = '3' as u8;
/// TFtdcRiskNotifyStatusType是一个风险通知状态类型
///
/// 未生成
pub const THOST_FTDC_RNS_NotGen: u8 = '0' as u8;
/// TFtdcRiskNotifyStatusType是一个风险通知状态类型
///
/// 已生成未发送
pub const THOST_FTDC_RNS_Generated: u8 = '1' as u8;
/// TFtdcRiskNotifyStatusType是一个风险通知状态类型
///
/// 发送失败
pub const THOST_FTDC_RNS_SendError: u8 = '2' as u8;
/// TFtdcRiskNotifyStatusType是一个风险通知状态类型
///
/// 已发送未接收
pub const THOST_FTDC_RNS_SendOk: u8 = '3' as u8;
/// TFtdcRiskNotifyStatusType是一个风险通知状态类型
///
/// 已接收未确认
pub const THOST_FTDC_RNS_Received: u8 = '4' as u8;
/// TFtdcRiskNotifyStatusType是一个风险通知状态类型
///
/// 已确认
pub const THOST_FTDC_RNS_Confirmed: u8 = '5' as u8;
/// TFtdcRiskUserEventType是一个风控用户操作事件类型
///
/// 导出数据
pub const THOST_FTDC_RUE_ExportData: u8 = '0' as u8;
/// TFtdcConditionalOrderSortTypeType是一个条件单索引条件类型
///
/// 使用最新价升序
pub const THOST_FTDC_COST_LastPriceAsc: u8 = '0' as u8;
/// TFtdcConditionalOrderSortTypeType是一个条件单索引条件类型
///
/// 使用最新价降序
pub const THOST_FTDC_COST_LastPriceDesc: u8 = '1' as u8;
/// TFtdcConditionalOrderSortTypeType是一个条件单索引条件类型
///
/// 使用卖价升序
pub const THOST_FTDC_COST_AskPriceAsc: u8 = '2' as u8;
/// TFtdcConditionalOrderSortTypeType是一个条件单索引条件类型
///
/// 使用卖价降序
pub const THOST_FTDC_COST_AskPriceDesc: u8 = '3' as u8;
/// TFtdcConditionalOrderSortTypeType是一个条件单索引条件类型
///
/// 使用买价升序
pub const THOST_FTDC_COST_BidPriceAsc: u8 = '4' as u8;
/// TFtdcConditionalOrderSortTypeType是一个条件单索引条件类型
///
/// 使用买价降序
pub const THOST_FTDC_COST_BidPriceDesc: u8 = '5' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 未发送
pub const THOST_FTDC_UOAST_NoSend: u8 = '0' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 已发送
pub const THOST_FTDC_UOAST_Sended: u8 = '1' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 已生成
pub const THOST_FTDC_UOAST_Generated: u8 = '2' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 报送失败
pub const THOST_FTDC_UOAST_SendFail: u8 = '3' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 接收成功
pub const THOST_FTDC_UOAST_Success: u8 = '4' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 接收失败
pub const THOST_FTDC_UOAST_Fail: u8 = '5' as u8;
/// TFtdcSendTypeType是一个报送状态类型
///
/// 取消报送
pub const THOST_FTDC_UOAST_Cancel: u8 = '6' as u8;
/// TFtdcClientIDStatusType是一个交易编码状态类型
///
/// 未申请
pub const THOST_FTDC_UOACS_NoApply: u8 = '1' as u8;
/// TFtdcClientIDStatusType是一个交易编码状态类型
///
/// 已提交申请
pub const THOST_FTDC_UOACS_Submited: u8 = '2' as u8;
/// TFtdcClientIDStatusType是一个交易编码状态类型
///
/// 已发送申请
pub const THOST_FTDC_UOACS_Sended: u8 = '3' as u8;
/// TFtdcClientIDStatusType是一个交易编码状态类型
///
/// 完成
pub const THOST_FTDC_UOACS_Success: u8 = '4' as u8;
/// TFtdcClientIDStatusType是一个交易编码状态类型
///
/// 拒绝
pub const THOST_FTDC_UOACS_Refuse: u8 = '5' as u8;
/// TFtdcClientIDStatusType是一个交易编码状态类型
///
/// 已撤销编码
pub const THOST_FTDC_UOACS_Cancel: u8 = '6' as u8;
/// TFtdcQuestionTypeType是一个特有信息类型类型
///
/// 单选
pub const THOST_FTDC_QT_Radio: u8 = '1' as u8;
/// TFtdcQuestionTypeType是一个特有信息类型类型
///
/// 多选
pub const THOST_FTDC_QT_Option: u8 = '2' as u8;
/// TFtdcQuestionTypeType是一个特有信息类型类型
///
/// 填空
pub const THOST_FTDC_QT_Blank: u8 = '3' as u8;
/// TFtdcBusinessTypeType是一个业务类型类型
///
/// 请求
pub const THOST_FTDC_BT_Request: u8 = '1' as u8;
/// TFtdcBusinessTypeType是一个业务类型类型
///
/// 应答
pub const THOST_FTDC_BT_Response: u8 = '2' as u8;
/// TFtdcBusinessTypeType是一个业务类型类型
///
/// 通知
pub const THOST_FTDC_BT_Notice: u8 = '3' as u8;
/// TFtdcCfmmcReturnCodeType是一个监控中心返回码类型
///
/// 成功
pub const THOST_FTDC_CRC_Success: u8 = '0' as u8;
/// TFtdcCfmmcReturnCodeType是一个监控中心返回码类型
///
/// 该客户已经有流程在处理中
pub const THOST_FTDC_CRC_Working: u8 = '1' as u8;
/// TFtdcCfmmcReturnCodeType是一个监控中心返回码类型
///
/// 监控中客户资料检查失败
pub const THOST_FTDC_CRC_InfoFail: u8 = '2' as u8;
/// TFtdcCfmmcReturnCodeType是一个监控中心返回码类型
///
/// 监控中实名制检查失败
pub const THOST_FTDC_CRC_IDCardFail: u8 = '3' as u8;
/// TFtdcCfmmcReturnCodeType是一个监控中心返回码类型
///
/// 其他错误
pub const THOST_FTDC_CRC_OtherFail: u8 = '4' as u8;
/// TFtdcClientTypeType是一个客户类型类型
///
/// 所有
pub const THOST_FTDC_CfMMCCT_All: u8 = '0' as u8;
/// TFtdcClientTypeType是一个客户类型类型
///
/// 个人
pub const THOST_FTDC_CfMMCCT_Person: u8 = '1' as u8;
/// TFtdcClientTypeType是一个客户类型类型
///
/// 单位
pub const THOST_FTDC_CfMMCCT_Company: u8 = '2' as u8;
/// TFtdcClientTypeType是一个客户类型类型
///
/// 其他
pub const THOST_FTDC_CfMMCCT_Other: u8 = '3' as u8;
/// TFtdcClientTypeType是一个客户类型类型
///
/// 特殊法人
pub const THOST_FTDC_CfMMCCT_SpecialOrgan: u8 = '4' as u8;
/// TFtdcClientTypeType是一个客户类型类型
///
/// 资管户
pub const THOST_FTDC_CfMMCCT_Asset: u8 = '5' as u8;
/// TFtdcExchangeIDTypeType是一个交易所编号类型
///
/// 上海期货交易所
pub const THOST_FTDC_EIDT_SHFE: u8 = 'S' as u8;
/// TFtdcExchangeIDTypeType是一个交易所编号类型
///
/// 郑州商品交易所
pub const THOST_FTDC_EIDT_CZCE: u8 = 'Z' as u8;
/// TFtdcExchangeIDTypeType是一个交易所编号类型
///
/// 大连商品交易所
pub const THOST_FTDC_EIDT_DCE: u8 = 'D' as u8;
/// TFtdcExchangeIDTypeType是一个交易所编号类型
///
/// 中国金融期货交易所
pub const THOST_FTDC_EIDT_CFFEX: u8 = 'J' as u8;
/// TFtdcExchangeIDTypeType是一个交易所编号类型
///
/// 上海国际能源交易中心股份有限公司
pub const THOST_FTDC_EIDT_INE: u8 = 'N' as u8;
/// TFtdcExClientIDTypeType是一个交易编码类型类型
///
/// 套保
pub const THOST_FTDC_ECIDT_Hedge: u8 = '1' as u8;
/// TFtdcExClientIDTypeType是一个交易编码类型类型
///
/// 套利
pub const THOST_FTDC_ECIDT_Arbitrage: u8 = '2' as u8;
/// TFtdcExClientIDTypeType是一个交易编码类型类型
///
/// 投机
pub const THOST_FTDC_ECIDT_Speculation: u8 = '3' as u8;
/// TFtdcUpdateFlagType是一个更新状态类型
///
/// 未更新
pub const THOST_FTDC_UF_NoUpdate: u8 = '0' as u8;
/// TFtdcUpdateFlagType是一个更新状态类型
///
/// 更新全部信息成功
pub const THOST_FTDC_UF_Success: u8 = '1' as u8;
/// TFtdcUpdateFlagType是一个更新状态类型
///
/// 更新全部信息失败
pub const THOST_FTDC_UF_Fail: u8 = '2' as u8;
/// TFtdcUpdateFlagType是一个更新状态类型
///
/// 更新交易编码成功
pub const THOST_FTDC_UF_TCSuccess: u8 = '3' as u8;
/// TFtdcUpdateFlagType是一个更新状态类型
///
/// 更新交易编码失败
pub const THOST_FTDC_UF_TCFail: u8 = '4' as u8;
/// TFtdcUpdateFlagType是一个更新状态类型
///
/// 已丢弃
pub const THOST_FTDC_UF_Cancel: u8 = '5' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 开户
pub const THOST_FTDC_AOID_OpenInvestor: u8 = '1' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 修改身份信息
pub const THOST_FTDC_AOID_ModifyIDCard: u8 = '2' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 修改一般信息
pub const THOST_FTDC_AOID_ModifyNoIDCard: u8 = '3' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 申请交易编码
pub const THOST_FTDC_AOID_ApplyTradingCode: u8 = '4' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 撤销交易编码
pub const THOST_FTDC_AOID_CancelTradingCode: u8 = '5' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 销户
pub const THOST_FTDC_AOID_CancelInvestor: u8 = '6' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 账户休眠
pub const THOST_FTDC_AOID_FreezeAccount: u8 = '8' as u8;
/// TFtdcApplyOperateIDType是一个申请动作类型
///
/// 激活休眠账户
pub const THOST_FTDC_AOID_ActiveFreezeAccount: u8 = '9' as u8;
/// TFtdcApplyStatusIDType是一个申请状态类型
///
/// 未补全
pub const THOST_FTDC_ASID_NoComplete: u8 = '1' as u8;
/// TFtdcApplyStatusIDType是一个申请状态类型
///
/// 已提交
pub const THOST_FTDC_ASID_Submited: u8 = '2' as u8;
/// TFtdcApplyStatusIDType是一个申请状态类型
///
/// 已审核
pub const THOST_FTDC_ASID_Checked: u8 = '3' as u8;
/// TFtdcApplyStatusIDType是一个申请状态类型
///
/// 已拒绝
pub const THOST_FTDC_ASID_Refused: u8 = '4' as u8;
/// TFtdcApplyStatusIDType是一个申请状态类型
///
/// 已删除
pub const THOST_FTDC_ASID_Deleted: u8 = '5' as u8;
/// TFtdcSendMethodType是一个发送方式类型
///
/// 文件发送
pub const THOST_FTDC_UOASM_ByAPI: u8 = '1' as u8;
/// TFtdcSendMethodType是一个发送方式类型
///
/// 电子发送
pub const THOST_FTDC_UOASM_ByFile: u8 = '2' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 增加
pub const THOST_FTDC_EvM_ADD: u8 = '1' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 修改
pub const THOST_FTDC_EvM_UPDATE: u8 = '2' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 删除
pub const THOST_FTDC_EvM_DELETE: u8 = '3' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 复核
pub const THOST_FTDC_EvM_CHECK: u8 = '4' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 复制
pub const THOST_FTDC_EvM_COPY: u8 = '5' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 注销
pub const THOST_FTDC_EvM_CANCEL: u8 = '6' as u8;
/// TFtdcEventModeType是一个操作方法类型
///
/// 冲销
pub const THOST_FTDC_EvM_Reverse: u8 = '7' as u8;
/// TFtdcUOAAutoSendType是一个统一开户申请自动发送类型
///
/// 自动发送并接收
pub const THOST_FTDC_UOAA_ASR: u8 = '1' as u8;
/// TFtdcUOAAutoSendType是一个统一开户申请自动发送类型
///
/// 自动发送，不自动接收
pub const THOST_FTDC_UOAA_ASNR: u8 = '2' as u8;
/// TFtdcUOAAutoSendType是一个统一开户申请自动发送类型
///
/// 不自动发送，自动接收
pub const THOST_FTDC_UOAA_NSAR: u8 = '3' as u8;
/// TFtdcUOAAutoSendType是一个统一开户申请自动发送类型
///
/// 不自动发送，也不自动接收
pub const THOST_FTDC_UOAA_NSR: u8 = '4' as u8;
/// TFtdcFlowIDType是一个流程ID类型
///
/// 投资者对应投资者组设置
pub const THOST_FTDC_EvM_InvestorGroupFlow: u8 = '1' as u8;
/// TFtdcFlowIDType是一个流程ID类型
///
/// 投资者手续费率设置
pub const THOST_FTDC_EvM_InvestorRate: u8 = '2' as u8;
/// TFtdcFlowIDType是一个流程ID类型
///
/// 投资者手续费率模板关系设置
pub const THOST_FTDC_EvM_InvestorCommRateModel: u8 = '3' as u8;
/// TFtdcCheckLevelType是一个复核级别类型
///
/// 零级复核
pub const THOST_FTDC_CL_Zero: u8 = '0' as u8;
/// TFtdcCheckLevelType是一个复核级别类型
///
/// 一级复核
pub const THOST_FTDC_CL_One: u8 = '1' as u8;
/// TFtdcCheckLevelType是一个复核级别类型
///
/// 二级复核
pub const THOST_FTDC_CL_Two: u8 = '2' as u8;
/// TFtdcCheckStatusType是一个复核级别类型
///
/// 未复核
pub const THOST_FTDC_CHS_Init: u8 = '0' as u8;
/// TFtdcCheckStatusType是一个复核级别类型
///
/// 复核中
pub const THOST_FTDC_CHS_Checking: u8 = '1' as u8;
/// TFtdcCheckStatusType是一个复核级别类型
///
/// 已复核
pub const THOST_FTDC_CHS_Checked: u8 = '2' as u8;
/// TFtdcCheckStatusType是一个复核级别类型
///
/// 拒绝
pub const THOST_FTDC_CHS_Refuse: u8 = '3' as u8;
/// TFtdcCheckStatusType是一个复核级别类型
///
/// 作废
pub const THOST_FTDC_CHS_Cancel: u8 = '4' as u8;
/// TFtdcUsedStatusType是一个生效状态类型
///
/// 未生效
pub const THOST_FTDC_CHU_Unused: u8 = '0' as u8;
/// TFtdcUsedStatusType是一个生效状态类型
///
/// 已生效
pub const THOST_FTDC_CHU_Used: u8 = '1' as u8;
/// TFtdcUsedStatusType是一个生效状态类型
///
/// 生效失败
pub const THOST_FTDC_CHU_Fail: u8 = '2' as u8;
/// TFtdcBankAcountOriginType是一个账户来源类型
///
/// 手工录入
pub const THOST_FTDC_BAO_ByAccProperty: u8 = '0' as u8;
/// TFtdcBankAcountOriginType是一个账户来源类型
///
/// 银期转账
pub const THOST_FTDC_BAO_ByFBTransfer: u8 = '1' as u8;
/// TFtdcMonthBillTradeSumType是一个结算单月报成交汇总方式类型
///
/// 同日同合约
pub const THOST_FTDC_MBTS_ByInstrument: u8 = '0' as u8;
/// TFtdcMonthBillTradeSumType是一个结算单月报成交汇总方式类型
///
/// 同日同合约同价格
pub const THOST_FTDC_MBTS_ByDayInsPrc: u8 = '1' as u8;
/// TFtdcMonthBillTradeSumType是一个结算单月报成交汇总方式类型
///
/// 同合约
pub const THOST_FTDC_MBTS_ByDayIns: u8 = '2' as u8;
/// TFtdcOTPTypeType是一个动态令牌类型类型
///
/// 无动态令牌
pub const THOST_FTDC_OTP_NONE: u8 = '0' as u8;
/// TFtdcOTPTypeType是一个动态令牌类型类型
///
/// 时间令牌
pub const THOST_FTDC_OTP_TOTP: u8 = '1' as u8;
/// TFtdcOTPStatusType是一个动态令牌状态类型
///
/// 未使用
pub const THOST_FTDC_OTPS_Unused: u8 = '0' as u8;
/// TFtdcOTPStatusType是一个动态令牌状态类型
///
/// 已使用
pub const THOST_FTDC_OTPS_Used: u8 = '1' as u8;
/// TFtdcOTPStatusType是一个动态令牌状态类型
///
/// 注销
pub const THOST_FTDC_OTPS_Disuse: u8 = '2' as u8;
/// TFtdcBrokerUserTypeType是一个经济公司用户类型类型
///
/// 投资者
pub const THOST_FTDC_BUT_Investor: u8 = '1' as u8;
/// TFtdcBrokerUserTypeType是一个经济公司用户类型类型
///
/// 操作员
pub const THOST_FTDC_BUT_BrokerUser: u8 = '2' as u8;
/// TFtdcFutureTypeType是一个期货类型类型
///
/// 商品期货
pub const THOST_FTDC_FUTT_Commodity: u8 = '1' as u8;
/// TFtdcFutureTypeType是一个期货类型类型
///
/// 金融期货
pub const THOST_FTDC_FUTT_Financial: u8 = '2' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 转账限额
pub const THOST_FTDC_FET_Restriction: u8 = '0' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 当日转账限额
pub const THOST_FTDC_FET_TodayRestriction: u8 = '1' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 期商流水
pub const THOST_FTDC_FET_Transfer: u8 = '2' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 资金冻结
pub const THOST_FTDC_FET_Credit: u8 = '3' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 投资者可提资金比例
pub const THOST_FTDC_FET_InvestorWithdrawAlm: u8 = '4' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 单个银行帐户转账限额
pub const THOST_FTDC_FET_BankRestriction: u8 = '5' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 银期签约账户
pub const THOST_FTDC_FET_Accountregister: u8 = '6' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 交易所出入金
pub const THOST_FTDC_FET_ExchangeFundIO: u8 = '7' as u8;
/// TFtdcFundEventTypeType是一个资金管理操作类型类型
///
/// 投资者出入金
pub const THOST_FTDC_FET_InvestorFundIO: u8 = '8' as u8;
/// TFtdcAccountSourceTypeType是一个资金账户来源类型
///
/// 银期同步
pub const THOST_FTDC_AST_FBTransfer: u8 = '0' as u8;
/// TFtdcAccountSourceTypeType是一个资金账户来源类型
///
/// 手工录入
pub const THOST_FTDC_AST_ManualEntry: u8 = '1' as u8;
/// TFtdcCodeSourceTypeType是一个交易编码来源类型
///
/// 统一开户(已规范)
pub const THOST_FTDC_CST_UnifyAccount: u8 = '0' as u8;
/// TFtdcCodeSourceTypeType是一个交易编码来源类型
///
/// 手工录入(未规范)
pub const THOST_FTDC_CST_ManualEntry: u8 = '1' as u8;
/// TFtdcUserRangeType是一个操作员范围类型
///
/// 所有
pub const THOST_FTDC_UR_All: u8 = '0' as u8;
/// TFtdcUserRangeType是一个操作员范围类型
///
/// 单一操作员
pub const THOST_FTDC_UR_Single: u8 = '1' as u8;
/// TFtdcByGroupType是一个交易统计表按客户统计方式类型
///
/// 按投资者统计
pub const THOST_FTDC_BG_Investor: u8 = '2' as u8;
/// TFtdcByGroupType是一个交易统计表按客户统计方式类型
///
/// 按类统计
pub const THOST_FTDC_BG_Group: u8 = '1' as u8;
/// TFtdcTradeSumStatModeType是一个交易统计表按范围统计方式类型
///
/// 按合约统计
pub const THOST_FTDC_TSSM_Instrument: u8 = '1' as u8;
/// TFtdcTradeSumStatModeType是一个交易统计表按范围统计方式类型
///
/// 按产品统计
pub const THOST_FTDC_TSSM_Product: u8 = '2' as u8;
/// TFtdcTradeSumStatModeType是一个交易统计表按范围统计方式类型
///
/// 按交易所统计
pub const THOST_FTDC_TSSM_Exchange: u8 = '3' as u8;
/// TFtdcExprSetModeType是一个日期表达式设置类型类型
///
/// 相对已有规则设置
pub const THOST_FTDC_ESM_Relative: u8 = '1' as u8;
/// TFtdcExprSetModeType是一个日期表达式设置类型类型
///
/// 典型设置
pub const THOST_FTDC_ESM_Typical: u8 = '2' as u8;
/// TFtdcRateInvestorRangeType是一个投资者范围类型
///
/// 公司标准
pub const THOST_FTDC_RIR_All: u8 = '1' as u8;
/// TFtdcRateInvestorRangeType是一个投资者范围类型
///
/// 模板
pub const THOST_FTDC_RIR_Model: u8 = '2' as u8;
/// TFtdcRateInvestorRangeType是一个投资者范围类型
///
/// 单一投资者
pub const THOST_FTDC_RIR_Single: u8 = '3' as u8;
/// TFtdcSyncDataStatusType是一个主次用系统数据同步状态类型
///
/// 未同步
pub const THOST_FTDC_SDS_Initialize: u8 = '0' as u8;
/// TFtdcSyncDataStatusType是一个主次用系统数据同步状态类型
///
/// 同步中
pub const THOST_FTDC_SDS_Settlementing: u8 = '1' as u8;
/// TFtdcSyncDataStatusType是一个主次用系统数据同步状态类型
///
/// 已同步
pub const THOST_FTDC_SDS_Settlemented: u8 = '2' as u8;
/// TFtdcTradeSourceType是一个成交来源类型
///
/// 来自交易所普通回报
pub const THOST_FTDC_TSRC_NORMAL: u8 = '0' as u8;
/// TFtdcTradeSourceType是一个成交来源类型
///
/// 来自查询
pub const THOST_FTDC_TSRC_QUERY: u8 = '1' as u8;
/// TFtdcFlexStatModeType是一个产品合约统计方式类型
///
/// 产品统计
pub const THOST_FTDC_FSM_Product: u8 = '1' as u8;
/// TFtdcFlexStatModeType是一个产品合约统计方式类型
///
/// 交易所统计
pub const THOST_FTDC_FSM_Exchange: u8 = '2' as u8;
/// TFtdcFlexStatModeType是一个产品合约统计方式类型
///
/// 统计所有
pub const THOST_FTDC_FSM_All: u8 = '3' as u8;
/// TFtdcByInvestorRangeType是一个投资者范围统计方式类型
///
/// 属性统计
pub const THOST_FTDC_BIR_Property: u8 = '1' as u8;
/// TFtdcByInvestorRangeType是一个投资者范围统计方式类型
///
/// 统计所有
pub const THOST_FTDC_BIR_All: u8 = '2' as u8;
/// TFtdcPropertyInvestorRangeType是一个投资者范围类型
///
/// 所有
pub const THOST_FTDC_PIR_All: u8 = '1' as u8;
/// TFtdcPropertyInvestorRangeType是一个投资者范围类型
///
/// 投资者属性
pub const THOST_FTDC_PIR_Property: u8 = '2' as u8;
/// TFtdcPropertyInvestorRangeType是一个投资者范围类型
///
/// 单一投资者
pub const THOST_FTDC_PIR_Single: u8 = '3' as u8;
/// TFtdcFileStatusType是一个文件状态类型
///
/// 未生成
pub const THOST_FTDC_FIS_NoCreate: u8 = '0' as u8;
/// TFtdcFileStatusType是一个文件状态类型
///
/// 已生成
pub const THOST_FTDC_FIS_Created: u8 = '1' as u8;
/// TFtdcFileStatusType是一个文件状态类型
///
/// 生成失败
pub const THOST_FTDC_FIS_Failed: u8 = '2' as u8;
/// TFtdcFileGenStyleType是一个文件生成方式类型
///
/// 下发
pub const THOST_FTDC_FGS_FileTransmit: u8 = '0' as u8;
/// TFtdcFileGenStyleType是一个文件生成方式类型
///
/// 生成
pub const THOST_FTDC_FGS_FileGen: u8 = '1' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 增加
pub const THOST_FTDC_SoM_Add: u8 = '1' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 修改
pub const THOST_FTDC_SoM_Update: u8 = '2' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 删除
pub const THOST_FTDC_SoM_Delete: u8 = '3' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 复制
pub const THOST_FTDC_SoM_Copy: u8 = '4' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 激活
pub const THOST_FTDC_SoM_AcTive: u8 = '5' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 注销
pub const THOST_FTDC_SoM_CanCel: u8 = '6' as u8;
/// TFtdcSysOperModeType是一个系统日志操作方法类型
///
/// 重置
pub const THOST_FTDC_SoM_ReSet: u8 = '7' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 修改操作员密码
pub const THOST_FTDC_SoT_UpdatePassword: u8 = '0' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 操作员组织架构关系
pub const THOST_FTDC_SoT_UserDepartment: u8 = '1' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 角色管理
pub const THOST_FTDC_SoT_RoleManager: u8 = '2' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 角色功能设置
pub const THOST_FTDC_SoT_RoleFunction: u8 = '3' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 基础参数设置
pub const THOST_FTDC_SoT_BaseParam: u8 = '4' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 设置操作员
pub const THOST_FTDC_SoT_SetUserID: u8 = '5' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 用户角色设置
pub const THOST_FTDC_SoT_SetUserRole: u8 = '6' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 用户IP限制
pub const THOST_FTDC_SoT_UserIpRestriction: u8 = '7' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 组织架构管理
pub const THOST_FTDC_SoT_DepartmentManager: u8 = '8' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 组织架构向查询分类复制
pub const THOST_FTDC_SoT_DepartmentCopy: u8 = '9' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 交易编码管理
pub const THOST_FTDC_SoT_Tradingcode: u8 = 'A' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 投资者状态维护
pub const THOST_FTDC_SoT_InvestorStatus: u8 = 'B' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 投资者权限管理
pub const THOST_FTDC_SoT_InvestorAuthority: u8 = 'C' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 属性设置
pub const THOST_FTDC_SoT_PropertySet: u8 = 'D' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 重置投资者密码
pub const THOST_FTDC_SoT_ReSetInvestorPasswd: u8 = 'E' as u8;
/// TFtdcSysOperTypeType是一个系统日志操作类型类型
///
/// 投资者个性信息维护
pub const THOST_FTDC_SoT_InvestorPersonalityInfo: u8 = 'F' as u8;
/// TFtdcCSRCDataQueyTypeType是一个上报数据查询类型类型
///
/// 查询当前交易日报送的数据
pub const THOST_FTDC_CSRCQ_Current: u8 = '0' as u8;
/// TFtdcCSRCDataQueyTypeType是一个上报数据查询类型类型
///
/// 查询历史报送的代理经纪公司的数据
pub const THOST_FTDC_CSRCQ_History: u8 = '1' as u8;
/// TFtdcFreezeStatusType是一个休眠状态类型
///
/// 活跃
pub const THOST_FTDC_FRS_Normal: u8 = '1' as u8;
/// TFtdcFreezeStatusType是一个休眠状态类型
///
/// 休眠
pub const THOST_FTDC_FRS_Freeze: u8 = '0' as u8;
/// TFtdcStandardStatusType是一个规范状态类型
///
/// 已规范
pub const THOST_FTDC_STST_Standard: u8 = '0' as u8;
/// TFtdcStandardStatusType是一个规范状态类型
///
/// 未规范
pub const THOST_FTDC_STST_NonStandard: u8 = '1' as u8;
/// TFtdcRightParamTypeType是一个配置类型类型
///
/// 休眠户
pub const THOST_FTDC_RPT_Freeze: u8 = '1' as u8;
/// TFtdcRightParamTypeType是一个配置类型类型
///
/// 激活休眠户
pub const THOST_FTDC_RPT_FreezeActive: u8 = '2' as u8;
/// TFtdcRightParamTypeType是一个配置类型类型
///
/// 开仓权限限制
pub const THOST_FTDC_RPT_OpenLimit: u8 = '3' as u8;
/// TFtdcRightParamTypeType是一个配置类型类型
///
/// 解除开仓权限限制
pub const THOST_FTDC_RPT_RelieveOpenLimit: u8 = '4' as u8;
/// TFtdcDataStatusType是一个反洗钱审核表数据状态类型
///
/// 正常
pub const THOST_FTDC_AMLDS_Normal: u8 = '0' as u8;
/// TFtdcDataStatusType是一个反洗钱审核表数据状态类型
///
/// 已删除
pub const THOST_FTDC_AMLDS_Deleted: u8 = '1' as u8;
/// TFtdcAMLCheckStatusType是一个审核状态类型
///
/// 未复核
pub const THOST_FTDC_AMLCHS_Init: u8 = '0' as u8;
/// TFtdcAMLCheckStatusType是一个审核状态类型
///
/// 复核中
pub const THOST_FTDC_AMLCHS_Checking: u8 = '1' as u8;
/// TFtdcAMLCheckStatusType是一个审核状态类型
///
/// 已复核
pub const THOST_FTDC_AMLCHS_Checked: u8 = '2' as u8;
/// TFtdcAMLCheckStatusType是一个审核状态类型
///
/// 拒绝上报
pub const THOST_FTDC_AMLCHS_RefuseReport: u8 = '3' as u8;
/// TFtdcAmlDateTypeType是一个日期类型类型
///
/// 检查日期
pub const THOST_FTDC_AMLDT_DrawDay: u8 = '0' as u8;
/// TFtdcAmlDateTypeType是一个日期类型类型
///
/// 发生日期
pub const THOST_FTDC_AMLDT_TouchDay: u8 = '1' as u8;
/// TFtdcAmlCheckLevelType是一个审核级别类型
///
/// 零级审核
pub const THOST_FTDC_AMLCL_CheckLevel0: u8 = '0' as u8;
/// TFtdcAmlCheckLevelType是一个审核级别类型
///
/// 一级审核
pub const THOST_FTDC_AMLCL_CheckLevel1: u8 = '1' as u8;
/// TFtdcAmlCheckLevelType是一个审核级别类型
///
/// 二级审核
pub const THOST_FTDC_AMLCL_CheckLevel2: u8 = '2' as u8;
/// TFtdcAmlCheckLevelType是一个审核级别类型
///
/// 三级审核
pub const THOST_FTDC_AMLCL_CheckLevel3: u8 = '3' as u8;
/// TFtdcExportFileTypeType是一个导出文件类型类型
///
/// CSV
pub const THOST_FTDC_EFT_CSV: u8 = '0' as u8;
/// TFtdcExportFileTypeType是一个导出文件类型类型
///
/// Excel
pub const THOST_FTDC_EFT_EXCEL: u8 = '1' as u8;
/// TFtdcExportFileTypeType是一个导出文件类型类型
///
/// DBF
pub const THOST_FTDC_EFT_DBF: u8 = '2' as u8;
/// TFtdcSettleManagerTypeType是一个结算配置类型类型
///
/// 结算前准备
pub const THOST_FTDC_SMT_Before: u8 = '1' as u8;
/// TFtdcSettleManagerTypeType是一个结算配置类型类型
///
/// 结算
pub const THOST_FTDC_SMT_Settlement: u8 = '2' as u8;
/// TFtdcSettleManagerTypeType是一个结算配置类型类型
///
/// 结算后核对
pub const THOST_FTDC_SMT_After: u8 = '3' as u8;
/// TFtdcSettleManagerTypeType是一个结算配置类型类型
///
/// 结算后处理
pub const THOST_FTDC_SMT_Settlemented: u8 = '4' as u8;
/// TFtdcSettleManagerLevelType是一个结算配置等级类型
///
/// 必要
pub const THOST_FTDC_SML_Must: u8 = '1' as u8;
/// TFtdcSettleManagerLevelType是一个结算配置等级类型
///
/// 警告
pub const THOST_FTDC_SML_Alarm: u8 = '2' as u8;
/// TFtdcSettleManagerLevelType是一个结算配置等级类型
///
/// 提示
pub const THOST_FTDC_SML_Prompt: u8 = '3' as u8;
/// TFtdcSettleManagerLevelType是一个结算配置等级类型
///
/// 不检查
pub const THOST_FTDC_SML_Ignore: u8 = '4' as u8;
/// TFtdcSettleManagerGroupType是一个模块分组类型
///
/// 交易所核对
pub const THOST_FTDC_SMG_Exhcange: u8 = '1' as u8;
/// TFtdcSettleManagerGroupType是一个模块分组类型
///
/// 内部核对
pub const THOST_FTDC_SMG_ASP: u8 = '2' as u8;
/// TFtdcSettleManagerGroupType是一个模块分组类型
///
/// 上报数据核对
pub const THOST_FTDC_SMG_CSRC: u8 = '3' as u8;
/// TFtdcLimitUseTypeType是一个保值额度使用类型类型
///
/// 可重复使用
pub const THOST_FTDC_LUT_Repeatable: u8 = '1' as u8;
/// TFtdcLimitUseTypeType是一个保值额度使用类型类型
///
/// 不可重复使用
pub const THOST_FTDC_LUT_Unrepeatable: u8 = '2' as u8;
/// TFtdcDataResourceType是一个数据来源类型
///
/// 本系统
pub const THOST_FTDC_DAR_Settle: u8 = '1' as u8;
/// TFtdcDataResourceType是一个数据来源类型
///
/// 交易所
pub const THOST_FTDC_DAR_Exchange: u8 = '2' as u8;
/// TFtdcDataResourceType是一个数据来源类型
///
/// 报送数据
pub const THOST_FTDC_DAR_CSRC: u8 = '3' as u8;
/// TFtdcMarginTypeType是一个保证金类型类型
///
/// 交易所保证金率
pub const THOST_FTDC_MGT_ExchMarginRate: u8 = '0' as u8;
/// TFtdcMarginTypeType是一个保证金类型类型
///
/// 投资者保证金率
pub const THOST_FTDC_MGT_InstrMarginRate: u8 = '1' as u8;
/// TFtdcMarginTypeType是一个保证金类型类型
///
/// 投资者交易保证金率
pub const THOST_FTDC_MGT_InstrMarginRateTrade: u8 = '2' as u8;
/// TFtdcActiveTypeType是一个生效类型类型
///
/// 仅当日生效
pub const THOST_FTDC_ACT_Intraday: u8 = '1' as u8;
/// TFtdcActiveTypeType是一个生效类型类型
///
/// 长期生效
pub const THOST_FTDC_ACT_Long: u8 = '2' as u8;
/// TFtdcMarginRateTypeType是一个冲突保证金率类型类型
///
/// 交易所保证金率
pub const THOST_FTDC_MRT_Exchange: u8 = '1' as u8;
/// TFtdcMarginRateTypeType是一个冲突保证金率类型类型
///
/// 投资者保证金率
pub const THOST_FTDC_MRT_Investor: u8 = '2' as u8;
/// TFtdcMarginRateTypeType是一个冲突保证金率类型类型
///
/// 投资者交易保证金率
pub const THOST_FTDC_MRT_InvestorTrade: u8 = '3' as u8;
/// TFtdcBackUpStatusType是一个备份数据状态类型
///
/// 未生成备份数据
pub const THOST_FTDC_BUS_UnBak: u8 = '0' as u8;
/// TFtdcBackUpStatusType是一个备份数据状态类型
///
/// 备份数据生成中
pub const THOST_FTDC_BUS_BakUp: u8 = '1' as u8;
/// TFtdcBackUpStatusType是一个备份数据状态类型
///
/// 已生成备份数据
pub const THOST_FTDC_BUS_BakUped: u8 = '2' as u8;
/// TFtdcBackUpStatusType是一个备份数据状态类型
///
/// 备份数据失败
pub const THOST_FTDC_BUS_BakFail: u8 = '3' as u8;
/// TFtdcInitSettlementType是一个结算初始化状态类型
///
/// 结算初始化未开始
pub const THOST_FTDC_SIS_UnInitialize: u8 = '0' as u8;
/// TFtdcInitSettlementType是一个结算初始化状态类型
///
/// 结算初始化中
pub const THOST_FTDC_SIS_Initialize: u8 = '1' as u8;
/// TFtdcInitSettlementType是一个结算初始化状态类型
///
/// 结算初始化完成
pub const THOST_FTDC_SIS_Initialized: u8 = '2' as u8;
/// TFtdcReportStatusType是一个报表数据生成状态类型
///
/// 未生成报表数据
pub const THOST_FTDC_SRS_NoCreate: u8 = '0' as u8;
/// TFtdcReportStatusType是一个报表数据生成状态类型
///
/// 报表数据生成中
pub const THOST_FTDC_SRS_Create: u8 = '1' as u8;
/// TFtdcReportStatusType是一个报表数据生成状态类型
///
/// 已生成报表数据
pub const THOST_FTDC_SRS_Created: u8 = '2' as u8;
/// TFtdcReportStatusType是一个报表数据生成状态类型
///
/// 生成报表数据失败
pub const THOST_FTDC_SRS_CreateFail: u8 = '3' as u8;
/// TFtdcSaveStatusType是一个数据归档状态类型
///
/// 归档未完成
pub const THOST_FTDC_SSS_UnSaveData: u8 = '0' as u8;
/// TFtdcSaveStatusType是一个数据归档状态类型
///
/// 归档完成
pub const THOST_FTDC_SSS_SaveDatad: u8 = '1' as u8;
/// TFtdcSettArchiveStatusType是一个结算确认数据归档状态类型
///
/// 未归档数据
pub const THOST_FTDC_SAS_UnArchived: u8 = '0' as u8;
/// TFtdcSettArchiveStatusType是一个结算确认数据归档状态类型
///
/// 数据归档中
pub const THOST_FTDC_SAS_Archiving: u8 = '1' as u8;
/// TFtdcSettArchiveStatusType是一个结算确认数据归档状态类型
///
/// 已归档数据
pub const THOST_FTDC_SAS_Archived: u8 = '2' as u8;
/// TFtdcSettArchiveStatusType是一个结算确认数据归档状态类型
///
/// 归档数据失败
pub const THOST_FTDC_SAS_ArchiveFail: u8 = '3' as u8;
/// TFtdcCTPTypeType是一个CTP交易系统类型类型
///
/// 未知类型
pub const THOST_FTDC_CTPT_Unkown: u8 = '0' as u8;
/// TFtdcCTPTypeType是一个CTP交易系统类型类型
///
/// 主中心
pub const THOST_FTDC_CTPT_MainCenter: u8 = '1' as u8;
/// TFtdcCTPTypeType是一个CTP交易系统类型类型
///
/// 备中心
pub const THOST_FTDC_CTPT_BackUp: u8 = '2' as u8;
/// TFtdcCloseDealTypeType是一个平仓处理类型类型
///
/// 正常
pub const THOST_FTDC_CDT_Normal: u8 = '0' as u8;
/// TFtdcCloseDealTypeType是一个平仓处理类型类型
///
/// 投机平仓优先
pub const THOST_FTDC_CDT_SpecFirst: u8 = '1' as u8;
/// TFtdcMortgageFundUseRangeType是一个货币质押资金可用范围类型
///
/// 不能使用
pub const THOST_FTDC_MFUR_None: u8 = '0' as u8;
/// TFtdcMortgageFundUseRangeType是一个货币质押资金可用范围类型
///
/// 用于保证金
pub const THOST_FTDC_MFUR_Margin: u8 = '1' as u8;
/// TFtdcMortgageFundUseRangeType是一个货币质押资金可用范围类型
///
/// 用于手续费、盈亏、保证金
pub const THOST_FTDC_MFUR_All: u8 = '2' as u8;
/// TFtdcMortgageFundUseRangeType是一个货币质押资金可用范围类型
///
/// 人民币方案3
pub const THOST_FTDC_MFUR_CNY3: u8 = '3' as u8;
/// TFtdcSpecProductTypeType是一个特殊产品类型类型
///
/// 郑商所套保产品
pub const THOST_FTDC_SPT_CzceHedge: u8 = '1' as u8;
/// TFtdcSpecProductTypeType是一个特殊产品类型类型
///
/// 货币质押产品
pub const THOST_FTDC_SPT_IneForeignCurrency: u8 = '2' as u8;
/// TFtdcSpecProductTypeType是一个特殊产品类型类型
///
/// 大连短线开平仓产品
pub const THOST_FTDC_SPT_DceOpenClose: u8 = '3' as u8;
/// TFtdcFundMortgageTypeType是一个货币质押类型类型
///
/// 质押
pub const THOST_FTDC_FMT_Mortgage: u8 = '1' as u8;
/// TFtdcFundMortgageTypeType是一个货币质押类型类型
///
/// 解质
pub const THOST_FTDC_FMT_Redemption: u8 = '2' as u8;
/// TFtdcAccountSettlementParamIDType是一个投资者账户结算参数代码类型
///
/// 基础保证金
pub const THOST_FTDC_ASPI_BaseMargin: u8 = '1' as u8;
/// TFtdcAccountSettlementParamIDType是一个投资者账户结算参数代码类型
///
/// 最低权益标准
pub const THOST_FTDC_ASPI_LowestInterest: u8 = '2' as u8;
/// TFtdcFundMortDirectionType是一个货币质押方向类型
///
/// 货币质入
pub const THOST_FTDC_FMD_In: u8 = '1' as u8;
/// TFtdcFundMortDirectionType是一个货币质押方向类型
///
/// 货币质出
pub const THOST_FTDC_FMD_Out: u8 = '2' as u8;
/// TFtdcBusinessClassType是一个换汇类别类型
///
/// 盈利
pub const THOST_FTDC_BT_Profit: u8 = '0' as u8;
/// TFtdcBusinessClassType是一个换汇类别类型
///
/// 亏损
pub const THOST_FTDC_BT_Loss: u8 = '1' as u8;
/// TFtdcBusinessClassType是一个换汇类别类型
///
/// 其他
pub const THOST_FTDC_BT_Other: u8 = 'Z' as u8;
/// TFtdcSwapSourceTypeType是一个换汇数据来源类型
///
/// 手工
pub const THOST_FTDC_SST_Manual: u8 = '0' as u8;
/// TFtdcSwapSourceTypeType是一个换汇数据来源类型
///
/// 自动生成
pub const THOST_FTDC_SST_Automatic: u8 = '1' as u8;
/// TFtdcCurrExDirectionType是一个换汇类型类型
///
/// 结汇
pub const THOST_FTDC_CED_Settlement: u8 = '0' as u8;
/// TFtdcCurrExDirectionType是一个换汇类型类型
///
/// 售汇
pub const THOST_FTDC_CED_Sale: u8 = '1' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 已录入
pub const THOST_FTDC_CSS_Entry: u8 = '1' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 已审核
pub const THOST_FTDC_CSS_Approve: u8 = '2' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 已拒绝
pub const THOST_FTDC_CSS_Refuse: u8 = '3' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 已撤销
pub const THOST_FTDC_CSS_Revoke: u8 = '4' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 已发送
pub const THOST_FTDC_CSS_Send: u8 = '5' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 换汇成功
pub const THOST_FTDC_CSS_Success: u8 = '6' as u8;
/// TFtdcCurrencySwapStatusType是一个申请状态类型
///
/// 换汇失败
pub const THOST_FTDC_CSS_Failure: u8 = '7' as u8;
/// TFtdcReqFlagType是一个换汇发送标志类型
///
/// 未发送
pub const THOST_FTDC_REQF_NoSend: u8 = '0' as u8;
/// TFtdcReqFlagType是一个换汇发送标志类型
///
/// 发送成功
pub const THOST_FTDC_REQF_SendSuccess: u8 = '1' as u8;
/// TFtdcReqFlagType是一个换汇发送标志类型
///
/// 发送失败
pub const THOST_FTDC_REQF_SendFailed: u8 = '2' as u8;
/// TFtdcReqFlagType是一个换汇发送标志类型
///
/// 等待重发
pub const THOST_FTDC_REQF_WaitReSend: u8 = '3' as u8;
/// TFtdcResFlagType是一个换汇返回成功标志类型
///
/// 成功
pub const THOST_FTDC_RESF_Success: u8 = '0' as u8;
/// TFtdcResFlagType是一个换汇返回成功标志类型
///
/// 账户余额不足
pub const THOST_FTDC_RESF_InsuffiCient: u8 = '1' as u8;
/// TFtdcResFlagType是一个换汇返回成功标志类型
///
/// 交易结果未知
pub const THOST_FTDC_RESF_UnKnown: u8 = '8' as u8;
/// TFtdcExStatusType是一个修改状态类型
///
/// 修改前
pub const THOST_FTDC_EXS_Before: u8 = '0' as u8;
/// TFtdcExStatusType是一个修改状态类型
///
/// 修改后
pub const THOST_FTDC_EXS_After: u8 = '1' as u8;
/// TFtdcClientRegionType是一个开户客户地域类型
///
/// 国内客户
pub const THOST_FTDC_CR_Domestic: u8 = '1' as u8;
/// TFtdcClientRegionType是一个开户客户地域类型
///
/// 港澳台客户
pub const THOST_FTDC_CR_GMT: u8 = '2' as u8;
/// TFtdcClientRegionType是一个开户客户地域类型
///
/// 国外客户
pub const THOST_FTDC_CR_Foreign: u8 = '3' as u8;
/// TFtdcHasBoardType是一个是否有董事会类型
///
/// 没有
pub const THOST_FTDC_HB_No: u8 = '0' as u8;
/// TFtdcHasBoardType是一个是否有董事会类型
///
/// 有
pub const THOST_FTDC_HB_Yes: u8 = '1' as u8;
/// TFtdcStartModeType是一个启动模式类型
///
/// 正常
pub const THOST_FTDC_SM_Normal: u8 = '1' as u8;
/// TFtdcStartModeType是一个启动模式类型
///
/// 应急
pub const THOST_FTDC_SM_Emerge: u8 = '2' as u8;
/// TFtdcStartModeType是一个启动模式类型
///
/// 恢复
pub const THOST_FTDC_SM_Restore: u8 = '3' as u8;
/// TFtdcTemplateTypeType是一个模型类型类型
///
/// 全量
pub const THOST_FTDC_TPT_Full: u8 = '1' as u8;
/// TFtdcTemplateTypeType是一个模型类型类型
///
/// 增量
pub const THOST_FTDC_TPT_Increment: u8 = '2' as u8;
/// TFtdcTemplateTypeType是一个模型类型类型
///
/// 备份
pub const THOST_FTDC_TPT_BackUp: u8 = '3' as u8;
/// TFtdcLoginModeType是一个登录模式类型
///
/// 交易
pub const THOST_FTDC_LM_Trade: u8 = '0' as u8;
/// TFtdcLoginModeType是一个登录模式类型
///
/// 转账
pub const THOST_FTDC_LM_Transfer: u8 = '1' as u8;
/// TFtdcPromptTypeType是一个日历提示类型类型
///
/// 合约上下市
pub const THOST_FTDC_CPT_Instrument: u8 = '1' as u8;
/// TFtdcPromptTypeType是一个日历提示类型类型
///
/// 保证金分段生效
pub const THOST_FTDC_CPT_Margin: u8 = '2' as u8;
/// TFtdcHasTrusteeType是一个是否有托管人类型
///
/// 有
pub const THOST_FTDC_HT_Yes: u8 = '1' as u8;
/// TFtdcHasTrusteeType是一个是否有托管人类型
///
/// 没有
pub const THOST_FTDC_HT_No: u8 = '0' as u8;
/// TFtdcAmTypeType是一个机构类型类型
///
/// 银行
pub const THOST_FTDC_AMT_Bank: u8 = '1' as u8;
/// TFtdcAmTypeType是一个机构类型类型
///
/// 证券公司
pub const THOST_FTDC_AMT_Securities: u8 = '2' as u8;
/// TFtdcAmTypeType是一个机构类型类型
///
/// 基金公司
pub const THOST_FTDC_AMT_Fund: u8 = '3' as u8;
/// TFtdcAmTypeType是一个机构类型类型
///
/// 保险公司
pub const THOST_FTDC_AMT_Insurance: u8 = '4' as u8;
/// TFtdcAmTypeType是一个机构类型类型
///
/// 信托公司
pub const THOST_FTDC_AMT_Trust: u8 = '5' as u8;
/// TFtdcAmTypeType是一个机构类型类型
///
/// 其他
pub const THOST_FTDC_AMT_Other: u8 = '9' as u8;
/// TFtdcCSRCFundIOTypeType是一个出入金类型类型
///
/// 出入金
pub const THOST_FTDC_CFIOT_FundIO: u8 = '0' as u8;
/// TFtdcCSRCFundIOTypeType是一个出入金类型类型
///
/// 银期换汇
pub const THOST_FTDC_CFIOT_SwapCurrency: u8 = '1' as u8;
/// TFtdcCusAccountTypeType是一个结算账户类型类型
///
/// 期货结算账户
pub const THOST_FTDC_CAT_Futures: u8 = '1' as u8;
/// TFtdcCusAccountTypeType是一个结算账户类型类型
///
/// 纯期货资管业务下的资管结算账户
pub const THOST_FTDC_CAT_AssetmgrFuture: u8 = '2' as u8;
/// TFtdcCusAccountTypeType是一个结算账户类型类型
///
/// 综合类资管业务下的期货资管托管账户
pub const THOST_FTDC_CAT_AssetmgrTrustee: u8 = '3' as u8;
/// TFtdcCusAccountTypeType是一个结算账户类型类型
///
/// 综合类资管业务下的资金中转账户
pub const THOST_FTDC_CAT_AssetmgrTransfer: u8 = '4' as u8;
/// TFtdcLanguageTypeType是一个通知语言类型类型
///
/// 中文
pub const THOST_FTDC_LT_Chinese: u8 = '1' as u8;
/// TFtdcLanguageTypeType是一个通知语言类型类型
///
/// 英文
pub const THOST_FTDC_LT_English: u8 = '2' as u8;
/// TFtdcAssetmgrClientTypeType是一个资产管理客户类型类型
///
/// 个人资管客户
pub const THOST_FTDC_AMCT_Person: u8 = '1' as u8;
/// TFtdcAssetmgrClientTypeType是一个资产管理客户类型类型
///
/// 单位资管客户
pub const THOST_FTDC_AMCT_Organ: u8 = '2' as u8;
/// TFtdcAssetmgrClientTypeType是一个资产管理客户类型类型
///
/// 特殊单位资管客户
pub const THOST_FTDC_AMCT_SpecialOrgan: u8 = '4' as u8;
/// TFtdcAssetmgrTypeType是一个投资类型类型
///
/// 期货类
pub const THOST_FTDC_ASST_Futures: u8 = '3' as u8;
/// TFtdcAssetmgrTypeType是一个投资类型类型
///
/// 综合类
pub const THOST_FTDC_ASST_SpecialOrgan: u8 = '4' as u8;
/// TFtdcCheckInstrTypeType是一个合约比较类型类型
///
/// 合约交易所不存在
pub const THOST_FTDC_CIT_HasExch: u8 = '0' as u8;
/// TFtdcCheckInstrTypeType是一个合约比较类型类型
///
/// 合约本系统不存在
pub const THOST_FTDC_CIT_HasATP: u8 = '1' as u8;
/// TFtdcCheckInstrTypeType是一个合约比较类型类型
///
/// 合约比较不一致
pub const THOST_FTDC_CIT_HasDiff: u8 = '2' as u8;
/// TFtdcDeliveryTypeType是一个交割类型类型
///
/// 手工交割
pub const THOST_FTDC_DT_HandDeliv: u8 = '1' as u8;
/// TFtdcDeliveryTypeType是一个交割类型类型
///
/// 到期交割
pub const THOST_FTDC_DT_PersonDeliv: u8 = '2' as u8;
/// TFtdcMaxMarginSideAlgorithmType是一个大额单边保证金算法类型
///
/// 不使用大额单边保证金算法
pub const THOST_FTDC_MMSA_NO: u8 = '0' as u8;
/// TFtdcMaxMarginSideAlgorithmType是一个大额单边保证金算法类型
///
/// 使用大额单边保证金算法
pub const THOST_FTDC_MMSA_YES: u8 = '1' as u8;
/// TFtdcDAClientTypeType是一个资产管理客户类型类型
///
/// 自然人
pub const THOST_FTDC_CACT_Person: u8 = '0' as u8;
/// TFtdcDAClientTypeType是一个资产管理客户类型类型
///
/// 法人
pub const THOST_FTDC_CACT_Company: u8 = '1' as u8;
/// TFtdcDAClientTypeType是一个资产管理客户类型类型
///
/// 其他
pub const THOST_FTDC_CACT_Other: u8 = '2' as u8;
/// TFtdcUOAAssetmgrTypeType是一个投资类型类型
///
/// 期货类
pub const THOST_FTDC_UOAAT_Futures: u8 = '1' as u8;
/// TFtdcUOAAssetmgrTypeType是一个投资类型类型
///
/// 综合类
pub const THOST_FTDC_UOAAT_SpecialOrgan: u8 = '2' as u8;
/// TFtdcDirectionEnType是一个买卖方向类型
///
/// Buy
pub const THOST_FTDC_DEN_Buy: u8 = '0' as u8;
/// TFtdcDirectionEnType是一个买卖方向类型
///
/// Sell
pub const THOST_FTDC_DEN_Sell: u8 = '1' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Position Opening
pub const THOST_FTDC_OFEN_Open: u8 = '0' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Position Close
pub const THOST_FTDC_OFEN_Close: u8 = '1' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Forced Liquidation
pub const THOST_FTDC_OFEN_ForceClose: u8 = '2' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Close Today
pub const THOST_FTDC_OFEN_CloseToday: u8 = '3' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Close Prev.
pub const THOST_FTDC_OFEN_CloseYesterday: u8 = '4' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Forced Reduction
pub const THOST_FTDC_OFEN_ForceOff: u8 = '5' as u8;
/// TFtdcOffsetFlagEnType是一个开平标志类型
///
/// Local Forced Liquidation
pub const THOST_FTDC_OFEN_LocalForceClose: u8 = '6' as u8;
/// TFtdcHedgeFlagEnType是一个投机套保标志类型
///
/// Speculation
pub const THOST_FTDC_HFEN_Speculation: u8 = '1' as u8;
/// TFtdcHedgeFlagEnType是一个投机套保标志类型
///
/// Arbitrage
pub const THOST_FTDC_HFEN_Arbitrage: u8 = '2' as u8;
/// TFtdcHedgeFlagEnType是一个投机套保标志类型
///
/// Hedge
pub const THOST_FTDC_HFEN_Hedge: u8 = '3' as u8;
/// TFtdcFundIOTypeEnType是一个出入金类型类型
///
/// Deposit/Withdrawal
pub const THOST_FTDC_FIOTEN_FundIO: u8 = '1' as u8;
/// TFtdcFundIOTypeEnType是一个出入金类型类型
///
/// Bank-Futures Transfer
pub const THOST_FTDC_FIOTEN_Transfer: u8 = '2' as u8;
/// TFtdcFundIOTypeEnType是一个出入金类型类型
///
/// Bank-Futures FX Exchange
pub const THOST_FTDC_FIOTEN_SwapCurrency: u8 = '3' as u8;
/// TFtdcFundTypeEnType是一个资金类型类型
///
/// Bank Deposit
pub const THOST_FTDC_FTEN_Deposite: u8 = '1' as u8;
/// TFtdcFundTypeEnType是一个资金类型类型
///
/// Payment/Fee
pub const THOST_FTDC_FTEN_ItemFund: u8 = '2' as u8;
/// TFtdcFundTypeEnType是一个资金类型类型
///
/// Brokerage Adj
pub const THOST_FTDC_FTEN_Company: u8 = '3' as u8;
/// TFtdcFundTypeEnType是一个资金类型类型
///
/// Internal Transfer
pub const THOST_FTDC_FTEN_InnerTransfer: u8 = '4' as u8;
/// TFtdcFundDirectionEnType是一个出入金方向类型
///
/// Deposit
pub const THOST_FTDC_FDEN_In: u8 = '1' as u8;
/// TFtdcFundDirectionEnType是一个出入金方向类型
///
/// Withdrawal
pub const THOST_FTDC_FDEN_Out: u8 = '2' as u8;
/// TFtdcFundMortDirectionEnType是一个货币质押方向类型
///
/// Pledge
pub const THOST_FTDC_FMDEN_In: u8 = '1' as u8;
/// TFtdcFundMortDirectionEnType是一个货币质押方向类型
///
/// Redemption
pub const THOST_FTDC_FMDEN_Out: u8 = '2' as u8;
/// TFtdcOptionsTypeType是一个期权类型类型
///
/// 看涨
pub const THOST_FTDC_CP_CallOptions: u8 = '1' as u8;
/// TFtdcOptionsTypeType是一个期权类型类型
///
/// 看跌
pub const THOST_FTDC_CP_PutOptions: u8 = '2' as u8;
/// TFtdcStrikeModeType是一个执行方式类型
///
/// 欧式
pub const THOST_FTDC_STM_Continental: u8 = '0' as u8;
/// TFtdcStrikeModeType是一个执行方式类型
///
/// 美式
pub const THOST_FTDC_STM_American: u8 = '1' as u8;
/// TFtdcStrikeModeType是一个执行方式类型
///
/// 百慕大
pub const THOST_FTDC_STM_Bermuda: u8 = '2' as u8;
/// TFtdcStrikeTypeType是一个执行类型类型
///
/// 自身对冲
pub const THOST_FTDC_STT_Hedge: u8 = '0' as u8;
/// TFtdcStrikeTypeType是一个执行类型类型
///
/// 匹配执行
pub const THOST_FTDC_STT_Match: u8 = '1' as u8;
/// TFtdcApplyTypeType是一个中金所期权放弃执行申请类型类型
///
/// 不执行数量
pub const THOST_FTDC_APPT_NotStrikeNum: u8 = '4' as u8;
/// TFtdcGiveUpDataSourceType是一个放弃执行申请数据来源类型
///
/// 系统生成
pub const THOST_FTDC_GUDS_Gen: u8 = '0' as u8;
/// TFtdcGiveUpDataSourceType是一个放弃执行申请数据来源类型
///
/// 手工添加
pub const THOST_FTDC_GUDS_Hand: u8 = '1' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 没有执行
pub const THOST_FTDC_OER_NoExec: u8 = 'n' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 已经取消
pub const THOST_FTDC_OER_Canceled: u8 = 'c' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 执行成功
pub const THOST_FTDC_OER_OK: u8 = '0' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 期权持仓不够
pub const THOST_FTDC_OER_NoPosition: u8 = '1' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 资金不够
pub const THOST_FTDC_OER_NoDeposit: u8 = '2' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 会员不存在
pub const THOST_FTDC_OER_NoParticipant: u8 = '3' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 客户不存在
pub const THOST_FTDC_OER_NoClient: u8 = '4' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 合约不存在
pub const THOST_FTDC_OER_NoInstrument: u8 = '6' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 没有执行权限
pub const THOST_FTDC_OER_NoRight: u8 = '7' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 不合理的数量
pub const THOST_FTDC_OER_InvalidVolume: u8 = '8' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 没有足够的历史成交
pub const THOST_FTDC_OER_NoEnoughHistoryTrade: u8 = '9' as u8;
/// TFtdcExecResultType是一个执行结果类型
///
/// 未知
pub const THOST_FTDC_OER_Unknown: u8 = 'a' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 期货组合
pub const THOST_FTDC_COMBT_Future: u8 = '0' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 垂直价差BUL
pub const THOST_FTDC_COMBT_BUL: u8 = '1' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 垂直价差BER
pub const THOST_FTDC_COMBT_BER: u8 = '2' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 跨式组合
pub const THOST_FTDC_COMBT_STD: u8 = '3' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 宽跨式组合
pub const THOST_FTDC_COMBT_STG: u8 = '4' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 备兑组合
pub const THOST_FTDC_COMBT_PRT: u8 = '5' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 时间价差组合
pub const THOST_FTDC_COMBT_CAS: u8 = '6' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 期权对锁组合
pub const THOST_FTDC_COMBT_OPL: u8 = '7' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 买备兑组合
pub const THOST_FTDC_COMBT_BFO: u8 = '8' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 买入期权垂直价差组合
pub const THOST_FTDC_COMBT_BLS: u8 = '9' as u8;
/// TFtdcCombinationTypeType是一个组合类型类型
///
/// 卖出期权垂直价差组合
pub const THOST_FTDC_COMBT_BES: u8 = 'a' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期货对锁组合
pub const THOST_FTDC_DCECOMBT_SPL: u8 = '0' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期权对锁组合
pub const THOST_FTDC_DCECOMBT_OPL: u8 = '1' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期货跨期组合
pub const THOST_FTDC_DCECOMBT_SP: u8 = '2' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期货跨品种组合
pub const THOST_FTDC_DCECOMBT_SPC: u8 = '3' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 买入期权垂直价差组合
pub const THOST_FTDC_DCECOMBT_BLS: u8 = '4' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 卖出期权垂直价差组合
pub const THOST_FTDC_DCECOMBT_BES: u8 = '5' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期权日历价差组合
pub const THOST_FTDC_DCECOMBT_CAS: u8 = '6' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期权跨式组合
pub const THOST_FTDC_DCECOMBT_STD: u8 = '7' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 期权宽跨式组合
pub const THOST_FTDC_DCECOMBT_STG: u8 = '8' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 买入期货期权组合
pub const THOST_FTDC_DCECOMBT_BFO: u8 = '9' as u8;
/// TFtdcDceCombinationTypeType是一个组合类型类型
///
/// 卖出期货期权组合
pub const THOST_FTDC_DCECOMBT_SFO: u8 = 'a' as u8;
/// TFtdcOptionRoyaltyPriceTypeType是一个期权权利金价格类型类型
///
/// 昨结算价
pub const THOST_FTDC_ORPT_PreSettlementPrice: u8 = '1' as u8;
/// TFtdcOptionRoyaltyPriceTypeType是一个期权权利金价格类型类型
///
/// 开仓价
pub const THOST_FTDC_ORPT_OpenPrice: u8 = '4' as u8;
/// TFtdcOptionRoyaltyPriceTypeType是一个期权权利金价格类型类型
///
/// 最新价与昨结算价较大值
pub const THOST_FTDC_ORPT_MaxPreSettlementPrice: u8 = '5' as u8;
/// TFtdcBalanceAlgorithmType是一个权益算法类型
///
/// 不计算期权市值盈亏
pub const THOST_FTDC_BLAG_Default: u8 = '1' as u8;
/// TFtdcBalanceAlgorithmType是一个权益算法类型
///
/// 计算期权市值亏损
pub const THOST_FTDC_BLAG_IncludeOptValLost: u8 = '2' as u8;
/// TFtdcActionTypeType是一个执行类型类型
///
/// 执行
pub const THOST_FTDC_ACTP_Exec: u8 = '1' as u8;
/// TFtdcActionTypeType是一个执行类型类型
///
/// 放弃
pub const THOST_FTDC_ACTP_Abandon: u8 = '2' as u8;
/// TFtdcForQuoteStatusType是一个询价状态类型
///
/// 已经提交
pub const THOST_FTDC_FQST_Submitted: u8 = 'a' as u8;
/// TFtdcForQuoteStatusType是一个询价状态类型
///
/// 已经接受
pub const THOST_FTDC_FQST_Accepted: u8 = 'b' as u8;
/// TFtdcForQuoteStatusType是一个询价状态类型
///
/// 已经被拒绝
pub const THOST_FTDC_FQST_Rejected: u8 = 'c' as u8;
/// TFtdcValueMethodType是一个取值方式类型
///
/// 按绝对值
pub const THOST_FTDC_VM_Absolute: u8 = '0' as u8;
/// TFtdcValueMethodType是一个取值方式类型
///
/// 按比率
pub const THOST_FTDC_VM_Ratio: u8 = '1' as u8;
/// TFtdcExecOrderPositionFlagType是一个期权行权后是否保留期货头寸的标记类型
///
/// 保留
pub const THOST_FTDC_EOPF_Reserve: u8 = '0' as u8;
/// TFtdcExecOrderPositionFlagType是一个期权行权后是否保留期货头寸的标记类型
///
/// 不保留
pub const THOST_FTDC_EOPF_UnReserve: u8 = '1' as u8;
/// TFtdcExecOrderCloseFlagType是一个期权行权后生成的头寸是否自动平仓类型
///
/// 自动平仓
pub const THOST_FTDC_EOCF_AutoClose: u8 = '0' as u8;
/// TFtdcExecOrderCloseFlagType是一个期权行权后生成的头寸是否自动平仓类型
///
/// 免于自动平仓
pub const THOST_FTDC_EOCF_NotToClose: u8 = '1' as u8;
/// TFtdcProductTypeType是一个产品类型类型
///
/// 期货
pub const THOST_FTDC_PTE_Futures: u8 = '1' as u8;
/// TFtdcProductTypeType是一个产品类型类型
///
/// 期权
pub const THOST_FTDC_PTE_Options: u8 = '2' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}_zz_\d{4}
pub const THOST_FTDC_CUFN_CUFN_O: u8 = 'O' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}成交表
pub const THOST_FTDC_CUFN_CUFN_T: u8 = 'T' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}单腿持仓表new
pub const THOST_FTDC_CUFN_CUFN_P: u8 = 'P' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}非平仓了结表
pub const THOST_FTDC_CUFN_CUFN_N: u8 = 'N' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}平仓表
pub const THOST_FTDC_CUFN_CUFN_L: u8 = 'L' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}资金表
pub const THOST_FTDC_CUFN_CUFN_F: u8 = 'F' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}组合持仓表
pub const THOST_FTDC_CUFN_CUFN_C: u8 = 'C' as u8;
/// TFtdcCZCEUploadFileNameType是一个郑商所结算文件名类型
///
/// ^\d{8}保证金参数表
pub const THOST_FTDC_CUFN_CUFN_M: u8 = 'M' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_dl_\d{3}
pub const THOST_FTDC_DUFN_DUFN_O: u8 = 'O' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_成交表
pub const THOST_FTDC_DUFN_DUFN_T: u8 = 'T' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_持仓表
pub const THOST_FTDC_DUFN_DUFN_P: u8 = 'P' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_资金结算表
pub const THOST_FTDC_DUFN_DUFN_F: u8 = 'F' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_优惠组合持仓明细表
pub const THOST_FTDC_DUFN_DUFN_C: u8 = 'C' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_持仓明细表
pub const THOST_FTDC_DUFN_DUFN_D: u8 = 'D' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_保证金参数表
pub const THOST_FTDC_DUFN_DUFN_M: u8 = 'M' as u8;
/// TFtdcDCEUploadFileNameType是一个大商所结算文件名类型
///
/// ^\d{8}_期权执行表
pub const THOST_FTDC_DUFN_DUFN_S: u8 = 'S' as u8;
/// TFtdcSHFEUploadFileNameType是一个上期所结算文件名类型
///
/// ^\d{4}_\d{8}_\d{8}_DailyFundChg
pub const THOST_FTDC_SUFN_SUFN_O: u8 = 'O' as u8;
/// TFtdcSHFEUploadFileNameType是一个上期所结算文件名类型
///
/// ^\d{4}_\d{8}_\d{8}_Trade
pub const THOST_FTDC_SUFN_SUFN_T: u8 = 'T' as u8;
/// TFtdcSHFEUploadFileNameType是一个上期所结算文件名类型
///
/// ^\d{4}_\d{8}_\d{8}_SettlementDetail
pub const THOST_FTDC_SUFN_SUFN_P: u8 = 'P' as u8;
/// TFtdcSHFEUploadFileNameType是一个上期所结算文件名类型
///
/// ^\d{4}_\d{8}_\d{8}_Capital
pub const THOST_FTDC_SUFN_SUFN_F: u8 = 'F' as u8;
/// TFtdcCFFEXUploadFileNameType是一个中金所结算文件名类型
///
/// ^\d{4}_SG\d{1}_\d{8}_\d{1}_Trade
pub const THOST_FTDC_CFUFN_SUFN_T: u8 = 'T' as u8;
/// TFtdcCFFEXUploadFileNameType是一个中金所结算文件名类型
///
/// ^\d{4}_SG\d{1}_\d{8}_\d{1}_SettlementDetail
pub const THOST_FTDC_CFUFN_SUFN_P: u8 = 'P' as u8;
/// TFtdcCFFEXUploadFileNameType是一个中金所结算文件名类型
///
/// ^\d{4}_SG\d{1}_\d{8}_\d{1}_Capital
pub const THOST_FTDC_CFUFN_SUFN_F: u8 = 'F' as u8;
/// TFtdcCFFEXUploadFileNameType是一个中金所结算文件名类型
///
/// ^\d{4}_SG\d{1}_\d{8}_\d{1}_OptionExec
pub const THOST_FTDC_CFUFN_SUFN_S: u8 = 'S' as u8;
/// TFtdcCombDirectionType是一个组合指令方向类型
///
/// 申请组合
pub const THOST_FTDC_CMDR_Comb: u8 = '0' as u8;
/// TFtdcCombDirectionType是一个组合指令方向类型
///
/// 申请拆分
pub const THOST_FTDC_CMDR_UnComb: u8 = '1' as u8;
/// TFtdcCombDirectionType是一个组合指令方向类型
///
/// 操作员删组合单
pub const THOST_FTDC_CMDR_DelComb: u8 = '2' as u8;
/// TFtdcStrikeOffsetTypeType是一个行权偏移类型类型
///
/// 实值额
pub const THOST_FTDC_STOV_RealValue: u8 = '1' as u8;
/// TFtdcStrikeOffsetTypeType是一个行权偏移类型类型
///
/// 盈利额
pub const THOST_FTDC_STOV_ProfitValue: u8 = '2' as u8;
/// TFtdcStrikeOffsetTypeType是一个行权偏移类型类型
///
/// 实值比例
pub const THOST_FTDC_STOV_RealRatio: u8 = '3' as u8;
/// TFtdcStrikeOffsetTypeType是一个行权偏移类型类型
///
/// 盈利比例
pub const THOST_FTDC_STOV_ProfitRatio: u8 = '4' as u8;
/// TFtdcReserveOpenAccStasType是一个预约开户状态类型
///
/// 等待处理中
pub const THOST_FTDC_ROAST_Processing: u8 = '0' as u8;
/// TFtdcReserveOpenAccStasType是一个预约开户状态类型
///
/// 已撤销
pub const THOST_FTDC_ROAST_Cancelled: u8 = '1' as u8;
/// TFtdcReserveOpenAccStasType是一个预约开户状态类型
///
/// 已开户
pub const THOST_FTDC_ROAST_Opened: u8 = '2' as u8;
/// TFtdcReserveOpenAccStasType是一个预约开户状态类型
///
/// 无效请求
pub const THOST_FTDC_ROAST_Invalid: u8 = '3' as u8;
/// TFtdcWeakPasswordSourceType是一个弱密码来源类型
///
/// 弱密码库
pub const THOST_FTDC_WPSR_Lib: u8 = '1' as u8;
/// TFtdcWeakPasswordSourceType是一个弱密码来源类型
///
/// 手工录入
pub const THOST_FTDC_WPSR_Manual: u8 = '2' as u8;
/// TFtdcOptSelfCloseFlagType是一个期权行权的头寸是否自对冲类型
///
/// 自对冲期权仓位
pub const THOST_FTDC_OSCF_CloseSelfOptionPosition: u8 = '1' as u8;
/// TFtdcOptSelfCloseFlagType是一个期权行权的头寸是否自对冲类型
///
/// 保留期权仓位
pub const THOST_FTDC_OSCF_ReserveOptionPosition: u8 = '2' as u8;
/// TFtdcOptSelfCloseFlagType是一个期权行权的头寸是否自对冲类型
///
/// 自对冲卖方履约后的期货仓位
pub const THOST_FTDC_OSCF_SellCloseSelfFuturePosition: u8 = '3' as u8;
/// TFtdcOptSelfCloseFlagType是一个期权行权的头寸是否自对冲类型
///
/// 保留卖方履约后的期货仓位
pub const THOST_FTDC_OSCF_ReserveFuturePosition: u8 = '4' as u8;
/// TFtdcBizTypeType是一个业务类型类型
///
/// 期货
pub const THOST_FTDC_BZTP_Future: u8 = '1' as u8;
/// TFtdcBizTypeType是一个业务类型类型
///
/// 证券
pub const THOST_FTDC_BZTP_Stock: u8 = '2' as u8;
/// TFtdcAppTypeType是一个用户App类型类型
///
/// 直连的投资者
pub const THOST_FTDC_APP_TYPE_Investor: u8 = '1' as u8;
/// TFtdcAppTypeType是一个用户App类型类型
///
/// 为每个投资者都创建连接的中继
pub const THOST_FTDC_APP_TYPE_InvestorRelay: u8 = '2' as u8;
/// TFtdcAppTypeType是一个用户App类型类型
///
/// 所有投资者共享一个操作员连接的中继
pub const THOST_FTDC_APP_TYPE_OperatorRelay: u8 = '3' as u8;
/// TFtdcAppTypeType是一个用户App类型类型
///
/// 未知
pub const THOST_FTDC_APP_TYPE_UnKnown: u8 = '4' as u8;
/// TFtdcResponseValueType是一个应答类型类型
///
/// 检查成功
pub const THOST_FTDC_RV_Right: u8 = '0' as u8;
/// TFtdcResponseValueType是一个应答类型类型
///
/// 检查失败
pub const THOST_FTDC_RV_Refuse: u8 = '1' as u8;
/// TFtdcOTCTradeTypeType是一个OTC成交类型类型
///
/// 大宗交易
pub const THOST_FTDC_OTC_TRDT_Block: u8 = '0' as u8;
/// TFtdcOTCTradeTypeType是一个OTC成交类型类型
///
/// 期转现
pub const THOST_FTDC_OTC_TRDT_EFP: u8 = '1' as u8;
/// TFtdcMatchTypeType是一个期现风险匹配方式类型
///
/// 基点价值
pub const THOST_FTDC_OTC_MT_DV01: u8 = '1' as u8;
/// TFtdcMatchTypeType是一个期现风险匹配方式类型
///
/// 面值
pub const THOST_FTDC_OTC_MT_ParValue: u8 = '2' as u8;
/// TFtdcAuthTypeType是一个用户终端认证方式类型
///
/// 白名单校验
pub const THOST_FTDC_AU_WHITE: u8 = '0' as u8;
/// TFtdcAuthTypeType是一个用户终端认证方式类型
///
/// 黑名单校验
pub const THOST_FTDC_AU_BLACK: u8 = '1' as u8;
/// TFtdcClassTypeType是一个合约分类方式类型
///
/// 所有合约
pub const THOST_FTDC_INS_ALL: u8 = '0' as u8;
/// TFtdcClassTypeType是一个合约分类方式类型
///
/// 期货、即期、期转现、Tas、金属指数合约
pub const THOST_FTDC_INS_FUTURE: u8 = '1' as u8;
/// TFtdcClassTypeType是一个合约分类方式类型
///
/// 期货、现货期权合约
pub const THOST_FTDC_INS_OPTION: u8 = '2' as u8;
/// TFtdcClassTypeType是一个合约分类方式类型
///
/// 组合合约
pub const THOST_FTDC_INS_COMB: u8 = '3' as u8;
/// TFtdcTradingTypeType是一个合约交易状态分类方式类型
///
/// 所有状态
pub const THOST_FTDC_TD_ALL: u8 = '0' as u8;
/// TFtdcTradingTypeType是一个合约交易状态分类方式类型
///
/// 交易
pub const THOST_FTDC_TD_TRADE: u8 = '1' as u8;
/// TFtdcTradingTypeType是一个合约交易状态分类方式类型
///
/// 非交易
pub const THOST_FTDC_TD_UNTRADE: u8 = '2' as u8;
/// TFtdcProductStatusType是一个产品状态类型
///
/// 可交易
pub const THOST_FTDC_PS_tradeable: u8 = '1' as u8;
/// TFtdcProductStatusType是一个产品状态类型
///
/// 不可交易
pub const THOST_FTDC_PS_untradeable: u8 = '2' as u8;
/// TFtdcSyncDeltaStatusType是一个追平状态类型
///
/// 交易可读
pub const THOST_FTDC_SDS_Readable: u8 = '1' as u8;
/// TFtdcSyncDeltaStatusType是一个追平状态类型
///
/// 交易在读
pub const THOST_FTDC_SDS_Reading: u8 = '2' as u8;
/// TFtdcSyncDeltaStatusType是一个追平状态类型
///
/// 交易读取完成
pub const THOST_FTDC_SDS_Readend: u8 = '3' as u8;
/// TFtdcSyncDeltaStatusType是一个追平状态类型
///
/// 追平失败 交易本地状态结算不存在
pub const THOST_FTDC_SDS_OptErr: u8 = 'e' as u8;
/// TFtdcActionDirectionType是一个操作标志类型
///
/// 增加
pub const THOST_FTDC_ACD_Add: u8 = '1' as u8;
/// TFtdcActionDirectionType是一个操作标志类型
///
/// 删除
pub const THOST_FTDC_ACD_Del: u8 = '2' as u8;
/// TFtdcActionDirectionType是一个操作标志类型
///
/// 更新
pub const THOST_FTDC_ACD_Upd: u8 = '3' as u8;
/// TFtdcOrderCancelAlgType是一个撤单时选择席位算法类型
///
/// 轮询席位撤单
pub const THOST_FTDC_OAC_Balance: u8 = '1' as u8;
/// TFtdcOrderCancelAlgType是一个撤单时选择席位算法类型
///
/// 优先原报单席位撤单
pub const THOST_FTDC_OAC_OrigFirst: u8 = '2' as u8;
/// TFtdcOpenLimitControlLevelType是一个开仓量限制粒度类型
///
/// 不控制
pub const THOST_FTDC_PLCL_None: u8 = '0' as u8;
/// TFtdcOpenLimitControlLevelType是一个开仓量限制粒度类型
///
/// 产品级别
pub const THOST_FTDC_PLCL_Product: u8 = '1' as u8;
/// TFtdcOpenLimitControlLevelType是一个开仓量限制粒度类型
///
/// 合约级别
pub const THOST_FTDC_PLCL_Inst: u8 = '2' as u8;
/// TFtdcOrderFreqControlLevelType是一个报单频率控制粒度类型
///
/// 不控制
pub const THOST_FTDC_OFCL_None: u8 = '0' as u8;
/// TFtdcOrderFreqControlLevelType是一个报单频率控制粒度类型
///
/// 产品级别
pub const THOST_FTDC_OFCL_Product: u8 = '1' as u8;
/// TFtdcOrderFreqControlLevelType是一个报单频率控制粒度类型
///
/// 合约级别
pub const THOST_FTDC_OFCL_Inst: u8 = '2' as u8;
/// TFtdcEnumBoolType是一个枚举bool类型类型
///
/// false
pub const THOST_FTDC_EBL_False: u8 = '0' as u8;
/// TFtdcEnumBoolType是一个枚举bool类型类型
///
/// true
pub const THOST_FTDC_EBL_True: u8 = '1' as u8;
/// TFtdcTimeRangeType是一个期货合约阶段标识类型
///
/// 一般月份
pub const THOST_FTDC_ETR_USUAL: u8 = '1' as u8;
/// TFtdcTimeRangeType是一个期货合约阶段标识类型
///
/// 交割月前一个月上半月
pub const THOST_FTDC_ETR_FNSP: u8 = '2' as u8;
/// TFtdcTimeRangeType是一个期货合约阶段标识类型
///
/// 交割月前一个月下半月
pub const THOST_FTDC_ETR_BNSP: u8 = '3' as u8;
/// TFtdcTimeRangeType是一个期货合约阶段标识类型
///
/// 交割月份
pub const THOST_FTDC_ETR_SPOT: u8 = '4' as u8;
/// TFtdcPortfolioType是一个新型组保算法类型
///
/// 不使用新型组保算法
pub const THOST_FTDC_EPF_None: u8 = '0' as u8;
/// TFtdcPortfolioType是一个新型组保算法类型
///
/// SPBM算法
pub const THOST_FTDC_EPF_SPBM: u8 = '1' as u8;
/// TFtdcPortfolioType是一个新型组保算法类型
///
/// RULE算法
pub const THOST_FTDC_EPF_RULE: u8 = '2' as u8;
/// TFtdcPortfolioType是一个新型组保算法类型
///
/// SPMM算法
pub const THOST_FTDC_EPF_SPMM: u8 = '3' as u8;
/// TFtdcPortfolioType是一个新型组保算法类型
///
/// RCAMS算法
pub const THOST_FTDC_EPF_RCAMS: u8 = '4' as u8;
/// TFtdcWithDrawParamIDType是一个可提参数代码类型
///
/// 权利金收支是否可提 1 代表可提 0 不可提
pub const THOST_FTDC_WDPID_CashIn: u8 = 'C' as u8;
/// TFtdcInvstTradingRightType是一个投资者交易权限类型
///
/// 只能平仓
pub const THOST_FTDC_ITR_CloseOnly: u8 = '1' as u8;
/// TFtdcInvstTradingRightType是一个投资者交易权限类型
///
/// 不能交易
pub const THOST_FTDC_ITR_Forbidden: u8 = '2' as u8;
/// TFtdcInstMarginCalIDType是一个SPMM合约保证金算法类型
///
/// 标准算法收取双边
pub const THOST_FTDC_IMID_BothSide: u8 = '1' as u8;
/// TFtdcInstMarginCalIDType是一个SPMM合约保证金算法类型
///
/// 单向大边
pub const THOST_FTDC_IMID_MMSA: u8 = '2' as u8;
/// TFtdcInstMarginCalIDType是一个SPMM合约保证金算法类型
///
/// 新组保SPMM
pub const THOST_FTDC_IMID_SPMM: u8 = '3' as u8;
/// TFtdcRCAMSCombinationTypeType是一个RCAMS组合类型类型
///
/// 牛市看涨价差组合
pub const THOST_FTDC_ERComb_BUC: u8 = '0' as u8;
/// TFtdcRCAMSCombinationTypeType是一个RCAMS组合类型类型
///
/// 熊市看涨价差组合
pub const THOST_FTDC_ERComb_BEC: u8 = '1' as u8;
/// TFtdcRCAMSCombinationTypeType是一个RCAMS组合类型类型
///
/// 熊市看跌价差组合
pub const THOST_FTDC_ERComb_BEP: u8 = '2' as u8;
/// TFtdcRCAMSCombinationTypeType是一个RCAMS组合类型类型
///
/// 牛市看跌价差组合
pub const THOST_FTDC_ERComb_BUP: u8 = '3' as u8;
/// TFtdcRCAMSCombinationTypeType是一个RCAMS组合类型类型
///
/// 日历价差组合
pub const THOST_FTDC_ERComb_CAS: u8 = '4' as u8;
/// TFtdcPortfTypeType是一个新组保算法启用类型类型
///
/// 使用初版交易所算法
pub const THOST_FTDC_EET_None: u8 = '0' as u8;
/// TFtdcPortfTypeType是一个新组保算法启用类型类型
///
/// SPBM算法V1.1.0_附加保证金调整
pub const THOST_FTDC_EET_SPBM_AddOnHedge: u8 = '1' as u8;
/// TFtdcInstrumentClassType是一个合约类型类型
///
/// 一般月份合约
pub const THOST_FTDC_EIC_Usual: u8 = '1' as u8;
/// TFtdcInstrumentClassType是一个合约类型类型
///
/// 临近交割合约
pub const THOST_FTDC_EIC_Delivery: u8 = '2' as u8;
/// TFtdcInstrumentClassType是一个合约类型类型
///
/// 非组合合约
pub const THOST_FTDC_EIC_NonComb: u8 = '3' as u8;
/// TFtdcProdChangeFlagType是一个品种记录改变状态类型
///
/// 持仓量和冻结量均无变化
pub const THOST_FTDC_PCF_None: u8 = '0' as u8;
/// TFtdcProdChangeFlagType是一个品种记录改变状态类型
///
/// 持仓量无变化，冻结量有变化
pub const THOST_FTDC_PCF_OnlyFrozen: u8 = '1' as u8;
/// TFtdcProdChangeFlagType是一个品种记录改变状态类型
///
/// 持仓量有变化
pub const THOST_FTDC_PCF_PositionChange: u8 = '2' as u8;
/// TFtdcPwdRcdSrcType是一个历史密码来源类型
///
/// 来源于Sync初始化数据
pub const THOST_FTDC_PRS_Init: u8 = '0' as u8;
/// TFtdcPwdRcdSrcType是一个历史密码来源类型
///
/// 来源于实时上场数据
pub const THOST_FTDC_PRS_Sync: u8 = '1' as u8;
/// TFtdcPwdRcdSrcType是一个历史密码来源类型
///
/// 来源于用户修改
pub const THOST_FTDC_PRS_UserUpd: u8 = '2' as u8;
/// TFtdcPwdRcdSrcType是一个历史密码来源类型
///
/// 来源于超户修改，很可能来自主席同步数据
pub const THOST_FTDC_PRS_SuperUserUpd: u8 = '3' as u8;
/// TFtdcPwdRcdSrcType是一个历史密码来源类型
///
/// 来源于次席同步的修改
pub const THOST_FTDC_PRS_SecUpd: u8 = '4' as u8;
/// TFtdcAddrSrvModeType是一个地址服务类型类型
///
/// 交易地址
pub const THOST_FTDC_ASM_Trade: u8 = '0' as u8;
/// TFtdcAddrSrvModeType是一个地址服务类型类型
///
/// 行情地址
pub const THOST_FTDC_ASM_MarketData: u8 = '1' as u8;
/// TFtdcAddrSrvModeType是一个地址服务类型类型
///
/// 其他
pub const THOST_FTDC_ASM_Other: u8 = '2' as u8;
/// TFtdcAddrVerType是一个地址版本类型
///
/// IPV4
pub const THOST_FTDC_ADV_V4: u8 = '0' as u8;
/// TFtdcAddrVerType是一个地址版本类型
///
/// IPV6
pub const THOST_FTDC_ADV_V6: u8 = '1' as u8;
/// TFtdcTGSessionQryStatusType是一个TGATE会话查询状态类型
///
/// 查询状态空闲
pub const THOST_FTDC_TGQS_QryIdle: u8 = '1' as u8;
/// TFtdcTGSessionQryStatusType是一个TGATE会话查询状态类型
///
/// 查询状态频繁
pub const THOST_FTDC_TGQS_QryBusy: u8 = '2' as u8;
/// TFtdcOffsetTypeType是一个对冲类型类型
///
/// 期权对冲
pub const THOST_FTDC_OT_OPT_OFFSET: u8 = '0' as u8;
/// TFtdcOffsetTypeType是一个对冲类型类型
///
/// 期货对冲
pub const THOST_FTDC_OT_FUT_OFFSET: u8 = '1' as u8;
/// TFtdcOffsetTypeType是一个对冲类型类型
///
/// 行权后期货对冲
pub const THOST_FTDC_OT_EXEC_OFFSET: u8 = '2' as u8;
/// TFtdcOffsetTypeType是一个对冲类型类型
///
/// 履约后期货对冲
pub const THOST_FTDC_OT_PERFORM_OFFSET: u8 = '3' as u8;
/// TFtdcApplySrcType是一个申请来源类型
///
/// 交易
pub const THOST_FTDC_AS_Trade: u8 = '0' as u8;
/// TFtdcApplySrcType是一个申请来源类型
///
/// 会服
pub const THOST_FTDC_AS_Member: u8 = '1' as u8;

unsafe impl Send for MdApi {}
unsafe impl Sync for MdApi {}

impl MdApi {
    pub fn CreateMdApiAndSpi(tx: Sender<MdSpiMsg>, flow_path: String, is_using_udp: bool, is_multicast: bool, is_production_mode: bool) -> UniquePtr<MdApi> {
        if !Path::new(&flow_path).exists() {
            create_dir_all(&flow_path).unwrap();
        }
        CreateMdApi(Box::new(MdSpi { tx }), flow_path, is_using_udp, is_multicast, is_production_mode)
    }
}

#[derive(Debug, Clone)]
pub enum MdSpiMsg {
    OnFrontConnected,
    OnFrontDisconnected(i32),
    OnHeartBeatWarning(i32),
    OnRspUserLogin(Box<RspUserLoginField>, Box<RspInfoField>, i32, bool),
    OnRspUserLogout(Box<UserLogoutField>, Box<RspInfoField>, i32, bool),
    OnRspQryMulticastInstrument(Box<MulticastInstrumentField>, Box<RspInfoField>, i32, bool),
    OnRspError(Box<RspInfoField>, i32, bool),
    OnRspSubMarketData(Box<SpecificInstrumentField>, Box<RspInfoField>, i32, bool),
    OnRspUnSubMarketData(Box<SpecificInstrumentField>, Box<RspInfoField>, i32, bool),
    OnRspSubForQuoteRsp(Box<SpecificInstrumentField>, Box<RspInfoField>, i32, bool),
    OnRspUnSubForQuoteRsp(Box<SpecificInstrumentField>, Box<RspInfoField>, i32, bool),
    OnRtnDepthMarketData(Box<DepthMarketDataField>),
    OnRtnForQuoteRsp(Box<ForQuoteRspField>),
}

pub struct MdSpi {
    tx: Sender<MdSpiMsg>,
}

impl MdSpi {
pub fn OnFrontConnected(&self) { self.tx.send(MdSpiMsg::OnFrontConnected).ok(); }
pub fn OnFrontDisconnected(&self, nReason: i32) { self.tx.send(MdSpiMsg::OnFrontDisconnected(nReason)).ok(); }
pub fn OnHeartBeatWarning(&self, nTimeLapse: i32) { self.tx.send(MdSpiMsg::OnHeartBeatWarning(nTimeLapse)).ok(); }
pub fn OnRspUserLogin(&self, pRspUserLogin: RspUserLoginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUserLogin(Box::new(pRspUserLogin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUserLogout(&self, pUserLogout: UserLogoutField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUserLogout(Box::new(pUserLogout), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryMulticastInstrument(&self, pMulticastInstrument: MulticastInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspQryMulticastInstrument(Box::new(pMulticastInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspError(&self, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspError(Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspSubMarketData(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspSubMarketData(Box::new(pSpecificInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUnSubMarketData(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUnSubMarketData(Box::new(pSpecificInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspSubForQuoteRsp(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspSubForQuoteRsp(Box::new(pSpecificInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUnSubForQuoteRsp(&self, pSpecificInstrument: SpecificInstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(MdSpiMsg::OnRspUnSubForQuoteRsp(Box::new(pSpecificInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRtnDepthMarketData(&self, pDepthMarketData: DepthMarketDataField) { self.tx.send(MdSpiMsg::OnRtnDepthMarketData(Box::new(pDepthMarketData))).ok(); }
pub fn OnRtnForQuoteRsp(&self, pForQuoteRsp: ForQuoteRspField) { self.tx.send(MdSpiMsg::OnRtnForQuoteRsp(Box::new(pForQuoteRsp))).ok(); }
}

unsafe impl Send for TraderApi {}
unsafe impl Sync for TraderApi {}

impl TraderApi {
    pub fn CreateTraderApiAndSpi(tx: Sender<TraderSpiMsg>, flow_path: String, is_production_mode: bool) -> UniquePtr<TraderApi> {
        if !Path::new(&flow_path).exists() {
            create_dir_all(&flow_path).unwrap();
        }
        CreateTraderApi(Box::new(TraderSpi { tx }), flow_path, is_production_mode)
    }
}

#[derive(Debug, Clone)]
pub enum TraderSpiMsg {
    OnFrontConnected,
    OnFrontDisconnected(i32),
    OnHeartBeatWarning(i32),
    OnRspAuthenticate(Box<RspAuthenticateField>, Box<RspInfoField>, i32, bool),
    OnRspUserLogin(Box<RspUserLoginField>, Box<RspInfoField>, i32, bool),
    OnRspUserLogout(Box<UserLogoutField>, Box<RspInfoField>, i32, bool),
    OnRspUserPasswordUpdate(Box<UserPasswordUpdateField>, Box<RspInfoField>, i32, bool),
    OnRspTradingAccountPasswordUpdate(Box<TradingAccountPasswordUpdateField>, Box<RspInfoField>, i32, bool),
    OnRspUserAuthMethod(Box<RspUserAuthMethodField>, Box<RspInfoField>, i32, bool),
    OnRspGenUserCaptcha(Box<RspGenUserCaptchaField>, Box<RspInfoField>, i32, bool),
    OnRspGenUserText(Box<RspGenUserTextField>, Box<RspInfoField>, i32, bool),
    OnRspOrderInsert(Box<InputOrderField>, Box<RspInfoField>, i32, bool),
    OnRspParkedOrderInsert(Box<ParkedOrderField>, Box<RspInfoField>, i32, bool),
    OnRspParkedOrderAction(Box<ParkedOrderActionField>, Box<RspInfoField>, i32, bool),
    OnRspOrderAction(Box<InputOrderActionField>, Box<RspInfoField>, i32, bool),
    OnRspQryMaxOrderVolume(Box<QryMaxOrderVolumeField>, Box<RspInfoField>, i32, bool),
    OnRspSettlementInfoConfirm(Box<SettlementInfoConfirmField>, Box<RspInfoField>, i32, bool),
    OnRspRemoveParkedOrder(Box<RemoveParkedOrderField>, Box<RspInfoField>, i32, bool),
    OnRspRemoveParkedOrderAction(Box<RemoveParkedOrderActionField>, Box<RspInfoField>, i32, bool),
    OnRspExecOrderInsert(Box<InputExecOrderField>, Box<RspInfoField>, i32, bool),
    OnRspExecOrderAction(Box<InputExecOrderActionField>, Box<RspInfoField>, i32, bool),
    OnRspForQuoteInsert(Box<InputForQuoteField>, Box<RspInfoField>, i32, bool),
    OnRspQuoteInsert(Box<InputQuoteField>, Box<RspInfoField>, i32, bool),
    OnRspQuoteAction(Box<InputQuoteActionField>, Box<RspInfoField>, i32, bool),
    OnRspBatchOrderAction(Box<InputBatchOrderActionField>, Box<RspInfoField>, i32, bool),
    OnRspOptionSelfCloseInsert(Box<InputOptionSelfCloseField>, Box<RspInfoField>, i32, bool),
    OnRspOptionSelfCloseAction(Box<InputOptionSelfCloseActionField>, Box<RspInfoField>, i32, bool),
    OnRspCombActionInsert(Box<InputCombActionField>, Box<RspInfoField>, i32, bool),
    OnRspQryOrder(Box<OrderField>, Box<RspInfoField>, i32, bool),
    OnRspQryTrade(Box<TradeField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorPosition(Box<InvestorPositionField>, Box<RspInfoField>, i32, bool),
    OnRspQryTradingAccount(Box<TradingAccountField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestor(Box<InvestorField>, Box<RspInfoField>, i32, bool),
    OnRspQryTradingCode(Box<TradingCodeField>, Box<RspInfoField>, i32, bool),
    OnRspQryInstrumentMarginRate(Box<InstrumentMarginRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryInstrumentCommissionRate(Box<InstrumentCommissionRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryUserSession(Box<UserSessionField>, Box<RspInfoField>, i32, bool),
    OnRspQryExchange(Box<ExchangeField>, Box<RspInfoField>, i32, bool),
    OnRspQryProduct(Box<ProductField>, Box<RspInfoField>, i32, bool),
    OnRspQryInstrument(Box<InstrumentField>, Box<RspInfoField>, i32, bool),
    OnRspQryDepthMarketData(Box<DepthMarketDataField>, Box<RspInfoField>, i32, bool),
    OnRspQryTraderOffer(Box<TraderOfferField>, Box<RspInfoField>, i32, bool),
    OnRspQrySettlementInfo(Box<SettlementInfoField>, Box<RspInfoField>, i32, bool),
    OnRspQryTransferBank(Box<TransferBankField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorPositionDetail(Box<InvestorPositionDetailField>, Box<RspInfoField>, i32, bool),
    OnRspQryNotice(Box<NoticeField>, Box<RspInfoField>, i32, bool),
    OnRspQrySettlementInfoConfirm(Box<SettlementInfoConfirmField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorPositionCombineDetail(Box<InvestorPositionCombineDetailField>, Box<RspInfoField>, i32, bool),
    OnRspQryCFMMCTradingAccountKey(Box<CFMMCTradingAccountKeyField>, Box<RspInfoField>, i32, bool),
    OnRspQryEWarrantOffset(Box<EWarrantOffsetField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorProductGroupMargin(Box<InvestorProductGroupMarginField>, Box<RspInfoField>, i32, bool),
    OnRspQryExchangeMarginRate(Box<ExchangeMarginRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryExchangeMarginRateAdjust(Box<ExchangeMarginRateAdjustField>, Box<RspInfoField>, i32, bool),
    OnRspQryExchangeRate(Box<ExchangeRateField>, Box<RspInfoField>, i32, bool),
    OnRspQrySecAgentACIDMap(Box<SecAgentACIDMapField>, Box<RspInfoField>, i32, bool),
    OnRspQryProductExchRate(Box<ProductExchRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryProductGroup(Box<ProductGroupField>, Box<RspInfoField>, i32, bool),
    OnRspQryMMInstrumentCommissionRate(Box<MMInstrumentCommissionRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryMMOptionInstrCommRate(Box<MMOptionInstrCommRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryInstrumentOrderCommRate(Box<InstrumentOrderCommRateField>, Box<RspInfoField>, i32, bool),
    OnRspQrySecAgentTradingAccount(Box<TradingAccountField>, Box<RspInfoField>, i32, bool),
    OnRspQrySecAgentCheckMode(Box<SecAgentCheckModeField>, Box<RspInfoField>, i32, bool),
    OnRspQrySecAgentTradeInfo(Box<SecAgentTradeInfoField>, Box<RspInfoField>, i32, bool),
    OnRspQryOptionInstrTradeCost(Box<OptionInstrTradeCostField>, Box<RspInfoField>, i32, bool),
    OnRspQryOptionInstrCommRate(Box<OptionInstrCommRateField>, Box<RspInfoField>, i32, bool),
    OnRspQryExecOrder(Box<ExecOrderField>, Box<RspInfoField>, i32, bool),
    OnRspQryForQuote(Box<ForQuoteField>, Box<RspInfoField>, i32, bool),
    OnRspQryQuote(Box<QuoteField>, Box<RspInfoField>, i32, bool),
    OnRspQryOptionSelfClose(Box<OptionSelfCloseField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestUnit(Box<InvestUnitField>, Box<RspInfoField>, i32, bool),
    OnRspQryCombInstrumentGuard(Box<CombInstrumentGuardField>, Box<RspInfoField>, i32, bool),
    OnRspQryCombAction(Box<CombActionField>, Box<RspInfoField>, i32, bool),
    OnRspQryTransferSerial(Box<TransferSerialField>, Box<RspInfoField>, i32, bool),
    OnRspQryAccountregister(Box<AccountregisterField>, Box<RspInfoField>, i32, bool),
    OnRspError(Box<RspInfoField>, i32, bool),
    OnRtnOrder(Box<OrderField>),
    OnRtnTrade(Box<TradeField>),
    OnErrRtnOrderInsert(Box<InputOrderField>, Box<RspInfoField>),
    OnErrRtnOrderAction(Box<OrderActionField>, Box<RspInfoField>),
    OnRtnInstrumentStatus(Box<InstrumentStatusField>),
    OnRtnBulletin(Box<BulletinField>),
    OnRtnTradingNotice(Box<TradingNoticeInfoField>),
    OnRtnErrorConditionalOrder(Box<ErrorConditionalOrderField>),
    OnRtnExecOrder(Box<ExecOrderField>),
    OnErrRtnExecOrderInsert(Box<InputExecOrderField>, Box<RspInfoField>),
    OnErrRtnExecOrderAction(Box<ExecOrderActionField>, Box<RspInfoField>),
    OnErrRtnForQuoteInsert(Box<InputForQuoteField>, Box<RspInfoField>),
    OnRtnQuote(Box<QuoteField>),
    OnErrRtnQuoteInsert(Box<InputQuoteField>, Box<RspInfoField>),
    OnErrRtnQuoteAction(Box<QuoteActionField>, Box<RspInfoField>),
    OnRtnForQuoteRsp(Box<ForQuoteRspField>),
    OnRtnCFMMCTradingAccountToken(Box<CFMMCTradingAccountTokenField>),
    OnErrRtnBatchOrderAction(Box<BatchOrderActionField>, Box<RspInfoField>),
    OnRtnOptionSelfClose(Box<OptionSelfCloseField>),
    OnErrRtnOptionSelfCloseInsert(Box<InputOptionSelfCloseField>, Box<RspInfoField>),
    OnErrRtnOptionSelfCloseAction(Box<OptionSelfCloseActionField>, Box<RspInfoField>),
    OnRtnCombAction(Box<CombActionField>),
    OnErrRtnCombActionInsert(Box<InputCombActionField>, Box<RspInfoField>),
    OnRspQryContractBank(Box<ContractBankField>, Box<RspInfoField>, i32, bool),
    OnRspQryParkedOrder(Box<ParkedOrderField>, Box<RspInfoField>, i32, bool),
    OnRspQryParkedOrderAction(Box<ParkedOrderActionField>, Box<RspInfoField>, i32, bool),
    OnRspQryTradingNotice(Box<TradingNoticeField>, Box<RspInfoField>, i32, bool),
    OnRspQryBrokerTradingParams(Box<BrokerTradingParamsField>, Box<RspInfoField>, i32, bool),
    OnRspQryBrokerTradingAlgos(Box<BrokerTradingAlgosField>, Box<RspInfoField>, i32, bool),
    OnRspQueryCFMMCTradingAccountToken(Box<QueryCFMMCTradingAccountTokenField>, Box<RspInfoField>, i32, bool),
    OnRtnFromBankToFutureByBank(Box<RspTransferField>),
    OnRtnFromFutureToBankByBank(Box<RspTransferField>),
    OnRtnRepealFromBankToFutureByBank(Box<RspRepealField>),
    OnRtnRepealFromFutureToBankByBank(Box<RspRepealField>),
    OnRtnFromBankToFutureByFuture(Box<RspTransferField>),
    OnRtnFromFutureToBankByFuture(Box<RspTransferField>),
    OnRtnRepealFromBankToFutureByFutureManual(Box<RspRepealField>),
    OnRtnRepealFromFutureToBankByFutureManual(Box<RspRepealField>),
    OnRtnQueryBankBalanceByFuture(Box<NotifyQueryAccountField>),
    OnErrRtnBankToFutureByFuture(Box<ReqTransferField>, Box<RspInfoField>),
    OnErrRtnFutureToBankByFuture(Box<ReqTransferField>, Box<RspInfoField>),
    OnErrRtnRepealBankToFutureByFutureManual(Box<ReqRepealField>, Box<RspInfoField>),
    OnErrRtnRepealFutureToBankByFutureManual(Box<ReqRepealField>, Box<RspInfoField>),
    OnErrRtnQueryBankBalanceByFuture(Box<ReqQueryAccountField>, Box<RspInfoField>),
    OnRtnRepealFromBankToFutureByFuture(Box<RspRepealField>),
    OnRtnRepealFromFutureToBankByFuture(Box<RspRepealField>),
    OnRspFromBankToFutureByFuture(Box<ReqTransferField>, Box<RspInfoField>, i32, bool),
    OnRspFromFutureToBankByFuture(Box<ReqTransferField>, Box<RspInfoField>, i32, bool),
    OnRspQueryBankAccountMoneyByFuture(Box<ReqQueryAccountField>, Box<RspInfoField>, i32, bool),
    OnRtnOpenAccountByBank(Box<OpenAccountField>),
    OnRtnCancelAccountByBank(Box<CancelAccountField>),
    OnRtnChangeAccountByBank(Box<ChangeAccountField>),
    OnRspQryClassifiedInstrument(Box<InstrumentField>, Box<RspInfoField>, i32, bool),
    OnRspQryCombPromotionParam(Box<CombPromotionParamField>, Box<RspInfoField>, i32, bool),
    OnRspQryRiskSettleInvstPosition(Box<RiskSettleInvstPositionField>, Box<RspInfoField>, i32, bool),
    OnRspQryRiskSettleProductStatus(Box<RiskSettleProductStatusField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMFutureParameter(Box<SPBMFutureParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMOptionParameter(Box<SPBMOptionParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMIntraParameter(Box<SPBMIntraParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMInterParameter(Box<SPBMInterParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMPortfDefinition(Box<SPBMPortfDefinitionField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMInvestorPortfDef(Box<SPBMInvestorPortfDefField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorPortfMarginRatio(Box<InvestorPortfMarginRatioField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorProdSPBMDetail(Box<InvestorProdSPBMDetailField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorCommoditySPMMMargin(Box<InvestorCommoditySPMMMarginField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorCommodityGroupSPMMMargin(Box<InvestorCommodityGroupSPMMMarginField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPMMInstParam(Box<SPMMInstParamField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPMMProductParam(Box<SPMMProductParamField>, Box<RspInfoField>, i32, bool),
    OnRspQrySPBMAddOnInterParameter(Box<SPBMAddOnInterParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryRCAMSCombProductInfo(Box<RCAMSCombProductInfoField>, Box<RspInfoField>, i32, bool),
    OnRspQryRCAMSInstrParameter(Box<RCAMSInstrParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryRCAMSIntraParameter(Box<RCAMSIntraParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryRCAMSInterParameter(Box<RCAMSInterParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryRCAMSShortOptAdjustParam(Box<RCAMSShortOptAdjustParamField>, Box<RspInfoField>, i32, bool),
    OnRspQryRCAMSInvestorCombPosition(Box<RCAMSInvestorCombPositionField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorProdRCAMSMargin(Box<InvestorProdRCAMSMarginField>, Box<RspInfoField>, i32, bool),
    OnRspQryRULEInstrParameter(Box<RULEInstrParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryRULEIntraParameter(Box<RULEIntraParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryRULEInterParameter(Box<RULEInterParameterField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorProdRULEMargin(Box<InvestorProdRULEMarginField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorPortfSetting(Box<InvestorPortfSettingField>, Box<RspInfoField>, i32, bool),
    OnRspQryInvestorInfoCommRec(Box<InvestorInfoCommRecField>, Box<RspInfoField>, i32, bool),
    OnRspQryCombLeg(Box<CombLegField>, Box<RspInfoField>, i32, bool),
    OnRspOffsetSetting(Box<InputOffsetSettingField>, Box<RspInfoField>, i32, bool),
    OnRspCancelOffsetSetting(Box<InputOffsetSettingField>, Box<RspInfoField>, i32, bool),
    OnRtnOffsetSetting(Box<OffsetSettingField>),
    OnErrRtnOffsetSetting(Box<InputOffsetSettingField>, Box<RspInfoField>),
    OnErrRtnCancelOffsetSetting(Box<CancelOffsetSettingField>, Box<RspInfoField>),
    OnRspQryOffsetSetting(Box<OffsetSettingField>, Box<RspInfoField>, i32, bool),
}

pub struct TraderSpi {
    tx: Sender<TraderSpiMsg>,
}

impl TraderSpi {
pub fn OnFrontConnected(&self) { self.tx.send(TraderSpiMsg::OnFrontConnected).ok(); }
pub fn OnFrontDisconnected(&self, nReason: i32) { self.tx.send(TraderSpiMsg::OnFrontDisconnected(nReason)).ok(); }
pub fn OnHeartBeatWarning(&self, nTimeLapse: i32) { self.tx.send(TraderSpiMsg::OnHeartBeatWarning(nTimeLapse)).ok(); }
pub fn OnRspAuthenticate(&self, pRspAuthenticateField: RspAuthenticateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspAuthenticate(Box::new(pRspAuthenticateField), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUserLogin(&self, pRspUserLogin: RspUserLoginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserLogin(Box::new(pRspUserLogin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUserLogout(&self, pUserLogout: UserLogoutField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserLogout(Box::new(pUserLogout), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUserPasswordUpdate(&self, pUserPasswordUpdate: UserPasswordUpdateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserPasswordUpdate(Box::new(pUserPasswordUpdate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspTradingAccountPasswordUpdate(&self, pTradingAccountPasswordUpdate: TradingAccountPasswordUpdateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspTradingAccountPasswordUpdate(Box::new(pTradingAccountPasswordUpdate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspUserAuthMethod(&self, pRspUserAuthMethod: RspUserAuthMethodField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspUserAuthMethod(Box::new(pRspUserAuthMethod), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspGenUserCaptcha(&self, pRspGenUserCaptcha: RspGenUserCaptchaField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspGenUserCaptcha(Box::new(pRspGenUserCaptcha), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspGenUserText(&self, pRspGenUserText: RspGenUserTextField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspGenUserText(Box::new(pRspGenUserText), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspOrderInsert(&self, pInputOrder: InputOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOrderInsert(Box::new(pInputOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspParkedOrderInsert(&self, pParkedOrder: ParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspParkedOrderInsert(Box::new(pParkedOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspParkedOrderAction(Box::new(pParkedOrderAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspOrderAction(&self, pInputOrderAction: InputOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOrderAction(Box::new(pInputOrderAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryMaxOrderVolume(&self, pQryMaxOrderVolume: QryMaxOrderVolumeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryMaxOrderVolume(Box::new(pQryMaxOrderVolume), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspSettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspSettlementInfoConfirm(Box::new(pSettlementInfoConfirm), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspRemoveParkedOrder(&self, pRemoveParkedOrder: RemoveParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspRemoveParkedOrder(Box::new(pRemoveParkedOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspRemoveParkedOrderAction(&self, pRemoveParkedOrderAction: RemoveParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspRemoveParkedOrderAction(Box::new(pRemoveParkedOrderAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspExecOrderInsert(Box::new(pInputExecOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspExecOrderAction(&self, pInputExecOrderAction: InputExecOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspExecOrderAction(Box::new(pInputExecOrderAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspForQuoteInsert(&self, pInputForQuote: InputForQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspForQuoteInsert(Box::new(pInputForQuote), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQuoteInsert(&self, pInputQuote: InputQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQuoteInsert(Box::new(pInputQuote), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQuoteAction(&self, pInputQuoteAction: InputQuoteActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQuoteAction(Box::new(pInputQuoteAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspBatchOrderAction(&self, pInputBatchOrderAction: InputBatchOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspBatchOrderAction(Box::new(pInputBatchOrderAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOptionSelfCloseInsert(Box::new(pInputOptionSelfClose), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspOptionSelfCloseAction(&self, pInputOptionSelfCloseAction: InputOptionSelfCloseActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOptionSelfCloseAction(Box::new(pInputOptionSelfCloseAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspCombActionInsert(&self, pInputCombAction: InputCombActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspCombActionInsert(Box::new(pInputCombAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryOrder(&self, pOrder: OrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOrder(Box::new(pOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTrade(&self, pTrade: TradeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTrade(Box::new(pTrade), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorPosition(&self, pInvestorPosition: InvestorPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPosition(Box::new(pInvestorPosition), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTradingAccount(&self, pTradingAccount: TradingAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTradingAccount(Box::new(pTradingAccount), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestor(&self, pInvestor: InvestorField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestor(Box::new(pInvestor), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTradingCode(&self, pTradingCode: TradingCodeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTradingCode(Box::new(pTradingCode), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInstrumentMarginRate(&self, pInstrumentMarginRate: InstrumentMarginRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrumentMarginRate(Box::new(pInstrumentMarginRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInstrumentCommissionRate(&self, pInstrumentCommissionRate: InstrumentCommissionRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrumentCommissionRate(Box::new(pInstrumentCommissionRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryUserSession(&self, pUserSession: UserSessionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryUserSession(Box::new(pUserSession), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryExchange(&self, pExchange: ExchangeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchange(Box::new(pExchange), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryProduct(&self, pProduct: ProductField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryProduct(Box::new(pProduct), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInstrument(&self, pInstrument: InstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrument(Box::new(pInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryDepthMarketData(&self, pDepthMarketData: DepthMarketDataField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryDepthMarketData(Box::new(pDepthMarketData), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTraderOffer(&self, pTraderOffer: TraderOfferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTraderOffer(Box::new(pTraderOffer), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySettlementInfo(&self, pSettlementInfo: SettlementInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySettlementInfo(Box::new(pSettlementInfo), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTransferBank(&self, pTransferBank: TransferBankField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTransferBank(Box::new(pTransferBank), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorPositionDetail(&self, pInvestorPositionDetail: InvestorPositionDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPositionDetail(Box::new(pInvestorPositionDetail), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryNotice(&self, pNotice: NoticeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryNotice(Box::new(pNotice), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySettlementInfoConfirm(Box::new(pSettlementInfoConfirm), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorPositionCombineDetail(&self, pInvestorPositionCombineDetail: InvestorPositionCombineDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPositionCombineDetail(Box::new(pInvestorPositionCombineDetail), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryCFMMCTradingAccountKey(&self, pCFMMCTradingAccountKey: CFMMCTradingAccountKeyField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCFMMCTradingAccountKey(Box::new(pCFMMCTradingAccountKey), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryEWarrantOffset(&self, pEWarrantOffset: EWarrantOffsetField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryEWarrantOffset(Box::new(pEWarrantOffset), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorProductGroupMargin(&self, pInvestorProductGroupMargin: InvestorProductGroupMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProductGroupMargin(Box::new(pInvestorProductGroupMargin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryExchangeMarginRate(&self, pExchangeMarginRate: ExchangeMarginRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchangeMarginRate(Box::new(pExchangeMarginRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryExchangeMarginRateAdjust(&self, pExchangeMarginRateAdjust: ExchangeMarginRateAdjustField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchangeMarginRateAdjust(Box::new(pExchangeMarginRateAdjust), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryExchangeRate(&self, pExchangeRate: ExchangeRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExchangeRate(Box::new(pExchangeRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySecAgentACIDMap(&self, pSecAgentACIDMap: SecAgentACIDMapField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentACIDMap(Box::new(pSecAgentACIDMap), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryProductExchRate(&self, pProductExchRate: ProductExchRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryProductExchRate(Box::new(pProductExchRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryProductGroup(&self, pProductGroup: ProductGroupField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryProductGroup(Box::new(pProductGroup), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryMMInstrumentCommissionRate(&self, pMMInstrumentCommissionRate: MMInstrumentCommissionRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryMMInstrumentCommissionRate(Box::new(pMMInstrumentCommissionRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryMMOptionInstrCommRate(&self, pMMOptionInstrCommRate: MMOptionInstrCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryMMOptionInstrCommRate(Box::new(pMMOptionInstrCommRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInstrumentOrderCommRate(&self, pInstrumentOrderCommRate: InstrumentOrderCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInstrumentOrderCommRate(Box::new(pInstrumentOrderCommRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySecAgentTradingAccount(&self, pTradingAccount: TradingAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentTradingAccount(Box::new(pTradingAccount), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySecAgentCheckMode(&self, pSecAgentCheckMode: SecAgentCheckModeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentCheckMode(Box::new(pSecAgentCheckMode), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySecAgentTradeInfo(&self, pSecAgentTradeInfo: SecAgentTradeInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySecAgentTradeInfo(Box::new(pSecAgentTradeInfo), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryOptionInstrTradeCost(&self, pOptionInstrTradeCost: OptionInstrTradeCostField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOptionInstrTradeCost(Box::new(pOptionInstrTradeCost), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryOptionInstrCommRate(&self, pOptionInstrCommRate: OptionInstrCommRateField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOptionInstrCommRate(Box::new(pOptionInstrCommRate), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryExecOrder(&self, pExecOrder: ExecOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryExecOrder(Box::new(pExecOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryForQuote(&self, pForQuote: ForQuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryForQuote(Box::new(pForQuote), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryQuote(&self, pQuote: QuoteField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryQuote(Box::new(pQuote), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryOptionSelfClose(&self, pOptionSelfClose: OptionSelfCloseField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOptionSelfClose(Box::new(pOptionSelfClose), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestUnit(&self, pInvestUnit: InvestUnitField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestUnit(Box::new(pInvestUnit), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryCombInstrumentGuard(&self, pCombInstrumentGuard: CombInstrumentGuardField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombInstrumentGuard(Box::new(pCombInstrumentGuard), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryCombAction(&self, pCombAction: CombActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombAction(Box::new(pCombAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTransferSerial(&self, pTransferSerial: TransferSerialField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTransferSerial(Box::new(pTransferSerial), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryAccountregister(&self, pAccountregister: AccountregisterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryAccountregister(Box::new(pAccountregister), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspError(&self, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspError(Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRtnOrder(&self, pOrder: OrderField) { self.tx.send(TraderSpiMsg::OnRtnOrder(Box::new(pOrder))).ok(); }
pub fn OnRtnTrade(&self, pTrade: TradeField) { self.tx.send(TraderSpiMsg::OnRtnTrade(Box::new(pTrade))).ok(); }
pub fn OnErrRtnOrderInsert(&self, pInputOrder: InputOrderField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOrderInsert(Box::new(pInputOrder), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnOrderAction(&self, pOrderAction: OrderActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOrderAction(Box::new(pOrderAction), Box::new(pRspInfo))).ok(); }
pub fn OnRtnInstrumentStatus(&self, pInstrumentStatus: InstrumentStatusField) { self.tx.send(TraderSpiMsg::OnRtnInstrumentStatus(Box::new(pInstrumentStatus))).ok(); }
pub fn OnRtnBulletin(&self, pBulletin: BulletinField) { self.tx.send(TraderSpiMsg::OnRtnBulletin(Box::new(pBulletin))).ok(); }
pub fn OnRtnTradingNotice(&self, pTradingNoticeInfo: TradingNoticeInfoField) { self.tx.send(TraderSpiMsg::OnRtnTradingNotice(Box::new(pTradingNoticeInfo))).ok(); }
pub fn OnRtnErrorConditionalOrder(&self, pErrorConditionalOrder: ErrorConditionalOrderField) { self.tx.send(TraderSpiMsg::OnRtnErrorConditionalOrder(Box::new(pErrorConditionalOrder))).ok(); }
pub fn OnRtnExecOrder(&self, pExecOrder: ExecOrderField) { self.tx.send(TraderSpiMsg::OnRtnExecOrder(Box::new(pExecOrder))).ok(); }
pub fn OnErrRtnExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnExecOrderInsert(Box::new(pInputExecOrder), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnExecOrderAction(&self, pExecOrderAction: ExecOrderActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnExecOrderAction(Box::new(pExecOrderAction), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnForQuoteInsert(&self, pInputForQuote: InputForQuoteField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnForQuoteInsert(Box::new(pInputForQuote), Box::new(pRspInfo))).ok(); }
pub fn OnRtnQuote(&self, pQuote: QuoteField) { self.tx.send(TraderSpiMsg::OnRtnQuote(Box::new(pQuote))).ok(); }
pub fn OnErrRtnQuoteInsert(&self, pInputQuote: InputQuoteField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnQuoteInsert(Box::new(pInputQuote), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnQuoteAction(&self, pQuoteAction: QuoteActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnQuoteAction(Box::new(pQuoteAction), Box::new(pRspInfo))).ok(); }
pub fn OnRtnForQuoteRsp(&self, pForQuoteRsp: ForQuoteRspField) { self.tx.send(TraderSpiMsg::OnRtnForQuoteRsp(Box::new(pForQuoteRsp))).ok(); }
pub fn OnRtnCFMMCTradingAccountToken(&self, pCFMMCTradingAccountToken: CFMMCTradingAccountTokenField) { self.tx.send(TraderSpiMsg::OnRtnCFMMCTradingAccountToken(Box::new(pCFMMCTradingAccountToken))).ok(); }
pub fn OnErrRtnBatchOrderAction(&self, pBatchOrderAction: BatchOrderActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnBatchOrderAction(Box::new(pBatchOrderAction), Box::new(pRspInfo))).ok(); }
pub fn OnRtnOptionSelfClose(&self, pOptionSelfClose: OptionSelfCloseField) { self.tx.send(TraderSpiMsg::OnRtnOptionSelfClose(Box::new(pOptionSelfClose))).ok(); }
pub fn OnErrRtnOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOptionSelfCloseInsert(Box::new(pInputOptionSelfClose), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnOptionSelfCloseAction(&self, pOptionSelfCloseAction: OptionSelfCloseActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOptionSelfCloseAction(Box::new(pOptionSelfCloseAction), Box::new(pRspInfo))).ok(); }
pub fn OnRtnCombAction(&self, pCombAction: CombActionField) { self.tx.send(TraderSpiMsg::OnRtnCombAction(Box::new(pCombAction))).ok(); }
pub fn OnErrRtnCombActionInsert(&self, pInputCombAction: InputCombActionField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnCombActionInsert(Box::new(pInputCombAction), Box::new(pRspInfo))).ok(); }
pub fn OnRspQryContractBank(&self, pContractBank: ContractBankField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryContractBank(Box::new(pContractBank), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryParkedOrder(&self, pParkedOrder: ParkedOrderField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryParkedOrder(Box::new(pParkedOrder), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryParkedOrderAction(Box::new(pParkedOrderAction), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryTradingNotice(&self, pTradingNotice: TradingNoticeField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryTradingNotice(Box::new(pTradingNotice), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryBrokerTradingParams(&self, pBrokerTradingParams: BrokerTradingParamsField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryBrokerTradingParams(Box::new(pBrokerTradingParams), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryBrokerTradingAlgos(&self, pBrokerTradingAlgos: BrokerTradingAlgosField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryBrokerTradingAlgos(Box::new(pBrokerTradingAlgos), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQueryCFMMCTradingAccountToken(&self, pQueryCFMMCTradingAccountToken: QueryCFMMCTradingAccountTokenField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQueryCFMMCTradingAccountToken(Box::new(pQueryCFMMCTradingAccountToken), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRtnFromBankToFutureByBank(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromBankToFutureByBank(Box::new(pRspTransfer))).ok(); }
pub fn OnRtnFromFutureToBankByBank(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromFutureToBankByBank(Box::new(pRspTransfer))).ok(); }
pub fn OnRtnRepealFromBankToFutureByBank(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromBankToFutureByBank(Box::new(pRspRepeal))).ok(); }
pub fn OnRtnRepealFromFutureToBankByBank(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromFutureToBankByBank(Box::new(pRspRepeal))).ok(); }
pub fn OnRtnFromBankToFutureByFuture(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromBankToFutureByFuture(Box::new(pRspTransfer))).ok(); }
pub fn OnRtnFromFutureToBankByFuture(&self, pRspTransfer: RspTransferField) { self.tx.send(TraderSpiMsg::OnRtnFromFutureToBankByFuture(Box::new(pRspTransfer))).ok(); }
pub fn OnRtnRepealFromBankToFutureByFutureManual(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromBankToFutureByFutureManual(Box::new(pRspRepeal))).ok(); }
pub fn OnRtnRepealFromFutureToBankByFutureManual(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromFutureToBankByFutureManual(Box::new(pRspRepeal))).ok(); }
pub fn OnRtnQueryBankBalanceByFuture(&self, pNotifyQueryAccount: NotifyQueryAccountField) { self.tx.send(TraderSpiMsg::OnRtnQueryBankBalanceByFuture(Box::new(pNotifyQueryAccount))).ok(); }
pub fn OnErrRtnBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnBankToFutureByFuture(Box::new(pReqTransfer), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnFutureToBankByFuture(Box::new(pReqTransfer), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnRepealBankToFutureByFutureManual(&self, pReqRepeal: ReqRepealField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnRepealBankToFutureByFutureManual(Box::new(pReqRepeal), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnRepealFutureToBankByFutureManual(&self, pReqRepeal: ReqRepealField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnRepealFutureToBankByFutureManual(Box::new(pReqRepeal), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnQueryBankBalanceByFuture(&self, pReqQueryAccount: ReqQueryAccountField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnQueryBankBalanceByFuture(Box::new(pReqQueryAccount), Box::new(pRspInfo))).ok(); }
pub fn OnRtnRepealFromBankToFutureByFuture(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromBankToFutureByFuture(Box::new(pRspRepeal))).ok(); }
pub fn OnRtnRepealFromFutureToBankByFuture(&self, pRspRepeal: RspRepealField) { self.tx.send(TraderSpiMsg::OnRtnRepealFromFutureToBankByFuture(Box::new(pRspRepeal))).ok(); }
pub fn OnRspFromBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspFromBankToFutureByFuture(Box::new(pReqTransfer), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspFromFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspFromFutureToBankByFuture(Box::new(pReqTransfer), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQueryBankAccountMoneyByFuture(&self, pReqQueryAccount: ReqQueryAccountField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQueryBankAccountMoneyByFuture(Box::new(pReqQueryAccount), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRtnOpenAccountByBank(&self, pOpenAccount: OpenAccountField) { self.tx.send(TraderSpiMsg::OnRtnOpenAccountByBank(Box::new(pOpenAccount))).ok(); }
pub fn OnRtnCancelAccountByBank(&self, pCancelAccount: CancelAccountField) { self.tx.send(TraderSpiMsg::OnRtnCancelAccountByBank(Box::new(pCancelAccount))).ok(); }
pub fn OnRtnChangeAccountByBank(&self, pChangeAccount: ChangeAccountField) { self.tx.send(TraderSpiMsg::OnRtnChangeAccountByBank(Box::new(pChangeAccount))).ok(); }
pub fn OnRspQryClassifiedInstrument(&self, pInstrument: InstrumentField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryClassifiedInstrument(Box::new(pInstrument), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryCombPromotionParam(&self, pCombPromotionParam: CombPromotionParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombPromotionParam(Box::new(pCombPromotionParam), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRiskSettleInvstPosition(&self, pRiskSettleInvstPosition: RiskSettleInvstPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRiskSettleInvstPosition(Box::new(pRiskSettleInvstPosition), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRiskSettleProductStatus(&self, pRiskSettleProductStatus: RiskSettleProductStatusField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRiskSettleProductStatus(Box::new(pRiskSettleProductStatus), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMFutureParameter(&self, pSPBMFutureParameter: SPBMFutureParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMFutureParameter(Box::new(pSPBMFutureParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMOptionParameter(&self, pSPBMOptionParameter: SPBMOptionParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMOptionParameter(Box::new(pSPBMOptionParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMIntraParameter(&self, pSPBMIntraParameter: SPBMIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMIntraParameter(Box::new(pSPBMIntraParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMInterParameter(&self, pSPBMInterParameter: SPBMInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMInterParameter(Box::new(pSPBMInterParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMPortfDefinition(&self, pSPBMPortfDefinition: SPBMPortfDefinitionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMPortfDefinition(Box::new(pSPBMPortfDefinition), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMInvestorPortfDef(&self, pSPBMInvestorPortfDef: SPBMInvestorPortfDefField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMInvestorPortfDef(Box::new(pSPBMInvestorPortfDef), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorPortfMarginRatio(&self, pInvestorPortfMarginRatio: InvestorPortfMarginRatioField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPortfMarginRatio(Box::new(pInvestorPortfMarginRatio), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorProdSPBMDetail(&self, pInvestorProdSPBMDetail: InvestorProdSPBMDetailField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProdSPBMDetail(Box::new(pInvestorProdSPBMDetail), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorCommoditySPMMMargin(&self, pInvestorCommoditySPMMMargin: InvestorCommoditySPMMMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorCommoditySPMMMargin(Box::new(pInvestorCommoditySPMMMargin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorCommodityGroupSPMMMargin(&self, pInvestorCommodityGroupSPMMMargin: InvestorCommodityGroupSPMMMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorCommodityGroupSPMMMargin(Box::new(pInvestorCommodityGroupSPMMMargin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPMMInstParam(&self, pSPMMInstParam: SPMMInstParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPMMInstParam(Box::new(pSPMMInstParam), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPMMProductParam(&self, pSPMMProductParam: SPMMProductParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPMMProductParam(Box::new(pSPMMProductParam), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQrySPBMAddOnInterParameter(&self, pSPBMAddOnInterParameter: SPBMAddOnInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQrySPBMAddOnInterParameter(Box::new(pSPBMAddOnInterParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRCAMSCombProductInfo(&self, pRCAMSCombProductInfo: RCAMSCombProductInfoField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSCombProductInfo(Box::new(pRCAMSCombProductInfo), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRCAMSInstrParameter(&self, pRCAMSInstrParameter: RCAMSInstrParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSInstrParameter(Box::new(pRCAMSInstrParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRCAMSIntraParameter(&self, pRCAMSIntraParameter: RCAMSIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSIntraParameter(Box::new(pRCAMSIntraParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRCAMSInterParameter(&self, pRCAMSInterParameter: RCAMSInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSInterParameter(Box::new(pRCAMSInterParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRCAMSShortOptAdjustParam(&self, pRCAMSShortOptAdjustParam: RCAMSShortOptAdjustParamField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSShortOptAdjustParam(Box::new(pRCAMSShortOptAdjustParam), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRCAMSInvestorCombPosition(&self, pRCAMSInvestorCombPosition: RCAMSInvestorCombPositionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRCAMSInvestorCombPosition(Box::new(pRCAMSInvestorCombPosition), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorProdRCAMSMargin(&self, pInvestorProdRCAMSMargin: InvestorProdRCAMSMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProdRCAMSMargin(Box::new(pInvestorProdRCAMSMargin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRULEInstrParameter(&self, pRULEInstrParameter: RULEInstrParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRULEInstrParameter(Box::new(pRULEInstrParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRULEIntraParameter(&self, pRULEIntraParameter: RULEIntraParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRULEIntraParameter(Box::new(pRULEIntraParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryRULEInterParameter(&self, pRULEInterParameter: RULEInterParameterField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryRULEInterParameter(Box::new(pRULEInterParameter), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorProdRULEMargin(&self, pInvestorProdRULEMargin: InvestorProdRULEMarginField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorProdRULEMargin(Box::new(pInvestorProdRULEMargin), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorPortfSetting(&self, pInvestorPortfSetting: InvestorPortfSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorPortfSetting(Box::new(pInvestorPortfSetting), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryInvestorInfoCommRec(&self, pInvestorInfoCommRec: InvestorInfoCommRecField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryInvestorInfoCommRec(Box::new(pInvestorInfoCommRec), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspQryCombLeg(&self, pCombLeg: CombLegField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryCombLeg(Box::new(pCombLeg), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspOffsetSetting(Box::new(pInputOffsetSetting), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRspCancelOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspCancelOffsetSetting(Box::new(pInputOffsetSetting), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
pub fn OnRtnOffsetSetting(&self, pOffsetSetting: OffsetSettingField) { self.tx.send(TraderSpiMsg::OnRtnOffsetSetting(Box::new(pOffsetSetting))).ok(); }
pub fn OnErrRtnOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnOffsetSetting(Box::new(pInputOffsetSetting), Box::new(pRspInfo))).ok(); }
pub fn OnErrRtnCancelOffsetSetting(&self, pCancelOffsetSetting: CancelOffsetSettingField, pRspInfo: RspInfoField) { self.tx.send(TraderSpiMsg::OnErrRtnCancelOffsetSetting(Box::new(pCancelOffsetSetting), Box::new(pRspInfo))).ok(); }
pub fn OnRspQryOffsetSetting(&self, pOffsetSetting: OffsetSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool) { self.tx.send(TraderSpiMsg::OnRspQryOffsetSetting(Box::new(pOffsetSetting), Box::new(pRspInfo), nRequestID, bIsLast)).ok(); }
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
        include!("ctp-rs/wrapper/include/MdApi.h");
        type MdApi;

        fn CreateMdApi(spi: Box<MdSpi>, flow_path: String, is_using_udp: bool, is_multicast: bool, is_production_mode: bool) -> UniquePtr<MdApi>;

        /// 获取API的版本信息
        ///
        /// # Returns
        /// 获取到的版本号
        fn GetApiVersion(&self)-> String;
        /// 初始化
        ///
        /// # Remarks
        /// 初始化运行环境,只有调用后,接口才开始工作
        fn Init(&self);
        /// 等待接口线程结束运行
        ///
        /// # Returns
        /// 线程退出代码
        fn Join(&self)-> i32;
        /// 获取当前交易日
        ///
        /// # Returns
        /// 获取到的交易日
        ///
        /// # Remarks
        /// 只有登录成功后,才能得到正确的交易日
        fn GetTradingDay(&self)-> String;
        /// 注册前置机网络地址
        ///
        /// # Parameters
        /// - `pszFrontAddress` — 前置机网络地址。
        ///
        /// # Remarks
        /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
        /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
        fn RegisterFront(&self, pszFrontAddress: String);
        /// 注册名字服务器网络地址
        ///
        /// # Parameters
        /// - `pszNsAddress` — 名字服务器网络地址。
        ///
        /// # Remarks
        /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
        /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
        /// RegisterNameServer优先于RegisterFront
        fn RegisterNameServer(&self, pszNsAddress: String);
        /// 注册名字服务器用户信息
        ///
        /// # Parameters
        /// - `pFensUserInfo` — 用户信息。
        fn RegisterFensUserInfo(&self, pFensUserInfo: FensUserInfoField);
        /// 订阅行情。
        ///
        /// # Parameters
        /// - `ppInstrumentID` — 合约ID
        /// - `nCount` — 要订阅/退订行情的合约个数
        fn SubscribeMarketData(&self, ppInstrumentID: Vec<String>)-> i32;
        /// 退订行情。
        ///
        /// # Parameters
        /// - `ppInstrumentID` — 合约ID
        /// - `nCount` — 要订阅/退订行情的合约个数
        fn UnSubscribeMarketData(&self, ppInstrumentID: Vec<String>)-> i32;
        /// 订阅询价。
        ///
        /// # Parameters
        /// - `ppInstrumentID` — 合约ID
        /// - `nCount` — 要订阅/退订行情的合约个数
        fn SubscribeForQuoteRsp(&self, ppInstrumentID: Vec<String>)-> i32;
        /// 退订询价。
        ///
        /// # Parameters
        /// - `ppInstrumentID` — 合约ID
        /// - `nCount` — 要订阅/退订行情的合约个数
        fn UnSubscribeForQuoteRsp(&self, ppInstrumentID: Vec<String>)-> i32;
        /// 用户登录请求
        fn ReqUserLogin(&self, pReqUserLoginField: ReqUserLoginField, nRequestID: i32)-> i32;
        /// 登出请求
        fn ReqUserLogout(&self, pUserLogout: UserLogoutField, nRequestID: i32)-> i32;
        /// 请求查询组播合约
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
        pub fn OnRspQryUserSession(&self, pUserSession: UserSessionField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
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
        pub fn OnRspQryCombLeg(&self, pCombLeg: CombLegField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRspCancelOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
        pub fn OnRtnOffsetSetting(&self, pOffsetSetting: OffsetSettingField);
        pub fn OnErrRtnOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, pRspInfo: RspInfoField);
        pub fn OnErrRtnCancelOffsetSetting(&self, pCancelOffsetSetting: CancelOffsetSettingField, pRspInfo: RspInfoField);
        pub fn OnRspQryOffsetSetting(&self, pOffsetSetting: OffsetSettingField, pRspInfo: RspInfoField, nRequestID: i32, bIsLast: bool);
    }

    unsafe extern "C++" {
        include!("ctp-rs/wrapper/include/TraderApi.h");
        type TraderApi;

        fn CreateTraderApi(spi: Box<TraderSpi>, flow_path: String, is_production_mode: bool) -> UniquePtr<TraderApi>;
        fn GetFrontInfo(&self) -> FrontInfoField;

        /// 获取API的版本信息
        ///
        /// # Returns
        /// 获取到的版本号
        fn GetApiVersion(&self)-> String;
        /// 初始化
        ///
        /// # Remarks
        /// 初始化运行环境,只有调用后,接口才开始工作
        fn Init(&self);
        /// 等待接口线程结束运行
        ///
        /// # Returns
        /// 线程退出代码
        fn Join(&self)-> i32;
        /// 获取当前交易日
        ///
        /// # Returns
        /// 获取到的交易日
        ///
        /// # Remarks
        /// 只有登录成功后,才能得到正确的交易日
        fn GetTradingDay(&self)-> String;
        /// 注册前置机网络地址
        ///
        /// # Parameters
        /// - `pszFrontAddress` — 前置机网络地址。
        ///
        /// # Remarks
        /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
        /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
        fn RegisterFront(&self, pszFrontAddress: String);
        /// 注册名字服务器网络地址
        ///
        /// # Parameters
        /// - `pszNsAddress` — 名字服务器网络地址。
        ///
        /// # Remarks
        /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
        /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
        /// RegisterNameServer优先于RegisterFront
        fn RegisterNameServer(&self, pszNsAddress: String);
        /// 注册名字服务器用户信息
        ///
        /// # Parameters
        /// - `pFensUserInfo` — 用户信息。
        fn RegisterFensUserInfo(&self, pFensUserInfo: FensUserInfoField);
        /// 订阅私有流。
        ///
        /// # Parameters
        /// - `nResumeType` — 私有流重传方式
        ///   - THOST_TERT_RESTART:从本交易日开始重传
        ///   - THOST_TERT_RESUME:从上次收到的续传
        ///   - THOST_TERT_QUICK:只传送登录后私有流的内容
        ///
        /// # Remarks
        /// 该方法要在Init方法前调用。若不调用则不会收到私有流的数据。
        fn SubscribePrivateTopic(&self, nResumeType: i32);
        /// 订阅公共流。
        ///
        /// # Parameters
        /// - `nResumeType` — 公共流重传方式
        ///   - THOST_TERT_RESTART:从本交易日开始重传
        ///   - THOST_TERT_RESUME:从上次收到的续传
        ///   - THOST_TERT_QUICK:只传送登录后公共流的内容
        ///   - THOST_TERT_NONE:取消订阅公共流
        ///
        /// # Remarks
        /// 该方法要在Init方法前调用。若不调用则不会收到公共流的数据。
        fn SubscribePublicTopic(&self, nResumeType: i32);
        /// 客户端认证请求
        fn ReqAuthenticate(&self, pReqAuthenticateField: ReqAuthenticateField, nRequestID: i32)-> i32;
        /// 注册用户终端信息，用于中继服务器多连接模式
        /// 需要在终端认证成功后，用户登录前调用该接口
        fn RegisterUserSystemInfo(&self, pUserSystemInfo: UserSystemInfoField)-> i32;
        /// 上报用户终端信息，用于中继服务器操作员登录模式
        /// 操作员登录后，可以多次调用该接口上报客户信息
        fn SubmitUserSystemInfo(&self, pUserSystemInfo: UserSystemInfoField)-> i32;
        /// 注册用户终端信息，用于中继服务器多连接模式.用于微信小程序等应用上报信息.
        fn RegisterWechatUserSystemInfo(&self, pUserSystemInfo: WechatUserSystemInfoField)-> i32;
        /// 上报用户终端信息，用于中继服务器操作员登录模式.用于微信小程序等应用上报信息.
        fn SubmitWechatUserSystemInfo(&self, pUserSystemInfo: WechatUserSystemInfoField)-> i32;
        /// 用户登录请求
        fn ReqUserLogin(&self, pReqUserLoginField: ReqUserLoginField, nRequestID: i32)-> i32;
        /// 登出请求
        fn ReqUserLogout(&self, pUserLogout: UserLogoutField, nRequestID: i32)-> i32;
        /// 用户口令更新请求
        fn ReqUserPasswordUpdate(&self, pUserPasswordUpdate: UserPasswordUpdateField, nRequestID: i32)-> i32;
        /// 资金账户口令更新请求
        fn ReqTradingAccountPasswordUpdate(&self, pTradingAccountPasswordUpdate: TradingAccountPasswordUpdateField, nRequestID: i32)-> i32;
        /// 查询用户当前支持的认证模式
        fn ReqUserAuthMethod(&self, pReqUserAuthMethod: ReqUserAuthMethodField, nRequestID: i32)-> i32;
        /// 用户发出获取图形验证码请求
        fn ReqGenUserCaptcha(&self, pReqGenUserCaptcha: ReqGenUserCaptchaField, nRequestID: i32)-> i32;
        /// 用户发出获取短信验证码请求
        fn ReqGenUserText(&self, pReqGenUserText: ReqGenUserTextField, nRequestID: i32)-> i32;
        /// 用户发出带有图片验证码的登陆请求
        fn ReqUserLoginWithCaptcha(&self, pReqUserLoginWithCaptcha: ReqUserLoginWithCaptchaField, nRequestID: i32)-> i32;
        /// 用户发出带有短信验证码的登陆请求
        fn ReqUserLoginWithText(&self, pReqUserLoginWithText: ReqUserLoginWithTextField, nRequestID: i32)-> i32;
        /// 用户发出带有动态口令的登陆请求
        fn ReqUserLoginWithOTP(&self, pReqUserLoginWithOTP: ReqUserLoginWithOTPField, nRequestID: i32)-> i32;
        /// 报单录入请求
        fn ReqOrderInsert(&self, pInputOrder: InputOrderField, nRequestID: i32)-> i32;
        /// 预埋单录入请求
        fn ReqParkedOrderInsert(&self, pParkedOrder: ParkedOrderField, nRequestID: i32)-> i32;
        /// 预埋撤单录入请求
        fn ReqParkedOrderAction(&self, pParkedOrderAction: ParkedOrderActionField, nRequestID: i32)-> i32;
        /// 报单操作请求
        fn ReqOrderAction(&self, pInputOrderAction: InputOrderActionField, nRequestID: i32)-> i32;
        /// 查询最大报单数量请求
        fn ReqQryMaxOrderVolume(&self, pQryMaxOrderVolume: QryMaxOrderVolumeField, nRequestID: i32)-> i32;
        /// 投资者结算结果确认
        fn ReqSettlementInfoConfirm(&self, pSettlementInfoConfirm: SettlementInfoConfirmField, nRequestID: i32)-> i32;
        /// 请求删除预埋单
        fn ReqRemoveParkedOrder(&self, pRemoveParkedOrder: RemoveParkedOrderField, nRequestID: i32)-> i32;
        /// 请求删除预埋撤单
        fn ReqRemoveParkedOrderAction(&self, pRemoveParkedOrderAction: RemoveParkedOrderActionField, nRequestID: i32)-> i32;
        /// 执行宣告录入请求
        fn ReqExecOrderInsert(&self, pInputExecOrder: InputExecOrderField, nRequestID: i32)-> i32;
        /// 执行宣告操作请求
        fn ReqExecOrderAction(&self, pInputExecOrderAction: InputExecOrderActionField, nRequestID: i32)-> i32;
        /// 询价录入请求
        fn ReqForQuoteInsert(&self, pInputForQuote: InputForQuoteField, nRequestID: i32)-> i32;
        /// 报价录入请求
        fn ReqQuoteInsert(&self, pInputQuote: InputQuoteField, nRequestID: i32)-> i32;
        /// 报价操作请求
        fn ReqQuoteAction(&self, pInputQuoteAction: InputQuoteActionField, nRequestID: i32)-> i32;
        /// 批量报单操作请求
        fn ReqBatchOrderAction(&self, pInputBatchOrderAction: InputBatchOrderActionField, nRequestID: i32)-> i32;
        /// 期权自对冲录入请求
        fn ReqOptionSelfCloseInsert(&self, pInputOptionSelfClose: InputOptionSelfCloseField, nRequestID: i32)-> i32;
        /// 期权自对冲操作请求
        fn ReqOptionSelfCloseAction(&self, pInputOptionSelfCloseAction: InputOptionSelfCloseActionField, nRequestID: i32)-> i32;
        /// 申请组合录入请求
        fn ReqCombActionInsert(&self, pInputCombAction: InputCombActionField, nRequestID: i32)-> i32;
        /// 请求查询报单
        fn ReqQryOrder(&self, pQryOrder: QryOrderField, nRequestID: i32)-> i32;
        /// 请求查询成交
        fn ReqQryTrade(&self, pQryTrade: QryTradeField, nRequestID: i32)-> i32;
        /// 请求查询投资者持仓
        fn ReqQryInvestorPosition(&self, pQryInvestorPosition: QryInvestorPositionField, nRequestID: i32)-> i32;
        /// 请求查询资金账户
        fn ReqQryTradingAccount(&self, pQryTradingAccount: QryTradingAccountField, nRequestID: i32)-> i32;
        /// 请求查询投资者
        fn ReqQryInvestor(&self, pQryInvestor: QryInvestorField, nRequestID: i32)-> i32;
        /// 请求查询交易编码
        fn ReqQryTradingCode(&self, pQryTradingCode: QryTradingCodeField, nRequestID: i32)-> i32;
        /// 请求查询合约保证金率
        fn ReqQryInstrumentMarginRate(&self, pQryInstrumentMarginRate: QryInstrumentMarginRateField, nRequestID: i32)-> i32;
        /// 请求查询合约手续费率
        fn ReqQryInstrumentCommissionRate(&self, pQryInstrumentCommissionRate: QryInstrumentCommissionRateField, nRequestID: i32)-> i32;
        /// 请求查询用户会话
        fn ReqQryUserSession(&self, pQryUserSession: QryUserSessionField, nRequestID: i32)-> i32;
        /// 请求查询交易所
        fn ReqQryExchange(&self, pQryExchange: QryExchangeField, nRequestID: i32)-> i32;
        /// 请求查询产品
        fn ReqQryProduct(&self, pQryProduct: QryProductField, nRequestID: i32)-> i32;
        /// 请求查询合约
        fn ReqQryInstrument(&self, pQryInstrument: QryInstrumentField, nRequestID: i32)-> i32;
        /// 请求查询行情
        fn ReqQryDepthMarketData(&self, pQryDepthMarketData: QryDepthMarketDataField, nRequestID: i32)-> i32;
        /// 请求查询交易员报盘机
        fn ReqQryTraderOffer(&self, pQryTraderOffer: QryTraderOfferField, nRequestID: i32)-> i32;
        /// 请求查询投资者结算结果
        fn ReqQrySettlementInfo(&self, pQrySettlementInfo: QrySettlementInfoField, nRequestID: i32)-> i32;
        /// 请求查询转帐银行
        fn ReqQryTransferBank(&self, pQryTransferBank: QryTransferBankField, nRequestID: i32)-> i32;
        /// 请求查询投资者持仓明细
        fn ReqQryInvestorPositionDetail(&self, pQryInvestorPositionDetail: QryInvestorPositionDetailField, nRequestID: i32)-> i32;
        /// 请求查询客户通知
        fn ReqQryNotice(&self, pQryNotice: QryNoticeField, nRequestID: i32)-> i32;
        /// 请求查询结算信息确认
        fn ReqQrySettlementInfoConfirm(&self, pQrySettlementInfoConfirm: QrySettlementInfoConfirmField, nRequestID: i32)-> i32;
        /// 请求查询投资者持仓明细
        fn ReqQryInvestorPositionCombineDetail(&self, pQryInvestorPositionCombineDetail: QryInvestorPositionCombineDetailField, nRequestID: i32)-> i32;
        /// 请求查询保证金监管系统经纪公司资金账户密钥
        fn ReqQryCFMMCTradingAccountKey(&self, pQryCFMMCTradingAccountKey: QryCFMMCTradingAccountKeyField, nRequestID: i32)-> i32;
        /// 请求查询仓单折抵信息
        fn ReqQryEWarrantOffset(&self, pQryEWarrantOffset: QryEWarrantOffsetField, nRequestID: i32)-> i32;
        /// 请求查询投资者品种/跨品种保证金
        fn ReqQryInvestorProductGroupMargin(&self, pQryInvestorProductGroupMargin: QryInvestorProductGroupMarginField, nRequestID: i32)-> i32;
        /// 请求查询交易所保证金率
        fn ReqQryExchangeMarginRate(&self, pQryExchangeMarginRate: QryExchangeMarginRateField, nRequestID: i32)-> i32;
        /// 请求查询交易所调整保证金率
        fn ReqQryExchangeMarginRateAdjust(&self, pQryExchangeMarginRateAdjust: QryExchangeMarginRateAdjustField, nRequestID: i32)-> i32;
        /// 请求查询汇率
        fn ReqQryExchangeRate(&self, pQryExchangeRate: QryExchangeRateField, nRequestID: i32)-> i32;
        /// 请求查询二级代理操作员银期权限
        fn ReqQrySecAgentACIDMap(&self, pQrySecAgentACIDMap: QrySecAgentACIDMapField, nRequestID: i32)-> i32;
        /// 请求查询产品报价汇率
        fn ReqQryProductExchRate(&self, pQryProductExchRate: QryProductExchRateField, nRequestID: i32)-> i32;
        /// 请求查询产品组
        fn ReqQryProductGroup(&self, pQryProductGroup: QryProductGroupField, nRequestID: i32)-> i32;
        /// 请求查询做市商合约手续费率
        fn ReqQryMMInstrumentCommissionRate(&self, pQryMMInstrumentCommissionRate: QryMMInstrumentCommissionRateField, nRequestID: i32)-> i32;
        /// 请求查询做市商期权合约手续费
        fn ReqQryMMOptionInstrCommRate(&self, pQryMMOptionInstrCommRate: QryMMOptionInstrCommRateField, nRequestID: i32)-> i32;
        /// 请求查询报单手续费
        fn ReqQryInstrumentOrderCommRate(&self, pQryInstrumentOrderCommRate: QryInstrumentOrderCommRateField, nRequestID: i32)-> i32;
        /// 请求查询资金账户
        fn ReqQrySecAgentTradingAccount(&self, pQryTradingAccount: QryTradingAccountField, nRequestID: i32)-> i32;
        /// 请求查询二级代理商资金校验模式
        fn ReqQrySecAgentCheckMode(&self, pQrySecAgentCheckMode: QrySecAgentCheckModeField, nRequestID: i32)-> i32;
        /// 请求查询二级代理商信息
        fn ReqQrySecAgentTradeInfo(&self, pQrySecAgentTradeInfo: QrySecAgentTradeInfoField, nRequestID: i32)-> i32;
        /// 请求查询期权交易成本
        fn ReqQryOptionInstrTradeCost(&self, pQryOptionInstrTradeCost: QryOptionInstrTradeCostField, nRequestID: i32)-> i32;
        /// 请求查询期权合约手续费
        fn ReqQryOptionInstrCommRate(&self, pQryOptionInstrCommRate: QryOptionInstrCommRateField, nRequestID: i32)-> i32;
        /// 请求查询执行宣告
        fn ReqQryExecOrder(&self, pQryExecOrder: QryExecOrderField, nRequestID: i32)-> i32;
        /// 请求查询询价
        fn ReqQryForQuote(&self, pQryForQuote: QryForQuoteField, nRequestID: i32)-> i32;
        /// 请求查询报价
        fn ReqQryQuote(&self, pQryQuote: QryQuoteField, nRequestID: i32)-> i32;
        /// 请求查询期权自对冲
        fn ReqQryOptionSelfClose(&self, pQryOptionSelfClose: QryOptionSelfCloseField, nRequestID: i32)-> i32;
        /// 请求查询投资单元
        fn ReqQryInvestUnit(&self, pQryInvestUnit: QryInvestUnitField, nRequestID: i32)-> i32;
        /// 请求查询组合合约安全系数
        fn ReqQryCombInstrumentGuard(&self, pQryCombInstrumentGuard: QryCombInstrumentGuardField, nRequestID: i32)-> i32;
        /// 请求查询申请组合
        fn ReqQryCombAction(&self, pQryCombAction: QryCombActionField, nRequestID: i32)-> i32;
        /// 请求查询转帐流水
        fn ReqQryTransferSerial(&self, pQryTransferSerial: QryTransferSerialField, nRequestID: i32)-> i32;
        /// 请求查询银期签约关系
        fn ReqQryAccountregister(&self, pQryAccountregister: QryAccountregisterField, nRequestID: i32)-> i32;
        /// 请求查询签约银行
        fn ReqQryContractBank(&self, pQryContractBank: QryContractBankField, nRequestID: i32)-> i32;
        /// 请求查询预埋单
        fn ReqQryParkedOrder(&self, pQryParkedOrder: QryParkedOrderField, nRequestID: i32)-> i32;
        /// 请求查询预埋撤单
        fn ReqQryParkedOrderAction(&self, pQryParkedOrderAction: QryParkedOrderActionField, nRequestID: i32)-> i32;
        /// 请求查询交易通知
        fn ReqQryTradingNotice(&self, pQryTradingNotice: QryTradingNoticeField, nRequestID: i32)-> i32;
        /// 请求查询经纪公司交易参数
        fn ReqQryBrokerTradingParams(&self, pQryBrokerTradingParams: QryBrokerTradingParamsField, nRequestID: i32)-> i32;
        /// 请求查询经纪公司交易算法
        fn ReqQryBrokerTradingAlgos(&self, pQryBrokerTradingAlgos: QryBrokerTradingAlgosField, nRequestID: i32)-> i32;
        /// 请求查询监控中心用户令牌
        fn ReqQueryCFMMCTradingAccountToken(&self, pQueryCFMMCTradingAccountToken: QueryCFMMCTradingAccountTokenField, nRequestID: i32)-> i32;
        /// 期货发起银行资金转期货请求
        fn ReqFromBankToFutureByFuture(&self, pReqTransfer: ReqTransferField, nRequestID: i32)-> i32;
        /// 期货发起期货资金转银行请求
        fn ReqFromFutureToBankByFuture(&self, pReqTransfer: ReqTransferField, nRequestID: i32)-> i32;
        /// 期货发起查询银行余额请求
        fn ReqQueryBankAccountMoneyByFuture(&self, pReqQueryAccount: ReqQueryAccountField, nRequestID: i32)-> i32;
        /// 请求查询分类合约
        fn ReqQryClassifiedInstrument(&self, pQryClassifiedInstrument: QryClassifiedInstrumentField, nRequestID: i32)-> i32;
        /// 请求组合优惠比例
        fn ReqQryCombPromotionParam(&self, pQryCombPromotionParam: QryCombPromotionParamField, nRequestID: i32)-> i32;
        /// 投资者风险结算持仓查询
        fn ReqQryRiskSettleInvstPosition(&self, pQryRiskSettleInvstPosition: QryRiskSettleInvstPositionField, nRequestID: i32)-> i32;
        /// 风险结算产品查询
        fn ReqQryRiskSettleProductStatus(&self, pQryRiskSettleProductStatus: QryRiskSettleProductStatusField, nRequestID: i32)-> i32;
        /// SPBM期货合约参数查询
        fn ReqQrySPBMFutureParameter(&self, pQrySPBMFutureParameter: QrySPBMFutureParameterField, nRequestID: i32)-> i32;
        /// SPBM期权合约参数查询
        fn ReqQrySPBMOptionParameter(&self, pQrySPBMOptionParameter: QrySPBMOptionParameterField, nRequestID: i32)-> i32;
        /// SPBM品种内对锁仓折扣参数查询
        fn ReqQrySPBMIntraParameter(&self, pQrySPBMIntraParameter: QrySPBMIntraParameterField, nRequestID: i32)-> i32;
        /// SPBM跨品种抵扣参数查询
        fn ReqQrySPBMInterParameter(&self, pQrySPBMInterParameter: QrySPBMInterParameterField, nRequestID: i32)-> i32;
        /// SPBM组合保证金套餐查询
        fn ReqQrySPBMPortfDefinition(&self, pQrySPBMPortfDefinition: QrySPBMPortfDefinitionField, nRequestID: i32)-> i32;
        /// 投资者SPBM套餐选择查询
        fn ReqQrySPBMInvestorPortfDef(&self, pQrySPBMInvestorPortfDef: QrySPBMInvestorPortfDefField, nRequestID: i32)-> i32;
        /// 投资者新型组合保证金系数查询
        fn ReqQryInvestorPortfMarginRatio(&self, pQryInvestorPortfMarginRatio: QryInvestorPortfMarginRatioField, nRequestID: i32)-> i32;
        /// 投资者产品SPBM明细查询
        fn ReqQryInvestorProdSPBMDetail(&self, pQryInvestorProdSPBMDetail: QryInvestorProdSPBMDetailField, nRequestID: i32)-> i32;
        /// 投资者商品组SPMM记录查询
        fn ReqQryInvestorCommoditySPMMMargin(&self, pQryInvestorCommoditySPMMMargin: QryInvestorCommoditySPMMMarginField, nRequestID: i32)-> i32;
        /// 投资者商品群SPMM记录查询
        fn ReqQryInvestorCommodityGroupSPMMMargin(&self, pQryInvestorCommodityGroupSPMMMargin: QryInvestorCommodityGroupSPMMMarginField, nRequestID: i32)-> i32;
        /// SPMM合约参数查询
        fn ReqQrySPMMInstParam(&self, pQrySPMMInstParam: QrySPMMInstParamField, nRequestID: i32)-> i32;
        /// SPMM产品参数查询
        fn ReqQrySPMMProductParam(&self, pQrySPMMProductParam: QrySPMMProductParamField, nRequestID: i32)-> i32;
        /// SPBM附加跨品种抵扣参数查询
        fn ReqQrySPBMAddOnInterParameter(&self, pQrySPBMAddOnInterParameter: QrySPBMAddOnInterParameterField, nRequestID: i32)-> i32;
        /// RCAMS产品组合信息查询
        fn ReqQryRCAMSCombProductInfo(&self, pQryRCAMSCombProductInfo: QryRCAMSCombProductInfoField, nRequestID: i32)-> i32;
        /// RCAMS同合约风险对冲参数查询
        fn ReqQryRCAMSInstrParameter(&self, pQryRCAMSInstrParameter: QryRCAMSInstrParameterField, nRequestID: i32)-> i32;
        /// RCAMS品种内风险对冲参数查询
        fn ReqQryRCAMSIntraParameter(&self, pQryRCAMSIntraParameter: QryRCAMSIntraParameterField, nRequestID: i32)-> i32;
        /// RCAMS跨品种风险折抵参数查询
        fn ReqQryRCAMSInterParameter(&self, pQryRCAMSInterParameter: QryRCAMSInterParameterField, nRequestID: i32)-> i32;
        /// RCAMS空头期权风险调整参数查询
        fn ReqQryRCAMSShortOptAdjustParam(&self, pQryRCAMSShortOptAdjustParam: QryRCAMSShortOptAdjustParamField, nRequestID: i32)-> i32;
        /// RCAMS策略组合持仓查询
        fn ReqQryRCAMSInvestorCombPosition(&self, pQryRCAMSInvestorCombPosition: QryRCAMSInvestorCombPositionField, nRequestID: i32)-> i32;
        /// 投资者品种RCAMS保证金查询
        fn ReqQryInvestorProdRCAMSMargin(&self, pQryInvestorProdRCAMSMargin: QryInvestorProdRCAMSMarginField, nRequestID: i32)-> i32;
        /// RULE合约保证金参数查询
        fn ReqQryRULEInstrParameter(&self, pQryRULEInstrParameter: QryRULEInstrParameterField, nRequestID: i32)-> i32;
        /// RULE品种内对锁仓折扣参数查询
        fn ReqQryRULEIntraParameter(&self, pQryRULEIntraParameter: QryRULEIntraParameterField, nRequestID: i32)-> i32;
        /// RULE跨品种抵扣参数查询
        fn ReqQryRULEInterParameter(&self, pQryRULEInterParameter: QryRULEInterParameterField, nRequestID: i32)-> i32;
        /// 投资者产品RULE保证金查询
        fn ReqQryInvestorProdRULEMargin(&self, pQryInvestorProdRULEMargin: QryInvestorProdRULEMarginField, nRequestID: i32)-> i32;
        /// 投资者新型组合保证金开关查询
        fn ReqQryInvestorPortfSetting(&self, pQryInvestorPortfSetting: QryInvestorPortfSettingField, nRequestID: i32)-> i32;
        /// 投资者申报费阶梯收取记录查询
        fn ReqQryInvestorInfoCommRec(&self, pQryInvestorInfoCommRec: QryInvestorInfoCommRecField, nRequestID: i32)-> i32;
        /// 组合腿信息查询
        fn ReqQryCombLeg(&self, pQryCombLeg: QryCombLegField, nRequestID: i32)-> i32;
        /// 对冲设置请求
        fn ReqOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, nRequestID: i32)-> i32;
        /// 对冲设置撤销请求
        fn ReqCancelOffsetSetting(&self, pInputOffsetSetting: InputOffsetSettingField, nRequestID: i32)-> i32;
        /// 投资者对冲设置查询
        fn ReqQryOffsetSetting(&self, pQryOffsetSetting: QryOffsetSettingField, nRequestID: i32)-> i32;
    }

    /// 信息分发
    #[derive(Debug, Clone, Default)]
    struct DisseminationField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 序列系列号
        SequenceSeries: u16,
        /// 序列号
        SequenceNo: i32,
    }
    /// 用户登录请求
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 密码
        Password: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// Mac地址
        MacAddress: String,
        /// 动态密码
        OneTimePassword: String,
        /// 登录备注
        LoginRemark: String,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 终端IP地址
        ClientIPAddress: String,
    }
    /// 用户登录应答
    #[derive(Debug, Clone, Default)]
    struct RspUserLoginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 登录成功时间
        LoginTime: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 交易系统名称
        SystemName: String,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 最大报单引用
        MaxOrderRef: String,
        /// 上期所时间
        SHFETime: String,
        /// 大商所时间
        DCETime: String,
        /// 郑商所时间
        CZCETime: String,
        /// 中金所时间
        FFEXTime: String,
        /// 能源中心时间
        INETime: String,
        /// 后台版本信息
        SysVersion: String,
        /// 广期所时间
        GFEXTime: String,
        /// 当前登录中心号
        LoginDRIdentityID: i32,
        /// 用户所属中心号
        UserDRIdentityID: i32,
        /// 上次登陆时间
        LastLoginTime: String,
        /// 预留信息
        ReserveInfo: String,
    }
    /// 用户登出请求
    #[derive(Debug, Clone, Default)]
    struct UserLogoutField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 强制交易员退出
    #[derive(Debug, Clone, Default)]
    struct ForceUserLogoutField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 客户端认证请求
    #[derive(Debug, Clone, Default)]
    struct ReqAuthenticateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 认证码
        AuthCode: String,
        /// App代码
        AppID: String,
    }
    /// 客户端认证响应
    #[derive(Debug, Clone, Default)]
    struct RspAuthenticateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// App代码
        AppID: String,
        /// App类型
        AppType: u8,
    }
    /// 客户端认证信息
    #[derive(Debug, Clone, Default)]
    struct AuthenticationInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 认证信息
        AuthInfo: String,
        /// 是否为认证结果
        IsResult: i32,
        /// App代码
        AppID: String,
        /// App类型
        AppType: u8,
        /// 终端IP地址
        ClientIPAddress: String,
    }
    /// 用户登录应答2
    #[derive(Debug, Clone, Default)]
    struct RspUserLogin2Field {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 登录成功时间
        LoginTime: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 交易系统名称
        SystemName: String,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 最大报单引用
        MaxOrderRef: String,
        /// 上期所时间
        SHFETime: String,
        /// 大商所时间
        DCETime: String,
        /// 郑商所时间
        CZCETime: String,
        /// 中金所时间
        FFEXTime: String,
        /// 能源中心时间
        INETime: String,
        /// 随机串
        RandomString: Vec<u8>,
    }
    /// 银期转帐报文头
    #[derive(Debug, Clone, Default)]
    struct TransferHeaderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 版本号，常量，1.0
        Version: String,
        /// 交易代码，必填
        TradeCode: String,
        /// 交易日期，必填，格式：yyyymmdd
        TradeDate: String,
        /// 交易时间，必填，格式：hhmmss
        TradeTime: String,
        /// 发起方流水号，N/A
        TradeSerial: String,
        /// 期货公司代码，必填
        FutureID: String,
        /// 银行代码，根据查询银行得到，必填
        BankID: String,
        /// 银行分中心代码，根据查询银行得到，必填
        BankBrchID: String,
        /// 操作员，N/A
        OperNo: String,
        /// 交易设备类型，N/A
        DeviceID: String,
        /// 记录数，N/A
        RecordNum: Vec<u8>,
        /// 会话编号，N/A
        SessionID: i32,
        /// 请求编号，N/A
        RequestID: i32,
    }
    /// 银行资金转期货请求，TradeCode=202001
    #[derive(Debug, Clone, Default)]
    struct TransferBankToFutureReqField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 期货资金账户
        FutureAccount: String,
        /// 密码标志
        FuturePwdFlag: u8,
        /// 密码
        FutureAccPwd: String,
        /// 转账金额
        TradeAmt: f64,
        /// 客户手续费
        CustFee: f64,
        /// 币种：RMB-人民币 USD-美圆 HKD-港元
        CurrencyCode: String,
    }
    /// 银行资金转期货请求响应
    #[derive(Debug, Clone, Default)]
    struct TransferBankToFutureRspField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 响应代码
        RetCode: String,
        /// 响应信息
        RetInfo: String,
        /// 资金账户
        FutureAccount: String,
        /// 转帐金额
        TradeAmt: f64,
        /// 应收客户手续费
        CustFee: f64,
        /// 币种
        CurrencyCode: String,
    }
    /// 期货资金转银行请求，TradeCode=202002
    #[derive(Debug, Clone, Default)]
    struct TransferFutureToBankReqField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 期货资金账户
        FutureAccount: String,
        /// 密码标志
        FuturePwdFlag: u8,
        /// 密码
        FutureAccPwd: String,
        /// 转账金额
        TradeAmt: f64,
        /// 客户手续费
        CustFee: f64,
        /// 币种：RMB-人民币 USD-美圆 HKD-港元
        CurrencyCode: String,
    }
    /// 期货资金转银行请求响应
    #[derive(Debug, Clone, Default)]
    struct TransferFutureToBankRspField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 响应代码
        RetCode: String,
        /// 响应信息
        RetInfo: String,
        /// 资金账户
        FutureAccount: String,
        /// 转帐金额
        TradeAmt: f64,
        /// 应收客户手续费
        CustFee: f64,
        /// 币种
        CurrencyCode: String,
    }
    /// 查询银行资金请求，TradeCode=204002
    #[derive(Debug, Clone, Default)]
    struct TransferQryBankReqField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 期货资金账户
        FutureAccount: String,
        /// 密码标志
        FuturePwdFlag: u8,
        /// 密码
        FutureAccPwd: String,
        /// 币种：RMB-人民币 USD-美圆 HKD-港元
        CurrencyCode: String,
    }
    /// 查询银行资金请求响应
    #[derive(Debug, Clone, Default)]
    struct TransferQryBankRspField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 响应代码
        RetCode: String,
        /// 响应信息
        RetInfo: String,
        /// 资金账户
        FutureAccount: String,
        /// 银行余额
        TradeAmt: f64,
        /// 银行可用余额
        UseAmt: f64,
        /// 银行可取余额
        FetchAmt: f64,
        /// 币种
        CurrencyCode: String,
    }
    /// 查询银行交易明细请求，TradeCode=204999
    #[derive(Debug, Clone, Default)]
    struct TransferQryDetailReqField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 期货资金账户
        FutureAccount: String,
    }
    /// 查询银行交易明细请求响应
    #[derive(Debug, Clone, Default)]
    struct TransferQryDetailRspField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 交易代码
        TradeCode: String,
        /// 期货流水号
        FutureSerial: i32,
        /// 期货公司代码
        FutureID: String,
        /// 资金帐号
        FutureAccount: String,
        /// 银行流水号
        BankSerial: i32,
        /// 银行代码
        BankID: String,
        /// 银行分中心代码
        BankBrchID: String,
        /// 银行账号
        BankAccount: String,
        /// 证件号码
        CertCode: String,
        /// 货币代码
        CurrencyCode: String,
        /// 发生金额
        TxAmount: f64,
        /// 有效标志
        Flag: u8,
    }
    /// 响应信息
    #[derive(Debug, Clone, Default)]
    struct RspInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 交易所
    #[derive(Debug, Clone, Default)]
    struct ExchangeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所名称
        ExchangeName: String,
        /// 交易所属性
        ExchangeProperty: u8,
    }
    /// 产品
    #[derive(Debug, Clone, Default)]
    struct ProductField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品名称
        ProductName: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品类型
        ProductClass: u8,
        /// 合约数量乘数
        VolumeMultiple: i32,
        /// 最小变动价位
        PriceTick: f64,
        /// 市价单最大下单量
        MaxMarketOrderVolume: i32,
        /// 市价单最小下单量
        MinMarketOrderVolume: i32,
        /// 限价单最大下单量
        MaxLimitOrderVolume: i32,
        /// 限价单最小下单量
        MinLimitOrderVolume: i32,
        /// 持仓类型
        PositionType: u8,
        /// 持仓日期类型
        PositionDateType: u8,
        /// 平仓处理类型
        CloseDealType: u8,
        /// 交易币种类型
        TradeCurrencyID: String,
        /// 质押资金可用范围
        MortgageFundUseRange: u8,
        /// 合约基础商品乘数
        UnderlyingMultiple: f64,
        /// 产品代码
        ProductID: String,
        /// 交易所产品代码
        ExchangeProductID: String,
        /// 开仓量限制粒度
        OpenLimitControlLevel: u8,
        /// 报单频率控制粒度
        OrderFreqControlLevel: u8,
    }
    /// 合约
    #[derive(Debug, Clone, Default)]
    struct InstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约名称
        InstrumentName: String,
        /// 产品类型
        ProductClass: u8,
        /// 交割年份
        DeliveryYear: i32,
        /// 交割月
        DeliveryMonth: i32,
        /// 市价单最大下单量
        MaxMarketOrderVolume: i32,
        /// 市价单最小下单量
        MinMarketOrderVolume: i32,
        /// 限价单最大下单量
        MaxLimitOrderVolume: i32,
        /// 限价单最小下单量
        MinLimitOrderVolume: i32,
        /// 合约数量乘数
        VolumeMultiple: i32,
        /// 最小变动价位
        PriceTick: f64,
        /// 创建日
        CreateDate: String,
        /// 上市日
        OpenDate: String,
        /// 到期日
        ExpireDate: String,
        /// 开始交割日
        StartDelivDate: String,
        /// 结束交割日
        EndDelivDate: String,
        /// 合约生命周期状态
        InstLifePhase: u8,
        /// 当前是否交易
        IsTrading: i32,
        /// 持仓类型
        PositionType: u8,
        /// 持仓日期类型
        PositionDateType: u8,
        /// 多头保证金率
        LongMarginRatio: f64,
        /// 空头保证金率
        ShortMarginRatio: f64,
        /// 是否使用大额单边保证金算法
        MaxMarginSideAlgorithm: u8,
        /// 执行价
        StrikePrice: f64,
        /// 期权类型
        OptionsType: u8,
        /// 合约基础商品乘数
        UnderlyingMultiple: f64,
        /// 组合类型
        CombinationType: u8,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 产品代码
        ProductID: String,
        /// 基础商品代码
        UnderlyingInstrID: String,
    }
    /// 经纪公司
    #[derive(Debug, Clone, Default)]
    struct BrokerField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 经纪公司简称
        BrokerAbbr: String,
        /// 经纪公司名称
        BrokerName: String,
        /// 是否活跃
        IsActive: i32,
    }
    /// 交易所交易员
    #[derive(Debug, Clone, Default)]
    struct TraderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 会员代码
        ParticipantID: String,
        /// 密码
        Password: String,
        /// 安装数量
        InstallCount: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 撤单时选择席位算法
        OrderCancelAlg: u8,
        /// 交易报盘安装数量
        TradeInstallCount: i32,
        /// 行情报盘安装数量
        MDInstallCount: i32,
    }
    /// 投资者
    #[derive(Debug, Clone, Default)]
    struct InvestorField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者代码
        InvestorID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者分组代码
        InvestorGroupID: String,
        /// 投资者名称
        InvestorName: String,
        /// 证件类型
        IdentifiedCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 是否活跃
        IsActive: i32,
        /// 联系电话
        Telephone: String,
        /// 通讯地址
        Address: String,
        /// 开户日期
        OpenDate: String,
        /// 手机
        Mobile: String,
        /// 手续费率模板代码
        CommModelID: String,
        /// 保证金率模板代码
        MarginModelID: String,
        /// 是否频率控制
        IsOrderFreq: u8,
        /// 是否开仓限制
        IsOpenVolLimit: u8,
    }
    /// 交易编码
    #[derive(Debug, Clone, Default)]
    struct TradingCodeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者代码
        InvestorID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 客户代码
        ClientID: String,
        /// 是否活跃
        IsActive: i32,
        /// 交易编码类型
        ClientIDType: u8,
        /// 营业部编号
        BranchID: String,
        /// 业务类型
        BizType: u8,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 会员编码和经纪公司编码对照表
    #[derive(Debug, Clone, Default)]
    struct PartBrokerField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 是否活跃
        IsActive: i32,
    }
    /// 管理用户
    #[derive(Debug, Clone, Default)]
    struct SuperUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 用户代码
        UserID: String,
        /// 用户名称
        UserName: String,
        /// 密码
        Password: String,
        /// 是否活跃
        IsActive: i32,
    }
    /// 管理用户功能权限
    #[derive(Debug, Clone, Default)]
    struct SuperUserFunctionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 用户代码
        UserID: String,
        /// 功能代码
        FunctionCode: u8,
    }
    /// 投资者组
    #[derive(Debug, Clone, Default)]
    struct InvestorGroupField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者分组代码
        InvestorGroupID: String,
        /// 投资者分组名称
        InvestorGroupName: String,
    }
    /// 资金账户
    #[derive(Debug, Clone, Default)]
    struct TradingAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 上次质押金额
        PreMortgage: f64,
        /// 上次信用额度
        PreCredit: f64,
        /// 上次存款额
        PreDeposit: f64,
        /// 上次结算准备金
        PreBalance: f64,
        /// 上次占用的保证金
        PreMargin: f64,
        /// 利息基数
        InterestBase: f64,
        /// 利息收入
        Interest: f64,
        /// 入金金额
        Deposit: f64,
        /// 出金金额
        Withdraw: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 当前保证金总额
        CurrMargin: f64,
        /// 资金差额
        CashIn: f64,
        /// 手续费
        Commission: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 期货结算准备金
        Balance: f64,
        /// 可用资金
        Available: f64,
        /// 可取资金
        WithdrawQuota: f64,
        /// 基本准备金
        Reserve: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 信用额度
        Credit: f64,
        /// 质押金额
        Mortgage: f64,
        /// 交易所保证金
        ExchangeMargin: f64,
        /// 投资者交割保证金
        DeliveryMargin: f64,
        /// 交易所交割保证金
        ExchangeDeliveryMargin: f64,
        /// 保底期货结算准备金
        ReserveBalance: f64,
        /// 币种代码
        CurrencyID: String,
        /// 上次货币质入金额
        PreFundMortgageIn: f64,
        /// 上次货币质出金额
        PreFundMortgageOut: f64,
        /// 货币质入金额
        FundMortgageIn: f64,
        /// 货币质出金额
        FundMortgageOut: f64,
        /// 货币质押余额
        FundMortgageAvailable: f64,
        /// 可质押货币金额
        MortgageableFund: f64,
        /// 特殊产品占用保证金
        SpecProductMargin: f64,
        /// 特殊产品冻结保证金
        SpecProductFrozenMargin: f64,
        /// 特殊产品手续费
        SpecProductCommission: f64,
        /// 特殊产品冻结手续费
        SpecProductFrozenCommission: f64,
        /// 特殊产品持仓盈亏
        SpecProductPositionProfit: f64,
        /// 特殊产品平仓盈亏
        SpecProductCloseProfit: f64,
        /// 根据持仓盈亏算法计算的特殊产品持仓盈亏
        SpecProductPositionProfitByAlg: f64,
        /// 特殊产品交易所保证金
        SpecProductExchangeMargin: f64,
        /// 业务类型
        BizType: u8,
        /// 延时换汇冻结金额
        FrozenSwap: f64,
        /// 剩余换汇额度
        RemainSwap: f64,
        /// 期权市值
        OptionValue: f64,
    }
    /// 投资者持仓
    #[derive(Debug, Clone, Default)]
    struct InvestorPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 持仓多空方向
        PosiDirection: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 持仓日期
        PositionDate: u8,
        /// 上日持仓
        YdPosition: i32,
        /// 今日持仓
        Position: i32,
        /// 多头冻结
        LongFrozen: i32,
        /// 空头冻结
        ShortFrozen: i32,
        /// 开仓冻结金额
        LongFrozenAmount: f64,
        /// 开仓冻结金额
        ShortFrozenAmount: f64,
        /// 开仓量
        OpenVolume: i32,
        /// 平仓量
        CloseVolume: i32,
        /// 开仓金额
        OpenAmount: f64,
        /// 平仓金额
        CloseAmount: f64,
        /// 持仓成本
        PositionCost: f64,
        /// 上次占用的保证金
        PreMargin: f64,
        /// 占用的保证金
        UseMargin: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 资金差额
        CashIn: f64,
        /// 手续费
        Commission: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 开仓成本
        OpenCost: f64,
        /// 交易所保证金
        ExchangeMargin: f64,
        /// 组合成交形成的持仓
        CombPosition: i32,
        /// 组合多头冻结
        CombLongFrozen: i32,
        /// 组合空头冻结
        CombShortFrozen: i32,
        /// 逐日盯市平仓盈亏
        CloseProfitByDate: f64,
        /// 逐笔对冲平仓盈亏
        CloseProfitByTrade: f64,
        /// 今日持仓
        TodayPosition: i32,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 执行冻结
        StrikeFrozen: i32,
        /// 执行冻结金额
        StrikeFrozenAmount: f64,
        /// 放弃执行冻结
        AbandonFrozen: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 执行冻结的昨仓
        YdStrikeFrozen: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// 持仓成本差值
        PositionCostOffset: f64,
        /// tas持仓手数
        TasPosition: i32,
        /// tas持仓成本
        TasPositionCost: f64,
        /// 合约代码
        InstrumentID: String,
        /// 期权市值
        OptionValue: f64,
    }
    /// 合约保证金率
    #[derive(Debug, Clone, Default)]
    struct InstrumentMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 是否相对交易所收取
        IsRelative: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 合约手续费率
    #[derive(Debug, Clone, Default)]
    struct InstrumentCommissionRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 业务类型
        BizType: u8,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 深度行情
    #[derive(Debug, Clone, Default)]
    struct DepthMarketDataField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 最新价
        LastPrice: f64,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 昨收盘
        PreClosePrice: f64,
        /// 昨持仓量
        PreOpenInterest: f64,
        /// 今开盘
        OpenPrice: f64,
        /// 最高价
        HighestPrice: f64,
        /// 最低价
        LowestPrice: f64,
        /// 数量
        Volume: i32,
        /// 成交金额
        Turnover: f64,
        /// 持仓量
        OpenInterest: f64,
        /// 今收盘
        ClosePrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 涨停板价
        UpperLimitPrice: f64,
        /// 跌停板价
        LowerLimitPrice: f64,
        /// 昨虚实度
        PreDelta: f64,
        /// 今虚实度
        CurrDelta: f64,
        /// 最后修改时间
        UpdateTime: String,
        /// 最后修改毫秒
        UpdateMillisec: i32,
        /// 申买价一
        BidPrice1: f64,
        /// 申买量一
        BidVolume1: i32,
        /// 申卖价一
        AskPrice1: f64,
        /// 申卖量一
        AskVolume1: i32,
        /// 申买价二
        BidPrice2: f64,
        /// 申买量二
        BidVolume2: i32,
        /// 申卖价二
        AskPrice2: f64,
        /// 申卖量二
        AskVolume2: i32,
        /// 申买价三
        BidPrice3: f64,
        /// 申买量三
        BidVolume3: i32,
        /// 申卖价三
        AskPrice3: f64,
        /// 申卖量三
        AskVolume3: i32,
        /// 申买价四
        BidPrice4: f64,
        /// 申买量四
        BidVolume4: i32,
        /// 申卖价四
        AskPrice4: f64,
        /// 申卖量四
        AskVolume4: i32,
        /// 申买价五
        BidPrice5: f64,
        /// 申买量五
        BidVolume5: i32,
        /// 申卖价五
        AskPrice5: f64,
        /// 申卖量五
        AskVolume5: i32,
        /// 当日均价
        AveragePrice: f64,
        /// 业务日期
        ActionDay: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 上带价
        BandingUpperPrice: f64,
        /// 下带价
        BandingLowerPrice: f64,
    }
    /// 投资者合约交易权限
    #[derive(Debug, Clone, Default)]
    struct InstrumentTradingRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易权限
        TradingRight: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 经纪公司用户
    #[derive(Debug, Clone, Default)]
    struct BrokerUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户名称
        UserName: String,
        /// 用户类型
        UserType: u8,
        /// 是否活跃
        IsActive: i32,
        /// 是否使用令牌
        IsUsingOTP: i32,
        /// 是否强制终端认证
        IsAuthForce: i32,
    }
    /// 经纪公司用户口令
    #[derive(Debug, Clone, Default)]
    struct BrokerUserPasswordField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 密码
        Password: String,
        /// 上次修改时间
        LastUpdateTime: String,
        /// 上次登陆时间
        LastLoginTime: String,
        /// 密码过期时间
        ExpireDate: String,
        /// 弱密码过期时间
        WeakExpireDate: String,
    }
    /// 经纪公司用户功能权限
    #[derive(Debug, Clone, Default)]
    struct BrokerUserFunctionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 经纪公司功能代码
        BrokerFunctionCode: u8,
    }
    /// 交易所交易员报盘机
    #[derive(Debug, Clone, Default)]
    struct TraderOfferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 会员代码
        ParticipantID: String,
        /// 密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 交易所交易员连接状态
        TraderConnectStatus: u8,
        /// 发出连接请求的日期
        ConnectRequestDate: String,
        /// 发出连接请求的时间
        ConnectRequestTime: String,
        /// 上次报告日期
        LastReportDate: String,
        /// 上次报告时间
        LastReportTime: String,
        /// 完成连接日期
        ConnectDate: String,
        /// 完成连接时间
        ConnectTime: String,
        /// 启动日期
        StartDate: String,
        /// 启动时间
        StartTime: String,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 本席位最大成交编号
        MaxTradeID: String,
        /// 本席位最大报单备拷
        MaxOrderMessageReference: Vec<u8>,
        /// 撤单时选择席位算法
        OrderCancelAlg: u8,
    }
    /// 投资者结算结果
    #[derive(Debug, Clone, Default)]
    struct SettlementInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 序号
        SequenceNo: i32,
        /// 消息正文
        Content: Vec<u8>,
        /// 投资者帐号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 合约保证金率调整
    #[derive(Debug, Clone, Default)]
    struct InstrumentMarginRateAdjustField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 是否相对交易所收取
        IsRelative: i32,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所保证金率
    #[derive(Debug, Clone, Default)]
    struct ExchangeMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所保证金率调整
    #[derive(Debug, Clone, Default)]
    struct ExchangeMarginRateAdjustField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 跟随交易所投资者多头保证金率
        LongMarginRatioByMoney: f64,
        /// 跟随交易所投资者多头保证金费
        LongMarginRatioByVolume: f64,
        /// 跟随交易所投资者空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 跟随交易所投资者空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 交易所多头保证金率
        ExchLongMarginRatioByMoney: f64,
        /// 交易所多头保证金费
        ExchLongMarginRatioByVolume: f64,
        /// 交易所空头保证金率
        ExchShortMarginRatioByMoney: f64,
        /// 交易所空头保证金费
        ExchShortMarginRatioByVolume: f64,
        /// 不跟随交易所投资者多头保证金率
        NoLongMarginRatioByMoney: f64,
        /// 不跟随交易所投资者多头保证金费
        NoLongMarginRatioByVolume: f64,
        /// 不跟随交易所投资者空头保证金率
        NoShortMarginRatioByMoney: f64,
        /// 不跟随交易所投资者空头保证金费
        NoShortMarginRatioByVolume: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 汇率
    #[derive(Debug, Clone, Default)]
    struct ExchangeRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 源币种
        FromCurrencyID: String,
        /// 源币种单位数量
        FromCurrencyUnit: f64,
        /// 目标币种
        ToCurrencyID: String,
        /// 汇率
        ExchangeRate: f64,
    }
    /// 结算引用
    #[derive(Debug, Clone, Default)]
    struct SettlementRefField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
    }
    /// 当前时间
    #[derive(Debug, Clone, Default)]
    struct CurrentTimeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 当前交易日
        CurrDate: String,
        /// 当前时间
        CurrTime: String,
        /// 当前时间（毫秒）
        CurrMillisec: i32,
        /// 自然日期
        ActionDay: String,
    }
    /// 通讯阶段
    #[derive(Debug, Clone, Default)]
    struct CommPhaseField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 通讯时段编号
        CommPhaseNo: u16,
        /// 系统编号
        SystemID: String,
    }
    /// 登录信息
    #[derive(Debug, Clone, Default)]
    struct LoginInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 登录日期
        LoginDate: String,
        /// 登录时间
        LoginTime: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// 系统名称
        SystemName: String,
        /// 密码,已弃用
        PasswordDeprecated: Vec<u8>,
        /// 最大报单引用
        MaxOrderRef: String,
        /// 上期所时间
        SHFETime: String,
        /// 大商所时间
        DCETime: String,
        /// 郑商所时间
        CZCETime: String,
        /// 中金所时间
        FFEXTime: String,
        /// Mac地址
        MacAddress: String,
        /// 动态密码
        OneTimePassword: String,
        /// 能源中心时间
        INETime: String,
        /// 查询时是否需要流控
        IsQryControl: i32,
        /// 登录备注
        LoginRemark: String,
        /// 密码
        Password: String,
        /// IP地址
        IPAddress: String,
    }
    /// 登录信息
    #[derive(Debug, Clone, Default)]
    struct LogoutAllField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 系统名称
        SystemName: String,
    }
    /// 前置状态
    #[derive(Debug, Clone, Default)]
    struct FrontStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置编号
        FrontID: i32,
        /// 上次报告日期
        LastReportDate: String,
        /// 上次报告时间
        LastReportTime: String,
        /// 是否活跃
        IsActive: i32,
    }
    /// 用户口令变更
    #[derive(Debug, Clone, Default)]
    struct UserPasswordUpdateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 原来的口令
        OldPassword: String,
        /// 新的口令
        NewPassword: String,
    }
    /// 输入报单
    #[derive(Debug, Clone, Default)]
    struct InputOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单引用
        OrderRef: String,
        /// 用户代码
        UserID: String,
        /// 报单价格条件
        OrderPriceType: u8,
        /// 买卖方向
        Direction: u8,
        /// 组合开平标志
        CombOffsetFlag: String,
        /// 组合投机套保标志
        CombHedgeFlag: String,
        /// 价格
        LimitPrice: f64,
        /// 数量
        VolumeTotalOriginal: i32,
        /// 有效期类型
        TimeCondition: u8,
        /// GTD日期
        GTDDate: String,
        /// 成交量类型
        VolumeCondition: u8,
        /// 最小成交量
        MinVolume: i32,
        /// 触发条件
        ContingentCondition: u8,
        /// 止损价
        StopPrice: f64,
        /// 强平原因
        ForceCloseReason: u8,
        /// 自动挂起标志
        IsAutoSuspend: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 请求编号
        RequestID: i32,
        /// 用户强平标志
        UserForceClose: i32,
        /// 互换单标志
        IsSwapOrder: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 报单
    #[derive(Debug, Clone, Default)]
    struct OrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单引用
        OrderRef: String,
        /// 用户代码
        UserID: String,
        /// 报单价格条件
        OrderPriceType: u8,
        /// 买卖方向
        Direction: u8,
        /// 组合开平标志
        CombOffsetFlag: String,
        /// 组合投机套保标志
        CombHedgeFlag: String,
        /// 价格
        LimitPrice: f64,
        /// 数量
        VolumeTotalOriginal: i32,
        /// 有效期类型
        TimeCondition: u8,
        /// GTD日期
        GTDDate: String,
        /// 成交量类型
        VolumeCondition: u8,
        /// 最小成交量
        MinVolume: i32,
        /// 触发条件
        ContingentCondition: u8,
        /// 止损价
        StopPrice: f64,
        /// 强平原因
        ForceCloseReason: u8,
        /// 自动挂起标志
        IsAutoSuspend: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 请求编号
        RequestID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报单提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 报单编号
        OrderSysID: String,
        /// 报单来源
        OrderSource: u8,
        /// 报单状态
        OrderStatus: u8,
        /// 报单类型
        OrderType: u8,
        /// 今成交数量
        VolumeTraded: i32,
        /// 剩余数量
        VolumeTotal: i32,
        /// 报单日期
        InsertDate: String,
        /// 委托时间
        InsertTime: String,
        /// 激活时间
        ActiveTime: String,
        /// 挂起时间
        SuspendTime: String,
        /// 最后修改时间
        UpdateTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 最后修改交易所交易员代码
        ActiveTraderID: String,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 状态信息
        StatusMsg: String,
        /// 用户强平标志
        UserForceClose: i32,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司报单编号
        BrokerOrderSeq: i32,
        /// 相关报单
        RelativeOrderSysID: String,
        /// 郑商所成交数量
        ZCETotalTradedVolume: i32,
        /// 互换单标志
        IsSwapOrder: i32,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 交易所报单
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 报单价格条件
        OrderPriceType: u8,
        /// 买卖方向
        Direction: u8,
        /// 组合开平标志
        CombOffsetFlag: String,
        /// 组合投机套保标志
        CombHedgeFlag: String,
        /// 价格
        LimitPrice: f64,
        /// 数量
        VolumeTotalOriginal: i32,
        /// 有效期类型
        TimeCondition: u8,
        /// GTD日期
        GTDDate: String,
        /// 成交量类型
        VolumeCondition: u8,
        /// 最小成交量
        MinVolume: i32,
        /// 触发条件
        ContingentCondition: u8,
        /// 止损价
        StopPrice: f64,
        /// 强平原因
        ForceCloseReason: u8,
        /// 自动挂起标志
        IsAutoSuspend: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 请求编号
        RequestID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报单提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 报单编号
        OrderSysID: String,
        /// 报单来源
        OrderSource: u8,
        /// 报单状态
        OrderStatus: u8,
        /// 报单类型
        OrderType: u8,
        /// 今成交数量
        VolumeTraded: i32,
        /// 剩余数量
        VolumeTotal: i32,
        /// 报单日期
        InsertDate: String,
        /// 委托时间
        InsertTime: String,
        /// 激活时间
        ActiveTime: String,
        /// 挂起时间
        SuspendTime: String,
        /// 最后修改时间
        UpdateTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 最后修改交易所交易员代码
        ActiveTraderID: String,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所报单插入失败
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderInsertErrorField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 输入报单操作
    #[derive(Debug, Clone, Default)]
    struct InputOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单操作引用
        OrderActionRef: i32,
        /// 报单引用
        OrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 价格
        LimitPrice: f64,
        /// 数量变化
        VolumeChange: i32,
        /// 用户代码
        UserID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 报单操作
    #[derive(Debug, Clone, Default)]
    struct OrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单操作引用
        OrderActionRef: i32,
        /// 报单引用
        OrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 价格
        LimitPrice: f64,
        /// 数量变化
        VolumeChange: i32,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 状态信息
        StatusMsg: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 交易所报单操作
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 价格
        LimitPrice: f64,
        /// 数量变化
        VolumeChange: i32,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所报单操作失败
    #[derive(Debug, Clone, Default)]
    struct ExchangeOrderActionErrorField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 交易所成交
    #[derive(Debug, Clone, Default)]
    struct ExchangeTradeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 成交编号
        TradeID: String,
        /// 买卖方向
        Direction: u8,
        /// 报单编号
        OrderSysID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易角色
        TradingRole: u8,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 价格
        Price: f64,
        /// 数量
        Volume: i32,
        /// 成交时期
        TradeDate: String,
        /// 成交时间
        TradeTime: String,
        /// 成交类型
        TradeType: u8,
        /// 成交价来源
        PriceSource: u8,
        /// 交易所交易员代码
        TraderID: String,
        /// 本地报单编号
        OrderLocalID: String,
        /// 结算会员编号
        ClearingPartID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 序号
        SequenceNo: i32,
        /// 成交来源
        TradeSource: u8,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 成交
    #[derive(Debug, Clone, Default)]
    struct TradeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单引用
        OrderRef: String,
        /// 用户代码
        UserID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 成交编号
        TradeID: String,
        /// 买卖方向
        Direction: u8,
        /// 报单编号
        OrderSysID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易角色
        TradingRole: u8,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 价格
        Price: f64,
        /// 数量
        Volume: i32,
        /// 成交时期
        TradeDate: String,
        /// 成交时间
        TradeTime: String,
        /// 成交类型
        TradeType: u8,
        /// 成交价来源
        PriceSource: u8,
        /// 交易所交易员代码
        TraderID: String,
        /// 本地报单编号
        OrderLocalID: String,
        /// 结算会员编号
        ClearingPartID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 序号
        SequenceNo: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 经纪公司报单编号
        BrokerOrderSeq: i32,
        /// 成交来源
        TradeSource: u8,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 用户会话
    #[derive(Debug, Clone, Default)]
    struct UserSessionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 登录日期
        LoginDate: String,
        /// 登录时间
        LoginTime: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// Mac地址
        MacAddress: String,
        /// 登录备注
        LoginRemark: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询最大报单数量
    #[derive(Debug, Clone, Default)]
    struct QryMaxOrderVolumeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 买卖方向
        Direction: u8,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 最大允许报单数量
        MaxVolume: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 投资者结算结果确认信息
    #[derive(Debug, Clone, Default)]
    struct SettlementInfoConfirmField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 确认日期
        ConfirmDate: String,
        /// 确认时间
        ConfirmTime: String,
        /// 结算编号
        SettlementID: i32,
        /// 投资者帐号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 出入金同步
    #[derive(Debug, Clone, Default)]
    struct SyncDepositField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 出入金流水号
        DepositSeqNo: Vec<u8>,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 入金金额
        Deposit: f64,
        /// 是否强制进行
        IsForce: i32,
        /// 币种代码
        CurrencyID: String,
        /// 是否是个股期权内转
        IsFromSopt: i32,
        /// 资金密码
        TradingPassword: String,
        /// 是否二级代理商的内转
        IsSecAgentTranfer: i32,
    }
    /// 货币质押同步
    #[derive(Debug, Clone, Default)]
    struct SyncFundMortgageField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 货币质押流水号
        MortgageSeqNo: Vec<u8>,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 源币种
        FromCurrencyID: String,
        /// 质押金额
        MortgageAmount: f64,
        /// 目标币种
        ToCurrencyID: String,
    }
    /// 经纪公司同步
    #[derive(Debug, Clone, Default)]
    struct BrokerSyncField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 正在同步中的投资者
    #[derive(Debug, Clone, Default)]
    struct SyncingInvestorField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者代码
        InvestorID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者分组代码
        InvestorGroupID: String,
        /// 投资者名称
        InvestorName: String,
        /// 证件类型
        IdentifiedCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 是否活跃
        IsActive: i32,
        /// 联系电话
        Telephone: String,
        /// 通讯地址
        Address: String,
        /// 开户日期
        OpenDate: String,
        /// 手机
        Mobile: String,
        /// 手续费率模板代码
        CommModelID: String,
        /// 保证金率模板代码
        MarginModelID: String,
        /// 是否频率控制
        IsOrderFreq: u8,
        /// 是否开仓限制
        IsOpenVolLimit: u8,
    }
    /// 正在同步中的交易代码
    #[derive(Debug, Clone, Default)]
    struct SyncingTradingCodeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者代码
        InvestorID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 客户代码
        ClientID: String,
        /// 是否活跃
        IsActive: i32,
        /// 交易编码类型
        ClientIDType: u8,
    }
    /// 正在同步中的投资者分组
    #[derive(Debug, Clone, Default)]
    struct SyncingInvestorGroupField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者分组代码
        InvestorGroupID: String,
        /// 投资者分组名称
        InvestorGroupName: String,
    }
    /// 正在同步中的交易账号
    #[derive(Debug, Clone, Default)]
    struct SyncingTradingAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 上次质押金额
        PreMortgage: f64,
        /// 上次信用额度
        PreCredit: f64,
        /// 上次存款额
        PreDeposit: f64,
        /// 上次结算准备金
        PreBalance: f64,
        /// 上次占用的保证金
        PreMargin: f64,
        /// 利息基数
        InterestBase: f64,
        /// 利息收入
        Interest: f64,
        /// 入金金额
        Deposit: f64,
        /// 出金金额
        Withdraw: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 当前保证金总额
        CurrMargin: f64,
        /// 资金差额
        CashIn: f64,
        /// 手续费
        Commission: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 期货结算准备金
        Balance: f64,
        /// 可用资金
        Available: f64,
        /// 可取资金
        WithdrawQuota: f64,
        /// 基本准备金
        Reserve: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 信用额度
        Credit: f64,
        /// 质押金额
        Mortgage: f64,
        /// 交易所保证金
        ExchangeMargin: f64,
        /// 投资者交割保证金
        DeliveryMargin: f64,
        /// 交易所交割保证金
        ExchangeDeliveryMargin: f64,
        /// 保底期货结算准备金
        ReserveBalance: f64,
        /// 币种代码
        CurrencyID: String,
        /// 上次货币质入金额
        PreFundMortgageIn: f64,
        /// 上次货币质出金额
        PreFundMortgageOut: f64,
        /// 货币质入金额
        FundMortgageIn: f64,
        /// 货币质出金额
        FundMortgageOut: f64,
        /// 货币质押余额
        FundMortgageAvailable: f64,
        /// 可质押货币金额
        MortgageableFund: f64,
        /// 特殊产品占用保证金
        SpecProductMargin: f64,
        /// 特殊产品冻结保证金
        SpecProductFrozenMargin: f64,
        /// 特殊产品手续费
        SpecProductCommission: f64,
        /// 特殊产品冻结手续费
        SpecProductFrozenCommission: f64,
        /// 特殊产品持仓盈亏
        SpecProductPositionProfit: f64,
        /// 特殊产品平仓盈亏
        SpecProductCloseProfit: f64,
        /// 根据持仓盈亏算法计算的特殊产品持仓盈亏
        SpecProductPositionProfitByAlg: f64,
        /// 特殊产品交易所保证金
        SpecProductExchangeMargin: f64,
        /// 延时换汇冻结金额
        FrozenSwap: f64,
        /// 剩余换汇额度
        RemainSwap: f64,
        /// 期权市值
        OptionValue: f64,
    }
    /// 正在同步中的投资者持仓
    #[derive(Debug, Clone, Default)]
    struct SyncingInvestorPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 持仓多空方向
        PosiDirection: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 持仓日期
        PositionDate: u8,
        /// 上日持仓
        YdPosition: i32,
        /// 今日持仓
        Position: i32,
        /// 多头冻结
        LongFrozen: i32,
        /// 空头冻结
        ShortFrozen: i32,
        /// 开仓冻结金额
        LongFrozenAmount: f64,
        /// 开仓冻结金额
        ShortFrozenAmount: f64,
        /// 开仓量
        OpenVolume: i32,
        /// 平仓量
        CloseVolume: i32,
        /// 开仓金额
        OpenAmount: f64,
        /// 平仓金额
        CloseAmount: f64,
        /// 持仓成本
        PositionCost: f64,
        /// 上次占用的保证金
        PreMargin: f64,
        /// 占用的保证金
        UseMargin: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 资金差额
        CashIn: f64,
        /// 手续费
        Commission: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 开仓成本
        OpenCost: f64,
        /// 交易所保证金
        ExchangeMargin: f64,
        /// 组合成交形成的持仓
        CombPosition: i32,
        /// 组合多头冻结
        CombLongFrozen: i32,
        /// 组合空头冻结
        CombShortFrozen: i32,
        /// 逐日盯市平仓盈亏
        CloseProfitByDate: f64,
        /// 逐笔对冲平仓盈亏
        CloseProfitByTrade: f64,
        /// 今日持仓
        TodayPosition: i32,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 执行冻结
        StrikeFrozen: i32,
        /// 执行冻结金额
        StrikeFrozenAmount: f64,
        /// 放弃执行冻结
        AbandonFrozen: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 执行冻结的昨仓
        YdStrikeFrozen: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// 持仓成本差值
        PositionCostOffset: f64,
        /// tas持仓手数
        TasPosition: i32,
        /// tas持仓成本
        TasPositionCost: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 正在同步中的合约保证金率
    #[derive(Debug, Clone, Default)]
    struct SyncingInstrumentMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 是否相对交易所收取
        IsRelative: i32,
        /// 合约代码
        InstrumentID: String,
    }
    /// 正在同步中的合约手续费率
    #[derive(Debug, Clone, Default)]
    struct SyncingInstrumentCommissionRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 正在同步中的合约交易权限
    #[derive(Debug, Clone, Default)]
    struct SyncingInstrumentTradingRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易权限
        TradingRight: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询报单
    #[derive(Debug, Clone, Default)]
    struct QryOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 开始时间
        InsertTimeStart: String,
        /// 结束时间
        InsertTimeEnd: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询成交
    #[derive(Debug, Clone, Default)]
    struct QryTradeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 成交编号
        TradeID: String,
        /// 开始时间
        TradeTimeStart: String,
        /// 结束时间
        TradeTimeEnd: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询投资者持仓
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询资金账户
    #[derive(Debug, Clone, Default)]
    struct QryTradingAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 币种代码
        CurrencyID: String,
        /// 业务类型
        BizType: u8,
        /// 投资者帐号
        AccountID: String,
    }
    /// 查询投资者
    #[derive(Debug, Clone, Default)]
    struct QryInvestorField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 查询交易编码
    #[derive(Debug, Clone, Default)]
    struct QryTradingCodeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 客户代码
        ClientID: String,
        /// 交易编码类型
        ClientIDType: u8,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 查询投资者组
    #[derive(Debug, Clone, Default)]
    struct QryInvestorGroupField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 查询合约保证金率
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询手续费率
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentCommissionRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询合约交易权限
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentTradingRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询经纪公司
    #[derive(Debug, Clone, Default)]
    struct QryBrokerField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 查询交易员
    #[derive(Debug, Clone, Default)]
    struct QryTraderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 交易所交易员代码
        TraderID: String,
    }
    /// 查询管理用户功能权限
    #[derive(Debug, Clone, Default)]
    struct QrySuperUserFunctionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 用户代码
        UserID: String,
    }
    /// 查询用户会话
    #[derive(Debug, Clone, Default)]
    struct QryUserSessionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 查询经纪公司会员代码
    #[derive(Debug, Clone, Default)]
    struct QryPartBrokerField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 会员代码
        ParticipantID: String,
    }
    /// 查询前置状态
    #[derive(Debug, Clone, Default)]
    struct QryFrontStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置编号
        FrontID: i32,
    }
    /// 查询交易所报单
    #[derive(Debug, Clone, Default)]
    struct QryExchangeOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 查询报单操作
    #[derive(Debug, Clone, Default)]
    struct QryOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 查询交易所报单操作
    #[derive(Debug, Clone, Default)]
    struct QryExchangeOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
    }
    /// 查询管理用户
    #[derive(Debug, Clone, Default)]
    struct QrySuperUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 用户代码
        UserID: String,
    }
    /// 查询交易所
    #[derive(Debug, Clone, Default)]
    struct QryExchangeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 查询产品
    #[derive(Debug, Clone, Default)]
    struct QryProductField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品类型
        ProductClass: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
    }
    /// 查询合约
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 产品代码
        ProductID: String,
    }
    /// 查询行情
    #[derive(Debug, Clone, Default)]
    struct QryDepthMarketDataField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 产品类型
        ProductClass: u8,
    }
    /// 查询经纪公司用户
    #[derive(Debug, Clone, Default)]
    struct QryBrokerUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 查询经纪公司用户权限
    #[derive(Debug, Clone, Default)]
    struct QryBrokerUserFunctionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 查询交易员报盘机
    #[derive(Debug, Clone, Default)]
    struct QryTraderOfferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 交易所交易员代码
        TraderID: String,
    }
    /// 查询出入金流水
    #[derive(Debug, Clone, Default)]
    struct QrySyncDepositField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 出入金流水号
        DepositSeqNo: Vec<u8>,
    }
    /// 查询投资者结算结果
    #[derive(Debug, Clone, Default)]
    struct QrySettlementInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易日
        TradingDay: String,
        /// 投资者帐号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 查询交易所保证金率
    #[derive(Debug, Clone, Default)]
    struct QryExchangeMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询交易所调整保证金率
    #[derive(Debug, Clone, Default)]
    struct QryExchangeMarginRateAdjustField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询汇率
    #[derive(Debug, Clone, Default)]
    struct QryExchangeRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 源币种
        FromCurrencyID: String,
        /// 目标币种
        ToCurrencyID: String,
    }
    /// 查询货币质押流水
    #[derive(Debug, Clone, Default)]
    struct QrySyncFundMortgageField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 货币质押流水号
        MortgageSeqNo: Vec<u8>,
    }
    /// 查询报单
    #[derive(Debug, Clone, Default)]
    struct QryHisOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 开始时间
        InsertTimeStart: String,
        /// 结束时间
        InsertTimeEnd: String,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 合约代码
        InstrumentID: String,
    }
    /// 当前期权合约最小保证金
    #[derive(Debug, Clone, Default)]
    struct OptionInstrMiniMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 单位（手）期权合约最小保证金
        MinMargin: f64,
        /// 取值方式
        ValueMethod: u8,
        /// 是否跟随交易所收取
        IsRelative: i32,
        /// 合约代码
        InstrumentID: String,
    }
    /// 当前期权合约保证金调整系数
    #[derive(Debug, Clone, Default)]
    struct OptionInstrMarginAdjustField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机空头保证金调整系数
        SShortMarginRatioByMoney: f64,
        /// 投机空头保证金调整系数
        SShortMarginRatioByVolume: f64,
        /// 保值空头保证金调整系数
        HShortMarginRatioByMoney: f64,
        /// 保值空头保证金调整系数
        HShortMarginRatioByVolume: f64,
        /// 套利空头保证金调整系数
        AShortMarginRatioByMoney: f64,
        /// 套利空头保证金调整系数
        AShortMarginRatioByVolume: f64,
        /// 是否跟随交易所收取
        IsRelative: i32,
        /// 做市商空头保证金调整系数
        MShortMarginRatioByMoney: f64,
        /// 做市商空头保证金调整系数
        MShortMarginRatioByVolume: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 当前期权合约手续费的详细内容
    #[derive(Debug, Clone, Default)]
    struct OptionInstrCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 执行手续费率
        StrikeRatioByMoney: f64,
        /// 执行手续费
        StrikeRatioByVolume: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 期权交易成本
    #[derive(Debug, Clone, Default)]
    struct OptionInstrTradeCostField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 期权合约保证金不变部分
        FixedMargin: f64,
        /// 期权合约最小保证金
        MiniMargin: f64,
        /// 期权合约权利金
        Royalty: f64,
        /// 交易所期权合约保证金不变部分
        ExchFixedMargin: f64,
        /// 交易所期权合约最小保证金
        ExchMiniMargin: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 期权交易成本查询
    #[derive(Debug, Clone, Default)]
    struct QryOptionInstrTradeCostField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 期权合约报价
        InputPrice: f64,
        /// 标的价格,填0则用昨结算价
        UnderlyingPrice: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 期权手续费率查询
    #[derive(Debug, Clone, Default)]
    struct QryOptionInstrCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 股指现货指数
    #[derive(Debug, Clone, Default)]
    struct IndexPriceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 指数现货收盘价
        ClosePrice: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 输入的执行宣告
    #[derive(Debug, Clone, Default)]
    struct InputExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行宣告引用
        ExecOrderRef: String,
        /// 用户代码
        UserID: String,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 执行类型
        ActionType: u8,
        /// 保留头寸申请的持仓方向
        PosiDirection: u8,
        /// 期权行权后是否保留期货头寸的标记,该字段已废弃
        ReservePositionFlag: u8,
        /// 期权行权后生成的头寸是否自动平仓
        CloseFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 输入执行宣告操作
    #[derive(Debug, Clone, Default)]
    struct InputExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行宣告操作引用
        ExecOrderActionRef: i32,
        /// 执行宣告引用
        ExecOrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 执行宣告操作编号
        ExecOrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 用户代码
        UserID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 执行宣告
    #[derive(Debug, Clone, Default)]
    struct ExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行宣告引用
        ExecOrderRef: String,
        /// 用户代码
        UserID: String,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 执行类型
        ActionType: u8,
        /// 保留头寸申请的持仓方向
        PosiDirection: u8,
        /// 期权行权后是否保留期货头寸的标记,该字段已废弃
        ReservePositionFlag: u8,
        /// 期权行权后生成的头寸是否自动平仓
        CloseFlag: u8,
        /// 本地执行宣告编号
        ExecOrderLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 执行宣告提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 执行宣告编号
        ExecOrderSysID: String,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 执行结果
        ExecResult: u8,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 状态信息
        StatusMsg: String,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司报单编号
        BrokerExecOrderSeq: i32,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 执行宣告操作
    #[derive(Debug, Clone, Default)]
    struct ExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行宣告操作引用
        ExecOrderActionRef: i32,
        /// 执行宣告引用
        ExecOrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 执行宣告操作编号
        ExecOrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地执行宣告编号
        ExecOrderLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 执行类型
        ActionType: u8,
        /// 状态信息
        StatusMsg: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 执行宣告查询
    #[derive(Debug, Clone, Default)]
    struct QryExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 执行宣告编号
        ExecOrderSysID: String,
        /// 开始时间
        InsertTimeStart: String,
        /// 结束时间
        InsertTimeEnd: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所执行宣告信息
    #[derive(Debug, Clone, Default)]
    struct ExchangeExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 执行类型
        ActionType: u8,
        /// 保留头寸申请的持仓方向
        PosiDirection: u8,
        /// 期权行权后是否保留期货头寸的标记,该字段已废弃
        ReservePositionFlag: u8,
        /// 期权行权后生成的头寸是否自动平仓
        CloseFlag: u8,
        /// 本地执行宣告编号
        ExecOrderLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 执行宣告提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 执行宣告编号
        ExecOrderSysID: String,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 执行结果
        ExecResult: u8,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所执行宣告查询
    #[derive(Debug, Clone, Default)]
    struct QryExchangeExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 执行宣告操作查询
    #[derive(Debug, Clone, Default)]
    struct QryExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 交易所执行宣告操作
    #[derive(Debug, Clone, Default)]
    struct ExchangeExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 执行宣告操作编号
        ExecOrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地执行宣告编号
        ExecOrderLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 执行类型
        ActionType: u8,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// 数量
        Volume: i32,
        /// IP地址
        IPAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 交易所执行宣告操作查询
    #[derive(Debug, Clone, Default)]
    struct QryExchangeExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
    }
    /// 错误执行宣告
    #[derive(Debug, Clone, Default)]
    struct ErrExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行宣告引用
        ExecOrderRef: String,
        /// 用户代码
        UserID: String,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 执行类型
        ActionType: u8,
        /// 保留头寸申请的持仓方向
        PosiDirection: u8,
        /// 期权行权后是否保留期货头寸的标记,该字段已废弃
        ReservePositionFlag: u8,
        /// 期权行权后生成的头寸是否自动平仓
        CloseFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询错误执行宣告
    #[derive(Debug, Clone, Default)]
    struct QryErrExecOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 错误执行宣告操作
    #[derive(Debug, Clone, Default)]
    struct ErrExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行宣告操作引用
        ExecOrderActionRef: i32,
        /// 执行宣告引用
        ExecOrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 执行宣告操作编号
        ExecOrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 用户代码
        UserID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询错误执行宣告操作
    #[derive(Debug, Clone, Default)]
    struct QryErrExecOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 投资者期权合约交易权限
    #[derive(Debug, Clone, Default)]
    struct OptionInstrTradingRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 买卖方向
        Direction: u8,
        /// 交易权限
        TradingRight: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询期权合约交易权限
    #[derive(Debug, Clone, Default)]
    struct QryOptionInstrTradingRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 买卖方向
        Direction: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 输入的询价
    #[derive(Debug, Clone, Default)]
    struct InputForQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 询价引用
        ForQuoteRef: String,
        /// 用户代码
        UserID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 询价
    #[derive(Debug, Clone, Default)]
    struct ForQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 询价引用
        ForQuoteRef: String,
        /// 用户代码
        UserID: String,
        /// 本地询价编号
        ForQuoteLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 询价状态
        ForQuoteStatus: u8,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 状态信息
        StatusMsg: String,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司询价编号
        BrokerForQutoSeq: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 询价查询
    #[derive(Debug, Clone, Default)]
    struct QryForQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 开始时间
        InsertTimeStart: String,
        /// 结束时间
        InsertTimeEnd: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所询价信息
    #[derive(Debug, Clone, Default)]
    struct ExchangeForQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 本地询价编号
        ForQuoteLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 询价状态
        ForQuoteStatus: u8,
        /// Mac地址
        MacAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所询价查询
    #[derive(Debug, Clone, Default)]
    struct QryExchangeForQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 输入的报价
    #[derive(Debug, Clone, Default)]
    struct InputQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报价引用
        QuoteRef: String,
        /// 用户代码
        UserID: String,
        /// 卖价格
        AskPrice: f64,
        /// 买价格
        BidPrice: f64,
        /// 卖数量
        AskVolume: i32,
        /// 买数量
        BidVolume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 卖开平标志
        AskOffsetFlag: u8,
        /// 买开平标志
        BidOffsetFlag: u8,
        /// 卖投机套保标志
        AskHedgeFlag: u8,
        /// 买投机套保标志
        BidHedgeFlag: u8,
        /// 衍生卖报单引用
        AskOrderRef: String,
        /// 衍生买报单引用
        BidOrderRef: String,
        /// 应价编号
        ForQuoteSysID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 被顶单编号
        ReplaceSysID: String,
        /// 有效期类型
        TimeCondition: u8,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 输入报价操作
    #[derive(Debug, Clone, Default)]
    struct InputQuoteActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报价操作引用
        QuoteActionRef: i32,
        /// 报价引用
        QuoteRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 报价操作编号
        QuoteSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 用户代码
        UserID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 报价
    #[derive(Debug, Clone, Default)]
    struct QuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报价引用
        QuoteRef: String,
        /// 用户代码
        UserID: String,
        /// 卖价格
        AskPrice: f64,
        /// 买价格
        BidPrice: f64,
        /// 卖数量
        AskVolume: i32,
        /// 买数量
        BidVolume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 卖开平标志
        AskOffsetFlag: u8,
        /// 买开平标志
        BidOffsetFlag: u8,
        /// 卖投机套保标志
        AskHedgeFlag: u8,
        /// 买投机套保标志
        BidHedgeFlag: u8,
        /// 本地报价编号
        QuoteLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报价提示序号
        NotifySequence: i32,
        /// 报价提交状态
        OrderSubmitStatus: u8,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 报价编号
        QuoteSysID: String,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 报价状态
        QuoteStatus: u8,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 卖方报单编号
        AskOrderSysID: String,
        /// 买方报单编号
        BidOrderSysID: String,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 状态信息
        StatusMsg: String,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司报价编号
        BrokerQuoteSeq: i32,
        /// 衍生卖报单引用
        AskOrderRef: String,
        /// 衍生买报单引用
        BidOrderRef: String,
        /// 应价编号
        ForQuoteSysID: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
        /// 被顶单编号
        ReplaceSysID: String,
        /// 有效期类型
        TimeCondition: u8,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 报价操作
    #[derive(Debug, Clone, Default)]
    struct QuoteActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报价操作引用
        QuoteActionRef: i32,
        /// 报价引用
        QuoteRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 报价操作编号
        QuoteSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报价编号
        QuoteLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 状态信息
        StatusMsg: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 报价查询
    #[derive(Debug, Clone, Default)]
    struct QryQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 报价编号
        QuoteSysID: String,
        /// 开始时间
        InsertTimeStart: String,
        /// 结束时间
        InsertTimeEnd: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所报价信息
    #[derive(Debug, Clone, Default)]
    struct ExchangeQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 卖价格
        AskPrice: f64,
        /// 买价格
        BidPrice: f64,
        /// 卖数量
        AskVolume: i32,
        /// 买数量
        BidVolume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 卖开平标志
        AskOffsetFlag: u8,
        /// 买开平标志
        BidOffsetFlag: u8,
        /// 卖投机套保标志
        AskHedgeFlag: u8,
        /// 买投机套保标志
        BidHedgeFlag: u8,
        /// 本地报价编号
        QuoteLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报价提示序号
        NotifySequence: i32,
        /// 报价提交状态
        OrderSubmitStatus: u8,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 报价编号
        QuoteSysID: String,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 报价状态
        QuoteStatus: u8,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 卖方报单编号
        AskOrderSysID: String,
        /// 买方报单编号
        BidOrderSysID: String,
        /// 应价编号
        ForQuoteSysID: String,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
        /// 有效期类型
        TimeCondition: u8,
    }
    /// 交易所报价查询
    #[derive(Debug, Clone, Default)]
    struct QryExchangeQuoteField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 报价操作查询
    #[derive(Debug, Clone, Default)]
    struct QryQuoteActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 交易所报价操作
    #[derive(Debug, Clone, Default)]
    struct ExchangeQuoteActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 报价操作编号
        QuoteSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报价编号
        QuoteLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// Mac地址
        MacAddress: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所报价操作查询
    #[derive(Debug, Clone, Default)]
    struct QryExchangeQuoteActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
    }
    /// 期权合约delta值
    #[derive(Debug, Clone, Default)]
    struct OptionInstrDeltaField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// Delta值
        Delta: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 发给做市商的询价请求
    #[derive(Debug, Clone, Default)]
    struct ForQuoteRspField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 询价编号
        ForQuoteSysID: String,
        /// 询价时间
        ForQuoteTime: String,
        /// 业务日期
        ActionDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 当前期权合约执行偏移值的详细内容
    #[derive(Debug, Clone, Default)]
    struct StrikeOffsetField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 执行偏移值
        Offset: f64,
        /// 执行偏移类型
        OffsetType: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 期权执行偏移值查询
    #[derive(Debug, Clone, Default)]
    struct QryStrikeOffsetField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 输入批量报单操作
    #[derive(Debug, Clone, Default)]
    struct InputBatchOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单操作引用
        OrderActionRef: i32,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 用户代码
        UserID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// IP地址
        IPAddress: String,
    }
    /// 批量报单操作
    #[derive(Debug, Clone, Default)]
    struct BatchOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单操作引用
        OrderActionRef: i32,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 状态信息
        StatusMsg: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所批量报单操作
    #[derive(Debug, Clone, Default)]
    struct ExchangeBatchOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// Mac地址
        MacAddress: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询批量报单操作
    #[derive(Debug, Clone, Default)]
    struct QryBatchOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 组合合约安全系数
    #[derive(Debug, Clone, Default)]
    struct CombInstrumentGuardField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        GuarantRatio: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 组合合约安全系数查询
    #[derive(Debug, Clone, Default)]
    struct QryCombInstrumentGuardField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 输入的申请组合
    #[derive(Debug, Clone, Default)]
    struct InputCombActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组合引用
        CombActionRef: String,
        /// 用户代码
        UserID: String,
        /// 买卖方向
        Direction: u8,
        /// 数量
        Volume: i32,
        /// 组合指令方向
        CombDirection: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// Mac地址
        MacAddress: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 申请组合
    #[derive(Debug, Clone, Default)]
    struct CombActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组合引用
        CombActionRef: String,
        /// 用户代码
        UserID: String,
        /// 买卖方向
        Direction: u8,
        /// 数量
        Volume: i32,
        /// 组合指令方向
        CombDirection: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 本地申请组合编号
        ActionLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 组合状态
        ActionStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 序号
        SequenceNo: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 状态信息
        StatusMsg: String,
        /// Mac地址
        MacAddress: String,
        /// 组合编号
        ComTradeID: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 申请组合查询
    #[derive(Debug, Clone, Default)]
    struct QryCombActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所申请组合信息
    #[derive(Debug, Clone, Default)]
    struct ExchangeCombActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 买卖方向
        Direction: u8,
        /// 数量
        Volume: i32,
        /// 组合指令方向
        CombDirection: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 本地申请组合编号
        ActionLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 组合状态
        ActionStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 序号
        SequenceNo: i32,
        /// Mac地址
        MacAddress: String,
        /// 组合编号
        ComTradeID: String,
        /// 营业部编号
        BranchID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 交易所申请组合查询
    #[derive(Debug, Clone, Default)]
    struct QryExchangeCombActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 产品报价汇率
    #[derive(Debug, Clone, Default)]
    struct ProductExchRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 报价币种类型
        QuoteCurrencyID: String,
        /// 汇率
        ExchangeRate: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
    }
    /// 产品报价汇率查询
    #[derive(Debug, Clone, Default)]
    struct QryProductExchRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
    }
    /// 查询询价价差参数
    #[derive(Debug, Clone, Default)]
    struct QryForQuoteParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 询价价差参数
    #[derive(Debug, Clone, Default)]
    struct ForQuoteParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 最新价
        LastPrice: f64,
        /// 价差
        PriceInterval: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 当前做市商期权合约手续费的详细内容
    #[derive(Debug, Clone, Default)]
    struct MMOptionInstrCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 执行手续费率
        StrikeRatioByMoney: f64,
        /// 执行手续费
        StrikeRatioByVolume: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 做市商期权手续费率查询
    #[derive(Debug, Clone, Default)]
    struct QryMMOptionInstrCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 做市商合约手续费率
    #[derive(Debug, Clone, Default)]
    struct MMInstrumentCommissionRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询做市商合约手续费率
    #[derive(Debug, Clone, Default)]
    struct QryMMInstrumentCommissionRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 当前报单手续费的详细内容
    #[derive(Debug, Clone, Default)]
    struct InstrumentOrderCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 报单手续费
        OrderCommByVolume: f64,
        /// 撤单手续费
        OrderActionCommByVolume: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
        /// 报单手续费
        OrderCommByTrade: f64,
        /// 撤单手续费
        OrderActionCommByTrade: f64,
    }
    /// 报单手续费率查询
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentOrderCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易参数
    #[derive(Debug, Clone, Default)]
    struct TradeParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 参数代码
        TradeParamID: u8,
        /// 参数代码值
        TradeParamValue: Vec<u8>,
        /// 备注
        Memo: String,
    }
    /// 合约保证金率调整
    #[derive(Debug, Clone, Default)]
    struct InstrumentMarginRateULField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// 期货持仓限制参数
    #[derive(Debug, Clone, Default)]
    struct FutureLimitPosiParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 当日投机开仓数量限制
        SpecOpenVolume: i32,
        /// 当日套利开仓数量限制
        ArbiOpenVolume: i32,
        /// 当日投机+套利开仓数量限制
        OpenVolume: i32,
        /// 产品代码
        ProductID: String,
    }
    /// 禁止登录IP
    #[derive(Debug, Clone, Default)]
    struct LoginForbiddenIPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// IP地址
        IPAddress: String,
    }
    /// IP列表
    #[derive(Debug, Clone, Default)]
    struct IPListField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 是否白名单
        IsWhite: i32,
        /// IP地址
        IPAddress: String,
    }
    /// 输入的期权自对冲
    #[derive(Debug, Clone, Default)]
    struct InputOptionSelfCloseField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 期权自对冲引用
        OptionSelfCloseRef: String,
        /// 用户代码
        UserID: String,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 期权行权的头寸是否自对冲
        OptSelfCloseFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 输入期权自对冲操作
    #[derive(Debug, Clone, Default)]
    struct InputOptionSelfCloseActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 期权自对冲操作引用
        OptionSelfCloseActionRef: i32,
        /// 期权自对冲引用
        OptionSelfCloseRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 期权自对冲操作编号
        OptionSelfCloseSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 用户代码
        UserID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 期权自对冲
    #[derive(Debug, Clone, Default)]
    struct OptionSelfCloseField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 期权自对冲引用
        OptionSelfCloseRef: String,
        /// 用户代码
        UserID: String,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 期权行权的头寸是否自对冲
        OptSelfCloseFlag: u8,
        /// 本地期权自对冲编号
        OptionSelfCloseLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 期权自对冲提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 期权自对冲编号
        OptionSelfCloseSysID: String,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 自对冲结果
        ExecResult: u8,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 状态信息
        StatusMsg: String,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司报单编号
        BrokerOptionSelfCloseSeq: i32,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 期权自对冲操作
    #[derive(Debug, Clone, Default)]
    struct OptionSelfCloseActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 期权自对冲操作引用
        OptionSelfCloseActionRef: i32,
        /// 期权自对冲引用
        OptionSelfCloseRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 期权自对冲操作编号
        OptionSelfCloseSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地期权自对冲编号
        OptionSelfCloseLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 状态信息
        StatusMsg: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 期权自对冲查询
    #[derive(Debug, Clone, Default)]
    struct QryOptionSelfCloseField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 期权自对冲编号
        OptionSelfCloseSysID: String,
        /// 开始时间
        InsertTimeStart: String,
        /// 结束时间
        InsertTimeEnd: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 交易所期权自对冲信息
    #[derive(Debug, Clone, Default)]
    struct ExchangeOptionSelfCloseField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 数量
        Volume: i32,
        /// 请求编号
        RequestID: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 期权行权的头寸是否自对冲
        OptSelfCloseFlag: u8,
        /// 本地期权自对冲编号
        OptionSelfCloseLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 期权自对冲提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 期权自对冲编号
        OptionSelfCloseSysID: String,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 自对冲结果
        ExecResult: u8,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 期权自对冲操作查询
    #[derive(Debug, Clone, Default)]
    struct QryOptionSelfCloseActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 交易所期权自对冲操作
    #[derive(Debug, Clone, Default)]
    struct ExchangeOptionSelfCloseActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 期权自对冲操作编号
        OptionSelfCloseSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地期权自对冲编号
        OptionSelfCloseLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 营业部编号
        BranchID: String,
        /// Mac地址
        MacAddress: String,
        /// 期权行权的头寸是否自对冲
        OptSelfCloseFlag: u8,
        /// IP地址
        IPAddress: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 延时换汇同步
    #[derive(Debug, Clone, Default)]
    struct SyncDelaySwapField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 换汇流水号
        DelaySwapSeqNo: Vec<u8>,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 源币种
        FromCurrencyID: String,
        /// 源金额
        FromAmount: f64,
        /// 源换汇冻结金额(可用冻结)
        FromFrozenSwap: f64,
        /// 源剩余换汇额度(可提冻结)
        FromRemainSwap: f64,
        /// 目标币种
        ToCurrencyID: String,
        /// 目标金额
        ToAmount: f64,
        /// 是否手工换汇
        IsManualSwap: i32,
        /// 是否将所有外币的剩余换汇额度设置为0
        IsAllRemainSetZero: i32,
    }
    /// 查询延时换汇同步
    #[derive(Debug, Clone, Default)]
    struct QrySyncDelaySwapField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 延时换汇流水号
        DelaySwapSeqNo: Vec<u8>,
    }
    /// 投资单元
    #[derive(Debug, Clone, Default)]
    struct InvestUnitField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 投资者单元名称
        InvestorUnitName: String,
        /// 投资者分组代码
        InvestorGroupID: String,
        /// 手续费率模板代码
        CommModelID: String,
        /// 保证金率模板代码
        MarginModelID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 查询投资单元
    #[derive(Debug, Clone, Default)]
    struct QryInvestUnitField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 二级代理商资金校验模式
    #[derive(Debug, Clone, Default)]
    struct SecAgentCheckModeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者代码
        InvestorID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 币种
        CurrencyID: String,
        /// 境外中介机构资金帐号
        BrokerSecAgentID: String,
        /// 是否需要校验自己的资金账户
        CheckSelfAccount: i32,
    }
    /// 二级代理商信息
    #[derive(Debug, Clone, Default)]
    struct SecAgentTradeInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 境外中介机构资金帐号
        BrokerSecAgentID: String,
        /// 投资者代码
        InvestorID: String,
        /// 二级代理商姓名
        LongCustomerName: String,
    }
    /// 市场行情
    #[derive(Debug, Clone, Default)]
    struct MarketDataField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 最新价
        LastPrice: f64,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 昨收盘
        PreClosePrice: f64,
        /// 昨持仓量
        PreOpenInterest: f64,
        /// 今开盘
        OpenPrice: f64,
        /// 最高价
        HighestPrice: f64,
        /// 最低价
        LowestPrice: f64,
        /// 数量
        Volume: i32,
        /// 成交金额
        Turnover: f64,
        /// 持仓量
        OpenInterest: f64,
        /// 今收盘
        ClosePrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 涨停板价
        UpperLimitPrice: f64,
        /// 跌停板价
        LowerLimitPrice: f64,
        /// 昨虚实度
        PreDelta: f64,
        /// 今虚实度
        CurrDelta: f64,
        /// 最后修改时间
        UpdateTime: String,
        /// 最后修改毫秒
        UpdateMillisec: i32,
        /// 业务日期
        ActionDay: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 行情基础属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataBaseField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 昨收盘
        PreClosePrice: f64,
        /// 昨持仓量
        PreOpenInterest: f64,
        /// 昨虚实度
        PreDelta: f64,
    }
    /// 行情静态属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataStaticField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 今开盘
        OpenPrice: f64,
        /// 最高价
        HighestPrice: f64,
        /// 最低价
        LowestPrice: f64,
        /// 今收盘
        ClosePrice: f64,
        /// 涨停板价
        UpperLimitPrice: f64,
        /// 跌停板价
        LowerLimitPrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 今虚实度
        CurrDelta: f64,
    }
    /// 行情最新成交属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataLastMatchField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 最新价
        LastPrice: f64,
        /// 数量
        Volume: i32,
        /// 成交金额
        Turnover: f64,
        /// 持仓量
        OpenInterest: f64,
    }
    /// 行情最优价属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataBestPriceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 申买价一
        BidPrice1: f64,
        /// 申买量一
        BidVolume1: i32,
        /// 申卖价一
        AskPrice1: f64,
        /// 申卖量一
        AskVolume1: i32,
    }
    /// 行情申买二、三属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataBid23Field {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 申买价二
        BidPrice2: f64,
        /// 申买量二
        BidVolume2: i32,
        /// 申买价三
        BidPrice3: f64,
        /// 申买量三
        BidVolume3: i32,
    }
    /// 行情申卖二、三属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataAsk23Field {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 申卖价二
        AskPrice2: f64,
        /// 申卖量二
        AskVolume2: i32,
        /// 申卖价三
        AskPrice3: f64,
        /// 申卖量三
        AskVolume3: i32,
    }
    /// 行情申买四、五属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataBid45Field {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 申买价四
        BidPrice4: f64,
        /// 申买量四
        BidVolume4: i32,
        /// 申买价五
        BidPrice5: f64,
        /// 申买量五
        BidVolume5: i32,
    }
    /// 行情申卖四、五属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataAsk45Field {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 申卖价四
        AskPrice4: f64,
        /// 申卖量四
        AskVolume4: i32,
        /// 申卖价五
        AskPrice5: f64,
        /// 申卖量五
        AskVolume5: i32,
    }
    /// 行情更新时间属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataUpdateTimeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 最后修改时间
        UpdateTime: String,
        /// 最后修改毫秒
        UpdateMillisec: i32,
        /// 业务日期
        ActionDay: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 行情上下带价
    #[derive(Debug, Clone, Default)]
    struct MarketDataBandingPriceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 上带价
        BandingUpperPrice: f64,
        /// 下带价
        BandingLowerPrice: f64,
    }
    /// 行情交易所代码属性
    #[derive(Debug, Clone, Default)]
    struct MarketDataExchangeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 指定的合约
    #[derive(Debug, Clone, Default)]
    struct SpecificInstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
    }
    /// 合约状态
    #[derive(Debug, Clone, Default)]
    struct InstrumentStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 结算组代码
        SettlementGroupID: String,
        /// 合约交易状态
        InstrumentStatus: u8,
        /// 交易阶段编号
        TradingSegmentSN: i32,
        /// 进入本状态时间
        EnterTime: String,
        /// 进入本状态原因
        EnterReason: u8,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询合约状态
    #[derive(Debug, Clone, Default)]
    struct QryInstrumentStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
    }
    /// 投资者账户
    #[derive(Debug, Clone, Default)]
    struct InvestorAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投资者帐号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 浮动盈亏算法
    #[derive(Debug, Clone, Default)]
    struct PositionProfitAlgorithmField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 盈亏算法
        Algorithm: u8,
        /// 备注
        Memo: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 会员资金折扣
    #[derive(Debug, Clone, Default)]
    struct DiscountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 投资者代码
        InvestorID: String,
        /// 资金折扣比例
        Discount: f64,
    }
    /// 查询转帐银行
    #[derive(Debug, Clone, Default)]
    struct QryTransferBankField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 银行代码
        BankID: String,
        /// 银行分中心代码
        BankBrchID: String,
    }
    /// 转帐银行
    #[derive(Debug, Clone, Default)]
    struct TransferBankField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 银行代码
        BankID: String,
        /// 银行分中心代码
        BankBrchID: String,
        /// 银行名称
        BankName: String,
        /// 是否活跃
        IsActive: i32,
    }
    /// 查询投资者持仓明细
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPositionDetailField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 投资者持仓明细
    #[derive(Debug, Clone, Default)]
    struct InvestorPositionDetailField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 买卖
        Direction: u8,
        /// 开仓日期
        OpenDate: String,
        /// 成交编号
        TradeID: String,
        /// 数量
        Volume: i32,
        /// 开仓价
        OpenPrice: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 成交类型
        TradeType: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 逐日盯市平仓盈亏
        CloseProfitByDate: f64,
        /// 逐笔对冲平仓盈亏
        CloseProfitByTrade: f64,
        /// 逐日盯市持仓盈亏
        PositionProfitByDate: f64,
        /// 逐笔对冲持仓盈亏
        PositionProfitByTrade: f64,
        /// 投资者保证金
        Margin: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 昨结算价
        LastSettlementPrice: f64,
        /// 结算价
        SettlementPrice: f64,
        /// 平仓量
        CloseVolume: i32,
        /// 平仓金额
        CloseAmount: f64,
        /// 先开先平剩余数量
        TimeFirstVolume: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// 特殊持仓标志
        SpecPosiType: u8,
        /// 合约代码
        InstrumentID: String,
        /// 组合合约代码
        CombInstrumentID: String,
    }
    /// 资金账户口令域
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 密码
        Password: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 交易所行情报盘机
    #[derive(Debug, Clone, Default)]
    struct MDTraderOfferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 会员代码
        ParticipantID: String,
        /// 密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 交易所交易员连接状态
        TraderConnectStatus: u8,
        /// 发出连接请求的日期
        ConnectRequestDate: String,
        /// 发出连接请求的时间
        ConnectRequestTime: String,
        /// 上次报告日期
        LastReportDate: String,
        /// 上次报告时间
        LastReportTime: String,
        /// 完成连接日期
        ConnectDate: String,
        /// 完成连接时间
        ConnectTime: String,
        /// 启动日期
        StartDate: String,
        /// 启动时间
        StartTime: String,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 本席位最大成交编号
        MaxTradeID: String,
        /// 本席位最大报单备拷
        MaxOrderMessageReference: Vec<u8>,
        /// 撤单时选择席位算法
        OrderCancelAlg: u8,
    }
    /// 查询行情报盘机
    #[derive(Debug, Clone, Default)]
    struct QryMDTraderOfferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 交易所交易员代码
        TraderID: String,
    }
    /// 查询客户通知
    #[derive(Debug, Clone, Default)]
    struct QryNoticeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 客户通知
    #[derive(Debug, Clone, Default)]
    struct NoticeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 消息正文
        Content: Vec<u8>,
        /// 经纪公司通知内容序列号
        SequenceLabel: Vec<u8>,
    }
    /// 用户权限
    #[derive(Debug, Clone, Default)]
    struct UserRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 客户权限类型
        UserRightType: u8,
        /// 是否禁止
        IsForbidden: i32,
    }
    /// 查询结算信息确认域
    #[derive(Debug, Clone, Default)]
    struct QrySettlementInfoConfirmField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投资者帐号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 装载结算信息
    #[derive(Debug, Clone, Default)]
    struct LoadSettlementInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 经纪公司可提资金算法表
    #[derive(Debug, Clone, Default)]
    struct BrokerWithdrawAlgorithmField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 可提资金算法
        WithdrawAlgorithm: u8,
        /// 资金使用率
        UsingRatio: f64,
        /// 可提是否包含平仓盈利
        IncludeCloseProfit: u8,
        /// 本日无仓且无成交客户是否受可提比例限制
        AllWithoutTrade: u8,
        /// 可用是否包含平仓盈利
        AvailIncludeCloseProfit: u8,
        /// 是否启用用户事件
        IsBrokerUserEvent: i32,
        /// 币种代码
        CurrencyID: String,
        /// 货币质押比率
        FundMortgageRatio: f64,
        /// 权益算法
        BalanceAlgorithm: u8,
    }
    /// 资金账户口令变更域
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordUpdateV1Field {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 原来的口令
        OldPassword: String,
        /// 新的口令
        NewPassword: String,
    }
    /// 资金账户口令变更域
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordUpdateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 原来的口令
        OldPassword: String,
        /// 新的口令
        NewPassword: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 查询组合合约分腿
    #[derive(Debug, Clone, Default)]
    struct QryCombinationLegField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 单腿编号
        LegID: i32,
        /// 组合合约代码
        CombInstrumentID: String,
        /// 单腿合约代码
        LegInstrumentID: String,
    }
    /// 查询组合合约分腿
    #[derive(Debug, Clone, Default)]
    struct QrySyncStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
    }
    /// 组合交易合约的单腿
    #[derive(Debug, Clone, Default)]
    struct CombinationLegField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 单腿编号
        LegID: i32,
        /// 买卖方向
        Direction: u8,
        /// 单腿乘数
        LegMultiple: i32,
        /// 派生层数
        ImplyLevel: i32,
        /// 组合合约代码
        CombInstrumentID: String,
        /// 单腿合约代码
        LegInstrumentID: String,
    }
    /// 数据同步状态
    #[derive(Debug, Clone, Default)]
    struct SyncStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 数据同步状态
        DataSyncStatus: u8,
    }
    /// 查询联系人
    #[derive(Debug, Clone, Default)]
    struct QryLinkManField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 联系人
    #[derive(Debug, Clone, Default)]
    struct LinkManField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 联系人类型
        PersonType: u8,
        /// 证件类型
        IdentifiedCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 名称
        PersonName: String,
        /// 联系电话
        Telephone: String,
        /// 通讯地址
        Address: String,
        /// 邮政编码
        ZipCode: String,
        /// 优先级
        Priority: i32,
        /// 开户邮政编码
        UOAZipCode: String,
        /// 全称
        PersonFullName: String,
    }
    /// 查询经纪公司用户事件
    #[derive(Debug, Clone, Default)]
    struct QryBrokerUserEventField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户事件类型
        UserEventType: u8,
    }
    /// 查询经纪公司用户事件
    #[derive(Debug, Clone, Default)]
    struct BrokerUserEventField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户事件类型
        UserEventType: u8,
        /// 用户事件序号
        EventSequenceNo: i32,
        /// 事件发生日期
        EventDate: String,
        /// 事件发生时间
        EventTime: String,
        /// 用户事件信息
        UserEventInfo: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 交易日
        TradingDay: String,
    }
    /// 查询签约银行请求
    #[derive(Debug, Clone, Default)]
    struct QryContractBankField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 银行代码
        BankID: String,
        /// 银行分中心代码
        BankBrchID: String,
    }
    /// 查询签约银行响应
    #[derive(Debug, Clone, Default)]
    struct ContractBankField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 银行代码
        BankID: String,
        /// 银行分中心代码
        BankBrchID: String,
        /// 银行名称
        BankName: String,
    }
    /// 投资者组合持仓明细
    #[derive(Debug, Clone, Default)]
    struct InvestorPositionCombineDetailField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 开仓日期
        OpenDate: String,
        /// 交易所代码
        ExchangeID: String,
        /// 结算编号
        SettlementID: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组合编号
        ComTradeID: String,
        /// 撮合编号
        TradeID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 买卖
        Direction: u8,
        /// 持仓量
        TotalAmt: i32,
        /// 投资者保证金
        Margin: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 单腿编号
        LegID: i32,
        /// 单腿乘数
        LegMultiple: i32,
        /// 成交组号
        TradeGroupID: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
        /// 组合持仓合约编码
        CombInstrumentID: String,
    }
    /// 预埋单
    #[derive(Debug, Clone, Default)]
    struct ParkedOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单引用
        OrderRef: String,
        /// 用户代码
        UserID: String,
        /// 报单价格条件
        OrderPriceType: u8,
        /// 买卖方向
        Direction: u8,
        /// 组合开平标志
        CombOffsetFlag: String,
        /// 组合投机套保标志
        CombHedgeFlag: String,
        /// 价格
        LimitPrice: f64,
        /// 数量
        VolumeTotalOriginal: i32,
        /// 有效期类型
        TimeCondition: u8,
        /// GTD日期
        GTDDate: String,
        /// 成交量类型
        VolumeCondition: u8,
        /// 最小成交量
        MinVolume: i32,
        /// 触发条件
        ContingentCondition: u8,
        /// 止损价
        StopPrice: f64,
        /// 强平原因
        ForceCloseReason: u8,
        /// 自动挂起标志
        IsAutoSuspend: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 请求编号
        RequestID: i32,
        /// 用户强平标志
        UserForceClose: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 预埋报单编号
        ParkedOrderID: String,
        /// 用户类型
        UserType: u8,
        /// 预埋单状态
        Status: u8,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 互换单标志
        IsSwapOrder: i32,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易编码
        ClientID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 输入预埋单操作
    #[derive(Debug, Clone, Default)]
    struct ParkedOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单操作引用
        OrderActionRef: i32,
        /// 报单引用
        OrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 价格
        LimitPrice: f64,
        /// 数量变化
        VolumeChange: i32,
        /// 用户代码
        UserID: String,
        /// 预埋撤单单编号
        ParkedOrderActionID: String,
        /// 用户类型
        UserType: u8,
        /// 预埋撤单状态
        Status: u8,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询预埋单
    #[derive(Debug, Clone, Default)]
    struct QryParkedOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询预埋撤单
    #[derive(Debug, Clone, Default)]
    struct QryParkedOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 删除预埋单
    #[derive(Debug, Clone, Default)]
    struct RemoveParkedOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 预埋报单编号
        ParkedOrderID: String,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 删除预埋撤单
    #[derive(Debug, Clone, Default)]
    struct RemoveParkedOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 预埋撤单编号
        ParkedOrderActionID: String,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 经纪公司可提资金算法表
    #[derive(Debug, Clone, Default)]
    struct InvestorWithdrawAlgorithmField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 投资者代码
        InvestorID: String,
        /// 可提资金比例
        UsingRatio: f64,
        /// 币种代码
        CurrencyID: String,
        /// 货币质押比率
        FundMortgageRatio: f64,
    }
    /// 查询组合持仓明细
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPositionCombineDetailField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 组合持仓合约编码
        CombInstrumentID: String,
    }
    /// 成交均价
    #[derive(Debug, Clone, Default)]
    struct MarketDataAveragePriceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 当日均价
        AveragePrice: f64,
    }
    /// 校验投资者密码
    #[derive(Debug, Clone, Default)]
    struct VerifyInvestorPasswordField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 密码
        Password: String,
    }
    /// 用户IP
    #[derive(Debug, Clone, Default)]
    struct UserIPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// Mac地址
        MacAddress: String,
        /// IP地址
        IPAddress: String,
        /// IP地址掩码
        IPMask: Vec<u8>,
    }
    /// 用户事件通知信息
    #[derive(Debug, Clone, Default)]
    struct TradingNoticeInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 发送时间
        SendTime: String,
        /// 消息正文
        FieldContent: Vec<u8>,
        /// 序列系列号
        SequenceSeries: u16,
        /// 序列号
        SequenceNo: i32,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 用户事件通知
    #[derive(Debug, Clone, Default)]
    struct TradingNoticeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 投资者代码
        InvestorID: String,
        /// 序列系列号
        SequenceSeries: u16,
        /// 用户代码
        UserID: String,
        /// 发送时间
        SendTime: String,
        /// 序列号
        SequenceNo: i32,
        /// 消息正文
        FieldContent: Vec<u8>,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 查询交易事件通知
    #[derive(Debug, Clone, Default)]
    struct QryTradingNoticeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 查询错误报单
    #[derive(Debug, Clone, Default)]
    struct QryErrOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 错误报单
    #[derive(Debug, Clone, Default)]
    struct ErrOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单引用
        OrderRef: String,
        /// 用户代码
        UserID: String,
        /// 报单价格条件
        OrderPriceType: u8,
        /// 买卖方向
        Direction: u8,
        /// 组合开平标志
        CombOffsetFlag: String,
        /// 组合投机套保标志
        CombHedgeFlag: String,
        /// 价格
        LimitPrice: f64,
        /// 数量
        VolumeTotalOriginal: i32,
        /// 有效期类型
        TimeCondition: u8,
        /// GTD日期
        GTDDate: String,
        /// 成交量类型
        VolumeCondition: u8,
        /// 最小成交量
        MinVolume: i32,
        /// 触发条件
        ContingentCondition: u8,
        /// 止损价
        StopPrice: f64,
        /// 强平原因
        ForceCloseReason: u8,
        /// 自动挂起标志
        IsAutoSuspend: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 请求编号
        RequestID: i32,
        /// 用户强平标志
        UserForceClose: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 互换单标志
        IsSwapOrder: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易编码
        ClientID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 查询错误报单操作
    #[derive(Debug, Clone, Default)]
    struct ErrorConditionalOrderField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单引用
        OrderRef: String,
        /// 用户代码
        UserID: String,
        /// 报单价格条件
        OrderPriceType: u8,
        /// 买卖方向
        Direction: u8,
        /// 组合开平标志
        CombOffsetFlag: String,
        /// 组合投机套保标志
        CombHedgeFlag: String,
        /// 价格
        LimitPrice: f64,
        /// 数量
        VolumeTotalOriginal: i32,
        /// 有效期类型
        TimeCondition: u8,
        /// GTD日期
        GTDDate: String,
        /// 成交量类型
        VolumeCondition: u8,
        /// 最小成交量
        MinVolume: i32,
        /// 触发条件
        ContingentCondition: u8,
        /// 止损价
        StopPrice: f64,
        /// 强平原因
        ForceCloseReason: u8,
        /// 自动挂起标志
        IsAutoSuspend: i32,
        /// 业务单元
        BusinessUnit: String,
        /// 请求编号
        RequestID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 报单提交状态
        OrderSubmitStatus: u8,
        /// 报单提示序号
        NotifySequence: i32,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 报单编号
        OrderSysID: String,
        /// 报单来源
        OrderSource: u8,
        /// 报单状态
        OrderStatus: u8,
        /// 报单类型
        OrderType: u8,
        /// 今成交数量
        VolumeTraded: i32,
        /// 剩余数量
        VolumeTotal: i32,
        /// 报单日期
        InsertDate: String,
        /// 委托时间
        InsertTime: String,
        /// 激活时间
        ActiveTime: String,
        /// 挂起时间
        SuspendTime: String,
        /// 最后修改时间
        UpdateTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 最后修改交易所交易员代码
        ActiveTraderID: String,
        /// 结算会员编号
        ClearingPartID: String,
        /// 序号
        SequenceNo: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 状态信息
        StatusMsg: String,
        /// 用户强平标志
        UserForceClose: i32,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司报单编号
        BrokerOrderSeq: i32,
        /// 相关报单
        RelativeOrderSysID: String,
        /// 郑商所成交数量
        ZCETotalTradedVolume: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 互换单标志
        IsSwapOrder: i32,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 资金账号
        AccountID: String,
        /// 币种代码
        CurrencyID: String,
        /// Mac地址
        MacAddress: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询错误报单操作
    #[derive(Debug, Clone, Default)]
    struct QryErrOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 错误报单操作
    #[derive(Debug, Clone, Default)]
    struct ErrOrderActionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 报单操作引用
        OrderActionRef: i32,
        /// 报单引用
        OrderRef: String,
        /// 请求编号
        RequestID: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 报单编号
        OrderSysID: String,
        /// 操作标志
        ActionFlag: u8,
        /// 价格
        LimitPrice: f64,
        /// 数量变化
        VolumeChange: i32,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 本地报单编号
        OrderLocalID: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 业务单元
        BusinessUnit: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 用户代码
        UserID: String,
        /// 状态信息
        StatusMsg: String,
        /// 营业部编号
        BranchID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// Mac地址
        MacAddress: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 合约代码
        InstrumentID: String,
        /// IP地址
        IPAddress: String,
        /// 报单回显字段
        OrderMemo: String,
        /// session上请求计数 api自动维护
        SessionReqSeq: i32,
    }
    /// 查询交易所状态
    #[derive(Debug, Clone, Default)]
    struct QryExchangeSequenceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 交易所状态
    #[derive(Debug, Clone, Default)]
    struct ExchangeSequenceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 序号
        SequenceNo: i32,
        /// 合约交易状态
        MarketStatus: u8,
    }
    /// 根据价格查询最大报单数量
    #[derive(Debug, Clone, Default)]
    struct QryMaxOrderVolumeWithPriceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 买卖方向
        Direction: u8,
        /// 开平标志
        OffsetFlag: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 最大允许报单数量
        MaxVolume: i32,
        /// 报单价格
        Price: f64,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询经纪公司交易参数
    #[derive(Debug, Clone, Default)]
    struct QryBrokerTradingParamsField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 币种代码
        CurrencyID: String,
        /// 投资者帐号
        AccountID: String,
    }
    /// 经纪公司交易参数
    #[derive(Debug, Clone, Default)]
    struct BrokerTradingParamsField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 保证金价格类型
        MarginPriceType: u8,
        /// 盈亏算法
        Algorithm: u8,
        /// 可用是否包含平仓盈利
        AvailIncludeCloseProfit: u8,
        /// 币种代码
        CurrencyID: String,
        /// 期权权利金价格类型
        OptionRoyaltyPriceType: u8,
        /// 投资者帐号
        AccountID: String,
    }
    /// 查询经纪公司交易算法
    #[derive(Debug, Clone, Default)]
    struct QryBrokerTradingAlgosField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 经纪公司交易算法
    #[derive(Debug, Clone, Default)]
    struct BrokerTradingAlgosField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 持仓处理算法编号
        HandlePositionAlgoID: u8,
        /// 寻找保证金率算法编号
        FindMarginRateAlgoID: u8,
        /// 资金处理算法编号
        HandleTradingAccountAlgoID: u8,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询经纪公司资金
    #[derive(Debug, Clone, Default)]
    struct QueryBrokerDepositField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
    }
    /// 经纪公司资金
    #[derive(Debug, Clone, Default)]
    struct BrokerDepositField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日期
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 会员代码
        ParticipantID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 上次结算准备金
        PreBalance: f64,
        /// 当前保证金总额
        CurrMargin: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 期货结算准备金
        Balance: f64,
        /// 入金金额
        Deposit: f64,
        /// 出金金额
        Withdraw: f64,
        /// 可提资金
        Available: f64,
        /// 基本准备金
        Reserve: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
    }
    /// 查询保证金监管系统经纪公司密钥
    #[derive(Debug, Clone, Default)]
    struct QryCFMMCBrokerKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 保证金监管系统经纪公司密钥
    #[derive(Debug, Clone, Default)]
    struct CFMMCBrokerKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 经纪公司统一编码
        ParticipantID: String,
        /// 密钥生成日期
        CreateDate: String,
        /// 密钥生成时间
        CreateTime: String,
        /// 密钥编号
        KeyID: i32,
        /// 动态密钥
        CurrentKey: Vec<u8>,
        /// 动态密钥类型
        KeyKind: u8,
    }
    /// 保证金监管系统经纪公司资金账户密钥
    #[derive(Debug, Clone, Default)]
    struct CFMMCTradingAccountKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 经纪公司统一编码
        ParticipantID: String,
        /// 投资者帐号
        AccountID: String,
        /// 密钥编号
        KeyID: i32,
        /// 动态密钥
        CurrentKey: Vec<u8>,
    }
    /// 请求查询保证金监管系统经纪公司资金账户密钥
    #[derive(Debug, Clone, Default)]
    struct QryCFMMCTradingAccountKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 用户动态令牌参数
    #[derive(Debug, Clone, Default)]
    struct BrokerUserOTPParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 动态令牌提供商
        OTPVendorsID: String,
        /// 动态令牌序列号
        SerialNumber: Vec<u8>,
        /// 令牌密钥
        AuthKey: Vec<u8>,
        /// 漂移值
        LastDrift: i32,
        /// 成功值
        LastSuccess: i32,
        /// 动态令牌类型
        OTPType: u8,
    }
    /// 手工同步用户动态令牌
    #[derive(Debug, Clone, Default)]
    struct ManualSyncBrokerUserOTPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 动态令牌类型
        OTPType: u8,
        /// 第一个动态密码
        FirstOTP: Vec<u8>,
        /// 第二个动态密码
        SecondOTP: Vec<u8>,
    }
    /// 投资者手续费率模板
    #[derive(Debug, Clone, Default)]
    struct CommRateModelField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 手续费率模板代码
        CommModelID: String,
        /// 模板名称
        CommModelName: String,
    }
    /// 请求查询投资者手续费率模板
    #[derive(Debug, Clone, Default)]
    struct QryCommRateModelField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 手续费率模板代码
        CommModelID: String,
    }
    /// 投资者保证金率模板
    #[derive(Debug, Clone, Default)]
    struct MarginModelField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 保证金率模板代码
        MarginModelID: String,
        /// 模板名称
        MarginModelName: String,
    }
    /// 请求查询投资者保证金率模板
    #[derive(Debug, Clone, Default)]
    struct QryMarginModelField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 保证金率模板代码
        MarginModelID: String,
    }
    /// 仓单折抵信息
    #[derive(Debug, Clone, Default)]
    struct EWarrantOffsetField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日期
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 买卖方向
        Direction: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 数量
        Volume: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询仓单折抵信息
    #[derive(Debug, Clone, Default)]
    struct QryEWarrantOffsetField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 查询投资者品种/跨品种保证金
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProductGroupMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 品种/跨品种标示
        ProductGroupID: String,
    }
    /// 投资者品种/跨品种保证金
    #[derive(Debug, Clone, Default)]
    struct InvestorProductGroupMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 多头冻结的保证金
        LongFrozenMargin: f64,
        /// 空头冻结的保证金
        ShortFrozenMargin: f64,
        /// 占用的保证金
        UseMargin: f64,
        /// 多头保证金
        LongUseMargin: f64,
        /// 空头保证金
        ShortUseMargin: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 交易所多头保证金
        LongExchMargin: f64,
        /// 交易所空头保证金
        ShortExchMargin: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 手续费
        Commission: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 资金差额
        CashIn: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 折抵总金额
        OffsetAmount: f64,
        /// 多头折抵总金额
        LongOffsetAmount: f64,
        /// 空头折抵总金额
        ShortOffsetAmount: f64,
        /// 交易所折抵总金额
        ExchOffsetAmount: f64,
        /// 交易所多头折抵总金额
        LongExchOffsetAmount: f64,
        /// 交易所空头折抵总金额
        ShortExchOffsetAmount: f64,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 交易所代码
        ExchangeID: String,
        /// 投资单元代码
        InvestUnitID: String,
        /// 品种/跨品种标示
        ProductGroupID: String,
    }
    /// 查询监控中心用户令牌
    #[derive(Debug, Clone, Default)]
    struct QueryCFMMCTradingAccountTokenField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投资单元代码
        InvestUnitID: String,
    }
    /// 监控中心用户令牌
    #[derive(Debug, Clone, Default)]
    struct CFMMCTradingAccountTokenField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 经纪公司统一编码
        ParticipantID: String,
        /// 投资者帐号
        AccountID: String,
        /// 密钥编号
        KeyID: i32,
        /// 动态令牌
        Token: Vec<u8>,
    }
    /// 查询产品组
    #[derive(Debug, Clone, Default)]
    struct QryProductGroupField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
    }
    /// 投资者品种/跨品种保证金产品组
    #[derive(Debug, Clone, Default)]
    struct ProductGroupField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 产品组代码
        ProductGroupID: String,
    }
    /// 交易所公告
    #[derive(Debug, Clone, Default)]
    struct BulletinField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 交易日
        TradingDay: String,
        /// 公告编号
        BulletinID: i32,
        /// 序列号
        SequenceNo: i32,
        /// 公告类型
        NewsType: Vec<u8>,
        /// 紧急程度
        NewsUrgency: u8,
        /// 发送时间
        SendTime: String,
        /// 消息摘要
        Abstract: Vec<u8>,
        /// 消息来源
        ComeFrom: Vec<u8>,
        /// 消息正文
        Content: Vec<u8>,
        /// WEB地址
        URLLink: Vec<u8>,
        /// 市场代码
        MarketID: String,
    }
    /// 查询交易所公告
    #[derive(Debug, Clone, Default)]
    struct QryBulletinField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 公告编号
        BulletinID: i32,
        /// 序列号
        SequenceNo: i32,
        /// 公告类型
        NewsType: Vec<u8>,
        /// 紧急程度
        NewsUrgency: u8,
    }
    /// MulticastInstrument
    #[derive(Debug, Clone, Default)]
    struct MulticastInstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 主题号
        TopicID: i32,
        /// 合约编号
        InstrumentNo: i32,
        /// 基准价
        CodePrice: f64,
        /// 合约数量乘数
        VolumeMultiple: i32,
        /// 最小变动价位
        PriceTick: f64,
        /// 合约代码
        InstrumentID: String,
    }
    /// QryMulticastInstrument
    #[derive(Debug, Clone, Default)]
    struct QryMulticastInstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 主题号
        TopicID: i32,
        /// 合约代码
        InstrumentID: String,
    }
    /// App客户端权限分配
    #[derive(Debug, Clone, Default)]
    struct AppIDAuthAssignField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// App代码
        AppID: String,
        /// 交易中心代码
        DRIdentityID: i32,
    }
    /// 转帐开户请求
    #[derive(Debug, Clone, Default)]
    struct ReqOpenAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 汇钞标志
        CashExchangeCode: u8,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 交易ID
        TID: i32,
        /// 用户标识
        UserID: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 转帐销户请求
    #[derive(Debug, Clone, Default)]
    struct ReqCancelAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 汇钞标志
        CashExchangeCode: u8,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 交易ID
        TID: i32,
        /// 用户标识
        UserID: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 变更银行账户请求
    #[derive(Debug, Clone, Default)]
    struct ReqChangeAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 新银行帐号
        NewBankAccount: String,
        /// 新银行密码
        NewBankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 银行帐号类型
        BankAccType: u8,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易ID
        TID: i32,
        /// 摘要
        Digest: Vec<u8>,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 转账请求
    #[derive(Debug, Clone, Default)]
    struct ReqTransferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 期货可取金额
        FutureFetchAmount: f64,
        /// 费用支付标志
        FeePayFlag: u8,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 发送方给接收方的消息
        Message: Vec<u8>,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 转账交易状态
        TransferStatus: u8,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 银行发起银行资金转期货响应
    #[derive(Debug, Clone, Default)]
    struct RspTransferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 期货可取金额
        FutureFetchAmount: f64,
        /// 费用支付标志
        FeePayFlag: u8,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 发送方给接收方的消息
        Message: Vec<u8>,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 转账交易状态
        TransferStatus: u8,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 冲正请求
    #[derive(Debug, Clone, Default)]
    struct ReqRepealField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 冲正时间间隔
        RepealTimeInterval: i32,
        /// 已经冲正次数
        RepealedTimes: i32,
        /// 银行冲正标志
        BankRepealFlag: u8,
        /// 期商冲正标志
        BrokerRepealFlag: u8,
        /// 被冲正平台流水号
        PlateRepealSerial: i32,
        /// 被冲正银行流水号
        BankRepealSerial: String,
        /// 被冲正期货流水号
        FutureRepealSerial: i32,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 期货可取金额
        FutureFetchAmount: f64,
        /// 费用支付标志
        FeePayFlag: u8,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 发送方给接收方的消息
        Message: Vec<u8>,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 转账交易状态
        TransferStatus: u8,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 冲正响应
    #[derive(Debug, Clone, Default)]
    struct RspRepealField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 冲正时间间隔
        RepealTimeInterval: i32,
        /// 已经冲正次数
        RepealedTimes: i32,
        /// 银行冲正标志
        BankRepealFlag: u8,
        /// 期商冲正标志
        BrokerRepealFlag: u8,
        /// 被冲正平台流水号
        PlateRepealSerial: i32,
        /// 被冲正银行流水号
        BankRepealSerial: String,
        /// 被冲正期货流水号
        FutureRepealSerial: i32,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 期货可取金额
        FutureFetchAmount: f64,
        /// 费用支付标志
        FeePayFlag: u8,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 发送方给接收方的消息
        Message: Vec<u8>,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 转账交易状态
        TransferStatus: u8,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 查询账户信息请求
    #[derive(Debug, Clone, Default)]
    struct ReqQueryAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 查询账户信息响应
    #[derive(Debug, Clone, Default)]
    struct RspQueryAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 银行可用金额
        BankUseAmount: f64,
        /// 银行可取金额
        BankFetchAmount: f64,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 期商签到签退
    #[derive(Debug, Clone, Default)]
    struct FutureSignIOField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 币种代码
        CurrencyID: String,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
    }
    /// 期商签到响应
    #[derive(Debug, Clone, Default)]
    struct RspFutureSignInField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 币种代码
        CurrencyID: String,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// PIN密钥
        PinKey: Vec<u8>,
        /// MAC密钥
        MacKey: Vec<u8>,
    }
    /// 期商签退请求
    #[derive(Debug, Clone, Default)]
    struct ReqFutureSignOutField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 币种代码
        CurrencyID: String,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
    }
    /// 期商签退响应
    #[derive(Debug, Clone, Default)]
    struct RspFutureSignOutField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 币种代码
        CurrencyID: String,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 查询指定流水号的交易结果请求
    #[derive(Debug, Clone, Default)]
    struct ReqQueryTradeResultBySerialField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 流水号
        Reference: i32,
        /// 本流水号发布者的机构类型
        RefrenceIssureType: u8,
        /// 本流水号发布者机构编码
        RefrenceIssure: Vec<u8>,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 摘要
        Digest: Vec<u8>,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 查询指定流水号的交易结果响应
    #[derive(Debug, Clone, Default)]
    struct RspQueryTradeResultBySerialField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 流水号
        Reference: i32,
        /// 本流水号发布者的机构类型
        RefrenceIssureType: u8,
        /// 本流水号发布者机构编码
        RefrenceIssure: Vec<u8>,
        /// 原始返回代码
        OriginReturnCode: String,
        /// 原始返回码描述
        OriginDescrInfoForReturnCode: String,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 摘要
        Digest: Vec<u8>,
    }
    /// 日终文件就绪请求
    #[derive(Debug, Clone, Default)]
    struct ReqDayEndFileReadyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 文件业务功能
        FileBusinessCode: u8,
        /// 摘要
        Digest: Vec<u8>,
    }
    /// 返回结果
    #[derive(Debug, Clone, Default)]
    struct ReturnResultField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 返回代码
        ReturnCode: String,
        /// 返回码描述
        DescrInfoForReturnCode: String,
    }
    /// 验证期货资金密码
    #[derive(Debug, Clone, Default)]
    struct VerifyFuturePasswordField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 安装编号
        InstallID: i32,
        /// 交易ID
        TID: i32,
        /// 币种代码
        CurrencyID: String,
    }
    /// 验证客户信息
    #[derive(Debug, Clone, Default)]
    struct VerifyCustInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 验证期货资金密码和客户信息
    #[derive(Debug, Clone, Default)]
    struct VerifyFuturePasswordAndCustInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 币种代码
        CurrencyID: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 验证期货资金密码和客户信息
    #[derive(Debug, Clone, Default)]
    struct DepositResultInformField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 出入金流水号，该流水号为银期报盘返回的流水号
        DepositSeqNo: Vec<u8>,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 入金金额
        Deposit: f64,
        /// 请求编号
        RequestID: i32,
        /// 返回代码
        ReturnCode: String,
        /// 返回码描述
        DescrInfoForReturnCode: String,
    }
    /// 交易核心向银期报盘发出密钥同步请求
    #[derive(Debug, Clone, Default)]
    struct ReqSyncKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 交易核心给银期报盘的消息
        Message: Vec<u8>,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
    }
    /// 交易核心向银期报盘发出密钥同步响应
    #[derive(Debug, Clone, Default)]
    struct RspSyncKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 交易核心给银期报盘的消息
        Message: Vec<u8>,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 查询账户信息通知
    #[derive(Debug, Clone, Default)]
    struct NotifyQueryAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 银行可用金额
        BankUseAmount: f64,
        /// 银行可取金额
        BankFetchAmount: f64,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 银期转账交易流水表
    #[derive(Debug, Clone, Default)]
    struct TransferSerialField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 平台流水号
        PlateSerial: i32,
        /// 交易发起方日期
        TradeDate: String,
        /// 交易日期
        TradingDay: String,
        /// 交易时间
        TradeTime: String,
        /// 交易代码
        TradeCode: String,
        /// 会话编号
        SessionID: i32,
        /// 银行编码
        BankID: String,
        /// 银行分支机构编码
        BankBranchID: String,
        /// 银行帐号类型
        BankAccType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行流水号
        BankSerial: String,
        /// 期货公司编码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 期货公司帐号类型
        FutureAccType: u8,
        /// 投资者帐号
        AccountID: String,
        /// 投资者代码
        InvestorID: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 币种代码
        CurrencyID: String,
        /// 交易金额
        TradeAmount: f64,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 有效标志
        AvailabilityFlag: u8,
        /// 操作员
        OperatorCode: String,
        /// 新银行帐号
        BankNewAccount: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 请求查询转帐流水
    #[derive(Debug, Clone, Default)]
    struct QryTransferSerialField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 银行编码
        BankID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 期商签到通知
    #[derive(Debug, Clone, Default)]
    struct NotifyFutureSignInField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 币种代码
        CurrencyID: String,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// PIN密钥
        PinKey: Vec<u8>,
        /// MAC密钥
        MacKey: Vec<u8>,
    }
    /// 期商签退通知
    #[derive(Debug, Clone, Default)]
    struct NotifyFutureSignOutField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 币种代码
        CurrencyID: String,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 交易核心向银期报盘发出密钥同步处理结果的通知
    #[derive(Debug, Clone, Default)]
    struct NotifySyncKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 交易核心给银期报盘的消息
        Message: Vec<u8>,
        /// 渠道标志
        DeviceID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 请求查询银期签约关系
    #[derive(Debug, Clone, Default)]
    struct QryAccountregisterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 银行编码
        BankID: String,
        /// 银行分支机构编码
        BankBranchID: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 客户开销户信息表
    #[derive(Debug, Clone, Default)]
    struct AccountregisterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日期
        TradeDay: String,
        /// 银行编码
        BankID: String,
        /// 银行分支机构编码
        BankBranchID: String,
        /// 银行帐号
        BankAccount: String,
        /// 期货公司编码
        BrokerID: String,
        /// 期货公司分支机构编码
        BrokerBranchID: String,
        /// 投资者帐号
        AccountID: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户姓名
        CustomerName: String,
        /// 币种代码
        CurrencyID: String,
        /// 开销户类别
        OpenOrDestroy: u8,
        /// 签约日期
        RegDate: String,
        /// 解约日期
        OutDate: String,
        /// 交易ID
        TID: i32,
        /// 客户类型
        CustType: u8,
        /// 银行帐号类型
        BankAccType: u8,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 银期开户信息
    #[derive(Debug, Clone, Default)]
    struct OpenAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 汇钞标志
        CashExchangeCode: u8,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 交易ID
        TID: i32,
        /// 用户标识
        UserID: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 银期销户信息
    #[derive(Debug, Clone, Default)]
    struct CancelAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 汇钞标志
        CashExchangeCode: u8,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 交易ID
        TID: i32,
        /// 用户标识
        UserID: String,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 银期变更银行账号信息
    #[derive(Debug, Clone, Default)]
    struct ChangeAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 新银行帐号
        NewBankAccount: String,
        /// 新银行密码
        NewBankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 银行帐号类型
        BankAccType: u8,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易ID
        TID: i32,
        /// 摘要
        Digest: Vec<u8>,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
    }
    /// 二级代理操作员银期权限
    #[derive(Debug, Clone, Default)]
    struct SecAgentACIDMapField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 资金账户
        AccountID: String,
        /// 币种
        CurrencyID: String,
        /// 境外中介机构资金帐号
        BrokerSecAgentID: String,
    }
    /// 二级代理操作员银期权限查询
    #[derive(Debug, Clone, Default)]
    struct QrySecAgentACIDMapField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 资金账户
        AccountID: String,
        /// 币种
        CurrencyID: String,
    }
    /// 灾备中心交易权限
    #[derive(Debug, Clone, Default)]
    struct UserRightsAssignField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 应用单元代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 交易中心代码
        DRIdentityID: i32,
    }
    /// 经济公司是否有在本标示的交易权限
    #[derive(Debug, Clone, Default)]
    struct BrokerUserRightAssignField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 应用单元代码
        BrokerID: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 能否交易
        Tradeable: i32,
    }
    /// 灾备交易转换报文
    #[derive(Debug, Clone, Default)]
    struct DRTransferField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 原交易中心代码
        OrigDRIdentityID: i32,
        /// 目标交易中心代码
        DestDRIdentityID: i32,
        /// 原应用单元代码
        OrigBrokerID: String,
        /// 目标易用单元代码
        DestBrokerID: String,
    }
    /// Fens用户信息
    #[derive(Debug, Clone, Default)]
    struct FensUserInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 登录模式
        LoginMode: u8,
    }
    /// 当前银期所属交易中心
    #[derive(Debug, Clone, Default)]
    struct CurrTransferIdentityField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易中心代码
        IdentityID: i32,
    }
    /// 禁止登录用户
    #[derive(Debug, Clone, Default)]
    struct LoginForbiddenUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// IP地址
        IPAddress: String,
    }
    /// 查询禁止登录用户
    #[derive(Debug, Clone, Default)]
    struct QryLoginForbiddenUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 资金账户基本准备金
    #[derive(Debug, Clone, Default)]
    struct TradingAccountReserveField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 基本准备金
        Reserve: f64,
        /// 币种代码
        CurrencyID: String,
    }
    /// 查询禁止登录IP
    #[derive(Debug, Clone, Default)]
    struct QryLoginForbiddenIPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// IP地址
        IPAddress: String,
    }
    /// 查询IP列表
    #[derive(Debug, Clone, Default)]
    struct QryIPListField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// IP地址
        IPAddress: String,
    }
    /// 查询用户下单权限分配表
    #[derive(Debug, Clone, Default)]
    struct QryUserRightsAssignField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 应用单元代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 银期预约开户确认请求
    #[derive(Debug, Clone, Default)]
    struct ReserveOpenAccountConfirmField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易ID
        TID: i32,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 预约开户银行流水号
        BankReserveOpenSeq: Vec<u8>,
        /// 预约开户日期
        BookDate: String,
        /// 预约开户验证密码
        BookPsw: Vec<u8>,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 银期预约开户
    #[derive(Debug, Clone, Default)]
    struct ReserveOpenAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 性别
        Gender: u8,
        /// 国家代码
        CountryCode: String,
        /// 客户类型
        CustType: u8,
        /// 地址
        Address: String,
        /// 邮编
        ZipCode: String,
        /// 电话号码
        Telephone: String,
        /// 手机
        MobilePhone: String,
        /// 传真
        Fax: String,
        /// 电子邮件
        EMail: String,
        /// 资金账户状态
        MoneyAccountStatus: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 安装编号
        InstallID: i32,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 交易ID
        TID: i32,
        /// 预约开户状态
        ReserveOpenAccStas: u8,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
    }
    /// 银行账户属性
    #[derive(Debug, Clone, Default)]
    struct AccountPropertyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 银行统一标识类型
        BankID: String,
        /// 银行账户
        BankAccount: String,
        /// 银行账户的开户人名称
        OpenName: String,
        /// 银行账户的开户行
        OpenBank: Vec<u8>,
        /// 是否活跃
        IsActive: i32,
        /// 账户来源
        AccountSourceType: u8,
        /// 开户日期
        OpenDate: String,
        /// 注销日期
        CancelDate: String,
        /// 录入员代码
        OperatorID: String,
        /// 录入日期
        OperateDate: String,
        /// 录入时间
        OperateTime: String,
        /// 币种代码
        CurrencyID: String,
    }
    /// 查询当前交易中心
    #[derive(Debug, Clone, Default)]
    struct QryCurrDRIdentityField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易中心代码
        DRIdentityID: i32,
    }
    /// 当前交易中心
    #[derive(Debug, Clone, Default)]
    struct CurrDRIdentityField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易中心代码
        DRIdentityID: i32,
    }
    /// 查询二级代理商资金校验模式
    #[derive(Debug, Clone, Default)]
    struct QrySecAgentCheckModeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 查询二级代理商信息
    #[derive(Debug, Clone, Default)]
    struct QrySecAgentTradeInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 境外中介机构资金帐号
        BrokerSecAgentID: String,
    }
    /// 用户发出获取安全安全登陆方法请求
    #[derive(Debug, Clone, Default)]
    struct ReqUserAuthMethodField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 用户发出获取安全安全登陆方法回复
    #[derive(Debug, Clone, Default)]
    struct RspUserAuthMethodField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 当前可以用的认证模式
        UsableAuthMethod: i32,
    }
    /// 用户发出获取安全安全登陆方法请求
    #[derive(Debug, Clone, Default)]
    struct ReqGenUserCaptchaField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 生成的图片验证码信息
    #[derive(Debug, Clone, Default)]
    struct RspGenUserCaptchaField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 图片信息长度
        CaptchaInfoLen: i32,
        /// 图片信息
        CaptchaInfo: String,
    }
    /// 用户发出获取安全安全登陆方法请求
    #[derive(Debug, Clone, Default)]
    struct ReqGenUserTextField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// 短信验证码生成的回复
    #[derive(Debug, Clone, Default)]
    struct RspGenUserTextField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 短信验证码序号
        UserTextSeq: i32,
    }
    /// 用户发出带图形验证码的登录请求请求
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginWithCaptchaField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 密码
        Password: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// Mac地址
        MacAddress: String,
        /// 登录备注
        LoginRemark: String,
        /// 图形验证码的文字内容
        Captcha: Vec<u8>,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 终端IP地址
        ClientIPAddress: String,
    }
    /// 用户发出带短信验证码的登录请求请求
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginWithTextField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 密码
        Password: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// Mac地址
        MacAddress: String,
        /// 登录备注
        LoginRemark: String,
        /// 短信验证码文字内容
        Text: Vec<u8>,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 终端IP地址
        ClientIPAddress: String,
    }
    /// 用户发出带动态验证码的登录请求请求
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginWithOTPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 密码
        Password: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// Mac地址
        MacAddress: String,
        /// 登录备注
        LoginRemark: String,
        /// OTP密码
        OTPPassword: String,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 终端IP地址
        ClientIPAddress: String,
    }
    /// api握手请求
    #[derive(Debug, Clone, Default)]
    struct ReqApiHandshakeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// api与front通信密钥版本号
        CryptoKeyVersion: String,
    }
    /// front发给api的握手回复
    #[derive(Debug, Clone, Default)]
    struct RspApiHandshakeField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 握手回复数据长度
        FrontHandshakeDataLen: i32,
        /// 握手回复数据
        FrontHandshakeData: Vec<u8>,
        /// API认证是否开启
        IsApiAuthEnabled: i32,
    }
    /// api给front的验证key的请求
    #[derive(Debug, Clone, Default)]
    struct ReqVerifyApiKeyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 握手回复数据长度
        ApiHandshakeDataLen: i32,
        /// 握手回复数据
        ApiHandshakeData: Vec<u8>,
    }
    /// 操作员组织架构关系
    #[derive(Debug, Clone, Default)]
    struct DepartmentUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 投资者代码
        InvestorID: String,
    }
    /// 查询频率，每秒查询比数
    #[derive(Debug, Clone, Default)]
    struct QueryFreqField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 查询频率
        QueryFreq: i32,
        /// FTD频率
        FTDPkgFreq: i32,
    }
    /// 禁止认证IP
    #[derive(Debug, Clone, Default)]
    struct AuthForbiddenIPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// IP地址
        IPAddress: String,
    }
    /// 查询禁止认证IP
    #[derive(Debug, Clone, Default)]
    struct QryAuthForbiddenIPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// IP地址
        IPAddress: String,
    }
    /// 换汇可提冻结
    #[derive(Debug, Clone, Default)]
    struct SyncDelaySwapFrozenField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 换汇流水号
        DelaySwapSeqNo: Vec<u8>,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 源币种
        FromCurrencyID: String,
        /// 源剩余换汇额度(可提冻结)
        FromRemainSwap: f64,
        /// 是否手工换汇
        IsManualSwap: i32,
    }
    /// 用户系统信息
    #[derive(Debug, Clone, Default)]
    struct UserSystemInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 用户端系统内部信息长度
        ClientSystemInfoLen: i32,
        /// 用户端系统内部信息
        ClientSystemInfo: String,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 登录成功时间
        ClientLoginTime: String,
        /// App代码
        ClientAppID: String,
        /// 用户公网IP
        ClientPublicIP: String,
        /// 客户登录备注2
        ClientLoginRemark: String,
    }
    /// 终端用户绑定信息
    #[derive(Debug, Clone, Default)]
    struct AuthUserIDField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// App代码
        AppID: String,
        /// 用户代码
        UserID: String,
        /// 校验类型
        AuthType: u8,
    }
    /// 用户IP绑定信息
    #[derive(Debug, Clone, Default)]
    struct AuthIPField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// App代码
        AppID: String,
        /// 用户代码
        IPAddress: String,
    }
    /// 查询分类合约
    #[derive(Debug, Clone, Default)]
    struct QryClassifiedInstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 产品代码
        ProductID: String,
        /// 合约交易状态
        TradingType: u8,
        /// 合约分类类型
        ClassType: u8,
    }
    /// 查询组合优惠比例
    #[derive(Debug, Clone, Default)]
    struct QryCombPromotionParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 组合优惠比例
    #[derive(Debug, Clone, Default)]
    struct CombPromotionParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 投机套保标志
        CombHedgeFlag: String,
        /// 期权组合保证金比例
        Xparameter: f64,
    }
    /// 国密用户登录请求
    #[derive(Debug, Clone, Default)]
    struct ReqUserLoginSMField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 密码
        Password: String,
        /// 用户端产品信息
        UserProductInfo: String,
        /// 接口端产品信息
        InterfaceProductInfo: String,
        /// 协议信息
        ProtocolInfo: String,
        /// Mac地址
        MacAddress: String,
        /// 动态密码
        OneTimePassword: String,
        /// 登录备注
        LoginRemark: String,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 终端IP地址
        ClientIPAddress: String,
        /// 经纪公司名称
        BrokerName: String,
        /// 认证码
        AuthCode: String,
        /// App代码
        AppID: String,
        /// PIN码
        PIN: Vec<u8>,
    }
    /// 投资者风险结算持仓查询
    #[derive(Debug, Clone, Default)]
    struct QryRiskSettleInvstPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// 风险结算产品查询
    #[derive(Debug, Clone, Default)]
    struct QryRiskSettleProductStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品代码
        ProductID: String,
    }
    /// 投资者风险结算持仓
    #[derive(Debug, Clone, Default)]
    struct RiskSettleInvstPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 持仓多空方向
        PosiDirection: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 持仓日期
        PositionDate: u8,
        /// 上日持仓
        YdPosition: i32,
        /// 今日持仓
        Position: i32,
        /// 多头冻结
        LongFrozen: i32,
        /// 空头冻结
        ShortFrozen: i32,
        /// 开仓冻结金额
        LongFrozenAmount: f64,
        /// 开仓冻结金额
        ShortFrozenAmount: f64,
        /// 开仓量
        OpenVolume: i32,
        /// 平仓量
        CloseVolume: i32,
        /// 开仓金额
        OpenAmount: f64,
        /// 平仓金额
        CloseAmount: f64,
        /// 持仓成本
        PositionCost: f64,
        /// 上次占用的保证金
        PreMargin: f64,
        /// 占用的保证金
        UseMargin: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 资金差额
        CashIn: f64,
        /// 手续费
        Commission: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 开仓成本
        OpenCost: f64,
        /// 交易所保证金
        ExchangeMargin: f64,
        /// 组合成交形成的持仓
        CombPosition: i32,
        /// 组合多头冻结
        CombLongFrozen: i32,
        /// 组合空头冻结
        CombShortFrozen: i32,
        /// 逐日盯市平仓盈亏
        CloseProfitByDate: f64,
        /// 逐笔对冲平仓盈亏
        CloseProfitByTrade: f64,
        /// 今日持仓
        TodayPosition: i32,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 执行冻结
        StrikeFrozen: i32,
        /// 执行冻结金额
        StrikeFrozenAmount: f64,
        /// 放弃执行冻结
        AbandonFrozen: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 执行冻结的昨仓
        YdStrikeFrozen: i32,
        /// 投资单元代码
        InvestUnitID: String,
        /// 持仓成本差值
        PositionCostOffset: f64,
        /// tas持仓手数
        TasPosition: i32,
        /// tas持仓成本
        TasPositionCost: f64,
    }
    /// 风险品种
    #[derive(Debug, Clone, Default)]
    struct RiskSettleProductStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 产品编号
        ProductID: String,
        /// 产品结算状态
        ProductStatus: u8,
    }
    /// 风险结算追平信息
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
        /// 追平状态
        SyncDeltaStatus: u8,
        /// 追平描述
        SyncDescription: Vec<u8>,
        /// 是否只有资金追平
        IsOnlyTrdDelta: i32,
    }
    /// 风险结算追平产品信息
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaProductStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 是否允许交易
        ProductStatus: u8,
    }
    /// 风险结算追平持仓明细
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstPosDtlField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 买卖
        Direction: u8,
        /// 开仓日期
        OpenDate: String,
        /// 成交编号
        TradeID: String,
        /// 数量
        Volume: i32,
        /// 开仓价
        OpenPrice: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 成交类型
        TradeType: u8,
        /// 组合合约代码
        CombInstrumentID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 逐日盯市平仓盈亏
        CloseProfitByDate: f64,
        /// 逐笔对冲平仓盈亏
        CloseProfitByTrade: f64,
        /// 逐日盯市持仓盈亏
        PositionProfitByDate: f64,
        /// 逐笔对冲持仓盈亏
        PositionProfitByTrade: f64,
        /// 投资者保证金
        Margin: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 昨结算价
        LastSettlementPrice: f64,
        /// 结算价
        SettlementPrice: f64,
        /// 平仓量
        CloseVolume: i32,
        /// 平仓金额
        CloseAmount: f64,
        /// 先开先平剩余数量
        TimeFirstVolume: i32,
        /// 特殊持仓标志
        SpecPosiType: u8,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平组合持仓明细
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstPosCombDtlField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 开仓日期
        OpenDate: String,
        /// 交易所代码
        ExchangeID: String,
        /// 结算编号
        SettlementID: i32,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组合编号
        ComTradeID: String,
        /// 撮合编号
        TradeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 买卖
        Direction: u8,
        /// 持仓量
        TotalAmt: i32,
        /// 投资者保证金
        Margin: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 保证金率
        MarginRateByMoney: f64,
        /// 保证金率(按手数)
        MarginRateByVolume: f64,
        /// 单腿编号
        LegID: i32,
        /// 单腿乘数
        LegMultiple: i32,
        /// 成交组号
        TradeGroupID: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平资金
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaTradingAccountField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 上次质押金额
        PreMortgage: f64,
        /// 上次信用额度
        PreCredit: f64,
        /// 上次存款额
        PreDeposit: f64,
        /// 上次结算准备金
        PreBalance: f64,
        /// 上次占用的保证金
        PreMargin: f64,
        /// 利息基数
        InterestBase: f64,
        /// 利息收入
        Interest: f64,
        /// 入金金额
        Deposit: f64,
        /// 出金金额
        Withdraw: f64,
        /// 冻结的保证金
        FrozenMargin: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 当前保证金总额
        CurrMargin: f64,
        /// 资金差额
        CashIn: f64,
        /// 手续费
        Commission: f64,
        /// 平仓盈亏
        CloseProfit: f64,
        /// 持仓盈亏
        PositionProfit: f64,
        /// 期货结算准备金
        Balance: f64,
        /// 可用资金
        Available: f64,
        /// 可取资金
        WithdrawQuota: f64,
        /// 基本准备金
        Reserve: f64,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 信用额度
        Credit: f64,
        /// 质押金额
        Mortgage: f64,
        /// 交易所保证金
        ExchangeMargin: f64,
        /// 投资者交割保证金
        DeliveryMargin: f64,
        /// 交易所交割保证金
        ExchangeDeliveryMargin: f64,
        /// 保底期货结算准备金
        ReserveBalance: f64,
        /// 币种代码
        CurrencyID: String,
        /// 上次货币质入金额
        PreFundMortgageIn: f64,
        /// 上次货币质出金额
        PreFundMortgageOut: f64,
        /// 货币质入金额
        FundMortgageIn: f64,
        /// 货币质出金额
        FundMortgageOut: f64,
        /// 货币质押余额
        FundMortgageAvailable: f64,
        /// 可质押货币金额
        MortgageableFund: f64,
        /// 特殊产品占用保证金
        SpecProductMargin: f64,
        /// 特殊产品冻结保证金
        SpecProductFrozenMargin: f64,
        /// 特殊产品手续费
        SpecProductCommission: f64,
        /// 特殊产品冻结手续费
        SpecProductFrozenCommission: f64,
        /// 特殊产品持仓盈亏
        SpecProductPositionProfit: f64,
        /// 特殊产品平仓盈亏
        SpecProductCloseProfit: f64,
        /// 根据持仓盈亏算法计算的特殊产品持仓盈亏
        SpecProductPositionProfitByAlg: f64,
        /// 特殊产品交易所保证金
        SpecProductExchangeMargin: f64,
        /// 延时换汇冻结金额
        FrozenSwap: f64,
        /// 剩余换汇额度
        RemainSwap: f64,
        /// 期权市值
        OptionValue: f64,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 投资者风险结算总保证金
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInitInvstMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 追平前总风险保证金
        LastRiskTotalInvstMargin: f64,
        /// 追平前交易所总风险保证金
        LastRiskTotalExchMargin: f64,
        /// 本次追平品种总保证金
        ThisSyncInvstMargin: f64,
        /// 本次追平品种交易所总保证金
        ThisSyncExchMargin: f64,
        /// 本次未追平品种总保证金
        RemainRiskInvstMargin: f64,
        /// 本次未追平品种交易所总保证金
        RemainRiskExchMargin: f64,
        /// 追平前总特殊产品风险保证金
        LastRiskSpecTotalInvstMargin: f64,
        /// 追平前总特殊产品交易所风险保证金
        LastRiskSpecTotalExchMargin: f64,
        /// 本次追平品种特殊产品总保证金
        ThisSyncSpecInvstMargin: f64,
        /// 本次追平品种特殊产品交易所总保证金
        ThisSyncSpecExchMargin: f64,
        /// 本次未追平品种特殊产品总保证金
        RemainRiskSpecInvstMargin: f64,
        /// 本次未追平品种特殊产品交易所总保证金
        RemainRiskSpecExchMargin: f64,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平组合优先级
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaDceCombInstrumentField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        CombInstrumentID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 成交组号
        TradeGroupID: i32,
        /// 投机套保标志
        CombHedgeFlag: u8,
        /// 组合类型
        CombinationType: u8,
        /// 买卖
        Direction: u8,
        /// 产品代码
        ProductID: String,
        /// 期权组合保证金比例
        Xparameter: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平投资者期货保证金率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 是否相对交易所收取
        IsRelative: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平交易所期货保证金率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaExchMarginRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 合约代码
        InstrumentID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平中金现货期权交易所保证金率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaOptExchMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 合约代码
        InstrumentID: String,
        /// 投机空头保证金调整系数
        SShortMarginRatioByMoney: f64,
        /// 投机空头保证金调整系数
        SShortMarginRatioByVolume: f64,
        /// 保值空头保证金调整系数
        HShortMarginRatioByMoney: f64,
        /// 保值空头保证金调整系数
        HShortMarginRatioByVolume: f64,
        /// 套利空头保证金调整系数
        AShortMarginRatioByMoney: f64,
        /// 套利空头保证金调整系数
        AShortMarginRatioByVolume: f64,
        /// 做市商空头保证金调整系数
        MShortMarginRatioByMoney: f64,
        /// 做市商空头保证金调整系数
        MShortMarginRatioByVolume: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平中金现货期权投资者保证金率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaOptInvstMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机空头保证金调整系数
        SShortMarginRatioByMoney: f64,
        /// 投机空头保证金调整系数
        SShortMarginRatioByVolume: f64,
        /// 保值空头保证金调整系数
        HShortMarginRatioByMoney: f64,
        /// 保值空头保证金调整系数
        HShortMarginRatioByVolume: f64,
        /// 套利空头保证金调整系数
        AShortMarginRatioByMoney: f64,
        /// 套利空头保证金调整系数
        AShortMarginRatioByVolume: f64,
        /// 是否跟随交易所收取
        IsRelative: i32,
        /// 做市商空头保证金调整系数
        MShortMarginRatioByMoney: f64,
        /// 做市商空头保证金调整系数
        MShortMarginRatioByVolume: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平期权标的调整保证金率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstMarginRateULField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 多头保证金率
        LongMarginRatioByMoney: f64,
        /// 多头保证金费
        LongMarginRatioByVolume: f64,
        /// 空头保证金率
        ShortMarginRatioByMoney: f64,
        /// 空头保证金费
        ShortMarginRatioByVolume: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平期权手续费率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaOptInvstCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 执行手续费率
        StrikeRatioByMoney: f64,
        /// 执行手续费
        StrikeRatioByVolume: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平期货手续费率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvstCommRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 开仓手续费率
        OpenRatioByMoney: f64,
        /// 开仓手续费
        OpenRatioByVolume: f64,
        /// 平仓手续费率
        CloseRatioByMoney: f64,
        /// 平仓手续费
        CloseRatioByVolume: f64,
        /// 平今手续费率
        CloseTodayRatioByMoney: f64,
        /// 平今手续费
        CloseTodayRatioByVolume: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平交叉汇率
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaProductExchRateField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品代码
        ProductID: String,
        /// 报价币种类型
        QuoteCurrencyID: String,
        /// 汇率
        ExchangeRate: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平行情
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaDepthMarketDataField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 合约代码
        InstrumentID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约在交易所的代码
        ExchangeInstID: String,
        /// 最新价
        LastPrice: f64,
        /// 上次结算价
        PreSettlementPrice: f64,
        /// 昨收盘
        PreClosePrice: f64,
        /// 昨持仓量
        PreOpenInterest: f64,
        /// 今开盘
        OpenPrice: f64,
        /// 最高价
        HighestPrice: f64,
        /// 最低价
        LowestPrice: f64,
        /// 数量
        Volume: i32,
        /// 成交金额
        Turnover: f64,
        /// 持仓量
        OpenInterest: f64,
        /// 今收盘
        ClosePrice: f64,
        /// 本次结算价
        SettlementPrice: f64,
        /// 涨停板价
        UpperLimitPrice: f64,
        /// 跌停板价
        LowerLimitPrice: f64,
        /// 昨虚实度
        PreDelta: f64,
        /// 今虚实度
        CurrDelta: f64,
        /// 最后修改时间
        UpdateTime: String,
        /// 最后修改毫秒
        UpdateMillisec: i32,
        /// 申买价一
        BidPrice1: f64,
        /// 申买量一
        BidVolume1: i32,
        /// 申卖价一
        AskPrice1: f64,
        /// 申卖量一
        AskVolume1: i32,
        /// 申买价二
        BidPrice2: f64,
        /// 申买量二
        BidVolume2: i32,
        /// 申卖价二
        AskPrice2: f64,
        /// 申卖量二
        AskVolume2: i32,
        /// 申买价三
        BidPrice3: f64,
        /// 申买量三
        BidVolume3: i32,
        /// 申卖价三
        AskPrice3: f64,
        /// 申卖量三
        AskVolume3: i32,
        /// 申买价四
        BidPrice4: f64,
        /// 申买量四
        BidVolume4: i32,
        /// 申卖价四
        AskPrice4: f64,
        /// 申卖量四
        AskVolume4: i32,
        /// 申买价五
        BidPrice5: f64,
        /// 申买量五
        BidVolume5: i32,
        /// 申卖价五
        AskPrice5: f64,
        /// 申卖量五
        AskVolume5: i32,
        /// 当日均价
        AveragePrice: f64,
        /// 业务日期
        ActionDay: String,
        /// 上带价
        BandingUpperPrice: f64,
        /// 下带价
        BandingLowerPrice: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平现货指数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaIndexPriceField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 合约代码
        InstrumentID: String,
        /// 指数现货收盘价
        ClosePrice: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平仓单折抵
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaEWarrantOffsetField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日期
        TradingDay: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 买卖方向
        Direction: u8,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 数量
        Volume: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// SPBM期货合约保证金参数
    #[derive(Debug, Clone, Default)]
    struct SPBMFutureParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 期货合约因子
        Cvf: i32,
        /// 阶段标识
        TimeRange: u8,
        /// 品种保证金标准
        MarginRate: f64,
        /// 期货合约内部对锁仓费率折扣比例
        LockRateX: f64,
        /// 提高保证金标准
        AddOnRate: f64,
        /// 昨结算价
        PreSettlementPrice: f64,
        /// 期货合约内部对锁仓附加费率折扣比例
        AddOnLockRateX2: f64,
    }
    /// SPBM期权合约保证金参数
    #[derive(Debug, Clone, Default)]
    struct SPBMOptionParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 期权合约因子
        Cvf: i32,
        /// 期权冲抵价格
        DownPrice: f64,
        /// Delta值
        Delta: f64,
        /// 卖方期权风险转换最低值
        SlimiDelta: f64,
        /// 昨结算价
        PreSettlementPrice: f64,
    }
    /// SPBM品种内对锁仓折扣参数
    #[derive(Debug, Clone, Default)]
    struct SPBMIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 品种内合约间对锁仓费率折扣比例
        IntraRateY: f64,
        /// 品种内合约间对锁仓附加费率折扣比例
        AddOnIntraRateY2: f64,
    }
    /// SPBM跨品种抵扣参数
    #[derive(Debug, Clone, Default)]
    struct SPBMInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 优先级
        SpreadId: i32,
        /// 品种间对锁仓费率折扣比例
        InterRateZ: f64,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
    }
    /// 同步SPBM参数结束
    #[derive(Debug, Clone, Default)]
    struct SyncSPBMParameterEndField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
    }
    /// SPBM期货合约保证金参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMFutureParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 品种代码
        ProdFamilyCode: String,
    }
    /// SPBM期权合约保证金参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMOptionParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 品种代码
        ProdFamilyCode: String,
    }
    /// SPBM品种内对锁仓折扣参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 品种代码
        ProdFamilyCode: String,
    }
    /// SPBM跨品种抵扣参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
    }
    /// 组合保证金套餐
    #[derive(Debug, Clone, Default)]
    struct SPBMPortfDefinitionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 组合保证金套餐代码
        PortfolioDefID: i32,
        /// 品种代码
        ProdFamilyCode: String,
        /// 是否启用SPBM
        IsSPBM: i32,
    }
    /// 投资者套餐选择
    #[derive(Debug, Clone, Default)]
    struct SPBMInvestorPortfDefField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组合保证金套餐代码
        PortfolioDefID: i32,
    }
    /// 投资者新型组合保证金系数
    #[derive(Debug, Clone, Default)]
    struct InvestorPortfMarginRatioField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者范围
        InvestorRange: u8,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 会员对投资者收取的保证金和交易所对投资者收取的保证金的比例
        MarginRatio: f64,
        /// 产品群代码
        ProductGroupID: String,
    }
    /// 组合保证金套餐查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMPortfDefinitionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 组合保证金套餐代码
        PortfolioDefID: i32,
        /// 品种代码
        ProdFamilyCode: String,
    }
    /// 投资者套餐选择查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMInvestorPortfDefField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
    }
    /// 投资者新型组合保证金系数查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPortfMarginRatioField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品群代码
        ProductGroupID: String,
    }
    /// 投资者产品SPBM明细
    #[derive(Debug, Clone, Default)]
    struct InvestorProdSPBMDetailField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 合约内对锁保证金
        IntraInstrMargin: f64,
        /// 买归集保证金
        BCollectingMargin: f64,
        /// 卖归集保证金
        SCollectingMargin: f64,
        /// 品种内合约间对锁保证金
        IntraProdMargin: f64,
        /// 净保证金
        NetMargin: f64,
        /// 产品间对锁保证金
        InterProdMargin: f64,
        /// 裸保证金
        SingleMargin: f64,
        /// 附加保证金
        AddOnMargin: f64,
        /// 交割月保证金
        DeliveryMargin: f64,
        /// 看涨期权最低风险
        CallOptionMinRisk: f64,
        /// 看跌期权最低风险
        PutOptionMinRisk: f64,
        /// 卖方期权最低风险
        OptionMinRisk: f64,
        /// 买方期权冲抵价值
        OptionValueOffset: f64,
        /// 卖方期权权利金
        OptionRoyalty: f64,
        /// 价值冲抵
        RealOptionValueOffset: f64,
        /// 保证金
        Margin: f64,
        /// 交易所保证金
        ExchMargin: f64,
    }
    /// 投资者产品SPBM明细查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProdSPBMDetailField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 品种代码
        ProdFamilyCode: String,
    }
    /// 组保交易参数设置
    #[derive(Debug, Clone, Default)]
    struct PortfTradeParamSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 新型组保算法
        Portfolio: u8,
        /// 撤单是否验资
        IsActionVerify: i32,
        /// 平仓是否验资
        IsCloseVerify: i32,
    }
    /// 投资者交易权限设置
    #[derive(Debug, Clone, Default)]
    struct InvestorTradingRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 交易权限
        InvstTradingRight: u8,
    }
    /// 质押配比参数
    #[derive(Debug, Clone, Default)]
    struct MortgageParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 质押配比系数
        MortgageBalance: f64,
        /// 开仓是否验证质押配比
        CheckMortgageRatio: i32,
    }
    /// 可提控制参数
    #[derive(Debug, Clone, Default)]
    struct WithDrawParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 参数代码
        WithDrawParamID: u8,
        /// 参数代码值
        WithDrawParamValue: Vec<u8>,
    }
    /// Thost终端用户功能权限
    #[derive(Debug, Clone, Default)]
    struct ThostUserFunctionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// Thost终端功能代码
        ThostFunctionCode: i32,
    }
    /// Thost终端用户功能权限查询
    #[derive(Debug, Clone, Default)]
    struct QryThostUserFunctionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
    }
    /// SPBM附加跨品种抵扣参数
    #[derive(Debug, Clone, Default)]
    struct SPBMAddOnInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 优先级
        SpreadId: i32,
        /// 品种间对锁仓附加费率折扣比例
        AddOnInterRateZ2: f64,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
    }
    /// SPBM附加跨品种抵扣参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPBMAddOnInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
    }
    /// 投资者商品组SPMM记录查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorCommoditySPMMMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 商品组代码
        CommodityID: String,
    }
    /// 投资者商品群SPMM记录查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorCommodityGroupSPMMMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 商品群代码
        CommodityGroupID: String,
    }
    /// SPMM合约参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPMMInstParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 合约代码
        InstrumentID: String,
    }
    /// SPMM产品参数查询
    #[derive(Debug, Clone, Default)]
    struct QrySPMMProductParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品代码
        ProductID: String,
    }
    /// 投资者商品组SPMM记录
    #[derive(Debug, Clone, Default)]
    struct InvestorCommoditySPMMMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 商品组代码
        CommodityID: String,
        /// 优惠仓位应收保证金
        MarginBeforeDiscount: f64,
        /// 不优惠仓位应收保证金
        MarginNoDiscount: f64,
        /// 多头实仓风险
        LongPosRisk: f64,
        /// 多头开仓冻结风险
        LongOpenFrozenRisk: f64,
        /// 多头被平冻结风险
        LongCloseFrozenRisk: f64,
        /// 空头实仓风险
        ShortPosRisk: f64,
        /// 空头开仓冻结风险
        ShortOpenFrozenRisk: f64,
        /// 空头被平冻结风险
        ShortCloseFrozenRisk: f64,
        /// SPMM品种内跨期优惠系数
        IntraCommodityRate: f64,
        /// SPMM期权优惠系数
        OptionDiscountRate: f64,
        /// 实仓对冲优惠金额
        PosDiscount: f64,
        /// 开仓报单对冲优惠金额
        OpenFrozenDiscount: f64,
        /// 品种风险净头
        NetRisk: f64,
        /// 平仓冻结保证金
        CloseFrozenMargin: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 手续费
        Commission: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 资金差额
        CashIn: f64,
        /// 行权冻结资金
        StrikeFrozenMargin: f64,
    }
    /// 投资者商品群SPMM记录
    #[derive(Debug, Clone, Default)]
    struct InvestorCommodityGroupSPMMMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 商品群代码
        CommodityGroupID: String,
        /// 优惠仓位应收保证金
        MarginBeforeDiscount: f64,
        /// 不优惠仓位应收保证金
        MarginNoDiscount: f64,
        /// 多头风险
        LongRisk: f64,
        /// 空头风险
        ShortRisk: f64,
        /// 商品群平仓冻结保证金
        CloseFrozenMargin: f64,
        /// SPMM跨品种优惠系数
        InterCommodityRate: f64,
        /// 商品群最小保证金比例
        MiniMarginRatio: f64,
        /// 投资者保证金和交易所保证金的比例
        AdjustRatio: f64,
        /// SPMM品种内优惠汇总
        IntraCommodityDiscount: f64,
        /// SPMM跨品种优惠
        InterCommodityDiscount: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 投资者保证金
        InvestorMargin: f64,
        /// 冻结的手续费
        FrozenCommission: f64,
        /// 手续费
        Commission: f64,
        /// 冻结的资金
        FrozenCash: f64,
        /// 资金差额
        CashIn: f64,
        /// 行权冻结资金
        StrikeFrozenMargin: f64,
    }
    /// SPMM合约参数
    #[derive(Debug, Clone, Default)]
    struct SPMMInstParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// SPMM合约保证金算法
        InstMarginCalID: u8,
        /// 商品组代码
        CommodityID: String,
        /// 商品群代码
        CommodityGroupID: String,
    }
    /// SPMM产品参数
    #[derive(Debug, Clone, Default)]
    struct SPMMProductParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 商品组代码
        CommodityID: String,
        /// 商品群代码
        CommodityGroupID: String,
    }
    /// 席位与交易中心对应关系维护查询
    #[derive(Debug, Clone, Default)]
    struct QryTraderAssignField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易员代码
        TraderID: String,
    }
    /// 席位与交易中心对应关系
    #[derive(Debug, Clone, Default)]
    struct TraderAssignField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 应用单元代码
        BrokerID: String,
        /// 交易所代码
        ExchangeID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 会员代码
        ParticipantID: String,
        /// 交易中心代码
        DRIdentityID: i32,
    }
    /// 投资者申报费阶梯收取设置
    #[derive(Debug, Clone, Default)]
    struct InvestorInfoCntSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 商品代码
        ProductID: String,
        /// 是否收取申报费
        IsCalInfoComm: i32,
        /// 是否限制信息量
        IsLimitInfoMax: i32,
        /// 信息量限制笔数
        InfoMaxLimit: i32,
    }
    /// RCAMS产品组合信息
    #[derive(Debug, Clone, Default)]
    struct RCAMSCombProductInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 商品组代码
        CombProductID: String,
        /// 商品群代码
        ProductGroupID: String,
    }
    /// RCAMS同合约风险对冲参数
    #[derive(Debug, Clone, Default)]
    struct RCAMSInstrParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 同合约风险对冲比率
        HedgeRate: f64,
    }
    /// RCAMS品种内风险对冲参数
    #[derive(Debug, Clone, Default)]
    struct RCAMSIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品组合代码
        CombProductID: String,
        /// 品种内对冲比率
        HedgeRate: f64,
    }
    /// RCAMS跨品种风险折抵参数
    #[derive(Debug, Clone, Default)]
    struct RCAMSInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 商品群代码
        ProductGroupID: String,
        /// 优先级
        Priority: i32,
        /// 折抵率
        CreditRate: f64,
        /// 产品组合代码1
        CombProduct1: String,
        /// 产品组合代码2
        CombProduct2: String,
    }
    /// RCAMS空头期权风险调整参数
    #[derive(Debug, Clone, Default)]
    struct RCAMSShortOptAdjustParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品组合代码
        CombProductID: String,
        /// 投套标志
        HedgeFlag: u8,
        /// 空头期权风险调整标准
        AdjustValue: f64,
    }
    /// RCAMS策略组合持仓
    #[derive(Debug, Clone, Default)]
    struct RCAMSInvestorCombPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 投套标志
        HedgeFlag: u8,
        /// 持仓多空方向
        PosiDirection: u8,
        /// 组合合约代码
        CombInstrumentID: String,
        /// 单腿编号
        LegID: i32,
        /// 交易所组合合约代码
        ExchangeInstID: String,
        /// 持仓量
        TotalAmt: i32,
        /// 交易所保证金
        ExchMargin: f64,
        /// 投资者保证金
        Margin: f64,
    }
    /// 投资者品种RCAMS保证金
    #[derive(Debug, Clone, Default)]
    struct InvestorProdRCAMSMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 产品组合代码
        CombProductID: String,
        /// 投套标志
        HedgeFlag: u8,
        /// 商品群代码
        ProductGroupID: String,
        /// 品种组合前风险
        RiskBeforeDiscount: f64,
        /// 同合约对冲风险
        IntraInstrRisk: f64,
        /// 品种买持仓风险
        BPosRisk: f64,
        /// 品种卖持仓风险
        SPosRisk: f64,
        /// 品种内对冲风险
        IntraProdRisk: f64,
        /// 品种净持仓风险
        NetRisk: f64,
        /// 品种间对冲风险
        InterProdRisk: f64,
        /// 空头期权风险调整
        ShortOptRiskAdj: f64,
        /// 空头期权权利金
        OptionRoyalty: f64,
        /// 大边组合平仓冻结保证金
        MMSACloseFrozenMargin: f64,
        /// 策略组合平仓/行权冻结保证金
        CloseCombFrozenMargin: f64,
        /// 平仓/行权冻结保证金
        CloseFrozenMargin: f64,
        /// 大边组合开仓冻结保证金
        MMSAOpenFrozenMargin: f64,
        /// 交割月期货开仓冻结保证金
        DeliveryOpenFrozenMargin: f64,
        /// 开仓冻结保证金
        OpenFrozenMargin: f64,
        /// 投资者冻结保证金
        UseFrozenMargin: f64,
        /// 大边组合交易所持仓保证金
        MMSAExchMargin: f64,
        /// 交割月期货交易所持仓保证金
        DeliveryExchMargin: f64,
        /// 策略组合交易所保证金
        CombExchMargin: f64,
        /// 交易所持仓保证金
        ExchMargin: f64,
        /// 投资者持仓保证金
        UseMargin: f64,
    }
    /// RCAMS产品组合信息查询
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSCombProductInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品代码
        ProductID: String,
        /// 商品组代码
        CombProductID: String,
        /// 商品群代码
        ProductGroupID: String,
    }
    /// RCAMS同合约风险对冲参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSInstrParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品代码
        ProductID: String,
    }
    /// RCAMS品种内风险对冲参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品组合代码
        CombProductID: String,
    }
    /// RCAMS跨品种风险折抵参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 商品群代码
        ProductGroupID: String,
        /// 产品组合代码1
        CombProduct1: String,
        /// 产品组合代码2
        CombProduct2: String,
    }
    /// RCAMS空头期权风险调整参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSShortOptAdjustParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 产品组合代码
        CombProductID: String,
    }
    /// RCAMS策略组合持仓查询
    #[derive(Debug, Clone, Default)]
    struct QryRCAMSInvestorCombPositionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 组合合约代码
        CombInstrumentID: String,
    }
    /// 投资者品种RCAMS保证金查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProdRCAMSMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 产品组合代码
        CombProductID: String,
        /// 商品群代码
        ProductGroupID: String,
    }
    /// RULE合约保证金参数
    #[derive(Debug, Clone, Default)]
    struct RULEInstrParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约类型
        InstrumentClass: u8,
        /// 标准合约
        StdInstrumentID: String,
        /// 投机买折算系数
        BSpecRatio: f64,
        /// 投机卖折算系数
        SSpecRatio: f64,
        /// 套保买折算系数
        BHedgeRatio: f64,
        /// 套保卖折算系数
        SHedgeRatio: f64,
        /// 买附加风险保证金
        BAddOnMargin: f64,
        /// 卖附加风险保证金
        SAddOnMargin: f64,
        /// 商品群号
        CommodityGroupID: i32,
    }
    /// RULE品种内对锁仓折扣参数
    #[derive(Debug, Clone, Default)]
    struct RULEIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 标准合约
        StdInstrumentID: String,
        /// 标准合约保证金
        StdInstrMargin: f64,
        /// 一般月份合约组合保证金系数
        UsualIntraRate: f64,
        /// 临近交割合约组合保证金系数
        DeliveryIntraRate: f64,
    }
    /// RULE跨品种抵扣参数
    #[derive(Debug, Clone, Default)]
    struct RULEInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 优先级
        SpreadId: i32,
        /// 品种间对锁仓费率折扣比例
        InterRate: f64,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
        /// 腿1比例系数
        Leg1PropFactor: i32,
        /// 腿2比例系数
        Leg2PropFactor: i32,
        /// 商品群号
        CommodityGroupID: i32,
        /// 商品群名称
        CommodityGroupName: String,
    }
    /// RULE合约保证金参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRULEInstrParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
    }
    /// RULE品种内对锁仓折扣参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRULEIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 品种代码
        ProdFamilyCode: String,
    }
    /// RULE跨品种抵扣参数查询
    #[derive(Debug, Clone, Default)]
    struct QryRULEInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
        /// 商品群号
        CommodityGroupID: i32,
    }
    /// 投资者产品RULE保证金
    #[derive(Debug, Clone, Default)]
    struct InvestorProdRULEMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 合约类型
        InstrumentClass: u8,
        /// 商品群号
        CommodityGroupID: i32,
        /// 买标准持仓
        BStdPosition: f64,
        /// 卖标准持仓
        SStdPosition: f64,
        /// 买标准开仓冻结
        BStdOpenFrozen: f64,
        /// 卖标准开仓冻结
        SStdOpenFrozen: f64,
        /// 买标准平仓冻结
        BStdCloseFrozen: f64,
        /// 卖标准平仓冻结
        SStdCloseFrozen: f64,
        /// 品种内对冲标准持仓
        IntraProdStdPosition: f64,
        /// 品种内单腿标准持仓
        NetStdPosition: f64,
        /// 品种间对冲标准持仓
        InterProdStdPosition: f64,
        /// 单腿标准持仓
        SingleStdPosition: f64,
        /// 品种内对锁保证金
        IntraProdMargin: f64,
        /// 品种间对锁保证金
        InterProdMargin: f64,
        /// 跨品种单腿保证金
        SingleMargin: f64,
        /// 非组合合约保证金
        NonCombMargin: f64,
        /// 附加保证金
        AddOnMargin: f64,
        /// 交易所保证金
        ExchMargin: f64,
        /// 附加冻结保证金
        AddOnFrozenMargin: f64,
        /// 开仓冻结保证金
        OpenFrozenMargin: f64,
        /// 平仓冻结保证金
        CloseFrozenMargin: f64,
        /// 品种保证金
        Margin: f64,
        /// 冻结保证金
        FrozenMargin: f64,
    }
    /// 投资者产品RULE保证金查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorProdRULEMarginField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 商品群号
        CommodityGroupID: i32,
    }
    /// 风险结算追平SPBM组合保证金套餐
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMPortfDefinitionField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 组合保证金套餐代码
        PortfolioDefID: i32,
        /// 品种代码
        ProdFamilyCode: String,
        /// 是否启用SPBM
        IsSPBM: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平投资者SPBM套餐选择
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMInvstPortfDefField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组合保证金套餐代码
        PortfolioDefID: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPBM期货合约保证金参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMFutureParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 期货合约因子
        Cvf: i32,
        /// 阶段标识
        TimeRange: u8,
        /// 品种保证金标准
        MarginRate: f64,
        /// 期货合约内部对锁仓费率折扣比例
        LockRateX: f64,
        /// 提高保证金标准
        AddOnRate: f64,
        /// 昨结算价
        PreSettlementPrice: f64,
        /// 期货合约内部对锁仓附加费率折扣比例
        AddOnLockRateX2: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPBM期权合约保证金参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMOptionParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 期权合约因子
        Cvf: i32,
        /// 期权冲抵价格
        DownPrice: f64,
        /// Delta值
        Delta: f64,
        /// 卖方期权风险转换最低值
        SlimiDelta: f64,
        /// 昨结算价
        PreSettlementPrice: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPBM品种内对锁仓折扣参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 品种内合约间对锁仓费率折扣比例
        IntraRateY: f64,
        /// 品种内合约间对锁仓附加费率折扣比例
        AddOnIntraRateY2: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPBM跨品种抵扣参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 优先级
        SpreadId: i32,
        /// 品种间对锁仓费率折扣比例
        InterRateZ: f64,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPBM附加跨品种抵扣参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPBMAddOnInterParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 优先级
        SpreadId: i32,
        /// 品种间对锁仓附加费率折扣比例
        AddOnInterRateZ2: f64,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPMM合约参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPMMInstParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// SPMM合约保证金算法
        InstMarginCalID: u8,
        /// 商品组代码
        CommodityID: String,
        /// 商品群代码
        CommodityGroupID: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPMM产品相关参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPMMProductParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 商品组代码
        CommodityID: String,
        /// 商品群代码
        CommodityGroupID: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平投资者SPMM模板选择
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaInvestorSPMMModelField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// SPMM模板ID
        SPMMModelID: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平SPMM模板参数设置
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaSPMMModelParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// SPMM模板ID
        SPMMModelID: String,
        /// 商品群代码
        CommodityGroupID: String,
        /// SPMM品种内跨期优惠系数
        IntraCommodityRate: f64,
        /// SPMM品种间优惠系数
        InterCommodityRate: f64,
        /// SPMM期权优惠系数
        OptionDiscountRate: f64,
        /// 商品群最小保证金比例
        MiniMarginRatio: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS产品组合信息
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSCombProdInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 商品组代码
        CombProductID: String,
        /// 商品群代码
        ProductGroupID: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS同合约风险对冲参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSInstrParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品代码
        ProductID: String,
        /// 同合约风险对冲比率
        HedgeRate: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS品种内风险对冲参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品组合代码
        CombProductID: String,
        /// 品种内对冲比率
        HedgeRate: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS跨品种风险折抵参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 商品群代码
        ProductGroupID: String,
        /// 优先级
        Priority: i32,
        /// 折抵率
        CreditRate: f64,
        /// 产品组合代码1
        CombProduct1: String,
        /// 产品组合代码2
        CombProduct2: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS空头期权风险调整参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSSOptAdjParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 产品组合代码
        CombProductID: String,
        /// 投套标志
        HedgeFlag: u8,
        /// 空头期权风险调整标准
        AdjustValue: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS策略组合规则明细
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSCombRuleDtlField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 策略产品
        ProdGroup: Vec<u8>,
        /// 策略id
        RuleId: Vec<u8>,
        /// 优先级
        Priority: i32,
        /// 投套标志
        HedgeFlag: u8,
        /// 组合保证金标准
        CombMargin: f64,
        /// 交易所组合合约代码
        ExchangeInstID: String,
        /// 单腿编号
        LegID: i32,
        /// 单腿合约代码
        LegInstrumentID: String,
        /// 买卖方向
        Direction: u8,
        /// 单腿乘数
        LegMultiple: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RCAMS策略组合持仓
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRCAMSInvstCombPosField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 投套标志
        HedgeFlag: u8,
        /// 持仓多空方向
        PosiDirection: u8,
        /// 组合合约代码
        CombInstrumentID: String,
        /// 单腿编号
        LegID: i32,
        /// 交易所组合合约代码
        ExchangeInstID: String,
        /// 持仓量
        TotalAmt: i32,
        /// 交易所保证金
        ExchMargin: f64,
        /// 投资者保证金
        Margin: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RULE合约保证金参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRULEInstrParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 合约代码
        InstrumentID: String,
        /// 合约类型
        InstrumentClass: u8,
        /// 标准合约
        StdInstrumentID: String,
        /// 投机买折算系数
        BSpecRatio: f64,
        /// 投机卖折算系数
        SSpecRatio: f64,
        /// 套保买折算系数
        BHedgeRatio: f64,
        /// 套保卖折算系数
        SHedgeRatio: f64,
        /// 买附加风险保证金
        BAddOnMargin: f64,
        /// 卖附加风险保证金
        SAddOnMargin: f64,
        /// 商品群号
        CommodityGroupID: i32,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RULE品种内对锁仓折扣参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRULEIntraParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 品种代码
        ProdFamilyCode: String,
        /// 标准合约
        StdInstrumentID: String,
        /// 标准合约保证金
        StdInstrMargin: f64,
        /// 一般月份合约组合保证金系数
        UsualIntraRate: f64,
        /// 临近交割合约组合保证金系数
        DeliveryIntraRate: f64,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 风险结算追平RULE跨品种抵扣参数
    #[derive(Debug, Clone, Default)]
    struct SyncDeltaRULEInterParameterField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易日
        TradingDay: String,
        /// 交易所代码
        ExchangeID: String,
        /// 优先级
        SpreadId: i32,
        /// 品种间对锁仓费率折扣比例
        InterRate: f64,
        /// 第一腿构成品种
        Leg1ProdFamilyCode: String,
        /// 第二腿构成品种
        Leg2ProdFamilyCode: String,
        /// 腿1比例系数
        Leg1PropFactor: i32,
        /// 腿2比例系数
        Leg2PropFactor: i32,
        /// 商品群号
        CommodityGroupID: i32,
        /// 商品群名称
        CommodityGroupName: String,
        /// 操作标志
        ActionDirection: u8,
        /// 追平序号
        SyncDeltaSequenceNo: i32,
    }
    /// 服务地址参数
    #[derive(Debug, Clone, Default)]
    struct IpAddrParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 服务地址
        Address: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 交易中心名称
        DRIdentityName: String,
        /// 交易地址OR行情地址
        AddrSrvMode: u8,
        /// 地址版本
        AddrVer: u8,
        /// 服务地址编号
        AddrNo: i32,
        /// 服务地址名称
        AddrName: String,
        /// 是否是国密地址
        IsSM: i32,
        /// 是否是内网地址
        IsLocalAddr: i32,
        /// 地址补充信息
        Remark: String,
        /// 站点
        Site: Vec<u8>,
        /// 网络运营商
        NetOperator: Vec<u8>,
    }
    /// 服务地址参数查询
    #[derive(Debug, Clone, Default)]
    struct QryIpAddrParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 服务地址参数
    #[derive(Debug, Clone, Default)]
    struct TGIpAddrParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 服务地址
        Address: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 交易中心名称
        DRIdentityName: String,
        /// 交易地址OR行情地址
        AddrSrvMode: u8,
        /// 地址版本
        AddrVer: u8,
        /// 服务地址编号
        AddrNo: i32,
        /// 服务地址名称
        AddrName: String,
        /// 是否是国密地址
        IsSM: i32,
        /// 是否是内网地址
        IsLocalAddr: i32,
        /// 地址补充信息
        Remark: String,
        /// 站点
        Site: Vec<u8>,
        /// 网络运营商
        NetOperator: Vec<u8>,
    }
    /// 服务地址参数查询
    #[derive(Debug, Clone, Default)]
    struct QryTGIpAddrParamField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// App代码
        AppID: String,
    }
    /// TGate会话查询状态
    #[derive(Debug, Clone, Default)]
    struct TGSessionQryStatusField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 最近30s的查询频率
        LastQryFreq: i32,
        /// 查询状态
        QryStatus: u8,
    }
    /// 内网地址配置
    #[derive(Debug, Clone, Default)]
    struct LocalAddrConfigField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 对端地址
        PeerAddr: String,
        /// 子网掩码
        NetMask: Vec<u8>,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 内网服务地址
        LocalAddress: String,
    }
    /// 内网地址配置查询
    #[derive(Debug, Clone, Default)]
    struct QryLocalAddrConfigField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 次席查询银行资金帐户信息请求
    #[derive(Debug, Clone, Default)]
    struct ReqQueryBankAccountBySecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 长客户姓名
        LongCustomerName: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 次中心发起转账期货公司流水号
        SecFutureSerial: i32,
    }
    /// 次席查询银行资金帐户信息回报
    #[derive(Debug, Clone, Default)]
    struct RspQueryBankAccountBySecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 银行可用金额
        BankUseAmount: f64,
        /// 银行可取金额
        BankFetchAmount: f64,
        /// 长客户姓名
        LongCustomerName: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 次中心发起转账期货公司流水号
        SecFutureSerial: i32,
    }
    /// 次中心发起的转帐交易
    #[derive(Debug, Clone, Default)]
    struct ReqTransferBySecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 期货可取金额
        FutureFetchAmount: f64,
        /// 费用支付标志
        FeePayFlag: u8,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 发送方给接收方的消息
        Message: Vec<u8>,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 转账交易状态
        TransferStatus: u8,
        /// 长客户姓名
        LongCustomerName: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 次中心发起转账期货公司流水号
        SecFutureSerial: i32,
    }
    /// 次中心发起的转帐交易回报
    #[derive(Debug, Clone, Default)]
    struct RspTransferBySecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 安装编号
        InstallID: i32,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 转帐金额
        TradeAmount: f64,
        /// 期货可取金额
        FutureFetchAmount: f64,
        /// 费用支付标志
        FeePayFlag: u8,
        /// 应收客户费用
        CustFee: f64,
        /// 应收期货公司费用
        BrokerFee: f64,
        /// 发送方给接收方的消息
        Message: Vec<u8>,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 转账交易状态
        TransferStatus: u8,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 次中心发起转账期货公司流水号
        SecFutureSerial: i32,
    }
    /// 查询银行资金帐户信息通知 要发往次席
    #[derive(Debug, Clone, Default)]
    struct NotifyQueryFutureAccountBySecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 业务功能码
        TradeCode: String,
        /// 银行代码
        BankID: String,
        /// 银行分支机构代码
        BankBranchID: String,
        /// 期商代码
        BrokerID: String,
        /// 期商分支机构代码
        BrokerBranchID: String,
        /// 交易日期
        TradeDate: String,
        /// 交易时间
        TradeTime: String,
        /// 银行流水号
        BankSerial: String,
        /// 交易系统日期
        TradingDay: String,
        /// 银期平台消息流水号
        PlateSerial: i32,
        /// 最后分片标志
        LastFragment: u8,
        /// 会话号
        SessionID: i32,
        /// 客户姓名
        CustomerName: String,
        /// 证件类型
        IdCardType: u8,
        /// 证件号码
        IdentifiedCardNo: String,
        /// 客户类型
        CustType: u8,
        /// 银行帐号
        BankAccount: String,
        /// 银行密码
        BankPassWord: String,
        /// 投资者帐号
        AccountID: String,
        /// 期货密码
        Password: String,
        /// 期货公司流水号
        FutureSerial: i32,
        /// 安装编号
        InstallID: i32,
        /// 用户标识
        UserID: String,
        /// 验证客户证件号码标志
        VerifyCertNoFlag: u8,
        /// 币种代码
        CurrencyID: String,
        /// 摘要
        Digest: Vec<u8>,
        /// 银行帐号类型
        BankAccType: u8,
        /// 渠道标志
        DeviceID: String,
        /// 期货单位帐号类型
        BankSecuAccType: u8,
        /// 期货公司银行编码
        BrokerIDByBank: Vec<u8>,
        /// 期货单位帐号
        BankSecuAcc: Vec<u8>,
        /// 银行密码标志
        BankPwdFlag: u8,
        /// 期货资金密码核对标志
        SecuPwdFlag: u8,
        /// 交易柜员
        OperNo: String,
        /// 请求编号
        RequestID: i32,
        /// 交易ID
        TID: i32,
        /// 银行可用金额
        BankUseAmount: f64,
        /// 银行可取金额
        BankFetchAmount: f64,
        /// 错误代码
        ErrorID: i32,
        /// 错误信息
        ErrorMsg: String,
        /// 长客户姓名
        LongCustomerName: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// 次中心发起转账期货公司流水号
        SecFutureSerial: i32,
    }
    /// 退出紧急状态参数
    #[derive(Debug, Clone, Default)]
    struct ExitEmergencyField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 新组保保证金系数投资者模板对应关系
    #[derive(Debug, Clone, Default)]
    struct InvestorPortfMarginModelField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 保证金系数模板
        MarginModelID: String,
    }
    /// 投资者新组保设置
    #[derive(Debug, Clone, Default)]
    struct InvestorPortfSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者编号
        InvestorID: String,
        /// 投机套保标志
        HedgeFlag: u8,
        /// 是否开启新组保
        UsePortf: i32,
    }
    /// 投资者新组保设置查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorPortfSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者编号
        InvestorID: String,
    }
    /// 来自次席的用户口令变更
    #[derive(Debug, Clone, Default)]
    struct UserPasswordUpdateFromSecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 原来的口令
        OldPassword: String,
        /// 新的口令
        NewPassword: String,
        /// 次席的交易中心代码
        FromSec: i32,
    }
    /// 来自次席的结算结果确认
    #[derive(Debug, Clone, Default)]
    struct SettlementInfoConfirmFromSecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 确认日期
        ConfirmDate: String,
        /// 确认时间
        ConfirmTime: String,
        /// 次席的交易中心代码
        FromSec: i32,
    }
    /// 来自次席的资金账户口令变更
    #[derive(Debug, Clone, Default)]
    struct TradingAccountPasswordUpdateFromSecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者帐号
        AccountID: String,
        /// 原来的口令
        OldPassword: String,
        /// 新的口令
        NewPassword: String,
        /// 币种代码
        CurrencyID: String,
        /// 次席的交易中心代码
        FromSec: i32,
    }
    /// 风控禁止的合约交易权限
    #[derive(Debug, Clone, Default)]
    struct RiskForbiddenRightField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者编号
        InvestorID: String,
        /// 合约/产品代码
        InstrumentID: String,
        /// 用户代码
        UserID: String,
    }
    /// 投资者申报费阶梯收取记录
    #[derive(Debug, Clone, Default)]
    struct InvestorInfoCommRecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 交易所代码
        ExchangeID: String,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 商品代码
        InstrumentID: String,
        /// 报单总笔数
        OrderCount: i32,
        /// 撤单总笔数
        OrderActionCount: i32,
        /// 询价总次数
        ForQuoteCnt: i32,
        /// 申报费
        InfoComm: f64,
        /// 是否期权系列
        IsOptSeries: i32,
        /// 品种代码
        ProductID: String,
        /// 信息量总量
        InfoCnt: i32,
    }
    /// 投资者申报费阶梯收取记录查询
    #[derive(Debug, Clone, Default)]
    struct QryInvestorInfoCommRecField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 投资者代码
        InvestorID: String,
        /// 商品代码
        InstrumentID: String,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 组合腿信息
    #[derive(Debug, Clone, Default)]
    struct CombLegField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 组合合约代码
        CombInstrumentID: String,
        /// 单腿编号
        LegID: i32,
        /// 单腿合约代码
        LegInstrumentID: String,
        /// 买卖方向
        Direction: u8,
        /// 单腿乘数
        LegMultiple: i32,
        /// 派生层数
        ImplyLevel: i32,
    }
    /// 组合腿信息查询
    #[derive(Debug, Clone, Default)]
    struct QryCombLegField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 单腿合约代码
        LegInstrumentID: String,
    }
    /// 输入的对冲设置
    #[derive(Debug, Clone, Default)]
    struct InputOffsetSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 标的期货合约代码
        UnderlyingInstrID: String,
        /// 产品代码
        ProductID: String,
        /// 对冲类型
        OffsetType: u8,
        /// 申请对冲的合约数量
        Volume: i32,
        /// 是否对冲
        IsOffset: i32,
        /// 请求编号
        RequestID: i32,
        /// 用户代码
        UserID: String,
        /// 交易所代码
        ExchangeID: String,
        /// IP地址
        IPAddress: String,
        /// Mac地址
        MacAddress: String,
    }
    /// 对冲设置
    #[derive(Debug, Clone, Default)]
    struct OffsetSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 标的期货合约代码
        UnderlyingInstrID: String,
        /// 产品代码
        ProductID: String,
        /// 对冲类型
        OffsetType: u8,
        /// 申请对冲的合约数量
        Volume: i32,
        /// 是否对冲
        IsOffset: i32,
        /// 请求编号
        RequestID: i32,
        /// 用户代码
        UserID: String,
        /// 交易所代码
        ExchangeID: String,
        /// IP地址
        IPAddress: String,
        /// Mac地址
        MacAddress: String,
        /// 交易所合约代码
        ExchangeInstID: String,
        /// 交易所期权系列号
        ExchangeSerialNo: Vec<u8>,
        /// 交易所产品代码
        ExchangeProductID: String,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 对冲提交状态
        OrderSubmitStatus: u8,
        /// 交易日
        TradingDay: String,
        /// 结算编号
        SettlementID: i32,
        /// 报单日期
        InsertDate: String,
        /// 插入时间
        InsertTime: String,
        /// 撤销时间
        CancelTime: String,
        /// 对冲设置结果
        ExecResult: u8,
        /// 序号
        SequenceNo: i32,
        /// 前置编号
        FrontID: i32,
        /// 会话编号
        SessionID: i32,
        /// 状态信息
        StatusMsg: String,
        /// 操作用户代码
        ActiveUserID: String,
        /// 经纪公司报单编号
        BrokerOffsetSettingSeq: i32,
        /// 申请来源
        ApplySrc: u8,
    }
    /// 撤销对冲设置
    #[derive(Debug, Clone, Default)]
    struct CancelOffsetSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 合约代码
        InstrumentID: String,
        /// 标的期货合约代码
        UnderlyingInstrID: String,
        /// 产品代码
        ProductID: String,
        /// 对冲类型
        OffsetType: u8,
        /// 申请对冲的合约数量
        Volume: i32,
        /// 是否对冲
        IsOffset: i32,
        /// 请求编号
        RequestID: i32,
        /// 用户代码
        UserID: String,
        /// 交易所代码
        ExchangeID: String,
        /// IP地址
        IPAddress: String,
        /// Mac地址
        MacAddress: String,
        /// 交易所合约代码
        ExchangeInstID: String,
        /// 交易所期权系列号
        ExchangeSerialNo: Vec<u8>,
        /// 交易所产品代码
        ExchangeProductID: String,
        /// 交易所交易员代码
        TraderID: String,
        /// 安装编号
        InstallID: i32,
        /// 会员代码
        ParticipantID: String,
        /// 客户代码
        ClientID: String,
        /// 报单操作状态
        OrderActionStatus: u8,
        /// 状态信息
        StatusMsg: String,
        /// 操作本地编号
        ActionLocalID: String,
        /// 操作日期
        ActionDate: String,
        /// 操作时间
        ActionTime: String,
    }
    /// 查询对冲设置
    #[derive(Debug, Clone, Default)]
    struct QryOffsetSettingField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 产品代码
        ProductID: String,
        /// 对冲类型
        OffsetType: u8,
    }
    /// 服务地址和AppID的关系
    #[derive(Debug, Clone, Default)]
    struct AddrAppIDRelationField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 服务地址
        Address: String,
        /// 交易中心代码
        DRIdentityID: i32,
        /// App代码
        AppID: String,
    }
    /// 服务地址和AppID的关系查询
    #[derive(Debug, Clone, Default)]
    struct QryAddrAppIDRelationField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 微信小程序等用户系统信息
    #[derive(Debug, Clone, Default)]
    struct WechatUserSystemInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 微信小程序等用户端系统内部信息长度
        WechatCltSysInfoLen: i32,
        /// 微信小程序等用户端系统内部信息
        WechatCltSysInfo: String,
        /// 终端IP端口
        ClientIPPort: i32,
        /// 登录成功时间
        ClientLoginTime: String,
        /// App代码
        ClientAppID: String,
        /// 用户公网IP
        ClientPublicIP: String,
        /// 客户登录备注2
        ClientLoginRemark: String,
    }
    /// 投资者预留信息
    #[derive(Debug, Clone, Default)]
    struct InvestorReserveInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 用户代码
        UserID: String,
        /// 预留信息
        ReserveInfo: String,
    }
    /// 查询组织架构投资者对应关系
    #[derive(Debug, Clone, Default)]
    struct QryInvestorDepartmentFlatField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 组织架构投资者对应关系
    #[derive(Debug, Clone, Default)]
    struct InvestorDepartmentFlatField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
        /// 投资者代码
        InvestorID: String,
        /// 组织架构代码
        DepartmentID: String,
    }
    /// 查询操作员组织架构关系
    #[derive(Debug, Clone, Default)]
    struct QryDepartmentUserField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 经纪公司代码
        BrokerID: String,
    }
    /// 前置信息
    #[derive(Debug, Clone, Default)]
    struct FrontInfoField {
        /// C++ 端传入的整体 Field 是否为 `nullptr`
        is_null: bool,
        /// 前置地址
        FrontAddr: String,
        /// 查询流控
        QryFreq: i32,
        /// FTD流控
        FTDPkgFreq: i32,
    }
}