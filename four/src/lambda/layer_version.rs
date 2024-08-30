use crate::lambda::{Architecture, Runtime};
use nutype::nutype;
use serde::{ser::SerializeMap, Serialize};

#[nutype(
    validate(predicate = |architectures| architectures.len() <= 2),
    derive(Debug, Clone, Serialize, TryFrom),
)]
pub struct CompatibleArchitectures(Vec<Architecture>);

#[nutype(
    validate(predicate = |runtimes| runtimes.len() <= 15),
    derive(Debug, Clone, Serialize, TryFrom),
)]
pub struct CompatibleRuntimes(Vec<Runtime>);

#[derive(Debug, Clone)]
pub struct FunctionContent {
    // TODO: define type in response to Amazon S3 support feature
    pub s3_bucket: String,
    pub s3_key: String,
    pub s3_object_version: Option<String>,
}

impl Serialize for FunctionContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("S3Bucket", &self.s3_bucket)?;
        map.serialize_entry("S3Key", &self.s3_key)?;
        if let Some(version) = &self.s3_object_version {
            map.serialize_entry("S3ObjectVersion", version)?;
        }
        map.end()
    }
}

#[nutype(
    validate(len_char_min = 0, len_char_max = 256),
    derive(Debug, Clone, Serialize, TryFrom)
)]
pub struct LayerVersionDescription(String);

#[nutype(
    validate(
        len_char_min = 1,
        len_char_max = 140,
        regex = r#"(arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+)|[a-zA-Z0-9-_]+"#
    ),
    derive(Debug, Clone, Serialize, TryFrom)
)]
pub struct LayerName(String);

#[nutype(validate(len_char_max = 512), derive(Debug, Clone, Serialize, TryFrom))]
pub struct LicenseInfo(String);
