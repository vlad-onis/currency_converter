use std::fmt::Display;

use serde::Deserialize;
use thiserror::Error;

// TODO: Implement Deref or provide getters for the inner field
// TODO: Do some validation when creating the currency.

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("The symbol must have a length of exactly 3 characters")]
    SymbolLength,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Currency(String);

impl Currency {
    pub fn parse(currency: &str) -> Result<Currency, ParseError> {
        if currency.len() != 3 {
            return Err(ParseError::SymbolLength);
        }

        Ok(Currency(currency.to_uppercase()))
    }

    pub fn symbol(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Currency {
    fn from(source: String) -> Currency {
        Currency(source)
    }
}

impl From<Currency> for String {
    fn from(source: Currency) -> String {
        source.0
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(dead_code)]
pub struct Rate {
    from: Currency,
    to: Currency,
    rate: f32,
}
