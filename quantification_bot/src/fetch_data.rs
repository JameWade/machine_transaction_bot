use std::sync::{Arc, Mutex};
use binance::market::Market;
use binance::model::KlineSummary;
use tokio::time::Duration as StdDuration;
use chrono::{DateTime, Duration, Utc};
use tokio::time::sleep;

pub async  fn get_all_data_from_online(data: Arc<Mutex<Vec<KlineSummary>>>, market: &Market, symbol:&str, interval:&str) -> Result<(), String> {
    //fetch first time
    let mut end_time = align_to_next_boundary(Utc::now(),15);
    let start_time = end_time-Duration::days(7);
    let mut all_klines =  match market.get_klines(symbol, interval, Some(1000), datetime_to_option_unix_timestamp(start_time), datetime_to_option_unix_timestamp(end_time)) {
            Ok(kline_summaries) => match kline_summaries {
                binance::model::KlineSummaries::AllKlineSummaries( kline_vec) => {
                    println!("kline num:{:?}", kline_vec.len());
                    kline_vec
                }
            },
            Err(e) => return Err(format!("Failed to get data: {}", e)),
    };

    loop {
        //share the data
        {
            let mut data_lock = data.lock().unwrap();
            data_lock.append(&mut all_klines);
        }
        //wait internals time
        sleep(StdDuration::from_mins(15)).await;
        //fetch some internals
        let mut last_fetch_time = end_time;
        end_time = align_to_next_boundary(Utc::now(),15);
        all_klines =  match market.get_klines(symbol, interval, Some(1), datetime_to_option_unix_timestamp(last_fetch_time), datetime_to_option_unix_timestamp(end_time)) {
            Ok(kline_summaries) => match kline_summaries {
                binance::model::KlineSummaries::AllKlineSummaries( kline_vec) => {
                    println!("kline num:{:?}", kline_vec.len());
                    kline_vec
                }
            },
            Err(e) => return Err(format!("Failed to get data: {}", e)),
        };
    }
}
/// 根据给定的时间间隔将时间向上取整到最近的边界
fn align_to_next_boundary(datetime: DateTime<Utc>, interval_minutes: i64) -> DateTime<Utc> {
    let interval_seconds = interval_minutes * 60; // 转换为秒
    let seconds = datetime.timestamp() % interval_seconds; // 计算余数

    if seconds == 0 {
        datetime // 如果已经是整的边界，则直接返回
    } else {
        datetime + Duration::seconds(interval_seconds - seconds) // 向上取整
    }
}
// 将 DateTime<Utc> 转换为 Option<u64>
fn datetime_to_option_unix_timestamp(dt: DateTime<Utc>) -> Option<u64> {
    Some(dt.timestamp_millis() as u64)
}