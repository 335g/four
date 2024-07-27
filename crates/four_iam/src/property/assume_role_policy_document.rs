use serde::Serialize;

use crate::property::{statement::Statement, version::Version};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssumeRolePolicyDocument {
    version: Version,
    statement: Vec<Statement>,
}

impl AssumeRolePolicyDocument {
    pub fn new(version: Version, statement: Vec<Statement>) -> AssumeRolePolicyDocument {
        AssumeRolePolicyDocument { version, statement }
    }

    pub fn latest(statement: Vec<Statement>) -> AssumeRolePolicyDocument {
        AssumeRolePolicyDocument {
            version: Version::latest(),
            statement,
        }
    }
}
