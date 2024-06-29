use serde::Serialize;

use crate::property::version::Version;

use crate::property::statement::Statement;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    version: Version,
    statement: Vec<Statement>,
}
