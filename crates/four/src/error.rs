use thiserror::Error;

use crate::lambda::error::LambdaError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid resource")]
    InvalidResource,

    #[error(transparent)]
    Lambda(#[from] LambdaError),
}
