#![feature(duration_constructors)]

mod fetch_data;

use std::sync::{Arc, Mutex};

use binance::api::Binance;
use binance::market::Market;
use fetch_data::get_all_data_from_online; // 引入具体函数
use tokio::time::{sleep, Duration as TokioDuration};




#[tokio::main]
async fn main(){
    println!("Hello, world!");
    let data = Arc::new(Mutex::new(Vec::new()));
    let market: Market = Binance::new(None, None);
    let data_clone = Arc::clone(&data);
    // tokio::spawn(async move {
    //     get_all_data_from_online(data_clone, &market, "btcusdt", "15m").await.expect("TODO: panic message");
    // });


    get_all_data_from_online(data_clone, &market, "btcusdt", "15m").await;

    // 在这里可以使用数据，示例使用简单的循环
    // loop {
    //     sleep(TokioDuration::from_secs(10)).await;
    //     let data_lock = data.lock().unwrap();
    //     println!("Current data: {:?}", *data_lock);
    // }
}



