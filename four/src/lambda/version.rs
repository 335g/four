use nutype::nutype;
use serde::Serialize;

#[nutype(validate(len_char_max = 256), derive(Debug, Clone, Serialize))]
pub struct VersionDescription(String);

#[derive(Debug, Clone, Serialize)]
pub struct RuntimePolicy {
    runtime_version_arn: Option<RuntimeVersionArn>,
    update_runtime_on: String,
}

#[nutype(
    validate(
        len_char_min = 26,
        len_char_max = 2048,
        regex = r#"^arn:(aws[a-zA-Z-]*):lambda:[a-z]{2}((-gov)|(-iso([a-z]?)))?-[a-z]+-\d{1}::runtime:.+$"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct RuntimeVersionArn(String);

#[derive(Debug, Clone)]
pub struct VersionNumber(usize);
