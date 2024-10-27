use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use mpl_token_metadata::accounts::Metadata;
use std::str::FromStr;

use solana_client::rpc_client::RpcClient;


use crate::models::{AmmState, PoolConfig, PoolState, StateConversion};

///https://github.com/JulianIrigoyen/solana-sniper/blob/26996e57cd6bf3b497429150e8f961a45d05775d/src/main.rs#L401
#[derive(Debug)]
pub enum RaydiumError {
    InvalidPoolData,
    RpcError(String),
    ConversionError(&'static str),
}

impl fmt::Display for RaydiumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RaydiumError::InvalidPoolData => write!(f, "Invalid pool data"),
            RaydiumError::RpcError(e) => write!(f, "RPC error: {}", e),
            RaydiumError::ConversionError(e) => write!(f, "Conversion error: {}", e),
        }
    }
}

impl Error for RaydiumError {}
pub struct RaydiumClient {
    rpc_client: RpcClient,
}

impl RaydiumClient {
    // 初始化 RaydiumClient，指定 Solana 的 RPC 连接地址
    pub fn new(rpc_url: &str) -> Self {
        RaydiumClient {
            rpc_client: RpcClient::new(rpc_url.to_string()),
        }
    }

    // 获取池子状态
    pub async fn get_pool_state(&self, config: &PoolConfig) -> Result<PoolState, Box<dyn Error>> {
        // 获取池子数据
        let pool_data = self.rpc_client
            .get_account_data(&config.pool_id)
            .map_err(|e| RaydiumError::RpcError(e.to_string()))?;
        // 解析AMM状态
        let amm_state = AmmState::try_from_bytes(&pool_data)
            .map_err(RaydiumError::ConversionError)?;

        // 转换为PoolState
        Ok(PoolState {
            base_reserve: amm_state.base_reserve,
            quote_reserve: amm_state.quote_reserve,
            last_update_timestamp: amm_state.last_update_timestamp,
            lp_supply: amm_state.lp_supply,
        })
    }

    // 计算当前价格 (base/quote)
    pub async fn get_pool_price(&self, config: &PoolConfig) -> Result<f64, Box<dyn Error>> {
        let pool_state = self.get_pool_state(config).await?;

        // 考虑代币精度进行价格计算
        let base_factor = 10f64.powi(config.base_decimals as i32);
        let quote_factor = 10f64.powi(config.quote_decimals as i32);

        let price = (pool_state.quote_reserve as f64 / quote_factor)
            / (pool_state.base_reserve as f64 / base_factor);

        Ok(price)
    }

    // 获取池子深度信息
    pub async fn get_pool_depth(&self, config: &PoolConfig) -> Result<(f64, f64), Box<dyn Error>> {
        let pool_state = self.get_pool_state(config).await?;

        // 转换为实际数量（考虑精度）
        let base_depth = pool_state.base_reserve as f64
            / 10f64.powi(config.base_decimals as i32);
        let quote_depth = pool_state.quote_reserve as f64
            / 10f64.powi(config.quote_decimals as i32);

        Ok((base_depth, quote_depth))
    }
    pub async fn get_account(&self, config: &PoolConfig) -> (String,String) {
        // 获取池子数据
        let pool_data = self.rpc_client
            .get_account_data(&config.pool_id);
        println!("{:?}",pool_data);
        let (token_name, token_symbol) = match pool_data {
            Ok(account_data) => match Metadata::from_bytes(&account_data) {
                Ok(metadata) => {
                    // println!("[[METADATA]] {:?}", metadata);
                    (metadata.name, metadata.symbol)
                },
                Err(e) => {
                    eprintln!("Error while parsing metadata: {:?}", e);
                    // Default to "Unknown" if there's an error parsing metadata
                    ("Unknown".to_string(), "Unknown".to_string())
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch account data: {:?}", e);
                // Same
                ("Unknown".to_string(), "Unknown".to_string())
            }
        };
        (token_name, token_symbol)
    }
}