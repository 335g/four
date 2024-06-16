use super::bucket::UnconformedBucketError;

#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("Cannot get a response from calling list_buckets request")]
    CannotGetResponseListBuckets,

    #[error("No bucket: {0}")]
    NoBucket(String),

    #[error(transparent)]
    InvalidBucketName(#[from] UnconformedBucketError),
}
