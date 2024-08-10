use crate::iam::property::{statement::Statement, version::Version};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyDocument {
    version: Version,
    statement: Vec<Statement>,
}

impl PolicyDocument {
    pub fn new(version: Version, statement: Vec<Statement>) -> PolicyDocument {
        PolicyDocument { version, statement }
    }

    pub fn latest(statement: Vec<Statement>) -> PolicyDocument {
        PolicyDocument {
            version: Version::latest(),
            statement,
        }
    }
}
