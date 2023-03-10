use std::fmt::Display;

use serde::Deserialize;

// TODO: Implement Deref or provide getters for the inner field
// TODO: Do some validation when creating the currency.

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Currency(pub String);

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
