mod realm;

use chrono::{NaiveDate, Utc};
use clap::Parser;

use realm::{
    exchange_rate_api_client::ExchangeRateClient, fiat::currency::Currency, utils::DateFormat,
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, required = false)]
    #[clap(requires = "format")]
    date: Option<String>,

    #[arg(short, long, required = false, help = "Supported formats: \"ymd\"")]
    format: Option<String>,

    #[arg(
        short,
        long,
        required = false,
        help = "Base currency for our rate converter, defaults to EUR"
    )]
    base_currency: Option<String>,
}

#[tokio::main]
async fn main() {
    let exchange_client = ExchangeRateClient::new().unwrap();

    let args = Args::parse();

    let date_format = match args.format {
        Some(format) => DateFormat::try_from(format).unwrap(),
        None => DateFormat::Ymd,
    };

    let date = match args.date {
        Some(date_string) => {
            NaiveDate::parse_from_str(&date_string, String::from(date_format.clone()).as_str())
                .unwrap()
        }
        None => Utc::now().date_naive(),
    };

    // TODO: Currency needs some validation for the inner string
    let base = match args.base_currency {
        Some(base_currency) => Currency(base_currency),
        None => Currency("EUR".to_string()),
    };

    exchange_client.get_rates(base, date).await;
}
