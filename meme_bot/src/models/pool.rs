use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;
use rust_decimal::prelude::ToPrimitive;

/// 代币信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub decimal: u8,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<Decimal>,
    pub token_account: Option<Pubkey>,
}
/// 价格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceInfo {
    pub price: Decimal,
    pub timestamp: i64,
    pub slippage: Decimal,
    pub volume_24h: Option<Decimal>,
    pub updated_at: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub owner: Pubkey,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}
/// 池子方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolDirection {
    Obverse, // 正向 token_a -> token_b
    Reverse, // 反向 token_b -> token_a
}
/// 池子状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolState {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub last_updated: i64,
    pub status: PoolStatus,
    pub price_history: Vec<PriceInfo>,
    pub volume_24h: Decimal,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PoolStatus {
    Active,
    Paused,
    Frozen,
    Deprecated,
}
/// 完整的池子信息
#[derive(Debug, Clone)]
pub struct PoolInfo {
    pub address: Pubkey,
    pub amm_id: Pubkey,
    pub token_a: TokenInfo,
    pub token_b: TokenInfo,
    pub lp_mint: Pubkey,
    pub config: PoolConfig,
    pub state: PoolState,

    // Raydium 特有字段
    pub serum_market: Option<Pubkey>,
    pub serum_program_id: Option<Pubkey>,
    pub serum_bids: Option<Pubkey>,
    pub serum_asks: Option<Pubkey>,

    // 缓存的计算结果
    #[serde(skip)]
    cache: HashMap<String, Decimal>,
}
impl PoolInfo {
    pub fn new(
        address: Pubkey,
        amm_id: Pubkey,
        token_a: TokenInfo,
        token_b: TokenInfo,
        lp_mint: Pubkey,
        config: PoolConfig,
        state: PoolState,
    ) -> Self {
        Self {
            address,
            amm_id,
            token_a,
            token_b,
            lp_mint,
            config,
            state,
            serum_market: None,
            serum_program_id: None,
            serum_bids: None,
            serum_asks: None,
            cache: HashMap::new(),
        }
    }

    /// 计算当前池子价格
    pub fn get_price(&self, direction: PoolDirection) -> Decimal {
        match direction {
            PoolDirection::Obverse => {
                Decimal::from(self.state.token_b_amount) / Decimal::from(self.state.token_a_amount)
            }
            PoolDirection::Reverse => {
                Decimal::from(self.state.token_a_amount) / Decimal::from(self.state.token_b_amount)
            }
        }
    }

    /// 计算交易滑点
    pub fn calculate_slippage(
        &self,
        input_amount: u64,
        direction: PoolDirection,
    ) -> (Decimal, Decimal) {
        let (pool_in, pool_out) = match direction {
            PoolDirection::Obverse => (self.state.token_a_amount, self.state.token_b_amount),
            PoolDirection::Reverse => (self.state.token_b_amount, self.state.token_a_amount),
        };

        let k = Decimal::from(pool_in) * Decimal::from(pool_out);
        let new_pool_in = Decimal::from(pool_in) + Decimal::from(input_amount);
        let new_pool_out = k / new_pool_in;

        let output_amount = Decimal::from(pool_out) - new_pool_out;
        let price_impact = (new_pool_out - Decimal::from(pool_out)) / Decimal::from(pool_out);

        (output_amount, price_impact)
    }

    /// 计算交易费用
    pub fn calculate_fees(&self, input_amount: u64) -> HashMap<String, u64> {
        let mut fees = HashMap::new();

        // 交易费用
        let trade_fee = input_amount * self.config.trade_fee_numerator
            / self.config.trade_fee_denominator;
        fees.insert("trade_fee".to_string(), trade_fee);

        // 所有者费用
        let owner_fee = input_amount * self.config.owner_trade_fee_numerator
            / self.config.owner_trade_fee_denominator;
        fees.insert("owner_fee".to_string(), owner_fee);

        // 主机费用
        let host_fee = input_amount * self.config.host_fee_numerator
            / self.config.host_fee_denominator;
        fees.insert("host_fee".to_string(), host_fee);

        fees
    }

    /// 检查池子是否健康
    pub fn is_healthy(&self) -> bool {
        let price = self.get_price(PoolDirection::Obverse);
        let volume = self.state.volume_24h;

        // 检查条件：
        // 1. 池子状态为激活
        // 2. 价格在合理范围内
        // 3. 24小时交易量足够
        // 4. 最后更新时间在合理范围内
        self.state.status == PoolStatus::Active
            && price > Decimal::from(0)
            && volume > Decimal::from(1000)
            && (chrono::Utc::now().timestamp() - self.state.last_updated) < 3600
    }

    /// 更新池子状态
    pub fn update_state(&mut self, new_state: PoolState) {
        self.state = new_state;
        self.cache.clear(); // 清除缓存的计算结果
    }

    /// 获取最优交易路径
    pub fn get_optimal_swap_amount(
        &self,
        input_amount: u64,
        min_output: u64,
        direction: PoolDirection,
    ) -> Option<(u64, u64)> {
        // 检查池子状态
        if !self.is_healthy() {
            return None;
        }

        // 计算滑点和输出金额
        let (output_amount, price_impact) = self.calculate_slippage(input_amount, direction);

        // 如果价格影响太大或输出金额小于最小要求，返回 None
        if price_impact.abs() > Decimal::from_str("0.05").unwrap()
            || output_amount < Decimal::from(min_output) {
            return None;
        }

        Some((
            input_amount,
            output_amount.to_u64().unwrap_or(0),
        ))
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::Zero;
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_pool_price_calculation() {
        let pool = create_test_pool();
        let price = pool.get_price(PoolDirection::Obverse);
        assert!(price > Decimal::zero());
    }

    #[test]
    fn test_slippage_calculation() {
        let pool = create_test_pool();
        let (output, impact) = pool.calculate_slippage(1000000, PoolDirection::Obverse);
        assert!(output > Decimal::zero());
        assert!(impact < Decimal::zero()); // Price impact should be negative
    }

    fn create_test_pool() -> PoolInfo {
        PoolInfo::new(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            TokenInfo {
                mint: Pubkey::new_unique(),
                decimal: 6,
                symbol: "SOL".to_string(),
                price_usd: Some(Decimal::from(100)),
                token_account: Some(Pubkey::new_unique()),
            },
            TokenInfo {
                mint: Pubkey::new_unique(),
                decimal: 6,
                symbol: "USDC".to_string(),
                price_usd: Some(Decimal::from(1)),
                token_account: Some(Pubkey::new_unique()),
            },
            Pubkey::new_unique(),
            PoolConfig {
                fee_numerator: 25,
                fee_denominator: 10000,
                owner: Pubkey::new_unique(),
                trade_fee_numerator: 25,
                trade_fee_denominator: 10000,
                owner_trade_fee_numerator: 5,
                owner_trade_fee_denominator: 10000,
                owner_withdraw_fee_numerator: 0,
                owner_withdraw_fee_denominator: 0,
                host_fee_numerator: 0,
                host_fee_denominator: 0,
            },
            PoolState {
                token_a_amount: 1000000000,
                token_b_amount: 100000000000,
                last_updated: chrono::Utc::now().timestamp(),
                status: PoolStatus::Active,
                price_history: vec![],
                volume_24h: Decimal::from(1000000),
            },
        )
    }
}