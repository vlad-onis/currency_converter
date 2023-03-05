use chrono::NaiveDate;
use thiserror::Error;

use super::currency::Currency;
use super::exchange_rate_api_client::{ExchangeRateClient, ExchangeRateClientError};

#[derive(Error, Debug)]
pub enum RateConversionError {
    #[error("Exchange api client failed: {0}")]
    ExchangeApiClientFailure(ExchangeRateClientError),
}

#[allow(clippy::redundant_closure)]
pub async fn convert(
    from: Currency,
    to: Currency,
    amount: f64,
    date: NaiveDate,
) -> Result<f64, RateConversionError> {
    let exchange_api_client =
        ExchangeRateClient::new().map_err(|e| RateConversionError::ExchangeApiClientFailure(e))?;

    let rates_response = exchange_api_client
        .get_rates(from.clone(), date)
        .await
        .map_err(|e| RateConversionError::ExchangeApiClientFailure(e))?;

    let conversion_rate = rates_response.get(&to).expect("No conversion rate found");

    let result: f64 = amount * conversion_rate;
    println!("{amount} {from:?} is equal to {result} {to:?}");
    println!("{date:?}");
    Ok(result)
}
