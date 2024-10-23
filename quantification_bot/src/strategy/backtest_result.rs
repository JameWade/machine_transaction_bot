use crate::kline_4h::KlineData;

pub(crate) struct BacktestResult {
    pub(crate) position: Option<String>,  // 记录当前是否有持仓
    pub(crate) entry_price: f64,          // 记录进入仓位时的价格
    pub(crate) profit_loss: f64,          // 总盈亏
    pub(crate) total_trades: usize,       // 交易总次数
    pub(crate) win_trades: usize,         // 赢的交易次数
    pub(crate) loss_trades: usize,        // 输的交易次数
}

impl BacktestResult {
    pub(crate) fn new() -> Self {
        BacktestResult {
            position: None,
            entry_price: 0.0,
            profit_loss: 0.0,
            total_trades: 0,
            win_trades: 0,
            loss_trades: 0,
        }
    }

    pub(crate) fn evaluate_signal(&mut self, ema20: f64, ema55: f64, rsi: f64, close_price: f64) {
        match &self.position {
            // 如果没有持仓，检查是否符合买入信号
            None => {
                if ema20 > ema55 && rsi < 30.0 {
                    self.position = Some("long".to_string());
                    self.entry_price = close_price;
                    println!("Buy at price: {}", close_price);
                }
            }
            Some(pos) => {
                if pos == "long" && ema20 < ema55 && rsi > 70.0 {
                    let profit = close_price - self.entry_price;
                    self.profit_loss += profit;
                    self.total_trades += 1;

                    if profit > 0.0 {
                        self.win_trades += 1;
                        println!("Sell with profit: {}", profit);
                    } else {
                        self.loss_trades += 1;
                        println!("Sell with loss: {}", profit);
                    }

                    // 清空仓位
                    self.position = None;
                }
            }
        }
    }

    pub(crate) fn print_result(&self) {
        let win_rate = if self.total_trades > 0 {
            (self.win_trades as f64 / self.total_trades as f64) * 100.0
        } else {
            0.0
        };

        println!("Total Profit/Loss: {}", self.profit_loss);
        println!("Total Trades: {}", self.total_trades);
        println!("Win Trades: {}", self.win_trades);
        println!("Loss Trades: {}", self.loss_trades);
        println!("Win Rate: {:.2}%", win_rate);
    }
}
