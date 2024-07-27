use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Code {
    #[serde(rename(serialize = "S3Bucket"))]
    bucket_name: String,

    #[serde(rename(serialize = "S3Key"))]
    key: String,
}

impl Code {
    pub fn new(bucket_name: &str, key: &str) -> Code {
        Code {
            bucket_name: bucket_name.to_string(),
            key: key.to_string(),
        }
    }
}
