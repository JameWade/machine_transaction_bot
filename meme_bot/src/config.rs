use std::error::Error;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

// 配置结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) solana_rpc_url: String,
    pub(crate) private_key: String,
    dex_program_id: String,
    pub(crate) cex_api_key: String,
    pub(crate) cex_secret: String,
    pub(crate) target_token_mint: String,
    pub(crate) min_profit_threshold: f64,
}



