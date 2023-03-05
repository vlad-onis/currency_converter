use chrono::NaiveDate;
use thiserror::Error;

use crate::realm::fiat::exchange_rate_api_client::{GetRates, ExchangeRateClient, ExchangeRateClientError};
use super::currency::Currency;

#[derive(Error, Debug)]
pub enum RateConversionError {
    #[error("Exchange api client failed: {0}")]
    ExchangeApiClientFailure(ExchangeRateClientError),
}

#[allow(clippy::redundant_closure)]
pub async fn convert<S: GetRates> (
    from: Currency,
    to: Currency,
    amount: f64,
    date: NaiveDate,
    client: S
) -> Result<f64, RateConversionError> {

    let rates_response = client
        .get_rates(from.clone(), date)
        .await
        .map_err(|e| RateConversionError::ExchangeApiClientFailure(e))?;

    let conversion_rate = rates_response.get(&to).expect("No conversion rate found");

    let result: f64 = amount * conversion_rate;
    println!("{amount} {from:?} is equal to {result} {to:?}");
    println!("{date:?}");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, net::ToSocketAddrs};

    use chrono::Utc;
    use mockall::predicate;

    use crate::realm::fiat::{exchange_rate_api_client::{MockGetRates, GetRatesResponse, GetRates}, currency::Currency};
    use super::convert;

    #[tokio::test]
    async fn test_convert() {

        let mut get_rates_mock = MockGetRates::new();
        
        let mut expectected_rates: HashMap<Currency, f64> = HashMap::new();
        expectected_rates.insert(Currency("RON".to_string()), 4.932639);
       
        let base = Currency("EUR".to_string());
        let date = Utc::now().date_naive();
        
        // Move is used because the closure may outlide the test function
        // So we want to take ownership of the arguments instead of borrowing them
        // TODO: understand why boxing the future is needed.
        get_rates_mock.expect_get_rates()
            .return_once(move |base, date| {
                let expected_response = GetRatesResponse {
                    rates: expectected_rates.clone()
                };
                Box::pin(async move {Ok(expected_response)})
            });

        let result = convert(base, Currency("RON".to_string()), 50.00, date, get_rates_mock).await.unwrap();
        println!("{result}");
        assert!(result > 246.00 && result < 247.00);

    }
}