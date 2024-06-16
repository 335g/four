use crate::{lambda::error::LambdaError, s3::error::S3Error};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid resource")]
    InvalidResource,

    #[error(transparent)]
    Lambda(#[from] LambdaError),

    #[error(transparent)]
    S3(#[from] S3Error),
}
