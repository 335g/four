use crate::{
    core::{convert::WillMappable, logical_id::LogicalId},
    lambda::{property::function_name::FunctionName, resource::function},
};
use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;
use thiserror::Error;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Alias"]
pub struct Alias {
    logical_id: LogicalId,
    description: Option<Description>,
    function_name: FunctionName,
    function_version: FunctionVersion,
    name: Name,
    provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    routing_config: Option<AliasRoutingConfiguration>,
}

impl WillMappable<function::FunctionName> for FunctionName {}

#[nutype(
    validate(not_empty, len_char_max = 256),
    derive(Debug, Clone, Serialize)
)]
pub struct Description(String);

#[nutype(
    validate(regex = r#"\$LATEST|[0-9]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct FunctionVersion(String);

#[nutype(validate(regex = r#"[a-zA-Z0-9-_]+"#), derive(Debug, Clone, Serialize))]
pub struct Name(String);

#[derive(Debug, Error)]
pub enum AliasError {
    #[error("description length must be less than or equal to 256: {0}")]
    InvalidDescription(String),

    #[error("function version must be ($LATEST|[0-9]+) pattern: {0}")]
    InvalidFunctionVersion(String),

    #[error("name must be (?!^[0-9]+$)([a-zA-Z0-9-_]+) pattern: {0}")]
    InvalidName(String),
}

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
