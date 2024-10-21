use crate::kline_4h::KlineData;

pub(crate) struct BacktestResult {
    pub(crate) total_trades: usize,
    pub(crate) total_profit: f64,
    pub(crate) max_drawdown: f64,
}

impl BacktestResult{
    // pub(crate) fn backtest(kline_data: &KlineData) -> Self {
    //
    //
    //     Self {
    //         total_trades,
    //         total_profit,
    //         max_drawdown,
    //     }
    // }
}
