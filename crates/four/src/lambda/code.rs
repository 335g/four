use aws_config::SdkConfig;
use aws_sdk_lambda::types::FunctionCode;
use nutype::nutype;
use serde::{Deserialize, Serialize};

use crate::{
    lambda::error::LambdaError,
    s3::{
        bucket::{ExistingBucket, UnconformedBucket},
        error::S3Error,
    },
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Code {
    image_uri: Option<String>,
    s3_bucket: Option<ExistingBucket>,
    s3_key: Option<S3Key>,
    s3_object_version: Option<String>,
    zip_file: Option<String>,
}

impl Code {
    async fn try_from(value: FunctionCode, sdk_config: &SdkConfig) -> Result<Self, LambdaError> {
        if value.zip_file().is_some() {
            return Err(LambdaError::UnsupportedZipFile);
        }
        let s3_key = value
            .s3_key()
            .map(|key| S3Key::new(key.to_string()))
            .transpose()?;
        let s3_bucket = value
            .s3_bucket()
            .map(|bucket| UnconformedBucket::new(bucket.to_string()))
            .transpose()
            .map_err(|e| S3Error::InvalidBucketName(e))?;
        let s3_bucket = if let Some(s3_bucket) = s3_bucket {
            let x = s3_bucket.check_exists(&sdk_config).await?;
            Some(x)
        } else {
            None
        };

        Ok(Code {
            image_uri: value.image_uri,
            s3_bucket,
            s3_key,
            s3_object_version: value.s3_object_version,
            zip_file: None,
        })
    }
}

#[nutype(
    validate(len_char_min = 1,),
    derive(Debug, Clone, Deserialize, Serialize, Deref)
)]
pub struct S3Key(String);
