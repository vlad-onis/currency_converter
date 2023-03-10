use std::str::FromStr;

use axum::body::Full;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use tracing::{error, warn};

use crate::realm::{
    fiat::currency::Currency,
    fiat::exchange_rate_api_client::{self, ExchangeRateClient, GetRates},
    fiat::rate_converter::convert,
};

#[derive(Deserialize, Debug)]
pub struct Params {
    to: Currency,
    from: Currency,
    amount: f64,
    date: Option<String>,
}

fn parse_date(date: &Option<String>) -> NaiveDate {
    let now = Utc::now().date_naive();
    match date {
        None => now,
        Some(date_string) => {
            let tentative_date = NaiveDate::from_str(&date_string);
            if let Ok(parsed_date) = tentative_date {
                parsed_date
            } else {
                warn!("Failed to parse the input date, format may be wrong. It should be YYYY-MM-DD. Defaulting to current time");
                now
            }
        }
    }
}

pub async fn handler(params: Query<Params>) -> Response {
    let date = parse_date(&params.date);

    // It's not a heavy object to be created this new call here is fine
    let exhange_rate_client_res = ExchangeRateClient::new();
    let Ok(exchange_rate_client) = exhange_rate_client_res else {
        error!("Could not create exchange api client: {}", exhange_rate_client_res.err().unwrap());
        return (StatusCode::INTERNAL_SERVER_ERROR, "Exchange api error").into_response();
    };

    let result = convert(
        params.from.clone(),
        params.to.clone(),
        params.amount,
        date,
        exchange_rate_client,
    )
    .await
    .unwrap();

    format!(
        "{} {} = {} {}",
        params.amount, params.from, result, params.to
    )
    .into_response()
}
