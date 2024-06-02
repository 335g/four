use crate::{lambda::error::LambdaError, template::InnerLogicalIdError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid logical id: {0:?}")]
    InvalidLogicalId(#[from] InnerLogicalIdError),

    #[error("duplicated logical id{0:?}")]
    DuplicatedLogicalId(String),

    #[error("Error (Lambda): {0:?}")]
    Lambda(#[from] LambdaError),
}
