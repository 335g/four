use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

static LOGICAL_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[[:alnum:]]+"#).unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalId {
    inner: String,
}

impl TryFrom<&str> for LogicalId {
    type Error = LogicalIdError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_string();
        if LOGICAL_NAME_REGEX.is_match(&s) {
            Ok(LogicalId { inner: s })
        } else {
            Err(LogicalIdError::Invalid(s))
        }
    }
}

impl Serialize for LogicalId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LogicalIdError {
    #[error("Invalid logical ID: {0}")]
    Invalid(String),
}

pub trait LogicalIdentified {
    fn logical_id(&self) -> &LogicalId;
}
