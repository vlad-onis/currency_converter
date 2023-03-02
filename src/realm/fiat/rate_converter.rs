use chrono::NaiveDate;

use super::currency::Currency;
use super::exchange_rate_api_client::ExchangeRateClient;

pub async fn convert(from: Currency, to: Currency, amount: f64, date: NaiveDate) {
    let exchange_api_client = ExchangeRateClient::new().unwrap();
    let rates_response = exchange_api_client.get_rates(from.clone(), date).await;

    let conversion_rate = rates_response.get(&to).expect("No conversion rate found");

    let result: f64 = amount * conversion_rate;
    println!("{amount} {from:?} is equal to {result} {to:?}");
    println!("{date:?}");
}
