use chrono::NaiveDate;
use serde::Deserialize;
use axum::{extract::Query, response::{IntoResponse, Response}, http::status::StatusCode, Json};
use thiserror::Error;
use tracing::debug;
use serde_json::{Value, Map};

use crate::realm::fiat::{
    currency::Currency, 
    exchange_rate_api_client::{ExchangeRateClient, ExchangeRateClientError},
    rate_converter::{convert, RateConversionError},
};

#[derive(Deserialize, Debug)]
pub struct Params {
    pub to: Currency,
    pub from: Currency,
    pub amount: f64,
    pub date: NaiveDate,
}

#[derive(Error, Debug)]
pub enum RateConversionApiLayerError {
    #[error("Exchange rate client error: {0}")]
    ExchangeRateClient(#[from] ExchangeRateClientError),

    #[error("Conversion error: {0}")]
    Conversion(#[from] RateConversionError),
}

impl IntoResponse for RateConversionApiLayerError {
    fn into_response(self) -> Response {
        let body = match self {
            RateConversionApiLayerError::ExchangeRateClient(inner) => format!("Exchange rate client error: {inner}"),
            RateConversionApiLayerError::Conversion(inner) => format!("Conversion error:{inner}")
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub async fn handle_rate_conversion(params: Query<Params>) -> Result<Json<String>, RateConversionApiLayerError> {
    
    let client = ExchangeRateClient::new()?;

    debug!("Params: from: {:?}, to: {:?}, amount: {:?}, date: {:?}", params.from, params.to, params.amount, params.date);
    
    let conversion_res = convert(params.from.clone(), params.to.clone(), params.amount, params.date).await?;

    
    debug!("Converting {} {:?} to {:?}", params.amount, params.from.clone(), params.to.clone());

    Ok(Json(format!("The result of conversion of {} {:?} to {:?} is {}", params.amount, params.from.clone(), params.to.clone(), conversion_res.to_string())))
}