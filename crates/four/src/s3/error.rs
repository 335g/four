use super::{BucketNameError, KeyError};

#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("Cannot get a response from calling list_buckets request")]
    CannotGetResponseListBuckets,

    #[error(transparent)]
    InvalidBucketName(#[from] BucketNameError),

    #[error(transparent)]
    Invalidkey(#[from] KeyError),
}
