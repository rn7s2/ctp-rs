use ctp_rs::{
    QryInstrumentField, ReqAuthenticateField, ReqUserLoginField, THOST_TE_RESUME_TYPE, TraderApi,
    TraderSpiMsg,
};
use std::sync::{Arc, mpsc::channel};

const BROKER_ID: &str = "...";
const USER_ID: &str = "...";
const PASSWORD: &str = "...";
const APP_ID: &str = "...";
const AUTH_CODE: &str = "...";
const FRONT_ADDR: &str = "...";
const FLOW_PATH: &str = "TraderFlow/";

fn main() {
    let (tx, rx) = channel();
    let api = Arc::new(TraderApi::CreateTraderApiAndSpi(tx, FLOW_PATH.to_string()));
    api.RegisterFront(FRONT_ADDR.to_string());
    api.SubscribePublicTopic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK as i32);
    api.SubscribePrivateTopic(THOST_TE_RESUME_TYPE::THOST_TERT_RESTART as i32);
    api.Init();

    loop {
        let msg = rx.recv().unwrap();
        match msg {
            TraderSpiMsg::OnFrontConnected => {
                println!("front connected");
                let req = ReqAuthenticateField {
                    BrokerID: BROKER_ID.to_string(),
                    UserID: USER_ID.to_string(),
                    AuthCode: AUTH_CODE.to_string(),
                    AppID: APP_ID.to_string(),
                    ..Default::default()
                };
                api.ReqAuthenticate(req, 0);
            }
            TraderSpiMsg::OnRspAuthenticate(_, rsp_info, _, _) => {
                if rsp_info.ErrorID != 0 {
                    println!("auth failed: {:?}", rsp_info);
                    std::process::exit(1);
                } else {
                    println!("auth success: {:?}", rsp_info);

                    let req = ReqUserLoginField {
                        BrokerID: BROKER_ID.to_string(),
                        UserID: USER_ID.to_string(),
                        Password: PASSWORD.to_string(),
                        ..Default::default()
                    };
                    api.ReqUserLogin(req, 0);
                }
            }
            TraderSpiMsg::OnRspUserLogin(_, rsp_info, _, _) => {
                if rsp_info.ErrorID != 0 {
                    println!("user login failed: {:?}", rsp_info);
                    std::process::exit(1);
                } else {
                    println!("user login success: {:?}", rsp_info);

                    api.ReqQryInstrument(QryInstrumentField::default(), 0);
                }
            }
            TraderSpiMsg::OnRspQryInstrument(instrument, rsp_info, _, _) => {
                if instrument.is_null {
                    eprintln!("qry instrument: {:?}", rsp_info);
                    std::process::exit(1);
                }

                println!("{:?}", instrument);
            }
            _ => {}
        }
    }
}
