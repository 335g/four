use nutype::nutype;
use serde::Serialize;

#[nutype(
    validate(not_empty, len_char_max = 256),
    derive(Debug, Clone, Serialize)
)]
pub struct AliasDescription(String);

#[nutype(
    validate(regex = r#"\$LATEST|[0-9]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct FunctionVersion(String);

#[nutype(validate(regex = r#"[a-zA-Z0-9-_]+"#), derive(Debug, Clone, Serialize))]
pub struct AliasName(String);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProvisionedConcurrencyConfiguration {
    provisioned_concurrency_executions: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AliasRoutingConfiguration {
    additional_version_weights: Vec<VersionWeight>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionWeight {
    function_version: String,
    function_weight: f32,
}
