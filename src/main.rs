mod realm;

use chrono::{NaiveDate, Utc};
use clap::Parser;
use realm::{
    fiat::currency::Currency,
    fiat::rate_converter::{convert, RateConversionError},
    utils::DateFormat,
};
use tracing::{debug, error, warn, Level};
use tracing_subscriber::FmtSubscriber;

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

fn set_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<(), RateConversionError> {
    set_tracing();
    let args = Args::parse();

    let date_format = match args.format {
        Some(format) => {
            let date_format = DateFormat::try_from(format.as_str());
            match date_format {
                Ok(date_format) => date_format,
                Err(_) => {
                    warn!("Format: {} is not supported, defaulting to YMD", format);
                    DateFormat::Ymd
                }
            }
        }
        None => DateFormat::Ymd,
    };

    let date = match args.date {
        Some(date_string) => {
            NaiveDate::parse_from_str(&date_string, String::from(date_format.clone()).as_str())
                .unwrap_or_else(|_| {
                    tracing::warn!(
                "Failed to parse date from input: {date_string}. Defaulting to today's date"
            );
                    Utc::now().date_naive()
                })
        }
        None => Utc::now().date_naive(),
    };

    // TODO: Currency needs some validation for the inner string
    let base = match args.base_currency {
        Some(base_currency) => Currency(base_currency),
        None => Currency("EUR".to_string()),
    };

    debug!("Input date: {date:?}");
    debug!("Dateformat: {date_format:?}");
    debug!("Base currency: {base:?}");

    let conversion_result = convert(base, Currency(String::from("EUR")), 50.0, date).await;
    if let Err(conversion_error) = conversion_result {
        error!("{conversion_error}");
    }

    Ok(())
}
