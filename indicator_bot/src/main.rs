use std::env;
use binance::model::KlineSummary;
use dotenvy::dotenv;
use database::query_data_from_db;
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let data = query_data_from_db(&database_url).await?;

    data.iter().for_each(|day_data|{
        println!("{:?}", day_data);

    });

    Ok(())

}