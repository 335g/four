use crate::iam::property::statement::Statement;
use derive_new::new;
use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
pub enum PolicyDocumentVersion {
    #[serde(rename(serialize = "2012-10-17"))]
    V2012_10_17,
    #[serde(rename(serialize = "2008-10-17"))]
    V2008_10_17,
}

impl PolicyDocumentVersion {
    pub fn latest() -> PolicyDocumentVersion {
        PolicyDocumentVersion::V2012_10_17
    }
}

#[derive(Debug, Clone, Serialize, new)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyDocument {
    version: PolicyDocumentVersion,
    statement: Vec<Statement>,
}

impl PolicyDocument {
    pub fn latest(statement: Vec<Statement>) -> PolicyDocument {
        PolicyDocument {
            version: PolicyDocumentVersion::latest(),
            statement,
        }
    }
}
