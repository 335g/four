use serde::Serialize;

use crate::property::{statement::Statement, version::Version};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    version: Version,
    statement: Vec<Statement>,
}
