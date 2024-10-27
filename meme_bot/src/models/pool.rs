use spl_associated_token_account::solana_program::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct PoolConfig {
    // 池子地址(AMM account) - 这个是必须的,用于查询池子状态
    pub pool_id: Pubkey,
    // 代币精度 - 这两个是为了正确计算价格
    pub base_decimals: u8,
    pub quote_decimals: u8,
}

impl PoolConfig {
    pub fn new(
        pool_id: Pubkey,
        base_decimals: u8,
        quote_decimals: u8,
    ) -> Self {
        Self {
            pool_id,
            base_decimals,
            quote_decimals,
        }
    }
}

// 池子状态数据
#[derive(Debug)]
pub struct PoolState {
    // 基础代币储备量
    pub base_reserve: u64,
    // 报价代币储备量
    pub quote_reserve: u64,
    // 最新交易时间戳
    pub last_update_timestamp: i64,
    // LP代币总供应量
    pub lp_supply: u64,
}