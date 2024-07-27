use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

static HANDLER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^[^\s]+$"#).unwrap());

#[derive(Debug, Clone, Serialize)]
pub struct Handler(String);

impl TryFrom<&str> for Handler {
    type Error = HandlerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if HANDLER_REGEX.is_match(s) {
            Ok(Handler(s.to_string()))
        } else {
            Err(HandlerError::Invalid(s.to_string()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    #[error("invalid handler: {0}")]
    Invalid(String),
}
