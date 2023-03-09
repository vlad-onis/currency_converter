use axum::extract::{Query, Path};
use chrono::Utc;
use serde::Deserialize;


use crate::realm::{
    fiat::exchange_rate_api_client::{self, ExchangeRateClient, GetRates},
    fiat::currency::Currency,
    fiat::rate_converter::convert,
};

#[derive(Deserialize, Debug)]
pub struct Params {
    to: Currency,
    from: Currency
}

pub async fn handler(params: Query<Params>) -> String {
    
    // Handle this error
    let exchange_rate_client = ExchangeRateClient::new().unwrap();
    
    let result = convert(
        params.from.clone(), 
        params.to.clone(),
        50.00, 
        Utc::now().date_naive(), 
        exchange_rate_client).await.unwrap();

    format!("Result of the conversion is: {}", result)
}