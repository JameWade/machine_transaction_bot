mod fetch_data;
mod kline_4h;
mod strategy;

use std::sync::{Arc, Mutex};

use binance::api::Binance;
use binance::market::Market;
use binance::model::{Kline, KlineSummary};

use chrono::{Duration, Utc};
use tokio::runtime::Runtime;
use fetch_data::{align_to_next_boundary, fetch_and_store_klines,continuously_fetch_kline_data}; // 引入具体函数
use fetch_data::{fetch_klines,ws_fetch_current_kline4h}; // 引入具体函数
use strategy::backtest_result::BacktestResult;
use strategy::trading_strategy::RangeStrategy;

use crate::kline_4h::KlineData;
use crate::strategy::trading_strategy::Backtest;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    // 使用 Mutex 来保护 Vec<Kline> 的共享访问
    static  ref   KLINE_DATA_4H: Mutex<KlineData> = Mutex::new(KlineData::new());
}
#[tokio::main]
 async fn main(){
    println!("Hello, world!");
     tokio::task::block_in_place(  ||{   let market: Market = Binance::new(None, None);
         let history_kline4h = fetch_klines(&market,"BTCUSDT","5m",Utc::now()-Duration::days(55),Utc::now(),240).unwrap();
         let mut kline_data_4h = KlineData::new();
         kline_data_4h.convert_to_kline_data(history_kline4h);
         println!("{:#?}",kline_data_4h.len());
         ws_fetch_current_kline4h(&mut kline_data_4h,process_kline_data4h)
     });
}
fn process_kline_data4h(kline_data:&mut KlineData, kline: Kline){
    kline_data.add_kline(kline.clone());
    let data_ema55 = kline_data.calculate_ema(55).unwrap();
    println!("EMA55:{}", data_ema55);
    let data_ema20 = kline_data.calculate_ema(20).unwrap();
    println!("EMA20:{}", data_ema20);
    let  rsi_vec = kline_data.calculate_rsi(6).unwrap();
    let  rsi = rsi_vec.last().unwrap();
    println!("rsi:{}", rsi);

    // let mut backtest_result = BacktestResult::new();
    // let close_price = kline_data.kline_vec.pop().unwrap().close.parse::<f64>().unwrap();
    // backtest_result.evaluate_signal(data_ema20, data_ema55, *rsi, close_price);
    // backtest_result.print_result();

    let strategy = Box::new(RangeStrategy::new(66400.0, 66550.0));
    let mut backtest = Backtest::new(strategy);
    let close_value: &str = &kline.close;
    let close = close_value.parse::<f64>().unwrap();
    let result = backtest.run(close);
    result.print_result();
}
