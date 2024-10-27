use std::error::Error;
use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

#[async_trait]
pub trait DexInterface {
    async fn get_pool_price(&self, pool_address: &Pubkey) -> Result<f64, Box<dyn Error>>;
    async fn swap(&self, pool_address: &Pubkey, amount_in: u64, min_amount_out: u64) -> Result<Transaction, Box<dyn Error>>;
}