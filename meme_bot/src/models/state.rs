use {
    bytemuck::{Pod, Zeroable},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct AmmState {
    // 状态标志 (8 bytes)
    pub status: u64,
    // nonce用于PDA派生 (8 bytes)
    pub nonce: u64,
    // 最大价格 (8 bytes)
    pub max_price: u64,
    // 最小价格 (8 bytes)
    pub min_price: u64,
    // LP代币发行量 (8 bytes)
    pub lp_supply: u64,
    // 累计费用基础代币数量 (8 bytes)
    pub accumulated_base_fees: u64,
    // 累计费用报价代币数量 (8 bytes)
    pub accumulated_quote_fees: u64,
    // 基础端波动性 (8 bytes)
    pub base_wave: u64,
    // 报价端波动性 (8 bytes)
    pub quote_wave: u64,
    // 基础代币储备量 (8 bytes)
    pub base_reserve: u64,
    // 报价代币储备量 (8 bytes)
    pub quote_reserve: u64,
    // 基础端目标值 (8 bytes)
    pub base_target: u64,
    // 报价端目标值 (8 bytes)
    pub quote_target: u64,
    // 最后更新的时间戳 (8 bytes)
    pub last_update_timestamp: i64,
    // padding to maintain proper alignment (4 bytes)
}

// 用于安全地转换字节数据到状态结构
pub trait StateConversion {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, &'static str>;
}

impl StateConversion for AmmState {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, &'static str> {
        if data.len() != size_of::<AmmState>() {
            return Err("Invalid data length for AmmState");
        }

        Ok(bytemuck::from_bytes(data))
    }
}