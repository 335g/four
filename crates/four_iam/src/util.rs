use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

static PATH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"((/[A-Za-z0-9\.,\+@=_-]+)*)/"#).unwrap());

#[derive(Debug, Clone, Serialize)]
pub struct Path(String);

impl TryFrom<&str> for Path {
    type Error = PathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 512 || !PATH_REGEX.is_match(&value) {
            Err(PathError::Invalid(value.to_string()))
        } else {
            Ok(Path(value.to_string()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PathError {
    #[error("invalid path pattern: {0}")]
    Invalid(String),
}
