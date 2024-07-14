use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

use crate::error::Error;

static LOGICAL_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[[:alnum:]]+"#).unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalId {
    inner: String,
}

impl TryFrom<&str> for LogicalId {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_string();
        if LOGICAL_NAME_REGEX.is_match(&s) {
            Ok(LogicalId { inner: s })
        } else {
            Err(Error::InvalidLogicalId(s))
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

pub trait LogicalIdentified {
    fn logical_id(&self) -> &LogicalId;
}