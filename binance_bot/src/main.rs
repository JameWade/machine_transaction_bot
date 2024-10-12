use std::env;
use binance::model::KlineSummary;
use dotenvy::dotenv;
use linfa::Dataset;
use linfa::metrics::ToConfusionMatrix;
use linfa::traits::{Fit, Predict};
use linfa_bayes::GaussianNb;
use ndarray::{Array, Array1, Array2, ArrayBase, Ix1, Ix2, OwnedRepr, s};
use database::query_data_from_db;
//camp model
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let data = query_data_from_db(&database_url).await?;
   deal_data(data);
    // data.iter().for_each(|day_data|{
    //     println!("{:?}", day_data);
    //
    // });

    Ok(())

}

fn deal_data(data: Vec<KlineSummary>) {
    let  features: Vec<f64> = data.iter()
        .skip(1)
        .flat_map(|kline| vec![
            kline.close.parse::<f64>().unwrap_or(0.0),
            kline.volume.parse::<f64>().unwrap_or(0.0),
        ])
        .collect();

    // 目标数据：将下一个时间段的收盘价与当前收盘价比较，高于则为1，否则为0
    let targets: Vec<usize> = data.windows(2)
        .map(|window| {
            let current_close = window[0].close.parse::<f64>().unwrap_or(0.0);
            let next_close = window[1].close.parse::<f64>().unwrap_or(0.0);
            if next_close > current_close { 1 } else { 0 }
        })
        .collect();

    // 确保特征和目标的数量匹配
    let features_len = features.len() / 2;  // 每个KlineSummary有2个特征

    // 将特征数据转换为 ndarray::Array2，并将目标数据转换为 ndarray::Array1
    let x = Array2::from_shape_vec((features_len, 2), features).unwrap();
    // let y = Array1::from_vec(targets);
    let y = Array1::from_shape_vec(targets.len(), targets).unwrap().to_owned();

    let feature_names = vec![
      "close", "volume",
    ];

    let training_records = x.mapv(|c| c); ///to_owned一样

    println!(
        "We obtain a {}x{} matrix of counts for the vocabulary entries",
        training_records.dim().0,
        training_records.dim().1
    );

    let mut training_dataset:Dataset<f64, usize, Ix1> = (training_records, y).into();
// 打印训练集的目标和特征
    println!("Training dataset before map_targets:");
    // 创建训练集
    let training_dataset =  training_dataset
        .map_targets(|x| {
            if *x == 1 {
                "accepted"
            } else {
                "denied"
            }
        })
        .with_feature_names(feature_names);


    print!("???????????{:?}",training_dataset);
    let model = GaussianNb::params().fit(&training_dataset).unwrap();

    let training_prediction = model.predict(&training_dataset);
    let cm = training_prediction
        .confusion_matrix(&training_dataset)
        .unwrap();
    // 0.9944
    let accuracy = cm.f1_score();
    println!("The fitted model has a training f1 score of {}", accuracy);

}




