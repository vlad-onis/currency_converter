use async_trait::async_trait;
use chrono::prelude::*;
use dotenv_loader::parser::Parser;
use mockall::*;
use reqwest::{self};
use serde::Deserialize;
use thiserror::Error;
use tracing::debug;

use std::{collections::HashMap, env, ops::Deref, path::Path};

use super::currency::Currency;

#[derive(Error, Debug)]
pub enum ExchangeRateClientError {
    #[error("ApiKeyLoad: Could not load api key for the exchange rate api")]
    ApiKeyLoad,

    #[error("Api: Failed to send the request to the api : {0}")]
    Api(reqwest::Error),

    #[error("ResponseDeserialization: Failed deserializing the response from the api. Your request may be erroneous: {0}")]
    ResponseDeserialization(reqwest::Error),
}

#[derive(Deserialize, Debug)]
pub struct GetRatesResponse {
    pub rates: HashMap<Currency, f64>,
}

impl Deref for GetRatesResponse {
    type Target = HashMap<Currency, f64>;

    fn deref(&self) -> &Self::Target {
        &self.rates
    }
}

#[derive(Debug)]
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

        Err(ExchangeRateClientError::ApiKeyLoad)
    }
}

#[async_trait]
#[automock]
pub trait GetRates {
    async fn get_rates(
        &self,
        base: Currency,
        date: NaiveDate,
    ) -> Result<GetRatesResponse, ExchangeRateClientError>;
}

#[async_trait]
impl GetRates for ExchangeRateClient {
    #[allow(clippy::redundant_closure)]
    async fn get_rates(
        &self,
        base: Currency,
        date: NaiveDate,
    ) -> Result<GetRatesResponse, ExchangeRateClientError> {
        let url = format!(
            "https://api.apilayer.com/exchangerates_data/{}?base={}",
            date,
            base.symbol()
        );

        let client = reqwest::Client::new();

        let request = client.get(url).header("apikey", self.api_key.clone());

        // TODO: This is logging sensitive information like api key, must be resolved.
        debug!("Sending request: {request:?}");

        let response = request
            .send()
            .await
            .map_err(|e| ExchangeRateClientError::Api(e))?
            .json::<GetRatesResponse>()
            .await
            .map_err(|e| ExchangeRateClientError::ResponseDeserialization(e))?;

        debug!("Rates: {:?}", response);
        Ok(response)
    }
}
