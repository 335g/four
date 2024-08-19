use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::{core::LogicalId, lambda::LooseFunctionName};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Version"]
pub struct Version {
    logical_id: LogicalId,
    code_sha256: Option<String>,
    description: Option<Description>,
    function_name: LooseFunctionName,
    provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    runtime_policy: Option<RuntimePolicy>,
}

#[nutype(validate(len_char_max = 256), derive(Debug, Clone, Serialize))]
pub struct Description(String);

#[derive(Debug, Clone, Serialize)]
pub struct ProvisionedConcurrencyConfiguration {
    provisioned_concurrent_executions: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimePolicy {
    runtime_version_arn: Option<RuntimeVersionArn>,
    update_runtime_on: String,
}

#[nutype(
    validate(
        len_char_min = 26,
        len_char_max = 2048,
        regex = r#"^arn:(aws[a-zA-Z-]*):lambda:[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\d{1}::runtime:.+$"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct RuntimeVersionArn(String);
