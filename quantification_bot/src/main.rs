mod fetch_data;

use std::sync::{Arc, Mutex};

use binance::api::Binance;
use binance::market::Market;
use fetch_data::continuously_fetch_kline_data; // 引入具体函数
use tokio::time::{sleep, Duration as TokioDuration};




#[tokio::main]
async fn main(){
    println!("Hello, world!");
    let data = Arc::new(Mutex::new(Vec::new()));
    let market: Market = Binance::new(None, None);
    let data_clone = Arc::clone(&data);

    // 使用 tokio::spawn 来运行异步任务
   continuously_fetch_kline_data(data_clone, &market, "btcusdt", "15m");


}



