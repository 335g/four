use aws_sdk_lambda::types::FunctionCode;
use nutype::nutype;
use serde::{Deserialize, Serialize};

use crate::lambda::error::LambdaError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Code {
    image_uri: Option<String>,
    s3_bucket: Option<S3BucketName>,
    s3_key: Option<S3Key>,
    s3_object_version: Option<String>,
    zip_file: Option<String>,
}

impl TryFrom<FunctionCode> for Code {
    type Error = LambdaError;

    fn try_from(value: FunctionCode) -> Result<Self, Self::Error> {
        if value.zip_file().is_some() {
            return Err(LambdaError::UnsupportedZipFile);
        }
        let s3_key = value
            .s3_key()
            .map(|key| S3Key::new(key.to_string()))
            .transpose()?;
        let s3_bucket = value
            .s3_bucket()
            .map(|bucket| S3BucketName::new(bucket))
            .transpose()?;

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
    validate(
        len_char_min = 3,
        len_char_max = 63,
        regex = r#"^[0-9a-z][0-9A-Za-z\.\-]+[0-9a-z]$"#,
        predicate = |s| {
            // TODO: custsom validation (cf. https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/userguide/bucketnamingrules.html)
            true
        }
    ),
    derive(Debug, Clone, Deserialize, Serialize)
)]
pub struct S3BucketName(String);

#[nutype(
    validate(len_char_min = 1,),
    derive(Debug, Clone, Deserialize, Serialize)
)]
pub struct S3Key(String);
