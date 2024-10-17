mod fetch_data;
mod kline_4h;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use binance::api::Binance;
use binance::market::Market;
use binance::model::{Kline, KlineSummary};
use binance::websockets::{WebsocketEvent, WebSockets};
use chrono::{Duration, Utc};
use tokio::runtime::Runtime;
use fetch_data::{align_to_next_boundary, fetch_and_store_klines,continuously_fetch_kline_data}; // 引入具体函数
use fetch_data::{fetch_klines,ws_fetch_current_kline4h}; // 引入具体函数

use tokio::time::{sleep, Duration as StdDuration};
use crate::kline_4h::KlineData4h;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    // 使用 Mutex 来保护 Vec<Kline> 的共享访问
    static  ref   KLINE_DATA_4H: Mutex<KlineData4h> = Mutex::new(KlineData4h::new());
}
#[tokio::main]
 async fn main(){
    println!("Hello, world!");
    // let data = Arc::new(Mutex::new(Vec::<KlineSummary>::new()));
    // let data_clone = Arc::clone(&data);
    //  let handle =  tokio::task::spawn_blocking( move || {
    //      let market: Market = Binance::new(None, None);
    //      continuously_fetch_kline_data(data_clone, &market, "BTCUSDT", "15m");
    //  });
    //  handle.await.unwrap();
    //  println!("{}", "here");
    // {
    //     let data = data.lock().unwrap();
    //     for kline in data.iter() {
    //         println!("{:?}", kline);
    //     }
    // }
    // tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    // println!("Shutting down...");

     tokio::task::block_in_place(  ||{   let market: Market = Binance::new(None, None);
         let history_kline4h = fetch_klines(&market,"BTCUSDT","4h",Utc::now()-Duration::days(7),Utc::now(),55).unwrap();

         let mut kline_data_4h = KlineData4h::new();
         kline_data_4h.convert_to_kline_data4h(history_kline4h);
         println!("{:#?}",kline_data_4h);
         ws_fetch_current_kline4h(&mut kline_data_4h,process_kline_data4h)
     });
}
fn process_kline_data4h(  kline_data_4h:&mut KlineData4h, kline: Kline){
    kline_data_4h.add_kline(kline);
    let data = kline_data_4h.calculate_average4h().unwrap();
    println!("{}",data);
}
