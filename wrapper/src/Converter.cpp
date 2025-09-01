#include "ctp-rs/wrapper/include/Converter.h"

#ifdef _WIN32

#include <Windows.h>

std::string Converter::Gb2312ToUtf8(const char *src_str)
{
    int len = MultiByteToWideChar(936, 0, src_str, -1, NULL, 0);
    wchar_t *wstr = new wchar_t[len + 1];
    memset(wstr, 0, len + 1);
    MultiByteToWideChar(936, 0, src_str, -1, wstr, len);
    len = WideCharToMultiByte(CP_UTF8, 0, wstr, -1, NULL, 0, NULL, NULL);
    char *str = new char[len + 1];
    memset(str, 0, len + 1);
    WideCharToMultiByte(CP_UTF8, 0, wstr, -1, str, len, NULL, NULL);
    std::string strTemp = str;
    if (wstr)
        delete[] wstr;
    if (str)
        delete[] str;
    return strTemp;
}

#else

#include <iconv.h>

#define MAX_BUF 3072

void gb2312_to_utf8(const char *src, char *dst, int len)
{
    int ret = 0;
    size_t inlen = strlen(src) + 1;
    size_t outlen = len;

    // duanqn: The iconv function in Linux requires non-const char *
    // So we need to copy the source string
    char *inbuf = (char *)malloc(len);
    char *inbuf_hold = inbuf; // iconv may change the address of inbuf
                              // so we use another pointer to keep the address
    memcpy(inbuf, src, len);

    char *outbuf2 = NULL;
    char *outbuf = dst;
    iconv_t cd;

    // starkwong: if src==dst, the string will become invalid during conversion since UTF-8 is 3 chars in Chinese but GBK is mostly 2 chars
    if (src == dst)
    {
        outbuf2 = (char *)malloc(len);
        memset(outbuf2, 0, len);
        outbuf = outbuf2;
    }

    cd = iconv_open("UTF-8", "GB2312");
    if (cd != (iconv_t)-1)
    {
        ret = iconv(cd, &inbuf, &inlen, &outbuf, &outlen);

        if (outbuf2 != NULL)
        {
            strcpy(dst, outbuf2);
            free(outbuf2);
        }

        iconv_close(cd);
    }

    if (ret != 0)
    {
        printf("iconv failed, buf: [0x%02x", (unsigned char)*inbuf_hold);
        for (char *p = inbuf_hold + 1; p && *p; p++)
            printf(", 0x%02x", (unsigned char)*p);
        printf("], err: %s\n", strerror(errno));
        dst[0] = '\0';
    }
    free(inbuf_hold); // Don't pass in inbuf as it may have been modified
}

std::string Converter::Gb2312ToUtf8(const char *src_str)
{
    char dst_str[MAX_BUF] = {0};
    gb2312_to_utf8(src_str, dst_str, MAX_BUF);
    return std::string(dst_str);
}

#endif

rust::String Converter::Gb2312ToRustString(const char *src_str)
{
    if (src_str == nullptr)
        return rust::String("");
    return rust::String(Converter::Gb2312ToUtf8(src_str));
}

CThostFtdcDisseminationField Converter::DisseminationFieldToCpp(DisseminationField x) {
    CThostFtdcDisseminationField y;
    memset(&y, 0, sizeof(y));
    y.SequenceSeries = x.SequenceSeries;
    y.SequenceNo = x.SequenceNo;
    return y;
}

DisseminationField Converter::CThostFtdcDisseminationFieldToRust(CThostFtdcDisseminationField* x) {
    if (x == nullptr)
        return DisseminationField{.is_null = true};
    DisseminationField y{};
    y.SequenceSeries = x->SequenceSeries;
    y.SequenceNo = x->SequenceNo;
    return y;
}

CThostFtdcReqUserLoginField Converter::ReqUserLoginFieldToCpp(ReqUserLoginField x) {
    CThostFtdcReqUserLoginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.OneTimePassword, x.OneTimePassword.c_str());
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientIPAddress, x.ClientIPAddress.c_str());
    return y;
}

ReqUserLoginField Converter::CThostFtdcReqUserLoginFieldToRust(CThostFtdcReqUserLoginField* x) {
    if (x == nullptr)
        return ReqUserLoginField{.is_null = true};
    ReqUserLoginField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.OneTimePassword = Converter::Gb2312ToRustString(x->OneTimePassword);
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientIPAddress = Converter::Gb2312ToRustString(x->ClientIPAddress);
    return y;
}

CThostFtdcRspUserLoginField Converter::RspUserLoginFieldToCpp(RspUserLoginField x) {
    CThostFtdcRspUserLoginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.LoginTime, x.LoginTime.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.SystemName, x.SystemName.c_str());
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.MaxOrderRef, x.MaxOrderRef.c_str());
    strcpy(y.SHFETime, x.SHFETime.c_str());
    strcpy(y.DCETime, x.DCETime.c_str());
    strcpy(y.CZCETime, x.CZCETime.c_str());
    strcpy(y.FFEXTime, x.FFEXTime.c_str());
    strcpy(y.INETime, x.INETime.c_str());
    strcpy(y.SysVersion, x.SysVersion.c_str());
    strcpy(y.GFEXTime, x.GFEXTime.c_str());
    y.LoginDRIdentityID = x.LoginDRIdentityID;
    y.UserDRIdentityID = x.UserDRIdentityID;
    strcpy(y.LastLoginTime, x.LastLoginTime.c_str());
    strcpy(y.ReserveInfo, x.ReserveInfo.c_str());
    return y;
}

RspUserLoginField Converter::CThostFtdcRspUserLoginFieldToRust(CThostFtdcRspUserLoginField* x) {
    if (x == nullptr)
        return RspUserLoginField{.is_null = true};
    RspUserLoginField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.LoginTime = Converter::Gb2312ToRustString(x->LoginTime);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.SystemName = Converter::Gb2312ToRustString(x->SystemName);
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.MaxOrderRef = Converter::Gb2312ToRustString(x->MaxOrderRef);
    y.SHFETime = Converter::Gb2312ToRustString(x->SHFETime);
    y.DCETime = Converter::Gb2312ToRustString(x->DCETime);
    y.CZCETime = Converter::Gb2312ToRustString(x->CZCETime);
    y.FFEXTime = Converter::Gb2312ToRustString(x->FFEXTime);
    y.INETime = Converter::Gb2312ToRustString(x->INETime);
    y.SysVersion = Converter::Gb2312ToRustString(x->SysVersion);
    y.GFEXTime = Converter::Gb2312ToRustString(x->GFEXTime);
    y.LoginDRIdentityID = x->LoginDRIdentityID;
    y.UserDRIdentityID = x->UserDRIdentityID;
    y.LastLoginTime = Converter::Gb2312ToRustString(x->LastLoginTime);
    y.ReserveInfo = Converter::Gb2312ToRustString(x->ReserveInfo);
    return y;
}

CThostFtdcUserLogoutField Converter::UserLogoutFieldToCpp(UserLogoutField x) {
    CThostFtdcUserLogoutField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

UserLogoutField Converter::CThostFtdcUserLogoutFieldToRust(CThostFtdcUserLogoutField* x) {
    if (x == nullptr)
        return UserLogoutField{.is_null = true};
    UserLogoutField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcForceUserLogoutField Converter::ForceUserLogoutFieldToCpp(ForceUserLogoutField x) {
    CThostFtdcForceUserLogoutField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

ForceUserLogoutField Converter::CThostFtdcForceUserLogoutFieldToRust(CThostFtdcForceUserLogoutField* x) {
    if (x == nullptr)
        return ForceUserLogoutField{.is_null = true};
    ForceUserLogoutField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcReqAuthenticateField Converter::ReqAuthenticateFieldToCpp(ReqAuthenticateField x) {
    CThostFtdcReqAuthenticateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.AuthCode, x.AuthCode.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    return y;
}

ReqAuthenticateField Converter::CThostFtdcReqAuthenticateFieldToRust(CThostFtdcReqAuthenticateField* x) {
    if (x == nullptr)
        return ReqAuthenticateField{.is_null = true};
    ReqAuthenticateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.AuthCode = Converter::Gb2312ToRustString(x->AuthCode);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    return y;
}

CThostFtdcRspAuthenticateField Converter::RspAuthenticateFieldToCpp(RspAuthenticateField x) {
    CThostFtdcRspAuthenticateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    y.AppType = x.AppType;
    return y;
}

RspAuthenticateField Converter::CThostFtdcRspAuthenticateFieldToRust(CThostFtdcRspAuthenticateField* x) {
    if (x == nullptr)
        return RspAuthenticateField{.is_null = true};
    RspAuthenticateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    y.AppType = x->AppType;
    return y;
}

CThostFtdcAuthenticationInfoField Converter::AuthenticationInfoFieldToCpp(AuthenticationInfoField x) {
    CThostFtdcAuthenticationInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.AuthInfo, x.AuthInfo.c_str());
    y.IsResult = x.IsResult;
    strcpy(y.AppID, x.AppID.c_str());
    y.AppType = x.AppType;
    strcpy(y.ClientIPAddress, x.ClientIPAddress.c_str());
    return y;
}

AuthenticationInfoField Converter::CThostFtdcAuthenticationInfoFieldToRust(CThostFtdcAuthenticationInfoField* x) {
    if (x == nullptr)
        return AuthenticationInfoField{.is_null = true};
    AuthenticationInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.AuthInfo = Converter::Gb2312ToRustString(x->AuthInfo);
    y.IsResult = x->IsResult;
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    y.AppType = x->AppType;
    y.ClientIPAddress = Converter::Gb2312ToRustString(x->ClientIPAddress);
    return y;
}

CThostFtdcRspUserLogin2Field Converter::RspUserLogin2FieldToCpp(RspUserLogin2Field x) {
    CThostFtdcRspUserLogin2Field y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.LoginTime, x.LoginTime.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.SystemName, x.SystemName.c_str());
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.MaxOrderRef, x.MaxOrderRef.c_str());
    strcpy(y.SHFETime, x.SHFETime.c_str());
    strcpy(y.DCETime, x.DCETime.c_str());
    strcpy(y.CZCETime, x.CZCETime.c_str());
    strcpy(y.FFEXTime, x.FFEXTime.c_str());
    strcpy(y.INETime, x.INETime.c_str());
    memcpy(y.RandomString, x.RandomString.data(), x.RandomString.size() * sizeof(uint8_t));
    return y;
}

RspUserLogin2Field Converter::CThostFtdcRspUserLogin2FieldToRust(CThostFtdcRspUserLogin2Field* x) {
    if (x == nullptr)
        return RspUserLogin2Field{.is_null = true};
    RspUserLogin2Field y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.LoginTime = Converter::Gb2312ToRustString(x->LoginTime);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.SystemName = Converter::Gb2312ToRustString(x->SystemName);
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.MaxOrderRef = Converter::Gb2312ToRustString(x->MaxOrderRef);
    y.SHFETime = Converter::Gb2312ToRustString(x->SHFETime);
    y.DCETime = Converter::Gb2312ToRustString(x->DCETime);
    y.CZCETime = Converter::Gb2312ToRustString(x->CZCETime);
    y.FFEXTime = Converter::Gb2312ToRustString(x->FFEXTime);
    y.INETime = Converter::Gb2312ToRustString(x->INETime);
    for (int i = 0; i < 17; i++)
        y.RandomString.push_back(x->RandomString[i]);
    return y;
}

CThostFtdcTransferHeaderField Converter::TransferHeaderFieldToCpp(TransferHeaderField x) {
    CThostFtdcTransferHeaderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.Version, x.Version.c_str());
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.TradeSerial, x.TradeSerial.c_str());
    strcpy(y.FutureID, x.FutureID.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBrchID, x.BankBrchID.c_str());
    strcpy(y.OperNo, x.OperNo.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.RecordNum, x.RecordNum.data(), x.RecordNum.size() * sizeof(uint8_t));
    y.SessionID = x.SessionID;
    y.RequestID = x.RequestID;
    return y;
}

TransferHeaderField Converter::CThostFtdcTransferHeaderFieldToRust(CThostFtdcTransferHeaderField* x) {
    if (x == nullptr)
        return TransferHeaderField{.is_null = true};
    TransferHeaderField y{};
    y.Version = Converter::Gb2312ToRustString(x->Version);
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.TradeSerial = Converter::Gb2312ToRustString(x->TradeSerial);
    y.FutureID = Converter::Gb2312ToRustString(x->FutureID);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBrchID = Converter::Gb2312ToRustString(x->BankBrchID);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 7; i++)
        y.RecordNum.push_back(x->RecordNum[i]);
    y.SessionID = x->SessionID;
    y.RequestID = x->RequestID;
    return y;
}

CThostFtdcTransferBankToFutureReqField Converter::TransferBankToFutureReqFieldToCpp(TransferBankToFutureReqField x) {
    CThostFtdcTransferBankToFutureReqField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.FuturePwdFlag = x.FuturePwdFlag;
    strcpy(y.FutureAccPwd, x.FutureAccPwd.c_str());
    y.TradeAmt = x.TradeAmt;
    y.CustFee = x.CustFee;
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    return y;
}

TransferBankToFutureReqField Converter::CThostFtdcTransferBankToFutureReqFieldToRust(CThostFtdcTransferBankToFutureReqField* x) {
    if (x == nullptr)
        return TransferBankToFutureReqField{.is_null = true};
    TransferBankToFutureReqField y{};
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.FuturePwdFlag = x->FuturePwdFlag;
    y.FutureAccPwd = Converter::Gb2312ToRustString(x->FutureAccPwd);
    y.TradeAmt = x->TradeAmt;
    y.CustFee = x->CustFee;
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    return y;
}

CThostFtdcTransferBankToFutureRspField Converter::TransferBankToFutureRspFieldToCpp(TransferBankToFutureRspField x) {
    CThostFtdcTransferBankToFutureRspField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.RetCode, x.RetCode.c_str());
    strcpy(y.RetInfo, x.RetInfo.c_str());
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.TradeAmt = x.TradeAmt;
    y.CustFee = x.CustFee;
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    return y;
}

TransferBankToFutureRspField Converter::CThostFtdcTransferBankToFutureRspFieldToRust(CThostFtdcTransferBankToFutureRspField* x) {
    if (x == nullptr)
        return TransferBankToFutureRspField{.is_null = true};
    TransferBankToFutureRspField y{};
    y.RetCode = Converter::Gb2312ToRustString(x->RetCode);
    y.RetInfo = Converter::Gb2312ToRustString(x->RetInfo);
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.TradeAmt = x->TradeAmt;
    y.CustFee = x->CustFee;
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    return y;
}

CThostFtdcTransferFutureToBankReqField Converter::TransferFutureToBankReqFieldToCpp(TransferFutureToBankReqField x) {
    CThostFtdcTransferFutureToBankReqField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.FuturePwdFlag = x.FuturePwdFlag;
    strcpy(y.FutureAccPwd, x.FutureAccPwd.c_str());
    y.TradeAmt = x.TradeAmt;
    y.CustFee = x.CustFee;
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    return y;
}

TransferFutureToBankReqField Converter::CThostFtdcTransferFutureToBankReqFieldToRust(CThostFtdcTransferFutureToBankReqField* x) {
    if (x == nullptr)
        return TransferFutureToBankReqField{.is_null = true};
    TransferFutureToBankReqField y{};
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.FuturePwdFlag = x->FuturePwdFlag;
    y.FutureAccPwd = Converter::Gb2312ToRustString(x->FutureAccPwd);
    y.TradeAmt = x->TradeAmt;
    y.CustFee = x->CustFee;
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    return y;
}

CThostFtdcTransferFutureToBankRspField Converter::TransferFutureToBankRspFieldToCpp(TransferFutureToBankRspField x) {
    CThostFtdcTransferFutureToBankRspField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.RetCode, x.RetCode.c_str());
    strcpy(y.RetInfo, x.RetInfo.c_str());
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.TradeAmt = x.TradeAmt;
    y.CustFee = x.CustFee;
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    return y;
}

TransferFutureToBankRspField Converter::CThostFtdcTransferFutureToBankRspFieldToRust(CThostFtdcTransferFutureToBankRspField* x) {
    if (x == nullptr)
        return TransferFutureToBankRspField{.is_null = true};
    TransferFutureToBankRspField y{};
    y.RetCode = Converter::Gb2312ToRustString(x->RetCode);
    y.RetInfo = Converter::Gb2312ToRustString(x->RetInfo);
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.TradeAmt = x->TradeAmt;
    y.CustFee = x->CustFee;
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    return y;
}

CThostFtdcTransferQryBankReqField Converter::TransferQryBankReqFieldToCpp(TransferQryBankReqField x) {
    CThostFtdcTransferQryBankReqField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.FuturePwdFlag = x.FuturePwdFlag;
    strcpy(y.FutureAccPwd, x.FutureAccPwd.c_str());
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    return y;
}

TransferQryBankReqField Converter::CThostFtdcTransferQryBankReqFieldToRust(CThostFtdcTransferQryBankReqField* x) {
    if (x == nullptr)
        return TransferQryBankReqField{.is_null = true};
    TransferQryBankReqField y{};
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.FuturePwdFlag = x->FuturePwdFlag;
    y.FutureAccPwd = Converter::Gb2312ToRustString(x->FutureAccPwd);
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    return y;
}

CThostFtdcTransferQryBankRspField Converter::TransferQryBankRspFieldToCpp(TransferQryBankRspField x) {
    CThostFtdcTransferQryBankRspField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.RetCode, x.RetCode.c_str());
    strcpy(y.RetInfo, x.RetInfo.c_str());
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.TradeAmt = x.TradeAmt;
    y.UseAmt = x.UseAmt;
    y.FetchAmt = x.FetchAmt;
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    return y;
}

TransferQryBankRspField Converter::CThostFtdcTransferQryBankRspFieldToRust(CThostFtdcTransferQryBankRspField* x) {
    if (x == nullptr)
        return TransferQryBankRspField{.is_null = true};
    TransferQryBankRspField y{};
    y.RetCode = Converter::Gb2312ToRustString(x->RetCode);
    y.RetInfo = Converter::Gb2312ToRustString(x->RetInfo);
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.TradeAmt = x->TradeAmt;
    y.UseAmt = x->UseAmt;
    y.FetchAmt = x->FetchAmt;
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    return y;
}

CThostFtdcTransferQryDetailReqField Converter::TransferQryDetailReqFieldToCpp(TransferQryDetailReqField x) {
    CThostFtdcTransferQryDetailReqField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    return y;
}

TransferQryDetailReqField Converter::CThostFtdcTransferQryDetailReqFieldToRust(CThostFtdcTransferQryDetailReqField* x) {
    if (x == nullptr)
        return TransferQryDetailReqField{.is_null = true};
    TransferQryDetailReqField y{};
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    return y;
}

CThostFtdcTransferQryDetailRspField Converter::TransferQryDetailRspFieldToCpp(TransferQryDetailRspField x) {
    CThostFtdcTransferQryDetailRspField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.TradeCode, x.TradeCode.c_str());
    y.FutureSerial = x.FutureSerial;
    strcpy(y.FutureID, x.FutureID.c_str());
    strcpy(y.FutureAccount, x.FutureAccount.c_str());
    y.BankSerial = x.BankSerial;
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBrchID, x.BankBrchID.c_str());
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.CertCode, x.CertCode.c_str());
    strcpy(y.CurrencyCode, x.CurrencyCode.c_str());
    y.TxAmount = x.TxAmount;
    y.Flag = x.Flag;
    return y;
}

TransferQryDetailRspField Converter::CThostFtdcTransferQryDetailRspFieldToRust(CThostFtdcTransferQryDetailRspField* x) {
    if (x == nullptr)
        return TransferQryDetailRspField{.is_null = true};
    TransferQryDetailRspField y{};
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.FutureSerial = x->FutureSerial;
    y.FutureID = Converter::Gb2312ToRustString(x->FutureID);
    y.FutureAccount = Converter::Gb2312ToRustString(x->FutureAccount);
    y.BankSerial = x->BankSerial;
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBrchID = Converter::Gb2312ToRustString(x->BankBrchID);
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.CertCode = Converter::Gb2312ToRustString(x->CertCode);
    y.CurrencyCode = Converter::Gb2312ToRustString(x->CurrencyCode);
    y.TxAmount = x->TxAmount;
    y.Flag = x->Flag;
    return y;
}

CThostFtdcRspInfoField Converter::RspInfoFieldToCpp(RspInfoField x) {
    CThostFtdcRspInfoField y;
    memset(&y, 0, sizeof(y));
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

RspInfoField Converter::CThostFtdcRspInfoFieldToRust(CThostFtdcRspInfoField* x) {
    if (x == nullptr)
        return RspInfoField{.is_null = true};
    RspInfoField y{};
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcExchangeField Converter::ExchangeFieldToCpp(ExchangeField x) {
    CThostFtdcExchangeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExchangeName, x.ExchangeName.c_str());
    y.ExchangeProperty = x.ExchangeProperty;
    return y;
}

ExchangeField Converter::CThostFtdcExchangeFieldToRust(CThostFtdcExchangeField* x) {
    if (x == nullptr)
        return ExchangeField{.is_null = true};
    ExchangeField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExchangeName = Converter::Gb2312ToRustString(x->ExchangeName);
    y.ExchangeProperty = x->ExchangeProperty;
    return y;
}

CThostFtdcProductField Converter::ProductFieldToCpp(ProductField x) {
    CThostFtdcProductField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductName, x.ProductName.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.ProductClass = x.ProductClass;
    y.VolumeMultiple = x.VolumeMultiple;
    y.PriceTick = x.PriceTick;
    y.MaxMarketOrderVolume = x.MaxMarketOrderVolume;
    y.MinMarketOrderVolume = x.MinMarketOrderVolume;
    y.MaxLimitOrderVolume = x.MaxLimitOrderVolume;
    y.MinLimitOrderVolume = x.MinLimitOrderVolume;
    y.PositionType = x.PositionType;
    y.PositionDateType = x.PositionDateType;
    y.CloseDealType = x.CloseDealType;
    strcpy(y.TradeCurrencyID, x.TradeCurrencyID.c_str());
    y.MortgageFundUseRange = x.MortgageFundUseRange;
    y.UnderlyingMultiple = x.UnderlyingMultiple;
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.ExchangeProductID, x.ExchangeProductID.c_str());
    y.OpenLimitControlLevel = x.OpenLimitControlLevel;
    y.OrderFreqControlLevel = x.OrderFreqControlLevel;
    return y;
}

ProductField Converter::CThostFtdcProductFieldToRust(CThostFtdcProductField* x) {
    if (x == nullptr)
        return ProductField{.is_null = true};
    ProductField y{};
    y.ProductName = Converter::Gb2312ToRustString(x->ProductName);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductClass = x->ProductClass;
    y.VolumeMultiple = x->VolumeMultiple;
    y.PriceTick = x->PriceTick;
    y.MaxMarketOrderVolume = x->MaxMarketOrderVolume;
    y.MinMarketOrderVolume = x->MinMarketOrderVolume;
    y.MaxLimitOrderVolume = x->MaxLimitOrderVolume;
    y.MinLimitOrderVolume = x->MinLimitOrderVolume;
    y.PositionType = x->PositionType;
    y.PositionDateType = x->PositionDateType;
    y.CloseDealType = x->CloseDealType;
    y.TradeCurrencyID = Converter::Gb2312ToRustString(x->TradeCurrencyID);
    y.MortgageFundUseRange = x->MortgageFundUseRange;
    y.UnderlyingMultiple = x->UnderlyingMultiple;
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.ExchangeProductID = Converter::Gb2312ToRustString(x->ExchangeProductID);
    y.OpenLimitControlLevel = x->OpenLimitControlLevel;
    y.OrderFreqControlLevel = x->OrderFreqControlLevel;
    return y;
}

CThostFtdcInstrumentField Converter::InstrumentFieldToCpp(InstrumentField x) {
    CThostFtdcInstrumentField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentName, x.InstrumentName.c_str());
    y.ProductClass = x.ProductClass;
    y.DeliveryYear = x.DeliveryYear;
    y.DeliveryMonth = x.DeliveryMonth;
    y.MaxMarketOrderVolume = x.MaxMarketOrderVolume;
    y.MinMarketOrderVolume = x.MinMarketOrderVolume;
    y.MaxLimitOrderVolume = x.MaxLimitOrderVolume;
    y.MinLimitOrderVolume = x.MinLimitOrderVolume;
    y.VolumeMultiple = x.VolumeMultiple;
    y.PriceTick = x.PriceTick;
    strcpy(y.CreateDate, x.CreateDate.c_str());
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.ExpireDate, x.ExpireDate.c_str());
    strcpy(y.StartDelivDate, x.StartDelivDate.c_str());
    strcpy(y.EndDelivDate, x.EndDelivDate.c_str());
    y.InstLifePhase = x.InstLifePhase;
    y.IsTrading = x.IsTrading;
    y.PositionType = x.PositionType;
    y.PositionDateType = x.PositionDateType;
    y.LongMarginRatio = x.LongMarginRatio;
    y.ShortMarginRatio = x.ShortMarginRatio;
    y.MaxMarginSideAlgorithm = x.MaxMarginSideAlgorithm;
    y.StrikePrice = x.StrikePrice;
    y.OptionsType = x.OptionsType;
    y.UnderlyingMultiple = x.UnderlyingMultiple;
    y.CombinationType = x.CombinationType;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.UnderlyingInstrID, x.UnderlyingInstrID.c_str());
    return y;
}

InstrumentField Converter::CThostFtdcInstrumentFieldToRust(CThostFtdcInstrumentField* x) {
    if (x == nullptr)
        return InstrumentField{.is_null = true};
    InstrumentField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentName = Converter::Gb2312ToRustString(x->InstrumentName);
    y.ProductClass = x->ProductClass;
    y.DeliveryYear = x->DeliveryYear;
    y.DeliveryMonth = x->DeliveryMonth;
    y.MaxMarketOrderVolume = x->MaxMarketOrderVolume;
    y.MinMarketOrderVolume = x->MinMarketOrderVolume;
    y.MaxLimitOrderVolume = x->MaxLimitOrderVolume;
    y.MinLimitOrderVolume = x->MinLimitOrderVolume;
    y.VolumeMultiple = x->VolumeMultiple;
    y.PriceTick = x->PriceTick;
    y.CreateDate = Converter::Gb2312ToRustString(x->CreateDate);
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.ExpireDate = Converter::Gb2312ToRustString(x->ExpireDate);
    y.StartDelivDate = Converter::Gb2312ToRustString(x->StartDelivDate);
    y.EndDelivDate = Converter::Gb2312ToRustString(x->EndDelivDate);
    y.InstLifePhase = x->InstLifePhase;
    y.IsTrading = x->IsTrading;
    y.PositionType = x->PositionType;
    y.PositionDateType = x->PositionDateType;
    y.LongMarginRatio = x->LongMarginRatio;
    y.ShortMarginRatio = x->ShortMarginRatio;
    y.MaxMarginSideAlgorithm = x->MaxMarginSideAlgorithm;
    y.StrikePrice = x->StrikePrice;
    y.OptionsType = x->OptionsType;
    y.UnderlyingMultiple = x->UnderlyingMultiple;
    y.CombinationType = x->CombinationType;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.UnderlyingInstrID = Converter::Gb2312ToRustString(x->UnderlyingInstrID);
    return y;
}

CThostFtdcBrokerField Converter::BrokerFieldToCpp(BrokerField x) {
    CThostFtdcBrokerField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerAbbr, x.BrokerAbbr.c_str());
    strcpy(y.BrokerName, x.BrokerName.c_str());
    y.IsActive = x.IsActive;
    return y;
}

BrokerField Converter::CThostFtdcBrokerFieldToRust(CThostFtdcBrokerField* x) {
    if (x == nullptr)
        return BrokerField{.is_null = true};
    BrokerField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerAbbr = Converter::Gb2312ToRustString(x->BrokerAbbr);
    y.BrokerName = Converter::Gb2312ToRustString(x->BrokerName);
    y.IsActive = x->IsActive;
    return y;
}

CThostFtdcTraderField Converter::TraderFieldToCpp(TraderField x) {
    CThostFtdcTraderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallCount = x.InstallCount;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.OrderCancelAlg = x.OrderCancelAlg;
    y.TradeInstallCount = x.TradeInstallCount;
    y.MDInstallCount = x.MDInstallCount;
    return y;
}

TraderField Converter::CThostFtdcTraderFieldToRust(CThostFtdcTraderField* x) {
    if (x == nullptr)
        return TraderField{.is_null = true};
    TraderField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallCount = x->InstallCount;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.OrderCancelAlg = x->OrderCancelAlg;
    y.TradeInstallCount = x->TradeInstallCount;
    y.MDInstallCount = x->MDInstallCount;
    return y;
}

CThostFtdcInvestorField Converter::InvestorFieldToCpp(InvestorField x) {
    CThostFtdcInvestorField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorGroupID, x.InvestorGroupID.c_str());
    strcpy(y.InvestorName, x.InvestorName.c_str());
    y.IdentifiedCardType = x.IdentifiedCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.IsActive = x.IsActive;
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.Mobile, x.Mobile.c_str());
    strcpy(y.CommModelID, x.CommModelID.c_str());
    strcpy(y.MarginModelID, x.MarginModelID.c_str());
    y.IsOrderFreq = x.IsOrderFreq;
    y.IsOpenVolLimit = x.IsOpenVolLimit;
    return y;
}

InvestorField Converter::CThostFtdcInvestorFieldToRust(CThostFtdcInvestorField* x) {
    if (x == nullptr)
        return InvestorField{.is_null = true};
    InvestorField y{};
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorGroupID = Converter::Gb2312ToRustString(x->InvestorGroupID);
    y.InvestorName = Converter::Gb2312ToRustString(x->InvestorName);
    y.IdentifiedCardType = x->IdentifiedCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.IsActive = x->IsActive;
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.Mobile = Converter::Gb2312ToRustString(x->Mobile);
    y.CommModelID = Converter::Gb2312ToRustString(x->CommModelID);
    y.MarginModelID = Converter::Gb2312ToRustString(x->MarginModelID);
    y.IsOrderFreq = x->IsOrderFreq;
    y.IsOpenVolLimit = x->IsOpenVolLimit;
    return y;
}

CThostFtdcTradingCodeField Converter::TradingCodeFieldToCpp(TradingCodeField x) {
    CThostFtdcTradingCodeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    y.IsActive = x.IsActive;
    y.ClientIDType = x.ClientIDType;
    strcpy(y.BranchID, x.BranchID.c_str());
    y.BizType = x.BizType;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

TradingCodeField Converter::CThostFtdcTradingCodeFieldToRust(CThostFtdcTradingCodeField* x) {
    if (x == nullptr)
        return TradingCodeField{.is_null = true};
    TradingCodeField y{};
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.IsActive = x->IsActive;
    y.ClientIDType = x->ClientIDType;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.BizType = x->BizType;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcPartBrokerField Converter::PartBrokerFieldToCpp(PartBrokerField x) {
    CThostFtdcPartBrokerField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    y.IsActive = x.IsActive;
    return y;
}

PartBrokerField Converter::CThostFtdcPartBrokerFieldToRust(CThostFtdcPartBrokerField* x) {
    if (x == nullptr)
        return PartBrokerField{.is_null = true};
    PartBrokerField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.IsActive = x->IsActive;
    return y;
}

CThostFtdcSuperUserField Converter::SuperUserFieldToCpp(SuperUserField x) {
    CThostFtdcSuperUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.UserName, x.UserName.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.IsActive = x.IsActive;
    return y;
}

SuperUserField Converter::CThostFtdcSuperUserFieldToRust(CThostFtdcSuperUserField* x) {
    if (x == nullptr)
        return SuperUserField{.is_null = true};
    SuperUserField y{};
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserName = Converter::Gb2312ToRustString(x->UserName);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.IsActive = x->IsActive;
    return y;
}

CThostFtdcSuperUserFunctionField Converter::SuperUserFunctionFieldToCpp(SuperUserFunctionField x) {
    CThostFtdcSuperUserFunctionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.UserID, x.UserID.c_str());
    y.FunctionCode = x.FunctionCode;
    return y;
}

SuperUserFunctionField Converter::CThostFtdcSuperUserFunctionFieldToRust(CThostFtdcSuperUserFunctionField* x) {
    if (x == nullptr)
        return SuperUserFunctionField{.is_null = true};
    SuperUserFunctionField y{};
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.FunctionCode = x->FunctionCode;
    return y;
}

CThostFtdcInvestorGroupField Converter::InvestorGroupFieldToCpp(InvestorGroupField x) {
    CThostFtdcInvestorGroupField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorGroupID, x.InvestorGroupID.c_str());
    strcpy(y.InvestorGroupName, x.InvestorGroupName.c_str());
    return y;
}

InvestorGroupField Converter::CThostFtdcInvestorGroupFieldToRust(CThostFtdcInvestorGroupField* x) {
    if (x == nullptr)
        return InvestorGroupField{.is_null = true};
    InvestorGroupField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorGroupID = Converter::Gb2312ToRustString(x->InvestorGroupID);
    y.InvestorGroupName = Converter::Gb2312ToRustString(x->InvestorGroupName);
    return y;
}

CThostFtdcTradingAccountField Converter::TradingAccountFieldToCpp(TradingAccountField x) {
    CThostFtdcTradingAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.PreMortgage = x.PreMortgage;
    y.PreCredit = x.PreCredit;
    y.PreDeposit = x.PreDeposit;
    y.PreBalance = x.PreBalance;
    y.PreMargin = x.PreMargin;
    y.InterestBase = x.InterestBase;
    y.Interest = x.Interest;
    y.Deposit = x.Deposit;
    y.Withdraw = x.Withdraw;
    y.FrozenMargin = x.FrozenMargin;
    y.FrozenCash = x.FrozenCash;
    y.FrozenCommission = x.FrozenCommission;
    y.CurrMargin = x.CurrMargin;
    y.CashIn = x.CashIn;
    y.Commission = x.Commission;
    y.CloseProfit = x.CloseProfit;
    y.PositionProfit = x.PositionProfit;
    y.Balance = x.Balance;
    y.Available = x.Available;
    y.WithdrawQuota = x.WithdrawQuota;
    y.Reserve = x.Reserve;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.Credit = x.Credit;
    y.Mortgage = x.Mortgage;
    y.ExchangeMargin = x.ExchangeMargin;
    y.DeliveryMargin = x.DeliveryMargin;
    y.ExchangeDeliveryMargin = x.ExchangeDeliveryMargin;
    y.ReserveBalance = x.ReserveBalance;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.PreFundMortgageIn = x.PreFundMortgageIn;
    y.PreFundMortgageOut = x.PreFundMortgageOut;
    y.FundMortgageIn = x.FundMortgageIn;
    y.FundMortgageOut = x.FundMortgageOut;
    y.FundMortgageAvailable = x.FundMortgageAvailable;
    y.MortgageableFund = x.MortgageableFund;
    y.SpecProductMargin = x.SpecProductMargin;
    y.SpecProductFrozenMargin = x.SpecProductFrozenMargin;
    y.SpecProductCommission = x.SpecProductCommission;
    y.SpecProductFrozenCommission = x.SpecProductFrozenCommission;
    y.SpecProductPositionProfit = x.SpecProductPositionProfit;
    y.SpecProductCloseProfit = x.SpecProductCloseProfit;
    y.SpecProductPositionProfitByAlg = x.SpecProductPositionProfitByAlg;
    y.SpecProductExchangeMargin = x.SpecProductExchangeMargin;
    y.BizType = x.BizType;
    y.FrozenSwap = x.FrozenSwap;
    y.RemainSwap = x.RemainSwap;
    y.OptionValue = x.OptionValue;
    return y;
}

TradingAccountField Converter::CThostFtdcTradingAccountFieldToRust(CThostFtdcTradingAccountField* x) {
    if (x == nullptr)
        return TradingAccountField{.is_null = true};
    TradingAccountField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.PreMortgage = x->PreMortgage;
    y.PreCredit = x->PreCredit;
    y.PreDeposit = x->PreDeposit;
    y.PreBalance = x->PreBalance;
    y.PreMargin = x->PreMargin;
    y.InterestBase = x->InterestBase;
    y.Interest = x->Interest;
    y.Deposit = x->Deposit;
    y.Withdraw = x->Withdraw;
    y.FrozenMargin = x->FrozenMargin;
    y.FrozenCash = x->FrozenCash;
    y.FrozenCommission = x->FrozenCommission;
    y.CurrMargin = x->CurrMargin;
    y.CashIn = x->CashIn;
    y.Commission = x->Commission;
    y.CloseProfit = x->CloseProfit;
    y.PositionProfit = x->PositionProfit;
    y.Balance = x->Balance;
    y.Available = x->Available;
    y.WithdrawQuota = x->WithdrawQuota;
    y.Reserve = x->Reserve;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.Credit = x->Credit;
    y.Mortgage = x->Mortgage;
    y.ExchangeMargin = x->ExchangeMargin;
    y.DeliveryMargin = x->DeliveryMargin;
    y.ExchangeDeliveryMargin = x->ExchangeDeliveryMargin;
    y.ReserveBalance = x->ReserveBalance;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.PreFundMortgageIn = x->PreFundMortgageIn;
    y.PreFundMortgageOut = x->PreFundMortgageOut;
    y.FundMortgageIn = x->FundMortgageIn;
    y.FundMortgageOut = x->FundMortgageOut;
    y.FundMortgageAvailable = x->FundMortgageAvailable;
    y.MortgageableFund = x->MortgageableFund;
    y.SpecProductMargin = x->SpecProductMargin;
    y.SpecProductFrozenMargin = x->SpecProductFrozenMargin;
    y.SpecProductCommission = x->SpecProductCommission;
    y.SpecProductFrozenCommission = x->SpecProductFrozenCommission;
    y.SpecProductPositionProfit = x->SpecProductPositionProfit;
    y.SpecProductCloseProfit = x->SpecProductCloseProfit;
    y.SpecProductPositionProfitByAlg = x->SpecProductPositionProfitByAlg;
    y.SpecProductExchangeMargin = x->SpecProductExchangeMargin;
    y.BizType = x->BizType;
    y.FrozenSwap = x->FrozenSwap;
    y.RemainSwap = x->RemainSwap;
    y.OptionValue = x->OptionValue;
    return y;
}

CThostFtdcInvestorPositionField Converter::InvestorPositionFieldToCpp(InvestorPositionField x) {
    CThostFtdcInvestorPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.PosiDirection = x.PosiDirection;
    y.HedgeFlag = x.HedgeFlag;
    y.PositionDate = x.PositionDate;
    y.YdPosition = x.YdPosition;
    y.Position = x.Position;
    y.LongFrozen = x.LongFrozen;
    y.ShortFrozen = x.ShortFrozen;
    y.LongFrozenAmount = x.LongFrozenAmount;
    y.ShortFrozenAmount = x.ShortFrozenAmount;
    y.OpenVolume = x.OpenVolume;
    y.CloseVolume = x.CloseVolume;
    y.OpenAmount = x.OpenAmount;
    y.CloseAmount = x.CloseAmount;
    y.PositionCost = x.PositionCost;
    y.PreMargin = x.PreMargin;
    y.UseMargin = x.UseMargin;
    y.FrozenMargin = x.FrozenMargin;
    y.FrozenCash = x.FrozenCash;
    y.FrozenCommission = x.FrozenCommission;
    y.CashIn = x.CashIn;
    y.Commission = x.Commission;
    y.CloseProfit = x.CloseProfit;
    y.PositionProfit = x.PositionProfit;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.SettlementPrice = x.SettlementPrice;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.OpenCost = x.OpenCost;
    y.ExchangeMargin = x.ExchangeMargin;
    y.CombPosition = x.CombPosition;
    y.CombLongFrozen = x.CombLongFrozen;
    y.CombShortFrozen = x.CombShortFrozen;
    y.CloseProfitByDate = x.CloseProfitByDate;
    y.CloseProfitByTrade = x.CloseProfitByTrade;
    y.TodayPosition = x.TodayPosition;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.StrikeFrozen = x.StrikeFrozen;
    y.StrikeFrozenAmount = x.StrikeFrozenAmount;
    y.AbandonFrozen = x.AbandonFrozen;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.YdStrikeFrozen = x.YdStrikeFrozen;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    y.PositionCostOffset = x.PositionCostOffset;
    y.TasPosition = x.TasPosition;
    y.TasPositionCost = x.TasPositionCost;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.OptionValue = x.OptionValue;
    return y;
}

InvestorPositionField Converter::CThostFtdcInvestorPositionFieldToRust(CThostFtdcInvestorPositionField* x) {
    if (x == nullptr)
        return InvestorPositionField{.is_null = true};
    InvestorPositionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.PosiDirection = x->PosiDirection;
    y.HedgeFlag = x->HedgeFlag;
    y.PositionDate = x->PositionDate;
    y.YdPosition = x->YdPosition;
    y.Position = x->Position;
    y.LongFrozen = x->LongFrozen;
    y.ShortFrozen = x->ShortFrozen;
    y.LongFrozenAmount = x->LongFrozenAmount;
    y.ShortFrozenAmount = x->ShortFrozenAmount;
    y.OpenVolume = x->OpenVolume;
    y.CloseVolume = x->CloseVolume;
    y.OpenAmount = x->OpenAmount;
    y.CloseAmount = x->CloseAmount;
    y.PositionCost = x->PositionCost;
    y.PreMargin = x->PreMargin;
    y.UseMargin = x->UseMargin;
    y.FrozenMargin = x->FrozenMargin;
    y.FrozenCash = x->FrozenCash;
    y.FrozenCommission = x->FrozenCommission;
    y.CashIn = x->CashIn;
    y.Commission = x->Commission;
    y.CloseProfit = x->CloseProfit;
    y.PositionProfit = x->PositionProfit;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.SettlementPrice = x->SettlementPrice;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OpenCost = x->OpenCost;
    y.ExchangeMargin = x->ExchangeMargin;
    y.CombPosition = x->CombPosition;
    y.CombLongFrozen = x->CombLongFrozen;
    y.CombShortFrozen = x->CombShortFrozen;
    y.CloseProfitByDate = x->CloseProfitByDate;
    y.CloseProfitByTrade = x->CloseProfitByTrade;
    y.TodayPosition = x->TodayPosition;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.StrikeFrozen = x->StrikeFrozen;
    y.StrikeFrozenAmount = x->StrikeFrozenAmount;
    y.AbandonFrozen = x->AbandonFrozen;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.YdStrikeFrozen = x->YdStrikeFrozen;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.PositionCostOffset = x->PositionCostOffset;
    y.TasPosition = x->TasPosition;
    y.TasPositionCost = x->TasPositionCost;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.OptionValue = x->OptionValue;
    return y;
}

CThostFtdcInstrumentMarginRateField Converter::InstrumentMarginRateFieldToCpp(InstrumentMarginRateField x) {
    CThostFtdcInstrumentMarginRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.IsRelative = x.IsRelative;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

InstrumentMarginRateField Converter::CThostFtdcInstrumentMarginRateFieldToRust(CThostFtdcInstrumentMarginRateField* x) {
    if (x == nullptr)
        return InstrumentMarginRateField{.is_null = true};
    InstrumentMarginRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.IsRelative = x->IsRelative;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInstrumentCommissionRateField Converter::InstrumentCommissionRateFieldToCpp(InstrumentCommissionRateField x) {
    CThostFtdcInstrumentCommissionRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.BizType = x.BizType;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

InstrumentCommissionRateField Converter::CThostFtdcInstrumentCommissionRateFieldToRust(CThostFtdcInstrumentCommissionRateField* x) {
    if (x == nullptr)
        return InstrumentCommissionRateField{.is_null = true};
    InstrumentCommissionRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BizType = x->BizType;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcDepthMarketDataField Converter::DepthMarketDataFieldToCpp(DepthMarketDataField x) {
    CThostFtdcDepthMarketDataField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.LastPrice = x.LastPrice;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.PreClosePrice = x.PreClosePrice;
    y.PreOpenInterest = x.PreOpenInterest;
    y.OpenPrice = x.OpenPrice;
    y.HighestPrice = x.HighestPrice;
    y.LowestPrice = x.LowestPrice;
    y.Volume = x.Volume;
    y.Turnover = x.Turnover;
    y.OpenInterest = x.OpenInterest;
    y.ClosePrice = x.ClosePrice;
    y.SettlementPrice = x.SettlementPrice;
    y.UpperLimitPrice = x.UpperLimitPrice;
    y.LowerLimitPrice = x.LowerLimitPrice;
    y.PreDelta = x.PreDelta;
    y.CurrDelta = x.CurrDelta;
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    y.UpdateMillisec = x.UpdateMillisec;
    y.BidPrice1 = x.BidPrice1;
    y.BidVolume1 = x.BidVolume1;
    y.AskPrice1 = x.AskPrice1;
    y.AskVolume1 = x.AskVolume1;
    y.BidPrice2 = x.BidPrice2;
    y.BidVolume2 = x.BidVolume2;
    y.AskPrice2 = x.AskPrice2;
    y.AskVolume2 = x.AskVolume2;
    y.BidPrice3 = x.BidPrice3;
    y.BidVolume3 = x.BidVolume3;
    y.AskPrice3 = x.AskPrice3;
    y.AskVolume3 = x.AskVolume3;
    y.BidPrice4 = x.BidPrice4;
    y.BidVolume4 = x.BidVolume4;
    y.AskPrice4 = x.AskPrice4;
    y.AskVolume4 = x.AskVolume4;
    y.BidPrice5 = x.BidPrice5;
    y.BidVolume5 = x.BidVolume5;
    y.AskPrice5 = x.AskPrice5;
    y.AskVolume5 = x.AskVolume5;
    y.AveragePrice = x.AveragePrice;
    strcpy(y.ActionDay, x.ActionDay.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    y.BandingUpperPrice = x.BandingUpperPrice;
    y.BandingLowerPrice = x.BandingLowerPrice;
    return y;
}

DepthMarketDataField Converter::CThostFtdcDepthMarketDataFieldToRust(CThostFtdcDepthMarketDataField* x) {
    if (x == nullptr)
        return DepthMarketDataField{.is_null = true};
    DepthMarketDataField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.LastPrice = x->LastPrice;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.PreClosePrice = x->PreClosePrice;
    y.PreOpenInterest = x->PreOpenInterest;
    y.OpenPrice = x->OpenPrice;
    y.HighestPrice = x->HighestPrice;
    y.LowestPrice = x->LowestPrice;
    y.Volume = x->Volume;
    y.Turnover = x->Turnover;
    y.OpenInterest = x->OpenInterest;
    y.ClosePrice = x->ClosePrice;
    y.SettlementPrice = x->SettlementPrice;
    y.UpperLimitPrice = x->UpperLimitPrice;
    y.LowerLimitPrice = x->LowerLimitPrice;
    y.PreDelta = x->PreDelta;
    y.CurrDelta = x->CurrDelta;
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.UpdateMillisec = x->UpdateMillisec;
    y.BidPrice1 = x->BidPrice1;
    y.BidVolume1 = x->BidVolume1;
    y.AskPrice1 = x->AskPrice1;
    y.AskVolume1 = x->AskVolume1;
    y.BidPrice2 = x->BidPrice2;
    y.BidVolume2 = x->BidVolume2;
    y.AskPrice2 = x->AskPrice2;
    y.AskVolume2 = x->AskVolume2;
    y.BidPrice3 = x->BidPrice3;
    y.BidVolume3 = x->BidVolume3;
    y.AskPrice3 = x->AskPrice3;
    y.AskVolume3 = x->AskVolume3;
    y.BidPrice4 = x->BidPrice4;
    y.BidVolume4 = x->BidVolume4;
    y.AskPrice4 = x->AskPrice4;
    y.AskVolume4 = x->AskVolume4;
    y.BidPrice5 = x->BidPrice5;
    y.BidVolume5 = x->BidVolume5;
    y.AskPrice5 = x->AskPrice5;
    y.AskVolume5 = x->AskVolume5;
    y.AveragePrice = x->AveragePrice;
    y.ActionDay = Converter::Gb2312ToRustString(x->ActionDay);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.BandingUpperPrice = x->BandingUpperPrice;
    y.BandingLowerPrice = x->BandingLowerPrice;
    return y;
}

CThostFtdcInstrumentTradingRightField Converter::InstrumentTradingRightFieldToCpp(InstrumentTradingRightField x) {
    CThostFtdcInstrumentTradingRightField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.TradingRight = x.TradingRight;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

InstrumentTradingRightField Converter::CThostFtdcInstrumentTradingRightFieldToRust(CThostFtdcInstrumentTradingRightField* x) {
    if (x == nullptr)
        return InstrumentTradingRightField{.is_null = true};
    InstrumentTradingRightField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.TradingRight = x->TradingRight;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcBrokerUserField Converter::BrokerUserFieldToCpp(BrokerUserField x) {
    CThostFtdcBrokerUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.UserName, x.UserName.c_str());
    y.UserType = x.UserType;
    y.IsActive = x.IsActive;
    y.IsUsingOTP = x.IsUsingOTP;
    y.IsAuthForce = x.IsAuthForce;
    return y;
}

BrokerUserField Converter::CThostFtdcBrokerUserFieldToRust(CThostFtdcBrokerUserField* x) {
    if (x == nullptr)
        return BrokerUserField{.is_null = true};
    BrokerUserField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserName = Converter::Gb2312ToRustString(x->UserName);
    y.UserType = x->UserType;
    y.IsActive = x->IsActive;
    y.IsUsingOTP = x->IsUsingOTP;
    y.IsAuthForce = x->IsAuthForce;
    return y;
}

CThostFtdcBrokerUserPasswordField Converter::BrokerUserPasswordFieldToCpp(BrokerUserPasswordField x) {
    CThostFtdcBrokerUserPasswordField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.LastUpdateTime, x.LastUpdateTime.c_str());
    strcpy(y.LastLoginTime, x.LastLoginTime.c_str());
    strcpy(y.ExpireDate, x.ExpireDate.c_str());
    strcpy(y.WeakExpireDate, x.WeakExpireDate.c_str());
    return y;
}

BrokerUserPasswordField Converter::CThostFtdcBrokerUserPasswordFieldToRust(CThostFtdcBrokerUserPasswordField* x) {
    if (x == nullptr)
        return BrokerUserPasswordField{.is_null = true};
    BrokerUserPasswordField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.LastUpdateTime = Converter::Gb2312ToRustString(x->LastUpdateTime);
    y.LastLoginTime = Converter::Gb2312ToRustString(x->LastLoginTime);
    y.ExpireDate = Converter::Gb2312ToRustString(x->ExpireDate);
    y.WeakExpireDate = Converter::Gb2312ToRustString(x->WeakExpireDate);
    return y;
}

CThostFtdcBrokerUserFunctionField Converter::BrokerUserFunctionFieldToCpp(BrokerUserFunctionField x) {
    CThostFtdcBrokerUserFunctionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.BrokerFunctionCode = x.BrokerFunctionCode;
    return y;
}

BrokerUserFunctionField Converter::CThostFtdcBrokerUserFunctionFieldToRust(CThostFtdcBrokerUserFunctionField* x) {
    if (x == nullptr)
        return BrokerUserFunctionField{.is_null = true};
    BrokerUserFunctionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.BrokerFunctionCode = x->BrokerFunctionCode;
    return y;
}

CThostFtdcTraderOfferField Converter::TraderOfferFieldToCpp(TraderOfferField x) {
    CThostFtdcTraderOfferField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    y.TraderConnectStatus = x.TraderConnectStatus;
    strcpy(y.ConnectRequestDate, x.ConnectRequestDate.c_str());
    strcpy(y.ConnectRequestTime, x.ConnectRequestTime.c_str());
    strcpy(y.LastReportDate, x.LastReportDate.c_str());
    strcpy(y.LastReportTime, x.LastReportTime.c_str());
    strcpy(y.ConnectDate, x.ConnectDate.c_str());
    strcpy(y.ConnectTime, x.ConnectTime.c_str());
    strcpy(y.StartDate, x.StartDate.c_str());
    strcpy(y.StartTime, x.StartTime.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.MaxTradeID, x.MaxTradeID.c_str());
    memcpy(y.MaxOrderMessageReference, x.MaxOrderMessageReference.data(), x.MaxOrderMessageReference.size() * sizeof(uint8_t));
    y.OrderCancelAlg = x.OrderCancelAlg;
    return y;
}

TraderOfferField Converter::CThostFtdcTraderOfferFieldToRust(CThostFtdcTraderOfferField* x) {
    if (x == nullptr)
        return TraderOfferField{.is_null = true};
    TraderOfferField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.TraderConnectStatus = x->TraderConnectStatus;
    y.ConnectRequestDate = Converter::Gb2312ToRustString(x->ConnectRequestDate);
    y.ConnectRequestTime = Converter::Gb2312ToRustString(x->ConnectRequestTime);
    y.LastReportDate = Converter::Gb2312ToRustString(x->LastReportDate);
    y.LastReportTime = Converter::Gb2312ToRustString(x->LastReportTime);
    y.ConnectDate = Converter::Gb2312ToRustString(x->ConnectDate);
    y.ConnectTime = Converter::Gb2312ToRustString(x->ConnectTime);
    y.StartDate = Converter::Gb2312ToRustString(x->StartDate);
    y.StartTime = Converter::Gb2312ToRustString(x->StartTime);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.MaxTradeID = Converter::Gb2312ToRustString(x->MaxTradeID);
    for (int i = 0; i < 7; i++)
        y.MaxOrderMessageReference.push_back(x->MaxOrderMessageReference[i]);
    y.OrderCancelAlg = x->OrderCancelAlg;
    return y;
}

CThostFtdcSettlementInfoField Converter::SettlementInfoFieldToCpp(SettlementInfoField x) {
    CThostFtdcSettlementInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.SequenceNo = x.SequenceNo;
    memcpy(y.Content, x.Content.data(), x.Content.size() * sizeof(uint8_t));
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

SettlementInfoField Converter::CThostFtdcSettlementInfoFieldToRust(CThostFtdcSettlementInfoField* x) {
    if (x == nullptr)
        return SettlementInfoField{.is_null = true};
    SettlementInfoField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SequenceNo = x->SequenceNo;
    for (int i = 0; i < 501; i++)
        y.Content.push_back(x->Content[i]);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcInstrumentMarginRateAdjustField Converter::InstrumentMarginRateAdjustFieldToCpp(InstrumentMarginRateAdjustField x) {
    CThostFtdcInstrumentMarginRateAdjustField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.IsRelative = x.IsRelative;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

InstrumentMarginRateAdjustField Converter::CThostFtdcInstrumentMarginRateAdjustFieldToRust(CThostFtdcInstrumentMarginRateAdjustField* x) {
    if (x == nullptr)
        return InstrumentMarginRateAdjustField{.is_null = true};
    InstrumentMarginRateAdjustField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.IsRelative = x->IsRelative;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeMarginRateField Converter::ExchangeMarginRateFieldToCpp(ExchangeMarginRateField x) {
    CThostFtdcExchangeMarginRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

ExchangeMarginRateField Converter::CThostFtdcExchangeMarginRateFieldToRust(CThostFtdcExchangeMarginRateField* x) {
    if (x == nullptr)
        return ExchangeMarginRateField{.is_null = true};
    ExchangeMarginRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeMarginRateAdjustField Converter::ExchangeMarginRateAdjustFieldToCpp(ExchangeMarginRateAdjustField x) {
    CThostFtdcExchangeMarginRateAdjustField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.ExchLongMarginRatioByMoney = x.ExchLongMarginRatioByMoney;
    y.ExchLongMarginRatioByVolume = x.ExchLongMarginRatioByVolume;
    y.ExchShortMarginRatioByMoney = x.ExchShortMarginRatioByMoney;
    y.ExchShortMarginRatioByVolume = x.ExchShortMarginRatioByVolume;
    y.NoLongMarginRatioByMoney = x.NoLongMarginRatioByMoney;
    y.NoLongMarginRatioByVolume = x.NoLongMarginRatioByVolume;
    y.NoShortMarginRatioByMoney = x.NoShortMarginRatioByMoney;
    y.NoShortMarginRatioByVolume = x.NoShortMarginRatioByVolume;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

ExchangeMarginRateAdjustField Converter::CThostFtdcExchangeMarginRateAdjustFieldToRust(CThostFtdcExchangeMarginRateAdjustField* x) {
    if (x == nullptr)
        return ExchangeMarginRateAdjustField{.is_null = true};
    ExchangeMarginRateAdjustField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.ExchLongMarginRatioByMoney = x->ExchLongMarginRatioByMoney;
    y.ExchLongMarginRatioByVolume = x->ExchLongMarginRatioByVolume;
    y.ExchShortMarginRatioByMoney = x->ExchShortMarginRatioByMoney;
    y.ExchShortMarginRatioByVolume = x->ExchShortMarginRatioByVolume;
    y.NoLongMarginRatioByMoney = x->NoLongMarginRatioByMoney;
    y.NoLongMarginRatioByVolume = x->NoLongMarginRatioByVolume;
    y.NoShortMarginRatioByMoney = x->NoShortMarginRatioByMoney;
    y.NoShortMarginRatioByVolume = x->NoShortMarginRatioByVolume;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeRateField Converter::ExchangeRateFieldToCpp(ExchangeRateField x) {
    CThostFtdcExchangeRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.FromCurrencyID, x.FromCurrencyID.c_str());
    y.FromCurrencyUnit = x.FromCurrencyUnit;
    strcpy(y.ToCurrencyID, x.ToCurrencyID.c_str());
    y.ExchangeRate = x.ExchangeRate;
    return y;
}

ExchangeRateField Converter::CThostFtdcExchangeRateFieldToRust(CThostFtdcExchangeRateField* x) {
    if (x == nullptr)
        return ExchangeRateField{.is_null = true};
    ExchangeRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.FromCurrencyID = Converter::Gb2312ToRustString(x->FromCurrencyID);
    y.FromCurrencyUnit = x->FromCurrencyUnit;
    y.ToCurrencyID = Converter::Gb2312ToRustString(x->ToCurrencyID);
    y.ExchangeRate = x->ExchangeRate;
    return y;
}

CThostFtdcSettlementRefField Converter::SettlementRefFieldToCpp(SettlementRefField x) {
    CThostFtdcSettlementRefField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    return y;
}

SettlementRefField Converter::CThostFtdcSettlementRefFieldToRust(CThostFtdcSettlementRefField* x) {
    if (x == nullptr)
        return SettlementRefField{.is_null = true};
    SettlementRefField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    return y;
}

CThostFtdcCurrentTimeField Converter::CurrentTimeFieldToCpp(CurrentTimeField x) {
    CThostFtdcCurrentTimeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CurrDate, x.CurrDate.c_str());
    strcpy(y.CurrTime, x.CurrTime.c_str());
    y.CurrMillisec = x.CurrMillisec;
    strcpy(y.ActionDay, x.ActionDay.c_str());
    return y;
}

CurrentTimeField Converter::CThostFtdcCurrentTimeFieldToRust(CThostFtdcCurrentTimeField* x) {
    if (x == nullptr)
        return CurrentTimeField{.is_null = true};
    CurrentTimeField y{};
    y.CurrDate = Converter::Gb2312ToRustString(x->CurrDate);
    y.CurrTime = Converter::Gb2312ToRustString(x->CurrTime);
    y.CurrMillisec = x->CurrMillisec;
    y.ActionDay = Converter::Gb2312ToRustString(x->ActionDay);
    return y;
}

CThostFtdcCommPhaseField Converter::CommPhaseFieldToCpp(CommPhaseField x) {
    CThostFtdcCommPhaseField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.CommPhaseNo = x.CommPhaseNo;
    strcpy(y.SystemID, x.SystemID.c_str());
    return y;
}

CommPhaseField Converter::CThostFtdcCommPhaseFieldToRust(CThostFtdcCommPhaseField* x) {
    if (x == nullptr)
        return CommPhaseField{.is_null = true};
    CommPhaseField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.CommPhaseNo = x->CommPhaseNo;
    y.SystemID = Converter::Gb2312ToRustString(x->SystemID);
    return y;
}

CThostFtdcLoginInfoField Converter::LoginInfoFieldToCpp(LoginInfoField x) {
    CThostFtdcLoginInfoField y;
    memset(&y, 0, sizeof(y));
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.LoginDate, x.LoginDate.c_str());
    strcpy(y.LoginTime, x.LoginTime.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.SystemName, x.SystemName.c_str());
    memcpy(y.PasswordDeprecated, x.PasswordDeprecated.data(), x.PasswordDeprecated.size() * sizeof(uint8_t));
    strcpy(y.MaxOrderRef, x.MaxOrderRef.c_str());
    strcpy(y.SHFETime, x.SHFETime.c_str());
    strcpy(y.DCETime, x.DCETime.c_str());
    strcpy(y.CZCETime, x.CZCETime.c_str());
    strcpy(y.FFEXTime, x.FFEXTime.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.OneTimePassword, x.OneTimePassword.c_str());
    strcpy(y.INETime, x.INETime.c_str());
    y.IsQryControl = x.IsQryControl;
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

LoginInfoField Converter::CThostFtdcLoginInfoFieldToRust(CThostFtdcLoginInfoField* x) {
    if (x == nullptr)
        return LoginInfoField{.is_null = true};
    LoginInfoField y{};
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.LoginDate = Converter::Gb2312ToRustString(x->LoginDate);
    y.LoginTime = Converter::Gb2312ToRustString(x->LoginTime);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.SystemName = Converter::Gb2312ToRustString(x->SystemName);
    for (int i = 0; i < 41; i++)
        y.PasswordDeprecated.push_back(x->PasswordDeprecated[i]);
    y.MaxOrderRef = Converter::Gb2312ToRustString(x->MaxOrderRef);
    y.SHFETime = Converter::Gb2312ToRustString(x->SHFETime);
    y.DCETime = Converter::Gb2312ToRustString(x->DCETime);
    y.CZCETime = Converter::Gb2312ToRustString(x->CZCETime);
    y.FFEXTime = Converter::Gb2312ToRustString(x->FFEXTime);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.OneTimePassword = Converter::Gb2312ToRustString(x->OneTimePassword);
    y.INETime = Converter::Gb2312ToRustString(x->INETime);
    y.IsQryControl = x->IsQryControl;
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcLogoutAllField Converter::LogoutAllFieldToCpp(LogoutAllField x) {
    CThostFtdcLogoutAllField y;
    memset(&y, 0, sizeof(y));
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.SystemName, x.SystemName.c_str());
    return y;
}

LogoutAllField Converter::CThostFtdcLogoutAllFieldToRust(CThostFtdcLogoutAllField* x) {
    if (x == nullptr)
        return LogoutAllField{.is_null = true};
    LogoutAllField y{};
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.SystemName = Converter::Gb2312ToRustString(x->SystemName);
    return y;
}

CThostFtdcFrontStatusField Converter::FrontStatusFieldToCpp(FrontStatusField x) {
    CThostFtdcFrontStatusField y;
    memset(&y, 0, sizeof(y));
    y.FrontID = x.FrontID;
    strcpy(y.LastReportDate, x.LastReportDate.c_str());
    strcpy(y.LastReportTime, x.LastReportTime.c_str());
    y.IsActive = x.IsActive;
    return y;
}

FrontStatusField Converter::CThostFtdcFrontStatusFieldToRust(CThostFtdcFrontStatusField* x) {
    if (x == nullptr)
        return FrontStatusField{.is_null = true};
    FrontStatusField y{};
    y.FrontID = x->FrontID;
    y.LastReportDate = Converter::Gb2312ToRustString(x->LastReportDate);
    y.LastReportTime = Converter::Gb2312ToRustString(x->LastReportTime);
    y.IsActive = x->IsActive;
    return y;
}

CThostFtdcUserPasswordUpdateField Converter::UserPasswordUpdateFieldToCpp(UserPasswordUpdateField x) {
    CThostFtdcUserPasswordUpdateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.OldPassword, x.OldPassword.c_str());
    strcpy(y.NewPassword, x.NewPassword.c_str());
    return y;
}

UserPasswordUpdateField Converter::CThostFtdcUserPasswordUpdateFieldToRust(CThostFtdcUserPasswordUpdateField* x) {
    if (x == nullptr)
        return UserPasswordUpdateField{.is_null = true};
    UserPasswordUpdateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OldPassword = Converter::Gb2312ToRustString(x->OldPassword);
    y.NewPassword = Converter::Gb2312ToRustString(x->NewPassword);
    return y;
}

CThostFtdcInputOrderField Converter::InputOrderFieldToCpp(InputOrderField x) {
    CThostFtdcInputOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OrderRef, x.OrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.OrderPriceType = x.OrderPriceType;
    y.Direction = x.Direction;
    strcpy(y.CombOffsetFlag, x.CombOffsetFlag.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.LimitPrice = x.LimitPrice;
    y.VolumeTotalOriginal = x.VolumeTotalOriginal;
    y.TimeCondition = x.TimeCondition;
    strcpy(y.GTDDate, x.GTDDate.c_str());
    y.VolumeCondition = x.VolumeCondition;
    y.MinVolume = x.MinVolume;
    y.ContingentCondition = x.ContingentCondition;
    y.StopPrice = x.StopPrice;
    y.ForceCloseReason = x.ForceCloseReason;
    y.IsAutoSuspend = x.IsAutoSuspend;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.RequestID = x.RequestID;
    y.UserForceClose = x.UserForceClose;
    y.IsSwapOrder = x.IsSwapOrder;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

InputOrderField Converter::CThostFtdcInputOrderFieldToRust(CThostFtdcInputOrderField* x) {
    if (x == nullptr)
        return InputOrderField{.is_null = true};
    InputOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OrderPriceType = x->OrderPriceType;
    y.Direction = x->Direction;
    y.CombOffsetFlag = Converter::Gb2312ToRustString(x->CombOffsetFlag);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.LimitPrice = x->LimitPrice;
    y.VolumeTotalOriginal = x->VolumeTotalOriginal;
    y.TimeCondition = x->TimeCondition;
    y.GTDDate = Converter::Gb2312ToRustString(x->GTDDate);
    y.VolumeCondition = x->VolumeCondition;
    y.MinVolume = x->MinVolume;
    y.ContingentCondition = x->ContingentCondition;
    y.StopPrice = x->StopPrice;
    y.ForceCloseReason = x->ForceCloseReason;
    y.IsAutoSuspend = x->IsAutoSuspend;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.RequestID = x->RequestID;
    y.UserForceClose = x->UserForceClose;
    y.IsSwapOrder = x->IsSwapOrder;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcOrderField Converter::OrderFieldToCpp(OrderField x) {
    CThostFtdcOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OrderRef, x.OrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.OrderPriceType = x.OrderPriceType;
    y.Direction = x.Direction;
    strcpy(y.CombOffsetFlag, x.CombOffsetFlag.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.LimitPrice = x.LimitPrice;
    y.VolumeTotalOriginal = x.VolumeTotalOriginal;
    y.TimeCondition = x.TimeCondition;
    strcpy(y.GTDDate, x.GTDDate.c_str());
    y.VolumeCondition = x.VolumeCondition;
    y.MinVolume = x.MinVolume;
    y.ContingentCondition = x.ContingentCondition;
    y.StopPrice = x.StopPrice;
    y.ForceCloseReason = x.ForceCloseReason;
    y.IsAutoSuspend = x.IsAutoSuspend;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.RequestID = x.RequestID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.OrderSource = x.OrderSource;
    y.OrderStatus = x.OrderStatus;
    y.OrderType = x.OrderType;
    y.VolumeTraded = x.VolumeTraded;
    y.VolumeTotal = x.VolumeTotal;
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.ActiveTime, x.ActiveTime.c_str());
    strcpy(y.SuspendTime, x.SuspendTime.c_str());
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    strcpy(y.ActiveTraderID, x.ActiveTraderID.c_str());
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    y.UserForceClose = x.UserForceClose;
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerOrderSeq = x.BrokerOrderSeq;
    strcpy(y.RelativeOrderSysID, x.RelativeOrderSysID.c_str());
    y.ZCETotalTradedVolume = x.ZCETotalTradedVolume;
    y.IsSwapOrder = x.IsSwapOrder;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

OrderField Converter::CThostFtdcOrderFieldToRust(CThostFtdcOrderField* x) {
    if (x == nullptr)
        return OrderField{.is_null = true};
    OrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OrderPriceType = x->OrderPriceType;
    y.Direction = x->Direction;
    y.CombOffsetFlag = Converter::Gb2312ToRustString(x->CombOffsetFlag);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.LimitPrice = x->LimitPrice;
    y.VolumeTotalOriginal = x->VolumeTotalOriginal;
    y.TimeCondition = x->TimeCondition;
    y.GTDDate = Converter::Gb2312ToRustString(x->GTDDate);
    y.VolumeCondition = x->VolumeCondition;
    y.MinVolume = x->MinVolume;
    y.ContingentCondition = x->ContingentCondition;
    y.StopPrice = x->StopPrice;
    y.ForceCloseReason = x->ForceCloseReason;
    y.IsAutoSuspend = x->IsAutoSuspend;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.RequestID = x->RequestID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.OrderSource = x->OrderSource;
    y.OrderStatus = x->OrderStatus;
    y.OrderType = x->OrderType;
    y.VolumeTraded = x->VolumeTraded;
    y.VolumeTotal = x->VolumeTotal;
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.ActiveTime = Converter::Gb2312ToRustString(x->ActiveTime);
    y.SuspendTime = Converter::Gb2312ToRustString(x->SuspendTime);
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ActiveTraderID = Converter::Gb2312ToRustString(x->ActiveTraderID);
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.UserForceClose = x->UserForceClose;
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerOrderSeq = x->BrokerOrderSeq;
    y.RelativeOrderSysID = Converter::Gb2312ToRustString(x->RelativeOrderSysID);
    y.ZCETotalTradedVolume = x->ZCETotalTradedVolume;
    y.IsSwapOrder = x->IsSwapOrder;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcExchangeOrderField Converter::ExchangeOrderFieldToCpp(ExchangeOrderField x) {
    CThostFtdcExchangeOrderField y;
    memset(&y, 0, sizeof(y));
    y.OrderPriceType = x.OrderPriceType;
    y.Direction = x.Direction;
    strcpy(y.CombOffsetFlag, x.CombOffsetFlag.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.LimitPrice = x.LimitPrice;
    y.VolumeTotalOriginal = x.VolumeTotalOriginal;
    y.TimeCondition = x.TimeCondition;
    strcpy(y.GTDDate, x.GTDDate.c_str());
    y.VolumeCondition = x.VolumeCondition;
    y.MinVolume = x.MinVolume;
    y.ContingentCondition = x.ContingentCondition;
    y.StopPrice = x.StopPrice;
    y.ForceCloseReason = x.ForceCloseReason;
    y.IsAutoSuspend = x.IsAutoSuspend;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.RequestID = x.RequestID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.OrderSource = x.OrderSource;
    y.OrderStatus = x.OrderStatus;
    y.OrderType = x.OrderType;
    y.VolumeTraded = x.VolumeTraded;
    y.VolumeTotal = x.VolumeTotal;
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.ActiveTime, x.ActiveTime.c_str());
    strcpy(y.SuspendTime, x.SuspendTime.c_str());
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    strcpy(y.ActiveTraderID, x.ActiveTraderID.c_str());
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeOrderField Converter::CThostFtdcExchangeOrderFieldToRust(CThostFtdcExchangeOrderField* x) {
    if (x == nullptr)
        return ExchangeOrderField{.is_null = true};
    ExchangeOrderField y{};
    y.OrderPriceType = x->OrderPriceType;
    y.Direction = x->Direction;
    y.CombOffsetFlag = Converter::Gb2312ToRustString(x->CombOffsetFlag);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.LimitPrice = x->LimitPrice;
    y.VolumeTotalOriginal = x->VolumeTotalOriginal;
    y.TimeCondition = x->TimeCondition;
    y.GTDDate = Converter::Gb2312ToRustString(x->GTDDate);
    y.VolumeCondition = x->VolumeCondition;
    y.MinVolume = x->MinVolume;
    y.ContingentCondition = x->ContingentCondition;
    y.StopPrice = x->StopPrice;
    y.ForceCloseReason = x->ForceCloseReason;
    y.IsAutoSuspend = x->IsAutoSuspend;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.RequestID = x->RequestID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.OrderSource = x->OrderSource;
    y.OrderStatus = x->OrderStatus;
    y.OrderType = x->OrderType;
    y.VolumeTraded = x->VolumeTraded;
    y.VolumeTotal = x->VolumeTotal;
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.ActiveTime = Converter::Gb2312ToRustString(x->ActiveTime);
    y.SuspendTime = Converter::Gb2312ToRustString(x->SuspendTime);
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ActiveTraderID = Converter::Gb2312ToRustString(x->ActiveTraderID);
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcExchangeOrderInsertErrorField Converter::ExchangeOrderInsertErrorFieldToCpp(ExchangeOrderInsertErrorField x) {
    CThostFtdcExchangeOrderInsertErrorField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

ExchangeOrderInsertErrorField Converter::CThostFtdcExchangeOrderInsertErrorFieldToRust(CThostFtdcExchangeOrderInsertErrorField* x) {
    if (x == nullptr)
        return ExchangeOrderInsertErrorField{.is_null = true};
    ExchangeOrderInsertErrorField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcInputOrderActionField Converter::InputOrderActionFieldToCpp(InputOrderActionField x) {
    CThostFtdcInputOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OrderActionRef = x.OrderActionRef;
    strcpy(y.OrderRef, x.OrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    y.LimitPrice = x.LimitPrice;
    y.VolumeChange = x.VolumeChange;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

InputOrderActionField Converter::CThostFtdcInputOrderActionFieldToRust(CThostFtdcInputOrderActionField* x) {
    if (x == nullptr)
        return InputOrderActionField{.is_null = true};
    InputOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderActionRef = x->OrderActionRef;
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.LimitPrice = x->LimitPrice;
    y.VolumeChange = x->VolumeChange;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcOrderActionField Converter::OrderActionFieldToCpp(OrderActionField x) {
    CThostFtdcOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OrderActionRef = x.OrderActionRef;
    strcpy(y.OrderRef, x.OrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    y.LimitPrice = x.LimitPrice;
    y.VolumeChange = x.VolumeChange;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

OrderActionField Converter::CThostFtdcOrderActionFieldToRust(CThostFtdcOrderActionField* x) {
    if (x == nullptr)
        return OrderActionField{.is_null = true};
    OrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderActionRef = x->OrderActionRef;
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.LimitPrice = x->LimitPrice;
    y.VolumeChange = x->VolumeChange;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcExchangeOrderActionField Converter::ExchangeOrderActionFieldToCpp(ExchangeOrderActionField x) {
    CThostFtdcExchangeOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    y.LimitPrice = x.LimitPrice;
    y.VolumeChange = x.VolumeChange;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeOrderActionField Converter::CThostFtdcExchangeOrderActionFieldToRust(CThostFtdcExchangeOrderActionField* x) {
    if (x == nullptr)
        return ExchangeOrderActionField{.is_null = true};
    ExchangeOrderActionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.LimitPrice = x->LimitPrice;
    y.VolumeChange = x->VolumeChange;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcExchangeOrderActionErrorField Converter::ExchangeOrderActionErrorFieldToCpp(ExchangeOrderActionErrorField x) {
    CThostFtdcExchangeOrderActionErrorField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

ExchangeOrderActionErrorField Converter::CThostFtdcExchangeOrderActionErrorFieldToRust(CThostFtdcExchangeOrderActionErrorField* x) {
    if (x == nullptr)
        return ExchangeOrderActionErrorField{.is_null = true};
    ExchangeOrderActionErrorField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcExchangeTradeField Converter::ExchangeTradeFieldToCpp(ExchangeTradeField x) {
    CThostFtdcExchangeTradeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    y.Direction = x.Direction;
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    y.TradingRole = x.TradingRole;
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.Price = x.Price;
    y.Volume = x.Volume;
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    y.TradeType = x.TradeType;
    y.PriceSource = x.PriceSource;
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.SequenceNo = x.SequenceNo;
    y.TradeSource = x.TradeSource;
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

ExchangeTradeField Converter::CThostFtdcExchangeTradeFieldToRust(CThostFtdcExchangeTradeField* x) {
    if (x == nullptr)
        return ExchangeTradeField{.is_null = true};
    ExchangeTradeField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.Direction = x->Direction;
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TradingRole = x->TradingRole;
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.Price = x->Price;
    y.Volume = x->Volume;
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.TradeType = x->TradeType;
    y.PriceSource = x->PriceSource;
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.SequenceNo = x->SequenceNo;
    y.TradeSource = x->TradeSource;
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcTradeField Converter::TradeFieldToCpp(TradeField x) {
    CThostFtdcTradeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OrderRef, x.OrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    y.Direction = x.Direction;
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    y.TradingRole = x.TradingRole;
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.Price = x.Price;
    y.Volume = x.Volume;
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    y.TradeType = x.TradeType;
    y.PriceSource = x.PriceSource;
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.SequenceNo = x.SequenceNo;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.BrokerOrderSeq = x.BrokerOrderSeq;
    y.TradeSource = x.TradeSource;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

TradeField Converter::CThostFtdcTradeFieldToRust(CThostFtdcTradeField* x) {
    if (x == nullptr)
        return TradeField{.is_null = true};
    TradeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.Direction = x->Direction;
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TradingRole = x->TradingRole;
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.Price = x->Price;
    y.Volume = x->Volume;
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.TradeType = x->TradeType;
    y.PriceSource = x->PriceSource;
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.SequenceNo = x->SequenceNo;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.BrokerOrderSeq = x->BrokerOrderSeq;
    y.TradeSource = x->TradeSource;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcUserSessionField Converter::UserSessionFieldToCpp(UserSessionField x) {
    CThostFtdcUserSessionField y;
    memset(&y, 0, sizeof(y));
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.LoginDate, x.LoginDate.c_str());
    strcpy(y.LoginTime, x.LoginTime.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

UserSessionField Converter::CThostFtdcUserSessionFieldToRust(CThostFtdcUserSessionField* x) {
    if (x == nullptr)
        return UserSessionField{.is_null = true};
    UserSessionField y{};
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.LoginDate = Converter::Gb2312ToRustString(x->LoginDate);
    y.LoginTime = Converter::Gb2312ToRustString(x->LoginTime);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryMaxOrderVolumeField Converter::QryMaxOrderVolumeFieldToCpp(QryMaxOrderVolumeField x) {
    CThostFtdcQryMaxOrderVolumeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Direction = x.Direction;
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.MaxVolume = x.MaxVolume;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryMaxOrderVolumeField Converter::CThostFtdcQryMaxOrderVolumeFieldToRust(CThostFtdcQryMaxOrderVolumeField* x) {
    if (x == nullptr)
        return QryMaxOrderVolumeField{.is_null = true};
    QryMaxOrderVolumeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Direction = x->Direction;
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.MaxVolume = x->MaxVolume;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcSettlementInfoConfirmField Converter::SettlementInfoConfirmFieldToCpp(SettlementInfoConfirmField x) {
    CThostFtdcSettlementInfoConfirmField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ConfirmDate, x.ConfirmDate.c_str());
    strcpy(y.ConfirmTime, x.ConfirmTime.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

SettlementInfoConfirmField Converter::CThostFtdcSettlementInfoConfirmFieldToRust(CThostFtdcSettlementInfoConfirmField* x) {
    if (x == nullptr)
        return SettlementInfoConfirmField{.is_null = true};
    SettlementInfoConfirmField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ConfirmDate = Converter::Gb2312ToRustString(x->ConfirmDate);
    y.ConfirmTime = Converter::Gb2312ToRustString(x->ConfirmTime);
    y.SettlementID = x->SettlementID;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcSyncDepositField Converter::SyncDepositFieldToCpp(SyncDepositField x) {
    CThostFtdcSyncDepositField y;
    memset(&y, 0, sizeof(y));
    memcpy(y.DepositSeqNo, x.DepositSeqNo.data(), x.DepositSeqNo.size() * sizeof(uint8_t));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Deposit = x.Deposit;
    y.IsForce = x.IsForce;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.IsFromSopt = x.IsFromSopt;
    strcpy(y.TradingPassword, x.TradingPassword.c_str());
    y.IsSecAgentTranfer = x.IsSecAgentTranfer;
    return y;
}

SyncDepositField Converter::CThostFtdcSyncDepositFieldToRust(CThostFtdcSyncDepositField* x) {
    if (x == nullptr)
        return SyncDepositField{.is_null = true};
    SyncDepositField y{};
    for (int i = 0; i < 15; i++)
        y.DepositSeqNo.push_back(x->DepositSeqNo[i]);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Deposit = x->Deposit;
    y.IsForce = x->IsForce;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.IsFromSopt = x->IsFromSopt;
    y.TradingPassword = Converter::Gb2312ToRustString(x->TradingPassword);
    y.IsSecAgentTranfer = x->IsSecAgentTranfer;
    return y;
}

CThostFtdcSyncFundMortgageField Converter::SyncFundMortgageFieldToCpp(SyncFundMortgageField x) {
    CThostFtdcSyncFundMortgageField y;
    memset(&y, 0, sizeof(y));
    memcpy(y.MortgageSeqNo, x.MortgageSeqNo.data(), x.MortgageSeqNo.size() * sizeof(uint8_t));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.FromCurrencyID, x.FromCurrencyID.c_str());
    y.MortgageAmount = x.MortgageAmount;
    strcpy(y.ToCurrencyID, x.ToCurrencyID.c_str());
    return y;
}

SyncFundMortgageField Converter::CThostFtdcSyncFundMortgageFieldToRust(CThostFtdcSyncFundMortgageField* x) {
    if (x == nullptr)
        return SyncFundMortgageField{.is_null = true};
    SyncFundMortgageField y{};
    for (int i = 0; i < 15; i++)
        y.MortgageSeqNo.push_back(x->MortgageSeqNo[i]);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.FromCurrencyID = Converter::Gb2312ToRustString(x->FromCurrencyID);
    y.MortgageAmount = x->MortgageAmount;
    y.ToCurrencyID = Converter::Gb2312ToRustString(x->ToCurrencyID);
    return y;
}

CThostFtdcBrokerSyncField Converter::BrokerSyncFieldToCpp(BrokerSyncField x) {
    CThostFtdcBrokerSyncField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

BrokerSyncField Converter::CThostFtdcBrokerSyncFieldToRust(CThostFtdcBrokerSyncField* x) {
    if (x == nullptr)
        return BrokerSyncField{.is_null = true};
    BrokerSyncField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcSyncingInvestorField Converter::SyncingInvestorFieldToCpp(SyncingInvestorField x) {
    CThostFtdcSyncingInvestorField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorGroupID, x.InvestorGroupID.c_str());
    strcpy(y.InvestorName, x.InvestorName.c_str());
    y.IdentifiedCardType = x.IdentifiedCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.IsActive = x.IsActive;
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.Mobile, x.Mobile.c_str());
    strcpy(y.CommModelID, x.CommModelID.c_str());
    strcpy(y.MarginModelID, x.MarginModelID.c_str());
    y.IsOrderFreq = x.IsOrderFreq;
    y.IsOpenVolLimit = x.IsOpenVolLimit;
    return y;
}

SyncingInvestorField Converter::CThostFtdcSyncingInvestorFieldToRust(CThostFtdcSyncingInvestorField* x) {
    if (x == nullptr)
        return SyncingInvestorField{.is_null = true};
    SyncingInvestorField y{};
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorGroupID = Converter::Gb2312ToRustString(x->InvestorGroupID);
    y.InvestorName = Converter::Gb2312ToRustString(x->InvestorName);
    y.IdentifiedCardType = x->IdentifiedCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.IsActive = x->IsActive;
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.Mobile = Converter::Gb2312ToRustString(x->Mobile);
    y.CommModelID = Converter::Gb2312ToRustString(x->CommModelID);
    y.MarginModelID = Converter::Gb2312ToRustString(x->MarginModelID);
    y.IsOrderFreq = x->IsOrderFreq;
    y.IsOpenVolLimit = x->IsOpenVolLimit;
    return y;
}

CThostFtdcSyncingTradingCodeField Converter::SyncingTradingCodeFieldToCpp(SyncingTradingCodeField x) {
    CThostFtdcSyncingTradingCodeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    y.IsActive = x.IsActive;
    y.ClientIDType = x.ClientIDType;
    return y;
}

SyncingTradingCodeField Converter::CThostFtdcSyncingTradingCodeFieldToRust(CThostFtdcSyncingTradingCodeField* x) {
    if (x == nullptr)
        return SyncingTradingCodeField{.is_null = true};
    SyncingTradingCodeField y{};
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.IsActive = x->IsActive;
    y.ClientIDType = x->ClientIDType;
    return y;
}

CThostFtdcSyncingInvestorGroupField Converter::SyncingInvestorGroupFieldToCpp(SyncingInvestorGroupField x) {
    CThostFtdcSyncingInvestorGroupField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorGroupID, x.InvestorGroupID.c_str());
    strcpy(y.InvestorGroupName, x.InvestorGroupName.c_str());
    return y;
}

SyncingInvestorGroupField Converter::CThostFtdcSyncingInvestorGroupFieldToRust(CThostFtdcSyncingInvestorGroupField* x) {
    if (x == nullptr)
        return SyncingInvestorGroupField{.is_null = true};
    SyncingInvestorGroupField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorGroupID = Converter::Gb2312ToRustString(x->InvestorGroupID);
    y.InvestorGroupName = Converter::Gb2312ToRustString(x->InvestorGroupName);
    return y;
}

CThostFtdcSyncingTradingAccountField Converter::SyncingTradingAccountFieldToCpp(SyncingTradingAccountField x) {
    CThostFtdcSyncingTradingAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.PreMortgage = x.PreMortgage;
    y.PreCredit = x.PreCredit;
    y.PreDeposit = x.PreDeposit;
    y.PreBalance = x.PreBalance;
    y.PreMargin = x.PreMargin;
    y.InterestBase = x.InterestBase;
    y.Interest = x.Interest;
    y.Deposit = x.Deposit;
    y.Withdraw = x.Withdraw;
    y.FrozenMargin = x.FrozenMargin;
    y.FrozenCash = x.FrozenCash;
    y.FrozenCommission = x.FrozenCommission;
    y.CurrMargin = x.CurrMargin;
    y.CashIn = x.CashIn;
    y.Commission = x.Commission;
    y.CloseProfit = x.CloseProfit;
    y.PositionProfit = x.PositionProfit;
    y.Balance = x.Balance;
    y.Available = x.Available;
    y.WithdrawQuota = x.WithdrawQuota;
    y.Reserve = x.Reserve;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.Credit = x.Credit;
    y.Mortgage = x.Mortgage;
    y.ExchangeMargin = x.ExchangeMargin;
    y.DeliveryMargin = x.DeliveryMargin;
    y.ExchangeDeliveryMargin = x.ExchangeDeliveryMargin;
    y.ReserveBalance = x.ReserveBalance;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.PreFundMortgageIn = x.PreFundMortgageIn;
    y.PreFundMortgageOut = x.PreFundMortgageOut;
    y.FundMortgageIn = x.FundMortgageIn;
    y.FundMortgageOut = x.FundMortgageOut;
    y.FundMortgageAvailable = x.FundMortgageAvailable;
    y.MortgageableFund = x.MortgageableFund;
    y.SpecProductMargin = x.SpecProductMargin;
    y.SpecProductFrozenMargin = x.SpecProductFrozenMargin;
    y.SpecProductCommission = x.SpecProductCommission;
    y.SpecProductFrozenCommission = x.SpecProductFrozenCommission;
    y.SpecProductPositionProfit = x.SpecProductPositionProfit;
    y.SpecProductCloseProfit = x.SpecProductCloseProfit;
    y.SpecProductPositionProfitByAlg = x.SpecProductPositionProfitByAlg;
    y.SpecProductExchangeMargin = x.SpecProductExchangeMargin;
    y.FrozenSwap = x.FrozenSwap;
    y.RemainSwap = x.RemainSwap;
    y.OptionValue = x.OptionValue;
    return y;
}

SyncingTradingAccountField Converter::CThostFtdcSyncingTradingAccountFieldToRust(CThostFtdcSyncingTradingAccountField* x) {
    if (x == nullptr)
        return SyncingTradingAccountField{.is_null = true};
    SyncingTradingAccountField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.PreMortgage = x->PreMortgage;
    y.PreCredit = x->PreCredit;
    y.PreDeposit = x->PreDeposit;
    y.PreBalance = x->PreBalance;
    y.PreMargin = x->PreMargin;
    y.InterestBase = x->InterestBase;
    y.Interest = x->Interest;
    y.Deposit = x->Deposit;
    y.Withdraw = x->Withdraw;
    y.FrozenMargin = x->FrozenMargin;
    y.FrozenCash = x->FrozenCash;
    y.FrozenCommission = x->FrozenCommission;
    y.CurrMargin = x->CurrMargin;
    y.CashIn = x->CashIn;
    y.Commission = x->Commission;
    y.CloseProfit = x->CloseProfit;
    y.PositionProfit = x->PositionProfit;
    y.Balance = x->Balance;
    y.Available = x->Available;
    y.WithdrawQuota = x->WithdrawQuota;
    y.Reserve = x->Reserve;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.Credit = x->Credit;
    y.Mortgage = x->Mortgage;
    y.ExchangeMargin = x->ExchangeMargin;
    y.DeliveryMargin = x->DeliveryMargin;
    y.ExchangeDeliveryMargin = x->ExchangeDeliveryMargin;
    y.ReserveBalance = x->ReserveBalance;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.PreFundMortgageIn = x->PreFundMortgageIn;
    y.PreFundMortgageOut = x->PreFundMortgageOut;
    y.FundMortgageIn = x->FundMortgageIn;
    y.FundMortgageOut = x->FundMortgageOut;
    y.FundMortgageAvailable = x->FundMortgageAvailable;
    y.MortgageableFund = x->MortgageableFund;
    y.SpecProductMargin = x->SpecProductMargin;
    y.SpecProductFrozenMargin = x->SpecProductFrozenMargin;
    y.SpecProductCommission = x->SpecProductCommission;
    y.SpecProductFrozenCommission = x->SpecProductFrozenCommission;
    y.SpecProductPositionProfit = x->SpecProductPositionProfit;
    y.SpecProductCloseProfit = x->SpecProductCloseProfit;
    y.SpecProductPositionProfitByAlg = x->SpecProductPositionProfitByAlg;
    y.SpecProductExchangeMargin = x->SpecProductExchangeMargin;
    y.FrozenSwap = x->FrozenSwap;
    y.RemainSwap = x->RemainSwap;
    y.OptionValue = x->OptionValue;
    return y;
}

CThostFtdcSyncingInvestorPositionField Converter::SyncingInvestorPositionFieldToCpp(SyncingInvestorPositionField x) {
    CThostFtdcSyncingInvestorPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.PosiDirection = x.PosiDirection;
    y.HedgeFlag = x.HedgeFlag;
    y.PositionDate = x.PositionDate;
    y.YdPosition = x.YdPosition;
    y.Position = x.Position;
    y.LongFrozen = x.LongFrozen;
    y.ShortFrozen = x.ShortFrozen;
    y.LongFrozenAmount = x.LongFrozenAmount;
    y.ShortFrozenAmount = x.ShortFrozenAmount;
    y.OpenVolume = x.OpenVolume;
    y.CloseVolume = x.CloseVolume;
    y.OpenAmount = x.OpenAmount;
    y.CloseAmount = x.CloseAmount;
    y.PositionCost = x.PositionCost;
    y.PreMargin = x.PreMargin;
    y.UseMargin = x.UseMargin;
    y.FrozenMargin = x.FrozenMargin;
    y.FrozenCash = x.FrozenCash;
    y.FrozenCommission = x.FrozenCommission;
    y.CashIn = x.CashIn;
    y.Commission = x.Commission;
    y.CloseProfit = x.CloseProfit;
    y.PositionProfit = x.PositionProfit;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.SettlementPrice = x.SettlementPrice;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.OpenCost = x.OpenCost;
    y.ExchangeMargin = x.ExchangeMargin;
    y.CombPosition = x.CombPosition;
    y.CombLongFrozen = x.CombLongFrozen;
    y.CombShortFrozen = x.CombShortFrozen;
    y.CloseProfitByDate = x.CloseProfitByDate;
    y.CloseProfitByTrade = x.CloseProfitByTrade;
    y.TodayPosition = x.TodayPosition;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.StrikeFrozen = x.StrikeFrozen;
    y.StrikeFrozenAmount = x.StrikeFrozenAmount;
    y.AbandonFrozen = x.AbandonFrozen;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.YdStrikeFrozen = x.YdStrikeFrozen;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    y.PositionCostOffset = x.PositionCostOffset;
    y.TasPosition = x.TasPosition;
    y.TasPositionCost = x.TasPositionCost;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

SyncingInvestorPositionField Converter::CThostFtdcSyncingInvestorPositionFieldToRust(CThostFtdcSyncingInvestorPositionField* x) {
    if (x == nullptr)
        return SyncingInvestorPositionField{.is_null = true};
    SyncingInvestorPositionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.PosiDirection = x->PosiDirection;
    y.HedgeFlag = x->HedgeFlag;
    y.PositionDate = x->PositionDate;
    y.YdPosition = x->YdPosition;
    y.Position = x->Position;
    y.LongFrozen = x->LongFrozen;
    y.ShortFrozen = x->ShortFrozen;
    y.LongFrozenAmount = x->LongFrozenAmount;
    y.ShortFrozenAmount = x->ShortFrozenAmount;
    y.OpenVolume = x->OpenVolume;
    y.CloseVolume = x->CloseVolume;
    y.OpenAmount = x->OpenAmount;
    y.CloseAmount = x->CloseAmount;
    y.PositionCost = x->PositionCost;
    y.PreMargin = x->PreMargin;
    y.UseMargin = x->UseMargin;
    y.FrozenMargin = x->FrozenMargin;
    y.FrozenCash = x->FrozenCash;
    y.FrozenCommission = x->FrozenCommission;
    y.CashIn = x->CashIn;
    y.Commission = x->Commission;
    y.CloseProfit = x->CloseProfit;
    y.PositionProfit = x->PositionProfit;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.SettlementPrice = x->SettlementPrice;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OpenCost = x->OpenCost;
    y.ExchangeMargin = x->ExchangeMargin;
    y.CombPosition = x->CombPosition;
    y.CombLongFrozen = x->CombLongFrozen;
    y.CombShortFrozen = x->CombShortFrozen;
    y.CloseProfitByDate = x->CloseProfitByDate;
    y.CloseProfitByTrade = x->CloseProfitByTrade;
    y.TodayPosition = x->TodayPosition;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.StrikeFrozen = x->StrikeFrozen;
    y.StrikeFrozenAmount = x->StrikeFrozenAmount;
    y.AbandonFrozen = x->AbandonFrozen;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.YdStrikeFrozen = x->YdStrikeFrozen;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.PositionCostOffset = x->PositionCostOffset;
    y.TasPosition = x->TasPosition;
    y.TasPositionCost = x->TasPositionCost;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcSyncingInstrumentMarginRateField Converter::SyncingInstrumentMarginRateFieldToCpp(SyncingInstrumentMarginRateField x) {
    CThostFtdcSyncingInstrumentMarginRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.IsRelative = x.IsRelative;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

SyncingInstrumentMarginRateField Converter::CThostFtdcSyncingInstrumentMarginRateFieldToRust(CThostFtdcSyncingInstrumentMarginRateField* x) {
    if (x == nullptr)
        return SyncingInstrumentMarginRateField{.is_null = true};
    SyncingInstrumentMarginRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.IsRelative = x->IsRelative;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcSyncingInstrumentCommissionRateField Converter::SyncingInstrumentCommissionRateFieldToCpp(SyncingInstrumentCommissionRateField x) {
    CThostFtdcSyncingInstrumentCommissionRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

SyncingInstrumentCommissionRateField Converter::CThostFtdcSyncingInstrumentCommissionRateFieldToRust(CThostFtdcSyncingInstrumentCommissionRateField* x) {
    if (x == nullptr)
        return SyncingInstrumentCommissionRateField{.is_null = true};
    SyncingInstrumentCommissionRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcSyncingInstrumentTradingRightField Converter::SyncingInstrumentTradingRightFieldToCpp(SyncingInstrumentTradingRightField x) {
    CThostFtdcSyncingInstrumentTradingRightField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.TradingRight = x.TradingRight;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

SyncingInstrumentTradingRightField Converter::CThostFtdcSyncingInstrumentTradingRightFieldToRust(CThostFtdcSyncingInstrumentTradingRightField* x) {
    if (x == nullptr)
        return SyncingInstrumentTradingRightField{.is_null = true};
    SyncingInstrumentTradingRightField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.TradingRight = x->TradingRight;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryOrderField Converter::QryOrderFieldToCpp(QryOrderField x) {
    CThostFtdcQryOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    strcpy(y.InsertTimeStart, x.InsertTimeStart.c_str());
    strcpy(y.InsertTimeEnd, x.InsertTimeEnd.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryOrderField Converter::CThostFtdcQryOrderFieldToRust(CThostFtdcQryOrderField* x) {
    if (x == nullptr)
        return QryOrderField{.is_null = true};
    QryOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.InsertTimeStart = Converter::Gb2312ToRustString(x->InsertTimeStart);
    y.InsertTimeEnd = Converter::Gb2312ToRustString(x->InsertTimeEnd);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryTradeField Converter::QryTradeFieldToCpp(QryTradeField x) {
    CThostFtdcQryTradeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    strcpy(y.TradeTimeStart, x.TradeTimeStart.c_str());
    strcpy(y.TradeTimeEnd, x.TradeTimeEnd.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryTradeField Converter::CThostFtdcQryTradeFieldToRust(CThostFtdcQryTradeField* x) {
    if (x == nullptr)
        return QryTradeField{.is_null = true};
    QryTradeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.TradeTimeStart = Converter::Gb2312ToRustString(x->TradeTimeStart);
    y.TradeTimeEnd = Converter::Gb2312ToRustString(x->TradeTimeEnd);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryInvestorPositionField Converter::QryInvestorPositionFieldToCpp(QryInvestorPositionField x) {
    CThostFtdcQryInvestorPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryInvestorPositionField Converter::CThostFtdcQryInvestorPositionFieldToRust(CThostFtdcQryInvestorPositionField* x) {
    if (x == nullptr)
        return QryInvestorPositionField{.is_null = true};
    QryInvestorPositionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryTradingAccountField Converter::QryTradingAccountFieldToCpp(QryTradingAccountField x) {
    CThostFtdcQryTradingAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.BizType = x.BizType;
    strcpy(y.AccountID, x.AccountID.c_str());
    return y;
}

QryTradingAccountField Converter::CThostFtdcQryTradingAccountFieldToRust(CThostFtdcQryTradingAccountField* x) {
    if (x == nullptr)
        return QryTradingAccountField{.is_null = true};
    QryTradingAccountField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.BizType = x->BizType;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    return y;
}

CThostFtdcQryInvestorField Converter::QryInvestorFieldToCpp(QryInvestorField x) {
    CThostFtdcQryInvestorField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryInvestorField Converter::CThostFtdcQryInvestorFieldToRust(CThostFtdcQryInvestorField* x) {
    if (x == nullptr)
        return QryInvestorField{.is_null = true};
    QryInvestorField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcQryTradingCodeField Converter::QryTradingCodeFieldToCpp(QryTradingCodeField x) {
    CThostFtdcQryTradingCodeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    y.ClientIDType = x.ClientIDType;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

QryTradingCodeField Converter::CThostFtdcQryTradingCodeFieldToRust(CThostFtdcQryTradingCodeField* x) {
    if (x == nullptr)
        return QryTradingCodeField{.is_null = true};
    QryTradingCodeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ClientIDType = x->ClientIDType;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcQryInvestorGroupField Converter::QryInvestorGroupFieldToCpp(QryInvestorGroupField x) {
    CThostFtdcQryInvestorGroupField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryInvestorGroupField Converter::CThostFtdcQryInvestorGroupFieldToRust(CThostFtdcQryInvestorGroupField* x) {
    if (x == nullptr)
        return QryInvestorGroupField{.is_null = true};
    QryInvestorGroupField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcQryInstrumentMarginRateField Converter::QryInstrumentMarginRateFieldToCpp(QryInstrumentMarginRateField x) {
    CThostFtdcQryInstrumentMarginRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryInstrumentMarginRateField Converter::CThostFtdcQryInstrumentMarginRateFieldToRust(CThostFtdcQryInstrumentMarginRateField* x) {
    if (x == nullptr)
        return QryInstrumentMarginRateField{.is_null = true};
    QryInstrumentMarginRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryInstrumentCommissionRateField Converter::QryInstrumentCommissionRateFieldToCpp(QryInstrumentCommissionRateField x) {
    CThostFtdcQryInstrumentCommissionRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryInstrumentCommissionRateField Converter::CThostFtdcQryInstrumentCommissionRateFieldToRust(CThostFtdcQryInstrumentCommissionRateField* x) {
    if (x == nullptr)
        return QryInstrumentCommissionRateField{.is_null = true};
    QryInstrumentCommissionRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryInstrumentTradingRightField Converter::QryInstrumentTradingRightFieldToCpp(QryInstrumentTradingRightField x) {
    CThostFtdcQryInstrumentTradingRightField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryInstrumentTradingRightField Converter::CThostFtdcQryInstrumentTradingRightFieldToRust(CThostFtdcQryInstrumentTradingRightField* x) {
    if (x == nullptr)
        return QryInstrumentTradingRightField{.is_null = true};
    QryInstrumentTradingRightField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryBrokerField Converter::QryBrokerFieldToCpp(QryBrokerField x) {
    CThostFtdcQryBrokerField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryBrokerField Converter::CThostFtdcQryBrokerFieldToRust(CThostFtdcQryBrokerField* x) {
    if (x == nullptr)
        return QryBrokerField{.is_null = true};
    QryBrokerField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcQryTraderField Converter::QryTraderFieldToCpp(QryTraderField x) {
    CThostFtdcQryTraderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryTraderField Converter::CThostFtdcQryTraderFieldToRust(CThostFtdcQryTraderField* x) {
    if (x == nullptr)
        return QryTraderField{.is_null = true};
    QryTraderField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcQrySuperUserFunctionField Converter::QrySuperUserFunctionFieldToCpp(QrySuperUserFunctionField x) {
    CThostFtdcQrySuperUserFunctionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QrySuperUserFunctionField Converter::CThostFtdcQrySuperUserFunctionFieldToRust(CThostFtdcQrySuperUserFunctionField* x) {
    if (x == nullptr)
        return QrySuperUserFunctionField{.is_null = true};
    QrySuperUserFunctionField y{};
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcQryUserSessionField Converter::QryUserSessionFieldToCpp(QryUserSessionField x) {
    CThostFtdcQryUserSessionField y;
    memset(&y, 0, sizeof(y));
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QryUserSessionField Converter::CThostFtdcQryUserSessionFieldToRust(CThostFtdcQryUserSessionField* x) {
    if (x == nullptr)
        return QryUserSessionField{.is_null = true};
    QryUserSessionField y{};
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcQryPartBrokerField Converter::QryPartBrokerFieldToCpp(QryPartBrokerField x) {
    CThostFtdcQryPartBrokerField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    return y;
}

QryPartBrokerField Converter::CThostFtdcQryPartBrokerFieldToRust(CThostFtdcQryPartBrokerField* x) {
    if (x == nullptr)
        return QryPartBrokerField{.is_null = true};
    QryPartBrokerField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    return y;
}

CThostFtdcQryFrontStatusField Converter::QryFrontStatusFieldToCpp(QryFrontStatusField x) {
    CThostFtdcQryFrontStatusField y;
    memset(&y, 0, sizeof(y));
    y.FrontID = x.FrontID;
    return y;
}

QryFrontStatusField Converter::CThostFtdcQryFrontStatusFieldToRust(CThostFtdcQryFrontStatusField* x) {
    if (x == nullptr)
        return QryFrontStatusField{.is_null = true};
    QryFrontStatusField y{};
    y.FrontID = x->FrontID;
    return y;
}

CThostFtdcQryExchangeOrderField Converter::QryExchangeOrderFieldToCpp(QryExchangeOrderField x) {
    CThostFtdcQryExchangeOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

QryExchangeOrderField Converter::CThostFtdcQryExchangeOrderFieldToRust(CThostFtdcQryExchangeOrderField* x) {
    if (x == nullptr)
        return QryExchangeOrderField{.is_null = true};
    QryExchangeOrderField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcQryOrderActionField Converter::QryOrderActionFieldToCpp(QryOrderActionField x) {
    CThostFtdcQryOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryOrderActionField Converter::CThostFtdcQryOrderActionFieldToRust(CThostFtdcQryOrderActionField* x) {
    if (x == nullptr)
        return QryOrderActionField{.is_null = true};
    QryOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcQryExchangeOrderActionField Converter::QryExchangeOrderActionFieldToCpp(QryExchangeOrderActionField x) {
    CThostFtdcQryExchangeOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryExchangeOrderActionField Converter::CThostFtdcQryExchangeOrderActionFieldToRust(CThostFtdcQryExchangeOrderActionField* x) {
    if (x == nullptr)
        return QryExchangeOrderActionField{.is_null = true};
    QryExchangeOrderActionField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcQrySuperUserField Converter::QrySuperUserFieldToCpp(QrySuperUserField x) {
    CThostFtdcQrySuperUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QrySuperUserField Converter::CThostFtdcQrySuperUserFieldToRust(CThostFtdcQrySuperUserField* x) {
    if (x == nullptr)
        return QrySuperUserField{.is_null = true};
    QrySuperUserField y{};
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcQryExchangeField Converter::QryExchangeFieldToCpp(QryExchangeField x) {
    CThostFtdcQryExchangeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryExchangeField Converter::CThostFtdcQryExchangeFieldToRust(CThostFtdcQryExchangeField* x) {
    if (x == nullptr)
        return QryExchangeField{.is_null = true};
    QryExchangeField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcQryProductField Converter::QryProductFieldToCpp(QryProductField x) {
    CThostFtdcQryProductField y;
    memset(&y, 0, sizeof(y));
    y.ProductClass = x.ProductClass;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QryProductField Converter::CThostFtdcQryProductFieldToRust(CThostFtdcQryProductField* x) {
    if (x == nullptr)
        return QryProductField{.is_null = true};
    QryProductField y{};
    y.ProductClass = x->ProductClass;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcQryInstrumentField Converter::QryInstrumentFieldToCpp(QryInstrumentField x) {
    CThostFtdcQryInstrumentField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QryInstrumentField Converter::CThostFtdcQryInstrumentFieldToRust(CThostFtdcQryInstrumentField* x) {
    if (x == nullptr)
        return QryInstrumentField{.is_null = true};
    QryInstrumentField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcQryDepthMarketDataField Converter::QryDepthMarketDataFieldToCpp(QryDepthMarketDataField x) {
    CThostFtdcQryDepthMarketDataField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.ProductClass = x.ProductClass;
    return y;
}

QryDepthMarketDataField Converter::CThostFtdcQryDepthMarketDataFieldToRust(CThostFtdcQryDepthMarketDataField* x) {
    if (x == nullptr)
        return QryDepthMarketDataField{.is_null = true};
    QryDepthMarketDataField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProductClass = x->ProductClass;
    return y;
}

CThostFtdcQryBrokerUserField Converter::QryBrokerUserFieldToCpp(QryBrokerUserField x) {
    CThostFtdcQryBrokerUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QryBrokerUserField Converter::CThostFtdcQryBrokerUserFieldToRust(CThostFtdcQryBrokerUserField* x) {
    if (x == nullptr)
        return QryBrokerUserField{.is_null = true};
    QryBrokerUserField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcQryBrokerUserFunctionField Converter::QryBrokerUserFunctionFieldToCpp(QryBrokerUserFunctionField x) {
    CThostFtdcQryBrokerUserFunctionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QryBrokerUserFunctionField Converter::CThostFtdcQryBrokerUserFunctionFieldToRust(CThostFtdcQryBrokerUserFunctionField* x) {
    if (x == nullptr)
        return QryBrokerUserFunctionField{.is_null = true};
    QryBrokerUserFunctionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcQryTraderOfferField Converter::QryTraderOfferFieldToCpp(QryTraderOfferField x) {
    CThostFtdcQryTraderOfferField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryTraderOfferField Converter::CThostFtdcQryTraderOfferFieldToRust(CThostFtdcQryTraderOfferField* x) {
    if (x == nullptr)
        return QryTraderOfferField{.is_null = true};
    QryTraderOfferField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcQrySyncDepositField Converter::QrySyncDepositFieldToCpp(QrySyncDepositField x) {
    CThostFtdcQrySyncDepositField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    memcpy(y.DepositSeqNo, x.DepositSeqNo.data(), x.DepositSeqNo.size() * sizeof(uint8_t));
    return y;
}

QrySyncDepositField Converter::CThostFtdcQrySyncDepositFieldToRust(CThostFtdcQrySyncDepositField* x) {
    if (x == nullptr)
        return QrySyncDepositField{.is_null = true};
    QrySyncDepositField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    for (int i = 0; i < 15; i++)
        y.DepositSeqNo.push_back(x->DepositSeqNo[i]);
    return y;
}

CThostFtdcQrySettlementInfoField Converter::QrySettlementInfoFieldToCpp(QrySettlementInfoField x) {
    CThostFtdcQrySettlementInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

QrySettlementInfoField Converter::CThostFtdcQrySettlementInfoFieldToRust(CThostFtdcQrySettlementInfoField* x) {
    if (x == nullptr)
        return QrySettlementInfoField{.is_null = true};
    QrySettlementInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcQryExchangeMarginRateField Converter::QryExchangeMarginRateFieldToCpp(QryExchangeMarginRateField x) {
    CThostFtdcQryExchangeMarginRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryExchangeMarginRateField Converter::CThostFtdcQryExchangeMarginRateFieldToRust(CThostFtdcQryExchangeMarginRateField* x) {
    if (x == nullptr)
        return QryExchangeMarginRateField{.is_null = true};
    QryExchangeMarginRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.HedgeFlag = x->HedgeFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryExchangeMarginRateAdjustField Converter::QryExchangeMarginRateAdjustFieldToCpp(QryExchangeMarginRateAdjustField x) {
    CThostFtdcQryExchangeMarginRateAdjustField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryExchangeMarginRateAdjustField Converter::CThostFtdcQryExchangeMarginRateAdjustFieldToRust(CThostFtdcQryExchangeMarginRateAdjustField* x) {
    if (x == nullptr)
        return QryExchangeMarginRateAdjustField{.is_null = true};
    QryExchangeMarginRateAdjustField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.HedgeFlag = x->HedgeFlag;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryExchangeRateField Converter::QryExchangeRateFieldToCpp(QryExchangeRateField x) {
    CThostFtdcQryExchangeRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.FromCurrencyID, x.FromCurrencyID.c_str());
    strcpy(y.ToCurrencyID, x.ToCurrencyID.c_str());
    return y;
}

QryExchangeRateField Converter::CThostFtdcQryExchangeRateFieldToRust(CThostFtdcQryExchangeRateField* x) {
    if (x == nullptr)
        return QryExchangeRateField{.is_null = true};
    QryExchangeRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.FromCurrencyID = Converter::Gb2312ToRustString(x->FromCurrencyID);
    y.ToCurrencyID = Converter::Gb2312ToRustString(x->ToCurrencyID);
    return y;
}

CThostFtdcQrySyncFundMortgageField Converter::QrySyncFundMortgageFieldToCpp(QrySyncFundMortgageField x) {
    CThostFtdcQrySyncFundMortgageField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    memcpy(y.MortgageSeqNo, x.MortgageSeqNo.data(), x.MortgageSeqNo.size() * sizeof(uint8_t));
    return y;
}

QrySyncFundMortgageField Converter::CThostFtdcQrySyncFundMortgageFieldToRust(CThostFtdcQrySyncFundMortgageField* x) {
    if (x == nullptr)
        return QrySyncFundMortgageField{.is_null = true};
    QrySyncFundMortgageField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    for (int i = 0; i < 15; i++)
        y.MortgageSeqNo.push_back(x->MortgageSeqNo[i]);
    return y;
}

CThostFtdcQryHisOrderField Converter::QryHisOrderFieldToCpp(QryHisOrderField x) {
    CThostFtdcQryHisOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    strcpy(y.InsertTimeStart, x.InsertTimeStart.c_str());
    strcpy(y.InsertTimeEnd, x.InsertTimeEnd.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryHisOrderField Converter::CThostFtdcQryHisOrderFieldToRust(CThostFtdcQryHisOrderField* x) {
    if (x == nullptr)
        return QryHisOrderField{.is_null = true};
    QryHisOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.InsertTimeStart = Converter::Gb2312ToRustString(x->InsertTimeStart);
    y.InsertTimeEnd = Converter::Gb2312ToRustString(x->InsertTimeEnd);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcOptionInstrMiniMarginField Converter::OptionInstrMiniMarginFieldToCpp(OptionInstrMiniMarginField x) {
    CThostFtdcOptionInstrMiniMarginField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.MinMargin = x.MinMargin;
    y.ValueMethod = x.ValueMethod;
    y.IsRelative = x.IsRelative;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

OptionInstrMiniMarginField Converter::CThostFtdcOptionInstrMiniMarginFieldToRust(CThostFtdcOptionInstrMiniMarginField* x) {
    if (x == nullptr)
        return OptionInstrMiniMarginField{.is_null = true};
    OptionInstrMiniMarginField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.MinMargin = x->MinMargin;
    y.ValueMethod = x->ValueMethod;
    y.IsRelative = x->IsRelative;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcOptionInstrMarginAdjustField Converter::OptionInstrMarginAdjustFieldToCpp(OptionInstrMarginAdjustField x) {
    CThostFtdcOptionInstrMarginAdjustField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.SShortMarginRatioByMoney = x.SShortMarginRatioByMoney;
    y.SShortMarginRatioByVolume = x.SShortMarginRatioByVolume;
    y.HShortMarginRatioByMoney = x.HShortMarginRatioByMoney;
    y.HShortMarginRatioByVolume = x.HShortMarginRatioByVolume;
    y.AShortMarginRatioByMoney = x.AShortMarginRatioByMoney;
    y.AShortMarginRatioByVolume = x.AShortMarginRatioByVolume;
    y.IsRelative = x.IsRelative;
    y.MShortMarginRatioByMoney = x.MShortMarginRatioByMoney;
    y.MShortMarginRatioByVolume = x.MShortMarginRatioByVolume;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

OptionInstrMarginAdjustField Converter::CThostFtdcOptionInstrMarginAdjustFieldToRust(CThostFtdcOptionInstrMarginAdjustField* x) {
    if (x == nullptr)
        return OptionInstrMarginAdjustField{.is_null = true};
    OptionInstrMarginAdjustField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SShortMarginRatioByMoney = x->SShortMarginRatioByMoney;
    y.SShortMarginRatioByVolume = x->SShortMarginRatioByVolume;
    y.HShortMarginRatioByMoney = x->HShortMarginRatioByMoney;
    y.HShortMarginRatioByVolume = x->HShortMarginRatioByVolume;
    y.AShortMarginRatioByMoney = x->AShortMarginRatioByMoney;
    y.AShortMarginRatioByVolume = x->AShortMarginRatioByVolume;
    y.IsRelative = x->IsRelative;
    y.MShortMarginRatioByMoney = x->MShortMarginRatioByMoney;
    y.MShortMarginRatioByVolume = x->MShortMarginRatioByVolume;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcOptionInstrCommRateField Converter::OptionInstrCommRateFieldToCpp(OptionInstrCommRateField x) {
    CThostFtdcOptionInstrCommRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    y.StrikeRatioByMoney = x.StrikeRatioByMoney;
    y.StrikeRatioByVolume = x.StrikeRatioByVolume;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

OptionInstrCommRateField Converter::CThostFtdcOptionInstrCommRateFieldToRust(CThostFtdcOptionInstrCommRateField* x) {
    if (x == nullptr)
        return OptionInstrCommRateField{.is_null = true};
    OptionInstrCommRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.StrikeRatioByMoney = x->StrikeRatioByMoney;
    y.StrikeRatioByVolume = x->StrikeRatioByVolume;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcOptionInstrTradeCostField Converter::OptionInstrTradeCostFieldToCpp(OptionInstrTradeCostField x) {
    CThostFtdcOptionInstrTradeCostField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.FixedMargin = x.FixedMargin;
    y.MiniMargin = x.MiniMargin;
    y.Royalty = x.Royalty;
    y.ExchFixedMargin = x.ExchFixedMargin;
    y.ExchMiniMargin = x.ExchMiniMargin;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

OptionInstrTradeCostField Converter::CThostFtdcOptionInstrTradeCostFieldToRust(CThostFtdcOptionInstrTradeCostField* x) {
    if (x == nullptr)
        return OptionInstrTradeCostField{.is_null = true};
    OptionInstrTradeCostField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.FixedMargin = x->FixedMargin;
    y.MiniMargin = x->MiniMargin;
    y.Royalty = x->Royalty;
    y.ExchFixedMargin = x->ExchFixedMargin;
    y.ExchMiniMargin = x->ExchMiniMargin;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryOptionInstrTradeCostField Converter::QryOptionInstrTradeCostFieldToCpp(QryOptionInstrTradeCostField x) {
    CThostFtdcQryOptionInstrTradeCostField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.InputPrice = x.InputPrice;
    y.UnderlyingPrice = x.UnderlyingPrice;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryOptionInstrTradeCostField Converter::CThostFtdcQryOptionInstrTradeCostFieldToRust(CThostFtdcQryOptionInstrTradeCostField* x) {
    if (x == nullptr)
        return QryOptionInstrTradeCostField{.is_null = true};
    QryOptionInstrTradeCostField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.InputPrice = x->InputPrice;
    y.UnderlyingPrice = x->UnderlyingPrice;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryOptionInstrCommRateField Converter::QryOptionInstrCommRateFieldToCpp(QryOptionInstrCommRateField x) {
    CThostFtdcQryOptionInstrCommRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryOptionInstrCommRateField Converter::CThostFtdcQryOptionInstrCommRateFieldToRust(CThostFtdcQryOptionInstrCommRateField* x) {
    if (x == nullptr)
        return QryOptionInstrCommRateField{.is_null = true};
    QryOptionInstrCommRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcIndexPriceField Converter::IndexPriceFieldToCpp(IndexPriceField x) {
    CThostFtdcIndexPriceField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.ClosePrice = x.ClosePrice;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

IndexPriceField Converter::CThostFtdcIndexPriceFieldToRust(CThostFtdcIndexPriceField* x) {
    if (x == nullptr)
        return IndexPriceField{.is_null = true};
    IndexPriceField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ClosePrice = x->ClosePrice;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInputExecOrderField Converter::InputExecOrderFieldToCpp(InputExecOrderField x) {
    CThostFtdcInputExecOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExecOrderRef, x.ExecOrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.ActionType = x.ActionType;
    y.PosiDirection = x.PosiDirection;
    y.ReservePositionFlag = x.ReservePositionFlag;
    y.CloseFlag = x.CloseFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputExecOrderField Converter::CThostFtdcInputExecOrderFieldToRust(CThostFtdcInputExecOrderField* x) {
    if (x == nullptr)
        return InputExecOrderField{.is_null = true};
    InputExecOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExecOrderRef = Converter::Gb2312ToRustString(x->ExecOrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.ActionType = x->ActionType;
    y.PosiDirection = x->PosiDirection;
    y.ReservePositionFlag = x->ReservePositionFlag;
    y.CloseFlag = x->CloseFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcInputExecOrderActionField Converter::InputExecOrderActionFieldToCpp(InputExecOrderActionField x) {
    CThostFtdcInputExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.ExecOrderActionRef = x.ExecOrderActionRef;
    strcpy(y.ExecOrderRef, x.ExecOrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputExecOrderActionField Converter::CThostFtdcInputExecOrderActionFieldToRust(CThostFtdcInputExecOrderActionField* x) {
    if (x == nullptr)
        return InputExecOrderActionField{.is_null = true};
    InputExecOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExecOrderActionRef = x->ExecOrderActionRef;
    y.ExecOrderRef = Converter::Gb2312ToRustString(x->ExecOrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcExecOrderField Converter::ExecOrderFieldToCpp(ExecOrderField x) {
    CThostFtdcExecOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExecOrderRef, x.ExecOrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.ActionType = x.ActionType;
    y.PosiDirection = x.PosiDirection;
    y.ReservePositionFlag = x.ReservePositionFlag;
    y.CloseFlag = x.CloseFlag;
    strcpy(y.ExecOrderLocalID, x.ExecOrderLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.ExecResult = x.ExecResult;
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerExecOrderSeq = x.BrokerExecOrderSeq;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExecOrderField Converter::CThostFtdcExecOrderFieldToRust(CThostFtdcExecOrderField* x) {
    if (x == nullptr)
        return ExecOrderField{.is_null = true};
    ExecOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExecOrderRef = Converter::Gb2312ToRustString(x->ExecOrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.ActionType = x->ActionType;
    y.PosiDirection = x->PosiDirection;
    y.ReservePositionFlag = x->ReservePositionFlag;
    y.CloseFlag = x->CloseFlag;
    y.ExecOrderLocalID = Converter::Gb2312ToRustString(x->ExecOrderLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ExecResult = x->ExecResult;
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerExecOrderSeq = x->BrokerExecOrderSeq;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcExecOrderActionField Converter::ExecOrderActionFieldToCpp(ExecOrderActionField x) {
    CThostFtdcExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.ExecOrderActionRef = x.ExecOrderActionRef;
    strcpy(y.ExecOrderRef, x.ExecOrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.ExecOrderLocalID, x.ExecOrderLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    y.ActionType = x.ActionType;
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExecOrderActionField Converter::CThostFtdcExecOrderActionFieldToRust(CThostFtdcExecOrderActionField* x) {
    if (x == nullptr)
        return ExecOrderActionField{.is_null = true};
    ExecOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExecOrderActionRef = x->ExecOrderActionRef;
    y.ExecOrderRef = Converter::Gb2312ToRustString(x->ExecOrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ExecOrderLocalID = Converter::Gb2312ToRustString(x->ExecOrderLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ActionType = x->ActionType;
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryExecOrderField Converter::QryExecOrderFieldToCpp(QryExecOrderField x) {
    CThostFtdcQryExecOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    strcpy(y.InsertTimeStart, x.InsertTimeStart.c_str());
    strcpy(y.InsertTimeEnd, x.InsertTimeEnd.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryExecOrderField Converter::CThostFtdcQryExecOrderFieldToRust(CThostFtdcQryExecOrderField* x) {
    if (x == nullptr)
        return QryExecOrderField{.is_null = true};
    QryExecOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.InsertTimeStart = Converter::Gb2312ToRustString(x->InsertTimeStart);
    y.InsertTimeEnd = Converter::Gb2312ToRustString(x->InsertTimeEnd);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeExecOrderField Converter::ExchangeExecOrderFieldToCpp(ExchangeExecOrderField x) {
    CThostFtdcExchangeExecOrderField y;
    memset(&y, 0, sizeof(y));
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.ActionType = x.ActionType;
    y.PosiDirection = x.PosiDirection;
    y.ReservePositionFlag = x.ReservePositionFlag;
    y.CloseFlag = x.CloseFlag;
    strcpy(y.ExecOrderLocalID, x.ExecOrderLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.ExecResult = x.ExecResult;
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeExecOrderField Converter::CThostFtdcExchangeExecOrderFieldToRust(CThostFtdcExchangeExecOrderField* x) {
    if (x == nullptr)
        return ExchangeExecOrderField{.is_null = true};
    ExchangeExecOrderField y{};
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.ActionType = x->ActionType;
    y.PosiDirection = x->PosiDirection;
    y.ReservePositionFlag = x->ReservePositionFlag;
    y.CloseFlag = x->CloseFlag;
    y.ExecOrderLocalID = Converter::Gb2312ToRustString(x->ExecOrderLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ExecResult = x->ExecResult;
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryExchangeExecOrderField Converter::QryExchangeExecOrderFieldToCpp(QryExchangeExecOrderField x) {
    CThostFtdcQryExchangeExecOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

QryExchangeExecOrderField Converter::CThostFtdcQryExchangeExecOrderFieldToRust(CThostFtdcQryExchangeExecOrderField* x) {
    if (x == nullptr)
        return QryExchangeExecOrderField{.is_null = true};
    QryExchangeExecOrderField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcQryExecOrderActionField Converter::QryExecOrderActionFieldToCpp(QryExecOrderActionField x) {
    CThostFtdcQryExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryExecOrderActionField Converter::CThostFtdcQryExecOrderActionFieldToRust(CThostFtdcQryExecOrderActionField* x) {
    if (x == nullptr)
        return QryExecOrderActionField{.is_null = true};
    QryExecOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcExchangeExecOrderActionField Converter::ExchangeExecOrderActionFieldToCpp(ExchangeExecOrderActionField x) {
    CThostFtdcExchangeExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.ExecOrderLocalID, x.ExecOrderLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    y.ActionType = x.ActionType;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    y.Volume = x.Volume;
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

ExchangeExecOrderActionField Converter::CThostFtdcExchangeExecOrderActionFieldToRust(CThostFtdcExchangeExecOrderActionField* x) {
    if (x == nullptr)
        return ExchangeExecOrderActionField{.is_null = true};
    ExchangeExecOrderActionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ExecOrderLocalID = Converter::Gb2312ToRustString(x->ExecOrderLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ActionType = x->ActionType;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.Volume = x->Volume;
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcQryExchangeExecOrderActionField Converter::QryExchangeExecOrderActionFieldToCpp(QryExchangeExecOrderActionField x) {
    CThostFtdcQryExchangeExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryExchangeExecOrderActionField Converter::CThostFtdcQryExchangeExecOrderActionFieldToRust(CThostFtdcQryExchangeExecOrderActionField* x) {
    if (x == nullptr)
        return QryExchangeExecOrderActionField{.is_null = true};
    QryExchangeExecOrderActionField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcErrExecOrderField Converter::ErrExecOrderFieldToCpp(ErrExecOrderField x) {
    CThostFtdcErrExecOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExecOrderRef, x.ExecOrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.ActionType = x.ActionType;
    y.PosiDirection = x.PosiDirection;
    y.ReservePositionFlag = x.ReservePositionFlag;
    y.CloseFlag = x.CloseFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ErrExecOrderField Converter::CThostFtdcErrExecOrderFieldToRust(CThostFtdcErrExecOrderField* x) {
    if (x == nullptr)
        return ErrExecOrderField{.is_null = true};
    ErrExecOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExecOrderRef = Converter::Gb2312ToRustString(x->ExecOrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.ActionType = x->ActionType;
    y.PosiDirection = x->PosiDirection;
    y.ReservePositionFlag = x->ReservePositionFlag;
    y.CloseFlag = x->CloseFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryErrExecOrderField Converter::QryErrExecOrderFieldToCpp(QryErrExecOrderField x) {
    CThostFtdcQryErrExecOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryErrExecOrderField Converter::CThostFtdcQryErrExecOrderFieldToRust(CThostFtdcQryErrExecOrderField* x) {
    if (x == nullptr)
        return QryErrExecOrderField{.is_null = true};
    QryErrExecOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcErrExecOrderActionField Converter::ErrExecOrderActionFieldToCpp(ErrExecOrderActionField x) {
    CThostFtdcErrExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.ExecOrderActionRef = x.ExecOrderActionRef;
    strcpy(y.ExecOrderRef, x.ExecOrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExecOrderSysID, x.ExecOrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ErrExecOrderActionField Converter::CThostFtdcErrExecOrderActionFieldToRust(CThostFtdcErrExecOrderActionField* x) {
    if (x == nullptr)
        return ErrExecOrderActionField{.is_null = true};
    ErrExecOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExecOrderActionRef = x->ExecOrderActionRef;
    y.ExecOrderRef = Converter::Gb2312ToRustString(x->ExecOrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExecOrderSysID = Converter::Gb2312ToRustString(x->ExecOrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryErrExecOrderActionField Converter::QryErrExecOrderActionFieldToCpp(QryErrExecOrderActionField x) {
    CThostFtdcQryErrExecOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryErrExecOrderActionField Converter::CThostFtdcQryErrExecOrderActionFieldToRust(CThostFtdcQryErrExecOrderActionField* x) {
    if (x == nullptr)
        return QryErrExecOrderActionField{.is_null = true};
    QryErrExecOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcOptionInstrTradingRightField Converter::OptionInstrTradingRightFieldToCpp(OptionInstrTradingRightField x) {
    CThostFtdcOptionInstrTradingRightField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Direction = x.Direction;
    y.TradingRight = x.TradingRight;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

OptionInstrTradingRightField Converter::CThostFtdcOptionInstrTradingRightFieldToRust(CThostFtdcOptionInstrTradingRightField* x) {
    if (x == nullptr)
        return OptionInstrTradingRightField{.is_null = true};
    OptionInstrTradingRightField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Direction = x->Direction;
    y.TradingRight = x->TradingRight;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryOptionInstrTradingRightField Converter::QryOptionInstrTradingRightFieldToCpp(QryOptionInstrTradingRightField x) {
    CThostFtdcQryOptionInstrTradingRightField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Direction = x.Direction;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryOptionInstrTradingRightField Converter::CThostFtdcQryOptionInstrTradingRightFieldToRust(CThostFtdcQryOptionInstrTradingRightField* x) {
    if (x == nullptr)
        return QryOptionInstrTradingRightField{.is_null = true};
    QryOptionInstrTradingRightField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Direction = x->Direction;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInputForQuoteField Converter::InputForQuoteFieldToCpp(InputForQuoteField x) {
    CThostFtdcInputForQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ForQuoteRef, x.ForQuoteRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputForQuoteField Converter::CThostFtdcInputForQuoteFieldToRust(CThostFtdcInputForQuoteField* x) {
    if (x == nullptr)
        return InputForQuoteField{.is_null = true};
    InputForQuoteField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ForQuoteRef = Converter::Gb2312ToRustString(x->ForQuoteRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcForQuoteField Converter::ForQuoteFieldToCpp(ForQuoteField x) {
    CThostFtdcForQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ForQuoteRef, x.ForQuoteRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ForQuoteLocalID, x.ForQuoteLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    y.ForQuoteStatus = x.ForQuoteStatus;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerForQutoSeq = x.BrokerForQutoSeq;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ForQuoteField Converter::CThostFtdcForQuoteFieldToRust(CThostFtdcForQuoteField* x) {
    if (x == nullptr)
        return ForQuoteField{.is_null = true};
    ForQuoteField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ForQuoteRef = Converter::Gb2312ToRustString(x->ForQuoteRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ForQuoteLocalID = Converter::Gb2312ToRustString(x->ForQuoteLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.ForQuoteStatus = x->ForQuoteStatus;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerForQutoSeq = x->BrokerForQutoSeq;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryForQuoteField Converter::QryForQuoteFieldToCpp(QryForQuoteField x) {
    CThostFtdcQryForQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InsertTimeStart, x.InsertTimeStart.c_str());
    strcpy(y.InsertTimeEnd, x.InsertTimeEnd.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryForQuoteField Converter::CThostFtdcQryForQuoteFieldToRust(CThostFtdcQryForQuoteField* x) {
    if (x == nullptr)
        return QryForQuoteField{.is_null = true};
    QryForQuoteField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InsertTimeStart = Converter::Gb2312ToRustString(x->InsertTimeStart);
    y.InsertTimeEnd = Converter::Gb2312ToRustString(x->InsertTimeEnd);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeForQuoteField Converter::ExchangeForQuoteFieldToCpp(ExchangeForQuoteField x) {
    CThostFtdcExchangeForQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ForQuoteLocalID, x.ForQuoteLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    y.ForQuoteStatus = x.ForQuoteStatus;
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeForQuoteField Converter::CThostFtdcExchangeForQuoteFieldToRust(CThostFtdcExchangeForQuoteField* x) {
    if (x == nullptr)
        return ExchangeForQuoteField{.is_null = true};
    ExchangeForQuoteField y{};
    y.ForQuoteLocalID = Converter::Gb2312ToRustString(x->ForQuoteLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.ForQuoteStatus = x->ForQuoteStatus;
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryExchangeForQuoteField Converter::QryExchangeForQuoteFieldToCpp(QryExchangeForQuoteField x) {
    CThostFtdcQryExchangeForQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

QryExchangeForQuoteField Converter::CThostFtdcQryExchangeForQuoteFieldToRust(CThostFtdcQryExchangeForQuoteField* x) {
    if (x == nullptr)
        return QryExchangeForQuoteField{.is_null = true};
    QryExchangeForQuoteField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcInputQuoteField Converter::InputQuoteFieldToCpp(InputQuoteField x) {
    CThostFtdcInputQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.QuoteRef, x.QuoteRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.AskPrice = x.AskPrice;
    y.BidPrice = x.BidPrice;
    y.AskVolume = x.AskVolume;
    y.BidVolume = x.BidVolume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.AskOffsetFlag = x.AskOffsetFlag;
    y.BidOffsetFlag = x.BidOffsetFlag;
    y.AskHedgeFlag = x.AskHedgeFlag;
    y.BidHedgeFlag = x.BidHedgeFlag;
    strcpy(y.AskOrderRef, x.AskOrderRef.c_str());
    strcpy(y.BidOrderRef, x.BidOrderRef.c_str());
    strcpy(y.ForQuoteSysID, x.ForQuoteSysID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.ReplaceSysID, x.ReplaceSysID.c_str());
    y.TimeCondition = x.TimeCondition;
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

InputQuoteField Converter::CThostFtdcInputQuoteFieldToRust(CThostFtdcInputQuoteField* x) {
    if (x == nullptr)
        return InputQuoteField{.is_null = true};
    InputQuoteField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.QuoteRef = Converter::Gb2312ToRustString(x->QuoteRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.AskPrice = x->AskPrice;
    y.BidPrice = x->BidPrice;
    y.AskVolume = x->AskVolume;
    y.BidVolume = x->BidVolume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.AskOffsetFlag = x->AskOffsetFlag;
    y.BidOffsetFlag = x->BidOffsetFlag;
    y.AskHedgeFlag = x->AskHedgeFlag;
    y.BidHedgeFlag = x->BidHedgeFlag;
    y.AskOrderRef = Converter::Gb2312ToRustString(x->AskOrderRef);
    y.BidOrderRef = Converter::Gb2312ToRustString(x->BidOrderRef);
    y.ForQuoteSysID = Converter::Gb2312ToRustString(x->ForQuoteSysID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.ReplaceSysID = Converter::Gb2312ToRustString(x->ReplaceSysID);
    y.TimeCondition = x->TimeCondition;
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcInputQuoteActionField Converter::InputQuoteActionFieldToCpp(InputQuoteActionField x) {
    CThostFtdcInputQuoteActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.QuoteActionRef = x.QuoteActionRef;
    strcpy(y.QuoteRef, x.QuoteRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.QuoteSysID, x.QuoteSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

InputQuoteActionField Converter::CThostFtdcInputQuoteActionFieldToRust(CThostFtdcInputQuoteActionField* x) {
    if (x == nullptr)
        return InputQuoteActionField{.is_null = true};
    InputQuoteActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.QuoteActionRef = x->QuoteActionRef;
    y.QuoteRef = Converter::Gb2312ToRustString(x->QuoteRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.QuoteSysID = Converter::Gb2312ToRustString(x->QuoteSysID);
    y.ActionFlag = x->ActionFlag;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcQuoteField Converter::QuoteFieldToCpp(QuoteField x) {
    CThostFtdcQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.QuoteRef, x.QuoteRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.AskPrice = x.AskPrice;
    y.BidPrice = x.BidPrice;
    y.AskVolume = x.AskVolume;
    y.BidVolume = x.BidVolume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.AskOffsetFlag = x.AskOffsetFlag;
    y.BidOffsetFlag = x.BidOffsetFlag;
    y.AskHedgeFlag = x.AskHedgeFlag;
    y.BidHedgeFlag = x.BidHedgeFlag;
    strcpy(y.QuoteLocalID, x.QuoteLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.NotifySequence = x.NotifySequence;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.QuoteSysID, x.QuoteSysID.c_str());
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.QuoteStatus = x.QuoteStatus;
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    strcpy(y.AskOrderSysID, x.AskOrderSysID.c_str());
    strcpy(y.BidOrderSysID, x.BidOrderSysID.c_str());
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerQuoteSeq = x.BrokerQuoteSeq;
    strcpy(y.AskOrderRef, x.AskOrderRef.c_str());
    strcpy(y.BidOrderRef, x.BidOrderRef.c_str());
    strcpy(y.ForQuoteSysID, x.ForQuoteSysID.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.ReplaceSysID, x.ReplaceSysID.c_str());
    y.TimeCondition = x.TimeCondition;
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

QuoteField Converter::CThostFtdcQuoteFieldToRust(CThostFtdcQuoteField* x) {
    if (x == nullptr)
        return QuoteField{.is_null = true};
    QuoteField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.QuoteRef = Converter::Gb2312ToRustString(x->QuoteRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.AskPrice = x->AskPrice;
    y.BidPrice = x->BidPrice;
    y.AskVolume = x->AskVolume;
    y.BidVolume = x->BidVolume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.AskOffsetFlag = x->AskOffsetFlag;
    y.BidOffsetFlag = x->BidOffsetFlag;
    y.AskHedgeFlag = x->AskHedgeFlag;
    y.BidHedgeFlag = x->BidHedgeFlag;
    y.QuoteLocalID = Converter::Gb2312ToRustString(x->QuoteLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.NotifySequence = x->NotifySequence;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.QuoteSysID = Converter::Gb2312ToRustString(x->QuoteSysID);
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.QuoteStatus = x->QuoteStatus;
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.AskOrderSysID = Converter::Gb2312ToRustString(x->AskOrderSysID);
    y.BidOrderSysID = Converter::Gb2312ToRustString(x->BidOrderSysID);
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerQuoteSeq = x->BrokerQuoteSeq;
    y.AskOrderRef = Converter::Gb2312ToRustString(x->AskOrderRef);
    y.BidOrderRef = Converter::Gb2312ToRustString(x->BidOrderRef);
    y.ForQuoteSysID = Converter::Gb2312ToRustString(x->ForQuoteSysID);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.ReplaceSysID = Converter::Gb2312ToRustString(x->ReplaceSysID);
    y.TimeCondition = x->TimeCondition;
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcQuoteActionField Converter::QuoteActionFieldToCpp(QuoteActionField x) {
    CThostFtdcQuoteActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.QuoteActionRef = x.QuoteActionRef;
    strcpy(y.QuoteRef, x.QuoteRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.QuoteSysID, x.QuoteSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.QuoteLocalID, x.QuoteLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

QuoteActionField Converter::CThostFtdcQuoteActionFieldToRust(CThostFtdcQuoteActionField* x) {
    if (x == nullptr)
        return QuoteActionField{.is_null = true};
    QuoteActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.QuoteActionRef = x->QuoteActionRef;
    y.QuoteRef = Converter::Gb2312ToRustString(x->QuoteRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.QuoteSysID = Converter::Gb2312ToRustString(x->QuoteSysID);
    y.ActionFlag = x->ActionFlag;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.QuoteLocalID = Converter::Gb2312ToRustString(x->QuoteLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcQryQuoteField Converter::QryQuoteFieldToCpp(QryQuoteField x) {
    CThostFtdcQryQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.QuoteSysID, x.QuoteSysID.c_str());
    strcpy(y.InsertTimeStart, x.InsertTimeStart.c_str());
    strcpy(y.InsertTimeEnd, x.InsertTimeEnd.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryQuoteField Converter::CThostFtdcQryQuoteFieldToRust(CThostFtdcQryQuoteField* x) {
    if (x == nullptr)
        return QryQuoteField{.is_null = true};
    QryQuoteField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.QuoteSysID = Converter::Gb2312ToRustString(x->QuoteSysID);
    y.InsertTimeStart = Converter::Gb2312ToRustString(x->InsertTimeStart);
    y.InsertTimeEnd = Converter::Gb2312ToRustString(x->InsertTimeEnd);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeQuoteField Converter::ExchangeQuoteFieldToCpp(ExchangeQuoteField x) {
    CThostFtdcExchangeQuoteField y;
    memset(&y, 0, sizeof(y));
    y.AskPrice = x.AskPrice;
    y.BidPrice = x.BidPrice;
    y.AskVolume = x.AskVolume;
    y.BidVolume = x.BidVolume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.AskOffsetFlag = x.AskOffsetFlag;
    y.BidOffsetFlag = x.BidOffsetFlag;
    y.AskHedgeFlag = x.AskHedgeFlag;
    y.BidHedgeFlag = x.BidHedgeFlag;
    strcpy(y.QuoteLocalID, x.QuoteLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.NotifySequence = x.NotifySequence;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.QuoteSysID, x.QuoteSysID.c_str());
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.QuoteStatus = x.QuoteStatus;
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    strcpy(y.AskOrderSysID, x.AskOrderSysID.c_str());
    strcpy(y.BidOrderSysID, x.BidOrderSysID.c_str());
    strcpy(y.ForQuoteSysID, x.ForQuoteSysID.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    y.TimeCondition = x.TimeCondition;
    return y;
}

ExchangeQuoteField Converter::CThostFtdcExchangeQuoteFieldToRust(CThostFtdcExchangeQuoteField* x) {
    if (x == nullptr)
        return ExchangeQuoteField{.is_null = true};
    ExchangeQuoteField y{};
    y.AskPrice = x->AskPrice;
    y.BidPrice = x->BidPrice;
    y.AskVolume = x->AskVolume;
    y.BidVolume = x->BidVolume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.AskOffsetFlag = x->AskOffsetFlag;
    y.BidOffsetFlag = x->BidOffsetFlag;
    y.AskHedgeFlag = x->AskHedgeFlag;
    y.BidHedgeFlag = x->BidHedgeFlag;
    y.QuoteLocalID = Converter::Gb2312ToRustString(x->QuoteLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.NotifySequence = x->NotifySequence;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.QuoteSysID = Converter::Gb2312ToRustString(x->QuoteSysID);
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.QuoteStatus = x->QuoteStatus;
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.AskOrderSysID = Converter::Gb2312ToRustString(x->AskOrderSysID);
    y.BidOrderSysID = Converter::Gb2312ToRustString(x->BidOrderSysID);
    y.ForQuoteSysID = Converter::Gb2312ToRustString(x->ForQuoteSysID);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.TimeCondition = x->TimeCondition;
    return y;
}

CThostFtdcQryExchangeQuoteField Converter::QryExchangeQuoteFieldToCpp(QryExchangeQuoteField x) {
    CThostFtdcQryExchangeQuoteField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

QryExchangeQuoteField Converter::CThostFtdcQryExchangeQuoteFieldToRust(CThostFtdcQryExchangeQuoteField* x) {
    if (x == nullptr)
        return QryExchangeQuoteField{.is_null = true};
    QryExchangeQuoteField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcQryQuoteActionField Converter::QryQuoteActionFieldToCpp(QryQuoteActionField x) {
    CThostFtdcQryQuoteActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryQuoteActionField Converter::CThostFtdcQryQuoteActionFieldToRust(CThostFtdcQryQuoteActionField* x) {
    if (x == nullptr)
        return QryQuoteActionField{.is_null = true};
    QryQuoteActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcExchangeQuoteActionField Converter::ExchangeQuoteActionFieldToCpp(ExchangeQuoteActionField x) {
    CThostFtdcExchangeQuoteActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.QuoteSysID, x.QuoteSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.QuoteLocalID, x.QuoteLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeQuoteActionField Converter::CThostFtdcExchangeQuoteActionFieldToRust(CThostFtdcExchangeQuoteActionField* x) {
    if (x == nullptr)
        return ExchangeQuoteActionField{.is_null = true};
    ExchangeQuoteActionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.QuoteSysID = Converter::Gb2312ToRustString(x->QuoteSysID);
    y.ActionFlag = x->ActionFlag;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.QuoteLocalID = Converter::Gb2312ToRustString(x->QuoteLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryExchangeQuoteActionField Converter::QryExchangeQuoteActionFieldToCpp(QryExchangeQuoteActionField x) {
    CThostFtdcQryExchangeQuoteActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryExchangeQuoteActionField Converter::CThostFtdcQryExchangeQuoteActionFieldToRust(CThostFtdcQryExchangeQuoteActionField* x) {
    if (x == nullptr)
        return QryExchangeQuoteActionField{.is_null = true};
    QryExchangeQuoteActionField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcOptionInstrDeltaField Converter::OptionInstrDeltaFieldToCpp(OptionInstrDeltaField x) {
    CThostFtdcOptionInstrDeltaField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Delta = x.Delta;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

OptionInstrDeltaField Converter::CThostFtdcOptionInstrDeltaFieldToRust(CThostFtdcOptionInstrDeltaField* x) {
    if (x == nullptr)
        return OptionInstrDeltaField{.is_null = true};
    OptionInstrDeltaField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Delta = x->Delta;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcForQuoteRspField Converter::ForQuoteRspFieldToCpp(ForQuoteRspField x) {
    CThostFtdcForQuoteRspField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ForQuoteSysID, x.ForQuoteSysID.c_str());
    strcpy(y.ForQuoteTime, x.ForQuoteTime.c_str());
    strcpy(y.ActionDay, x.ActionDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

ForQuoteRspField Converter::CThostFtdcForQuoteRspFieldToRust(CThostFtdcForQuoteRspField* x) {
    if (x == nullptr)
        return ForQuoteRspField{.is_null = true};
    ForQuoteRspField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ForQuoteSysID = Converter::Gb2312ToRustString(x->ForQuoteSysID);
    y.ForQuoteTime = Converter::Gb2312ToRustString(x->ForQuoteTime);
    y.ActionDay = Converter::Gb2312ToRustString(x->ActionDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcStrikeOffsetField Converter::StrikeOffsetFieldToCpp(StrikeOffsetField x) {
    CThostFtdcStrikeOffsetField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Offset = x.Offset;
    y.OffsetType = x.OffsetType;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

StrikeOffsetField Converter::CThostFtdcStrikeOffsetFieldToRust(CThostFtdcStrikeOffsetField* x) {
    if (x == nullptr)
        return StrikeOffsetField{.is_null = true};
    StrikeOffsetField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Offset = x->Offset;
    y.OffsetType = x->OffsetType;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryStrikeOffsetField Converter::QryStrikeOffsetFieldToCpp(QryStrikeOffsetField x) {
    CThostFtdcQryStrikeOffsetField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryStrikeOffsetField Converter::CThostFtdcQryStrikeOffsetFieldToRust(CThostFtdcQryStrikeOffsetField* x) {
    if (x == nullptr)
        return QryStrikeOffsetField{.is_null = true};
    QryStrikeOffsetField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInputBatchOrderActionField Converter::InputBatchOrderActionFieldToCpp(InputBatchOrderActionField x) {
    CThostFtdcInputBatchOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OrderActionRef = x.OrderActionRef;
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputBatchOrderActionField Converter::CThostFtdcInputBatchOrderActionFieldToRust(CThostFtdcInputBatchOrderActionField* x) {
    if (x == nullptr)
        return InputBatchOrderActionField{.is_null = true};
    InputBatchOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderActionRef = x->OrderActionRef;
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcBatchOrderActionField Converter::BatchOrderActionFieldToCpp(BatchOrderActionField x) {
    CThostFtdcBatchOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OrderActionRef = x.OrderActionRef;
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

BatchOrderActionField Converter::CThostFtdcBatchOrderActionFieldToRust(CThostFtdcBatchOrderActionField* x) {
    if (x == nullptr)
        return BatchOrderActionField{.is_null = true};
    BatchOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderActionRef = x->OrderActionRef;
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcExchangeBatchOrderActionField Converter::ExchangeBatchOrderActionFieldToCpp(ExchangeBatchOrderActionField x) {
    CThostFtdcExchangeBatchOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeBatchOrderActionField Converter::CThostFtdcExchangeBatchOrderActionFieldToRust(CThostFtdcExchangeBatchOrderActionField* x) {
    if (x == nullptr)
        return ExchangeBatchOrderActionField{.is_null = true};
    ExchangeBatchOrderActionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryBatchOrderActionField Converter::QryBatchOrderActionFieldToCpp(QryBatchOrderActionField x) {
    CThostFtdcQryBatchOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryBatchOrderActionField Converter::CThostFtdcQryBatchOrderActionFieldToRust(CThostFtdcQryBatchOrderActionField* x) {
    if (x == nullptr)
        return QryBatchOrderActionField{.is_null = true};
    QryBatchOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcCombInstrumentGuardField Converter::CombInstrumentGuardFieldToCpp(CombInstrumentGuardField x) {
    CThostFtdcCombInstrumentGuardField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.GuarantRatio = x.GuarantRatio;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

CombInstrumentGuardField Converter::CThostFtdcCombInstrumentGuardFieldToRust(CThostFtdcCombInstrumentGuardField* x) {
    if (x == nullptr)
        return CombInstrumentGuardField{.is_null = true};
    CombInstrumentGuardField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.GuarantRatio = x->GuarantRatio;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryCombInstrumentGuardField Converter::QryCombInstrumentGuardFieldToCpp(QryCombInstrumentGuardField x) {
    CThostFtdcQryCombInstrumentGuardField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryCombInstrumentGuardField Converter::CThostFtdcQryCombInstrumentGuardFieldToRust(CThostFtdcQryCombInstrumentGuardField* x) {
    if (x == nullptr)
        return QryCombInstrumentGuardField{.is_null = true};
    QryCombInstrumentGuardField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInputCombActionField Converter::InputCombActionFieldToCpp(InputCombActionField x) {
    CThostFtdcInputCombActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CombActionRef, x.CombActionRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Direction = x.Direction;
    y.Volume = x.Volume;
    y.CombDirection = x.CombDirection;
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputCombActionField Converter::CThostFtdcInputCombActionFieldToRust(CThostFtdcInputCombActionField* x) {
    if (x == nullptr)
        return InputCombActionField{.is_null = true};
    InputCombActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CombActionRef = Converter::Gb2312ToRustString(x->CombActionRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Direction = x->Direction;
    y.Volume = x->Volume;
    y.CombDirection = x->CombDirection;
    y.HedgeFlag = x->HedgeFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcCombActionField Converter::CombActionFieldToCpp(CombActionField x) {
    CThostFtdcCombActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CombActionRef, x.CombActionRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Direction = x.Direction;
    y.Volume = x.Volume;
    y.CombDirection = x.CombDirection;
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.ActionStatus = x.ActionStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.SequenceNo = x.SequenceNo;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ComTradeID, x.ComTradeID.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

CombActionField Converter::CThostFtdcCombActionFieldToRust(CThostFtdcCombActionField* x) {
    if (x == nullptr)
        return CombActionField{.is_null = true};
    CombActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CombActionRef = Converter::Gb2312ToRustString(x->CombActionRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Direction = x->Direction;
    y.Volume = x->Volume;
    y.CombDirection = x->CombDirection;
    y.HedgeFlag = x->HedgeFlag;
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ActionStatus = x->ActionStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.SequenceNo = x->SequenceNo;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ComTradeID = Converter::Gb2312ToRustString(x->ComTradeID);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryCombActionField Converter::QryCombActionFieldToCpp(QryCombActionField x) {
    CThostFtdcQryCombActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryCombActionField Converter::CThostFtdcQryCombActionFieldToRust(CThostFtdcQryCombActionField* x) {
    if (x == nullptr)
        return QryCombActionField{.is_null = true};
    QryCombActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeCombActionField Converter::ExchangeCombActionFieldToCpp(ExchangeCombActionField x) {
    CThostFtdcExchangeCombActionField y;
    memset(&y, 0, sizeof(y));
    y.Direction = x.Direction;
    y.Volume = x.Volume;
    y.CombDirection = x.CombDirection;
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.ActionStatus = x.ActionStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.SequenceNo = x.SequenceNo;
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ComTradeID, x.ComTradeID.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeCombActionField Converter::CThostFtdcExchangeCombActionFieldToRust(CThostFtdcExchangeCombActionField* x) {
    if (x == nullptr)
        return ExchangeCombActionField{.is_null = true};
    ExchangeCombActionField y{};
    y.Direction = x->Direction;
    y.Volume = x->Volume;
    y.CombDirection = x->CombDirection;
    y.HedgeFlag = x->HedgeFlag;
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ActionStatus = x->ActionStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.SequenceNo = x->SequenceNo;
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ComTradeID = Converter::Gb2312ToRustString(x->ComTradeID);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryExchangeCombActionField Converter::QryExchangeCombActionFieldToCpp(QryExchangeCombActionField x) {
    CThostFtdcQryExchangeCombActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

QryExchangeCombActionField Converter::CThostFtdcQryExchangeCombActionFieldToRust(CThostFtdcQryExchangeCombActionField* x) {
    if (x == nullptr)
        return QryExchangeCombActionField{.is_null = true};
    QryExchangeCombActionField y{};
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcProductExchRateField Converter::ProductExchRateFieldToCpp(ProductExchRateField x) {
    CThostFtdcProductExchRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.QuoteCurrencyID, x.QuoteCurrencyID.c_str());
    y.ExchangeRate = x.ExchangeRate;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

ProductExchRateField Converter::CThostFtdcProductExchRateFieldToRust(CThostFtdcProductExchRateField* x) {
    if (x == nullptr)
        return ProductExchRateField{.is_null = true};
    ProductExchRateField y{};
    y.QuoteCurrencyID = Converter::Gb2312ToRustString(x->QuoteCurrencyID);
    y.ExchangeRate = x->ExchangeRate;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcQryProductExchRateField Converter::QryProductExchRateFieldToCpp(QryProductExchRateField x) {
    CThostFtdcQryProductExchRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QryProductExchRateField Converter::CThostFtdcQryProductExchRateFieldToRust(CThostFtdcQryProductExchRateField* x) {
    if (x == nullptr)
        return QryProductExchRateField{.is_null = true};
    QryProductExchRateField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcQryForQuoteParamField Converter::QryForQuoteParamFieldToCpp(QryForQuoteParamField x) {
    CThostFtdcQryForQuoteParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryForQuoteParamField Converter::CThostFtdcQryForQuoteParamFieldToRust(CThostFtdcQryForQuoteParamField* x) {
    if (x == nullptr)
        return QryForQuoteParamField{.is_null = true};
    QryForQuoteParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcForQuoteParamField Converter::ForQuoteParamFieldToCpp(ForQuoteParamField x) {
    CThostFtdcForQuoteParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.LastPrice = x.LastPrice;
    y.PriceInterval = x.PriceInterval;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

ForQuoteParamField Converter::CThostFtdcForQuoteParamFieldToRust(CThostFtdcForQuoteParamField* x) {
    if (x == nullptr)
        return ForQuoteParamField{.is_null = true};
    ForQuoteParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.LastPrice = x->LastPrice;
    y.PriceInterval = x->PriceInterval;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcMMOptionInstrCommRateField Converter::MMOptionInstrCommRateFieldToCpp(MMOptionInstrCommRateField x) {
    CThostFtdcMMOptionInstrCommRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    y.StrikeRatioByMoney = x.StrikeRatioByMoney;
    y.StrikeRatioByVolume = x.StrikeRatioByVolume;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

MMOptionInstrCommRateField Converter::CThostFtdcMMOptionInstrCommRateFieldToRust(CThostFtdcMMOptionInstrCommRateField* x) {
    if (x == nullptr)
        return MMOptionInstrCommRateField{.is_null = true};
    MMOptionInstrCommRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.StrikeRatioByMoney = x->StrikeRatioByMoney;
    y.StrikeRatioByVolume = x->StrikeRatioByVolume;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryMMOptionInstrCommRateField Converter::QryMMOptionInstrCommRateFieldToCpp(QryMMOptionInstrCommRateField x) {
    CThostFtdcQryMMOptionInstrCommRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryMMOptionInstrCommRateField Converter::CThostFtdcQryMMOptionInstrCommRateFieldToRust(CThostFtdcQryMMOptionInstrCommRateField* x) {
    if (x == nullptr)
        return QryMMOptionInstrCommRateField{.is_null = true};
    QryMMOptionInstrCommRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcMMInstrumentCommissionRateField Converter::MMInstrumentCommissionRateFieldToCpp(MMInstrumentCommissionRateField x) {
    CThostFtdcMMInstrumentCommissionRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

MMInstrumentCommissionRateField Converter::CThostFtdcMMInstrumentCommissionRateFieldToRust(CThostFtdcMMInstrumentCommissionRateField* x) {
    if (x == nullptr)
        return MMInstrumentCommissionRateField{.is_null = true};
    MMInstrumentCommissionRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryMMInstrumentCommissionRateField Converter::QryMMInstrumentCommissionRateFieldToCpp(QryMMInstrumentCommissionRateField x) {
    CThostFtdcQryMMInstrumentCommissionRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryMMInstrumentCommissionRateField Converter::CThostFtdcQryMMInstrumentCommissionRateFieldToRust(CThostFtdcQryMMInstrumentCommissionRateField* x) {
    if (x == nullptr)
        return QryMMInstrumentCommissionRateField{.is_null = true};
    QryMMInstrumentCommissionRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInstrumentOrderCommRateField Converter::InstrumentOrderCommRateFieldToCpp(InstrumentOrderCommRateField x) {
    CThostFtdcInstrumentOrderCommRateField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.OrderCommByVolume = x.OrderCommByVolume;
    y.OrderActionCommByVolume = x.OrderActionCommByVolume;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.OrderCommByTrade = x.OrderCommByTrade;
    y.OrderActionCommByTrade = x.OrderActionCommByTrade;
    return y;
}

InstrumentOrderCommRateField Converter::CThostFtdcInstrumentOrderCommRateFieldToRust(CThostFtdcInstrumentOrderCommRateField* x) {
    if (x == nullptr)
        return InstrumentOrderCommRateField{.is_null = true};
    InstrumentOrderCommRateField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.OrderCommByVolume = x->OrderCommByVolume;
    y.OrderActionCommByVolume = x->OrderActionCommByVolume;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.OrderCommByTrade = x->OrderCommByTrade;
    y.OrderActionCommByTrade = x->OrderActionCommByTrade;
    return y;
}

CThostFtdcQryInstrumentOrderCommRateField Converter::QryInstrumentOrderCommRateFieldToCpp(QryInstrumentOrderCommRateField x) {
    CThostFtdcQryInstrumentOrderCommRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryInstrumentOrderCommRateField Converter::CThostFtdcQryInstrumentOrderCommRateFieldToRust(CThostFtdcQryInstrumentOrderCommRateField* x) {
    if (x == nullptr)
        return QryInstrumentOrderCommRateField{.is_null = true};
    QryInstrumentOrderCommRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcTradeParamField Converter::TradeParamFieldToCpp(TradeParamField x) {
    CThostFtdcTradeParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.TradeParamID = x.TradeParamID;
    memcpy(y.TradeParamValue, x.TradeParamValue.data(), x.TradeParamValue.size() * sizeof(uint8_t));
    strcpy(y.Memo, x.Memo.c_str());
    return y;
}

TradeParamField Converter::CThostFtdcTradeParamFieldToRust(CThostFtdcTradeParamField* x) {
    if (x == nullptr)
        return TradeParamField{.is_null = true};
    TradeParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.TradeParamID = x->TradeParamID;
    for (int i = 0; i < 256; i++)
        y.TradeParamValue.push_back(x->TradeParamValue[i]);
    y.Memo = Converter::Gb2312ToRustString(x->Memo);
    return y;
}

CThostFtdcInstrumentMarginRateULField Converter::InstrumentMarginRateULFieldToCpp(InstrumentMarginRateULField x) {
    CThostFtdcInstrumentMarginRateULField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

InstrumentMarginRateULField Converter::CThostFtdcInstrumentMarginRateULFieldToRust(CThostFtdcInstrumentMarginRateULField* x) {
    if (x == nullptr)
        return InstrumentMarginRateULField{.is_null = true};
    InstrumentMarginRateULField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcFutureLimitPosiParamField Converter::FutureLimitPosiParamFieldToCpp(FutureLimitPosiParamField x) {
    CThostFtdcFutureLimitPosiParamField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.SpecOpenVolume = x.SpecOpenVolume;
    y.ArbiOpenVolume = x.ArbiOpenVolume;
    y.OpenVolume = x.OpenVolume;
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

FutureLimitPosiParamField Converter::CThostFtdcFutureLimitPosiParamFieldToRust(CThostFtdcFutureLimitPosiParamField* x) {
    if (x == nullptr)
        return FutureLimitPosiParamField{.is_null = true};
    FutureLimitPosiParamField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SpecOpenVolume = x->SpecOpenVolume;
    y.ArbiOpenVolume = x->ArbiOpenVolume;
    y.OpenVolume = x->OpenVolume;
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcLoginForbiddenIPField Converter::LoginForbiddenIPFieldToCpp(LoginForbiddenIPField x) {
    CThostFtdcLoginForbiddenIPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

LoginForbiddenIPField Converter::CThostFtdcLoginForbiddenIPFieldToRust(CThostFtdcLoginForbiddenIPField* x) {
    if (x == nullptr)
        return LoginForbiddenIPField{.is_null = true};
    LoginForbiddenIPField y{};
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcIPListField Converter::IPListFieldToCpp(IPListField x) {
    CThostFtdcIPListField y;
    memset(&y, 0, sizeof(y));
    y.IsWhite = x.IsWhite;
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

IPListField Converter::CThostFtdcIPListFieldToRust(CThostFtdcIPListField* x) {
    if (x == nullptr)
        return IPListField{.is_null = true};
    IPListField y{};
    y.IsWhite = x->IsWhite;
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcInputOptionSelfCloseField Converter::InputOptionSelfCloseFieldToCpp(InputOptionSelfCloseField x) {
    CThostFtdcInputOptionSelfCloseField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OptionSelfCloseRef, x.OptionSelfCloseRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.OptSelfCloseFlag = x.OptSelfCloseFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputOptionSelfCloseField Converter::CThostFtdcInputOptionSelfCloseFieldToRust(CThostFtdcInputOptionSelfCloseField* x) {
    if (x == nullptr)
        return InputOptionSelfCloseField{.is_null = true};
    InputOptionSelfCloseField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OptionSelfCloseRef = Converter::Gb2312ToRustString(x->OptionSelfCloseRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.HedgeFlag = x->HedgeFlag;
    y.OptSelfCloseFlag = x->OptSelfCloseFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcInputOptionSelfCloseActionField Converter::InputOptionSelfCloseActionFieldToCpp(InputOptionSelfCloseActionField x) {
    CThostFtdcInputOptionSelfCloseActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OptionSelfCloseActionRef = x.OptionSelfCloseActionRef;
    strcpy(y.OptionSelfCloseRef, x.OptionSelfCloseRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OptionSelfCloseSysID, x.OptionSelfCloseSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

InputOptionSelfCloseActionField Converter::CThostFtdcInputOptionSelfCloseActionFieldToRust(CThostFtdcInputOptionSelfCloseActionField* x) {
    if (x == nullptr)
        return InputOptionSelfCloseActionField{.is_null = true};
    InputOptionSelfCloseActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OptionSelfCloseActionRef = x->OptionSelfCloseActionRef;
    y.OptionSelfCloseRef = Converter::Gb2312ToRustString(x->OptionSelfCloseRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OptionSelfCloseSysID = Converter::Gb2312ToRustString(x->OptionSelfCloseSysID);
    y.ActionFlag = x->ActionFlag;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcOptionSelfCloseField Converter::OptionSelfCloseFieldToCpp(OptionSelfCloseField x) {
    CThostFtdcOptionSelfCloseField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OptionSelfCloseRef, x.OptionSelfCloseRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.OptSelfCloseFlag = x.OptSelfCloseFlag;
    strcpy(y.OptionSelfCloseLocalID, x.OptionSelfCloseLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.OptionSelfCloseSysID, x.OptionSelfCloseSysID.c_str());
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.ExecResult = x.ExecResult;
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerOptionSelfCloseSeq = x.BrokerOptionSelfCloseSeq;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

OptionSelfCloseField Converter::CThostFtdcOptionSelfCloseFieldToRust(CThostFtdcOptionSelfCloseField* x) {
    if (x == nullptr)
        return OptionSelfCloseField{.is_null = true};
    OptionSelfCloseField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OptionSelfCloseRef = Converter::Gb2312ToRustString(x->OptionSelfCloseRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.HedgeFlag = x->HedgeFlag;
    y.OptSelfCloseFlag = x->OptSelfCloseFlag;
    y.OptionSelfCloseLocalID = Converter::Gb2312ToRustString(x->OptionSelfCloseLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OptionSelfCloseSysID = Converter::Gb2312ToRustString(x->OptionSelfCloseSysID);
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ExecResult = x->ExecResult;
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerOptionSelfCloseSeq = x->BrokerOptionSelfCloseSeq;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcOptionSelfCloseActionField Converter::OptionSelfCloseActionFieldToCpp(OptionSelfCloseActionField x) {
    CThostFtdcOptionSelfCloseActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OptionSelfCloseActionRef = x.OptionSelfCloseActionRef;
    strcpy(y.OptionSelfCloseRef, x.OptionSelfCloseRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OptionSelfCloseSysID, x.OptionSelfCloseSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OptionSelfCloseLocalID, x.OptionSelfCloseLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

OptionSelfCloseActionField Converter::CThostFtdcOptionSelfCloseActionFieldToRust(CThostFtdcOptionSelfCloseActionField* x) {
    if (x == nullptr)
        return OptionSelfCloseActionField{.is_null = true};
    OptionSelfCloseActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OptionSelfCloseActionRef = x->OptionSelfCloseActionRef;
    y.OptionSelfCloseRef = Converter::Gb2312ToRustString(x->OptionSelfCloseRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OptionSelfCloseSysID = Converter::Gb2312ToRustString(x->OptionSelfCloseSysID);
    y.ActionFlag = x->ActionFlag;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OptionSelfCloseLocalID = Converter::Gb2312ToRustString(x->OptionSelfCloseLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryOptionSelfCloseField Converter::QryOptionSelfCloseFieldToCpp(QryOptionSelfCloseField x) {
    CThostFtdcQryOptionSelfCloseField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OptionSelfCloseSysID, x.OptionSelfCloseSysID.c_str());
    strcpy(y.InsertTimeStart, x.InsertTimeStart.c_str());
    strcpy(y.InsertTimeEnd, x.InsertTimeEnd.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryOptionSelfCloseField Converter::CThostFtdcQryOptionSelfCloseFieldToRust(CThostFtdcQryOptionSelfCloseField* x) {
    if (x == nullptr)
        return QryOptionSelfCloseField{.is_null = true};
    QryOptionSelfCloseField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OptionSelfCloseSysID = Converter::Gb2312ToRustString(x->OptionSelfCloseSysID);
    y.InsertTimeStart = Converter::Gb2312ToRustString(x->InsertTimeStart);
    y.InsertTimeEnd = Converter::Gb2312ToRustString(x->InsertTimeEnd);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcExchangeOptionSelfCloseField Converter::ExchangeOptionSelfCloseFieldToCpp(ExchangeOptionSelfCloseField x) {
    CThostFtdcExchangeOptionSelfCloseField y;
    memset(&y, 0, sizeof(y));
    y.Volume = x.Volume;
    y.RequestID = x.RequestID;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.OptSelfCloseFlag = x.OptSelfCloseFlag;
    strcpy(y.OptionSelfCloseLocalID, x.OptionSelfCloseLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.OptionSelfCloseSysID, x.OptionSelfCloseSysID.c_str());
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.ExecResult = x.ExecResult;
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ExchangeOptionSelfCloseField Converter::CThostFtdcExchangeOptionSelfCloseFieldToRust(CThostFtdcExchangeOptionSelfCloseField* x) {
    if (x == nullptr)
        return ExchangeOptionSelfCloseField{.is_null = true};
    ExchangeOptionSelfCloseField y{};
    y.Volume = x->Volume;
    y.RequestID = x->RequestID;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.HedgeFlag = x->HedgeFlag;
    y.OptSelfCloseFlag = x->OptSelfCloseFlag;
    y.OptionSelfCloseLocalID = Converter::Gb2312ToRustString(x->OptionSelfCloseLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OptionSelfCloseSysID = Converter::Gb2312ToRustString(x->OptionSelfCloseSysID);
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ExecResult = x->ExecResult;
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryOptionSelfCloseActionField Converter::QryOptionSelfCloseActionFieldToCpp(QryOptionSelfCloseActionField x) {
    CThostFtdcQryOptionSelfCloseActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryOptionSelfCloseActionField Converter::CThostFtdcQryOptionSelfCloseActionFieldToRust(CThostFtdcQryOptionSelfCloseActionField* x) {
    if (x == nullptr)
        return QryOptionSelfCloseActionField{.is_null = true};
    QryOptionSelfCloseActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcExchangeOptionSelfCloseActionField Converter::ExchangeOptionSelfCloseActionFieldToCpp(ExchangeOptionSelfCloseActionField x) {
    CThostFtdcExchangeOptionSelfCloseActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OptionSelfCloseSysID, x.OptionSelfCloseSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OptionSelfCloseLocalID, x.OptionSelfCloseLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    y.OptSelfCloseFlag = x.OptSelfCloseFlag;
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

ExchangeOptionSelfCloseActionField Converter::CThostFtdcExchangeOptionSelfCloseActionFieldToRust(CThostFtdcExchangeOptionSelfCloseActionField* x) {
    if (x == nullptr)
        return ExchangeOptionSelfCloseActionField{.is_null = true};
    ExchangeOptionSelfCloseActionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OptionSelfCloseSysID = Converter::Gb2312ToRustString(x->OptionSelfCloseSysID);
    y.ActionFlag = x->ActionFlag;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OptionSelfCloseLocalID = Converter::Gb2312ToRustString(x->OptionSelfCloseLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.OptSelfCloseFlag = x->OptSelfCloseFlag;
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcSyncDelaySwapField Converter::SyncDelaySwapFieldToCpp(SyncDelaySwapField x) {
    CThostFtdcSyncDelaySwapField y;
    memset(&y, 0, sizeof(y));
    memcpy(y.DelaySwapSeqNo, x.DelaySwapSeqNo.data(), x.DelaySwapSeqNo.size() * sizeof(uint8_t));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.FromCurrencyID, x.FromCurrencyID.c_str());
    y.FromAmount = x.FromAmount;
    y.FromFrozenSwap = x.FromFrozenSwap;
    y.FromRemainSwap = x.FromRemainSwap;
    strcpy(y.ToCurrencyID, x.ToCurrencyID.c_str());
    y.ToAmount = x.ToAmount;
    y.IsManualSwap = x.IsManualSwap;
    y.IsAllRemainSetZero = x.IsAllRemainSetZero;
    return y;
}

SyncDelaySwapField Converter::CThostFtdcSyncDelaySwapFieldToRust(CThostFtdcSyncDelaySwapField* x) {
    if (x == nullptr)
        return SyncDelaySwapField{.is_null = true};
    SyncDelaySwapField y{};
    for (int i = 0; i < 15; i++)
        y.DelaySwapSeqNo.push_back(x->DelaySwapSeqNo[i]);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.FromCurrencyID = Converter::Gb2312ToRustString(x->FromCurrencyID);
    y.FromAmount = x->FromAmount;
    y.FromFrozenSwap = x->FromFrozenSwap;
    y.FromRemainSwap = x->FromRemainSwap;
    y.ToCurrencyID = Converter::Gb2312ToRustString(x->ToCurrencyID);
    y.ToAmount = x->ToAmount;
    y.IsManualSwap = x->IsManualSwap;
    y.IsAllRemainSetZero = x->IsAllRemainSetZero;
    return y;
}

CThostFtdcQrySyncDelaySwapField Converter::QrySyncDelaySwapFieldToCpp(QrySyncDelaySwapField x) {
    CThostFtdcQrySyncDelaySwapField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    memcpy(y.DelaySwapSeqNo, x.DelaySwapSeqNo.data(), x.DelaySwapSeqNo.size() * sizeof(uint8_t));
    return y;
}

QrySyncDelaySwapField Converter::CThostFtdcQrySyncDelaySwapFieldToRust(CThostFtdcQrySyncDelaySwapField* x) {
    if (x == nullptr)
        return QrySyncDelaySwapField{.is_null = true};
    QrySyncDelaySwapField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    for (int i = 0; i < 15; i++)
        y.DelaySwapSeqNo.push_back(x->DelaySwapSeqNo[i]);
    return y;
}

CThostFtdcInvestUnitField Converter::InvestUnitFieldToCpp(InvestUnitField x) {
    CThostFtdcInvestUnitField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InvestorUnitName, x.InvestorUnitName.c_str());
    strcpy(y.InvestorGroupID, x.InvestorGroupID.c_str());
    strcpy(y.CommModelID, x.CommModelID.c_str());
    strcpy(y.MarginModelID, x.MarginModelID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

InvestUnitField Converter::CThostFtdcInvestUnitFieldToRust(CThostFtdcInvestUnitField* x) {
    if (x == nullptr)
        return InvestUnitField{.is_null = true};
    InvestUnitField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InvestorUnitName = Converter::Gb2312ToRustString(x->InvestorUnitName);
    y.InvestorGroupID = Converter::Gb2312ToRustString(x->InvestorGroupID);
    y.CommModelID = Converter::Gb2312ToRustString(x->CommModelID);
    y.MarginModelID = Converter::Gb2312ToRustString(x->MarginModelID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcQryInvestUnitField Converter::QryInvestUnitFieldToCpp(QryInvestUnitField x) {
    CThostFtdcQryInvestUnitField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

QryInvestUnitField Converter::CThostFtdcQryInvestUnitFieldToRust(CThostFtdcQryInvestUnitField* x) {
    if (x == nullptr)
        return QryInvestUnitField{.is_null = true};
    QryInvestUnitField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcSecAgentCheckModeField Converter::SecAgentCheckModeFieldToCpp(SecAgentCheckModeField x) {
    CThostFtdcSecAgentCheckModeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.BrokerSecAgentID, x.BrokerSecAgentID.c_str());
    y.CheckSelfAccount = x.CheckSelfAccount;
    return y;
}

SecAgentCheckModeField Converter::CThostFtdcSecAgentCheckModeFieldToRust(CThostFtdcSecAgentCheckModeField* x) {
    if (x == nullptr)
        return SecAgentCheckModeField{.is_null = true};
    SecAgentCheckModeField y{};
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.BrokerSecAgentID = Converter::Gb2312ToRustString(x->BrokerSecAgentID);
    y.CheckSelfAccount = x->CheckSelfAccount;
    return y;
}

CThostFtdcSecAgentTradeInfoField Converter::SecAgentTradeInfoFieldToCpp(SecAgentTradeInfoField x) {
    CThostFtdcSecAgentTradeInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerSecAgentID, x.BrokerSecAgentID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

SecAgentTradeInfoField Converter::CThostFtdcSecAgentTradeInfoFieldToRust(CThostFtdcSecAgentTradeInfoField* x) {
    if (x == nullptr)
        return SecAgentTradeInfoField{.is_null = true};
    SecAgentTradeInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerSecAgentID = Converter::Gb2312ToRustString(x->BrokerSecAgentID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcMarketDataField Converter::MarketDataFieldToCpp(MarketDataField x) {
    CThostFtdcMarketDataField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.LastPrice = x.LastPrice;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.PreClosePrice = x.PreClosePrice;
    y.PreOpenInterest = x.PreOpenInterest;
    y.OpenPrice = x.OpenPrice;
    y.HighestPrice = x.HighestPrice;
    y.LowestPrice = x.LowestPrice;
    y.Volume = x.Volume;
    y.Turnover = x.Turnover;
    y.OpenInterest = x.OpenInterest;
    y.ClosePrice = x.ClosePrice;
    y.SettlementPrice = x.SettlementPrice;
    y.UpperLimitPrice = x.UpperLimitPrice;
    y.LowerLimitPrice = x.LowerLimitPrice;
    y.PreDelta = x.PreDelta;
    y.CurrDelta = x.CurrDelta;
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    y.UpdateMillisec = x.UpdateMillisec;
    strcpy(y.ActionDay, x.ActionDay.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

MarketDataField Converter::CThostFtdcMarketDataFieldToRust(CThostFtdcMarketDataField* x) {
    if (x == nullptr)
        return MarketDataField{.is_null = true};
    MarketDataField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.LastPrice = x->LastPrice;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.PreClosePrice = x->PreClosePrice;
    y.PreOpenInterest = x->PreOpenInterest;
    y.OpenPrice = x->OpenPrice;
    y.HighestPrice = x->HighestPrice;
    y.LowestPrice = x->LowestPrice;
    y.Volume = x->Volume;
    y.Turnover = x->Turnover;
    y.OpenInterest = x->OpenInterest;
    y.ClosePrice = x->ClosePrice;
    y.SettlementPrice = x->SettlementPrice;
    y.UpperLimitPrice = x->UpperLimitPrice;
    y.LowerLimitPrice = x->LowerLimitPrice;
    y.PreDelta = x->PreDelta;
    y.CurrDelta = x->CurrDelta;
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.UpdateMillisec = x->UpdateMillisec;
    y.ActionDay = Converter::Gb2312ToRustString(x->ActionDay);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcMarketDataBaseField Converter::MarketDataBaseFieldToCpp(MarketDataBaseField x) {
    CThostFtdcMarketDataBaseField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.PreClosePrice = x.PreClosePrice;
    y.PreOpenInterest = x.PreOpenInterest;
    y.PreDelta = x.PreDelta;
    return y;
}

MarketDataBaseField Converter::CThostFtdcMarketDataBaseFieldToRust(CThostFtdcMarketDataBaseField* x) {
    if (x == nullptr)
        return MarketDataBaseField{.is_null = true};
    MarketDataBaseField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.PreClosePrice = x->PreClosePrice;
    y.PreOpenInterest = x->PreOpenInterest;
    y.PreDelta = x->PreDelta;
    return y;
}

CThostFtdcMarketDataStaticField Converter::MarketDataStaticFieldToCpp(MarketDataStaticField x) {
    CThostFtdcMarketDataStaticField y;
    memset(&y, 0, sizeof(y));
    y.OpenPrice = x.OpenPrice;
    y.HighestPrice = x.HighestPrice;
    y.LowestPrice = x.LowestPrice;
    y.ClosePrice = x.ClosePrice;
    y.UpperLimitPrice = x.UpperLimitPrice;
    y.LowerLimitPrice = x.LowerLimitPrice;
    y.SettlementPrice = x.SettlementPrice;
    y.CurrDelta = x.CurrDelta;
    return y;
}

MarketDataStaticField Converter::CThostFtdcMarketDataStaticFieldToRust(CThostFtdcMarketDataStaticField* x) {
    if (x == nullptr)
        return MarketDataStaticField{.is_null = true};
    MarketDataStaticField y{};
    y.OpenPrice = x->OpenPrice;
    y.HighestPrice = x->HighestPrice;
    y.LowestPrice = x->LowestPrice;
    y.ClosePrice = x->ClosePrice;
    y.UpperLimitPrice = x->UpperLimitPrice;
    y.LowerLimitPrice = x->LowerLimitPrice;
    y.SettlementPrice = x->SettlementPrice;
    y.CurrDelta = x->CurrDelta;
    return y;
}

CThostFtdcMarketDataLastMatchField Converter::MarketDataLastMatchFieldToCpp(MarketDataLastMatchField x) {
    CThostFtdcMarketDataLastMatchField y;
    memset(&y, 0, sizeof(y));
    y.LastPrice = x.LastPrice;
    y.Volume = x.Volume;
    y.Turnover = x.Turnover;
    y.OpenInterest = x.OpenInterest;
    return y;
}

MarketDataLastMatchField Converter::CThostFtdcMarketDataLastMatchFieldToRust(CThostFtdcMarketDataLastMatchField* x) {
    if (x == nullptr)
        return MarketDataLastMatchField{.is_null = true};
    MarketDataLastMatchField y{};
    y.LastPrice = x->LastPrice;
    y.Volume = x->Volume;
    y.Turnover = x->Turnover;
    y.OpenInterest = x->OpenInterest;
    return y;
}

CThostFtdcMarketDataBestPriceField Converter::MarketDataBestPriceFieldToCpp(MarketDataBestPriceField x) {
    CThostFtdcMarketDataBestPriceField y;
    memset(&y, 0, sizeof(y));
    y.BidPrice1 = x.BidPrice1;
    y.BidVolume1 = x.BidVolume1;
    y.AskPrice1 = x.AskPrice1;
    y.AskVolume1 = x.AskVolume1;
    return y;
}

MarketDataBestPriceField Converter::CThostFtdcMarketDataBestPriceFieldToRust(CThostFtdcMarketDataBestPriceField* x) {
    if (x == nullptr)
        return MarketDataBestPriceField{.is_null = true};
    MarketDataBestPriceField y{};
    y.BidPrice1 = x->BidPrice1;
    y.BidVolume1 = x->BidVolume1;
    y.AskPrice1 = x->AskPrice1;
    y.AskVolume1 = x->AskVolume1;
    return y;
}

CThostFtdcMarketDataBid23Field Converter::MarketDataBid23FieldToCpp(MarketDataBid23Field x) {
    CThostFtdcMarketDataBid23Field y;
    memset(&y, 0, sizeof(y));
    y.BidPrice2 = x.BidPrice2;
    y.BidVolume2 = x.BidVolume2;
    y.BidPrice3 = x.BidPrice3;
    y.BidVolume3 = x.BidVolume3;
    return y;
}

MarketDataBid23Field Converter::CThostFtdcMarketDataBid23FieldToRust(CThostFtdcMarketDataBid23Field* x) {
    if (x == nullptr)
        return MarketDataBid23Field{.is_null = true};
    MarketDataBid23Field y{};
    y.BidPrice2 = x->BidPrice2;
    y.BidVolume2 = x->BidVolume2;
    y.BidPrice3 = x->BidPrice3;
    y.BidVolume3 = x->BidVolume3;
    return y;
}

CThostFtdcMarketDataAsk23Field Converter::MarketDataAsk23FieldToCpp(MarketDataAsk23Field x) {
    CThostFtdcMarketDataAsk23Field y;
    memset(&y, 0, sizeof(y));
    y.AskPrice2 = x.AskPrice2;
    y.AskVolume2 = x.AskVolume2;
    y.AskPrice3 = x.AskPrice3;
    y.AskVolume3 = x.AskVolume3;
    return y;
}

MarketDataAsk23Field Converter::CThostFtdcMarketDataAsk23FieldToRust(CThostFtdcMarketDataAsk23Field* x) {
    if (x == nullptr)
        return MarketDataAsk23Field{.is_null = true};
    MarketDataAsk23Field y{};
    y.AskPrice2 = x->AskPrice2;
    y.AskVolume2 = x->AskVolume2;
    y.AskPrice3 = x->AskPrice3;
    y.AskVolume3 = x->AskVolume3;
    return y;
}

CThostFtdcMarketDataBid45Field Converter::MarketDataBid45FieldToCpp(MarketDataBid45Field x) {
    CThostFtdcMarketDataBid45Field y;
    memset(&y, 0, sizeof(y));
    y.BidPrice4 = x.BidPrice4;
    y.BidVolume4 = x.BidVolume4;
    y.BidPrice5 = x.BidPrice5;
    y.BidVolume5 = x.BidVolume5;
    return y;
}

MarketDataBid45Field Converter::CThostFtdcMarketDataBid45FieldToRust(CThostFtdcMarketDataBid45Field* x) {
    if (x == nullptr)
        return MarketDataBid45Field{.is_null = true};
    MarketDataBid45Field y{};
    y.BidPrice4 = x->BidPrice4;
    y.BidVolume4 = x->BidVolume4;
    y.BidPrice5 = x->BidPrice5;
    y.BidVolume5 = x->BidVolume5;
    return y;
}

CThostFtdcMarketDataAsk45Field Converter::MarketDataAsk45FieldToCpp(MarketDataAsk45Field x) {
    CThostFtdcMarketDataAsk45Field y;
    memset(&y, 0, sizeof(y));
    y.AskPrice4 = x.AskPrice4;
    y.AskVolume4 = x.AskVolume4;
    y.AskPrice5 = x.AskPrice5;
    y.AskVolume5 = x.AskVolume5;
    return y;
}

MarketDataAsk45Field Converter::CThostFtdcMarketDataAsk45FieldToRust(CThostFtdcMarketDataAsk45Field* x) {
    if (x == nullptr)
        return MarketDataAsk45Field{.is_null = true};
    MarketDataAsk45Field y{};
    y.AskPrice4 = x->AskPrice4;
    y.AskVolume4 = x->AskVolume4;
    y.AskPrice5 = x->AskPrice5;
    y.AskVolume5 = x->AskVolume5;
    return y;
}

CThostFtdcMarketDataUpdateTimeField Converter::MarketDataUpdateTimeFieldToCpp(MarketDataUpdateTimeField x) {
    CThostFtdcMarketDataUpdateTimeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    y.UpdateMillisec = x.UpdateMillisec;
    strcpy(y.ActionDay, x.ActionDay.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

MarketDataUpdateTimeField Converter::CThostFtdcMarketDataUpdateTimeFieldToRust(CThostFtdcMarketDataUpdateTimeField* x) {
    if (x == nullptr)
        return MarketDataUpdateTimeField{.is_null = true};
    MarketDataUpdateTimeField y{};
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.UpdateMillisec = x->UpdateMillisec;
    y.ActionDay = Converter::Gb2312ToRustString(x->ActionDay);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcMarketDataBandingPriceField Converter::MarketDataBandingPriceFieldToCpp(MarketDataBandingPriceField x) {
    CThostFtdcMarketDataBandingPriceField y;
    memset(&y, 0, sizeof(y));
    y.BandingUpperPrice = x.BandingUpperPrice;
    y.BandingLowerPrice = x.BandingLowerPrice;
    return y;
}

MarketDataBandingPriceField Converter::CThostFtdcMarketDataBandingPriceFieldToRust(CThostFtdcMarketDataBandingPriceField* x) {
    if (x == nullptr)
        return MarketDataBandingPriceField{.is_null = true};
    MarketDataBandingPriceField y{};
    y.BandingUpperPrice = x->BandingUpperPrice;
    y.BandingLowerPrice = x->BandingLowerPrice;
    return y;
}

CThostFtdcMarketDataExchangeField Converter::MarketDataExchangeFieldToCpp(MarketDataExchangeField x) {
    CThostFtdcMarketDataExchangeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

MarketDataExchangeField Converter::CThostFtdcMarketDataExchangeFieldToRust(CThostFtdcMarketDataExchangeField* x) {
    if (x == nullptr)
        return MarketDataExchangeField{.is_null = true};
    MarketDataExchangeField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcSpecificInstrumentField Converter::SpecificInstrumentFieldToCpp(SpecificInstrumentField x) {
    CThostFtdcSpecificInstrumentField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

SpecificInstrumentField Converter::CThostFtdcSpecificInstrumentFieldToRust(CThostFtdcSpecificInstrumentField* x) {
    if (x == nullptr)
        return SpecificInstrumentField{.is_null = true};
    SpecificInstrumentField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInstrumentStatusField Converter::InstrumentStatusFieldToCpp(InstrumentStatusField x) {
    CThostFtdcInstrumentStatusField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.SettlementGroupID, x.SettlementGroupID.c_str());
    y.InstrumentStatus = x.InstrumentStatus;
    y.TradingSegmentSN = x.TradingSegmentSN;
    strcpy(y.EnterTime, x.EnterTime.c_str());
    y.EnterReason = x.EnterReason;
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

InstrumentStatusField Converter::CThostFtdcInstrumentStatusFieldToRust(CThostFtdcInstrumentStatusField* x) {
    if (x == nullptr)
        return InstrumentStatusField{.is_null = true};
    InstrumentStatusField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SettlementGroupID = Converter::Gb2312ToRustString(x->SettlementGroupID);
    y.InstrumentStatus = x->InstrumentStatus;
    y.TradingSegmentSN = x->TradingSegmentSN;
    y.EnterTime = Converter::Gb2312ToRustString(x->EnterTime);
    y.EnterReason = x->EnterReason;
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryInstrumentStatusField Converter::QryInstrumentStatusFieldToCpp(QryInstrumentStatusField x) {
    CThostFtdcQryInstrumentStatusField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    return y;
}

QryInstrumentStatusField Converter::CThostFtdcQryInstrumentStatusFieldToRust(CThostFtdcQryInstrumentStatusField* x) {
    if (x == nullptr)
        return QryInstrumentStatusField{.is_null = true};
    QryInstrumentStatusField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    return y;
}

CThostFtdcInvestorAccountField Converter::InvestorAccountFieldToCpp(InvestorAccountField x) {
    CThostFtdcInvestorAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

InvestorAccountField Converter::CThostFtdcInvestorAccountFieldToRust(CThostFtdcInvestorAccountField* x) {
    if (x == nullptr)
        return InvestorAccountField{.is_null = true};
    InvestorAccountField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcPositionProfitAlgorithmField Converter::PositionProfitAlgorithmFieldToCpp(PositionProfitAlgorithmField x) {
    CThostFtdcPositionProfitAlgorithmField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.Algorithm = x.Algorithm;
    strcpy(y.Memo, x.Memo.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

PositionProfitAlgorithmField Converter::CThostFtdcPositionProfitAlgorithmFieldToRust(CThostFtdcPositionProfitAlgorithmField* x) {
    if (x == nullptr)
        return PositionProfitAlgorithmField{.is_null = true};
    PositionProfitAlgorithmField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Algorithm = x->Algorithm;
    y.Memo = Converter::Gb2312ToRustString(x->Memo);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcDiscountField Converter::DiscountFieldToCpp(DiscountField x) {
    CThostFtdcDiscountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Discount = x.Discount;
    return y;
}

DiscountField Converter::CThostFtdcDiscountFieldToRust(CThostFtdcDiscountField* x) {
    if (x == nullptr)
        return DiscountField{.is_null = true};
    DiscountField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorRange = x->InvestorRange;
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Discount = x->Discount;
    return y;
}

CThostFtdcQryTransferBankField Converter::QryTransferBankFieldToCpp(QryTransferBankField x) {
    CThostFtdcQryTransferBankField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBrchID, x.BankBrchID.c_str());
    return y;
}

QryTransferBankField Converter::CThostFtdcQryTransferBankFieldToRust(CThostFtdcQryTransferBankField* x) {
    if (x == nullptr)
        return QryTransferBankField{.is_null = true};
    QryTransferBankField y{};
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBrchID = Converter::Gb2312ToRustString(x->BankBrchID);
    return y;
}

CThostFtdcTransferBankField Converter::TransferBankFieldToCpp(TransferBankField x) {
    CThostFtdcTransferBankField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBrchID, x.BankBrchID.c_str());
    strcpy(y.BankName, x.BankName.c_str());
    y.IsActive = x.IsActive;
    return y;
}

TransferBankField Converter::CThostFtdcTransferBankFieldToRust(CThostFtdcTransferBankField* x) {
    if (x == nullptr)
        return TransferBankField{.is_null = true};
    TransferBankField y{};
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBrchID = Converter::Gb2312ToRustString(x->BankBrchID);
    y.BankName = Converter::Gb2312ToRustString(x->BankName);
    y.IsActive = x->IsActive;
    return y;
}

CThostFtdcQryInvestorPositionDetailField Converter::QryInvestorPositionDetailFieldToCpp(QryInvestorPositionDetailField x) {
    CThostFtdcQryInvestorPositionDetailField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryInvestorPositionDetailField Converter::CThostFtdcQryInvestorPositionDetailFieldToRust(CThostFtdcQryInvestorPositionDetailField* x) {
    if (x == nullptr)
        return QryInvestorPositionDetailField{.is_null = true};
    QryInvestorPositionDetailField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcInvestorPositionDetailField Converter::InvestorPositionDetailFieldToCpp(InvestorPositionDetailField x) {
    CThostFtdcInvestorPositionDetailField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.Direction = x.Direction;
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    y.Volume = x.Volume;
    y.OpenPrice = x.OpenPrice;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.TradeType = x.TradeType;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.CloseProfitByDate = x.CloseProfitByDate;
    y.CloseProfitByTrade = x.CloseProfitByTrade;
    y.PositionProfitByDate = x.PositionProfitByDate;
    y.PositionProfitByTrade = x.PositionProfitByTrade;
    y.Margin = x.Margin;
    y.ExchMargin = x.ExchMargin;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.LastSettlementPrice = x.LastSettlementPrice;
    y.SettlementPrice = x.SettlementPrice;
    y.CloseVolume = x.CloseVolume;
    y.CloseAmount = x.CloseAmount;
    y.TimeFirstVolume = x.TimeFirstVolume;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    y.SpecPosiType = x.SpecPosiType;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    return y;
}

InvestorPositionDetailField Converter::CThostFtdcInvestorPositionDetailFieldToRust(CThostFtdcInvestorPositionDetailField* x) {
    if (x == nullptr)
        return InvestorPositionDetailField{.is_null = true};
    InvestorPositionDetailField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.Direction = x->Direction;
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.Volume = x->Volume;
    y.OpenPrice = x->OpenPrice;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.TradeType = x->TradeType;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.CloseProfitByDate = x->CloseProfitByDate;
    y.CloseProfitByTrade = x->CloseProfitByTrade;
    y.PositionProfitByDate = x->PositionProfitByDate;
    y.PositionProfitByTrade = x->PositionProfitByTrade;
    y.Margin = x->Margin;
    y.ExchMargin = x->ExchMargin;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.LastSettlementPrice = x->LastSettlementPrice;
    y.SettlementPrice = x->SettlementPrice;
    y.CloseVolume = x->CloseVolume;
    y.CloseAmount = x->CloseAmount;
    y.TimeFirstVolume = x->TimeFirstVolume;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.SpecPosiType = x->SpecPosiType;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    return y;
}

CThostFtdcTradingAccountPasswordField Converter::TradingAccountPasswordFieldToCpp(TradingAccountPasswordField x) {
    CThostFtdcTradingAccountPasswordField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

TradingAccountPasswordField Converter::CThostFtdcTradingAccountPasswordFieldToRust(CThostFtdcTradingAccountPasswordField* x) {
    if (x == nullptr)
        return TradingAccountPasswordField{.is_null = true};
    TradingAccountPasswordField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcMDTraderOfferField Converter::MDTraderOfferFieldToCpp(MDTraderOfferField x) {
    CThostFtdcMDTraderOfferField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    y.TraderConnectStatus = x.TraderConnectStatus;
    strcpy(y.ConnectRequestDate, x.ConnectRequestDate.c_str());
    strcpy(y.ConnectRequestTime, x.ConnectRequestTime.c_str());
    strcpy(y.LastReportDate, x.LastReportDate.c_str());
    strcpy(y.LastReportTime, x.LastReportTime.c_str());
    strcpy(y.ConnectDate, x.ConnectDate.c_str());
    strcpy(y.ConnectTime, x.ConnectTime.c_str());
    strcpy(y.StartDate, x.StartDate.c_str());
    strcpy(y.StartTime, x.StartTime.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.MaxTradeID, x.MaxTradeID.c_str());
    memcpy(y.MaxOrderMessageReference, x.MaxOrderMessageReference.data(), x.MaxOrderMessageReference.size() * sizeof(uint8_t));
    y.OrderCancelAlg = x.OrderCancelAlg;
    return y;
}

MDTraderOfferField Converter::CThostFtdcMDTraderOfferFieldToRust(CThostFtdcMDTraderOfferField* x) {
    if (x == nullptr)
        return MDTraderOfferField{.is_null = true};
    MDTraderOfferField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.TraderConnectStatus = x->TraderConnectStatus;
    y.ConnectRequestDate = Converter::Gb2312ToRustString(x->ConnectRequestDate);
    y.ConnectRequestTime = Converter::Gb2312ToRustString(x->ConnectRequestTime);
    y.LastReportDate = Converter::Gb2312ToRustString(x->LastReportDate);
    y.LastReportTime = Converter::Gb2312ToRustString(x->LastReportTime);
    y.ConnectDate = Converter::Gb2312ToRustString(x->ConnectDate);
    y.ConnectTime = Converter::Gb2312ToRustString(x->ConnectTime);
    y.StartDate = Converter::Gb2312ToRustString(x->StartDate);
    y.StartTime = Converter::Gb2312ToRustString(x->StartTime);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.MaxTradeID = Converter::Gb2312ToRustString(x->MaxTradeID);
    for (int i = 0; i < 7; i++)
        y.MaxOrderMessageReference.push_back(x->MaxOrderMessageReference[i]);
    y.OrderCancelAlg = x->OrderCancelAlg;
    return y;
}

CThostFtdcQryMDTraderOfferField Converter::QryMDTraderOfferFieldToCpp(QryMDTraderOfferField x) {
    CThostFtdcQryMDTraderOfferField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryMDTraderOfferField Converter::CThostFtdcQryMDTraderOfferFieldToRust(CThostFtdcQryMDTraderOfferField* x) {
    if (x == nullptr)
        return QryMDTraderOfferField{.is_null = true};
    QryMDTraderOfferField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcQryNoticeField Converter::QryNoticeFieldToCpp(QryNoticeField x) {
    CThostFtdcQryNoticeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryNoticeField Converter::CThostFtdcQryNoticeFieldToRust(CThostFtdcQryNoticeField* x) {
    if (x == nullptr)
        return QryNoticeField{.is_null = true};
    QryNoticeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcNoticeField Converter::NoticeFieldToCpp(NoticeField x) {
    CThostFtdcNoticeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    memcpy(y.Content, x.Content.data(), x.Content.size() * sizeof(uint8_t));
    memcpy(y.SequenceLabel, x.SequenceLabel.data(), x.SequenceLabel.size() * sizeof(uint8_t));
    return y;
}

NoticeField Converter::CThostFtdcNoticeFieldToRust(CThostFtdcNoticeField* x) {
    if (x == nullptr)
        return NoticeField{.is_null = true};
    NoticeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    for (int i = 0; i < 501; i++)
        y.Content.push_back(x->Content[i]);
    for (int i = 0; i < 2; i++)
        y.SequenceLabel.push_back(x->SequenceLabel[i]);
    return y;
}

CThostFtdcUserRightField Converter::UserRightFieldToCpp(UserRightField x) {
    CThostFtdcUserRightField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.UserRightType = x.UserRightType;
    y.IsForbidden = x.IsForbidden;
    return y;
}

UserRightField Converter::CThostFtdcUserRightFieldToRust(CThostFtdcUserRightField* x) {
    if (x == nullptr)
        return UserRightField{.is_null = true};
    UserRightField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserRightType = x->UserRightType;
    y.IsForbidden = x->IsForbidden;
    return y;
}

CThostFtdcQrySettlementInfoConfirmField Converter::QrySettlementInfoConfirmFieldToCpp(QrySettlementInfoConfirmField x) {
    CThostFtdcQrySettlementInfoConfirmField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

QrySettlementInfoConfirmField Converter::CThostFtdcQrySettlementInfoConfirmFieldToRust(CThostFtdcQrySettlementInfoConfirmField* x) {
    if (x == nullptr)
        return QrySettlementInfoConfirmField{.is_null = true};
    QrySettlementInfoConfirmField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcLoadSettlementInfoField Converter::LoadSettlementInfoFieldToCpp(LoadSettlementInfoField x) {
    CThostFtdcLoadSettlementInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

LoadSettlementInfoField Converter::CThostFtdcLoadSettlementInfoFieldToRust(CThostFtdcLoadSettlementInfoField* x) {
    if (x == nullptr)
        return LoadSettlementInfoField{.is_null = true};
    LoadSettlementInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcBrokerWithdrawAlgorithmField Converter::BrokerWithdrawAlgorithmFieldToCpp(BrokerWithdrawAlgorithmField x) {
    CThostFtdcBrokerWithdrawAlgorithmField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.WithdrawAlgorithm = x.WithdrawAlgorithm;
    y.UsingRatio = x.UsingRatio;
    y.IncludeCloseProfit = x.IncludeCloseProfit;
    y.AllWithoutTrade = x.AllWithoutTrade;
    y.AvailIncludeCloseProfit = x.AvailIncludeCloseProfit;
    y.IsBrokerUserEvent = x.IsBrokerUserEvent;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.FundMortgageRatio = x.FundMortgageRatio;
    y.BalanceAlgorithm = x.BalanceAlgorithm;
    return y;
}

BrokerWithdrawAlgorithmField Converter::CThostFtdcBrokerWithdrawAlgorithmFieldToRust(CThostFtdcBrokerWithdrawAlgorithmField* x) {
    if (x == nullptr)
        return BrokerWithdrawAlgorithmField{.is_null = true};
    BrokerWithdrawAlgorithmField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.WithdrawAlgorithm = x->WithdrawAlgorithm;
    y.UsingRatio = x->UsingRatio;
    y.IncludeCloseProfit = x->IncludeCloseProfit;
    y.AllWithoutTrade = x->AllWithoutTrade;
    y.AvailIncludeCloseProfit = x->AvailIncludeCloseProfit;
    y.IsBrokerUserEvent = x->IsBrokerUserEvent;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.FundMortgageRatio = x->FundMortgageRatio;
    y.BalanceAlgorithm = x->BalanceAlgorithm;
    return y;
}

CThostFtdcTradingAccountPasswordUpdateV1Field Converter::TradingAccountPasswordUpdateV1FieldToCpp(TradingAccountPasswordUpdateV1Field x) {
    CThostFtdcTradingAccountPasswordUpdateV1Field y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OldPassword, x.OldPassword.c_str());
    strcpy(y.NewPassword, x.NewPassword.c_str());
    return y;
}

TradingAccountPasswordUpdateV1Field Converter::CThostFtdcTradingAccountPasswordUpdateV1FieldToRust(CThostFtdcTradingAccountPasswordUpdateV1Field* x) {
    if (x == nullptr)
        return TradingAccountPasswordUpdateV1Field{.is_null = true};
    TradingAccountPasswordUpdateV1Field y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OldPassword = Converter::Gb2312ToRustString(x->OldPassword);
    y.NewPassword = Converter::Gb2312ToRustString(x->NewPassword);
    return y;
}

CThostFtdcTradingAccountPasswordUpdateField Converter::TradingAccountPasswordUpdateFieldToCpp(TradingAccountPasswordUpdateField x) {
    CThostFtdcTradingAccountPasswordUpdateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.OldPassword, x.OldPassword.c_str());
    strcpy(y.NewPassword, x.NewPassword.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

TradingAccountPasswordUpdateField Converter::CThostFtdcTradingAccountPasswordUpdateFieldToRust(CThostFtdcTradingAccountPasswordUpdateField* x) {
    if (x == nullptr)
        return TradingAccountPasswordUpdateField{.is_null = true};
    TradingAccountPasswordUpdateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.OldPassword = Converter::Gb2312ToRustString(x->OldPassword);
    y.NewPassword = Converter::Gb2312ToRustString(x->NewPassword);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcQryCombinationLegField Converter::QryCombinationLegFieldToCpp(QryCombinationLegField x) {
    CThostFtdcQryCombinationLegField y;
    memset(&y, 0, sizeof(y));
    y.LegID = x.LegID;
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    strcpy(y.LegInstrumentID, x.LegInstrumentID.c_str());
    return y;
}

QryCombinationLegField Converter::CThostFtdcQryCombinationLegFieldToRust(CThostFtdcQryCombinationLegField* x) {
    if (x == nullptr)
        return QryCombinationLegField{.is_null = true};
    QryCombinationLegField y{};
    y.LegID = x->LegID;
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.LegInstrumentID = Converter::Gb2312ToRustString(x->LegInstrumentID);
    return y;
}

CThostFtdcQrySyncStatusField Converter::QrySyncStatusFieldToCpp(QrySyncStatusField x) {
    CThostFtdcQrySyncStatusField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    return y;
}

QrySyncStatusField Converter::CThostFtdcQrySyncStatusFieldToRust(CThostFtdcQrySyncStatusField* x) {
    if (x == nullptr)
        return QrySyncStatusField{.is_null = true};
    QrySyncStatusField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    return y;
}

CThostFtdcCombinationLegField Converter::CombinationLegFieldToCpp(CombinationLegField x) {
    CThostFtdcCombinationLegField y;
    memset(&y, 0, sizeof(y));
    y.LegID = x.LegID;
    y.Direction = x.Direction;
    y.LegMultiple = x.LegMultiple;
    y.ImplyLevel = x.ImplyLevel;
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    strcpy(y.LegInstrumentID, x.LegInstrumentID.c_str());
    return y;
}

CombinationLegField Converter::CThostFtdcCombinationLegFieldToRust(CThostFtdcCombinationLegField* x) {
    if (x == nullptr)
        return CombinationLegField{.is_null = true};
    CombinationLegField y{};
    y.LegID = x->LegID;
    y.Direction = x->Direction;
    y.LegMultiple = x->LegMultiple;
    y.ImplyLevel = x->ImplyLevel;
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.LegInstrumentID = Converter::Gb2312ToRustString(x->LegInstrumentID);
    return y;
}

CThostFtdcSyncStatusField Converter::SyncStatusFieldToCpp(SyncStatusField x) {
    CThostFtdcSyncStatusField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.DataSyncStatus = x.DataSyncStatus;
    return y;
}

SyncStatusField Converter::CThostFtdcSyncStatusFieldToRust(CThostFtdcSyncStatusField* x) {
    if (x == nullptr)
        return SyncStatusField{.is_null = true};
    SyncStatusField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.DataSyncStatus = x->DataSyncStatus;
    return y;
}

CThostFtdcQryLinkManField Converter::QryLinkManFieldToCpp(QryLinkManField x) {
    CThostFtdcQryLinkManField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryLinkManField Converter::CThostFtdcQryLinkManFieldToRust(CThostFtdcQryLinkManField* x) {
    if (x == nullptr)
        return QryLinkManField{.is_null = true};
    QryLinkManField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcLinkManField Converter::LinkManFieldToCpp(LinkManField x) {
    CThostFtdcLinkManField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.PersonType = x.PersonType;
    y.IdentifiedCardType = x.IdentifiedCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    strcpy(y.PersonName, x.PersonName.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    y.Priority = x.Priority;
    strcpy(y.UOAZipCode, x.UOAZipCode.c_str());
    strcpy(y.PersonFullName, x.PersonFullName.c_str());
    return y;
}

LinkManField Converter::CThostFtdcLinkManFieldToRust(CThostFtdcLinkManField* x) {
    if (x == nullptr)
        return LinkManField{.is_null = true};
    LinkManField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.PersonType = x->PersonType;
    y.IdentifiedCardType = x->IdentifiedCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.PersonName = Converter::Gb2312ToRustString(x->PersonName);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Priority = x->Priority;
    y.UOAZipCode = Converter::Gb2312ToRustString(x->UOAZipCode);
    y.PersonFullName = Converter::Gb2312ToRustString(x->PersonFullName);
    return y;
}

CThostFtdcQryBrokerUserEventField Converter::QryBrokerUserEventFieldToCpp(QryBrokerUserEventField x) {
    CThostFtdcQryBrokerUserEventField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.UserEventType = x.UserEventType;
    return y;
}

QryBrokerUserEventField Converter::CThostFtdcQryBrokerUserEventFieldToRust(CThostFtdcQryBrokerUserEventField* x) {
    if (x == nullptr)
        return QryBrokerUserEventField{.is_null = true};
    QryBrokerUserEventField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserEventType = x->UserEventType;
    return y;
}

CThostFtdcBrokerUserEventField Converter::BrokerUserEventFieldToCpp(BrokerUserEventField x) {
    CThostFtdcBrokerUserEventField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.UserEventType = x.UserEventType;
    y.EventSequenceNo = x.EventSequenceNo;
    strcpy(y.EventDate, x.EventDate.c_str());
    strcpy(y.EventTime, x.EventTime.c_str());
    strcpy(y.UserEventInfo, x.UserEventInfo.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.DRIdentityID = x.DRIdentityID;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    return y;
}

BrokerUserEventField Converter::CThostFtdcBrokerUserEventFieldToRust(CThostFtdcBrokerUserEventField* x) {
    if (x == nullptr)
        return BrokerUserEventField{.is_null = true};
    BrokerUserEventField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.UserEventType = x->UserEventType;
    y.EventSequenceNo = x->EventSequenceNo;
    y.EventDate = Converter::Gb2312ToRustString(x->EventDate);
    y.EventTime = Converter::Gb2312ToRustString(x->EventTime);
    y.UserEventInfo = Converter::Gb2312ToRustString(x->UserEventInfo);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.DRIdentityID = x->DRIdentityID;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    return y;
}

CThostFtdcQryContractBankField Converter::QryContractBankFieldToCpp(QryContractBankField x) {
    CThostFtdcQryContractBankField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBrchID, x.BankBrchID.c_str());
    return y;
}

QryContractBankField Converter::CThostFtdcQryContractBankFieldToRust(CThostFtdcQryContractBankField* x) {
    if (x == nullptr)
        return QryContractBankField{.is_null = true};
    QryContractBankField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBrchID = Converter::Gb2312ToRustString(x->BankBrchID);
    return y;
}

CThostFtdcContractBankField Converter::ContractBankFieldToCpp(ContractBankField x) {
    CThostFtdcContractBankField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBrchID, x.BankBrchID.c_str());
    strcpy(y.BankName, x.BankName.c_str());
    return y;
}

ContractBankField Converter::CThostFtdcContractBankFieldToRust(CThostFtdcContractBankField* x) {
    if (x == nullptr)
        return ContractBankField{.is_null = true};
    ContractBankField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBrchID = Converter::Gb2312ToRustString(x->BankBrchID);
    y.BankName = Converter::Gb2312ToRustString(x->BankName);
    return y;
}

CThostFtdcInvestorPositionCombineDetailField Converter::InvestorPositionCombineDetailFieldToCpp(InvestorPositionCombineDetailField x) {
    CThostFtdcInvestorPositionCombineDetailField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ComTradeID, x.ComTradeID.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.Direction = x.Direction;
    y.TotalAmt = x.TotalAmt;
    y.Margin = x.Margin;
    y.ExchMargin = x.ExchMargin;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.LegID = x.LegID;
    y.LegMultiple = x.LegMultiple;
    y.TradeGroupID = x.TradeGroupID;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    return y;
}

InvestorPositionCombineDetailField Converter::CThostFtdcInvestorPositionCombineDetailFieldToRust(CThostFtdcInvestorPositionCombineDetailField* x) {
    if (x == nullptr)
        return InvestorPositionCombineDetailField{.is_null = true};
    InvestorPositionCombineDetailField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SettlementID = x->SettlementID;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ComTradeID = Converter::Gb2312ToRustString(x->ComTradeID);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.HedgeFlag = x->HedgeFlag;
    y.Direction = x->Direction;
    y.TotalAmt = x->TotalAmt;
    y.Margin = x->Margin;
    y.ExchMargin = x->ExchMargin;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.LegID = x->LegID;
    y.LegMultiple = x->LegMultiple;
    y.TradeGroupID = x->TradeGroupID;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    return y;
}

CThostFtdcParkedOrderField Converter::ParkedOrderFieldToCpp(ParkedOrderField x) {
    CThostFtdcParkedOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OrderRef, x.OrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.OrderPriceType = x.OrderPriceType;
    y.Direction = x.Direction;
    strcpy(y.CombOffsetFlag, x.CombOffsetFlag.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.LimitPrice = x.LimitPrice;
    y.VolumeTotalOriginal = x.VolumeTotalOriginal;
    y.TimeCondition = x.TimeCondition;
    strcpy(y.GTDDate, x.GTDDate.c_str());
    y.VolumeCondition = x.VolumeCondition;
    y.MinVolume = x.MinVolume;
    y.ContingentCondition = x.ContingentCondition;
    y.StopPrice = x.StopPrice;
    y.ForceCloseReason = x.ForceCloseReason;
    y.IsAutoSuspend = x.IsAutoSuspend;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.RequestID = x.RequestID;
    y.UserForceClose = x.UserForceClose;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParkedOrderID, x.ParkedOrderID.c_str());
    y.UserType = x.UserType;
    y.Status = x.Status;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    y.IsSwapOrder = x.IsSwapOrder;
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ParkedOrderField Converter::CThostFtdcParkedOrderFieldToRust(CThostFtdcParkedOrderField* x) {
    if (x == nullptr)
        return ParkedOrderField{.is_null = true};
    ParkedOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OrderPriceType = x->OrderPriceType;
    y.Direction = x->Direction;
    y.CombOffsetFlag = Converter::Gb2312ToRustString(x->CombOffsetFlag);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.LimitPrice = x->LimitPrice;
    y.VolumeTotalOriginal = x->VolumeTotalOriginal;
    y.TimeCondition = x->TimeCondition;
    y.GTDDate = Converter::Gb2312ToRustString(x->GTDDate);
    y.VolumeCondition = x->VolumeCondition;
    y.MinVolume = x->MinVolume;
    y.ContingentCondition = x->ContingentCondition;
    y.StopPrice = x->StopPrice;
    y.ForceCloseReason = x->ForceCloseReason;
    y.IsAutoSuspend = x->IsAutoSuspend;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.RequestID = x->RequestID;
    y.UserForceClose = x->UserForceClose;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParkedOrderID = Converter::Gb2312ToRustString(x->ParkedOrderID);
    y.UserType = x->UserType;
    y.Status = x->Status;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.IsSwapOrder = x->IsSwapOrder;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcParkedOrderActionField Converter::ParkedOrderActionFieldToCpp(ParkedOrderActionField x) {
    CThostFtdcParkedOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OrderActionRef = x.OrderActionRef;
    strcpy(y.OrderRef, x.OrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    y.LimitPrice = x.LimitPrice;
    y.VolumeChange = x.VolumeChange;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ParkedOrderActionID, x.ParkedOrderActionID.c_str());
    y.UserType = x.UserType;
    y.Status = x.Status;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ParkedOrderActionField Converter::CThostFtdcParkedOrderActionFieldToRust(CThostFtdcParkedOrderActionField* x) {
    if (x == nullptr)
        return ParkedOrderActionField{.is_null = true};
    ParkedOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderActionRef = x->OrderActionRef;
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.LimitPrice = x->LimitPrice;
    y.VolumeChange = x->VolumeChange;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ParkedOrderActionID = Converter::Gb2312ToRustString(x->ParkedOrderActionID);
    y.UserType = x->UserType;
    y.Status = x->Status;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryParkedOrderField Converter::QryParkedOrderFieldToCpp(QryParkedOrderField x) {
    CThostFtdcQryParkedOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryParkedOrderField Converter::CThostFtdcQryParkedOrderFieldToRust(CThostFtdcQryParkedOrderField* x) {
    if (x == nullptr)
        return QryParkedOrderField{.is_null = true};
    QryParkedOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryParkedOrderActionField Converter::QryParkedOrderActionFieldToCpp(QryParkedOrderActionField x) {
    CThostFtdcQryParkedOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryParkedOrderActionField Converter::CThostFtdcQryParkedOrderActionFieldToRust(CThostFtdcQryParkedOrderActionField* x) {
    if (x == nullptr)
        return QryParkedOrderActionField{.is_null = true};
    QryParkedOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcRemoveParkedOrderField Converter::RemoveParkedOrderFieldToCpp(RemoveParkedOrderField x) {
    CThostFtdcRemoveParkedOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ParkedOrderID, x.ParkedOrderID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

RemoveParkedOrderField Converter::CThostFtdcRemoveParkedOrderFieldToRust(CThostFtdcRemoveParkedOrderField* x) {
    if (x == nullptr)
        return RemoveParkedOrderField{.is_null = true};
    RemoveParkedOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ParkedOrderID = Converter::Gb2312ToRustString(x->ParkedOrderID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcRemoveParkedOrderActionField Converter::RemoveParkedOrderActionFieldToCpp(RemoveParkedOrderActionField x) {
    CThostFtdcRemoveParkedOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ParkedOrderActionID, x.ParkedOrderActionID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

RemoveParkedOrderActionField Converter::CThostFtdcRemoveParkedOrderActionFieldToRust(CThostFtdcRemoveParkedOrderActionField* x) {
    if (x == nullptr)
        return RemoveParkedOrderActionField{.is_null = true};
    RemoveParkedOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ParkedOrderActionID = Converter::Gb2312ToRustString(x->ParkedOrderActionID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcInvestorWithdrawAlgorithmField Converter::InvestorWithdrawAlgorithmFieldToCpp(InvestorWithdrawAlgorithmField x) {
    CThostFtdcInvestorWithdrawAlgorithmField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.UsingRatio = x.UsingRatio;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.FundMortgageRatio = x.FundMortgageRatio;
    return y;
}

InvestorWithdrawAlgorithmField Converter::CThostFtdcInvestorWithdrawAlgorithmFieldToRust(CThostFtdcInvestorWithdrawAlgorithmField* x) {
    if (x == nullptr)
        return InvestorWithdrawAlgorithmField{.is_null = true};
    InvestorWithdrawAlgorithmField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorRange = x->InvestorRange;
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.UsingRatio = x->UsingRatio;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.FundMortgageRatio = x->FundMortgageRatio;
    return y;
}

CThostFtdcQryInvestorPositionCombineDetailField Converter::QryInvestorPositionCombineDetailFieldToCpp(QryInvestorPositionCombineDetailField x) {
    CThostFtdcQryInvestorPositionCombineDetailField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    return y;
}

QryInvestorPositionCombineDetailField Converter::CThostFtdcQryInvestorPositionCombineDetailFieldToRust(CThostFtdcQryInvestorPositionCombineDetailField* x) {
    if (x == nullptr)
        return QryInvestorPositionCombineDetailField{.is_null = true};
    QryInvestorPositionCombineDetailField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    return y;
}

CThostFtdcMarketDataAveragePriceField Converter::MarketDataAveragePriceFieldToCpp(MarketDataAveragePriceField x) {
    CThostFtdcMarketDataAveragePriceField y;
    memset(&y, 0, sizeof(y));
    y.AveragePrice = x.AveragePrice;
    return y;
}

MarketDataAveragePriceField Converter::CThostFtdcMarketDataAveragePriceFieldToRust(CThostFtdcMarketDataAveragePriceField* x) {
    if (x == nullptr)
        return MarketDataAveragePriceField{.is_null = true};
    MarketDataAveragePriceField y{};
    y.AveragePrice = x->AveragePrice;
    return y;
}

CThostFtdcVerifyInvestorPasswordField Converter::VerifyInvestorPasswordFieldToCpp(VerifyInvestorPasswordField x) {
    CThostFtdcVerifyInvestorPasswordField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.Password, x.Password.c_str());
    return y;
}

VerifyInvestorPasswordField Converter::CThostFtdcVerifyInvestorPasswordFieldToRust(CThostFtdcVerifyInvestorPasswordField* x) {
    if (x == nullptr)
        return VerifyInvestorPasswordField{.is_null = true};
    VerifyInvestorPasswordField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    return y;
}

CThostFtdcUserIPField Converter::UserIPFieldToCpp(UserIPField x) {
    CThostFtdcUserIPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    memcpy(y.IPMask, x.IPMask.data(), x.IPMask.size() * sizeof(uint8_t));
    return y;
}

UserIPField Converter::CThostFtdcUserIPFieldToRust(CThostFtdcUserIPField* x) {
    if (x == nullptr)
        return UserIPField{.is_null = true};
    UserIPField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    for (int i = 0; i < 33; i++)
        y.IPMask.push_back(x->IPMask[i]);
    return y;
}

CThostFtdcTradingNoticeInfoField Converter::TradingNoticeInfoFieldToCpp(TradingNoticeInfoField x) {
    CThostFtdcTradingNoticeInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.SendTime, x.SendTime.c_str());
    memcpy(y.FieldContent, x.FieldContent.data(), x.FieldContent.size() * sizeof(uint8_t));
    y.SequenceSeries = x.SequenceSeries;
    y.SequenceNo = x.SequenceNo;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

TradingNoticeInfoField Converter::CThostFtdcTradingNoticeInfoFieldToRust(CThostFtdcTradingNoticeInfoField* x) {
    if (x == nullptr)
        return TradingNoticeInfoField{.is_null = true};
    TradingNoticeInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SendTime = Converter::Gb2312ToRustString(x->SendTime);
    for (int i = 0; i < 501; i++)
        y.FieldContent.push_back(x->FieldContent[i]);
    y.SequenceSeries = x->SequenceSeries;
    y.SequenceNo = x->SequenceNo;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcTradingNoticeField Converter::TradingNoticeFieldToCpp(TradingNoticeField x) {
    CThostFtdcTradingNoticeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.SequenceSeries = x.SequenceSeries;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.SendTime, x.SendTime.c_str());
    y.SequenceNo = x.SequenceNo;
    memcpy(y.FieldContent, x.FieldContent.data(), x.FieldContent.size() * sizeof(uint8_t));
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

TradingNoticeField Converter::CThostFtdcTradingNoticeFieldToRust(CThostFtdcTradingNoticeField* x) {
    if (x == nullptr)
        return TradingNoticeField{.is_null = true};
    TradingNoticeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorRange = x->InvestorRange;
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SequenceSeries = x->SequenceSeries;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.SendTime = Converter::Gb2312ToRustString(x->SendTime);
    y.SequenceNo = x->SequenceNo;
    for (int i = 0; i < 501; i++)
        y.FieldContent.push_back(x->FieldContent[i]);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcQryTradingNoticeField Converter::QryTradingNoticeFieldToCpp(QryTradingNoticeField x) {
    CThostFtdcQryTradingNoticeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

QryTradingNoticeField Converter::CThostFtdcQryTradingNoticeFieldToRust(CThostFtdcQryTradingNoticeField* x) {
    if (x == nullptr)
        return QryTradingNoticeField{.is_null = true};
    QryTradingNoticeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcQryErrOrderField Converter::QryErrOrderFieldToCpp(QryErrOrderField x) {
    CThostFtdcQryErrOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryErrOrderField Converter::CThostFtdcQryErrOrderFieldToRust(CThostFtdcQryErrOrderField* x) {
    if (x == nullptr)
        return QryErrOrderField{.is_null = true};
    QryErrOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcErrOrderField Converter::ErrOrderFieldToCpp(ErrOrderField x) {
    CThostFtdcErrOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OrderRef, x.OrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.OrderPriceType = x.OrderPriceType;
    y.Direction = x.Direction;
    strcpy(y.CombOffsetFlag, x.CombOffsetFlag.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.LimitPrice = x.LimitPrice;
    y.VolumeTotalOriginal = x.VolumeTotalOriginal;
    y.TimeCondition = x.TimeCondition;
    strcpy(y.GTDDate, x.GTDDate.c_str());
    y.VolumeCondition = x.VolumeCondition;
    y.MinVolume = x.MinVolume;
    y.ContingentCondition = x.ContingentCondition;
    y.StopPrice = x.StopPrice;
    y.ForceCloseReason = x.ForceCloseReason;
    y.IsAutoSuspend = x.IsAutoSuspend;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.RequestID = x.RequestID;
    y.UserForceClose = x.UserForceClose;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    y.IsSwapOrder = x.IsSwapOrder;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

ErrOrderField Converter::CThostFtdcErrOrderFieldToRust(CThostFtdcErrOrderField* x) {
    if (x == nullptr)
        return ErrOrderField{.is_null = true};
    ErrOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OrderPriceType = x->OrderPriceType;
    y.Direction = x->Direction;
    y.CombOffsetFlag = Converter::Gb2312ToRustString(x->CombOffsetFlag);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.LimitPrice = x->LimitPrice;
    y.VolumeTotalOriginal = x->VolumeTotalOriginal;
    y.TimeCondition = x->TimeCondition;
    y.GTDDate = Converter::Gb2312ToRustString(x->GTDDate);
    y.VolumeCondition = x->VolumeCondition;
    y.MinVolume = x->MinVolume;
    y.ContingentCondition = x->ContingentCondition;
    y.StopPrice = x->StopPrice;
    y.ForceCloseReason = x->ForceCloseReason;
    y.IsAutoSuspend = x->IsAutoSuspend;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.RequestID = x->RequestID;
    y.UserForceClose = x->UserForceClose;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.IsSwapOrder = x->IsSwapOrder;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcErrorConditionalOrderField Converter::ErrorConditionalOrderFieldToCpp(ErrorConditionalOrderField x) {
    CThostFtdcErrorConditionalOrderField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.OrderRef, x.OrderRef.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.OrderPriceType = x.OrderPriceType;
    y.Direction = x.Direction;
    strcpy(y.CombOffsetFlag, x.CombOffsetFlag.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.LimitPrice = x.LimitPrice;
    y.VolumeTotalOriginal = x.VolumeTotalOriginal;
    y.TimeCondition = x.TimeCondition;
    strcpy(y.GTDDate, x.GTDDate.c_str());
    y.VolumeCondition = x.VolumeCondition;
    y.MinVolume = x.MinVolume;
    y.ContingentCondition = x.ContingentCondition;
    y.StopPrice = x.StopPrice;
    y.ForceCloseReason = x.ForceCloseReason;
    y.IsAutoSuspend = x.IsAutoSuspend;
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.RequestID = x.RequestID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    y.NotifySequence = x.NotifySequence;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.OrderSource = x.OrderSource;
    y.OrderStatus = x.OrderStatus;
    y.OrderType = x.OrderType;
    y.VolumeTraded = x.VolumeTraded;
    y.VolumeTotal = x.VolumeTotal;
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.ActiveTime, x.ActiveTime.c_str());
    strcpy(y.SuspendTime, x.SuspendTime.c_str());
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    strcpy(y.ActiveTraderID, x.ActiveTraderID.c_str());
    strcpy(y.ClearingPartID, x.ClearingPartID.c_str());
    y.SequenceNo = x.SequenceNo;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    y.UserForceClose = x.UserForceClose;
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerOrderSeq = x.BrokerOrderSeq;
    strcpy(y.RelativeOrderSysID, x.RelativeOrderSysID.c_str());
    y.ZCETotalTradedVolume = x.ZCETotalTradedVolume;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    y.IsSwapOrder = x.IsSwapOrder;
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

ErrorConditionalOrderField Converter::CThostFtdcErrorConditionalOrderFieldToRust(CThostFtdcErrorConditionalOrderField* x) {
    if (x == nullptr)
        return ErrorConditionalOrderField{.is_null = true};
    ErrorConditionalOrderField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OrderPriceType = x->OrderPriceType;
    y.Direction = x->Direction;
    y.CombOffsetFlag = Converter::Gb2312ToRustString(x->CombOffsetFlag);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.LimitPrice = x->LimitPrice;
    y.VolumeTotalOriginal = x->VolumeTotalOriginal;
    y.TimeCondition = x->TimeCondition;
    y.GTDDate = Converter::Gb2312ToRustString(x->GTDDate);
    y.VolumeCondition = x->VolumeCondition;
    y.MinVolume = x->MinVolume;
    y.ContingentCondition = x->ContingentCondition;
    y.StopPrice = x->StopPrice;
    y.ForceCloseReason = x->ForceCloseReason;
    y.IsAutoSuspend = x->IsAutoSuspend;
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.RequestID = x->RequestID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.NotifySequence = x->NotifySequence;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.OrderSource = x->OrderSource;
    y.OrderStatus = x->OrderStatus;
    y.OrderType = x->OrderType;
    y.VolumeTraded = x->VolumeTraded;
    y.VolumeTotal = x->VolumeTotal;
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.ActiveTime = Converter::Gb2312ToRustString(x->ActiveTime);
    y.SuspendTime = Converter::Gb2312ToRustString(x->SuspendTime);
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ActiveTraderID = Converter::Gb2312ToRustString(x->ActiveTraderID);
    y.ClearingPartID = Converter::Gb2312ToRustString(x->ClearingPartID);
    y.SequenceNo = x->SequenceNo;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.UserForceClose = x->UserForceClose;
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerOrderSeq = x->BrokerOrderSeq;
    y.RelativeOrderSysID = Converter::Gb2312ToRustString(x->RelativeOrderSysID);
    y.ZCETotalTradedVolume = x->ZCETotalTradedVolume;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.IsSwapOrder = x->IsSwapOrder;
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryErrOrderActionField Converter::QryErrOrderActionFieldToCpp(QryErrOrderActionField x) {
    CThostFtdcQryErrOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryErrOrderActionField Converter::CThostFtdcQryErrOrderActionFieldToRust(CThostFtdcQryErrOrderActionField* x) {
    if (x == nullptr)
        return QryErrOrderActionField{.is_null = true};
    QryErrOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcErrOrderActionField Converter::ErrOrderActionFieldToCpp(ErrOrderActionField x) {
    CThostFtdcErrOrderActionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OrderActionRef = x.OrderActionRef;
    strcpy(y.OrderRef, x.OrderRef.c_str());
    y.RequestID = x.RequestID;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.OrderSysID, x.OrderSysID.c_str());
    y.ActionFlag = x.ActionFlag;
    y.LimitPrice = x.LimitPrice;
    y.VolumeChange = x.VolumeChange;
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.OrderLocalID, x.OrderLocalID.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.BusinessUnit, x.BusinessUnit.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.BranchID, x.BranchID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.OrderMemo, x.OrderMemo.c_str());
    y.SessionReqSeq = x.SessionReqSeq;
    return y;
}

ErrOrderActionField Converter::CThostFtdcErrOrderActionFieldToRust(CThostFtdcErrOrderActionField* x) {
    if (x == nullptr)
        return ErrOrderActionField{.is_null = true};
    ErrOrderActionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OrderActionRef = x->OrderActionRef;
    y.OrderRef = Converter::Gb2312ToRustString(x->OrderRef);
    y.RequestID = x->RequestID;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.OrderSysID = Converter::Gb2312ToRustString(x->OrderSysID);
    y.ActionFlag = x->ActionFlag;
    y.LimitPrice = x->LimitPrice;
    y.VolumeChange = x->VolumeChange;
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderLocalID = Converter::Gb2312ToRustString(x->OrderLocalID);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.BusinessUnit = Converter::Gb2312ToRustString(x->BusinessUnit);
    y.OrderActionStatus = x->OrderActionStatus;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.BranchID = Converter::Gb2312ToRustString(x->BranchID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.OrderMemo = Converter::Gb2312ToRustString(x->OrderMemo);
    y.SessionReqSeq = x->SessionReqSeq;
    return y;
}

CThostFtdcQryExchangeSequenceField Converter::QryExchangeSequenceFieldToCpp(QryExchangeSequenceField x) {
    CThostFtdcQryExchangeSequenceField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QryExchangeSequenceField Converter::CThostFtdcQryExchangeSequenceFieldToRust(CThostFtdcQryExchangeSequenceField* x) {
    if (x == nullptr)
        return QryExchangeSequenceField{.is_null = true};
    QryExchangeSequenceField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcExchangeSequenceField Converter::ExchangeSequenceFieldToCpp(ExchangeSequenceField x) {
    CThostFtdcExchangeSequenceField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SequenceNo = x.SequenceNo;
    y.MarketStatus = x.MarketStatus;
    return y;
}

ExchangeSequenceField Converter::CThostFtdcExchangeSequenceFieldToRust(CThostFtdcExchangeSequenceField* x) {
    if (x == nullptr)
        return ExchangeSequenceField{.is_null = true};
    ExchangeSequenceField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SequenceNo = x->SequenceNo;
    y.MarketStatus = x->MarketStatus;
    return y;
}

CThostFtdcQryMaxOrderVolumeWithPriceField Converter::QryMaxOrderVolumeWithPriceFieldToCpp(QryMaxOrderVolumeWithPriceField x) {
    CThostFtdcQryMaxOrderVolumeWithPriceField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Direction = x.Direction;
    y.OffsetFlag = x.OffsetFlag;
    y.HedgeFlag = x.HedgeFlag;
    y.MaxVolume = x.MaxVolume;
    y.Price = x.Price;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryMaxOrderVolumeWithPriceField Converter::CThostFtdcQryMaxOrderVolumeWithPriceFieldToRust(CThostFtdcQryMaxOrderVolumeWithPriceField* x) {
    if (x == nullptr)
        return QryMaxOrderVolumeWithPriceField{.is_null = true};
    QryMaxOrderVolumeWithPriceField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Direction = x->Direction;
    y.OffsetFlag = x->OffsetFlag;
    y.HedgeFlag = x->HedgeFlag;
    y.MaxVolume = x->MaxVolume;
    y.Price = x->Price;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryBrokerTradingParamsField Converter::QryBrokerTradingParamsFieldToCpp(QryBrokerTradingParamsField x) {
    CThostFtdcQryBrokerTradingParamsField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    return y;
}

QryBrokerTradingParamsField Converter::CThostFtdcQryBrokerTradingParamsFieldToRust(CThostFtdcQryBrokerTradingParamsField* x) {
    if (x == nullptr)
        return QryBrokerTradingParamsField{.is_null = true};
    QryBrokerTradingParamsField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    return y;
}

CThostFtdcBrokerTradingParamsField Converter::BrokerTradingParamsFieldToCpp(BrokerTradingParamsField x) {
    CThostFtdcBrokerTradingParamsField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.MarginPriceType = x.MarginPriceType;
    y.Algorithm = x.Algorithm;
    y.AvailIncludeCloseProfit = x.AvailIncludeCloseProfit;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.OptionRoyaltyPriceType = x.OptionRoyaltyPriceType;
    strcpy(y.AccountID, x.AccountID.c_str());
    return y;
}

BrokerTradingParamsField Converter::CThostFtdcBrokerTradingParamsFieldToRust(CThostFtdcBrokerTradingParamsField* x) {
    if (x == nullptr)
        return BrokerTradingParamsField{.is_null = true};
    BrokerTradingParamsField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.MarginPriceType = x->MarginPriceType;
    y.Algorithm = x->Algorithm;
    y.AvailIncludeCloseProfit = x->AvailIncludeCloseProfit;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.OptionRoyaltyPriceType = x->OptionRoyaltyPriceType;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    return y;
}

CThostFtdcQryBrokerTradingAlgosField Converter::QryBrokerTradingAlgosFieldToCpp(QryBrokerTradingAlgosField x) {
    CThostFtdcQryBrokerTradingAlgosField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryBrokerTradingAlgosField Converter::CThostFtdcQryBrokerTradingAlgosFieldToRust(CThostFtdcQryBrokerTradingAlgosField* x) {
    if (x == nullptr)
        return QryBrokerTradingAlgosField{.is_null = true};
    QryBrokerTradingAlgosField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcBrokerTradingAlgosField Converter::BrokerTradingAlgosFieldToCpp(BrokerTradingAlgosField x) {
    CThostFtdcBrokerTradingAlgosField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.HandlePositionAlgoID = x.HandlePositionAlgoID;
    y.FindMarginRateAlgoID = x.FindMarginRateAlgoID;
    y.HandleTradingAccountAlgoID = x.HandleTradingAccountAlgoID;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

BrokerTradingAlgosField Converter::CThostFtdcBrokerTradingAlgosFieldToRust(CThostFtdcBrokerTradingAlgosField* x) {
    if (x == nullptr)
        return BrokerTradingAlgosField{.is_null = true};
    BrokerTradingAlgosField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.HandlePositionAlgoID = x->HandlePositionAlgoID;
    y.FindMarginRateAlgoID = x->FindMarginRateAlgoID;
    y.HandleTradingAccountAlgoID = x->HandleTradingAccountAlgoID;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQueryBrokerDepositField Converter::QueryBrokerDepositFieldToCpp(QueryBrokerDepositField x) {
    CThostFtdcQueryBrokerDepositField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    return y;
}

QueryBrokerDepositField Converter::CThostFtdcQueryBrokerDepositFieldToRust(CThostFtdcQueryBrokerDepositField* x) {
    if (x == nullptr)
        return QueryBrokerDepositField{.is_null = true};
    QueryBrokerDepositField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    return y;
}

CThostFtdcBrokerDepositField Converter::BrokerDepositFieldToCpp(BrokerDepositField x) {
    CThostFtdcBrokerDepositField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.PreBalance = x.PreBalance;
    y.CurrMargin = x.CurrMargin;
    y.CloseProfit = x.CloseProfit;
    y.Balance = x.Balance;
    y.Deposit = x.Deposit;
    y.Withdraw = x.Withdraw;
    y.Available = x.Available;
    y.Reserve = x.Reserve;
    y.FrozenMargin = x.FrozenMargin;
    return y;
}

BrokerDepositField Converter::CThostFtdcBrokerDepositFieldToRust(CThostFtdcBrokerDepositField* x) {
    if (x == nullptr)
        return BrokerDepositField{.is_null = true};
    BrokerDepositField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.PreBalance = x->PreBalance;
    y.CurrMargin = x->CurrMargin;
    y.CloseProfit = x->CloseProfit;
    y.Balance = x->Balance;
    y.Deposit = x->Deposit;
    y.Withdraw = x->Withdraw;
    y.Available = x->Available;
    y.Reserve = x->Reserve;
    y.FrozenMargin = x->FrozenMargin;
    return y;
}

CThostFtdcQryCFMMCBrokerKeyField Converter::QryCFMMCBrokerKeyFieldToCpp(QryCFMMCBrokerKeyField x) {
    CThostFtdcQryCFMMCBrokerKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryCFMMCBrokerKeyField Converter::CThostFtdcQryCFMMCBrokerKeyFieldToRust(CThostFtdcQryCFMMCBrokerKeyField* x) {
    if (x == nullptr)
        return QryCFMMCBrokerKeyField{.is_null = true};
    QryCFMMCBrokerKeyField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcCFMMCBrokerKeyField Converter::CFMMCBrokerKeyFieldToCpp(CFMMCBrokerKeyField x) {
    CThostFtdcCFMMCBrokerKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.CreateDate, x.CreateDate.c_str());
    strcpy(y.CreateTime, x.CreateTime.c_str());
    y.KeyID = x.KeyID;
    memcpy(y.CurrentKey, x.CurrentKey.data(), x.CurrentKey.size() * sizeof(uint8_t));
    y.KeyKind = x.KeyKind;
    return y;
}

CFMMCBrokerKeyField Converter::CThostFtdcCFMMCBrokerKeyFieldToRust(CThostFtdcCFMMCBrokerKeyField* x) {
    if (x == nullptr)
        return CFMMCBrokerKeyField{.is_null = true};
    CFMMCBrokerKeyField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.CreateDate = Converter::Gb2312ToRustString(x->CreateDate);
    y.CreateTime = Converter::Gb2312ToRustString(x->CreateTime);
    y.KeyID = x->KeyID;
    for (int i = 0; i < 21; i++)
        y.CurrentKey.push_back(x->CurrentKey[i]);
    y.KeyKind = x->KeyKind;
    return y;
}

CThostFtdcCFMMCTradingAccountKeyField Converter::CFMMCTradingAccountKeyFieldToCpp(CFMMCTradingAccountKeyField x) {
    CThostFtdcCFMMCTradingAccountKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.KeyID = x.KeyID;
    memcpy(y.CurrentKey, x.CurrentKey.data(), x.CurrentKey.size() * sizeof(uint8_t));
    return y;
}

CFMMCTradingAccountKeyField Converter::CThostFtdcCFMMCTradingAccountKeyFieldToRust(CThostFtdcCFMMCTradingAccountKeyField* x) {
    if (x == nullptr)
        return CFMMCTradingAccountKeyField{.is_null = true};
    CFMMCTradingAccountKeyField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.KeyID = x->KeyID;
    for (int i = 0; i < 21; i++)
        y.CurrentKey.push_back(x->CurrentKey[i]);
    return y;
}

CThostFtdcQryCFMMCTradingAccountKeyField Converter::QryCFMMCTradingAccountKeyFieldToCpp(QryCFMMCTradingAccountKeyField x) {
    CThostFtdcQryCFMMCTradingAccountKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryCFMMCTradingAccountKeyField Converter::CThostFtdcQryCFMMCTradingAccountKeyFieldToRust(CThostFtdcQryCFMMCTradingAccountKeyField* x) {
    if (x == nullptr)
        return QryCFMMCTradingAccountKeyField{.is_null = true};
    QryCFMMCTradingAccountKeyField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcBrokerUserOTPParamField Converter::BrokerUserOTPParamFieldToCpp(BrokerUserOTPParamField x) {
    CThostFtdcBrokerUserOTPParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.OTPVendorsID, x.OTPVendorsID.c_str());
    memcpy(y.SerialNumber, x.SerialNumber.data(), x.SerialNumber.size() * sizeof(uint8_t));
    memcpy(y.AuthKey, x.AuthKey.data(), x.AuthKey.size() * sizeof(uint8_t));
    y.LastDrift = x.LastDrift;
    y.LastSuccess = x.LastSuccess;
    y.OTPType = x.OTPType;
    return y;
}

BrokerUserOTPParamField Converter::CThostFtdcBrokerUserOTPParamFieldToRust(CThostFtdcBrokerUserOTPParamField* x) {
    if (x == nullptr)
        return BrokerUserOTPParamField{.is_null = true};
    BrokerUserOTPParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OTPVendorsID = Converter::Gb2312ToRustString(x->OTPVendorsID);
    for (int i = 0; i < 17; i++)
        y.SerialNumber.push_back(x->SerialNumber[i]);
    for (int i = 0; i < 41; i++)
        y.AuthKey.push_back(x->AuthKey[i]);
    y.LastDrift = x->LastDrift;
    y.LastSuccess = x->LastSuccess;
    y.OTPType = x->OTPType;
    return y;
}

CThostFtdcManualSyncBrokerUserOTPField Converter::ManualSyncBrokerUserOTPFieldToCpp(ManualSyncBrokerUserOTPField x) {
    CThostFtdcManualSyncBrokerUserOTPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.OTPType = x.OTPType;
    memcpy(y.FirstOTP, x.FirstOTP.data(), x.FirstOTP.size() * sizeof(uint8_t));
    memcpy(y.SecondOTP, x.SecondOTP.data(), x.SecondOTP.size() * sizeof(uint8_t));
    return y;
}

ManualSyncBrokerUserOTPField Converter::CThostFtdcManualSyncBrokerUserOTPFieldToRust(CThostFtdcManualSyncBrokerUserOTPField* x) {
    if (x == nullptr)
        return ManualSyncBrokerUserOTPField{.is_null = true};
    ManualSyncBrokerUserOTPField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OTPType = x->OTPType;
    for (int i = 0; i < 41; i++)
        y.FirstOTP.push_back(x->FirstOTP[i]);
    for (int i = 0; i < 41; i++)
        y.SecondOTP.push_back(x->SecondOTP[i]);
    return y;
}

CThostFtdcCommRateModelField Converter::CommRateModelFieldToCpp(CommRateModelField x) {
    CThostFtdcCommRateModelField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.CommModelID, x.CommModelID.c_str());
    strcpy(y.CommModelName, x.CommModelName.c_str());
    return y;
}

CommRateModelField Converter::CThostFtdcCommRateModelFieldToRust(CThostFtdcCommRateModelField* x) {
    if (x == nullptr)
        return CommRateModelField{.is_null = true};
    CommRateModelField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.CommModelID = Converter::Gb2312ToRustString(x->CommModelID);
    y.CommModelName = Converter::Gb2312ToRustString(x->CommModelName);
    return y;
}

CThostFtdcQryCommRateModelField Converter::QryCommRateModelFieldToCpp(QryCommRateModelField x) {
    CThostFtdcQryCommRateModelField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.CommModelID, x.CommModelID.c_str());
    return y;
}

QryCommRateModelField Converter::CThostFtdcQryCommRateModelFieldToRust(CThostFtdcQryCommRateModelField* x) {
    if (x == nullptr)
        return QryCommRateModelField{.is_null = true};
    QryCommRateModelField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.CommModelID = Converter::Gb2312ToRustString(x->CommModelID);
    return y;
}

CThostFtdcMarginModelField Converter::MarginModelFieldToCpp(MarginModelField x) {
    CThostFtdcMarginModelField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.MarginModelID, x.MarginModelID.c_str());
    strcpy(y.MarginModelName, x.MarginModelName.c_str());
    return y;
}

MarginModelField Converter::CThostFtdcMarginModelFieldToRust(CThostFtdcMarginModelField* x) {
    if (x == nullptr)
        return MarginModelField{.is_null = true};
    MarginModelField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.MarginModelID = Converter::Gb2312ToRustString(x->MarginModelID);
    y.MarginModelName = Converter::Gb2312ToRustString(x->MarginModelName);
    return y;
}

CThostFtdcQryMarginModelField Converter::QryMarginModelFieldToCpp(QryMarginModelField x) {
    CThostFtdcQryMarginModelField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.MarginModelID, x.MarginModelID.c_str());
    return y;
}

QryMarginModelField Converter::CThostFtdcQryMarginModelFieldToRust(CThostFtdcQryMarginModelField* x) {
    if (x == nullptr)
        return QryMarginModelField{.is_null = true};
    QryMarginModelField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.MarginModelID = Converter::Gb2312ToRustString(x->MarginModelID);
    return y;
}

CThostFtdcEWarrantOffsetField Converter::EWarrantOffsetFieldToCpp(EWarrantOffsetField x) {
    CThostFtdcEWarrantOffsetField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.Direction = x.Direction;
    y.HedgeFlag = x.HedgeFlag;
    y.Volume = x.Volume;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

EWarrantOffsetField Converter::CThostFtdcEWarrantOffsetFieldToRust(CThostFtdcEWarrantOffsetField* x) {
    if (x == nullptr)
        return EWarrantOffsetField{.is_null = true};
    EWarrantOffsetField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.Direction = x->Direction;
    y.HedgeFlag = x->HedgeFlag;
    y.Volume = x->Volume;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryEWarrantOffsetField Converter::QryEWarrantOffsetFieldToCpp(QryEWarrantOffsetField x) {
    CThostFtdcQryEWarrantOffsetField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryEWarrantOffsetField Converter::CThostFtdcQryEWarrantOffsetFieldToRust(CThostFtdcQryEWarrantOffsetField* x) {
    if (x == nullptr)
        return QryEWarrantOffsetField{.is_null = true};
    QryEWarrantOffsetField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryInvestorProductGroupMarginField Converter::QryInvestorProductGroupMarginFieldToCpp(QryInvestorProductGroupMarginField x) {
    CThostFtdcQryInvestorProductGroupMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

QryInvestorProductGroupMarginField Converter::CThostFtdcQryInvestorProductGroupMarginFieldToRust(CThostFtdcQryInvestorProductGroupMarginField* x) {
    if (x == nullptr)
        return QryInvestorProductGroupMarginField{.is_null = true};
    QryInvestorProductGroupMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcInvestorProductGroupMarginField Converter::InvestorProductGroupMarginFieldToCpp(InvestorProductGroupMarginField x) {
    CThostFtdcInvestorProductGroupMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.FrozenMargin = x.FrozenMargin;
    y.LongFrozenMargin = x.LongFrozenMargin;
    y.ShortFrozenMargin = x.ShortFrozenMargin;
    y.UseMargin = x.UseMargin;
    y.LongUseMargin = x.LongUseMargin;
    y.ShortUseMargin = x.ShortUseMargin;
    y.ExchMargin = x.ExchMargin;
    y.LongExchMargin = x.LongExchMargin;
    y.ShortExchMargin = x.ShortExchMargin;
    y.CloseProfit = x.CloseProfit;
    y.FrozenCommission = x.FrozenCommission;
    y.Commission = x.Commission;
    y.FrozenCash = x.FrozenCash;
    y.CashIn = x.CashIn;
    y.PositionProfit = x.PositionProfit;
    y.OffsetAmount = x.OffsetAmount;
    y.LongOffsetAmount = x.LongOffsetAmount;
    y.ShortOffsetAmount = x.ShortOffsetAmount;
    y.ExchOffsetAmount = x.ExchOffsetAmount;
    y.LongExchOffsetAmount = x.LongExchOffsetAmount;
    y.ShortExchOffsetAmount = x.ShortExchOffsetAmount;
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

InvestorProductGroupMarginField Converter::CThostFtdcInvestorProductGroupMarginFieldToRust(CThostFtdcInvestorProductGroupMarginField* x) {
    if (x == nullptr)
        return InvestorProductGroupMarginField{.is_null = true};
    InvestorProductGroupMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.FrozenMargin = x->FrozenMargin;
    y.LongFrozenMargin = x->LongFrozenMargin;
    y.ShortFrozenMargin = x->ShortFrozenMargin;
    y.UseMargin = x->UseMargin;
    y.LongUseMargin = x->LongUseMargin;
    y.ShortUseMargin = x->ShortUseMargin;
    y.ExchMargin = x->ExchMargin;
    y.LongExchMargin = x->LongExchMargin;
    y.ShortExchMargin = x->ShortExchMargin;
    y.CloseProfit = x->CloseProfit;
    y.FrozenCommission = x->FrozenCommission;
    y.Commission = x->Commission;
    y.FrozenCash = x->FrozenCash;
    y.CashIn = x->CashIn;
    y.PositionProfit = x->PositionProfit;
    y.OffsetAmount = x->OffsetAmount;
    y.LongOffsetAmount = x->LongOffsetAmount;
    y.ShortOffsetAmount = x->ShortOffsetAmount;
    y.ExchOffsetAmount = x->ExchOffsetAmount;
    y.LongExchOffsetAmount = x->LongExchOffsetAmount;
    y.ShortExchOffsetAmount = x->ShortExchOffsetAmount;
    y.HedgeFlag = x->HedgeFlag;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcQueryCFMMCTradingAccountTokenField Converter::QueryCFMMCTradingAccountTokenFieldToCpp(QueryCFMMCTradingAccountTokenField x) {
    CThostFtdcQueryCFMMCTradingAccountTokenField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    return y;
}

QueryCFMMCTradingAccountTokenField Converter::CThostFtdcQueryCFMMCTradingAccountTokenFieldToRust(CThostFtdcQueryCFMMCTradingAccountTokenField* x) {
    if (x == nullptr)
        return QueryCFMMCTradingAccountTokenField{.is_null = true};
    QueryCFMMCTradingAccountTokenField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    return y;
}

CThostFtdcCFMMCTradingAccountTokenField Converter::CFMMCTradingAccountTokenFieldToCpp(CFMMCTradingAccountTokenField x) {
    CThostFtdcCFMMCTradingAccountTokenField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.KeyID = x.KeyID;
    memcpy(y.Token, x.Token.data(), x.Token.size() * sizeof(uint8_t));
    return y;
}

CFMMCTradingAccountTokenField Converter::CThostFtdcCFMMCTradingAccountTokenFieldToRust(CThostFtdcCFMMCTradingAccountTokenField* x) {
    if (x == nullptr)
        return CFMMCTradingAccountTokenField{.is_null = true};
    CFMMCTradingAccountTokenField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.KeyID = x->KeyID;
    for (int i = 0; i < 21; i++)
        y.Token.push_back(x->Token[i]);
    return y;
}

CThostFtdcQryProductGroupField Converter::QryProductGroupFieldToCpp(QryProductGroupField x) {
    CThostFtdcQryProductGroupField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QryProductGroupField Converter::CThostFtdcQryProductGroupFieldToRust(CThostFtdcQryProductGroupField* x) {
    if (x == nullptr)
        return QryProductGroupField{.is_null = true};
    QryProductGroupField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcProductGroupField Converter::ProductGroupFieldToCpp(ProductGroupField x) {
    CThostFtdcProductGroupField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

ProductGroupField Converter::CThostFtdcProductGroupFieldToRust(CThostFtdcProductGroupField* x) {
    if (x == nullptr)
        return ProductGroupField{.is_null = true};
    ProductGroupField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcBulletinField Converter::BulletinFieldToCpp(BulletinField x) {
    CThostFtdcBulletinField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.BulletinID = x.BulletinID;
    y.SequenceNo = x.SequenceNo;
    memcpy(y.NewsType, x.NewsType.data(), x.NewsType.size() * sizeof(uint8_t));
    y.NewsUrgency = x.NewsUrgency;
    strcpy(y.SendTime, x.SendTime.c_str());
    memcpy(y.Abstract, x.Abstract.data(), x.Abstract.size() * sizeof(uint8_t));
    memcpy(y.ComeFrom, x.ComeFrom.data(), x.ComeFrom.size() * sizeof(uint8_t));
    memcpy(y.Content, x.Content.data(), x.Content.size() * sizeof(uint8_t));
    memcpy(y.URLLink, x.URLLink.data(), x.URLLink.size() * sizeof(uint8_t));
    strcpy(y.MarketID, x.MarketID.c_str());
    return y;
}

BulletinField Converter::CThostFtdcBulletinFieldToRust(CThostFtdcBulletinField* x) {
    if (x == nullptr)
        return BulletinField{.is_null = true};
    BulletinField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BulletinID = x->BulletinID;
    y.SequenceNo = x->SequenceNo;
    for (int i = 0; i < 3; i++)
        y.NewsType.push_back(x->NewsType[i]);
    y.NewsUrgency = x->NewsUrgency;
    y.SendTime = Converter::Gb2312ToRustString(x->SendTime);
    for (int i = 0; i < 81; i++)
        y.Abstract.push_back(x->Abstract[i]);
    for (int i = 0; i < 21; i++)
        y.ComeFrom.push_back(x->ComeFrom[i]);
    for (int i = 0; i < 501; i++)
        y.Content.push_back(x->Content[i]);
    for (int i = 0; i < 201; i++)
        y.URLLink.push_back(x->URLLink[i]);
    y.MarketID = Converter::Gb2312ToRustString(x->MarketID);
    return y;
}

CThostFtdcQryBulletinField Converter::QryBulletinFieldToCpp(QryBulletinField x) {
    CThostFtdcQryBulletinField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.BulletinID = x.BulletinID;
    y.SequenceNo = x.SequenceNo;
    memcpy(y.NewsType, x.NewsType.data(), x.NewsType.size() * sizeof(uint8_t));
    y.NewsUrgency = x.NewsUrgency;
    return y;
}

QryBulletinField Converter::CThostFtdcQryBulletinFieldToRust(CThostFtdcQryBulletinField* x) {
    if (x == nullptr)
        return QryBulletinField{.is_null = true};
    QryBulletinField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BulletinID = x->BulletinID;
    y.SequenceNo = x->SequenceNo;
    for (int i = 0; i < 3; i++)
        y.NewsType.push_back(x->NewsType[i]);
    y.NewsUrgency = x->NewsUrgency;
    return y;
}

CThostFtdcMulticastInstrumentField Converter::MulticastInstrumentFieldToCpp(MulticastInstrumentField x) {
    CThostFtdcMulticastInstrumentField y;
    memset(&y, 0, sizeof(y));
    y.TopicID = x.TopicID;
    y.InstrumentNo = x.InstrumentNo;
    y.CodePrice = x.CodePrice;
    y.VolumeMultiple = x.VolumeMultiple;
    y.PriceTick = x.PriceTick;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

MulticastInstrumentField Converter::CThostFtdcMulticastInstrumentFieldToRust(CThostFtdcMulticastInstrumentField* x) {
    if (x == nullptr)
        return MulticastInstrumentField{.is_null = true};
    MulticastInstrumentField y{};
    y.TopicID = x->TopicID;
    y.InstrumentNo = x->InstrumentNo;
    y.CodePrice = x->CodePrice;
    y.VolumeMultiple = x->VolumeMultiple;
    y.PriceTick = x->PriceTick;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryMulticastInstrumentField Converter::QryMulticastInstrumentFieldToCpp(QryMulticastInstrumentField x) {
    CThostFtdcQryMulticastInstrumentField y;
    memset(&y, 0, sizeof(y));
    y.TopicID = x.TopicID;
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryMulticastInstrumentField Converter::CThostFtdcQryMulticastInstrumentFieldToRust(CThostFtdcQryMulticastInstrumentField* x) {
    if (x == nullptr)
        return QryMulticastInstrumentField{.is_null = true};
    QryMulticastInstrumentField y{};
    y.TopicID = x->TopicID;
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcAppIDAuthAssignField Converter::AppIDAuthAssignFieldToCpp(AppIDAuthAssignField x) {
    CThostFtdcAppIDAuthAssignField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    y.DRIdentityID = x.DRIdentityID;
    return y;
}

AppIDAuthAssignField Converter::CThostFtdcAppIDAuthAssignFieldToRust(CThostFtdcAppIDAuthAssignField* x) {
    if (x == nullptr)
        return AppIDAuthAssignField{.is_null = true};
    AppIDAuthAssignField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    y.DRIdentityID = x->DRIdentityID;
    return y;
}

CThostFtdcReqOpenAccountField Converter::ReqOpenAccountFieldToCpp(ReqOpenAccountField x) {
    CThostFtdcReqOpenAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.CashExchangeCode = x.CashExchangeCode;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.TID = x.TID;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqOpenAccountField Converter::CThostFtdcReqOpenAccountFieldToRust(CThostFtdcReqOpenAccountField* x) {
    if (x == nullptr)
        return ReqOpenAccountField{.is_null = true};
    ReqOpenAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.CashExchangeCode = x->CashExchangeCode;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.TID = x->TID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcReqCancelAccountField Converter::ReqCancelAccountFieldToCpp(ReqCancelAccountField x) {
    CThostFtdcReqCancelAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.CashExchangeCode = x.CashExchangeCode;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.TID = x.TID;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqCancelAccountField Converter::CThostFtdcReqCancelAccountFieldToRust(CThostFtdcReqCancelAccountField* x) {
    if (x == nullptr)
        return ReqCancelAccountField{.is_null = true};
    ReqCancelAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.CashExchangeCode = x->CashExchangeCode;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.TID = x->TID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcReqChangeAccountField Converter::ReqChangeAccountFieldToCpp(ReqChangeAccountField x) {
    CThostFtdcReqChangeAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.NewBankAccount, x.NewBankAccount.c_str());
    strcpy(y.NewBankPassWord, x.NewBankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.BankAccType = x.BankAccType;
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    y.TID = x.TID;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqChangeAccountField Converter::CThostFtdcReqChangeAccountFieldToRust(CThostFtdcReqChangeAccountField* x) {
    if (x == nullptr)
        return ReqChangeAccountField{.is_null = true};
    ReqChangeAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.NewBankAccount = Converter::Gb2312ToRustString(x->NewBankAccount);
    y.NewBankPassWord = Converter::Gb2312ToRustString(x->NewBankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.BankAccType = x->BankAccType;
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.TID = x->TID;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcReqTransferField Converter::ReqTransferFieldToCpp(ReqTransferField x) {
    CThostFtdcReqTransferField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.FutureSerial = x.FutureSerial;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.FutureFetchAmount = x.FutureFetchAmount;
    y.FeePayFlag = x.FeePayFlag;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.TransferStatus = x.TransferStatus;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqTransferField Converter::CThostFtdcReqTransferFieldToRust(CThostFtdcReqTransferField* x) {
    if (x == nullptr)
        return ReqTransferField{.is_null = true};
    ReqTransferField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.FutureSerial = x->FutureSerial;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.FutureFetchAmount = x->FutureFetchAmount;
    y.FeePayFlag = x->FeePayFlag;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.TransferStatus = x->TransferStatus;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcRspTransferField Converter::RspTransferFieldToCpp(RspTransferField x) {
    CThostFtdcRspTransferField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.FutureSerial = x.FutureSerial;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.FutureFetchAmount = x.FutureFetchAmount;
    y.FeePayFlag = x.FeePayFlag;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.TransferStatus = x.TransferStatus;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

RspTransferField Converter::CThostFtdcRspTransferFieldToRust(CThostFtdcRspTransferField* x) {
    if (x == nullptr)
        return RspTransferField{.is_null = true};
    RspTransferField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.FutureSerial = x->FutureSerial;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.FutureFetchAmount = x->FutureFetchAmount;
    y.FeePayFlag = x->FeePayFlag;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.TransferStatus = x->TransferStatus;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcReqRepealField Converter::ReqRepealFieldToCpp(ReqRepealField x) {
    CThostFtdcReqRepealField y;
    memset(&y, 0, sizeof(y));
    y.RepealTimeInterval = x.RepealTimeInterval;
    y.RepealedTimes = x.RepealedTimes;
    y.BankRepealFlag = x.BankRepealFlag;
    y.BrokerRepealFlag = x.BrokerRepealFlag;
    y.PlateRepealSerial = x.PlateRepealSerial;
    strcpy(y.BankRepealSerial, x.BankRepealSerial.c_str());
    y.FutureRepealSerial = x.FutureRepealSerial;
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.FutureSerial = x.FutureSerial;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.FutureFetchAmount = x.FutureFetchAmount;
    y.FeePayFlag = x.FeePayFlag;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.TransferStatus = x.TransferStatus;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqRepealField Converter::CThostFtdcReqRepealFieldToRust(CThostFtdcReqRepealField* x) {
    if (x == nullptr)
        return ReqRepealField{.is_null = true};
    ReqRepealField y{};
    y.RepealTimeInterval = x->RepealTimeInterval;
    y.RepealedTimes = x->RepealedTimes;
    y.BankRepealFlag = x->BankRepealFlag;
    y.BrokerRepealFlag = x->BrokerRepealFlag;
    y.PlateRepealSerial = x->PlateRepealSerial;
    y.BankRepealSerial = Converter::Gb2312ToRustString(x->BankRepealSerial);
    y.FutureRepealSerial = x->FutureRepealSerial;
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.FutureSerial = x->FutureSerial;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.FutureFetchAmount = x->FutureFetchAmount;
    y.FeePayFlag = x->FeePayFlag;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.TransferStatus = x->TransferStatus;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcRspRepealField Converter::RspRepealFieldToCpp(RspRepealField x) {
    CThostFtdcRspRepealField y;
    memset(&y, 0, sizeof(y));
    y.RepealTimeInterval = x.RepealTimeInterval;
    y.RepealedTimes = x.RepealedTimes;
    y.BankRepealFlag = x.BankRepealFlag;
    y.BrokerRepealFlag = x.BrokerRepealFlag;
    y.PlateRepealSerial = x.PlateRepealSerial;
    strcpy(y.BankRepealSerial, x.BankRepealSerial.c_str());
    y.FutureRepealSerial = x.FutureRepealSerial;
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.FutureSerial = x.FutureSerial;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.FutureFetchAmount = x.FutureFetchAmount;
    y.FeePayFlag = x.FeePayFlag;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.TransferStatus = x.TransferStatus;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

RspRepealField Converter::CThostFtdcRspRepealFieldToRust(CThostFtdcRspRepealField* x) {
    if (x == nullptr)
        return RspRepealField{.is_null = true};
    RspRepealField y{};
    y.RepealTimeInterval = x->RepealTimeInterval;
    y.RepealedTimes = x->RepealedTimes;
    y.BankRepealFlag = x->BankRepealFlag;
    y.BrokerRepealFlag = x->BrokerRepealFlag;
    y.PlateRepealSerial = x->PlateRepealSerial;
    y.BankRepealSerial = Converter::Gb2312ToRustString(x->BankRepealSerial);
    y.FutureRepealSerial = x->FutureRepealSerial;
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.FutureSerial = x->FutureSerial;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.FutureFetchAmount = x->FutureFetchAmount;
    y.FeePayFlag = x->FeePayFlag;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.TransferStatus = x->TransferStatus;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcReqQueryAccountField Converter::ReqQueryAccountFieldToCpp(ReqQueryAccountField x) {
    CThostFtdcReqQueryAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.FutureSerial = x.FutureSerial;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqQueryAccountField Converter::CThostFtdcReqQueryAccountFieldToRust(CThostFtdcReqQueryAccountField* x) {
    if (x == nullptr)
        return ReqQueryAccountField{.is_null = true};
    ReqQueryAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.FutureSerial = x->FutureSerial;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcRspQueryAccountField Converter::RspQueryAccountFieldToCpp(RspQueryAccountField x) {
    CThostFtdcRspQueryAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.FutureSerial = x.FutureSerial;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.BankUseAmount = x.BankUseAmount;
    y.BankFetchAmount = x.BankFetchAmount;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

RspQueryAccountField Converter::CThostFtdcRspQueryAccountFieldToRust(CThostFtdcRspQueryAccountField* x) {
    if (x == nullptr)
        return RspQueryAccountField{.is_null = true};
    RspQueryAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.FutureSerial = x->FutureSerial;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.BankUseAmount = x->BankUseAmount;
    y.BankFetchAmount = x->BankFetchAmount;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcFutureSignIOField Converter::FutureSignIOFieldToCpp(FutureSignIOField x) {
    CThostFtdcFutureSignIOField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    return y;
}

FutureSignIOField Converter::CThostFtdcFutureSignIOFieldToRust(CThostFtdcFutureSignIOField* x) {
    if (x == nullptr)
        return FutureSignIOField{.is_null = true};
    FutureSignIOField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    return y;
}

CThostFtdcRspFutureSignInField Converter::RspFutureSignInFieldToCpp(RspFutureSignInField x) {
    CThostFtdcRspFutureSignInField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    memcpy(y.PinKey, x.PinKey.data(), x.PinKey.size() * sizeof(uint8_t));
    memcpy(y.MacKey, x.MacKey.data(), x.MacKey.size() * sizeof(uint8_t));
    return y;
}

RspFutureSignInField Converter::CThostFtdcRspFutureSignInFieldToRust(CThostFtdcRspFutureSignInField* x) {
    if (x == nullptr)
        return RspFutureSignInField{.is_null = true};
    RspFutureSignInField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    for (int i = 0; i < 129; i++)
        y.PinKey.push_back(x->PinKey[i]);
    for (int i = 0; i < 129; i++)
        y.MacKey.push_back(x->MacKey[i]);
    return y;
}

CThostFtdcReqFutureSignOutField Converter::ReqFutureSignOutFieldToCpp(ReqFutureSignOutField x) {
    CThostFtdcReqFutureSignOutField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    return y;
}

ReqFutureSignOutField Converter::CThostFtdcReqFutureSignOutFieldToRust(CThostFtdcReqFutureSignOutField* x) {
    if (x == nullptr)
        return ReqFutureSignOutField{.is_null = true};
    ReqFutureSignOutField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    return y;
}

CThostFtdcRspFutureSignOutField Converter::RspFutureSignOutFieldToCpp(RspFutureSignOutField x) {
    CThostFtdcRspFutureSignOutField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

RspFutureSignOutField Converter::CThostFtdcRspFutureSignOutFieldToRust(CThostFtdcRspFutureSignOutField* x) {
    if (x == nullptr)
        return RspFutureSignOutField{.is_null = true};
    RspFutureSignOutField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcReqQueryTradeResultBySerialField Converter::ReqQueryTradeResultBySerialFieldToCpp(ReqQueryTradeResultBySerialField x) {
    CThostFtdcReqQueryTradeResultBySerialField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.Reference = x.Reference;
    y.RefrenceIssureType = x.RefrenceIssureType;
    memcpy(y.RefrenceIssure, x.RefrenceIssure.data(), x.RefrenceIssure.size() * sizeof(uint8_t));
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ReqQueryTradeResultBySerialField Converter::CThostFtdcReqQueryTradeResultBySerialFieldToRust(CThostFtdcReqQueryTradeResultBySerialField* x) {
    if (x == nullptr)
        return ReqQueryTradeResultBySerialField{.is_null = true};
    ReqQueryTradeResultBySerialField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.Reference = x->Reference;
    y.RefrenceIssureType = x->RefrenceIssureType;
    for (int i = 0; i < 36; i++)
        y.RefrenceIssure.push_back(x->RefrenceIssure[i]);
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcRspQueryTradeResultBySerialField Converter::RspQueryTradeResultBySerialFieldToCpp(RspQueryTradeResultBySerialField x) {
    CThostFtdcRspQueryTradeResultBySerialField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    y.Reference = x.Reference;
    y.RefrenceIssureType = x.RefrenceIssureType;
    memcpy(y.RefrenceIssure, x.RefrenceIssure.data(), x.RefrenceIssure.size() * sizeof(uint8_t));
    strcpy(y.OriginReturnCode, x.OriginReturnCode.c_str());
    strcpy(y.OriginDescrInfoForReturnCode, x.OriginDescrInfoForReturnCode.c_str());
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    return y;
}

RspQueryTradeResultBySerialField Converter::CThostFtdcRspQueryTradeResultBySerialFieldToRust(CThostFtdcRspQueryTradeResultBySerialField* x) {
    if (x == nullptr)
        return RspQueryTradeResultBySerialField{.is_null = true};
    RspQueryTradeResultBySerialField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.Reference = x->Reference;
    y.RefrenceIssureType = x->RefrenceIssureType;
    for (int i = 0; i < 36; i++)
        y.RefrenceIssure.push_back(x->RefrenceIssure[i]);
    y.OriginReturnCode = Converter::Gb2312ToRustString(x->OriginReturnCode);
    y.OriginDescrInfoForReturnCode = Converter::Gb2312ToRustString(x->OriginDescrInfoForReturnCode);
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    return y;
}

CThostFtdcReqDayEndFileReadyField Converter::ReqDayEndFileReadyFieldToCpp(ReqDayEndFileReadyField x) {
    CThostFtdcReqDayEndFileReadyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.FileBusinessCode = x.FileBusinessCode;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    return y;
}

ReqDayEndFileReadyField Converter::CThostFtdcReqDayEndFileReadyFieldToRust(CThostFtdcReqDayEndFileReadyField* x) {
    if (x == nullptr)
        return ReqDayEndFileReadyField{.is_null = true};
    ReqDayEndFileReadyField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.FileBusinessCode = x->FileBusinessCode;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    return y;
}

CThostFtdcReturnResultField Converter::ReturnResultFieldToCpp(ReturnResultField x) {
    CThostFtdcReturnResultField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ReturnCode, x.ReturnCode.c_str());
    strcpy(y.DescrInfoForReturnCode, x.DescrInfoForReturnCode.c_str());
    return y;
}

ReturnResultField Converter::CThostFtdcReturnResultFieldToRust(CThostFtdcReturnResultField* x) {
    if (x == nullptr)
        return ReturnResultField{.is_null = true};
    ReturnResultField y{};
    y.ReturnCode = Converter::Gb2312ToRustString(x->ReturnCode);
    y.DescrInfoForReturnCode = Converter::Gb2312ToRustString(x->DescrInfoForReturnCode);
    return y;
}

CThostFtdcVerifyFuturePasswordField Converter::VerifyFuturePasswordFieldToCpp(VerifyFuturePasswordField x) {
    CThostFtdcVerifyFuturePasswordField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    y.InstallID = x.InstallID;
    y.TID = x.TID;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

VerifyFuturePasswordField Converter::CThostFtdcVerifyFuturePasswordFieldToRust(CThostFtdcVerifyFuturePasswordField* x) {
    if (x == nullptr)
        return VerifyFuturePasswordField{.is_null = true};
    VerifyFuturePasswordField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.InstallID = x->InstallID;
    y.TID = x->TID;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcVerifyCustInfoField Converter::VerifyCustInfoFieldToCpp(VerifyCustInfoField x) {
    CThostFtdcVerifyCustInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

VerifyCustInfoField Converter::CThostFtdcVerifyCustInfoFieldToRust(CThostFtdcVerifyCustInfoField* x) {
    if (x == nullptr)
        return VerifyCustInfoField{.is_null = true};
    VerifyCustInfoField y{};
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcVerifyFuturePasswordAndCustInfoField Converter::VerifyFuturePasswordAndCustInfoFieldToCpp(VerifyFuturePasswordAndCustInfoField x) {
    CThostFtdcVerifyFuturePasswordAndCustInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

VerifyFuturePasswordAndCustInfoField Converter::CThostFtdcVerifyFuturePasswordAndCustInfoFieldToRust(CThostFtdcVerifyFuturePasswordAndCustInfoField* x) {
    if (x == nullptr)
        return VerifyFuturePasswordAndCustInfoField{.is_null = true};
    VerifyFuturePasswordAndCustInfoField y{};
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcDepositResultInformField Converter::DepositResultInformFieldToCpp(DepositResultInformField x) {
    CThostFtdcDepositResultInformField y;
    memset(&y, 0, sizeof(y));
    memcpy(y.DepositSeqNo, x.DepositSeqNo.data(), x.DepositSeqNo.size() * sizeof(uint8_t));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Deposit = x.Deposit;
    y.RequestID = x.RequestID;
    strcpy(y.ReturnCode, x.ReturnCode.c_str());
    strcpy(y.DescrInfoForReturnCode, x.DescrInfoForReturnCode.c_str());
    return y;
}

DepositResultInformField Converter::CThostFtdcDepositResultInformFieldToRust(CThostFtdcDepositResultInformField* x) {
    if (x == nullptr)
        return DepositResultInformField{.is_null = true};
    DepositResultInformField y{};
    for (int i = 0; i < 15; i++)
        y.DepositSeqNo.push_back(x->DepositSeqNo[i]);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Deposit = x->Deposit;
    y.RequestID = x->RequestID;
    y.ReturnCode = Converter::Gb2312ToRustString(x->ReturnCode);
    y.DescrInfoForReturnCode = Converter::Gb2312ToRustString(x->DescrInfoForReturnCode);
    return y;
}

CThostFtdcReqSyncKeyField Converter::ReqSyncKeyFieldToCpp(ReqSyncKeyField x) {
    CThostFtdcReqSyncKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    return y;
}

ReqSyncKeyField Converter::CThostFtdcReqSyncKeyFieldToRust(CThostFtdcReqSyncKeyField* x) {
    if (x == nullptr)
        return ReqSyncKeyField{.is_null = true};
    ReqSyncKeyField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    return y;
}

CThostFtdcRspSyncKeyField Converter::RspSyncKeyFieldToCpp(RspSyncKeyField x) {
    CThostFtdcRspSyncKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

RspSyncKeyField Converter::CThostFtdcRspSyncKeyFieldToRust(CThostFtdcRspSyncKeyField* x) {
    if (x == nullptr)
        return RspSyncKeyField{.is_null = true};
    RspSyncKeyField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcNotifyQueryAccountField Converter::NotifyQueryAccountFieldToCpp(NotifyQueryAccountField x) {
    CThostFtdcNotifyQueryAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.FutureSerial = x.FutureSerial;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.BankUseAmount = x.BankUseAmount;
    y.BankFetchAmount = x.BankFetchAmount;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

NotifyQueryAccountField Converter::CThostFtdcNotifyQueryAccountFieldToRust(CThostFtdcNotifyQueryAccountField* x) {
    if (x == nullptr)
        return NotifyQueryAccountField{.is_null = true};
    NotifyQueryAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.FutureSerial = x->FutureSerial;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.BankUseAmount = x->BankUseAmount;
    y.BankFetchAmount = x->BankFetchAmount;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcTransferSerialField Converter::TransferSerialFieldToCpp(TransferSerialField x) {
    CThostFtdcTransferSerialField y;
    memset(&y, 0, sizeof(y));
    y.PlateSerial = x.PlateSerial;
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.TradeCode, x.TradeCode.c_str());
    y.SessionID = x.SessionID;
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    y.BankAccType = x.BankAccType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    y.FutureAccType = x.FutureAccType;
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.FutureSerial = x.FutureSerial;
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    y.AvailabilityFlag = x.AvailabilityFlag;
    strcpy(y.OperatorCode, x.OperatorCode.c_str());
    strcpy(y.BankNewAccount, x.BankNewAccount.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

TransferSerialField Converter::CThostFtdcTransferSerialFieldToRust(CThostFtdcTransferSerialField* x) {
    if (x == nullptr)
        return TransferSerialField{.is_null = true};
    TransferSerialField y{};
    y.PlateSerial = x->PlateSerial;
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.SessionID = x->SessionID;
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BankAccType = x->BankAccType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.FutureAccType = x->FutureAccType;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.FutureSerial = x->FutureSerial;
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    y.AvailabilityFlag = x->AvailabilityFlag;
    y.OperatorCode = Converter::Gb2312ToRustString(x->OperatorCode);
    y.BankNewAccount = Converter::Gb2312ToRustString(x->BankNewAccount);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcQryTransferSerialField Converter::QryTransferSerialFieldToCpp(QryTransferSerialField x) {
    CThostFtdcQryTransferSerialField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

QryTransferSerialField Converter::CThostFtdcQryTransferSerialFieldToRust(CThostFtdcQryTransferSerialField* x) {
    if (x == nullptr)
        return QryTransferSerialField{.is_null = true};
    QryTransferSerialField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcNotifyFutureSignInField Converter::NotifyFutureSignInFieldToCpp(NotifyFutureSignInField x) {
    CThostFtdcNotifyFutureSignInField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    memcpy(y.PinKey, x.PinKey.data(), x.PinKey.size() * sizeof(uint8_t));
    memcpy(y.MacKey, x.MacKey.data(), x.MacKey.size() * sizeof(uint8_t));
    return y;
}

NotifyFutureSignInField Converter::CThostFtdcNotifyFutureSignInFieldToRust(CThostFtdcNotifyFutureSignInField* x) {
    if (x == nullptr)
        return NotifyFutureSignInField{.is_null = true};
    NotifyFutureSignInField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    for (int i = 0; i < 129; i++)
        y.PinKey.push_back(x->PinKey[i]);
    for (int i = 0; i < 129; i++)
        y.MacKey.push_back(x->MacKey[i]);
    return y;
}

CThostFtdcNotifyFutureSignOutField Converter::NotifyFutureSignOutFieldToCpp(NotifyFutureSignOutField x) {
    CThostFtdcNotifyFutureSignOutField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

NotifyFutureSignOutField Converter::CThostFtdcNotifyFutureSignOutFieldToRust(CThostFtdcNotifyFutureSignOutField* x) {
    if (x == nullptr)
        return NotifyFutureSignOutField{.is_null = true};
    NotifyFutureSignOutField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcNotifySyncKeyField Converter::NotifySyncKeyFieldToCpp(NotifySyncKeyField x) {
    CThostFtdcNotifySyncKeyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    strcpy(y.DeviceID, x.DeviceID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

NotifySyncKeyField Converter::CThostFtdcNotifySyncKeyFieldToRust(CThostFtdcNotifySyncKeyField* x) {
    if (x == nullptr)
        return NotifySyncKeyField{.is_null = true};
    NotifySyncKeyField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcQryAccountregisterField Converter::QryAccountregisterFieldToCpp(QryAccountregisterField x) {
    CThostFtdcQryAccountregisterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

QryAccountregisterField Converter::CThostFtdcQryAccountregisterFieldToRust(CThostFtdcQryAccountregisterField* x) {
    if (x == nullptr)
        return QryAccountregisterField{.is_null = true};
    QryAccountregisterField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcAccountregisterField Converter::AccountregisterFieldToCpp(AccountregisterField x) {
    CThostFtdcAccountregisterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeDay, x.TradeDay.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    strcpy(y.CustomerName, x.CustomerName.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.OpenOrDestroy = x.OpenOrDestroy;
    strcpy(y.RegDate, x.RegDate.c_str());
    strcpy(y.OutDate, x.OutDate.c_str());
    y.TID = x.TID;
    y.CustType = x.CustType;
    y.BankAccType = x.BankAccType;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

AccountregisterField Converter::CThostFtdcAccountregisterFieldToRust(CThostFtdcAccountregisterField* x) {
    if (x == nullptr)
        return AccountregisterField{.is_null = true};
    AccountregisterField y{};
    y.TradeDay = Converter::Gb2312ToRustString(x->TradeDay);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.OpenOrDestroy = x->OpenOrDestroy;
    y.RegDate = Converter::Gb2312ToRustString(x->RegDate);
    y.OutDate = Converter::Gb2312ToRustString(x->OutDate);
    y.TID = x->TID;
    y.CustType = x->CustType;
    y.BankAccType = x->BankAccType;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcOpenAccountField Converter::OpenAccountFieldToCpp(OpenAccountField x) {
    CThostFtdcOpenAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.CashExchangeCode = x.CashExchangeCode;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.TID = x.TID;
    strcpy(y.UserID, x.UserID.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

OpenAccountField Converter::CThostFtdcOpenAccountFieldToRust(CThostFtdcOpenAccountField* x) {
    if (x == nullptr)
        return OpenAccountField{.is_null = true};
    OpenAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.CashExchangeCode = x->CashExchangeCode;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.TID = x->TID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcCancelAccountField Converter::CancelAccountFieldToCpp(CancelAccountField x) {
    CThostFtdcCancelAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.CashExchangeCode = x.CashExchangeCode;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.TID = x.TID;
    strcpy(y.UserID, x.UserID.c_str());
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

CancelAccountField Converter::CThostFtdcCancelAccountFieldToRust(CThostFtdcCancelAccountField* x) {
    if (x == nullptr)
        return CancelAccountField{.is_null = true};
    CancelAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.CashExchangeCode = x->CashExchangeCode;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.TID = x->TID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcChangeAccountField Converter::ChangeAccountFieldToCpp(ChangeAccountField x) {
    CThostFtdcChangeAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.NewBankAccount, x.NewBankAccount.c_str());
    strcpy(y.NewBankPassWord, x.NewBankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.BankAccType = x.BankAccType;
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    y.TID = x.TID;
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    return y;
}

ChangeAccountField Converter::CThostFtdcChangeAccountFieldToRust(CThostFtdcChangeAccountField* x) {
    if (x == nullptr)
        return ChangeAccountField{.is_null = true};
    ChangeAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.NewBankAccount = Converter::Gb2312ToRustString(x->NewBankAccount);
    y.NewBankPassWord = Converter::Gb2312ToRustString(x->NewBankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.BankAccType = x->BankAccType;
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.TID = x->TID;
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    return y;
}

CThostFtdcSecAgentACIDMapField Converter::SecAgentACIDMapFieldToCpp(SecAgentACIDMapField x) {
    CThostFtdcSecAgentACIDMapField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    strcpy(y.BrokerSecAgentID, x.BrokerSecAgentID.c_str());
    return y;
}

SecAgentACIDMapField Converter::CThostFtdcSecAgentACIDMapFieldToRust(CThostFtdcSecAgentACIDMapField* x) {
    if (x == nullptr)
        return SecAgentACIDMapField{.is_null = true};
    SecAgentACIDMapField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.BrokerSecAgentID = Converter::Gb2312ToRustString(x->BrokerSecAgentID);
    return y;
}

CThostFtdcQrySecAgentACIDMapField Converter::QrySecAgentACIDMapFieldToCpp(QrySecAgentACIDMapField x) {
    CThostFtdcQrySecAgentACIDMapField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

QrySecAgentACIDMapField Converter::CThostFtdcQrySecAgentACIDMapFieldToRust(CThostFtdcQrySecAgentACIDMapField* x) {
    if (x == nullptr)
        return QrySecAgentACIDMapField{.is_null = true};
    QrySecAgentACIDMapField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcUserRightsAssignField Converter::UserRightsAssignFieldToCpp(UserRightsAssignField x) {
    CThostFtdcUserRightsAssignField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.DRIdentityID = x.DRIdentityID;
    return y;
}

UserRightsAssignField Converter::CThostFtdcUserRightsAssignFieldToRust(CThostFtdcUserRightsAssignField* x) {
    if (x == nullptr)
        return UserRightsAssignField{.is_null = true};
    UserRightsAssignField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.DRIdentityID = x->DRIdentityID;
    return y;
}

CThostFtdcBrokerUserRightAssignField Converter::BrokerUserRightAssignFieldToCpp(BrokerUserRightAssignField x) {
    CThostFtdcBrokerUserRightAssignField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    y.DRIdentityID = x.DRIdentityID;
    y.Tradeable = x.Tradeable;
    return y;
}

BrokerUserRightAssignField Converter::CThostFtdcBrokerUserRightAssignFieldToRust(CThostFtdcBrokerUserRightAssignField* x) {
    if (x == nullptr)
        return BrokerUserRightAssignField{.is_null = true};
    BrokerUserRightAssignField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.DRIdentityID = x->DRIdentityID;
    y.Tradeable = x->Tradeable;
    return y;
}

CThostFtdcDRTransferField Converter::DRTransferFieldToCpp(DRTransferField x) {
    CThostFtdcDRTransferField y;
    memset(&y, 0, sizeof(y));
    y.OrigDRIdentityID = x.OrigDRIdentityID;
    y.DestDRIdentityID = x.DestDRIdentityID;
    strcpy(y.OrigBrokerID, x.OrigBrokerID.c_str());
    strcpy(y.DestBrokerID, x.DestBrokerID.c_str());
    return y;
}

DRTransferField Converter::CThostFtdcDRTransferFieldToRust(CThostFtdcDRTransferField* x) {
    if (x == nullptr)
        return DRTransferField{.is_null = true};
    DRTransferField y{};
    y.OrigDRIdentityID = x->OrigDRIdentityID;
    y.DestDRIdentityID = x->DestDRIdentityID;
    y.OrigBrokerID = Converter::Gb2312ToRustString(x->OrigBrokerID);
    y.DestBrokerID = Converter::Gb2312ToRustString(x->DestBrokerID);
    return y;
}

CThostFtdcFensUserInfoField Converter::FensUserInfoFieldToCpp(FensUserInfoField x) {
    CThostFtdcFensUserInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.LoginMode = x.LoginMode;
    return y;
}

FensUserInfoField Converter::CThostFtdcFensUserInfoFieldToRust(CThostFtdcFensUserInfoField* x) {
    if (x == nullptr)
        return FensUserInfoField{.is_null = true};
    FensUserInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.LoginMode = x->LoginMode;
    return y;
}

CThostFtdcCurrTransferIdentityField Converter::CurrTransferIdentityFieldToCpp(CurrTransferIdentityField x) {
    CThostFtdcCurrTransferIdentityField y;
    memset(&y, 0, sizeof(y));
    y.IdentityID = x.IdentityID;
    return y;
}

CurrTransferIdentityField Converter::CThostFtdcCurrTransferIdentityFieldToRust(CThostFtdcCurrTransferIdentityField* x) {
    if (x == nullptr)
        return CurrTransferIdentityField{.is_null = true};
    CurrTransferIdentityField y{};
    y.IdentityID = x->IdentityID;
    return y;
}

CThostFtdcLoginForbiddenUserField Converter::LoginForbiddenUserFieldToCpp(LoginForbiddenUserField x) {
    CThostFtdcLoginForbiddenUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

LoginForbiddenUserField Converter::CThostFtdcLoginForbiddenUserFieldToRust(CThostFtdcLoginForbiddenUserField* x) {
    if (x == nullptr)
        return LoginForbiddenUserField{.is_null = true};
    LoginForbiddenUserField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryLoginForbiddenUserField Converter::QryLoginForbiddenUserFieldToCpp(QryLoginForbiddenUserField x) {
    CThostFtdcQryLoginForbiddenUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QryLoginForbiddenUserField Converter::CThostFtdcQryLoginForbiddenUserFieldToRust(CThostFtdcQryLoginForbiddenUserField* x) {
    if (x == nullptr)
        return QryLoginForbiddenUserField{.is_null = true};
    QryLoginForbiddenUserField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcTradingAccountReserveField Converter::TradingAccountReserveFieldToCpp(TradingAccountReserveField x) {
    CThostFtdcTradingAccountReserveField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.Reserve = x.Reserve;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

TradingAccountReserveField Converter::CThostFtdcTradingAccountReserveFieldToRust(CThostFtdcTradingAccountReserveField* x) {
    if (x == nullptr)
        return TradingAccountReserveField{.is_null = true};
    TradingAccountReserveField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Reserve = x->Reserve;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcQryLoginForbiddenIPField Converter::QryLoginForbiddenIPFieldToCpp(QryLoginForbiddenIPField x) {
    CThostFtdcQryLoginForbiddenIPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

QryLoginForbiddenIPField Converter::CThostFtdcQryLoginForbiddenIPFieldToRust(CThostFtdcQryLoginForbiddenIPField* x) {
    if (x == nullptr)
        return QryLoginForbiddenIPField{.is_null = true};
    QryLoginForbiddenIPField y{};
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryIPListField Converter::QryIPListFieldToCpp(QryIPListField x) {
    CThostFtdcQryIPListField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

QryIPListField Converter::CThostFtdcQryIPListFieldToRust(CThostFtdcQryIPListField* x) {
    if (x == nullptr)
        return QryIPListField{.is_null = true};
    QryIPListField y{};
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryUserRightsAssignField Converter::QryUserRightsAssignFieldToCpp(QryUserRightsAssignField x) {
    CThostFtdcQryUserRightsAssignField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QryUserRightsAssignField Converter::CThostFtdcQryUserRightsAssignFieldToRust(CThostFtdcQryUserRightsAssignField* x) {
    if (x == nullptr)
        return QryUserRightsAssignField{.is_null = true};
    QryUserRightsAssignField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcReserveOpenAccountConfirmField Converter::ReserveOpenAccountConfirmFieldToCpp(ReserveOpenAccountConfirmField x) {
    CThostFtdcReserveOpenAccountConfirmField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    y.TID = x.TID;
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    memcpy(y.BankReserveOpenSeq, x.BankReserveOpenSeq.data(), x.BankReserveOpenSeq.size() * sizeof(uint8_t));
    strcpy(y.BookDate, x.BookDate.c_str());
    memcpy(y.BookPsw, x.BookPsw.data(), x.BookPsw.size() * sizeof(uint8_t));
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

ReserveOpenAccountConfirmField Converter::CThostFtdcReserveOpenAccountConfirmFieldToRust(CThostFtdcReserveOpenAccountConfirmField* x) {
    if (x == nullptr)
        return ReserveOpenAccountConfirmField{.is_null = true};
    ReserveOpenAccountConfirmField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.TID = x->TID;
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    for (int i = 0; i < 13; i++)
        y.BankReserveOpenSeq.push_back(x->BankReserveOpenSeq[i]);
    y.BookDate = Converter::Gb2312ToRustString(x->BookDate);
    for (int i = 0; i < 41; i++)
        y.BookPsw.push_back(x->BookPsw[i]);
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcReserveOpenAccountField Converter::ReserveOpenAccountFieldToCpp(ReserveOpenAccountField x) {
    CThostFtdcReserveOpenAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.Gender = x.Gender;
    strcpy(y.CountryCode, x.CountryCode.c_str());
    y.CustType = x.CustType;
    strcpy(y.Address, x.Address.c_str());
    strcpy(y.ZipCode, x.ZipCode.c_str());
    strcpy(y.Telephone, x.Telephone.c_str());
    strcpy(y.MobilePhone, x.MobilePhone.c_str());
    strcpy(y.Fax, x.Fax.c_str());
    strcpy(y.EMail, x.EMail.c_str());
    y.MoneyAccountStatus = x.MoneyAccountStatus;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    y.InstallID = x.InstallID;
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    y.TID = x.TID;
    y.ReserveOpenAccStas = x.ReserveOpenAccStas;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    return y;
}

ReserveOpenAccountField Converter::CThostFtdcReserveOpenAccountFieldToRust(CThostFtdcReserveOpenAccountField* x) {
    if (x == nullptr)
        return ReserveOpenAccountField{.is_null = true};
    ReserveOpenAccountField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.Gender = x->Gender;
    y.CountryCode = Converter::Gb2312ToRustString(x->CountryCode);
    y.CustType = x->CustType;
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.ZipCode = Converter::Gb2312ToRustString(x->ZipCode);
    y.Telephone = Converter::Gb2312ToRustString(x->Telephone);
    y.MobilePhone = Converter::Gb2312ToRustString(x->MobilePhone);
    y.Fax = Converter::Gb2312ToRustString(x->Fax);
    y.EMail = Converter::Gb2312ToRustString(x->EMail);
    y.MoneyAccountStatus = x->MoneyAccountStatus;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.InstallID = x->InstallID;
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    y.TID = x->TID;
    y.ReserveOpenAccStas = x->ReserveOpenAccStas;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    return y;
}

CThostFtdcAccountPropertyField Converter::AccountPropertyFieldToCpp(AccountPropertyField x) {
    CThostFtdcAccountPropertyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.OpenName, x.OpenName.c_str());
    memcpy(y.OpenBank, x.OpenBank.data(), x.OpenBank.size() * sizeof(uint8_t));
    y.IsActive = x.IsActive;
    y.AccountSourceType = x.AccountSourceType;
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.CancelDate, x.CancelDate.c_str());
    strcpy(y.OperatorID, x.OperatorID.c_str());
    strcpy(y.OperateDate, x.OperateDate.c_str());
    strcpy(y.OperateTime, x.OperateTime.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    return y;
}

AccountPropertyField Converter::CThostFtdcAccountPropertyFieldToRust(CThostFtdcAccountPropertyField* x) {
    if (x == nullptr)
        return AccountPropertyField{.is_null = true};
    AccountPropertyField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.OpenName = Converter::Gb2312ToRustString(x->OpenName);
    for (int i = 0; i < 101; i++)
        y.OpenBank.push_back(x->OpenBank[i]);
    y.IsActive = x->IsActive;
    y.AccountSourceType = x->AccountSourceType;
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.CancelDate = Converter::Gb2312ToRustString(x->CancelDate);
    y.OperatorID = Converter::Gb2312ToRustString(x->OperatorID);
    y.OperateDate = Converter::Gb2312ToRustString(x->OperateDate);
    y.OperateTime = Converter::Gb2312ToRustString(x->OperateTime);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    return y;
}

CThostFtdcQryCurrDRIdentityField Converter::QryCurrDRIdentityFieldToCpp(QryCurrDRIdentityField x) {
    CThostFtdcQryCurrDRIdentityField y;
    memset(&y, 0, sizeof(y));
    y.DRIdentityID = x.DRIdentityID;
    return y;
}

QryCurrDRIdentityField Converter::CThostFtdcQryCurrDRIdentityFieldToRust(CThostFtdcQryCurrDRIdentityField* x) {
    if (x == nullptr)
        return QryCurrDRIdentityField{.is_null = true};
    QryCurrDRIdentityField y{};
    y.DRIdentityID = x->DRIdentityID;
    return y;
}

CThostFtdcCurrDRIdentityField Converter::CurrDRIdentityFieldToCpp(CurrDRIdentityField x) {
    CThostFtdcCurrDRIdentityField y;
    memset(&y, 0, sizeof(y));
    y.DRIdentityID = x.DRIdentityID;
    return y;
}

CurrDRIdentityField Converter::CThostFtdcCurrDRIdentityFieldToRust(CThostFtdcCurrDRIdentityField* x) {
    if (x == nullptr)
        return CurrDRIdentityField{.is_null = true};
    CurrDRIdentityField y{};
    y.DRIdentityID = x->DRIdentityID;
    return y;
}

CThostFtdcQrySecAgentCheckModeField Converter::QrySecAgentCheckModeFieldToCpp(QrySecAgentCheckModeField x) {
    CThostFtdcQrySecAgentCheckModeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QrySecAgentCheckModeField Converter::CThostFtdcQrySecAgentCheckModeFieldToRust(CThostFtdcQrySecAgentCheckModeField* x) {
    if (x == nullptr)
        return QrySecAgentCheckModeField{.is_null = true};
    QrySecAgentCheckModeField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcQrySecAgentTradeInfoField Converter::QrySecAgentTradeInfoFieldToCpp(QrySecAgentTradeInfoField x) {
    CThostFtdcQrySecAgentTradeInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerSecAgentID, x.BrokerSecAgentID.c_str());
    return y;
}

QrySecAgentTradeInfoField Converter::CThostFtdcQrySecAgentTradeInfoFieldToRust(CThostFtdcQrySecAgentTradeInfoField* x) {
    if (x == nullptr)
        return QrySecAgentTradeInfoField{.is_null = true};
    QrySecAgentTradeInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerSecAgentID = Converter::Gb2312ToRustString(x->BrokerSecAgentID);
    return y;
}

CThostFtdcReqUserAuthMethodField Converter::ReqUserAuthMethodFieldToCpp(ReqUserAuthMethodField x) {
    CThostFtdcReqUserAuthMethodField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

ReqUserAuthMethodField Converter::CThostFtdcReqUserAuthMethodFieldToRust(CThostFtdcReqUserAuthMethodField* x) {
    if (x == nullptr)
        return ReqUserAuthMethodField{.is_null = true};
    ReqUserAuthMethodField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcRspUserAuthMethodField Converter::RspUserAuthMethodFieldToCpp(RspUserAuthMethodField x) {
    CThostFtdcRspUserAuthMethodField y;
    memset(&y, 0, sizeof(y));
    y.UsableAuthMethod = x.UsableAuthMethod;
    return y;
}

RspUserAuthMethodField Converter::CThostFtdcRspUserAuthMethodFieldToRust(CThostFtdcRspUserAuthMethodField* x) {
    if (x == nullptr)
        return RspUserAuthMethodField{.is_null = true};
    RspUserAuthMethodField y{};
    y.UsableAuthMethod = x->UsableAuthMethod;
    return y;
}

CThostFtdcReqGenUserCaptchaField Converter::ReqGenUserCaptchaFieldToCpp(ReqGenUserCaptchaField x) {
    CThostFtdcReqGenUserCaptchaField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

ReqGenUserCaptchaField Converter::CThostFtdcReqGenUserCaptchaFieldToRust(CThostFtdcReqGenUserCaptchaField* x) {
    if (x == nullptr)
        return ReqGenUserCaptchaField{.is_null = true};
    ReqGenUserCaptchaField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcRspGenUserCaptchaField Converter::RspGenUserCaptchaFieldToCpp(RspGenUserCaptchaField x) {
    CThostFtdcRspGenUserCaptchaField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.CaptchaInfoLen = x.CaptchaInfoLen;
    strcpy(y.CaptchaInfo, x.CaptchaInfo.c_str());
    return y;
}

RspGenUserCaptchaField Converter::CThostFtdcRspGenUserCaptchaFieldToRust(CThostFtdcRspGenUserCaptchaField* x) {
    if (x == nullptr)
        return RspGenUserCaptchaField{.is_null = true};
    RspGenUserCaptchaField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.CaptchaInfoLen = x->CaptchaInfoLen;
    y.CaptchaInfo = Converter::Gb2312ToRustString(x->CaptchaInfo);
    return y;
}

CThostFtdcReqGenUserTextField Converter::ReqGenUserTextFieldToCpp(ReqGenUserTextField x) {
    CThostFtdcReqGenUserTextField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

ReqGenUserTextField Converter::CThostFtdcReqGenUserTextFieldToRust(CThostFtdcReqGenUserTextField* x) {
    if (x == nullptr)
        return ReqGenUserTextField{.is_null = true};
    ReqGenUserTextField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcRspGenUserTextField Converter::RspGenUserTextFieldToCpp(RspGenUserTextField x) {
    CThostFtdcRspGenUserTextField y;
    memset(&y, 0, sizeof(y));
    y.UserTextSeq = x.UserTextSeq;
    return y;
}

RspGenUserTextField Converter::CThostFtdcRspGenUserTextFieldToRust(CThostFtdcRspGenUserTextField* x) {
    if (x == nullptr)
        return RspGenUserTextField{.is_null = true};
    RspGenUserTextField y{};
    y.UserTextSeq = x->UserTextSeq;
    return y;
}

CThostFtdcReqUserLoginWithCaptchaField Converter::ReqUserLoginWithCaptchaFieldToCpp(ReqUserLoginWithCaptchaField x) {
    CThostFtdcReqUserLoginWithCaptchaField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    memcpy(y.Captcha, x.Captcha.data(), x.Captcha.size() * sizeof(uint8_t));
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientIPAddress, x.ClientIPAddress.c_str());
    return y;
}

ReqUserLoginWithCaptchaField Converter::CThostFtdcReqUserLoginWithCaptchaFieldToRust(CThostFtdcReqUserLoginWithCaptchaField* x) {
    if (x == nullptr)
        return ReqUserLoginWithCaptchaField{.is_null = true};
    ReqUserLoginWithCaptchaField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    for (int i = 0; i < 41; i++)
        y.Captcha.push_back(x->Captcha[i]);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientIPAddress = Converter::Gb2312ToRustString(x->ClientIPAddress);
    return y;
}

CThostFtdcReqUserLoginWithTextField Converter::ReqUserLoginWithTextFieldToCpp(ReqUserLoginWithTextField x) {
    CThostFtdcReqUserLoginWithTextField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    memcpy(y.Text, x.Text.data(), x.Text.size() * sizeof(uint8_t));
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientIPAddress, x.ClientIPAddress.c_str());
    return y;
}

ReqUserLoginWithTextField Converter::CThostFtdcReqUserLoginWithTextFieldToRust(CThostFtdcReqUserLoginWithTextField* x) {
    if (x == nullptr)
        return ReqUserLoginWithTextField{.is_null = true};
    ReqUserLoginWithTextField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    for (int i = 0; i < 41; i++)
        y.Text.push_back(x->Text[i]);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientIPAddress = Converter::Gb2312ToRustString(x->ClientIPAddress);
    return y;
}

CThostFtdcReqUserLoginWithOTPField Converter::ReqUserLoginWithOTPFieldToCpp(ReqUserLoginWithOTPField x) {
    CThostFtdcReqUserLoginWithOTPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    strcpy(y.OTPPassword, x.OTPPassword.c_str());
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientIPAddress, x.ClientIPAddress.c_str());
    return y;
}

ReqUserLoginWithOTPField Converter::CThostFtdcReqUserLoginWithOTPFieldToRust(CThostFtdcReqUserLoginWithOTPField* x) {
    if (x == nullptr)
        return ReqUserLoginWithOTPField{.is_null = true};
    ReqUserLoginWithOTPField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    y.OTPPassword = Converter::Gb2312ToRustString(x->OTPPassword);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientIPAddress = Converter::Gb2312ToRustString(x->ClientIPAddress);
    return y;
}

CThostFtdcReqApiHandshakeField Converter::ReqApiHandshakeFieldToCpp(ReqApiHandshakeField x) {
    CThostFtdcReqApiHandshakeField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CryptoKeyVersion, x.CryptoKeyVersion.c_str());
    return y;
}

ReqApiHandshakeField Converter::CThostFtdcReqApiHandshakeFieldToRust(CThostFtdcReqApiHandshakeField* x) {
    if (x == nullptr)
        return ReqApiHandshakeField{.is_null = true};
    ReqApiHandshakeField y{};
    y.CryptoKeyVersion = Converter::Gb2312ToRustString(x->CryptoKeyVersion);
    return y;
}

CThostFtdcRspApiHandshakeField Converter::RspApiHandshakeFieldToCpp(RspApiHandshakeField x) {
    CThostFtdcRspApiHandshakeField y;
    memset(&y, 0, sizeof(y));
    y.FrontHandshakeDataLen = x.FrontHandshakeDataLen;
    memcpy(y.FrontHandshakeData, x.FrontHandshakeData.data(), x.FrontHandshakeData.size() * sizeof(uint8_t));
    y.IsApiAuthEnabled = x.IsApiAuthEnabled;
    return y;
}

RspApiHandshakeField Converter::CThostFtdcRspApiHandshakeFieldToRust(CThostFtdcRspApiHandshakeField* x) {
    if (x == nullptr)
        return RspApiHandshakeField{.is_null = true};
    RspApiHandshakeField y{};
    y.FrontHandshakeDataLen = x->FrontHandshakeDataLen;
    for (int i = 0; i < 301; i++)
        y.FrontHandshakeData.push_back(x->FrontHandshakeData[i]);
    y.IsApiAuthEnabled = x->IsApiAuthEnabled;
    return y;
}

CThostFtdcReqVerifyApiKeyField Converter::ReqVerifyApiKeyFieldToCpp(ReqVerifyApiKeyField x) {
    CThostFtdcReqVerifyApiKeyField y;
    memset(&y, 0, sizeof(y));
    y.ApiHandshakeDataLen = x.ApiHandshakeDataLen;
    memcpy(y.ApiHandshakeData, x.ApiHandshakeData.data(), x.ApiHandshakeData.size() * sizeof(uint8_t));
    return y;
}

ReqVerifyApiKeyField Converter::CThostFtdcReqVerifyApiKeyFieldToRust(CThostFtdcReqVerifyApiKeyField* x) {
    if (x == nullptr)
        return ReqVerifyApiKeyField{.is_null = true};
    ReqVerifyApiKeyField y{};
    y.ApiHandshakeDataLen = x->ApiHandshakeDataLen;
    for (int i = 0; i < 301; i++)
        y.ApiHandshakeData.push_back(x->ApiHandshakeData[i]);
    return y;
}

CThostFtdcDepartmentUserField Converter::DepartmentUserFieldToCpp(DepartmentUserField x) {
    CThostFtdcDepartmentUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

DepartmentUserField Converter::CThostFtdcDepartmentUserFieldToRust(CThostFtdcDepartmentUserField* x) {
    if (x == nullptr)
        return DepartmentUserField{.is_null = true};
    DepartmentUserField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.InvestorRange = x->InvestorRange;
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcQueryFreqField Converter::QueryFreqFieldToCpp(QueryFreqField x) {
    CThostFtdcQueryFreqField y;
    memset(&y, 0, sizeof(y));
    y.QueryFreq = x.QueryFreq;
    y.FTDPkgFreq = x.FTDPkgFreq;
    return y;
}

QueryFreqField Converter::CThostFtdcQueryFreqFieldToRust(CThostFtdcQueryFreqField* x) {
    if (x == nullptr)
        return QueryFreqField{.is_null = true};
    QueryFreqField y{};
    y.QueryFreq = x->QueryFreq;
    y.FTDPkgFreq = x->FTDPkgFreq;
    return y;
}

CThostFtdcAuthForbiddenIPField Converter::AuthForbiddenIPFieldToCpp(AuthForbiddenIPField x) {
    CThostFtdcAuthForbiddenIPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

AuthForbiddenIPField Converter::CThostFtdcAuthForbiddenIPFieldToRust(CThostFtdcAuthForbiddenIPField* x) {
    if (x == nullptr)
        return AuthForbiddenIPField{.is_null = true};
    AuthForbiddenIPField y{};
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryAuthForbiddenIPField Converter::QryAuthForbiddenIPFieldToCpp(QryAuthForbiddenIPField x) {
    CThostFtdcQryAuthForbiddenIPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

QryAuthForbiddenIPField Converter::CThostFtdcQryAuthForbiddenIPFieldToRust(CThostFtdcQryAuthForbiddenIPField* x) {
    if (x == nullptr)
        return QryAuthForbiddenIPField{.is_null = true};
    QryAuthForbiddenIPField y{};
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcSyncDelaySwapFrozenField Converter::SyncDelaySwapFrozenFieldToCpp(SyncDelaySwapFrozenField x) {
    CThostFtdcSyncDelaySwapFrozenField y;
    memset(&y, 0, sizeof(y));
    memcpy(y.DelaySwapSeqNo, x.DelaySwapSeqNo.data(), x.DelaySwapSeqNo.size() * sizeof(uint8_t));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.FromCurrencyID, x.FromCurrencyID.c_str());
    y.FromRemainSwap = x.FromRemainSwap;
    y.IsManualSwap = x.IsManualSwap;
    return y;
}

SyncDelaySwapFrozenField Converter::CThostFtdcSyncDelaySwapFrozenFieldToRust(CThostFtdcSyncDelaySwapFrozenField* x) {
    if (x == nullptr)
        return SyncDelaySwapFrozenField{.is_null = true};
    SyncDelaySwapFrozenField y{};
    for (int i = 0; i < 15; i++)
        y.DelaySwapSeqNo.push_back(x->DelaySwapSeqNo[i]);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.FromCurrencyID = Converter::Gb2312ToRustString(x->FromCurrencyID);
    y.FromRemainSwap = x->FromRemainSwap;
    y.IsManualSwap = x->IsManualSwap;
    return y;
}

CThostFtdcUserSystemInfoField Converter::UserSystemInfoFieldToCpp(UserSystemInfoField x) {
    CThostFtdcUserSystemInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.ClientSystemInfoLen = x.ClientSystemInfoLen;
    strcpy(y.ClientSystemInfo, x.ClientSystemInfo.c_str());
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientLoginTime, x.ClientLoginTime.c_str());
    strcpy(y.ClientAppID, x.ClientAppID.c_str());
    strcpy(y.ClientPublicIP, x.ClientPublicIP.c_str());
    strcpy(y.ClientLoginRemark, x.ClientLoginRemark.c_str());
    return y;
}

UserSystemInfoField Converter::CThostFtdcUserSystemInfoFieldToRust(CThostFtdcUserSystemInfoField* x) {
    if (x == nullptr)
        return UserSystemInfoField{.is_null = true};
    UserSystemInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ClientSystemInfoLen = x->ClientSystemInfoLen;
    y.ClientSystemInfo = Converter::Gb2312ToRustString(x->ClientSystemInfo);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientLoginTime = Converter::Gb2312ToRustString(x->ClientLoginTime);
    y.ClientAppID = Converter::Gb2312ToRustString(x->ClientAppID);
    y.ClientPublicIP = Converter::Gb2312ToRustString(x->ClientPublicIP);
    y.ClientLoginRemark = Converter::Gb2312ToRustString(x->ClientLoginRemark);
    return y;
}

CThostFtdcAuthUserIDField Converter::AuthUserIDFieldToCpp(AuthUserIDField x) {
    CThostFtdcAuthUserIDField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.AuthType = x.AuthType;
    return y;
}

AuthUserIDField Converter::CThostFtdcAuthUserIDFieldToRust(CThostFtdcAuthUserIDField* x) {
    if (x == nullptr)
        return AuthUserIDField{.is_null = true};
    AuthUserIDField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.AuthType = x->AuthType;
    return y;
}

CThostFtdcAuthIPField Converter::AuthIPFieldToCpp(AuthIPField x) {
    CThostFtdcAuthIPField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    return y;
}

AuthIPField Converter::CThostFtdcAuthIPFieldToRust(CThostFtdcAuthIPField* x) {
    if (x == nullptr)
        return AuthIPField{.is_null = true};
    AuthIPField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    return y;
}

CThostFtdcQryClassifiedInstrumentField Converter::QryClassifiedInstrumentFieldToCpp(QryClassifiedInstrumentField x) {
    CThostFtdcQryClassifiedInstrumentField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.TradingType = x.TradingType;
    y.ClassType = x.ClassType;
    return y;
}

QryClassifiedInstrumentField Converter::CThostFtdcQryClassifiedInstrumentFieldToRust(CThostFtdcQryClassifiedInstrumentField* x) {
    if (x == nullptr)
        return QryClassifiedInstrumentField{.is_null = true};
    QryClassifiedInstrumentField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.TradingType = x->TradingType;
    y.ClassType = x->ClassType;
    return y;
}

CThostFtdcQryCombPromotionParamField Converter::QryCombPromotionParamFieldToCpp(QryCombPromotionParamField x) {
    CThostFtdcQryCombPromotionParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryCombPromotionParamField Converter::CThostFtdcQryCombPromotionParamFieldToRust(CThostFtdcQryCombPromotionParamField* x) {
    if (x == nullptr)
        return QryCombPromotionParamField{.is_null = true};
    QryCombPromotionParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcCombPromotionParamField Converter::CombPromotionParamFieldToCpp(CombPromotionParamField x) {
    CThostFtdcCombPromotionParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.CombHedgeFlag, x.CombHedgeFlag.c_str());
    y.Xparameter = x.Xparameter;
    return y;
}

CombPromotionParamField Converter::CThostFtdcCombPromotionParamFieldToRust(CThostFtdcCombPromotionParamField* x) {
    if (x == nullptr)
        return CombPromotionParamField{.is_null = true};
    CombPromotionParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.CombHedgeFlag = Converter::Gb2312ToRustString(x->CombHedgeFlag);
    y.Xparameter = x->Xparameter;
    return y;
}

CThostFtdcReqUserLoginSMField Converter::ReqUserLoginSMFieldToCpp(ReqUserLoginSMField x) {
    CThostFtdcReqUserLoginSMField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Password, x.Password.c_str());
    strcpy(y.UserProductInfo, x.UserProductInfo.c_str());
    strcpy(y.InterfaceProductInfo, x.InterfaceProductInfo.c_str());
    strcpy(y.ProtocolInfo, x.ProtocolInfo.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.OneTimePassword, x.OneTimePassword.c_str());
    strcpy(y.LoginRemark, x.LoginRemark.c_str());
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientIPAddress, x.ClientIPAddress.c_str());
    strcpy(y.BrokerName, x.BrokerName.c_str());
    strcpy(y.AuthCode, x.AuthCode.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    memcpy(y.PIN, x.PIN.data(), x.PIN.size() * sizeof(uint8_t));
    return y;
}

ReqUserLoginSMField Converter::CThostFtdcReqUserLoginSMFieldToRust(CThostFtdcReqUserLoginSMField* x) {
    if (x == nullptr)
        return ReqUserLoginSMField{.is_null = true};
    ReqUserLoginSMField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.UserProductInfo = Converter::Gb2312ToRustString(x->UserProductInfo);
    y.InterfaceProductInfo = Converter::Gb2312ToRustString(x->InterfaceProductInfo);
    y.ProtocolInfo = Converter::Gb2312ToRustString(x->ProtocolInfo);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.OneTimePassword = Converter::Gb2312ToRustString(x->OneTimePassword);
    y.LoginRemark = Converter::Gb2312ToRustString(x->LoginRemark);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientIPAddress = Converter::Gb2312ToRustString(x->ClientIPAddress);
    y.BrokerName = Converter::Gb2312ToRustString(x->BrokerName);
    y.AuthCode = Converter::Gb2312ToRustString(x->AuthCode);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    for (int i = 0; i < 41; i++)
        y.PIN.push_back(x->PIN[i]);
    return y;
}

CThostFtdcQryRiskSettleInvstPositionField Converter::QryRiskSettleInvstPositionFieldToCpp(QryRiskSettleInvstPositionField x) {
    CThostFtdcQryRiskSettleInvstPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryRiskSettleInvstPositionField Converter::CThostFtdcQryRiskSettleInvstPositionFieldToRust(CThostFtdcQryRiskSettleInvstPositionField* x) {
    if (x == nullptr)
        return QryRiskSettleInvstPositionField{.is_null = true};
    QryRiskSettleInvstPositionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryRiskSettleProductStatusField Converter::QryRiskSettleProductStatusFieldToCpp(QryRiskSettleProductStatusField x) {
    CThostFtdcQryRiskSettleProductStatusField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QryRiskSettleProductStatusField Converter::CThostFtdcQryRiskSettleProductStatusFieldToRust(CThostFtdcQryRiskSettleProductStatusField* x) {
    if (x == nullptr)
        return QryRiskSettleProductStatusField{.is_null = true};
    QryRiskSettleProductStatusField y{};
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcRiskSettleInvstPositionField Converter::RiskSettleInvstPositionFieldToCpp(RiskSettleInvstPositionField x) {
    CThostFtdcRiskSettleInvstPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.PosiDirection = x.PosiDirection;
    y.HedgeFlag = x.HedgeFlag;
    y.PositionDate = x.PositionDate;
    y.YdPosition = x.YdPosition;
    y.Position = x.Position;
    y.LongFrozen = x.LongFrozen;
    y.ShortFrozen = x.ShortFrozen;
    y.LongFrozenAmount = x.LongFrozenAmount;
    y.ShortFrozenAmount = x.ShortFrozenAmount;
    y.OpenVolume = x.OpenVolume;
    y.CloseVolume = x.CloseVolume;
    y.OpenAmount = x.OpenAmount;
    y.CloseAmount = x.CloseAmount;
    y.PositionCost = x.PositionCost;
    y.PreMargin = x.PreMargin;
    y.UseMargin = x.UseMargin;
    y.FrozenMargin = x.FrozenMargin;
    y.FrozenCash = x.FrozenCash;
    y.FrozenCommission = x.FrozenCommission;
    y.CashIn = x.CashIn;
    y.Commission = x.Commission;
    y.CloseProfit = x.CloseProfit;
    y.PositionProfit = x.PositionProfit;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.SettlementPrice = x.SettlementPrice;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.OpenCost = x.OpenCost;
    y.ExchangeMargin = x.ExchangeMargin;
    y.CombPosition = x.CombPosition;
    y.CombLongFrozen = x.CombLongFrozen;
    y.CombShortFrozen = x.CombShortFrozen;
    y.CloseProfitByDate = x.CloseProfitByDate;
    y.CloseProfitByTrade = x.CloseProfitByTrade;
    y.TodayPosition = x.TodayPosition;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.StrikeFrozen = x.StrikeFrozen;
    y.StrikeFrozenAmount = x.StrikeFrozenAmount;
    y.AbandonFrozen = x.AbandonFrozen;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.YdStrikeFrozen = x.YdStrikeFrozen;
    strcpy(y.InvestUnitID, x.InvestUnitID.c_str());
    y.PositionCostOffset = x.PositionCostOffset;
    y.TasPosition = x.TasPosition;
    y.TasPositionCost = x.TasPositionCost;
    return y;
}

RiskSettleInvstPositionField Converter::CThostFtdcRiskSettleInvstPositionFieldToRust(CThostFtdcRiskSettleInvstPositionField* x) {
    if (x == nullptr)
        return RiskSettleInvstPositionField{.is_null = true};
    RiskSettleInvstPositionField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.PosiDirection = x->PosiDirection;
    y.HedgeFlag = x->HedgeFlag;
    y.PositionDate = x->PositionDate;
    y.YdPosition = x->YdPosition;
    y.Position = x->Position;
    y.LongFrozen = x->LongFrozen;
    y.ShortFrozen = x->ShortFrozen;
    y.LongFrozenAmount = x->LongFrozenAmount;
    y.ShortFrozenAmount = x->ShortFrozenAmount;
    y.OpenVolume = x->OpenVolume;
    y.CloseVolume = x->CloseVolume;
    y.OpenAmount = x->OpenAmount;
    y.CloseAmount = x->CloseAmount;
    y.PositionCost = x->PositionCost;
    y.PreMargin = x->PreMargin;
    y.UseMargin = x->UseMargin;
    y.FrozenMargin = x->FrozenMargin;
    y.FrozenCash = x->FrozenCash;
    y.FrozenCommission = x->FrozenCommission;
    y.CashIn = x->CashIn;
    y.Commission = x->Commission;
    y.CloseProfit = x->CloseProfit;
    y.PositionProfit = x->PositionProfit;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.SettlementPrice = x->SettlementPrice;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.OpenCost = x->OpenCost;
    y.ExchangeMargin = x->ExchangeMargin;
    y.CombPosition = x->CombPosition;
    y.CombLongFrozen = x->CombLongFrozen;
    y.CombShortFrozen = x->CombShortFrozen;
    y.CloseProfitByDate = x->CloseProfitByDate;
    y.CloseProfitByTrade = x->CloseProfitByTrade;
    y.TodayPosition = x->TodayPosition;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.StrikeFrozen = x->StrikeFrozen;
    y.StrikeFrozenAmount = x->StrikeFrozenAmount;
    y.AbandonFrozen = x->AbandonFrozen;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.YdStrikeFrozen = x->YdStrikeFrozen;
    y.InvestUnitID = Converter::Gb2312ToRustString(x->InvestUnitID);
    y.PositionCostOffset = x->PositionCostOffset;
    y.TasPosition = x->TasPosition;
    y.TasPositionCost = x->TasPositionCost;
    return y;
}

CThostFtdcRiskSettleProductStatusField Converter::RiskSettleProductStatusFieldToCpp(RiskSettleProductStatusField x) {
    CThostFtdcRiskSettleProductStatusField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.ProductStatus = x.ProductStatus;
    return y;
}

RiskSettleProductStatusField Converter::CThostFtdcRiskSettleProductStatusFieldToRust(CThostFtdcRiskSettleProductStatusField* x) {
    if (x == nullptr)
        return RiskSettleProductStatusField{.is_null = true};
    RiskSettleProductStatusField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.ProductStatus = x->ProductStatus;
    return y;
}

CThostFtdcSyncDeltaInfoField Converter::SyncDeltaInfoFieldToCpp(SyncDeltaInfoField x) {
    CThostFtdcSyncDeltaInfoField y;
    memset(&y, 0, sizeof(y));
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    y.SyncDeltaStatus = x.SyncDeltaStatus;
    memcpy(y.SyncDescription, x.SyncDescription.data(), x.SyncDescription.size() * sizeof(uint8_t));
    y.IsOnlyTrdDelta = x.IsOnlyTrdDelta;
    return y;
}

SyncDeltaInfoField Converter::CThostFtdcSyncDeltaInfoFieldToRust(CThostFtdcSyncDeltaInfoField* x) {
    if (x == nullptr)
        return SyncDeltaInfoField{.is_null = true};
    SyncDeltaInfoField y{};
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    y.SyncDeltaStatus = x->SyncDeltaStatus;
    for (int i = 0; i < 257; i++)
        y.SyncDescription.push_back(x->SyncDescription[i]);
    y.IsOnlyTrdDelta = x->IsOnlyTrdDelta;
    return y;
}

CThostFtdcSyncDeltaProductStatusField Converter::SyncDeltaProductStatusFieldToCpp(SyncDeltaProductStatusField x) {
    CThostFtdcSyncDeltaProductStatusField y;
    memset(&y, 0, sizeof(y));
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.ProductStatus = x.ProductStatus;
    return y;
}

SyncDeltaProductStatusField Converter::CThostFtdcSyncDeltaProductStatusFieldToRust(CThostFtdcSyncDeltaProductStatusField* x) {
    if (x == nullptr)
        return SyncDeltaProductStatusField{.is_null = true};
    SyncDeltaProductStatusField y{};
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.ProductStatus = x->ProductStatus;
    return y;
}

CThostFtdcSyncDeltaInvstPosDtlField Converter::SyncDeltaInvstPosDtlFieldToCpp(SyncDeltaInvstPosDtlField x) {
    CThostFtdcSyncDeltaInvstPosDtlField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.Direction = x.Direction;
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    y.Volume = x.Volume;
    y.OpenPrice = x.OpenPrice;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.TradeType = x.TradeType;
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.CloseProfitByDate = x.CloseProfitByDate;
    y.CloseProfitByTrade = x.CloseProfitByTrade;
    y.PositionProfitByDate = x.PositionProfitByDate;
    y.PositionProfitByTrade = x.PositionProfitByTrade;
    y.Margin = x.Margin;
    y.ExchMargin = x.ExchMargin;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.LastSettlementPrice = x.LastSettlementPrice;
    y.SettlementPrice = x.SettlementPrice;
    y.CloseVolume = x.CloseVolume;
    y.CloseAmount = x.CloseAmount;
    y.TimeFirstVolume = x.TimeFirstVolume;
    y.SpecPosiType = x.SpecPosiType;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInvstPosDtlField Converter::CThostFtdcSyncDeltaInvstPosDtlFieldToRust(CThostFtdcSyncDeltaInvstPosDtlField* x) {
    if (x == nullptr)
        return SyncDeltaInvstPosDtlField{.is_null = true};
    SyncDeltaInvstPosDtlField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.Direction = x->Direction;
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.Volume = x->Volume;
    y.OpenPrice = x->OpenPrice;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.TradeType = x->TradeType;
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.CloseProfitByDate = x->CloseProfitByDate;
    y.CloseProfitByTrade = x->CloseProfitByTrade;
    y.PositionProfitByDate = x->PositionProfitByDate;
    y.PositionProfitByTrade = x->PositionProfitByTrade;
    y.Margin = x->Margin;
    y.ExchMargin = x->ExchMargin;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.LastSettlementPrice = x->LastSettlementPrice;
    y.SettlementPrice = x->SettlementPrice;
    y.CloseVolume = x->CloseVolume;
    y.CloseAmount = x->CloseAmount;
    y.TimeFirstVolume = x->TimeFirstVolume;
    y.SpecPosiType = x->SpecPosiType;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaInvstPosCombDtlField Converter::SyncDeltaInvstPosCombDtlFieldToCpp(SyncDeltaInvstPosCombDtlField x) {
    CThostFtdcSyncDeltaInvstPosCombDtlField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.OpenDate, x.OpenDate.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ComTradeID, x.ComTradeID.c_str());
    strcpy(y.TradeID, x.TradeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.Direction = x.Direction;
    y.TotalAmt = x.TotalAmt;
    y.Margin = x.Margin;
    y.ExchMargin = x.ExchMargin;
    y.MarginRateByMoney = x.MarginRateByMoney;
    y.MarginRateByVolume = x.MarginRateByVolume;
    y.LegID = x.LegID;
    y.LegMultiple = x.LegMultiple;
    y.TradeGroupID = x.TradeGroupID;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInvstPosCombDtlField Converter::CThostFtdcSyncDeltaInvstPosCombDtlFieldToRust(CThostFtdcSyncDeltaInvstPosCombDtlField* x) {
    if (x == nullptr)
        return SyncDeltaInvstPosCombDtlField{.is_null = true};
    SyncDeltaInvstPosCombDtlField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.OpenDate = Converter::Gb2312ToRustString(x->OpenDate);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SettlementID = x->SettlementID;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ComTradeID = Converter::Gb2312ToRustString(x->ComTradeID);
    y.TradeID = Converter::Gb2312ToRustString(x->TradeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.HedgeFlag = x->HedgeFlag;
    y.Direction = x->Direction;
    y.TotalAmt = x->TotalAmt;
    y.Margin = x->Margin;
    y.ExchMargin = x->ExchMargin;
    y.MarginRateByMoney = x->MarginRateByMoney;
    y.MarginRateByVolume = x->MarginRateByVolume;
    y.LegID = x->LegID;
    y.LegMultiple = x->LegMultiple;
    y.TradeGroupID = x->TradeGroupID;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaTradingAccountField Converter::SyncDeltaTradingAccountFieldToCpp(SyncDeltaTradingAccountField x) {
    CThostFtdcSyncDeltaTradingAccountField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.PreMortgage = x.PreMortgage;
    y.PreCredit = x.PreCredit;
    y.PreDeposit = x.PreDeposit;
    y.PreBalance = x.PreBalance;
    y.PreMargin = x.PreMargin;
    y.InterestBase = x.InterestBase;
    y.Interest = x.Interest;
    y.Deposit = x.Deposit;
    y.Withdraw = x.Withdraw;
    y.FrozenMargin = x.FrozenMargin;
    y.FrozenCash = x.FrozenCash;
    y.FrozenCommission = x.FrozenCommission;
    y.CurrMargin = x.CurrMargin;
    y.CashIn = x.CashIn;
    y.Commission = x.Commission;
    y.CloseProfit = x.CloseProfit;
    y.PositionProfit = x.PositionProfit;
    y.Balance = x.Balance;
    y.Available = x.Available;
    y.WithdrawQuota = x.WithdrawQuota;
    y.Reserve = x.Reserve;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    y.Credit = x.Credit;
    y.Mortgage = x.Mortgage;
    y.ExchangeMargin = x.ExchangeMargin;
    y.DeliveryMargin = x.DeliveryMargin;
    y.ExchangeDeliveryMargin = x.ExchangeDeliveryMargin;
    y.ReserveBalance = x.ReserveBalance;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.PreFundMortgageIn = x.PreFundMortgageIn;
    y.PreFundMortgageOut = x.PreFundMortgageOut;
    y.FundMortgageIn = x.FundMortgageIn;
    y.FundMortgageOut = x.FundMortgageOut;
    y.FundMortgageAvailable = x.FundMortgageAvailable;
    y.MortgageableFund = x.MortgageableFund;
    y.SpecProductMargin = x.SpecProductMargin;
    y.SpecProductFrozenMargin = x.SpecProductFrozenMargin;
    y.SpecProductCommission = x.SpecProductCommission;
    y.SpecProductFrozenCommission = x.SpecProductFrozenCommission;
    y.SpecProductPositionProfit = x.SpecProductPositionProfit;
    y.SpecProductCloseProfit = x.SpecProductCloseProfit;
    y.SpecProductPositionProfitByAlg = x.SpecProductPositionProfitByAlg;
    y.SpecProductExchangeMargin = x.SpecProductExchangeMargin;
    y.FrozenSwap = x.FrozenSwap;
    y.RemainSwap = x.RemainSwap;
    y.OptionValue = x.OptionValue;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaTradingAccountField Converter::CThostFtdcSyncDeltaTradingAccountFieldToRust(CThostFtdcSyncDeltaTradingAccountField* x) {
    if (x == nullptr)
        return SyncDeltaTradingAccountField{.is_null = true};
    SyncDeltaTradingAccountField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.PreMortgage = x->PreMortgage;
    y.PreCredit = x->PreCredit;
    y.PreDeposit = x->PreDeposit;
    y.PreBalance = x->PreBalance;
    y.PreMargin = x->PreMargin;
    y.InterestBase = x->InterestBase;
    y.Interest = x->Interest;
    y.Deposit = x->Deposit;
    y.Withdraw = x->Withdraw;
    y.FrozenMargin = x->FrozenMargin;
    y.FrozenCash = x->FrozenCash;
    y.FrozenCommission = x->FrozenCommission;
    y.CurrMargin = x->CurrMargin;
    y.CashIn = x->CashIn;
    y.Commission = x->Commission;
    y.CloseProfit = x->CloseProfit;
    y.PositionProfit = x->PositionProfit;
    y.Balance = x->Balance;
    y.Available = x->Available;
    y.WithdrawQuota = x->WithdrawQuota;
    y.Reserve = x->Reserve;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.Credit = x->Credit;
    y.Mortgage = x->Mortgage;
    y.ExchangeMargin = x->ExchangeMargin;
    y.DeliveryMargin = x->DeliveryMargin;
    y.ExchangeDeliveryMargin = x->ExchangeDeliveryMargin;
    y.ReserveBalance = x->ReserveBalance;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.PreFundMortgageIn = x->PreFundMortgageIn;
    y.PreFundMortgageOut = x->PreFundMortgageOut;
    y.FundMortgageIn = x->FundMortgageIn;
    y.FundMortgageOut = x->FundMortgageOut;
    y.FundMortgageAvailable = x->FundMortgageAvailable;
    y.MortgageableFund = x->MortgageableFund;
    y.SpecProductMargin = x->SpecProductMargin;
    y.SpecProductFrozenMargin = x->SpecProductFrozenMargin;
    y.SpecProductCommission = x->SpecProductCommission;
    y.SpecProductFrozenCommission = x->SpecProductFrozenCommission;
    y.SpecProductPositionProfit = x->SpecProductPositionProfit;
    y.SpecProductCloseProfit = x->SpecProductCloseProfit;
    y.SpecProductPositionProfitByAlg = x->SpecProductPositionProfitByAlg;
    y.SpecProductExchangeMargin = x->SpecProductExchangeMargin;
    y.FrozenSwap = x->FrozenSwap;
    y.RemainSwap = x->RemainSwap;
    y.OptionValue = x->OptionValue;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaInitInvstMarginField Converter::SyncDeltaInitInvstMarginFieldToCpp(SyncDeltaInitInvstMarginField x) {
    CThostFtdcSyncDeltaInitInvstMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.LastRiskTotalInvstMargin = x.LastRiskTotalInvstMargin;
    y.LastRiskTotalExchMargin = x.LastRiskTotalExchMargin;
    y.ThisSyncInvstMargin = x.ThisSyncInvstMargin;
    y.ThisSyncExchMargin = x.ThisSyncExchMargin;
    y.RemainRiskInvstMargin = x.RemainRiskInvstMargin;
    y.RemainRiskExchMargin = x.RemainRiskExchMargin;
    y.LastRiskSpecTotalInvstMargin = x.LastRiskSpecTotalInvstMargin;
    y.LastRiskSpecTotalExchMargin = x.LastRiskSpecTotalExchMargin;
    y.ThisSyncSpecInvstMargin = x.ThisSyncSpecInvstMargin;
    y.ThisSyncSpecExchMargin = x.ThisSyncSpecExchMargin;
    y.RemainRiskSpecInvstMargin = x.RemainRiskSpecInvstMargin;
    y.RemainRiskSpecExchMargin = x.RemainRiskSpecExchMargin;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInitInvstMarginField Converter::CThostFtdcSyncDeltaInitInvstMarginFieldToRust(CThostFtdcSyncDeltaInitInvstMarginField* x) {
    if (x == nullptr)
        return SyncDeltaInitInvstMarginField{.is_null = true};
    SyncDeltaInitInvstMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.LastRiskTotalInvstMargin = x->LastRiskTotalInvstMargin;
    y.LastRiskTotalExchMargin = x->LastRiskTotalExchMargin;
    y.ThisSyncInvstMargin = x->ThisSyncInvstMargin;
    y.ThisSyncExchMargin = x->ThisSyncExchMargin;
    y.RemainRiskInvstMargin = x->RemainRiskInvstMargin;
    y.RemainRiskExchMargin = x->RemainRiskExchMargin;
    y.LastRiskSpecTotalInvstMargin = x->LastRiskSpecTotalInvstMargin;
    y.LastRiskSpecTotalExchMargin = x->LastRiskSpecTotalExchMargin;
    y.ThisSyncSpecInvstMargin = x->ThisSyncSpecInvstMargin;
    y.ThisSyncSpecExchMargin = x->ThisSyncSpecExchMargin;
    y.RemainRiskSpecInvstMargin = x->RemainRiskSpecInvstMargin;
    y.RemainRiskSpecExchMargin = x->RemainRiskSpecExchMargin;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaDceCombInstrumentField Converter::SyncDeltaDceCombInstrumentFieldToCpp(SyncDeltaDceCombInstrumentField x) {
    CThostFtdcSyncDeltaDceCombInstrumentField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    y.TradeGroupID = x.TradeGroupID;
    y.CombHedgeFlag = x.CombHedgeFlag;
    y.CombinationType = x.CombinationType;
    y.Direction = x.Direction;
    strcpy(y.ProductID, x.ProductID.c_str());
    y.Xparameter = x.Xparameter;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaDceCombInstrumentField Converter::CThostFtdcSyncDeltaDceCombInstrumentFieldToRust(CThostFtdcSyncDeltaDceCombInstrumentField* x) {
    if (x == nullptr)
        return SyncDeltaDceCombInstrumentField{.is_null = true};
    SyncDeltaDceCombInstrumentField y{};
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.TradeGroupID = x->TradeGroupID;
    y.CombHedgeFlag = x->CombHedgeFlag;
    y.CombinationType = x->CombinationType;
    y.Direction = x->Direction;
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.Xparameter = x->Xparameter;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaInvstMarginRateField Converter::SyncDeltaInvstMarginRateFieldToCpp(SyncDeltaInvstMarginRateField x) {
    CThostFtdcSyncDeltaInvstMarginRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.IsRelative = x.IsRelative;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInvstMarginRateField Converter::CThostFtdcSyncDeltaInvstMarginRateFieldToRust(CThostFtdcSyncDeltaInvstMarginRateField* x) {
    if (x == nullptr)
        return SyncDeltaInvstMarginRateField{.is_null = true};
    SyncDeltaInvstMarginRateField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.IsRelative = x->IsRelative;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaExchMarginRateField Converter::SyncDeltaExchMarginRateFieldToCpp(SyncDeltaExchMarginRateField x) {
    CThostFtdcSyncDeltaExchMarginRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaExchMarginRateField Converter::CThostFtdcSyncDeltaExchMarginRateFieldToRust(CThostFtdcSyncDeltaExchMarginRateField* x) {
    if (x == nullptr)
        return SyncDeltaExchMarginRateField{.is_null = true};
    SyncDeltaExchMarginRateField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaOptExchMarginField Converter::SyncDeltaOptExchMarginFieldToCpp(SyncDeltaOptExchMarginField x) {
    CThostFtdcSyncDeltaOptExchMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.SShortMarginRatioByMoney = x.SShortMarginRatioByMoney;
    y.SShortMarginRatioByVolume = x.SShortMarginRatioByVolume;
    y.HShortMarginRatioByMoney = x.HShortMarginRatioByMoney;
    y.HShortMarginRatioByVolume = x.HShortMarginRatioByVolume;
    y.AShortMarginRatioByMoney = x.AShortMarginRatioByMoney;
    y.AShortMarginRatioByVolume = x.AShortMarginRatioByVolume;
    y.MShortMarginRatioByMoney = x.MShortMarginRatioByMoney;
    y.MShortMarginRatioByVolume = x.MShortMarginRatioByVolume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaOptExchMarginField Converter::CThostFtdcSyncDeltaOptExchMarginFieldToRust(CThostFtdcSyncDeltaOptExchMarginField* x) {
    if (x == nullptr)
        return SyncDeltaOptExchMarginField{.is_null = true};
    SyncDeltaOptExchMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.SShortMarginRatioByMoney = x->SShortMarginRatioByMoney;
    y.SShortMarginRatioByVolume = x->SShortMarginRatioByVolume;
    y.HShortMarginRatioByMoney = x->HShortMarginRatioByMoney;
    y.HShortMarginRatioByVolume = x->HShortMarginRatioByVolume;
    y.AShortMarginRatioByMoney = x->AShortMarginRatioByMoney;
    y.AShortMarginRatioByVolume = x->AShortMarginRatioByVolume;
    y.MShortMarginRatioByMoney = x->MShortMarginRatioByMoney;
    y.MShortMarginRatioByVolume = x->MShortMarginRatioByVolume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaOptInvstMarginField Converter::SyncDeltaOptInvstMarginFieldToCpp(SyncDeltaOptInvstMarginField x) {
    CThostFtdcSyncDeltaOptInvstMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.SShortMarginRatioByMoney = x.SShortMarginRatioByMoney;
    y.SShortMarginRatioByVolume = x.SShortMarginRatioByVolume;
    y.HShortMarginRatioByMoney = x.HShortMarginRatioByMoney;
    y.HShortMarginRatioByVolume = x.HShortMarginRatioByVolume;
    y.AShortMarginRatioByMoney = x.AShortMarginRatioByMoney;
    y.AShortMarginRatioByVolume = x.AShortMarginRatioByVolume;
    y.IsRelative = x.IsRelative;
    y.MShortMarginRatioByMoney = x.MShortMarginRatioByMoney;
    y.MShortMarginRatioByVolume = x.MShortMarginRatioByVolume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaOptInvstMarginField Converter::CThostFtdcSyncDeltaOptInvstMarginFieldToRust(CThostFtdcSyncDeltaOptInvstMarginField* x) {
    if (x == nullptr)
        return SyncDeltaOptInvstMarginField{.is_null = true};
    SyncDeltaOptInvstMarginField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SShortMarginRatioByMoney = x->SShortMarginRatioByMoney;
    y.SShortMarginRatioByVolume = x->SShortMarginRatioByVolume;
    y.HShortMarginRatioByMoney = x->HShortMarginRatioByMoney;
    y.HShortMarginRatioByVolume = x->HShortMarginRatioByVolume;
    y.AShortMarginRatioByMoney = x->AShortMarginRatioByMoney;
    y.AShortMarginRatioByVolume = x->AShortMarginRatioByVolume;
    y.IsRelative = x->IsRelative;
    y.MShortMarginRatioByMoney = x->MShortMarginRatioByMoney;
    y.MShortMarginRatioByVolume = x->MShortMarginRatioByVolume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaInvstMarginRateULField Converter::SyncDeltaInvstMarginRateULFieldToCpp(SyncDeltaInvstMarginRateULField x) {
    CThostFtdcSyncDeltaInvstMarginRateULField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.LongMarginRatioByMoney = x.LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x.LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x.ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x.ShortMarginRatioByVolume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInvstMarginRateULField Converter::CThostFtdcSyncDeltaInvstMarginRateULFieldToRust(CThostFtdcSyncDeltaInvstMarginRateULField* x) {
    if (x == nullptr)
        return SyncDeltaInvstMarginRateULField{.is_null = true};
    SyncDeltaInvstMarginRateULField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.LongMarginRatioByMoney = x->LongMarginRatioByMoney;
    y.LongMarginRatioByVolume = x->LongMarginRatioByVolume;
    y.ShortMarginRatioByMoney = x->ShortMarginRatioByMoney;
    y.ShortMarginRatioByVolume = x->ShortMarginRatioByVolume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaOptInvstCommRateField Converter::SyncDeltaOptInvstCommRateFieldToCpp(SyncDeltaOptInvstCommRateField x) {
    CThostFtdcSyncDeltaOptInvstCommRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    y.StrikeRatioByMoney = x.StrikeRatioByMoney;
    y.StrikeRatioByVolume = x.StrikeRatioByVolume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaOptInvstCommRateField Converter::CThostFtdcSyncDeltaOptInvstCommRateFieldToRust(CThostFtdcSyncDeltaOptInvstCommRateField* x) {
    if (x == nullptr)
        return SyncDeltaOptInvstCommRateField{.is_null = true};
    SyncDeltaOptInvstCommRateField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.StrikeRatioByMoney = x->StrikeRatioByMoney;
    y.StrikeRatioByVolume = x->StrikeRatioByVolume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaInvstCommRateField Converter::SyncDeltaInvstCommRateFieldToCpp(SyncDeltaInvstCommRateField x) {
    CThostFtdcSyncDeltaInvstCommRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.OpenRatioByMoney = x.OpenRatioByMoney;
    y.OpenRatioByVolume = x.OpenRatioByVolume;
    y.CloseRatioByMoney = x.CloseRatioByMoney;
    y.CloseRatioByVolume = x.CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x.CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x.CloseTodayRatioByVolume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInvstCommRateField Converter::CThostFtdcSyncDeltaInvstCommRateFieldToRust(CThostFtdcSyncDeltaInvstCommRateField* x) {
    if (x == nullptr)
        return SyncDeltaInvstCommRateField{.is_null = true};
    SyncDeltaInvstCommRateField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.OpenRatioByMoney = x->OpenRatioByMoney;
    y.OpenRatioByVolume = x->OpenRatioByVolume;
    y.CloseRatioByMoney = x->CloseRatioByMoney;
    y.CloseRatioByVolume = x->CloseRatioByVolume;
    y.CloseTodayRatioByMoney = x->CloseTodayRatioByMoney;
    y.CloseTodayRatioByVolume = x->CloseTodayRatioByVolume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaProductExchRateField Converter::SyncDeltaProductExchRateFieldToCpp(SyncDeltaProductExchRateField x) {
    CThostFtdcSyncDeltaProductExchRateField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.QuoteCurrencyID, x.QuoteCurrencyID.c_str());
    y.ExchangeRate = x.ExchangeRate;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaProductExchRateField Converter::CThostFtdcSyncDeltaProductExchRateFieldToRust(CThostFtdcSyncDeltaProductExchRateField* x) {
    if (x == nullptr)
        return SyncDeltaProductExchRateField{.is_null = true};
    SyncDeltaProductExchRateField y{};
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.QuoteCurrencyID = Converter::Gb2312ToRustString(x->QuoteCurrencyID);
    y.ExchangeRate = x->ExchangeRate;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaDepthMarketDataField Converter::SyncDeltaDepthMarketDataFieldToCpp(SyncDeltaDepthMarketDataField x) {
    CThostFtdcSyncDeltaDepthMarketDataField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    y.LastPrice = x.LastPrice;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.PreClosePrice = x.PreClosePrice;
    y.PreOpenInterest = x.PreOpenInterest;
    y.OpenPrice = x.OpenPrice;
    y.HighestPrice = x.HighestPrice;
    y.LowestPrice = x.LowestPrice;
    y.Volume = x.Volume;
    y.Turnover = x.Turnover;
    y.OpenInterest = x.OpenInterest;
    y.ClosePrice = x.ClosePrice;
    y.SettlementPrice = x.SettlementPrice;
    y.UpperLimitPrice = x.UpperLimitPrice;
    y.LowerLimitPrice = x.LowerLimitPrice;
    y.PreDelta = x.PreDelta;
    y.CurrDelta = x.CurrDelta;
    strcpy(y.UpdateTime, x.UpdateTime.c_str());
    y.UpdateMillisec = x.UpdateMillisec;
    y.BidPrice1 = x.BidPrice1;
    y.BidVolume1 = x.BidVolume1;
    y.AskPrice1 = x.AskPrice1;
    y.AskVolume1 = x.AskVolume1;
    y.BidPrice2 = x.BidPrice2;
    y.BidVolume2 = x.BidVolume2;
    y.AskPrice2 = x.AskPrice2;
    y.AskVolume2 = x.AskVolume2;
    y.BidPrice3 = x.BidPrice3;
    y.BidVolume3 = x.BidVolume3;
    y.AskPrice3 = x.AskPrice3;
    y.AskVolume3 = x.AskVolume3;
    y.BidPrice4 = x.BidPrice4;
    y.BidVolume4 = x.BidVolume4;
    y.AskPrice4 = x.AskPrice4;
    y.AskVolume4 = x.AskVolume4;
    y.BidPrice5 = x.BidPrice5;
    y.BidVolume5 = x.BidVolume5;
    y.AskPrice5 = x.AskPrice5;
    y.AskVolume5 = x.AskVolume5;
    y.AveragePrice = x.AveragePrice;
    strcpy(y.ActionDay, x.ActionDay.c_str());
    y.BandingUpperPrice = x.BandingUpperPrice;
    y.BandingLowerPrice = x.BandingLowerPrice;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaDepthMarketDataField Converter::CThostFtdcSyncDeltaDepthMarketDataFieldToRust(CThostFtdcSyncDeltaDepthMarketDataField* x) {
    if (x == nullptr)
        return SyncDeltaDepthMarketDataField{.is_null = true};
    SyncDeltaDepthMarketDataField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.LastPrice = x->LastPrice;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.PreClosePrice = x->PreClosePrice;
    y.PreOpenInterest = x->PreOpenInterest;
    y.OpenPrice = x->OpenPrice;
    y.HighestPrice = x->HighestPrice;
    y.LowestPrice = x->LowestPrice;
    y.Volume = x->Volume;
    y.Turnover = x->Turnover;
    y.OpenInterest = x->OpenInterest;
    y.ClosePrice = x->ClosePrice;
    y.SettlementPrice = x->SettlementPrice;
    y.UpperLimitPrice = x->UpperLimitPrice;
    y.LowerLimitPrice = x->LowerLimitPrice;
    y.PreDelta = x->PreDelta;
    y.CurrDelta = x->CurrDelta;
    y.UpdateTime = Converter::Gb2312ToRustString(x->UpdateTime);
    y.UpdateMillisec = x->UpdateMillisec;
    y.BidPrice1 = x->BidPrice1;
    y.BidVolume1 = x->BidVolume1;
    y.AskPrice1 = x->AskPrice1;
    y.AskVolume1 = x->AskVolume1;
    y.BidPrice2 = x->BidPrice2;
    y.BidVolume2 = x->BidVolume2;
    y.AskPrice2 = x->AskPrice2;
    y.AskVolume2 = x->AskVolume2;
    y.BidPrice3 = x->BidPrice3;
    y.BidVolume3 = x->BidVolume3;
    y.AskPrice3 = x->AskPrice3;
    y.AskVolume3 = x->AskVolume3;
    y.BidPrice4 = x->BidPrice4;
    y.BidVolume4 = x->BidVolume4;
    y.AskPrice4 = x->AskPrice4;
    y.AskVolume4 = x->AskVolume4;
    y.BidPrice5 = x->BidPrice5;
    y.BidVolume5 = x->BidVolume5;
    y.AskPrice5 = x->AskPrice5;
    y.AskVolume5 = x->AskVolume5;
    y.AveragePrice = x->AveragePrice;
    y.ActionDay = Converter::Gb2312ToRustString(x->ActionDay);
    y.BandingUpperPrice = x->BandingUpperPrice;
    y.BandingLowerPrice = x->BandingLowerPrice;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaIndexPriceField Converter::SyncDeltaIndexPriceFieldToCpp(SyncDeltaIndexPriceField x) {
    CThostFtdcSyncDeltaIndexPriceField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.ClosePrice = x.ClosePrice;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaIndexPriceField Converter::CThostFtdcSyncDeltaIndexPriceFieldToRust(CThostFtdcSyncDeltaIndexPriceField* x) {
    if (x == nullptr)
        return SyncDeltaIndexPriceField{.is_null = true};
    SyncDeltaIndexPriceField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ClosePrice = x->ClosePrice;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaEWarrantOffsetField Converter::SyncDeltaEWarrantOffsetFieldToCpp(SyncDeltaEWarrantOffsetField x) {
    CThostFtdcSyncDeltaEWarrantOffsetField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.Direction = x.Direction;
    y.HedgeFlag = x.HedgeFlag;
    y.Volume = x.Volume;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaEWarrantOffsetField Converter::CThostFtdcSyncDeltaEWarrantOffsetFieldToRust(CThostFtdcSyncDeltaEWarrantOffsetField* x) {
    if (x == nullptr)
        return SyncDeltaEWarrantOffsetField{.is_null = true};
    SyncDeltaEWarrantOffsetField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.Direction = x->Direction;
    y.HedgeFlag = x->HedgeFlag;
    y.Volume = x->Volume;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSPBMFutureParameterField Converter::SPBMFutureParameterFieldToCpp(SPBMFutureParameterField x) {
    CThostFtdcSPBMFutureParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.Cvf = x.Cvf;
    y.TimeRange = x.TimeRange;
    y.MarginRate = x.MarginRate;
    y.LockRateX = x.LockRateX;
    y.AddOnRate = x.AddOnRate;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.AddOnLockRateX2 = x.AddOnLockRateX2;
    return y;
}

SPBMFutureParameterField Converter::CThostFtdcSPBMFutureParameterFieldToRust(CThostFtdcSPBMFutureParameterField* x) {
    if (x == nullptr)
        return SPBMFutureParameterField{.is_null = true};
    SPBMFutureParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.Cvf = x->Cvf;
    y.TimeRange = x->TimeRange;
    y.MarginRate = x->MarginRate;
    y.LockRateX = x->LockRateX;
    y.AddOnRate = x->AddOnRate;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.AddOnLockRateX2 = x->AddOnLockRateX2;
    return y;
}

CThostFtdcSPBMOptionParameterField Converter::SPBMOptionParameterFieldToCpp(SPBMOptionParameterField x) {
    CThostFtdcSPBMOptionParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.Cvf = x.Cvf;
    y.DownPrice = x.DownPrice;
    y.Delta = x.Delta;
    y.SlimiDelta = x.SlimiDelta;
    y.PreSettlementPrice = x.PreSettlementPrice;
    return y;
}

SPBMOptionParameterField Converter::CThostFtdcSPBMOptionParameterFieldToRust(CThostFtdcSPBMOptionParameterField* x) {
    if (x == nullptr)
        return SPBMOptionParameterField{.is_null = true};
    SPBMOptionParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.Cvf = x->Cvf;
    y.DownPrice = x->DownPrice;
    y.Delta = x->Delta;
    y.SlimiDelta = x->SlimiDelta;
    y.PreSettlementPrice = x->PreSettlementPrice;
    return y;
}

CThostFtdcSPBMIntraParameterField Converter::SPBMIntraParameterFieldToCpp(SPBMIntraParameterField x) {
    CThostFtdcSPBMIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.IntraRateY = x.IntraRateY;
    y.AddOnIntraRateY2 = x.AddOnIntraRateY2;
    return y;
}

SPBMIntraParameterField Converter::CThostFtdcSPBMIntraParameterFieldToRust(CThostFtdcSPBMIntraParameterField* x) {
    if (x == nullptr)
        return SPBMIntraParameterField{.is_null = true};
    SPBMIntraParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.IntraRateY = x->IntraRateY;
    y.AddOnIntraRateY2 = x->AddOnIntraRateY2;
    return y;
}

CThostFtdcSPBMInterParameterField Converter::SPBMInterParameterFieldToCpp(SPBMInterParameterField x) {
    CThostFtdcSPBMInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SpreadId = x.SpreadId;
    y.InterRateZ = x.InterRateZ;
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    return y;
}

SPBMInterParameterField Converter::CThostFtdcSPBMInterParameterFieldToRust(CThostFtdcSPBMInterParameterField* x) {
    if (x == nullptr)
        return SPBMInterParameterField{.is_null = true};
    SPBMInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SpreadId = x->SpreadId;
    y.InterRateZ = x->InterRateZ;
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    return y;
}

CThostFtdcSyncSPBMParameterEndField Converter::SyncSPBMParameterEndFieldToCpp(SyncSPBMParameterEndField x) {
    CThostFtdcSyncSPBMParameterEndField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    return y;
}

SyncSPBMParameterEndField Converter::CThostFtdcSyncSPBMParameterEndFieldToRust(CThostFtdcSyncSPBMParameterEndField* x) {
    if (x == nullptr)
        return SyncSPBMParameterEndField{.is_null = true};
    SyncSPBMParameterEndField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    return y;
}

CThostFtdcQrySPBMFutureParameterField Converter::QrySPBMFutureParameterFieldToCpp(QrySPBMFutureParameterField x) {
    CThostFtdcQrySPBMFutureParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    return y;
}

QrySPBMFutureParameterField Converter::CThostFtdcQrySPBMFutureParameterFieldToRust(CThostFtdcQrySPBMFutureParameterField* x) {
    if (x == nullptr)
        return QrySPBMFutureParameterField{.is_null = true};
    QrySPBMFutureParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    return y;
}

CThostFtdcQrySPBMOptionParameterField Converter::QrySPBMOptionParameterFieldToCpp(QrySPBMOptionParameterField x) {
    CThostFtdcQrySPBMOptionParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    return y;
}

QrySPBMOptionParameterField Converter::CThostFtdcQrySPBMOptionParameterFieldToRust(CThostFtdcQrySPBMOptionParameterField* x) {
    if (x == nullptr)
        return QrySPBMOptionParameterField{.is_null = true};
    QrySPBMOptionParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    return y;
}

CThostFtdcQrySPBMIntraParameterField Converter::QrySPBMIntraParameterFieldToCpp(QrySPBMIntraParameterField x) {
    CThostFtdcQrySPBMIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    return y;
}

QrySPBMIntraParameterField Converter::CThostFtdcQrySPBMIntraParameterFieldToRust(CThostFtdcQrySPBMIntraParameterField* x) {
    if (x == nullptr)
        return QrySPBMIntraParameterField{.is_null = true};
    QrySPBMIntraParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    return y;
}

CThostFtdcQrySPBMInterParameterField Converter::QrySPBMInterParameterFieldToCpp(QrySPBMInterParameterField x) {
    CThostFtdcQrySPBMInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    return y;
}

QrySPBMInterParameterField Converter::CThostFtdcQrySPBMInterParameterFieldToRust(CThostFtdcQrySPBMInterParameterField* x) {
    if (x == nullptr)
        return QrySPBMInterParameterField{.is_null = true};
    QrySPBMInterParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    return y;
}

CThostFtdcSPBMPortfDefinitionField Converter::SPBMPortfDefinitionFieldToCpp(SPBMPortfDefinitionField x) {
    CThostFtdcSPBMPortfDefinitionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.PortfolioDefID = x.PortfolioDefID;
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.IsSPBM = x.IsSPBM;
    return y;
}

SPBMPortfDefinitionField Converter::CThostFtdcSPBMPortfDefinitionFieldToRust(CThostFtdcSPBMPortfDefinitionField* x) {
    if (x == nullptr)
        return SPBMPortfDefinitionField{.is_null = true};
    SPBMPortfDefinitionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.PortfolioDefID = x->PortfolioDefID;
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.IsSPBM = x->IsSPBM;
    return y;
}

CThostFtdcSPBMInvestorPortfDefField Converter::SPBMInvestorPortfDefFieldToCpp(SPBMInvestorPortfDefField x) {
    CThostFtdcSPBMInvestorPortfDefField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.PortfolioDefID = x.PortfolioDefID;
    return y;
}

SPBMInvestorPortfDefField Converter::CThostFtdcSPBMInvestorPortfDefFieldToRust(CThostFtdcSPBMInvestorPortfDefField* x) {
    if (x == nullptr)
        return SPBMInvestorPortfDefField{.is_null = true};
    SPBMInvestorPortfDefField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.PortfolioDefID = x->PortfolioDefID;
    return y;
}

CThostFtdcInvestorPortfMarginRatioField Converter::InvestorPortfMarginRatioFieldToCpp(InvestorPortfMarginRatioField x) {
    CThostFtdcInvestorPortfMarginRatioField y;
    memset(&y, 0, sizeof(y));
    y.InvestorRange = x.InvestorRange;
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.MarginRatio = x.MarginRatio;
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

InvestorPortfMarginRatioField Converter::CThostFtdcInvestorPortfMarginRatioFieldToRust(CThostFtdcInvestorPortfMarginRatioField* x) {
    if (x == nullptr)
        return InvestorPortfMarginRatioField{.is_null = true};
    InvestorPortfMarginRatioField y{};
    y.InvestorRange = x->InvestorRange;
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.MarginRatio = x->MarginRatio;
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcQrySPBMPortfDefinitionField Converter::QrySPBMPortfDefinitionFieldToCpp(QrySPBMPortfDefinitionField x) {
    CThostFtdcQrySPBMPortfDefinitionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.PortfolioDefID = x.PortfolioDefID;
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    return y;
}

QrySPBMPortfDefinitionField Converter::CThostFtdcQrySPBMPortfDefinitionFieldToRust(CThostFtdcQrySPBMPortfDefinitionField* x) {
    if (x == nullptr)
        return QrySPBMPortfDefinitionField{.is_null = true};
    QrySPBMPortfDefinitionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.PortfolioDefID = x->PortfolioDefID;
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    return y;
}

CThostFtdcQrySPBMInvestorPortfDefField Converter::QrySPBMInvestorPortfDefFieldToCpp(QrySPBMInvestorPortfDefField x) {
    CThostFtdcQrySPBMInvestorPortfDefField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QrySPBMInvestorPortfDefField Converter::CThostFtdcQrySPBMInvestorPortfDefFieldToRust(CThostFtdcQrySPBMInvestorPortfDefField* x) {
    if (x == nullptr)
        return QrySPBMInvestorPortfDefField{.is_null = true};
    QrySPBMInvestorPortfDefField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcQryInvestorPortfMarginRatioField Converter::QryInvestorPortfMarginRatioFieldToCpp(QryInvestorPortfMarginRatioField x) {
    CThostFtdcQryInvestorPortfMarginRatioField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

QryInvestorPortfMarginRatioField Converter::CThostFtdcQryInvestorPortfMarginRatioFieldToRust(CThostFtdcQryInvestorPortfMarginRatioField* x) {
    if (x == nullptr)
        return QryInvestorPortfMarginRatioField{.is_null = true};
    QryInvestorPortfMarginRatioField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcInvestorProdSPBMDetailField Converter::InvestorProdSPBMDetailFieldToCpp(InvestorProdSPBMDetailField x) {
    CThostFtdcInvestorProdSPBMDetailField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.IntraInstrMargin = x.IntraInstrMargin;
    y.BCollectingMargin = x.BCollectingMargin;
    y.SCollectingMargin = x.SCollectingMargin;
    y.IntraProdMargin = x.IntraProdMargin;
    y.NetMargin = x.NetMargin;
    y.InterProdMargin = x.InterProdMargin;
    y.SingleMargin = x.SingleMargin;
    y.AddOnMargin = x.AddOnMargin;
    y.DeliveryMargin = x.DeliveryMargin;
    y.CallOptionMinRisk = x.CallOptionMinRisk;
    y.PutOptionMinRisk = x.PutOptionMinRisk;
    y.OptionMinRisk = x.OptionMinRisk;
    y.OptionValueOffset = x.OptionValueOffset;
    y.OptionRoyalty = x.OptionRoyalty;
    y.RealOptionValueOffset = x.RealOptionValueOffset;
    y.Margin = x.Margin;
    y.ExchMargin = x.ExchMargin;
    return y;
}

InvestorProdSPBMDetailField Converter::CThostFtdcInvestorProdSPBMDetailFieldToRust(CThostFtdcInvestorProdSPBMDetailField* x) {
    if (x == nullptr)
        return InvestorProdSPBMDetailField{.is_null = true};
    InvestorProdSPBMDetailField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.IntraInstrMargin = x->IntraInstrMargin;
    y.BCollectingMargin = x->BCollectingMargin;
    y.SCollectingMargin = x->SCollectingMargin;
    y.IntraProdMargin = x->IntraProdMargin;
    y.NetMargin = x->NetMargin;
    y.InterProdMargin = x->InterProdMargin;
    y.SingleMargin = x->SingleMargin;
    y.AddOnMargin = x->AddOnMargin;
    y.DeliveryMargin = x->DeliveryMargin;
    y.CallOptionMinRisk = x->CallOptionMinRisk;
    y.PutOptionMinRisk = x->PutOptionMinRisk;
    y.OptionMinRisk = x->OptionMinRisk;
    y.OptionValueOffset = x->OptionValueOffset;
    y.OptionRoyalty = x->OptionRoyalty;
    y.RealOptionValueOffset = x->RealOptionValueOffset;
    y.Margin = x->Margin;
    y.ExchMargin = x->ExchMargin;
    return y;
}

CThostFtdcQryInvestorProdSPBMDetailField Converter::QryInvestorProdSPBMDetailFieldToCpp(QryInvestorProdSPBMDetailField x) {
    CThostFtdcQryInvestorProdSPBMDetailField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    return y;
}

QryInvestorProdSPBMDetailField Converter::CThostFtdcQryInvestorProdSPBMDetailFieldToRust(CThostFtdcQryInvestorProdSPBMDetailField* x) {
    if (x == nullptr)
        return QryInvestorProdSPBMDetailField{.is_null = true};
    QryInvestorProdSPBMDetailField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    return y;
}

CThostFtdcPortfTradeParamSettingField Converter::PortfTradeParamSettingFieldToCpp(PortfTradeParamSettingField x) {
    CThostFtdcPortfTradeParamSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.Portfolio = x.Portfolio;
    y.IsActionVerify = x.IsActionVerify;
    y.IsCloseVerify = x.IsCloseVerify;
    return y;
}

PortfTradeParamSettingField Converter::CThostFtdcPortfTradeParamSettingFieldToRust(CThostFtdcPortfTradeParamSettingField* x) {
    if (x == nullptr)
        return PortfTradeParamSettingField{.is_null = true};
    PortfTradeParamSettingField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.Portfolio = x->Portfolio;
    y.IsActionVerify = x->IsActionVerify;
    y.IsCloseVerify = x->IsCloseVerify;
    return y;
}

CThostFtdcInvestorTradingRightField Converter::InvestorTradingRightFieldToCpp(InvestorTradingRightField x) {
    CThostFtdcInvestorTradingRightField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.InvstTradingRight = x.InvstTradingRight;
    return y;
}

InvestorTradingRightField Converter::CThostFtdcInvestorTradingRightFieldToRust(CThostFtdcInvestorTradingRightField* x) {
    if (x == nullptr)
        return InvestorTradingRightField{.is_null = true};
    InvestorTradingRightField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InvstTradingRight = x->InvstTradingRight;
    return y;
}

CThostFtdcMortgageParamField Converter::MortgageParamFieldToCpp(MortgageParamField x) {
    CThostFtdcMortgageParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.MortgageBalance = x.MortgageBalance;
    y.CheckMortgageRatio = x.CheckMortgageRatio;
    return y;
}

MortgageParamField Converter::CThostFtdcMortgageParamFieldToRust(CThostFtdcMortgageParamField* x) {
    if (x == nullptr)
        return MortgageParamField{.is_null = true};
    MortgageParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.MortgageBalance = x->MortgageBalance;
    y.CheckMortgageRatio = x->CheckMortgageRatio;
    return y;
}

CThostFtdcWithDrawParamField Converter::WithDrawParamFieldToCpp(WithDrawParamField x) {
    CThostFtdcWithDrawParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    y.WithDrawParamID = x.WithDrawParamID;
    memcpy(y.WithDrawParamValue, x.WithDrawParamValue.data(), x.WithDrawParamValue.size() * sizeof(uint8_t));
    return y;
}

WithDrawParamField Converter::CThostFtdcWithDrawParamFieldToRust(CThostFtdcWithDrawParamField* x) {
    if (x == nullptr)
        return WithDrawParamField{.is_null = true};
    WithDrawParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.WithDrawParamID = x->WithDrawParamID;
    for (int i = 0; i < 41; i++)
        y.WithDrawParamValue.push_back(x->WithDrawParamValue[i]);
    return y;
}

CThostFtdcThostUserFunctionField Converter::ThostUserFunctionFieldToCpp(ThostUserFunctionField x) {
    CThostFtdcThostUserFunctionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.ThostFunctionCode = x.ThostFunctionCode;
    return y;
}

ThostUserFunctionField Converter::CThostFtdcThostUserFunctionFieldToRust(CThostFtdcThostUserFunctionField* x) {
    if (x == nullptr)
        return ThostUserFunctionField{.is_null = true};
    ThostUserFunctionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ThostFunctionCode = x->ThostFunctionCode;
    return y;
}

CThostFtdcQryThostUserFunctionField Converter::QryThostUserFunctionFieldToCpp(QryThostUserFunctionField x) {
    CThostFtdcQryThostUserFunctionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

QryThostUserFunctionField Converter::CThostFtdcQryThostUserFunctionFieldToRust(CThostFtdcQryThostUserFunctionField* x) {
    if (x == nullptr)
        return QryThostUserFunctionField{.is_null = true};
    QryThostUserFunctionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcSPBMAddOnInterParameterField Converter::SPBMAddOnInterParameterFieldToCpp(SPBMAddOnInterParameterField x) {
    CThostFtdcSPBMAddOnInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SpreadId = x.SpreadId;
    y.AddOnInterRateZ2 = x.AddOnInterRateZ2;
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    return y;
}

SPBMAddOnInterParameterField Converter::CThostFtdcSPBMAddOnInterParameterFieldToRust(CThostFtdcSPBMAddOnInterParameterField* x) {
    if (x == nullptr)
        return SPBMAddOnInterParameterField{.is_null = true};
    SPBMAddOnInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SpreadId = x->SpreadId;
    y.AddOnInterRateZ2 = x->AddOnInterRateZ2;
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    return y;
}

CThostFtdcQrySPBMAddOnInterParameterField Converter::QrySPBMAddOnInterParameterFieldToCpp(QrySPBMAddOnInterParameterField x) {
    CThostFtdcQrySPBMAddOnInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    return y;
}

QrySPBMAddOnInterParameterField Converter::CThostFtdcQrySPBMAddOnInterParameterFieldToRust(CThostFtdcQrySPBMAddOnInterParameterField* x) {
    if (x == nullptr)
        return QrySPBMAddOnInterParameterField{.is_null = true};
    QrySPBMAddOnInterParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    return y;
}

CThostFtdcQryInvestorCommoditySPMMMarginField Converter::QryInvestorCommoditySPMMMarginFieldToCpp(QryInvestorCommoditySPMMMarginField x) {
    CThostFtdcQryInvestorCommoditySPMMMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CommodityID, x.CommodityID.c_str());
    return y;
}

QryInvestorCommoditySPMMMarginField Converter::CThostFtdcQryInvestorCommoditySPMMMarginFieldToRust(CThostFtdcQryInvestorCommoditySPMMMarginField* x) {
    if (x == nullptr)
        return QryInvestorCommoditySPMMMarginField{.is_null = true};
    QryInvestorCommoditySPMMMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CommodityID = Converter::Gb2312ToRustString(x->CommodityID);
    return y;
}

CThostFtdcQryInvestorCommodityGroupSPMMMarginField Converter::QryInvestorCommodityGroupSPMMMarginFieldToCpp(QryInvestorCommodityGroupSPMMMarginField x) {
    CThostFtdcQryInvestorCommodityGroupSPMMMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    return y;
}

QryInvestorCommodityGroupSPMMMarginField Converter::CThostFtdcQryInvestorCommodityGroupSPMMMarginFieldToRust(CThostFtdcQryInvestorCommodityGroupSPMMMarginField* x) {
    if (x == nullptr)
        return QryInvestorCommodityGroupSPMMMarginField{.is_null = true};
    QryInvestorCommodityGroupSPMMMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    return y;
}

CThostFtdcQrySPMMInstParamField Converter::QrySPMMInstParamFieldToCpp(QrySPMMInstParamField x) {
    CThostFtdcQrySPMMInstParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QrySPMMInstParamField Converter::CThostFtdcQrySPMMInstParamFieldToRust(CThostFtdcQrySPMMInstParamField* x) {
    if (x == nullptr)
        return QrySPMMInstParamField{.is_null = true};
    QrySPMMInstParamField y{};
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQrySPMMProductParamField Converter::QrySPMMProductParamFieldToCpp(QrySPMMProductParamField x) {
    CThostFtdcQrySPMMProductParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QrySPMMProductParamField Converter::CThostFtdcQrySPMMProductParamFieldToRust(CThostFtdcQrySPMMProductParamField* x) {
    if (x == nullptr)
        return QrySPMMProductParamField{.is_null = true};
    QrySPMMProductParamField y{};
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcInvestorCommoditySPMMMarginField Converter::InvestorCommoditySPMMMarginFieldToCpp(InvestorCommoditySPMMMarginField x) {
    CThostFtdcInvestorCommoditySPMMMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CommodityID, x.CommodityID.c_str());
    y.MarginBeforeDiscount = x.MarginBeforeDiscount;
    y.MarginNoDiscount = x.MarginNoDiscount;
    y.LongPosRisk = x.LongPosRisk;
    y.LongOpenFrozenRisk = x.LongOpenFrozenRisk;
    y.LongCloseFrozenRisk = x.LongCloseFrozenRisk;
    y.ShortPosRisk = x.ShortPosRisk;
    y.ShortOpenFrozenRisk = x.ShortOpenFrozenRisk;
    y.ShortCloseFrozenRisk = x.ShortCloseFrozenRisk;
    y.IntraCommodityRate = x.IntraCommodityRate;
    y.OptionDiscountRate = x.OptionDiscountRate;
    y.PosDiscount = x.PosDiscount;
    y.OpenFrozenDiscount = x.OpenFrozenDiscount;
    y.NetRisk = x.NetRisk;
    y.CloseFrozenMargin = x.CloseFrozenMargin;
    y.FrozenCommission = x.FrozenCommission;
    y.Commission = x.Commission;
    y.FrozenCash = x.FrozenCash;
    y.CashIn = x.CashIn;
    y.StrikeFrozenMargin = x.StrikeFrozenMargin;
    return y;
}

InvestorCommoditySPMMMarginField Converter::CThostFtdcInvestorCommoditySPMMMarginFieldToRust(CThostFtdcInvestorCommoditySPMMMarginField* x) {
    if (x == nullptr)
        return InvestorCommoditySPMMMarginField{.is_null = true};
    InvestorCommoditySPMMMarginField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CommodityID = Converter::Gb2312ToRustString(x->CommodityID);
    y.MarginBeforeDiscount = x->MarginBeforeDiscount;
    y.MarginNoDiscount = x->MarginNoDiscount;
    y.LongPosRisk = x->LongPosRisk;
    y.LongOpenFrozenRisk = x->LongOpenFrozenRisk;
    y.LongCloseFrozenRisk = x->LongCloseFrozenRisk;
    y.ShortPosRisk = x->ShortPosRisk;
    y.ShortOpenFrozenRisk = x->ShortOpenFrozenRisk;
    y.ShortCloseFrozenRisk = x->ShortCloseFrozenRisk;
    y.IntraCommodityRate = x->IntraCommodityRate;
    y.OptionDiscountRate = x->OptionDiscountRate;
    y.PosDiscount = x->PosDiscount;
    y.OpenFrozenDiscount = x->OpenFrozenDiscount;
    y.NetRisk = x->NetRisk;
    y.CloseFrozenMargin = x->CloseFrozenMargin;
    y.FrozenCommission = x->FrozenCommission;
    y.Commission = x->Commission;
    y.FrozenCash = x->FrozenCash;
    y.CashIn = x->CashIn;
    y.StrikeFrozenMargin = x->StrikeFrozenMargin;
    return y;
}

CThostFtdcInvestorCommodityGroupSPMMMarginField Converter::InvestorCommodityGroupSPMMMarginFieldToCpp(InvestorCommodityGroupSPMMMarginField x) {
    CThostFtdcInvestorCommodityGroupSPMMMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    y.MarginBeforeDiscount = x.MarginBeforeDiscount;
    y.MarginNoDiscount = x.MarginNoDiscount;
    y.LongRisk = x.LongRisk;
    y.ShortRisk = x.ShortRisk;
    y.CloseFrozenMargin = x.CloseFrozenMargin;
    y.InterCommodityRate = x.InterCommodityRate;
    y.MiniMarginRatio = x.MiniMarginRatio;
    y.AdjustRatio = x.AdjustRatio;
    y.IntraCommodityDiscount = x.IntraCommodityDiscount;
    y.InterCommodityDiscount = x.InterCommodityDiscount;
    y.ExchMargin = x.ExchMargin;
    y.InvestorMargin = x.InvestorMargin;
    y.FrozenCommission = x.FrozenCommission;
    y.Commission = x.Commission;
    y.FrozenCash = x.FrozenCash;
    y.CashIn = x.CashIn;
    y.StrikeFrozenMargin = x.StrikeFrozenMargin;
    return y;
}

InvestorCommodityGroupSPMMMarginField Converter::CThostFtdcInvestorCommodityGroupSPMMMarginFieldToRust(CThostFtdcInvestorCommodityGroupSPMMMarginField* x) {
    if (x == nullptr)
        return InvestorCommodityGroupSPMMMarginField{.is_null = true};
    InvestorCommodityGroupSPMMMarginField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    y.MarginBeforeDiscount = x->MarginBeforeDiscount;
    y.MarginNoDiscount = x->MarginNoDiscount;
    y.LongRisk = x->LongRisk;
    y.ShortRisk = x->ShortRisk;
    y.CloseFrozenMargin = x->CloseFrozenMargin;
    y.InterCommodityRate = x->InterCommodityRate;
    y.MiniMarginRatio = x->MiniMarginRatio;
    y.AdjustRatio = x->AdjustRatio;
    y.IntraCommodityDiscount = x->IntraCommodityDiscount;
    y.InterCommodityDiscount = x->InterCommodityDiscount;
    y.ExchMargin = x->ExchMargin;
    y.InvestorMargin = x->InvestorMargin;
    y.FrozenCommission = x->FrozenCommission;
    y.Commission = x->Commission;
    y.FrozenCash = x->FrozenCash;
    y.CashIn = x->CashIn;
    y.StrikeFrozenMargin = x->StrikeFrozenMargin;
    return y;
}

CThostFtdcSPMMInstParamField Converter::SPMMInstParamFieldToCpp(SPMMInstParamField x) {
    CThostFtdcSPMMInstParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InstMarginCalID = x.InstMarginCalID;
    strcpy(y.CommodityID, x.CommodityID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    return y;
}

SPMMInstParamField Converter::CThostFtdcSPMMInstParamFieldToRust(CThostFtdcSPMMInstParamField* x) {
    if (x == nullptr)
        return SPMMInstParamField{.is_null = true};
    SPMMInstParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InstMarginCalID = x->InstMarginCalID;
    y.CommodityID = Converter::Gb2312ToRustString(x->CommodityID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    return y;
}

CThostFtdcSPMMProductParamField Converter::SPMMProductParamFieldToCpp(SPMMProductParamField x) {
    CThostFtdcSPMMProductParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.CommodityID, x.CommodityID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    return y;
}

SPMMProductParamField Converter::CThostFtdcSPMMProductParamFieldToRust(CThostFtdcSPMMProductParamField* x) {
    if (x == nullptr)
        return SPMMProductParamField{.is_null = true};
    SPMMProductParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.CommodityID = Converter::Gb2312ToRustString(x->CommodityID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    return y;
}

CThostFtdcQryTraderAssignField Converter::QryTraderAssignFieldToCpp(QryTraderAssignField x) {
    CThostFtdcQryTraderAssignField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TraderID, x.TraderID.c_str());
    return y;
}

QryTraderAssignField Converter::CThostFtdcQryTraderAssignFieldToRust(CThostFtdcQryTraderAssignField* x) {
    if (x == nullptr)
        return QryTraderAssignField{.is_null = true};
    QryTraderAssignField y{};
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    return y;
}

CThostFtdcTraderAssignField Converter::TraderAssignFieldToCpp(TraderAssignField x) {
    CThostFtdcTraderAssignField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    y.DRIdentityID = x.DRIdentityID;
    return y;
}

TraderAssignField Converter::CThostFtdcTraderAssignFieldToRust(CThostFtdcTraderAssignField* x) {
    if (x == nullptr)
        return TraderAssignField{.is_null = true};
    TraderAssignField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.DRIdentityID = x->DRIdentityID;
    return y;
}

CThostFtdcInvestorInfoCntSettingField Converter::InvestorInfoCntSettingFieldToCpp(InvestorInfoCntSettingField x) {
    CThostFtdcInvestorInfoCntSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.IsCalInfoComm = x.IsCalInfoComm;
    y.IsLimitInfoMax = x.IsLimitInfoMax;
    y.InfoMaxLimit = x.InfoMaxLimit;
    return y;
}

InvestorInfoCntSettingField Converter::CThostFtdcInvestorInfoCntSettingFieldToRust(CThostFtdcInvestorInfoCntSettingField* x) {
    if (x == nullptr)
        return InvestorInfoCntSettingField{.is_null = true};
    InvestorInfoCntSettingField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.IsCalInfoComm = x->IsCalInfoComm;
    y.IsLimitInfoMax = x->IsLimitInfoMax;
    y.InfoMaxLimit = x->InfoMaxLimit;
    return y;
}

CThostFtdcRCAMSCombProductInfoField Converter::RCAMSCombProductInfoFieldToCpp(RCAMSCombProductInfoField x) {
    CThostFtdcRCAMSCombProductInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

RCAMSCombProductInfoField Converter::CThostFtdcRCAMSCombProductInfoFieldToRust(CThostFtdcRCAMSCombProductInfoField* x) {
    if (x == nullptr)
        return RCAMSCombProductInfoField{.is_null = true};
    RCAMSCombProductInfoField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcRCAMSInstrParameterField Converter::RCAMSInstrParameterFieldToCpp(RCAMSInstrParameterField x) {
    CThostFtdcRCAMSInstrParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.HedgeRate = x.HedgeRate;
    return y;
}

RCAMSInstrParameterField Converter::CThostFtdcRCAMSInstrParameterFieldToRust(CThostFtdcRCAMSInstrParameterField* x) {
    if (x == nullptr)
        return RCAMSInstrParameterField{.is_null = true};
    RCAMSInstrParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.HedgeRate = x->HedgeRate;
    return y;
}

CThostFtdcRCAMSIntraParameterField Converter::RCAMSIntraParameterFieldToCpp(RCAMSIntraParameterField x) {
    CThostFtdcRCAMSIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    y.HedgeRate = x.HedgeRate;
    return y;
}

RCAMSIntraParameterField Converter::CThostFtdcRCAMSIntraParameterFieldToRust(CThostFtdcRCAMSIntraParameterField* x) {
    if (x == nullptr)
        return RCAMSIntraParameterField{.is_null = true};
    RCAMSIntraParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.HedgeRate = x->HedgeRate;
    return y;
}

CThostFtdcRCAMSInterParameterField Converter::RCAMSInterParameterFieldToCpp(RCAMSInterParameterField x) {
    CThostFtdcRCAMSInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    y.Priority = x.Priority;
    y.CreditRate = x.CreditRate;
    strcpy(y.CombProduct1, x.CombProduct1.c_str());
    strcpy(y.CombProduct2, x.CombProduct2.c_str());
    return y;
}

RCAMSInterParameterField Converter::CThostFtdcRCAMSInterParameterFieldToRust(CThostFtdcRCAMSInterParameterField* x) {
    if (x == nullptr)
        return RCAMSInterParameterField{.is_null = true};
    RCAMSInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    y.Priority = x->Priority;
    y.CreditRate = x->CreditRate;
    y.CombProduct1 = Converter::Gb2312ToRustString(x->CombProduct1);
    y.CombProduct2 = Converter::Gb2312ToRustString(x->CombProduct2);
    return y;
}

CThostFtdcRCAMSShortOptAdjustParamField Converter::RCAMSShortOptAdjustParamFieldToCpp(RCAMSShortOptAdjustParamField x) {
    CThostFtdcRCAMSShortOptAdjustParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.AdjustValue = x.AdjustValue;
    return y;
}

RCAMSShortOptAdjustParamField Converter::CThostFtdcRCAMSShortOptAdjustParamFieldToRust(CThostFtdcRCAMSShortOptAdjustParamField* x) {
    if (x == nullptr)
        return RCAMSShortOptAdjustParamField{.is_null = true};
    RCAMSShortOptAdjustParamField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.HedgeFlag = x->HedgeFlag;
    y.AdjustValue = x->AdjustValue;
    return y;
}

CThostFtdcRCAMSInvestorCombPositionField Converter::RCAMSInvestorCombPositionFieldToCpp(RCAMSInvestorCombPositionField x) {
    CThostFtdcRCAMSInvestorCombPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.PosiDirection = x.PosiDirection;
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    y.LegID = x.LegID;
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    y.TotalAmt = x.TotalAmt;
    y.ExchMargin = x.ExchMargin;
    y.Margin = x.Margin;
    return y;
}

RCAMSInvestorCombPositionField Converter::CThostFtdcRCAMSInvestorCombPositionFieldToRust(CThostFtdcRCAMSInvestorCombPositionField* x) {
    if (x == nullptr)
        return RCAMSInvestorCombPositionField{.is_null = true};
    RCAMSInvestorCombPositionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.HedgeFlag = x->HedgeFlag;
    y.PosiDirection = x->PosiDirection;
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.LegID = x->LegID;
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.TotalAmt = x->TotalAmt;
    y.ExchMargin = x->ExchMargin;
    y.Margin = x->Margin;
    return y;
}

CThostFtdcInvestorProdRCAMSMarginField Converter::InvestorProdRCAMSMarginFieldToCpp(InvestorProdRCAMSMarginField x) {
    CThostFtdcInvestorProdRCAMSMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    y.RiskBeforeDiscount = x.RiskBeforeDiscount;
    y.IntraInstrRisk = x.IntraInstrRisk;
    y.BPosRisk = x.BPosRisk;
    y.SPosRisk = x.SPosRisk;
    y.IntraProdRisk = x.IntraProdRisk;
    y.NetRisk = x.NetRisk;
    y.InterProdRisk = x.InterProdRisk;
    y.ShortOptRiskAdj = x.ShortOptRiskAdj;
    y.OptionRoyalty = x.OptionRoyalty;
    y.MMSACloseFrozenMargin = x.MMSACloseFrozenMargin;
    y.CloseCombFrozenMargin = x.CloseCombFrozenMargin;
    y.CloseFrozenMargin = x.CloseFrozenMargin;
    y.MMSAOpenFrozenMargin = x.MMSAOpenFrozenMargin;
    y.DeliveryOpenFrozenMargin = x.DeliveryOpenFrozenMargin;
    y.OpenFrozenMargin = x.OpenFrozenMargin;
    y.UseFrozenMargin = x.UseFrozenMargin;
    y.MMSAExchMargin = x.MMSAExchMargin;
    y.DeliveryExchMargin = x.DeliveryExchMargin;
    y.CombExchMargin = x.CombExchMargin;
    y.ExchMargin = x.ExchMargin;
    y.UseMargin = x.UseMargin;
    return y;
}

InvestorProdRCAMSMarginField Converter::CThostFtdcInvestorProdRCAMSMarginFieldToRust(CThostFtdcInvestorProdRCAMSMarginField* x) {
    if (x == nullptr)
        return InvestorProdRCAMSMarginField{.is_null = true};
    InvestorProdRCAMSMarginField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.HedgeFlag = x->HedgeFlag;
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    y.RiskBeforeDiscount = x->RiskBeforeDiscount;
    y.IntraInstrRisk = x->IntraInstrRisk;
    y.BPosRisk = x->BPosRisk;
    y.SPosRisk = x->SPosRisk;
    y.IntraProdRisk = x->IntraProdRisk;
    y.NetRisk = x->NetRisk;
    y.InterProdRisk = x->InterProdRisk;
    y.ShortOptRiskAdj = x->ShortOptRiskAdj;
    y.OptionRoyalty = x->OptionRoyalty;
    y.MMSACloseFrozenMargin = x->MMSACloseFrozenMargin;
    y.CloseCombFrozenMargin = x->CloseCombFrozenMargin;
    y.CloseFrozenMargin = x->CloseFrozenMargin;
    y.MMSAOpenFrozenMargin = x->MMSAOpenFrozenMargin;
    y.DeliveryOpenFrozenMargin = x->DeliveryOpenFrozenMargin;
    y.OpenFrozenMargin = x->OpenFrozenMargin;
    y.UseFrozenMargin = x->UseFrozenMargin;
    y.MMSAExchMargin = x->MMSAExchMargin;
    y.DeliveryExchMargin = x->DeliveryExchMargin;
    y.CombExchMargin = x->CombExchMargin;
    y.ExchMargin = x->ExchMargin;
    y.UseMargin = x->UseMargin;
    return y;
}

CThostFtdcQryRCAMSCombProductInfoField Converter::QryRCAMSCombProductInfoFieldToCpp(QryRCAMSCombProductInfoField x) {
    CThostFtdcQryRCAMSCombProductInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

QryRCAMSCombProductInfoField Converter::CThostFtdcQryRCAMSCombProductInfoFieldToRust(CThostFtdcQryRCAMSCombProductInfoField* x) {
    if (x == nullptr)
        return QryRCAMSCombProductInfoField{.is_null = true};
    QryRCAMSCombProductInfoField y{};
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcQryRCAMSInstrParameterField Converter::QryRCAMSInstrParameterFieldToCpp(QryRCAMSInstrParameterField x) {
    CThostFtdcQryRCAMSInstrParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductID, x.ProductID.c_str());
    return y;
}

QryRCAMSInstrParameterField Converter::CThostFtdcQryRCAMSInstrParameterFieldToRust(CThostFtdcQryRCAMSInstrParameterField* x) {
    if (x == nullptr)
        return QryRCAMSInstrParameterField{.is_null = true};
    QryRCAMSInstrParameterField y{};
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    return y;
}

CThostFtdcQryRCAMSIntraParameterField Converter::QryRCAMSIntraParameterFieldToCpp(QryRCAMSIntraParameterField x) {
    CThostFtdcQryRCAMSIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CombProductID, x.CombProductID.c_str());
    return y;
}

QryRCAMSIntraParameterField Converter::CThostFtdcQryRCAMSIntraParameterFieldToRust(CThostFtdcQryRCAMSIntraParameterField* x) {
    if (x == nullptr)
        return QryRCAMSIntraParameterField{.is_null = true};
    QryRCAMSIntraParameterField y{};
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    return y;
}

CThostFtdcQryRCAMSInterParameterField Converter::QryRCAMSInterParameterFieldToCpp(QryRCAMSInterParameterField x) {
    CThostFtdcQryRCAMSInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    strcpy(y.CombProduct1, x.CombProduct1.c_str());
    strcpy(y.CombProduct2, x.CombProduct2.c_str());
    return y;
}

QryRCAMSInterParameterField Converter::CThostFtdcQryRCAMSInterParameterFieldToRust(CThostFtdcQryRCAMSInterParameterField* x) {
    if (x == nullptr)
        return QryRCAMSInterParameterField{.is_null = true};
    QryRCAMSInterParameterField y{};
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    y.CombProduct1 = Converter::Gb2312ToRustString(x->CombProduct1);
    y.CombProduct2 = Converter::Gb2312ToRustString(x->CombProduct2);
    return y;
}

CThostFtdcQryRCAMSShortOptAdjustParamField Converter::QryRCAMSShortOptAdjustParamFieldToCpp(QryRCAMSShortOptAdjustParamField x) {
    CThostFtdcQryRCAMSShortOptAdjustParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CombProductID, x.CombProductID.c_str());
    return y;
}

QryRCAMSShortOptAdjustParamField Converter::CThostFtdcQryRCAMSShortOptAdjustParamFieldToRust(CThostFtdcQryRCAMSShortOptAdjustParamField* x) {
    if (x == nullptr)
        return QryRCAMSShortOptAdjustParamField{.is_null = true};
    QryRCAMSShortOptAdjustParamField y{};
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    return y;
}

CThostFtdcQryRCAMSInvestorCombPositionField Converter::QryRCAMSInvestorCombPositionFieldToCpp(QryRCAMSInvestorCombPositionField x) {
    CThostFtdcQryRCAMSInvestorCombPositionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    return y;
}

QryRCAMSInvestorCombPositionField Converter::CThostFtdcQryRCAMSInvestorCombPositionFieldToRust(CThostFtdcQryRCAMSInvestorCombPositionField* x) {
    if (x == nullptr)
        return QryRCAMSInvestorCombPositionField{.is_null = true};
    QryRCAMSInvestorCombPositionField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    return y;
}

CThostFtdcQryInvestorProdRCAMSMarginField Converter::QryInvestorProdRCAMSMarginFieldToCpp(QryInvestorProdRCAMSMarginField x) {
    CThostFtdcQryInvestorProdRCAMSMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    return y;
}

QryInvestorProdRCAMSMarginField Converter::CThostFtdcQryInvestorProdRCAMSMarginFieldToRust(CThostFtdcQryInvestorProdRCAMSMarginField* x) {
    if (x == nullptr)
        return QryInvestorProdRCAMSMarginField{.is_null = true};
    QryInvestorProdRCAMSMarginField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    return y;
}

CThostFtdcRULEInstrParameterField Converter::RULEInstrParameterFieldToCpp(RULEInstrParameterField x) {
    CThostFtdcRULEInstrParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InstrumentClass = x.InstrumentClass;
    strcpy(y.StdInstrumentID, x.StdInstrumentID.c_str());
    y.BSpecRatio = x.BSpecRatio;
    y.SSpecRatio = x.SSpecRatio;
    y.BHedgeRatio = x.BHedgeRatio;
    y.SHedgeRatio = x.SHedgeRatio;
    y.BAddOnMargin = x.BAddOnMargin;
    y.SAddOnMargin = x.SAddOnMargin;
    y.CommodityGroupID = x.CommodityGroupID;
    return y;
}

RULEInstrParameterField Converter::CThostFtdcRULEInstrParameterFieldToRust(CThostFtdcRULEInstrParameterField* x) {
    if (x == nullptr)
        return RULEInstrParameterField{.is_null = true};
    RULEInstrParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InstrumentClass = x->InstrumentClass;
    y.StdInstrumentID = Converter::Gb2312ToRustString(x->StdInstrumentID);
    y.BSpecRatio = x->BSpecRatio;
    y.SSpecRatio = x->SSpecRatio;
    y.BHedgeRatio = x->BHedgeRatio;
    y.SHedgeRatio = x->SHedgeRatio;
    y.BAddOnMargin = x->BAddOnMargin;
    y.SAddOnMargin = x->SAddOnMargin;
    y.CommodityGroupID = x->CommodityGroupID;
    return y;
}

CThostFtdcRULEIntraParameterField Converter::RULEIntraParameterFieldToCpp(RULEIntraParameterField x) {
    CThostFtdcRULEIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    strcpy(y.StdInstrumentID, x.StdInstrumentID.c_str());
    y.StdInstrMargin = x.StdInstrMargin;
    y.UsualIntraRate = x.UsualIntraRate;
    y.DeliveryIntraRate = x.DeliveryIntraRate;
    return y;
}

RULEIntraParameterField Converter::CThostFtdcRULEIntraParameterFieldToRust(CThostFtdcRULEIntraParameterField* x) {
    if (x == nullptr)
        return RULEIntraParameterField{.is_null = true};
    RULEIntraParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.StdInstrumentID = Converter::Gb2312ToRustString(x->StdInstrumentID);
    y.StdInstrMargin = x->StdInstrMargin;
    y.UsualIntraRate = x->UsualIntraRate;
    y.DeliveryIntraRate = x->DeliveryIntraRate;
    return y;
}

CThostFtdcRULEInterParameterField Converter::RULEInterParameterFieldToCpp(RULEInterParameterField x) {
    CThostFtdcRULEInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SpreadId = x.SpreadId;
    y.InterRate = x.InterRate;
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    y.Leg1PropFactor = x.Leg1PropFactor;
    y.Leg2PropFactor = x.Leg2PropFactor;
    y.CommodityGroupID = x.CommodityGroupID;
    strcpy(y.CommodityGroupName, x.CommodityGroupName.c_str());
    return y;
}

RULEInterParameterField Converter::CThostFtdcRULEInterParameterFieldToRust(CThostFtdcRULEInterParameterField* x) {
    if (x == nullptr)
        return RULEInterParameterField{.is_null = true};
    RULEInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SpreadId = x->SpreadId;
    y.InterRate = x->InterRate;
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    y.Leg1PropFactor = x->Leg1PropFactor;
    y.Leg2PropFactor = x->Leg2PropFactor;
    y.CommodityGroupID = x->CommodityGroupID;
    y.CommodityGroupName = Converter::Gb2312ToRustString(x->CommodityGroupName);
    return y;
}

CThostFtdcQryRULEInstrParameterField Converter::QryRULEInstrParameterFieldToCpp(QryRULEInstrParameterField x) {
    CThostFtdcQryRULEInstrParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    return y;
}

QryRULEInstrParameterField Converter::CThostFtdcQryRULEInstrParameterFieldToRust(CThostFtdcQryRULEInstrParameterField* x) {
    if (x == nullptr)
        return QryRULEInstrParameterField{.is_null = true};
    QryRULEInstrParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    return y;
}

CThostFtdcQryRULEIntraParameterField Converter::QryRULEIntraParameterFieldToCpp(QryRULEIntraParameterField x) {
    CThostFtdcQryRULEIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    return y;
}

QryRULEIntraParameterField Converter::CThostFtdcQryRULEIntraParameterFieldToRust(CThostFtdcQryRULEIntraParameterField* x) {
    if (x == nullptr)
        return QryRULEIntraParameterField{.is_null = true};
    QryRULEIntraParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    return y;
}

CThostFtdcQryRULEInterParameterField Converter::QryRULEInterParameterFieldToCpp(QryRULEInterParameterField x) {
    CThostFtdcQryRULEInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    y.CommodityGroupID = x.CommodityGroupID;
    return y;
}

QryRULEInterParameterField Converter::CThostFtdcQryRULEInterParameterFieldToRust(CThostFtdcQryRULEInterParameterField* x) {
    if (x == nullptr)
        return QryRULEInterParameterField{.is_null = true};
    QryRULEInterParameterField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    y.CommodityGroupID = x->CommodityGroupID;
    return y;
}

CThostFtdcInvestorProdRULEMarginField Converter::InvestorProdRULEMarginFieldToCpp(InvestorProdRULEMarginField x) {
    CThostFtdcInvestorProdRULEMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.InstrumentClass = x.InstrumentClass;
    y.CommodityGroupID = x.CommodityGroupID;
    y.BStdPosition = x.BStdPosition;
    y.SStdPosition = x.SStdPosition;
    y.BStdOpenFrozen = x.BStdOpenFrozen;
    y.SStdOpenFrozen = x.SStdOpenFrozen;
    y.BStdCloseFrozen = x.BStdCloseFrozen;
    y.SStdCloseFrozen = x.SStdCloseFrozen;
    y.IntraProdStdPosition = x.IntraProdStdPosition;
    y.NetStdPosition = x.NetStdPosition;
    y.InterProdStdPosition = x.InterProdStdPosition;
    y.SingleStdPosition = x.SingleStdPosition;
    y.IntraProdMargin = x.IntraProdMargin;
    y.InterProdMargin = x.InterProdMargin;
    y.SingleMargin = x.SingleMargin;
    y.NonCombMargin = x.NonCombMargin;
    y.AddOnMargin = x.AddOnMargin;
    y.ExchMargin = x.ExchMargin;
    y.AddOnFrozenMargin = x.AddOnFrozenMargin;
    y.OpenFrozenMargin = x.OpenFrozenMargin;
    y.CloseFrozenMargin = x.CloseFrozenMargin;
    y.Margin = x.Margin;
    y.FrozenMargin = x.FrozenMargin;
    return y;
}

InvestorProdRULEMarginField Converter::CThostFtdcInvestorProdRULEMarginFieldToRust(CThostFtdcInvestorProdRULEMarginField* x) {
    if (x == nullptr)
        return InvestorProdRULEMarginField{.is_null = true};
    InvestorProdRULEMarginField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.InstrumentClass = x->InstrumentClass;
    y.CommodityGroupID = x->CommodityGroupID;
    y.BStdPosition = x->BStdPosition;
    y.SStdPosition = x->SStdPosition;
    y.BStdOpenFrozen = x->BStdOpenFrozen;
    y.SStdOpenFrozen = x->SStdOpenFrozen;
    y.BStdCloseFrozen = x->BStdCloseFrozen;
    y.SStdCloseFrozen = x->SStdCloseFrozen;
    y.IntraProdStdPosition = x->IntraProdStdPosition;
    y.NetStdPosition = x->NetStdPosition;
    y.InterProdStdPosition = x->InterProdStdPosition;
    y.SingleStdPosition = x->SingleStdPosition;
    y.IntraProdMargin = x->IntraProdMargin;
    y.InterProdMargin = x->InterProdMargin;
    y.SingleMargin = x->SingleMargin;
    y.NonCombMargin = x->NonCombMargin;
    y.AddOnMargin = x->AddOnMargin;
    y.ExchMargin = x->ExchMargin;
    y.AddOnFrozenMargin = x->AddOnFrozenMargin;
    y.OpenFrozenMargin = x->OpenFrozenMargin;
    y.CloseFrozenMargin = x->CloseFrozenMargin;
    y.Margin = x->Margin;
    y.FrozenMargin = x->FrozenMargin;
    return y;
}

CThostFtdcQryInvestorProdRULEMarginField Converter::QryInvestorProdRULEMarginFieldToCpp(QryInvestorProdRULEMarginField x) {
    CThostFtdcQryInvestorProdRULEMarginField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.CommodityGroupID = x.CommodityGroupID;
    return y;
}

QryInvestorProdRULEMarginField Converter::CThostFtdcQryInvestorProdRULEMarginFieldToRust(CThostFtdcQryInvestorProdRULEMarginField* x) {
    if (x == nullptr)
        return QryInvestorProdRULEMarginField{.is_null = true};
    QryInvestorProdRULEMarginField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.CommodityGroupID = x->CommodityGroupID;
    return y;
}

CThostFtdcSyncDeltaSPBMPortfDefinitionField Converter::SyncDeltaSPBMPortfDefinitionFieldToCpp(SyncDeltaSPBMPortfDefinitionField x) {
    CThostFtdcSyncDeltaSPBMPortfDefinitionField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.PortfolioDefID = x.PortfolioDefID;
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.IsSPBM = x.IsSPBM;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMPortfDefinitionField Converter::CThostFtdcSyncDeltaSPBMPortfDefinitionFieldToRust(CThostFtdcSyncDeltaSPBMPortfDefinitionField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMPortfDefinitionField{.is_null = true};
    SyncDeltaSPBMPortfDefinitionField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.PortfolioDefID = x->PortfolioDefID;
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.IsSPBM = x->IsSPBM;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPBMInvstPortfDefField Converter::SyncDeltaSPBMInvstPortfDefFieldToCpp(SyncDeltaSPBMInvstPortfDefField x) {
    CThostFtdcSyncDeltaSPBMInvstPortfDefField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.PortfolioDefID = x.PortfolioDefID;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMInvstPortfDefField Converter::CThostFtdcSyncDeltaSPBMInvstPortfDefFieldToRust(CThostFtdcSyncDeltaSPBMInvstPortfDefField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMInvstPortfDefField{.is_null = true};
    SyncDeltaSPBMInvstPortfDefField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.PortfolioDefID = x->PortfolioDefID;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPBMFutureParameterField Converter::SyncDeltaSPBMFutureParameterFieldToCpp(SyncDeltaSPBMFutureParameterField x) {
    CThostFtdcSyncDeltaSPBMFutureParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.Cvf = x.Cvf;
    y.TimeRange = x.TimeRange;
    y.MarginRate = x.MarginRate;
    y.LockRateX = x.LockRateX;
    y.AddOnRate = x.AddOnRate;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.AddOnLockRateX2 = x.AddOnLockRateX2;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMFutureParameterField Converter::CThostFtdcSyncDeltaSPBMFutureParameterFieldToRust(CThostFtdcSyncDeltaSPBMFutureParameterField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMFutureParameterField{.is_null = true};
    SyncDeltaSPBMFutureParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.Cvf = x->Cvf;
    y.TimeRange = x->TimeRange;
    y.MarginRate = x->MarginRate;
    y.LockRateX = x->LockRateX;
    y.AddOnRate = x->AddOnRate;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.AddOnLockRateX2 = x->AddOnLockRateX2;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPBMOptionParameterField Converter::SyncDeltaSPBMOptionParameterFieldToCpp(SyncDeltaSPBMOptionParameterField x) {
    CThostFtdcSyncDeltaSPBMOptionParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.Cvf = x.Cvf;
    y.DownPrice = x.DownPrice;
    y.Delta = x.Delta;
    y.SlimiDelta = x.SlimiDelta;
    y.PreSettlementPrice = x.PreSettlementPrice;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMOptionParameterField Converter::CThostFtdcSyncDeltaSPBMOptionParameterFieldToRust(CThostFtdcSyncDeltaSPBMOptionParameterField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMOptionParameterField{.is_null = true};
    SyncDeltaSPBMOptionParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.Cvf = x->Cvf;
    y.DownPrice = x->DownPrice;
    y.Delta = x->Delta;
    y.SlimiDelta = x->SlimiDelta;
    y.PreSettlementPrice = x->PreSettlementPrice;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPBMIntraParameterField Converter::SyncDeltaSPBMIntraParameterFieldToCpp(SyncDeltaSPBMIntraParameterField x) {
    CThostFtdcSyncDeltaSPBMIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    y.IntraRateY = x.IntraRateY;
    y.AddOnIntraRateY2 = x.AddOnIntraRateY2;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMIntraParameterField Converter::CThostFtdcSyncDeltaSPBMIntraParameterFieldToRust(CThostFtdcSyncDeltaSPBMIntraParameterField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMIntraParameterField{.is_null = true};
    SyncDeltaSPBMIntraParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.IntraRateY = x->IntraRateY;
    y.AddOnIntraRateY2 = x->AddOnIntraRateY2;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPBMInterParameterField Converter::SyncDeltaSPBMInterParameterFieldToCpp(SyncDeltaSPBMInterParameterField x) {
    CThostFtdcSyncDeltaSPBMInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SpreadId = x.SpreadId;
    y.InterRateZ = x.InterRateZ;
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMInterParameterField Converter::CThostFtdcSyncDeltaSPBMInterParameterFieldToRust(CThostFtdcSyncDeltaSPBMInterParameterField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMInterParameterField{.is_null = true};
    SyncDeltaSPBMInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SpreadId = x->SpreadId;
    y.InterRateZ = x->InterRateZ;
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPBMAddOnInterParamField Converter::SyncDeltaSPBMAddOnInterParamFieldToCpp(SyncDeltaSPBMAddOnInterParamField x) {
    CThostFtdcSyncDeltaSPBMAddOnInterParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SpreadId = x.SpreadId;
    y.AddOnInterRateZ2 = x.AddOnInterRateZ2;
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPBMAddOnInterParamField Converter::CThostFtdcSyncDeltaSPBMAddOnInterParamFieldToRust(CThostFtdcSyncDeltaSPBMAddOnInterParamField* x) {
    if (x == nullptr)
        return SyncDeltaSPBMAddOnInterParamField{.is_null = true};
    SyncDeltaSPBMAddOnInterParamField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SpreadId = x->SpreadId;
    y.AddOnInterRateZ2 = x->AddOnInterRateZ2;
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPMMInstParamField Converter::SyncDeltaSPMMInstParamFieldToCpp(SyncDeltaSPMMInstParamField x) {
    CThostFtdcSyncDeltaSPMMInstParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InstMarginCalID = x.InstMarginCalID;
    strcpy(y.CommodityID, x.CommodityID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPMMInstParamField Converter::CThostFtdcSyncDeltaSPMMInstParamFieldToRust(CThostFtdcSyncDeltaSPMMInstParamField* x) {
    if (x == nullptr)
        return SyncDeltaSPMMInstParamField{.is_null = true};
    SyncDeltaSPMMInstParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InstMarginCalID = x->InstMarginCalID;
    y.CommodityID = Converter::Gb2312ToRustString(x->CommodityID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPMMProductParamField Converter::SyncDeltaSPMMProductParamFieldToCpp(SyncDeltaSPMMProductParamField x) {
    CThostFtdcSyncDeltaSPMMProductParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.CommodityID, x.CommodityID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPMMProductParamField Converter::CThostFtdcSyncDeltaSPMMProductParamFieldToRust(CThostFtdcSyncDeltaSPMMProductParamField* x) {
    if (x == nullptr)
        return SyncDeltaSPMMProductParamField{.is_null = true};
    SyncDeltaSPMMProductParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.CommodityID = Converter::Gb2312ToRustString(x->CommodityID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaInvestorSPMMModelField Converter::SyncDeltaInvestorSPMMModelFieldToCpp(SyncDeltaInvestorSPMMModelField x) {
    CThostFtdcSyncDeltaInvestorSPMMModelField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.SPMMModelID, x.SPMMModelID.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaInvestorSPMMModelField Converter::CThostFtdcSyncDeltaInvestorSPMMModelFieldToRust(CThostFtdcSyncDeltaInvestorSPMMModelField* x) {
    if (x == nullptr)
        return SyncDeltaInvestorSPMMModelField{.is_null = true};
    SyncDeltaInvestorSPMMModelField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.SPMMModelID = Converter::Gb2312ToRustString(x->SPMMModelID);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaSPMMModelParamField Converter::SyncDeltaSPMMModelParamFieldToCpp(SyncDeltaSPMMModelParamField x) {
    CThostFtdcSyncDeltaSPMMModelParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.SPMMModelID, x.SPMMModelID.c_str());
    strcpy(y.CommodityGroupID, x.CommodityGroupID.c_str());
    y.IntraCommodityRate = x.IntraCommodityRate;
    y.InterCommodityRate = x.InterCommodityRate;
    y.OptionDiscountRate = x.OptionDiscountRate;
    y.MiniMarginRatio = x.MiniMarginRatio;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaSPMMModelParamField Converter::CThostFtdcSyncDeltaSPMMModelParamFieldToRust(CThostFtdcSyncDeltaSPMMModelParamField* x) {
    if (x == nullptr)
        return SyncDeltaSPMMModelParamField{.is_null = true};
    SyncDeltaSPMMModelParamField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SPMMModelID = Converter::Gb2312ToRustString(x->SPMMModelID);
    y.CommodityGroupID = Converter::Gb2312ToRustString(x->CommodityGroupID);
    y.IntraCommodityRate = x->IntraCommodityRate;
    y.InterCommodityRate = x->InterCommodityRate;
    y.OptionDiscountRate = x->OptionDiscountRate;
    y.MiniMarginRatio = x->MiniMarginRatio;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSCombProdInfoField Converter::SyncDeltaRCAMSCombProdInfoFieldToCpp(SyncDeltaRCAMSCombProdInfoField x) {
    CThostFtdcSyncDeltaRCAMSCombProdInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSCombProdInfoField Converter::CThostFtdcSyncDeltaRCAMSCombProdInfoFieldToRust(CThostFtdcSyncDeltaRCAMSCombProdInfoField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSCombProdInfoField{.is_null = true};
    SyncDeltaRCAMSCombProdInfoField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSInstrParameterField Converter::SyncDeltaRCAMSInstrParameterFieldToCpp(SyncDeltaRCAMSInstrParameterField x) {
    CThostFtdcSyncDeltaRCAMSInstrParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.HedgeRate = x.HedgeRate;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSInstrParameterField Converter::CThostFtdcSyncDeltaRCAMSInstrParameterFieldToRust(CThostFtdcSyncDeltaRCAMSInstrParameterField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSInstrParameterField{.is_null = true};
    SyncDeltaRCAMSInstrParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.HedgeRate = x->HedgeRate;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSIntraParameterField Converter::SyncDeltaRCAMSIntraParameterFieldToCpp(SyncDeltaRCAMSIntraParameterField x) {
    CThostFtdcSyncDeltaRCAMSIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    y.HedgeRate = x.HedgeRate;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSIntraParameterField Converter::CThostFtdcSyncDeltaRCAMSIntraParameterFieldToRust(CThostFtdcSyncDeltaRCAMSIntraParameterField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSIntraParameterField{.is_null = true};
    SyncDeltaRCAMSIntraParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.HedgeRate = x->HedgeRate;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSInterParameterField Converter::SyncDeltaRCAMSInterParameterFieldToCpp(SyncDeltaRCAMSInterParameterField x) {
    CThostFtdcSyncDeltaRCAMSInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProductGroupID, x.ProductGroupID.c_str());
    y.Priority = x.Priority;
    y.CreditRate = x.CreditRate;
    strcpy(y.CombProduct1, x.CombProduct1.c_str());
    strcpy(y.CombProduct2, x.CombProduct2.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSInterParameterField Converter::CThostFtdcSyncDeltaRCAMSInterParameterFieldToRust(CThostFtdcSyncDeltaRCAMSInterParameterField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSInterParameterField{.is_null = true};
    SyncDeltaRCAMSInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProductGroupID = Converter::Gb2312ToRustString(x->ProductGroupID);
    y.Priority = x->Priority;
    y.CreditRate = x->CreditRate;
    y.CombProduct1 = Converter::Gb2312ToRustString(x->CombProduct1);
    y.CombProduct2 = Converter::Gb2312ToRustString(x->CombProduct2);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSSOptAdjParamField Converter::SyncDeltaRCAMSSOptAdjParamFieldToCpp(SyncDeltaRCAMSSOptAdjParamField x) {
    CThostFtdcSyncDeltaRCAMSSOptAdjParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.CombProductID, x.CombProductID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.AdjustValue = x.AdjustValue;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSSOptAdjParamField Converter::CThostFtdcSyncDeltaRCAMSSOptAdjParamFieldToRust(CThostFtdcSyncDeltaRCAMSSOptAdjParamField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSSOptAdjParamField{.is_null = true};
    SyncDeltaRCAMSSOptAdjParamField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.CombProductID = Converter::Gb2312ToRustString(x->CombProductID);
    y.HedgeFlag = x->HedgeFlag;
    y.AdjustValue = x->AdjustValue;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSCombRuleDtlField Converter::SyncDeltaRCAMSCombRuleDtlFieldToCpp(SyncDeltaRCAMSCombRuleDtlField x) {
    CThostFtdcSyncDeltaRCAMSCombRuleDtlField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    memcpy(y.ProdGroup, x.ProdGroup.data(), x.ProdGroup.size() * sizeof(uint8_t));
    memcpy(y.RuleId, x.RuleId.data(), x.RuleId.size() * sizeof(uint8_t));
    y.Priority = x.Priority;
    y.HedgeFlag = x.HedgeFlag;
    y.CombMargin = x.CombMargin;
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    y.LegID = x.LegID;
    strcpy(y.LegInstrumentID, x.LegInstrumentID.c_str());
    y.Direction = x.Direction;
    y.LegMultiple = x.LegMultiple;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSCombRuleDtlField Converter::CThostFtdcSyncDeltaRCAMSCombRuleDtlFieldToRust(CThostFtdcSyncDeltaRCAMSCombRuleDtlField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSCombRuleDtlField{.is_null = true};
    SyncDeltaRCAMSCombRuleDtlField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    for (int i = 0; i < 41; i++)
        y.ProdGroup.push_back(x->ProdGroup[i]);
    for (int i = 0; i < 51; i++)
        y.RuleId.push_back(x->RuleId[i]);
    y.Priority = x->Priority;
    y.HedgeFlag = x->HedgeFlag;
    y.CombMargin = x->CombMargin;
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.LegID = x->LegID;
    y.LegInstrumentID = Converter::Gb2312ToRustString(x->LegInstrumentID);
    y.Direction = x->Direction;
    y.LegMultiple = x->LegMultiple;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRCAMSInvstCombPosField Converter::SyncDeltaRCAMSInvstCombPosFieldToCpp(SyncDeltaRCAMSInvstCombPosField x) {
    CThostFtdcSyncDeltaRCAMSInvstCombPosField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.PosiDirection = x.PosiDirection;
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    y.LegID = x.LegID;
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    y.TotalAmt = x.TotalAmt;
    y.ExchMargin = x.ExchMargin;
    y.Margin = x.Margin;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRCAMSInvstCombPosField Converter::CThostFtdcSyncDeltaRCAMSInvstCombPosFieldToRust(CThostFtdcSyncDeltaRCAMSInvstCombPosField* x) {
    if (x == nullptr)
        return SyncDeltaRCAMSInvstCombPosField{.is_null = true};
    SyncDeltaRCAMSInvstCombPosField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.HedgeFlag = x->HedgeFlag;
    y.PosiDirection = x->PosiDirection;
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.LegID = x->LegID;
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    y.TotalAmt = x->TotalAmt;
    y.ExchMargin = x->ExchMargin;
    y.Margin = x->Margin;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRULEInstrParameterField Converter::SyncDeltaRULEInstrParameterFieldToCpp(SyncDeltaRULEInstrParameterField x) {
    CThostFtdcSyncDeltaRULEInstrParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.InstrumentClass = x.InstrumentClass;
    strcpy(y.StdInstrumentID, x.StdInstrumentID.c_str());
    y.BSpecRatio = x.BSpecRatio;
    y.SSpecRatio = x.SSpecRatio;
    y.BHedgeRatio = x.BHedgeRatio;
    y.SHedgeRatio = x.SHedgeRatio;
    y.BAddOnMargin = x.BAddOnMargin;
    y.SAddOnMargin = x.SAddOnMargin;
    y.CommodityGroupID = x.CommodityGroupID;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRULEInstrParameterField Converter::CThostFtdcSyncDeltaRULEInstrParameterFieldToRust(CThostFtdcSyncDeltaRULEInstrParameterField* x) {
    if (x == nullptr)
        return SyncDeltaRULEInstrParameterField{.is_null = true};
    SyncDeltaRULEInstrParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.InstrumentClass = x->InstrumentClass;
    y.StdInstrumentID = Converter::Gb2312ToRustString(x->StdInstrumentID);
    y.BSpecRatio = x->BSpecRatio;
    y.SSpecRatio = x->SSpecRatio;
    y.BHedgeRatio = x->BHedgeRatio;
    y.SHedgeRatio = x->SHedgeRatio;
    y.BAddOnMargin = x->BAddOnMargin;
    y.SAddOnMargin = x->SAddOnMargin;
    y.CommodityGroupID = x->CommodityGroupID;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRULEIntraParameterField Converter::SyncDeltaRULEIntraParameterFieldToCpp(SyncDeltaRULEIntraParameterField x) {
    CThostFtdcSyncDeltaRULEIntraParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.ProdFamilyCode, x.ProdFamilyCode.c_str());
    strcpy(y.StdInstrumentID, x.StdInstrumentID.c_str());
    y.StdInstrMargin = x.StdInstrMargin;
    y.UsualIntraRate = x.UsualIntraRate;
    y.DeliveryIntraRate = x.DeliveryIntraRate;
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRULEIntraParameterField Converter::CThostFtdcSyncDeltaRULEIntraParameterFieldToRust(CThostFtdcSyncDeltaRULEIntraParameterField* x) {
    if (x == nullptr)
        return SyncDeltaRULEIntraParameterField{.is_null = true};
    SyncDeltaRULEIntraParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.ProdFamilyCode = Converter::Gb2312ToRustString(x->ProdFamilyCode);
    y.StdInstrumentID = Converter::Gb2312ToRustString(x->StdInstrumentID);
    y.StdInstrMargin = x->StdInstrMargin;
    y.UsualIntraRate = x->UsualIntraRate;
    y.DeliveryIntraRate = x->DeliveryIntraRate;
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcSyncDeltaRULEInterParameterField Converter::SyncDeltaRULEInterParameterFieldToCpp(SyncDeltaRULEInterParameterField x) {
    CThostFtdcSyncDeltaRULEInterParameterField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradingDay, x.TradingDay.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    y.SpreadId = x.SpreadId;
    y.InterRate = x.InterRate;
    strcpy(y.Leg1ProdFamilyCode, x.Leg1ProdFamilyCode.c_str());
    strcpy(y.Leg2ProdFamilyCode, x.Leg2ProdFamilyCode.c_str());
    y.Leg1PropFactor = x.Leg1PropFactor;
    y.Leg2PropFactor = x.Leg2PropFactor;
    y.CommodityGroupID = x.CommodityGroupID;
    strcpy(y.CommodityGroupName, x.CommodityGroupName.c_str());
    y.ActionDirection = x.ActionDirection;
    y.SyncDeltaSequenceNo = x.SyncDeltaSequenceNo;
    return y;
}

SyncDeltaRULEInterParameterField Converter::CThostFtdcSyncDeltaRULEInterParameterFieldToRust(CThostFtdcSyncDeltaRULEInterParameterField* x) {
    if (x == nullptr)
        return SyncDeltaRULEInterParameterField{.is_null = true};
    SyncDeltaRULEInterParameterField y{};
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.SpreadId = x->SpreadId;
    y.InterRate = x->InterRate;
    y.Leg1ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg1ProdFamilyCode);
    y.Leg2ProdFamilyCode = Converter::Gb2312ToRustString(x->Leg2ProdFamilyCode);
    y.Leg1PropFactor = x->Leg1PropFactor;
    y.Leg2PropFactor = x->Leg2PropFactor;
    y.CommodityGroupID = x->CommodityGroupID;
    y.CommodityGroupName = Converter::Gb2312ToRustString(x->CommodityGroupName);
    y.ActionDirection = x->ActionDirection;
    y.SyncDeltaSequenceNo = x->SyncDeltaSequenceNo;
    return y;
}

CThostFtdcIpAddrParamField Converter::IpAddrParamFieldToCpp(IpAddrParamField x) {
    CThostFtdcIpAddrParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.Address, x.Address.c_str());
    y.DRIdentityID = x.DRIdentityID;
    strcpy(y.DRIdentityName, x.DRIdentityName.c_str());
    y.AddrSrvMode = x.AddrSrvMode;
    y.AddrVer = x.AddrVer;
    y.AddrNo = x.AddrNo;
    strcpy(y.AddrName, x.AddrName.c_str());
    y.IsSM = x.IsSM;
    y.IsLocalAddr = x.IsLocalAddr;
    strcpy(y.Remark, x.Remark.c_str());
    memcpy(y.Site, x.Site.data(), x.Site.size() * sizeof(uint8_t));
    memcpy(y.NetOperator, x.NetOperator.data(), x.NetOperator.size() * sizeof(uint8_t));
    return y;
}

IpAddrParamField Converter::CThostFtdcIpAddrParamFieldToRust(CThostFtdcIpAddrParamField* x) {
    if (x == nullptr)
        return IpAddrParamField{.is_null = true};
    IpAddrParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.DRIdentityID = x->DRIdentityID;
    y.DRIdentityName = Converter::Gb2312ToRustString(x->DRIdentityName);
    y.AddrSrvMode = x->AddrSrvMode;
    y.AddrVer = x->AddrVer;
    y.AddrNo = x->AddrNo;
    y.AddrName = Converter::Gb2312ToRustString(x->AddrName);
    y.IsSM = x->IsSM;
    y.IsLocalAddr = x->IsLocalAddr;
    y.Remark = Converter::Gb2312ToRustString(x->Remark);
    for (int i = 0; i < 51; i++)
        y.Site.push_back(x->Site[i]);
    for (int i = 0; i < 9; i++)
        y.NetOperator.push_back(x->NetOperator[i]);
    return y;
}

CThostFtdcQryIpAddrParamField Converter::QryIpAddrParamFieldToCpp(QryIpAddrParamField x) {
    CThostFtdcQryIpAddrParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryIpAddrParamField Converter::CThostFtdcQryIpAddrParamFieldToRust(CThostFtdcQryIpAddrParamField* x) {
    if (x == nullptr)
        return QryIpAddrParamField{.is_null = true};
    QryIpAddrParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcTGIpAddrParamField Converter::TGIpAddrParamFieldToCpp(TGIpAddrParamField x) {
    CThostFtdcTGIpAddrParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.Address, x.Address.c_str());
    y.DRIdentityID = x.DRIdentityID;
    strcpy(y.DRIdentityName, x.DRIdentityName.c_str());
    y.AddrSrvMode = x.AddrSrvMode;
    y.AddrVer = x.AddrVer;
    y.AddrNo = x.AddrNo;
    strcpy(y.AddrName, x.AddrName.c_str());
    y.IsSM = x.IsSM;
    y.IsLocalAddr = x.IsLocalAddr;
    strcpy(y.Remark, x.Remark.c_str());
    memcpy(y.Site, x.Site.data(), x.Site.size() * sizeof(uint8_t));
    memcpy(y.NetOperator, x.NetOperator.data(), x.NetOperator.size() * sizeof(uint8_t));
    return y;
}

TGIpAddrParamField Converter::CThostFtdcTGIpAddrParamFieldToRust(CThostFtdcTGIpAddrParamField* x) {
    if (x == nullptr)
        return TGIpAddrParamField{.is_null = true};
    TGIpAddrParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.DRIdentityID = x->DRIdentityID;
    y.DRIdentityName = Converter::Gb2312ToRustString(x->DRIdentityName);
    y.AddrSrvMode = x->AddrSrvMode;
    y.AddrVer = x->AddrVer;
    y.AddrNo = x->AddrNo;
    y.AddrName = Converter::Gb2312ToRustString(x->AddrName);
    y.IsSM = x->IsSM;
    y.IsLocalAddr = x->IsLocalAddr;
    y.Remark = Converter::Gb2312ToRustString(x->Remark);
    for (int i = 0; i < 51; i++)
        y.Site.push_back(x->Site[i]);
    for (int i = 0; i < 9; i++)
        y.NetOperator.push_back(x->NetOperator[i]);
    return y;
}

CThostFtdcQryTGIpAddrParamField Converter::QryTGIpAddrParamFieldToCpp(QryTGIpAddrParamField x) {
    CThostFtdcQryTGIpAddrParamField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.AppID, x.AppID.c_str());
    return y;
}

QryTGIpAddrParamField Converter::CThostFtdcQryTGIpAddrParamFieldToRust(CThostFtdcQryTGIpAddrParamField* x) {
    if (x == nullptr)
        return QryTGIpAddrParamField{.is_null = true};
    QryTGIpAddrParamField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    return y;
}

CThostFtdcTGSessionQryStatusField Converter::TGSessionQryStatusFieldToCpp(TGSessionQryStatusField x) {
    CThostFtdcTGSessionQryStatusField y;
    memset(&y, 0, sizeof(y));
    y.LastQryFreq = x.LastQryFreq;
    y.QryStatus = x.QryStatus;
    return y;
}

TGSessionQryStatusField Converter::CThostFtdcTGSessionQryStatusFieldToRust(CThostFtdcTGSessionQryStatusField* x) {
    if (x == nullptr)
        return TGSessionQryStatusField{.is_null = true};
    TGSessionQryStatusField y{};
    y.LastQryFreq = x->LastQryFreq;
    y.QryStatus = x->QryStatus;
    return y;
}

CThostFtdcLocalAddrConfigField Converter::LocalAddrConfigFieldToCpp(LocalAddrConfigField x) {
    CThostFtdcLocalAddrConfigField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.PeerAddr, x.PeerAddr.c_str());
    memcpy(y.NetMask, x.NetMask.data(), x.NetMask.size() * sizeof(uint8_t));
    y.DRIdentityID = x.DRIdentityID;
    strcpy(y.LocalAddress, x.LocalAddress.c_str());
    return y;
}

LocalAddrConfigField Converter::CThostFtdcLocalAddrConfigFieldToRust(CThostFtdcLocalAddrConfigField* x) {
    if (x == nullptr)
        return LocalAddrConfigField{.is_null = true};
    LocalAddrConfigField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.PeerAddr = Converter::Gb2312ToRustString(x->PeerAddr);
    for (int i = 0; i < 129; i++)
        y.NetMask.push_back(x->NetMask[i]);
    y.DRIdentityID = x->DRIdentityID;
    y.LocalAddress = Converter::Gb2312ToRustString(x->LocalAddress);
    return y;
}

CThostFtdcQryLocalAddrConfigField Converter::QryLocalAddrConfigFieldToCpp(QryLocalAddrConfigField x) {
    CThostFtdcQryLocalAddrConfigField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryLocalAddrConfigField Converter::CThostFtdcQryLocalAddrConfigFieldToRust(CThostFtdcQryLocalAddrConfigField* x) {
    if (x == nullptr)
        return QryLocalAddrConfigField{.is_null = true};
    QryLocalAddrConfigField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcReqQueryBankAccountBySecField Converter::ReqQueryBankAccountBySecFieldToCpp(ReqQueryBankAccountBySecField x) {
    CThostFtdcReqQueryBankAccountBySecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.FutureSerial = x.FutureSerial;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    y.DRIdentityID = x.DRIdentityID;
    y.SecFutureSerial = x.SecFutureSerial;
    return y;
}

ReqQueryBankAccountBySecField Converter::CThostFtdcReqQueryBankAccountBySecFieldToRust(CThostFtdcReqQueryBankAccountBySecField* x) {
    if (x == nullptr)
        return ReqQueryBankAccountBySecField{.is_null = true};
    ReqQueryBankAccountBySecField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.FutureSerial = x->FutureSerial;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    y.DRIdentityID = x->DRIdentityID;
    y.SecFutureSerial = x->SecFutureSerial;
    return y;
}

CThostFtdcRspQueryBankAccountBySecField Converter::RspQueryBankAccountBySecFieldToCpp(RspQueryBankAccountBySecField x) {
    CThostFtdcRspQueryBankAccountBySecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.FutureSerial = x.FutureSerial;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.BankUseAmount = x.BankUseAmount;
    y.BankFetchAmount = x.BankFetchAmount;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    y.DRIdentityID = x.DRIdentityID;
    y.SecFutureSerial = x.SecFutureSerial;
    return y;
}

RspQueryBankAccountBySecField Converter::CThostFtdcRspQueryBankAccountBySecFieldToRust(CThostFtdcRspQueryBankAccountBySecField* x) {
    if (x == nullptr)
        return RspQueryBankAccountBySecField{.is_null = true};
    RspQueryBankAccountBySecField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.FutureSerial = x->FutureSerial;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.BankUseAmount = x->BankUseAmount;
    y.BankFetchAmount = x->BankFetchAmount;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    y.DRIdentityID = x->DRIdentityID;
    y.SecFutureSerial = x->SecFutureSerial;
    return y;
}

CThostFtdcReqTransferBySecField Converter::ReqTransferBySecFieldToCpp(ReqTransferBySecField x) {
    CThostFtdcReqTransferBySecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.FutureSerial = x.FutureSerial;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.FutureFetchAmount = x.FutureFetchAmount;
    y.FeePayFlag = x.FeePayFlag;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.TransferStatus = x.TransferStatus;
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    y.DRIdentityID = x.DRIdentityID;
    y.SecFutureSerial = x.SecFutureSerial;
    return y;
}

ReqTransferBySecField Converter::CThostFtdcReqTransferBySecFieldToRust(CThostFtdcReqTransferBySecField* x) {
    if (x == nullptr)
        return ReqTransferBySecField{.is_null = true};
    ReqTransferBySecField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.FutureSerial = x->FutureSerial;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.FutureFetchAmount = x->FutureFetchAmount;
    y.FeePayFlag = x->FeePayFlag;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.TransferStatus = x->TransferStatus;
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    y.DRIdentityID = x->DRIdentityID;
    y.SecFutureSerial = x->SecFutureSerial;
    return y;
}

CThostFtdcRspTransferBySecField Converter::RspTransferBySecFieldToCpp(RspTransferBySecField x) {
    CThostFtdcRspTransferBySecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.InstallID = x.InstallID;
    y.FutureSerial = x.FutureSerial;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.TradeAmount = x.TradeAmount;
    y.FutureFetchAmount = x.FutureFetchAmount;
    y.FeePayFlag = x.FeePayFlag;
    y.CustFee = x.CustFee;
    y.BrokerFee = x.BrokerFee;
    memcpy(y.Message, x.Message.data(), x.Message.size() * sizeof(uint8_t));
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.TransferStatus = x.TransferStatus;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    y.DRIdentityID = x.DRIdentityID;
    y.SecFutureSerial = x.SecFutureSerial;
    return y;
}

RspTransferBySecField Converter::CThostFtdcRspTransferBySecFieldToRust(CThostFtdcRspTransferBySecField* x) {
    if (x == nullptr)
        return RspTransferBySecField{.is_null = true};
    RspTransferBySecField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.InstallID = x->InstallID;
    y.FutureSerial = x->FutureSerial;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.TradeAmount = x->TradeAmount;
    y.FutureFetchAmount = x->FutureFetchAmount;
    y.FeePayFlag = x->FeePayFlag;
    y.CustFee = x->CustFee;
    y.BrokerFee = x->BrokerFee;
    for (int i = 0; i < 129; i++)
        y.Message.push_back(x->Message[i]);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.TransferStatus = x->TransferStatus;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    y.DRIdentityID = x->DRIdentityID;
    y.SecFutureSerial = x->SecFutureSerial;
    return y;
}

CThostFtdcNotifyQueryFutureAccountBySecField Converter::NotifyQueryFutureAccountBySecFieldToCpp(NotifyQueryFutureAccountBySecField x) {
    CThostFtdcNotifyQueryFutureAccountBySecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.TradeCode, x.TradeCode.c_str());
    strcpy(y.BankID, x.BankID.c_str());
    strcpy(y.BankBranchID, x.BankBranchID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.BrokerBranchID, x.BrokerBranchID.c_str());
    strcpy(y.TradeDate, x.TradeDate.c_str());
    strcpy(y.TradeTime, x.TradeTime.c_str());
    strcpy(y.BankSerial, x.BankSerial.c_str());
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.PlateSerial = x.PlateSerial;
    y.LastFragment = x.LastFragment;
    y.SessionID = x.SessionID;
    strcpy(y.CustomerName, x.CustomerName.c_str());
    y.IdCardType = x.IdCardType;
    strcpy(y.IdentifiedCardNo, x.IdentifiedCardNo.c_str());
    y.CustType = x.CustType;
    strcpy(y.BankAccount, x.BankAccount.c_str());
    strcpy(y.BankPassWord, x.BankPassWord.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.Password, x.Password.c_str());
    y.FutureSerial = x.FutureSerial;
    y.InstallID = x.InstallID;
    strcpy(y.UserID, x.UserID.c_str());
    y.VerifyCertNoFlag = x.VerifyCertNoFlag;
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    memcpy(y.Digest, x.Digest.data(), x.Digest.size() * sizeof(uint8_t));
    y.BankAccType = x.BankAccType;
    strcpy(y.DeviceID, x.DeviceID.c_str());
    y.BankSecuAccType = x.BankSecuAccType;
    memcpy(y.BrokerIDByBank, x.BrokerIDByBank.data(), x.BrokerIDByBank.size() * sizeof(uint8_t));
    memcpy(y.BankSecuAcc, x.BankSecuAcc.data(), x.BankSecuAcc.size() * sizeof(uint8_t));
    y.BankPwdFlag = x.BankPwdFlag;
    y.SecuPwdFlag = x.SecuPwdFlag;
    strcpy(y.OperNo, x.OperNo.c_str());
    y.RequestID = x.RequestID;
    y.TID = x.TID;
    y.BankUseAmount = x.BankUseAmount;
    y.BankFetchAmount = x.BankFetchAmount;
    y.ErrorID = x.ErrorID;
    strcpy(y.ErrorMsg, x.ErrorMsg.c_str());
    strcpy(y.LongCustomerName, x.LongCustomerName.c_str());
    y.DRIdentityID = x.DRIdentityID;
    y.SecFutureSerial = x.SecFutureSerial;
    return y;
}

NotifyQueryFutureAccountBySecField Converter::CThostFtdcNotifyQueryFutureAccountBySecFieldToRust(CThostFtdcNotifyQueryFutureAccountBySecField* x) {
    if (x == nullptr)
        return NotifyQueryFutureAccountBySecField{.is_null = true};
    NotifyQueryFutureAccountBySecField y{};
    y.TradeCode = Converter::Gb2312ToRustString(x->TradeCode);
    y.BankID = Converter::Gb2312ToRustString(x->BankID);
    y.BankBranchID = Converter::Gb2312ToRustString(x->BankBranchID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.BrokerBranchID = Converter::Gb2312ToRustString(x->BrokerBranchID);
    y.TradeDate = Converter::Gb2312ToRustString(x->TradeDate);
    y.TradeTime = Converter::Gb2312ToRustString(x->TradeTime);
    y.BankSerial = Converter::Gb2312ToRustString(x->BankSerial);
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.PlateSerial = x->PlateSerial;
    y.LastFragment = x->LastFragment;
    y.SessionID = x->SessionID;
    y.CustomerName = Converter::Gb2312ToRustString(x->CustomerName);
    y.IdCardType = x->IdCardType;
    y.IdentifiedCardNo = Converter::Gb2312ToRustString(x->IdentifiedCardNo);
    y.CustType = x->CustType;
    y.BankAccount = Converter::Gb2312ToRustString(x->BankAccount);
    y.BankPassWord = Converter::Gb2312ToRustString(x->BankPassWord);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.Password = Converter::Gb2312ToRustString(x->Password);
    y.FutureSerial = x->FutureSerial;
    y.InstallID = x->InstallID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.VerifyCertNoFlag = x->VerifyCertNoFlag;
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    for (int i = 0; i < 36; i++)
        y.Digest.push_back(x->Digest[i]);
    y.BankAccType = x->BankAccType;
    y.DeviceID = Converter::Gb2312ToRustString(x->DeviceID);
    y.BankSecuAccType = x->BankSecuAccType;
    for (int i = 0; i < 33; i++)
        y.BrokerIDByBank.push_back(x->BrokerIDByBank[i]);
    for (int i = 0; i < 41; i++)
        y.BankSecuAcc.push_back(x->BankSecuAcc[i]);
    y.BankPwdFlag = x->BankPwdFlag;
    y.SecuPwdFlag = x->SecuPwdFlag;
    y.OperNo = Converter::Gb2312ToRustString(x->OperNo);
    y.RequestID = x->RequestID;
    y.TID = x->TID;
    y.BankUseAmount = x->BankUseAmount;
    y.BankFetchAmount = x->BankFetchAmount;
    y.ErrorID = x->ErrorID;
    y.ErrorMsg = Converter::Gb2312ToRustString(x->ErrorMsg);
    y.LongCustomerName = Converter::Gb2312ToRustString(x->LongCustomerName);
    y.DRIdentityID = x->DRIdentityID;
    y.SecFutureSerial = x->SecFutureSerial;
    return y;
}

CThostFtdcExitEmergencyField Converter::ExitEmergencyFieldToCpp(ExitEmergencyField x) {
    CThostFtdcExitEmergencyField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

ExitEmergencyField Converter::CThostFtdcExitEmergencyFieldToRust(CThostFtdcExitEmergencyField* x) {
    if (x == nullptr)
        return ExitEmergencyField{.is_null = true};
    ExitEmergencyField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcInvestorPortfMarginModelField Converter::InvestorPortfMarginModelFieldToCpp(InvestorPortfMarginModelField x) {
    CThostFtdcInvestorPortfMarginModelField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.MarginModelID, x.MarginModelID.c_str());
    return y;
}

InvestorPortfMarginModelField Converter::CThostFtdcInvestorPortfMarginModelFieldToRust(CThostFtdcInvestorPortfMarginModelField* x) {
    if (x == nullptr)
        return InvestorPortfMarginModelField{.is_null = true};
    InvestorPortfMarginModelField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.MarginModelID = Converter::Gb2312ToRustString(x->MarginModelID);
    return y;
}

CThostFtdcInvestorPortfSettingField Converter::InvestorPortfSettingFieldToCpp(InvestorPortfSettingField x) {
    CThostFtdcInvestorPortfSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    y.HedgeFlag = x.HedgeFlag;
    y.UsePortf = x.UsePortf;
    return y;
}

InvestorPortfSettingField Converter::CThostFtdcInvestorPortfSettingFieldToRust(CThostFtdcInvestorPortfSettingField* x) {
    if (x == nullptr)
        return InvestorPortfSettingField{.is_null = true};
    InvestorPortfSettingField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.HedgeFlag = x->HedgeFlag;
    y.UsePortf = x->UsePortf;
    return y;
}

CThostFtdcQryInvestorPortfSettingField Converter::QryInvestorPortfSettingFieldToCpp(QryInvestorPortfSettingField x) {
    CThostFtdcQryInvestorPortfSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    return y;
}

QryInvestorPortfSettingField Converter::CThostFtdcQryInvestorPortfSettingFieldToRust(CThostFtdcQryInvestorPortfSettingField* x) {
    if (x == nullptr)
        return QryInvestorPortfSettingField{.is_null = true};
    QryInvestorPortfSettingField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    return y;
}

CThostFtdcUserPasswordUpdateFromSecField Converter::UserPasswordUpdateFromSecFieldToCpp(UserPasswordUpdateFromSecField x) {
    CThostFtdcUserPasswordUpdateFromSecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.OldPassword, x.OldPassword.c_str());
    strcpy(y.NewPassword, x.NewPassword.c_str());
    y.FromSec = x.FromSec;
    return y;
}

UserPasswordUpdateFromSecField Converter::CThostFtdcUserPasswordUpdateFromSecFieldToRust(CThostFtdcUserPasswordUpdateFromSecField* x) {
    if (x == nullptr)
        return UserPasswordUpdateFromSecField{.is_null = true};
    UserPasswordUpdateFromSecField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.OldPassword = Converter::Gb2312ToRustString(x->OldPassword);
    y.NewPassword = Converter::Gb2312ToRustString(x->NewPassword);
    y.FromSec = x->FromSec;
    return y;
}

CThostFtdcSettlementInfoConfirmFromSecField Converter::SettlementInfoConfirmFromSecFieldToCpp(SettlementInfoConfirmFromSecField x) {
    CThostFtdcSettlementInfoConfirmFromSecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ConfirmDate, x.ConfirmDate.c_str());
    strcpy(y.ConfirmTime, x.ConfirmTime.c_str());
    y.FromSec = x.FromSec;
    return y;
}

SettlementInfoConfirmFromSecField Converter::CThostFtdcSettlementInfoConfirmFromSecFieldToRust(CThostFtdcSettlementInfoConfirmFromSecField* x) {
    if (x == nullptr)
        return SettlementInfoConfirmFromSecField{.is_null = true};
    SettlementInfoConfirmFromSecField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ConfirmDate = Converter::Gb2312ToRustString(x->ConfirmDate);
    y.ConfirmTime = Converter::Gb2312ToRustString(x->ConfirmTime);
    y.FromSec = x->FromSec;
    return y;
}

CThostFtdcTradingAccountPasswordUpdateFromSecField Converter::TradingAccountPasswordUpdateFromSecFieldToCpp(TradingAccountPasswordUpdateFromSecField x) {
    CThostFtdcTradingAccountPasswordUpdateFromSecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.AccountID, x.AccountID.c_str());
    strcpy(y.OldPassword, x.OldPassword.c_str());
    strcpy(y.NewPassword, x.NewPassword.c_str());
    strcpy(y.CurrencyID, x.CurrencyID.c_str());
    y.FromSec = x.FromSec;
    return y;
}

TradingAccountPasswordUpdateFromSecField Converter::CThostFtdcTradingAccountPasswordUpdateFromSecFieldToRust(CThostFtdcTradingAccountPasswordUpdateFromSecField* x) {
    if (x == nullptr)
        return TradingAccountPasswordUpdateFromSecField{.is_null = true};
    TradingAccountPasswordUpdateFromSecField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.AccountID = Converter::Gb2312ToRustString(x->AccountID);
    y.OldPassword = Converter::Gb2312ToRustString(x->OldPassword);
    y.NewPassword = Converter::Gb2312ToRustString(x->NewPassword);
    y.CurrencyID = Converter::Gb2312ToRustString(x->CurrencyID);
    y.FromSec = x->FromSec;
    return y;
}

CThostFtdcRiskForbiddenRightField Converter::RiskForbiddenRightFieldToCpp(RiskForbiddenRightField x) {
    CThostFtdcRiskForbiddenRightField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    return y;
}

RiskForbiddenRightField Converter::CThostFtdcRiskForbiddenRightFieldToRust(CThostFtdcRiskForbiddenRightField* x) {
    if (x == nullptr)
        return RiskForbiddenRightField{.is_null = true};
    RiskForbiddenRightField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    return y;
}

CThostFtdcInvestorInfoCommRecField Converter::InvestorInfoCommRecFieldToCpp(InvestorInfoCommRecField x) {
    CThostFtdcInvestorInfoCommRecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    y.OrderCount = x.OrderCount;
    y.OrderActionCount = x.OrderActionCount;
    y.ForQuoteCnt = x.ForQuoteCnt;
    y.InfoComm = x.InfoComm;
    y.IsOptSeries = x.IsOptSeries;
    strcpy(y.ProductID, x.ProductID.c_str());
    y.InfoCnt = x.InfoCnt;
    return y;
}

InvestorInfoCommRecField Converter::CThostFtdcInvestorInfoCommRecFieldToRust(CThostFtdcInvestorInfoCommRecField* x) {
    if (x == nullptr)
        return InvestorInfoCommRecField{.is_null = true};
    InvestorInfoCommRecField y{};
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.OrderCount = x->OrderCount;
    y.OrderActionCount = x->OrderActionCount;
    y.ForQuoteCnt = x->ForQuoteCnt;
    y.InfoComm = x->InfoComm;
    y.IsOptSeries = x->IsOptSeries;
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.InfoCnt = x->InfoCnt;
    return y;
}

CThostFtdcQryInvestorInfoCommRecField Converter::QryInvestorInfoCommRecFieldToCpp(QryInvestorInfoCommRecField x) {
    CThostFtdcQryInvestorInfoCommRecField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryInvestorInfoCommRecField Converter::CThostFtdcQryInvestorInfoCommRecFieldToRust(CThostFtdcQryInvestorInfoCommRecField* x) {
    if (x == nullptr)
        return QryInvestorInfoCommRecField{.is_null = true};
    QryInvestorInfoCommRecField y{};
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcCombLegField Converter::CombLegFieldToCpp(CombLegField x) {
    CThostFtdcCombLegField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.CombInstrumentID, x.CombInstrumentID.c_str());
    y.LegID = x.LegID;
    strcpy(y.LegInstrumentID, x.LegInstrumentID.c_str());
    y.Direction = x.Direction;
    y.LegMultiple = x.LegMultiple;
    y.ImplyLevel = x.ImplyLevel;
    return y;
}

CombLegField Converter::CThostFtdcCombLegFieldToRust(CThostFtdcCombLegField* x) {
    if (x == nullptr)
        return CombLegField{.is_null = true};
    CombLegField y{};
    y.CombInstrumentID = Converter::Gb2312ToRustString(x->CombInstrumentID);
    y.LegID = x->LegID;
    y.LegInstrumentID = Converter::Gb2312ToRustString(x->LegInstrumentID);
    y.Direction = x->Direction;
    y.LegMultiple = x->LegMultiple;
    y.ImplyLevel = x->ImplyLevel;
    return y;
}

CThostFtdcQryCombLegField Converter::QryCombLegFieldToCpp(QryCombLegField x) {
    CThostFtdcQryCombLegField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.LegInstrumentID, x.LegInstrumentID.c_str());
    return y;
}

QryCombLegField Converter::CThostFtdcQryCombLegFieldToRust(CThostFtdcQryCombLegField* x) {
    if (x == nullptr)
        return QryCombLegField{.is_null = true};
    QryCombLegField y{};
    y.LegInstrumentID = Converter::Gb2312ToRustString(x->LegInstrumentID);
    return y;
}

CThostFtdcInputOffsetSettingField Converter::InputOffsetSettingFieldToCpp(InputOffsetSettingField x) {
    CThostFtdcInputOffsetSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.UnderlyingInstrID, x.UnderlyingInstrID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.OffsetType = x.OffsetType;
    y.Volume = x.Volume;
    y.IsOffset = x.IsOffset;
    y.RequestID = x.RequestID;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    return y;
}

InputOffsetSettingField Converter::CThostFtdcInputOffsetSettingFieldToRust(CThostFtdcInputOffsetSettingField* x) {
    if (x == nullptr)
        return InputOffsetSettingField{.is_null = true};
    InputOffsetSettingField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.UnderlyingInstrID = Converter::Gb2312ToRustString(x->UnderlyingInstrID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.OffsetType = x->OffsetType;
    y.Volume = x->Volume;
    y.IsOffset = x->IsOffset;
    y.RequestID = x->RequestID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    return y;
}

CThostFtdcOffsetSettingField Converter::OffsetSettingFieldToCpp(OffsetSettingField x) {
    CThostFtdcOffsetSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.UnderlyingInstrID, x.UnderlyingInstrID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.OffsetType = x.OffsetType;
    y.Volume = x.Volume;
    y.IsOffset = x.IsOffset;
    y.RequestID = x.RequestID;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    memcpy(y.ExchangeSerialNo, x.ExchangeSerialNo.data(), x.ExchangeSerialNo.size() * sizeof(uint8_t));
    strcpy(y.ExchangeProductID, x.ExchangeProductID.c_str());
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    y.OrderSubmitStatus = x.OrderSubmitStatus;
    strcpy(y.TradingDay, x.TradingDay.c_str());
    y.SettlementID = x.SettlementID;
    strcpy(y.InsertDate, x.InsertDate.c_str());
    strcpy(y.InsertTime, x.InsertTime.c_str());
    strcpy(y.CancelTime, x.CancelTime.c_str());
    y.ExecResult = x.ExecResult;
    y.SequenceNo = x.SequenceNo;
    y.FrontID = x.FrontID;
    y.SessionID = x.SessionID;
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.ActiveUserID, x.ActiveUserID.c_str());
    y.BrokerOffsetSettingSeq = x.BrokerOffsetSettingSeq;
    y.ApplySrc = x.ApplySrc;
    return y;
}

OffsetSettingField Converter::CThostFtdcOffsetSettingFieldToRust(CThostFtdcOffsetSettingField* x) {
    if (x == nullptr)
        return OffsetSettingField{.is_null = true};
    OffsetSettingField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.UnderlyingInstrID = Converter::Gb2312ToRustString(x->UnderlyingInstrID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.OffsetType = x->OffsetType;
    y.Volume = x->Volume;
    y.IsOffset = x->IsOffset;
    y.RequestID = x->RequestID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    for (int i = 0; i < 81; i++)
        y.ExchangeSerialNo.push_back(x->ExchangeSerialNo[i]);
    y.ExchangeProductID = Converter::Gb2312ToRustString(x->ExchangeProductID);
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.OrderSubmitStatus = x->OrderSubmitStatus;
    y.TradingDay = Converter::Gb2312ToRustString(x->TradingDay);
    y.SettlementID = x->SettlementID;
    y.InsertDate = Converter::Gb2312ToRustString(x->InsertDate);
    y.InsertTime = Converter::Gb2312ToRustString(x->InsertTime);
    y.CancelTime = Converter::Gb2312ToRustString(x->CancelTime);
    y.ExecResult = x->ExecResult;
    y.SequenceNo = x->SequenceNo;
    y.FrontID = x->FrontID;
    y.SessionID = x->SessionID;
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.ActiveUserID = Converter::Gb2312ToRustString(x->ActiveUserID);
    y.BrokerOffsetSettingSeq = x->BrokerOffsetSettingSeq;
    y.ApplySrc = x->ApplySrc;
    return y;
}

CThostFtdcCancelOffsetSettingField Converter::CancelOffsetSettingFieldToCpp(CancelOffsetSettingField x) {
    CThostFtdcCancelOffsetSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.InstrumentID, x.InstrumentID.c_str());
    strcpy(y.UnderlyingInstrID, x.UnderlyingInstrID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.OffsetType = x.OffsetType;
    y.Volume = x.Volume;
    y.IsOffset = x.IsOffset;
    y.RequestID = x.RequestID;
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ExchangeID, x.ExchangeID.c_str());
    strcpy(y.IPAddress, x.IPAddress.c_str());
    strcpy(y.MacAddress, x.MacAddress.c_str());
    strcpy(y.ExchangeInstID, x.ExchangeInstID.c_str());
    memcpy(y.ExchangeSerialNo, x.ExchangeSerialNo.data(), x.ExchangeSerialNo.size() * sizeof(uint8_t));
    strcpy(y.ExchangeProductID, x.ExchangeProductID.c_str());
    strcpy(y.TraderID, x.TraderID.c_str());
    y.InstallID = x.InstallID;
    strcpy(y.ParticipantID, x.ParticipantID.c_str());
    strcpy(y.ClientID, x.ClientID.c_str());
    y.OrderActionStatus = x.OrderActionStatus;
    strcpy(y.StatusMsg, x.StatusMsg.c_str());
    strcpy(y.ActionLocalID, x.ActionLocalID.c_str());
    strcpy(y.ActionDate, x.ActionDate.c_str());
    strcpy(y.ActionTime, x.ActionTime.c_str());
    return y;
}

CancelOffsetSettingField Converter::CThostFtdcCancelOffsetSettingFieldToRust(CThostFtdcCancelOffsetSettingField* x) {
    if (x == nullptr)
        return CancelOffsetSettingField{.is_null = true};
    CancelOffsetSettingField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.InstrumentID = Converter::Gb2312ToRustString(x->InstrumentID);
    y.UnderlyingInstrID = Converter::Gb2312ToRustString(x->UnderlyingInstrID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.OffsetType = x->OffsetType;
    y.Volume = x->Volume;
    y.IsOffset = x->IsOffset;
    y.RequestID = x->RequestID;
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ExchangeID = Converter::Gb2312ToRustString(x->ExchangeID);
    y.IPAddress = Converter::Gb2312ToRustString(x->IPAddress);
    y.MacAddress = Converter::Gb2312ToRustString(x->MacAddress);
    y.ExchangeInstID = Converter::Gb2312ToRustString(x->ExchangeInstID);
    for (int i = 0; i < 81; i++)
        y.ExchangeSerialNo.push_back(x->ExchangeSerialNo[i]);
    y.ExchangeProductID = Converter::Gb2312ToRustString(x->ExchangeProductID);
    y.TraderID = Converter::Gb2312ToRustString(x->TraderID);
    y.InstallID = x->InstallID;
    y.ParticipantID = Converter::Gb2312ToRustString(x->ParticipantID);
    y.ClientID = Converter::Gb2312ToRustString(x->ClientID);
    y.OrderActionStatus = x->OrderActionStatus;
    y.StatusMsg = Converter::Gb2312ToRustString(x->StatusMsg);
    y.ActionLocalID = Converter::Gb2312ToRustString(x->ActionLocalID);
    y.ActionDate = Converter::Gb2312ToRustString(x->ActionDate);
    y.ActionTime = Converter::Gb2312ToRustString(x->ActionTime);
    return y;
}

CThostFtdcQryOffsetSettingField Converter::QryOffsetSettingFieldToCpp(QryOffsetSettingField x) {
    CThostFtdcQryOffsetSettingField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.ProductID, x.ProductID.c_str());
    y.OffsetType = x.OffsetType;
    return y;
}

QryOffsetSettingField Converter::CThostFtdcQryOffsetSettingFieldToRust(CThostFtdcQryOffsetSettingField* x) {
    if (x == nullptr)
        return QryOffsetSettingField{.is_null = true};
    QryOffsetSettingField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.ProductID = Converter::Gb2312ToRustString(x->ProductID);
    y.OffsetType = x->OffsetType;
    return y;
}

CThostFtdcAddrAppIDRelationField Converter::AddrAppIDRelationFieldToCpp(AddrAppIDRelationField x) {
    CThostFtdcAddrAppIDRelationField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.Address, x.Address.c_str());
    y.DRIdentityID = x.DRIdentityID;
    strcpy(y.AppID, x.AppID.c_str());
    return y;
}

AddrAppIDRelationField Converter::CThostFtdcAddrAppIDRelationFieldToRust(CThostFtdcAddrAppIDRelationField* x) {
    if (x == nullptr)
        return AddrAppIDRelationField{.is_null = true};
    AddrAppIDRelationField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.Address = Converter::Gb2312ToRustString(x->Address);
    y.DRIdentityID = x->DRIdentityID;
    y.AppID = Converter::Gb2312ToRustString(x->AppID);
    return y;
}

CThostFtdcQryAddrAppIDRelationField Converter::QryAddrAppIDRelationFieldToCpp(QryAddrAppIDRelationField x) {
    CThostFtdcQryAddrAppIDRelationField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryAddrAppIDRelationField Converter::CThostFtdcQryAddrAppIDRelationFieldToRust(CThostFtdcQryAddrAppIDRelationField* x) {
    if (x == nullptr)
        return QryAddrAppIDRelationField{.is_null = true};
    QryAddrAppIDRelationField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcWechatUserSystemInfoField Converter::WechatUserSystemInfoFieldToCpp(WechatUserSystemInfoField x) {
    CThostFtdcWechatUserSystemInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    y.WechatCltSysInfoLen = x.WechatCltSysInfoLen;
    strcpy(y.WechatCltSysInfo, x.WechatCltSysInfo.c_str());
    y.ClientIPPort = x.ClientIPPort;
    strcpy(y.ClientLoginTime, x.ClientLoginTime.c_str());
    strcpy(y.ClientAppID, x.ClientAppID.c_str());
    strcpy(y.ClientPublicIP, x.ClientPublicIP.c_str());
    strcpy(y.ClientLoginRemark, x.ClientLoginRemark.c_str());
    return y;
}

WechatUserSystemInfoField Converter::CThostFtdcWechatUserSystemInfoFieldToRust(CThostFtdcWechatUserSystemInfoField* x) {
    if (x == nullptr)
        return WechatUserSystemInfoField{.is_null = true};
    WechatUserSystemInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.WechatCltSysInfoLen = x->WechatCltSysInfoLen;
    y.WechatCltSysInfo = Converter::Gb2312ToRustString(x->WechatCltSysInfo);
    y.ClientIPPort = x->ClientIPPort;
    y.ClientLoginTime = Converter::Gb2312ToRustString(x->ClientLoginTime);
    y.ClientAppID = Converter::Gb2312ToRustString(x->ClientAppID);
    y.ClientPublicIP = Converter::Gb2312ToRustString(x->ClientPublicIP);
    y.ClientLoginRemark = Converter::Gb2312ToRustString(x->ClientLoginRemark);
    return y;
}

CThostFtdcInvestorReserveInfoField Converter::InvestorReserveInfoFieldToCpp(InvestorReserveInfoField x) {
    CThostFtdcInvestorReserveInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.UserID, x.UserID.c_str());
    strcpy(y.ReserveInfo, x.ReserveInfo.c_str());
    return y;
}

InvestorReserveInfoField Converter::CThostFtdcInvestorReserveInfoFieldToRust(CThostFtdcInvestorReserveInfoField* x) {
    if (x == nullptr)
        return InvestorReserveInfoField{.is_null = true};
    InvestorReserveInfoField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.UserID = Converter::Gb2312ToRustString(x->UserID);
    y.ReserveInfo = Converter::Gb2312ToRustString(x->ReserveInfo);
    return y;
}

CThostFtdcQryInvestorDepartmentFlatField Converter::QryInvestorDepartmentFlatFieldToCpp(QryInvestorDepartmentFlatField x) {
    CThostFtdcQryInvestorDepartmentFlatField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryInvestorDepartmentFlatField Converter::CThostFtdcQryInvestorDepartmentFlatFieldToRust(CThostFtdcQryInvestorDepartmentFlatField* x) {
    if (x == nullptr)
        return QryInvestorDepartmentFlatField{.is_null = true};
    QryInvestorDepartmentFlatField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcInvestorDepartmentFlatField Converter::InvestorDepartmentFlatFieldToCpp(InvestorDepartmentFlatField x) {
    CThostFtdcInvestorDepartmentFlatField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    strcpy(y.InvestorID, x.InvestorID.c_str());
    strcpy(y.DepartmentID, x.DepartmentID.c_str());
    return y;
}

InvestorDepartmentFlatField Converter::CThostFtdcInvestorDepartmentFlatFieldToRust(CThostFtdcInvestorDepartmentFlatField* x) {
    if (x == nullptr)
        return InvestorDepartmentFlatField{.is_null = true};
    InvestorDepartmentFlatField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    y.InvestorID = Converter::Gb2312ToRustString(x->InvestorID);
    y.DepartmentID = Converter::Gb2312ToRustString(x->DepartmentID);
    return y;
}

CThostFtdcQryDepartmentUserField Converter::QryDepartmentUserFieldToCpp(QryDepartmentUserField x) {
    CThostFtdcQryDepartmentUserField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.BrokerID, x.BrokerID.c_str());
    return y;
}

QryDepartmentUserField Converter::CThostFtdcQryDepartmentUserFieldToRust(CThostFtdcQryDepartmentUserField* x) {
    if (x == nullptr)
        return QryDepartmentUserField{.is_null = true};
    QryDepartmentUserField y{};
    y.BrokerID = Converter::Gb2312ToRustString(x->BrokerID);
    return y;
}

CThostFtdcFrontInfoField Converter::FrontInfoFieldToCpp(FrontInfoField x) {
    CThostFtdcFrontInfoField y;
    memset(&y, 0, sizeof(y));
    strcpy(y.FrontAddr, x.FrontAddr.c_str());
    y.QryFreq = x.QryFreq;
    y.FTDPkgFreq = x.FTDPkgFreq;
    return y;
}

FrontInfoField Converter::CThostFtdcFrontInfoFieldToRust(CThostFtdcFrontInfoField* x) {
    if (x == nullptr)
        return FrontInfoField{.is_null = true};
    FrontInfoField y{};
    y.FrontAddr = Converter::Gb2312ToRustString(x->FrontAddr);
    y.QryFreq = x->QryFreq;
    y.FTDPkgFreq = x->FTDPkgFreq;
    return y;
}
