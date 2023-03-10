use chrono::NaiveDate;
use thiserror::Error;

use super::currency::Currency;
use crate::realm::fiat::exchange_rate_api_client::{ExchangeRateClientError, GetRates};

#[derive(Error, Debug)]
pub enum RateConversionError {
    #[error("Exchange api client failed: {0}")]
    ExchangeApiClientFailure(ExchangeRateClientError),
}

#[allow(clippy::redundant_closure)]
pub async fn convert<S: GetRates>(
    from: Currency,
    to: Currency,
    amount: f64,
    date: NaiveDate,
    client: S,
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
    use chrono::{NaiveDate, Utc};
    use std::{collections::HashMap, str::FromStr};

    use super::convert;
    use crate::realm::fiat::{
        currency::Currency,
        exchange_rate_api_client::{GetRatesResponse, MockGetRates},
    };

    #[tokio::test]
    async fn test_valid_conversion() {
        let mut get_rates_mock = MockGetRates::new();

        let mut expectected_rates: HashMap<Currency, f64> = HashMap::new();
        expectected_rates.insert(Currency::parse("RON").unwrap(), 4.932639);

        let base = Currency::parse("EUR").unwrap();
        let date = NaiveDate::from_str("2023-03-04").unwrap();

        // Move is used because the closure may outlide the test function
        // So we want to take ownership of the arguments instead of borrowing them
        // TODO: understand why boxing the future is needed.
        get_rates_mock
            .expect_get_rates()
            .return_once(move |_base, _date| {
                let expected_response = GetRatesResponse {
                    rates: expectected_rates.clone(),
                };
                Box::pin(async move { Ok(expected_response) })
            });

        let result = convert(
            base,
            Currency::parse("RON").unwrap(),
            50.00,
            date,
            get_rates_mock,
        )
        .await
        .unwrap();
        println!("{result}");
        assert!(result > 246.00 && result < 247.00);
    }
}
