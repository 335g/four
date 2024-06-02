use std::collections::HashMap;

use nutype::nutype;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Architectures {
    Arm64,
    X86_64,
}

impl Default for Architectures {
    fn default() -> Self {
        Architectures::X86_64
    }
}

#[nutype(
    validate(regex = r#"^(arn:(aws[a-zA-Z-]*)?:[a-z0-9-.]+:.*)|()$"#),
    derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)
)]
pub struct DeadLetterConfigTargetArn(String);

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeadLetterConfig {
    target_arn: DeadLetterConfigTargetArn,
}

impl DeadLetterConfig {
    pub fn new(target_arn: DeadLetterConfigTargetArn) -> Self {
        Self { target_arn }
    }
}

#[nutype(
    validate(
        regex = r#"arn:(aws[a-zA-Z-]*)?:lambda:[a-z]{2}((-gov)|(-iso(b?)))?-[a-z]+-\d{1}:\d{12}:code-signing-config:csc-[a-z0-9]{17}"#
    ),
    derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)
)]
pub struct CodeSigningConfigArn(String);

#[nutype(
    validate(len_char_max = 256),
    derive(Debug, Clone, Serialize, Deserialize)
)]
pub struct Description(String);

#[nutype(
    validate(greater_or_equal = 512, less_or_equal = 10240),
    derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)
)]
pub struct EphemeralStorageValue(usize);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "CamelCase")]
pub struct EphemeralStorage {
    size: EphemeralStorageValue,
}

impl EphemeralStorage {
    pub fn new(size: EphemeralStorageValue) -> Self {
        Self { size }
    }
}
