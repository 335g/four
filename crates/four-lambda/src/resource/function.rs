use four_core::Construct;
use four_iam::Role;
use serde::{Deserialize, Serialize};

use crate::property::code::Code;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Function {
    role: Role,

    #[serde(rename(serialize = "Code"))]
    code: Code,
}

impl Construct for Function {
    fn resource_type(&self) -> &'static str {
        "AWS::Lambda::Function"
    }
}
