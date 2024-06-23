use std::path::PathBuf;

use cargo_lambda_metadata::error::MetadataError;

use crate::s3;

#[derive(Debug, thiserror::Error)]
pub enum LambdaError {
    #[error("Unexpected Error: {0:?}")]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    S3(#[from] s3::Error),

    #[error("Unsupported zip file archive")]
    UnsupportedZipFile,

    #[error("binary_path must be path for binary: {0:?}")]
    InvalidBinaryPath(PathBuf),

    #[error("Cannot zip archive: {0:?}")]
    CannotZipArchive(String),

    #[error(transparent)]
    CannotLoadMetadata(#[from] MetadataError),

    #[error("No binary file: {0:?}")]
    BinaryMissing(PathBuf),
}
