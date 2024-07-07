use serde::Serialize;

use crate::property::{statement::Statement, version::Version};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    version: Version,
    statement: Vec<Statement>,
}

impl Policy {
    pub fn new(version: Version, statement: Vec<Statement>) -> Policy {
        Policy { version, statement }
    }

    pub fn latest(statement: Vec<Statement>) -> Policy {
        Policy {
            version: Version::latest(),
            statement,
        }
    }
}
