use ctp_rs::{MdApi, MdSpiMsg, ReqUserLoginField};
use std::sync::{Arc, mpsc::channel};

// for more fronts, see: http://www.openctp.cn/simenv.html
const FRONT_ADDR: &str = "tcp://...";
const FLOW_PATH: &str = "MdFlow/";
const INSTRUMENTS: &[&str] = &["...", "..."];

fn main() {
    let (tx, rx) = channel();
    let api = Arc::new(MdApi::CreateMdApiAndSpi(
        tx,
        FLOW_PATH.to_string(),
        false,
        false,
        true,
    ));
    api.RegisterFront(FRONT_ADDR.to_string());
    api.Init();

    loop {
        let msg = rx.recv().unwrap();
        match msg {
            MdSpiMsg::OnFrontConnected => {
                println!("front connected");
                let mut req = ReqUserLoginField::default();
                req.BrokerID = "".to_string();
                req.UserID = "".to_string();
                req.Password = "".to_string();
                api.ReqUserLogin(req, 0);
            }
            MdSpiMsg::OnRspUserLogin(_, rsp_info, _, _) => {
                if rsp_info.ErrorID != 0 {
                    println!("user login failed: {:?}", rsp_info);
                    continue;
                } else {
                    println!("user login success: {:?}", rsp_info);
                    let instruments: Vec<String> =
                        INSTRUMENTS.iter().map(|&s| s.to_string()).collect();
                    api.SubscribeMarketData(instruments);
                }
            }
            MdSpiMsg::OnRtnDepthMarketData(tick) => {
                println!("{:?}", tick);
            }
            _ => {}
        }
    }
}
