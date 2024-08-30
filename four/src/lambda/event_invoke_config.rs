use nutype::nutype;
use serde::Serialize;

#[nutype(
    validate(greater_or_equal = 60, less_or_equal = 21600),
    derive(Debug, Clone, Serialize, TryFrom)
)]
pub struct MaximumEventAgeInSeconds(usize);

#[nutype(validate(less_or_equal = 2), derive(Debug, Clone, Serialize, TryFrom))]
pub struct MaximumRetryAttempts(usize);

#[derive(Debug, Clone, Serialize)]
pub struct OnFailure {
    destination: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OnSuccess {
    destination: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum DestinationConfig {
    OnSuccess(OnSuccess),
    OnFailure(OnFailure),
}
