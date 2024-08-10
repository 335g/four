use four::{logical_id::LogicalId, ManagedResource};
use regex::Regex;
use serde::Serialize;
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

#[derive(Debug, Clone, Serialize)]
pub struct FunctionName(String);

impl TryFrom<&str> for FunctionName {
    type Error = EventInvokeConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        if len == 0 || len > 64 {
            return Err(EventInvokeConfigError::InvalidFunctionName(
                value.to_string(),
            ));
        }

        let pattern = Regex::new(r#"[a-zA-Z0-9-_]+"#).unwrap();
        if !pattern.is_match(value) {
            return Err(EventInvokeConfigError::InvalidFunctionName(
                value.to_string(),
            ));
        } else {
            return Ok(FunctionName(value.to_string()));
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MaximumEventAgeInSeconds(usize);

impl TryFrom<usize> for MaximumEventAgeInSeconds {
    type Error = EventInvokeConfigError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 60 || value > 21600 {
            Err(EventInvokeConfigError::InvalidMaximumEventAgeInSeconds(
                value,
            ))
        } else {
            Ok(MaximumEventAgeInSeconds(value))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MaximumRetryAttempts(usize);

impl TryFrom<usize> for MaximumRetryAttempts {
    type Error = EventInvokeConfigError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > 2 {
            Err(EventInvokeConfigError::InvalidMaximumRetryAttempts((value)))
        } else {
            Ok(MaximumRetryAttempts(value))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Qualifier(String);

impl TryFrom<&str> for Qualifier {
    type Error = EventInvokeConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(r#"^(|[a-zA-Z0-9$_-]{1,129})$"#).unwrap();

        if !pattern.is_match(value) {
            Err(EventInvokeConfigError::InvalidQualifier(value.to_string()))
        } else {
            Ok(Qualifier(value.to_string()))
        }
    }
}
