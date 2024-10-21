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


use crate::kline_4h::KlineData;


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
         let history_kline4h = fetch_klines(&market,"BTCUSDT","15m",Utc::now()-Duration::days(55),Utc::now(),240).unwrap();
         let mut kline_data_4h = KlineData::new();
         kline_data_4h.convert_to_kline_data(history_kline4h);
         println!("{:#?}",kline_data_4h.len());
         ws_fetch_current_kline4h(&mut kline_data_4h,process_kline_data4h)
     });
}
fn process_kline_data4h(kline_data_4h:&mut KlineData, kline: Kline){
    kline_data_4h.add_kline(kline);
    let data_ema55 = kline_data_4h.calculate_ema(55).unwrap();
    println!("EMA55:{}", data_ema55);
    let data_ema20 = kline_data_4h.calculate_ema(20).unwrap();
    println!("EMA20:{}", data_ema20);
    let  rsi = kline_data_4h.calculate_rsi(6).unwrap();
    println!("rsi:{}", rsi.last().unwrap());

    // // 运行回测
    // let result = BacktestResult::backtest(&kline_data_4h);
    //
    // // 输出回测结果
    // println!("总交易次数: {}", result.total_trades);
    // println!("总盈利: {}", result.total_profit);
    // println!("最大回撤: {}", result.max_drawdown);

}
