use binance::model::{Kline, KlineSummary};
// 回调函数类型
pub type Kline4hCallback = fn(&mut KlineData4h,Kline);
#[derive(Clone, Debug)]
pub struct KlineData4h{
    kline_vec:Vec<Kline>
}
impl KlineData4h {
    pub(crate) fn new() ->Self{
        KlineData4h{kline_vec: Vec::new()}
    }

    pub(crate) fn add_kline(&mut self, kline: Kline){
        let last_kline = self.kline_vec.last().unwrap();
        println!("{}",last_kline.open_time);
         if kline.open_time == last_kline.open_time{
            self.kline_vec.pop();
            self.kline_vec.push(kline)
        }else {
             self.kline_vec.push(kline);
         }
        if self.kline_vec.len()>55{
            self.kline_vec.remove(0);
        }
    }

    pub(crate)  fn calculate_average4h(&self) ->Option<f64>{
        if self.kline_vec.is_empty() {
            return None;
        }

        let sum: f64 = self.kline_vec.iter().map(|kline| kline.close.parse::<f64>().unwrap()).sum();
        Some(sum / self.kline_vec.len() as f64)
    }
    pub(crate) fn convert_to_kline_data4h(&mut self, kline_summaries:Vec<KlineSummary>) {

        let klines: Vec<Kline> = kline_summaries
            .into_iter()
            .map(|summary| {
                Kline {
                    open_time: summary.open_time,
                    open: summary.open,
                    high: summary.high,
                    low: summary.low,
                    close: summary.close,
                    volume: summary.volume,
                    number_of_trades: summary.number_of_trades,
                    is_final_bar: false,
                    quote_asset_volume: summary.quote_asset_volume,
                    taker_buy_base_asset_volume: summary.taker_buy_base_asset_volume,
                    taker_buy_quote_asset_volume: summary.taker_buy_quote_asset_volume,
                    close_time: summary.close_time,
                    symbol: "".to_string(),
                    interval: "".to_string(),
                    first_trade_id: 0,
                    last_trade_id: 0,
                    ignore_me: "".to_string(),
                }
            })
            .collect();

       self.kline_vec=klines ;
    }
    
}