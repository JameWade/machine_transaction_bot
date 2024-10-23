use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use binance::market::Market;
use binance::model::{Kline, KlineSummary};
use binance::websockets::{WebsocketEvent, WebSockets};
use tokio::time::Duration as StdDuration;
use chrono::{DateTime, Duration, Utc};
use tokio::time::sleep;
use crate::kline_4h::{Kline4hCallback, KlineData};

pub    fn continuously_fetch_kline_data(
    data: Arc<Mutex<Vec<KlineSummary>>>,
    market: &Market,
    symbol: &str,
    interval: &str,
) -> Result<(), String> {
    let mut end_time = align_to_next_boundary(Utc::now(), 15);
    let initial_start_time = end_time - Duration::days(7);
    let rt = tokio::runtime::Builder::new_current_thread()
     .enable_all()
     .build().unwrap();
    // 初始获取
    fetch_and_store_klines(data.clone(), market, symbol, interval, initial_start_time, end_time, 1000)?;


    loop {
        // sleep(StdDuration::from_secs(15 * 60)).await;
        rt.block_on(wait_15m());
        let last_fetch_time = end_time;
        end_time = align_to_next_boundary(Utc::now(), 15);

        // 增量获取
        fetch_and_store_klines(data.clone(), market, symbol, interval, last_fetch_time, end_time, 1)?;
    }
}
 pub fn fetch_and_store_klines(
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
 pub fn fetch_klines(
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
         //datetime_to_option_unix_timestamp(start_time),
         None,
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
pub fn align_to_next_boundary(datetime: DateTime<Utc>, interval_minutes: i64) -> DateTime<Utc> {
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

async fn wait_15m(){
    sleep(StdDuration::from_secs(15 * 60)).await;
}
pub fn ws_fetch_current_kline4h(kline_data_4h:&mut KlineData, callback:Kline4hCallback){
    // 启动 WebSocket 监听
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline = format!("{}", "btcusdt@kline_5m");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                println!("Symbol: {}, high: {}, low: {}", kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high);
                callback(kline_data_4h, kline_event.kline);
            },
            _ => (),
        };
        Ok(())
    });
    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => {
                println!("Error: {:?}", err);
            }
        }
    }
    web_socket.disconnect().unwrap();
}
