use std::path::PathBuf;

use cargo_lambda_build::BinaryArchive;
use serde::{Deserialize, Serialize};

use crate::{lambda::code::Code, resource::IntoResource};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    #[serde(rename(serialize = "Role"))]
    role_arn: String,

    #[serde(rename(serialize = "Code"))]
    code: Code,
}

impl IntoResource for Function {
    fn resource_type() -> &'static str {
        "AWS::Lambda::Function"
    }
}

pub struct SimpleFunction {
    manifest_path: PathBuf,
}

impl SimpleFunction {}
