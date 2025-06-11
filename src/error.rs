use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActionError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    ParseBool(#[from] std::str::ParseBoolError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Other(String),
}

impl From<&str> for ActionError {
    fn from(s: &str) -> Self {
        ActionError::Other(s.to_string())
    }
}

impl From<String> for ActionError {
    fn from(s: String) -> Self {
        ActionError::Other(s)
    }
}