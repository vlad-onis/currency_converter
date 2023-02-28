use thiserror::Error;

#[derive(Debug, Error)]
pub enum DateFormatError {
    #[error("Format specified is not yet supported")]
    UnsupportedDateFormat,
}

#[derive(Clone, Debug)]
pub enum DateFormat {
    Ymd,
}

impl TryFrom<String> for DateFormat {
    type Error = DateFormatError;
    fn try_from(source: String) -> Result<DateFormat, Self::Error> {
        match source.to_lowercase().as_str() {
            "ymd" => Ok(DateFormat::Ymd),
            _ => Err(DateFormatError::UnsupportedDateFormat),
        }
    }
}

impl From<DateFormat> for String {
    fn from(source: DateFormat) -> String {
        match source {
            DateFormat::Ymd => "%Y-%m-%d".to_string(),
        }
    }
}
