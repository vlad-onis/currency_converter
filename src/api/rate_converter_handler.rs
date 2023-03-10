use axum::extract::{Query, Path};
use axum::response::{Response, IntoResponse};
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
    from: Currency,
    amount: f64
}

pub async fn handler(params: Query<Params>) -> Response {
    
    // TODO: Handle this error
    let exchange_rate_client = ExchangeRateClient::new().unwrap();
    
    let result = convert(
        params.from.clone(), 
        params.to.clone(),
        params.amount, 
        Utc::now().date_naive(), 
        exchange_rate_client).await.unwrap();
        
    format!("{} {} = {} {}", params.amount, params.from, result, params.to).into_response()
}