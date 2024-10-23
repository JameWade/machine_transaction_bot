use binance::api::Binance;
// use binance::market::Market;
// use binance::model::KlineSummary;
// use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
// use dotenvy::dotenv;
// use std::env;
// use sqlx::Row;
use tokio::task;
//
// #[cfg_attr(not(debug_assertions), allow(dead_code))]
// #[derive(Debug)]
// #[derive(sqlx::FromRow)]
// struct MyKlineSummary {
//     pub open_time: Option<i64>,
//
//     pub open: Option<String>,
//
//     pub high: Option<String>,
//
//     pub low: Option<String>,
//
//     pub close: Option<String>,
//
//     pub volume: Option<String>,
//
//     pub close_time: Option<i64>,
//
//     pub quote_asset_volume: Option<String>,
//
//     pub number_of_trades: Option<i64>,
//
//     pub taker_buy_base_asset_volume: Option<String>,
//
//     pub taker_buy_quote_asset_volume: Option<String>,
// }
//
//https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d&startTime=1346198400000&endTime=1704067200000&limit=1000
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("########GET BTC PRICE!##############");
//
//     // Load environment variables
//     dotenv().ok();
//     match env::var("DATABASE_URL") {
//         Ok(url) => println!("DATABASE_URL: {}", url),
//         Err(_) => println!("DATABASE_URL is not set"),
//     }
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//
//     // Spawn a blocking task to get data from Binance
//     let klines = task::spawn_blocking(move || {
//         let market: Market = Binance::new(None, None);
//         get_all_data_from_online(&market)
//         // get_data_from_online(&market)
//     }).await??;
//
//     // Store the data in the database
//     store_data_in_db(&database_url, klines).await?;
//
//     println!("Data processing completed successfully");
//     Ok(())
// }

// fn get_data_from_online(market: &Market) -> Result<Vec<KlineSummary>, binance::errors::Error> {
//     // last 10 1-day klines (candlesticks) for a symbol:
//     let klines = market.get_klines("BTCUSDT", "1d", None, None, None)?;
//
//     match klines {
//         binance::model::KlineSummaries::AllKlineSummaries(klines) => {
//             println!("Fetched {} klines", klines.len());
//             Ok(klines)
//         }
//     }
// }
//
// fn get_all_data_from_online(market: &Market) -> Result<Vec<KlineSummary>, String> {
//     let mut all_klines = Vec::new();
//     let mut start_time: Option<u64> = Some(1502928000000);  //2017.8.17
//     let symbol = "BTCUSDT";
//     let interval = "1d";
//
//
//     loop {
//         match market.get_klines(symbol, interval, Some(1000), start_time, None) {
//             Ok(klines) => match klines {
//                 binance::model::KlineSummaries::AllKlineSummaries(mut klines) => {
//                     //println!("klines:{:?}",klines);
//                     if klines.len() < 1000 {
//                         all_klines.append(&mut klines);
//                         break;  // No more data to fetch
//                     }
//
//                     // Get the time of the last kline in this batch
//                     start_time = Some(klines.last().unwrap().close_time as u64);
//
//                     all_klines.append(&mut klines);
//
//                     println!("kline num:{:?}", all_klines.len());
//                 }
//             },
//             Err(e) => return Err(format!("Failed to get data: {}", e)),
//         }
//     }
//
//     Ok(all_klines)
// }
//
// pub async  fn store_data_in_db(database_url: &str, data: Vec<KlineSummary>) -> Result<(), sqlx::Error> {
//     let pool = MySqlPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await?;
//
//     for kline in data {
//         sqlx::query!(
//             r#"INSERT INTO btc_daily_data
//                (open_time, open, high, low, close, volume, close_time,
//                 quote_asset_volume, number_of_trades, taker_buy_base_asset_volume,
//                 taker_buy_quote_asset_volume)
//                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
//             kline.open_time as i64,
//             kline.open,
//             kline.high,
//             kline.low,
//             kline.close,
//             kline.volume,
//             kline.close_time as i64,
//             kline.quote_asset_volume,
//             kline.number_of_trades,
//             kline.taker_buy_base_asset_volume,
//             kline.taker_buy_quote_asset_volume,
//         )
//             .execute(&pool)
//             .await?;
//     }
//
//     println!("Data stored successfully");
//     Ok(())
// }
//
// pub async fn query_data_from_db(database_url: &str) -> Result<Vec<KlineSummary>, sqlx::Error> {
//     let pool = MySqlPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await?;
//
//     let rows = sqlx::query(
//         r#"SELECT
//             open_time,
//             open,
//             high,
//             low,
//             close,
//             volume,
//             close_time,
//             quote_asset_volume,
//             number_of_trades,
//             taker_buy_base_asset_volume,
//             taker_buy_quote_asset_volume
//         FROM btc_daily_data"#
//     )
//         .fetch_all(&pool)
//         .await?;
//
//     let data = process_rows(rows)?;
//
//     Ok(data)
// }
// fn process_rows(rows: Vec<sqlx::mysql::MySqlRow>) -> Result<Vec<KlineSummary>, sqlx::Error> {
//     let mut data = Vec::new();
//
//     for row in rows.iter() {
//         let kline_summary = KlineSummary {
//             open_time: parse_i64(row.try_get("open_time"), 0),
//             open: get_string(row, "open"),
//             high: get_string(row, "high"),
//             low: get_string(row, "low"),
//             close: get_string(row, "close"),
//             volume: get_string(row, "volume"),
//             close_time: parse_i64(row.try_get("close_time"), 0),
//             quote_asset_volume: get_string(row, "quote_asset_volume"),
//             number_of_trades: parse_i64(row.try_get("number_of_trades"), 0),
//             taker_buy_base_asset_volume: get_string(row, "taker_buy_base_asset_volume"),
//             taker_buy_quote_asset_volume: get_string(row, "taker_buy_quote_asset_volume"),
//         };
//
//         data.push(kline_summary);
//     }
//
//     Ok(data)
// }
// // 辅助函数：安全地解析 i64，出错时返回默认值
// fn parse_i64(value: Result<i64, sqlx::Error>, default: i64) -> i64 {
//     value.unwrap_or(default)
// }
//
// // 辅助函数：安全地获取字符串，出错时返回空字符串
// fn get_string(row: &MySqlRow, index: &str) -> String {
//     row.try_get(index).unwrap_or_else(|_| String::new())
// }
// #[cfg(test)]
// mod test {
//     use std::env;
//     use dotenvy::dotenv;
//     use crate::query_data_from_db;
//
//     #[tokio::test]
//     async fn test_btc_day_data()-> Result<(), Box<dyn std::error::Error>> {
//         // Add your test implementation here
//         dotenv().ok();
//
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//
//         let data = query_data_from_db(&database_url).await?;
//         data.iter().for_each(|day_data|{
//             println!("{:?}", day_data);
//
//         });
//         Ok(())
//     }
// }