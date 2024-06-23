use serde::{Deserialize, Serialize};

use crate::{lambda::Code, resource::Resource};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Function {
    #[serde(rename(serialize = "Role"))]
    role_arn: String,

    #[serde(rename(serialize = "Code"))]
    code: Code,
}

impl Resource for Function {
    fn resource_type(&self) -> &'static str {
        "AWS::Lambda::Function"
    }
}
