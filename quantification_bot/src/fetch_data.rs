use std::sync::{Arc, Mutex};
use binance::market::Market;
use binance::model::KlineSummary;
use tokio::time::Duration as StdDuration;
use chrono::{DateTime, Duration, Utc};
use tokio::time::sleep;

pub async fn continuously_fetch_kline_data(
    data: Arc<Mutex<Vec<KlineSummary>>>,
    market: &Market,
    symbol: &str,
    interval: &str,
) -> Result<(), String> {
    let mut end_time = align_to_next_boundary(Utc::now(), 15);
    let initial_start_time = end_time - Duration::days(7);

    // 初始获取
    fetch_and_store_klines(data.clone(), market, symbol, interval, initial_start_time, end_time, 1000).await?;


    loop {
        // sleep(StdDuration::from_secs(15 * 60)).await;
        let last_fetch_time = end_time;
        end_time = align_to_next_boundary(Utc::now(), 15);

        // 增量获取
        fetch_and_store_klines(data.clone(), market, symbol, interval, last_fetch_time, end_time, 1).await?;
    }
}
async fn fetch_and_store_klines(
    data: Arc<Mutex<Vec<KlineSummary>>>,
    market: &Market,
    symbol: &str,
    interval: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    limit: u16,
) -> Result<(), String> {
    let klines = fetch_klines(market, symbol, interval, start_time, end_time, limit).unwrap();

    let mut data_lock = data.lock().unwrap();
    data_lock.extend(klines);

    Ok(())
}
 fn fetch_klines(
    market: &Market,
    symbol: &str,
    interval: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    limit: u16,
) -> Result<Vec<KlineSummary>, String> {
     let data = match market.get_klines(
         symbol,
         interval,
         Some(limit),
         datetime_to_option_unix_timestamp(start_time),
         datetime_to_option_unix_timestamp(end_time),
     ) {
         Ok(binance::model::KlineSummaries::AllKlineSummaries(kline_vec)) => {
             println!("Fetched {} klines", kline_vec.len());
             Ok(kline_vec)
         }
         Err(e) => Err(format!("Failed to get data: {}", e)),
     };
     data

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