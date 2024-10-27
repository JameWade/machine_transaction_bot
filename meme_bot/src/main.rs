
use std::error::Error;
use std::str::FromStr;
use meme_bot::dex::RaydiumClient;
use meme_bot::models::PoolConfig;
use mpl_token_metadata::ID;
use solana_sdk::pubkey::Pubkey;

mod bot;
mod config;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    example_usage().await.expect("TODO: panic message");
}
async fn example_usage() -> Result<(), Box<dyn Error>>  {
    // Solana 主网的 RPC URL
    let rpc_url = "https://api.mainnet-beta.solana.com";

    // 创建一个 RaydiumClient 实例
    let raydium_client = RaydiumClient::new(rpc_url);

    // Raydium 池子的地址，例如 RAY/USDC 池
    let pool_address = "BWCeEqb7naeoHeQgFdxDjUpywCoBFHzmBCe3UAuviFYC";

    // 获取池子信息
    let pool = get_sol_usdc_pool_config();
    let (token_name, token_symbol) = raydium_client.get_account(&pool).await;


    print!("{:?}",(token_name,token_symbol));
    Ok(())
}
pub fn get_sol_usdc_pool_config() -> PoolConfig {
    let token_account_pubkey = Pubkey::from_str("4amstKcbziHCqwev9esMtRGDTdjHSviiNXT7WtajgjUq").unwrap();
    let metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();;
    let token_mint_address = Pubkey::from_str("4amstKcbziHCqwev9esMtRGDTdjHSviiNXT7WtajgjUq").unwrap();
    let (metadata_account_address, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            metadata_program_id.as_ref(),
            token_mint_address.as_ref(),
        ],
        &metadata_program_id,
    );
    PoolConfig::new(
        // SOL-USDC pool address on mainnet
        // Pubkey::from_str("4amstKcbziHCqwev9esMtRGDTdjHSviiNXT7WtajgjUq").unwrap(),
        metadata_account_address,
        9,  // SOL decimals
        6   // USDC decimals
    )
}