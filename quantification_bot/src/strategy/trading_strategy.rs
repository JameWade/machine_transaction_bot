use crate::strategy::backtest_result::BacktestResult;

pub trait TradingStrategy {
    fn update(&mut self, price: f64) -> Option<String>; // 返回交易信号
    fn get_backtest_result(&self) -> &BacktestResult;
}

// 区间交易策略实现
pub struct RangeStrategy {
    lower_bound: f64,
    upper_bound: f64,
    result: BacktestResult,
}

impl RangeStrategy {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        RangeStrategy {
            lower_bound,
            upper_bound,
            result: BacktestResult::new(),
        }
    }

    fn update_trade_stats(&mut self, exit_price: f64) {
        if let Some(pos) = &self.result.position {
            self.result.total_trades += 1;

            let trade_profit = if pos == "long" {
                exit_price - self.result.entry_price
            } else {
                self.result.entry_price - exit_price
            };

            if trade_profit > 0.0 {
                self.result.win_trades += 1;
            } else {
                self.result.loss_trades += 1;
            }

            self.result.profit_loss += trade_profit;
        }
    }
}

impl TradingStrategy for RangeStrategy {
    fn update(&mut self, price: f64) -> Option<String> {
        let signal = match &self.result.position {
            None => {
                if price <= self.lower_bound {
                    self.result.position = Some("long".to_string());
                    self.result.entry_price = price;
                    Some("buy".to_string())
                } else {
                    None
                }
            },
            Some(pos) if pos == "long" => {
                if price >= self.upper_bound {
                    self.update_trade_stats(price);
                    self.result.position = None;
                    Some("sell".to_string())
                } else {
                    None
                }
            },
            _ => None,
        };

        signal
    }

    fn get_backtest_result(&self) -> &BacktestResult {
        &self.result
    }
}


pub struct Backtest {
    strategy: Box<dyn TradingStrategy>,
}

impl Backtest {
    pub fn new(strategy: Box<dyn TradingStrategy>) -> Self {
        Backtest { strategy }
    }

    pub fn run(&mut self, price: f64) -> &BacktestResult {
            let signal = self.strategy.update(price);
            if let Some(action) = signal {
                println!("Price: {}, Action: {}", price, action);
            }

        self.strategy.get_backtest_result()
    }
}
