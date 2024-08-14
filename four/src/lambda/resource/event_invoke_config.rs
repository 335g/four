use crate::core::logical_id::LogicalId;
use four_derive::ManagedResource;
use nutype::nutype;
use thiserror::Error;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::EventInvokeConfig"]
pub struct EventInvokeConfig {
    logical_id: LogicalId,
    function_name: FunctionName,
    maximum_event_age_in_seconds: Option<MaximumEventAgeInSeconds>,
    maximum_retry_attempts: Option<MaximumRetryAttempts>,
    qualifier: Qualifier,
}

#[derive(Debug, Error)]
pub enum EventInvokeConfigError {
    #[error("invalid function name: {0}")]
    InvalidFunctionName(String),

    #[error("invalid maximum event age in seconds: {0}")]
    InvalidMaximumEventAgeInSeconds(usize),

    #[error("invalid maximum retry attempts: {0}")]
    InvalidMaximumRetryAttempts(usize),

    #[error("invalid qualifier: {0}")]
    InvalidQualifier(String),
}

#[nutype(
    validate(len_char_max = 64, regex = r#"[a-zA-Z0-9-_]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct FunctionName(String);

#[nutype(
    validate(greater_or_equal = 60, less_or_equal = 21600),
    derive(Debug, Clone, Serialize)
)]
pub struct MaximumEventAgeInSeconds(usize);

#[nutype(validate(less_or_equal = 2), derive(Debug, Clone, Serialize))]
pub struct MaximumRetryAttempts(usize);

#[nutype(
    validate(regex = r#"^(|[a-zA-Z0-9$_-]{1,129})$"#),
    derive(Debug, Clone, Serialize)
)]
pub struct Qualifier(String);
