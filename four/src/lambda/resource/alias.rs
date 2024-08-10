use crate::{
    core::{
        arn::PartialArn,
        convert::{WillBe, WillMappable},
        logical_id::LogicalId,
    },
    lambda::resource::function::{self, FunctionArn},
};
use four_derive::ManagedResource;
use regex::Regex;
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

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FunctionName {
    Name(String),
    Arn(WillBe<FunctionArn>),
    Partial(WillBe<PartialArn>),
}

impl WillMappable<function::FunctionName> for FunctionName {}

#[derive(Debug, Clone, Serialize)]
pub struct Description(String);

impl TryFrom<&str> for Description {
    type Error = AliasError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        if len > 256 {
            Err(AliasError::InvalidDescription(value.to_string()))
        } else {
            Ok(Description(value.to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionVersion(String);

impl TryFrom<&str> for FunctionVersion {
    type Error = AliasError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r#"\$LATEST|[0-9]+"#).unwrap();
        if pattern.is_match(value) {
            Ok(FunctionVersion(value.to_string()))
        } else {
            Err(AliasError::InvalidFunctionVersion(value.to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Name(String);

impl TryFrom<&str> for Name {
    type Error = AliasError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r#"(?!^[0-9]+$)([a-zA-Z0-9-_]+"#).unwrap();

        if pattern.is_match(value) {
            Ok(Name(value.to_string()))
        } else {
            Err(AliasError::InvalidName(value.to_string()))
        }
    }
}

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
