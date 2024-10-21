use binance::model::{Kline, KlineSummary};
// 回调函数类型
pub type Kline4hCallback = fn(&mut KlineData, Kline);
#[derive(Clone, Debug)]
pub struct KlineData {
    pub(crate) kline_vec:Vec<Kline>   //if 4h ,size 24/4*period
}
impl KlineData {
    pub(crate) fn new() ->Self{
        KlineData {kline_vec: Vec::new()}
    }
    pub(crate) fn len(& self) ->usize{
        self.kline_vec.len()
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
        if self.kline_vec.len()>240{
            self.kline_vec.remove(0);
        }
    }

    pub(crate)  fn calculate_average(&self) ->Option<f64>{
        if self.kline_vec.is_empty() {
            return None;
        }

        let sum: f64 = self.kline_vec.iter().map(|kline| kline.close.parse::<f64>().unwrap()).sum();
        Some(sum / self.kline_vec.len() as f64)
    }
    pub(crate) fn convert_to_kline_data(&mut self, kline_summaries:Vec<KlineSummary>) {

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
    pub(crate) fn calculate_ema(&self, period: usize) -> Option<f64> {
        // 如果 kline_vec 为空，返回 None
        if self.kline_vec.is_empty() {
            return None;
        }

        // 如果数据不足以计算EMA，返回 None
        if self.kline_vec.len() < period {
            return None;
        }

        // 提取所有的收盘价并转换为 f64
        let closes: Vec<f64> = self.kline_vec
            .iter()
            .map(|kline| kline.close.parse::<f64>().unwrap())
            .collect();

        // 计算平滑系数 alpha
        let alpha = 2.0 / (period as f64 + 1.0);

        // 初始化：用第一个 N 个收盘价的简单移动平均作为初始 EMA
        let mut ema = closes[0..period].iter().sum::<f64>() / period as f64;

        // 从第 N 个数据点开始迭代计算 EMA
        for &price in &closes[period..] {
            ema = alpha * price + (1.0 - alpha) * ema;
        }

        Some(ema)
    }
    pub(crate) fn calculate_rsi(&self, period: usize) -> Option<Vec<f64>> {
        let prices: Vec<f64> = self.kline_vec.iter()
            .map(|kline| kline.close.parse::<f64>().unwrap())
            .collect();

        if prices.len() < period {
            return None; // 数据不足，无法计算 RSI
        }

        let mut rsi_values = Vec::new();

        let mut gains = 0.0;
        let mut losses = 0.0;

        // 计算初始平均增益和平均损失
        for i in 1..period + 1 {
            let change = prices[i] - prices[i - 1];
            if change > 0.0 {
                gains += change;
            } else {
                losses -= change;
            }
        }

        let mut avg_gain = gains / period as f64;
        let mut avg_loss = losses / period as f64;

        // 计算初始 RSI
        let rs = if avg_loss != 0.0 { avg_gain / avg_loss } else { f64::INFINITY };
        let initial_rsi = 100.0 - (100.0 / (1.0 + rs));
        rsi_values.push(initial_rsi);

        // 计算后续的 RSI 值
        for i in period + 1..prices.len() {
            let change = prices[i] - prices[i - 1];
            if change > 0.0 {
                avg_gain = (avg_gain * (period as f64 - 1.0) + change) / period as f64;
                avg_loss = (avg_loss * (period as f64 - 1.0)) / period as f64;
            } else {
                avg_gain = (avg_gain * (period as f64 - 1.0)) / period as f64;
                avg_loss = (avg_loss * (period as f64 - 1.0) - change) / period as f64;
            }

            let rs = if avg_loss != 0.0 { avg_gain / avg_loss } else { f64::INFINITY };
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            rsi_values.push(rsi);
        }

        Some(rsi_values)
    }
}