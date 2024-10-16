mod fetch_data;

use std::sync::{Arc, Mutex};

use binance::api::Binance;
use binance::market::Market;
use binance::model::KlineSummary;
use chrono::{Duration, Utc};
use tokio::runtime::Runtime;
use fetch_data::{align_to_next_boundary, fetch_and_store_klines,continuously_fetch_kline_data}; // 引入具体函数
use fetch_data::fetch_klines; // 引入具体函数

use tokio::time::{sleep, Duration as StdDuration};






 #[tokio::main]
 async fn main(){
    println!("Hello, world!");
    let data = Arc::new(Mutex::new(Vec::<KlineSummary>::new()));
    let data_clone = Arc::clone(&data);
        //
        //
        //
        // let market: Market = Binance::new(None, None);
        // let mut end_time = align_to_next_boundary(Utc::now(), 15);
        // let initial_start_time = end_time - Duration::days(7);
        // let rt = tokio::runtime::Builder::new_current_thread()
        //  .enable_all()
        //  .build().unwrap();
        //
        // // 初始获取
        // fetch_and_store_klines(data.clone(), &market, "BTCUSDT", "15m", initial_start_time, end_time, 1000).unwrap();
        //
        //
        // loop {
        //     // sleep(StdDuration::from_secs(15 * 60)).await
        //     rt.block_on(wait_15m());
        //
        //     let last_fetch_time = end_time;
        //     end_time = align_to_next_boundary(Utc::now(), 15);
        //
        //     // 增量获取
        //     fetch_and_store_klines(data.clone(), &market, "BTCUSDT", "15m", last_fetch_time, end_time, 1).unwrap();
        // }



     tokio::task::spawn_blocking( move || {
         let market: Market = Binance::new(None, None);
         continuously_fetch_kline_data(data.clone(), &market, "BTCUSDT", "15m");
     }).await.unwrap();




    // 读取并打印数据
    {
        let data = data_clone.lock().unwrap();
        for kline in data.iter() {
            println!("{:?}", kline);
        }
    }
    // 等待 Ctrl+C 来阻止主线程过早退出
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Shutting down...");
}

//
async fn wait_15m(){
    sleep(StdDuration::from_secs(15 * 60)).await;
}

//shangmian