use std::error::Error;

pub trait CexInterface {
    async fn get_price(&self, symbol: &str) -> Result<f64, Box<dyn Error>>;
    async fn place_order(&self, symbol: &str, side: OrderSide, amount: f64, price: f64) -> Result<String, Box<dyn Error>>;
    async fn get_balance(&self, asset: &str) -> Result<f64, Box<dyn Error>>;
}