use crate::{iam, lambda, s3};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid resource")]
    InvalidResource,

    #[error(transparent)]
    Lambda(#[from] lambda::Error),

    #[error(transparent)]
    S3(#[from] s3::Error),

    #[error(transparent)]
    Iam(#[from] iam::Error),
}
