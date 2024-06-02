use std::collections::HashMap;

use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(
    validate(regex = r#"[a-zA-Z][a-zA-Z0-9_]+"#),
    derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)
)]
pub struct EnvironmentKey(String);

#[nutype(
    validate(regex = r#"[a-zA-Z][a-zA-Z0-9_]+"#),
    derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)
)]
pub struct EnvironmentValue(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "Variables")]
    variables: HashMap<EnvironmentKey, EnvironmentValue>,
}

impl FromIterator<(EnvironmentKey, EnvironmentValue)> for Environment {
    fn from_iter<T: IntoIterator<Item = (EnvironmentKey, EnvironmentValue)>>(iter: T) -> Self {
        let variables = iter.into_iter().collect::<HashMap<_, _>>();
        Self { variables }
    }
}
