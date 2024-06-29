use crate::property::bucket::{BucketNameError, KeyError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    InvalidBucketName(#[from] BucketNameError),

    #[error(transparent)]
    InvalidKey(#[from] KeyError),
}
