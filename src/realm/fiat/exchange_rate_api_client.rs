use chrono::prelude::*;
use dotenv_loader::parser::Parser;
use reqwest;
use std::env;
use std::path::Path;
use thiserror::Error;

use super::currency::Currency;

#[derive(Error, Debug)]
pub enum ExchangeRateClientError {
    #[error("Could not load api key for the exchange rate api")]
    ApiKeyLoadFailure,
}

pub struct ExchangeRateClient {
    pub api_key: String,
}

impl ExchangeRateClient {
    pub fn new() -> Result<ExchangeRateClient, ExchangeRateClientError> {
        let mut dotenv_parser = Parser::new();
        let _res = dotenv_parser.parse(Path::new(".env"));

        if let Ok(mut api_key) = env::var("EXCHANGE_API_KEY") {
            if api_key.starts_with('"') && api_key.ends_with('"') {
                api_key.remove(0);
                api_key.remove(api_key.len() - 1);
            }

            return Ok(ExchangeRateClient { api_key });
        }

        Err(ExchangeRateClientError::ApiKeyLoadFailure)
    }

    pub async fn get_rates(&self, base: Currency, date: NaiveDate) {
        let url = format!(
            "https://api.apilayer.com/exchangerates_data/{}?base={}",
            date, base.0
        );

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("apikey", self.api_key.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{response}");
    }
}
