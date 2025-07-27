use ctp_rs::{MdApi, MdSpiMsg, ReqUserLoginField};
use std::sync::{Arc, mpsc::channel};

// for more fronts, see: http://www.openctp.cn/simenv.html
const FRONT_ADDR: &str = "tcp://210.14.72.12:4602";
const FLOW_PATH: &str = "MdFlow/";
const INSTRUMENTS: &[&str] = &["au2509", "cu2509"];

fn main() {
    let (tx, rx) = channel();
    let api = Arc::new(MdApi::CreateMdApiAndSpi(tx, FLOW_PATH.to_string()));
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
                    let len = instruments.len() as i32;
                    api.SubscribeMarketData(instruments, len);
                }
            }
            MdSpiMsg::OnRtnDepthMarketData(tick) => {
                println!("{:?}", tick);
            }
            _ => {}
        }
    }
}
