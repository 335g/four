use super::property::{EnvironmentKeyError, EnvironmentValueError};

#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    #[error("invalid environemnt key: {0:?}")]
    InvalidEnvironmentKey(#[from] EnvironmentKeyError),

    #[error("invalid environemnt value: {0:?}")]
    InvalidEnvironmentValue(#[from] EnvironmentValueError),
}
