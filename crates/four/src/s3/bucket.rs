use aws_config::{
    retry::{RetryConfig, RetryMode},
    SdkConfig,
};
use nutype::nutype;
use serde::{Deserialize, Serialize};

use crate::s3::error::S3Error;

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
    derive(Debug, Clone, Deserialize, Serialize, Deref)
)]
pub struct UnconformedBucket(String);

impl UnconformedBucket {
    pub async fn check_exists(self, sdk_config: &SdkConfig) -> Result<ExistingBucket, S3Error> {
        let bucket_name = self.as_str();
        let client = aws_sdk_s3::Client::new(&sdk_config);
        let resp = client
            .list_buckets()
            .send()
            .await
            .map_err(|_| S3Error::CannotGetResponseListBuckets)?;

        if resp
            .buckets
            .unwrap_or_default()
            .into_iter()
            .flat_map(|bucket| bucket.name)
            .filter(|x| x == bucket_name)
            .next()
            .is_some()
        {
            Ok(ExistingBucket(bucket_name.to_string()))
        } else {
            Err(S3Error::NoBucket(bucket_name.to_string()))
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ExistingBucket(String);
