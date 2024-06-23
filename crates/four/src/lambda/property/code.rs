use aws_sdk_lambda::types::FunctionCode;
use serde::{Deserialize, Serialize};

use crate::{lambda::error::LambdaError, s3};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Code {
    image_uri: Option<String>,
    s3_bucket: Option<s3::BucketName>,
    s3_key: Option<s3::Key>,
    s3_object_version: Option<String>,
    zip_file: Option<String>,
}

impl Code {
    async fn try_from(value: FunctionCode) -> Result<Code, LambdaError> {
        if value.zip_file().is_some() {
            return Err(LambdaError::UnsupportedZipFile);
        }
        let s3_key = value
            .s3_key()
            .map(|key| s3::Key::new(key.to_string()))
            .transpose()
            .map_err(|e| s3::Error::Invalidkey(e))?;
        let s3_bucket = value
            .s3_bucket()
            .map(|bucket| s3::BucketName::new(bucket.to_string()))
            .transpose()
            .map_err(|e| s3::Error::InvalidBucketName(e))?;

        Ok(Code {
            image_uri: value.image_uri,
            s3_bucket,
            s3_key,
            s3_object_version: value.s3_object_version,
            zip_file: None,
        })
    }
}
