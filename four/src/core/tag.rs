use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    key: TagKey,
    value: TagValue,
}

#[derive(Debug, Error)]
pub enum TagError {
    #[error("invalid tag key: {0}")]
    InvalidKey(String),

    #[error("invalid tag value: {0}")]
    InvalidValue(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct TagKey(String);

impl TryFrom<&str> for TagKey {
    type Error = TagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 128 {
            Err(TagError::InvalidKey(value.to_string()))
        } else {
            Ok(TagKey(value.to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TagValue(String);

impl TryFrom<&str> for TagValue {
    type Error = TagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 256 {
            Err(TagError::InvalidValue(value.to_string()))
        } else {
            Ok(TagValue(value.to_string()))
        }
    }
}
