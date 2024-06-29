use four_s3 as s3;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported zip file archive")]
    UnsupportedZipFile,

    #[error(transparent)]
    S3(#[from] s3::Error),
}
