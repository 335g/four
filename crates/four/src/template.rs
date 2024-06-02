use std::collections::HashSet;

use lazy_static::lazy_static;
use nutype::nutype;
use parse_display::Display;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Display)]
pub enum FormatVersion {
    #[display("2010-09-09")]
    Version1,
}

pub trait Template {
    fn format_vesion() -> FormatVersion {
        FormatVersion::Version1
    }

    fn description(&self) -> Option<&str> {
        None
    }
}

#[nutype(
    validate(regex = r#"[[:alpha:]0-9]+"#),
    derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)
)]
pub struct InnerLogicalId(String);

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct LogicalId(InnerLogicalId);

impl TryFrom<String> for LogicalId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if LOGICAL_IDS.contains(&s) {
            return Err(Error::DuplicatedLogicalId(s.clone()));
        }
        let s = InnerLogicalId::new(s)?;
        Ok(LogicalId(s))
    }
}

pub struct LogicalIds(HashSet<String>);

impl LogicalIds {
    pub fn new() -> Self {
        LogicalIds(HashSet::new())
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(s)
    }
}

lazy_static! {
    static ref LOGICAL_IDS: LogicalIds = LogicalIds::new();
}

pub trait Resource {
    fn resource_type() -> &'static str;
    fn logical_id(&self) -> &LogicalId;
    // fn properties(&self) -> Option<HashMap<String, >
}
