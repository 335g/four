use crate::lambda::{Architecture, Runtime};
use nutype::nutype;
use serde::Serialize;

#[nutype(
    validate(predicate = |architectures| architectures.len() <= 2),
    derive(Debug, Clone, Serialize),
)]
pub struct CompatibleArchitectures(Vec<Architecture>);

#[nutype(
    validate(predicate = |runtimes| runtimes.len() <= 15),
    derive(Debug, Clone, Serialize),
)]
pub struct CompatibleRuntimes(Vec<Runtime>);

#[derive(Debug, Clone, Serialize)]
pub struct FunctionContent {
    // TODO: define type in response to Amazon S3 support feature
    s3_bucket: String,
    s3_key: String,
    s3_object_version: String,
}

#[nutype(
    validate(len_char_min = 0, len_char_max = 256),
    derive(Debug, Clone, Serialize)
)]
pub struct LayerVersionDescription(String);

#[nutype(
    validate(
        len_char_min = 1,
        len_char_max = 140,
        regex = r#"(arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+)|[a-zA-Z0-9-_]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct LayerName(String);

#[nutype(validate(len_char_max = 512), derive(Debug, Clone, Serialize))]
pub struct LicenseInfo(String);
