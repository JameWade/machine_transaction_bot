pub mod config;

pub mod models;
pub mod dex;
pub mod cex;

pub mod bot;
mod error;
mod types;

pub use error::ArbitrageError;
pub use types::{ OrderSide};