use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArbitrageError {
    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    #[error("Price slippage: {0}")]
    PriceSlippage(String),
}

pub type Result<T> = std::result::Result<T, ArbitrageError>;