use crate::{
    core::LogicalId,
    lambda::{
        event_invoke_config::Qualifier, FunctionName, MaximumEventAgeInSeconds,
        MaximumRetryAttempts,
    },
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::EventInvokeConfig"]
pub struct EventInvokeConfig {
    logical_id: LogicalId,
    function_name: FunctionName,
    maximum_event_age_in_seconds: Option<MaximumEventAgeInSeconds>,
    maximum_retry_attempts: Option<MaximumRetryAttempts>,
    qualifier: Qualifier,
}
