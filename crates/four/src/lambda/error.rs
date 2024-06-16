use std::path::PathBuf;

use cargo_lambda_metadata::error::MetadataError;

use crate::lambda::code::{S3BucketNameError, S3KeyError};

#[derive(Debug, thiserror::Error)]
pub enum LambdaError {
    #[error("Unexpected Error: {0:?}")]
    Unexpected(#[from] anyhow::Error),

    #[error("Unsupported zip file archive")]
    UnsupportedZipFile,

    #[error("Invalid S3 key: {0:?}")]
    InvalidS3Key(#[from] S3KeyError),

    #[error("Invalid S3 bucket: {0:?}")]
    InvalidS3Bucket(#[from] S3BucketNameError),

    #[error("binary_path must be path for binary: {0:?}")]
    InvalidBinaryPath(PathBuf),

    #[error("Cannot zip archive: {0:?}")]
    CannotZipArchive(String),

    #[error(transparent)]
    CannotLoadMetadata(#[from] MetadataError),

    #[error("No binary file: {0:?}")]
    BinaryMissing(PathBuf),
}
