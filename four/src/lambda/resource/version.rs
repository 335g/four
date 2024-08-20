use crate::{
    core::LogicalId,
    lambda::{
        LooseFunctionName, ProvisionedConcurrencyConfiguration, RuntimePolicy, VersionDescription,
    },
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Version"]
pub struct Version {
    logical_id: LogicalId,
    code_sha256: Option<String>,
    description: Option<VersionDescription>,
    function_name: LooseFunctionName,
    provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    runtime_policy: Option<RuntimePolicy>,
}
