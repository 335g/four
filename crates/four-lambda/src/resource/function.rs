use four_core::{
    convert::WillBe,
    logical_id::{LogicalId, LogicalIdentified},
    resource::ManagedResource,
};
use four_iam::resource::role::{Role, RoleArn};
use serde::{Deserialize, Serialize};

use crate::property::code::Code;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Function<'a> {
    role: WillBe<'a, RoleArn>,

    #[serde(rename(serialize = "Code"))]
    code: Code,

    #[serde(skip)]
    logical_id: LogicalId,
}

impl<'a> ManagedResource for Function<'a> {
    fn resource_type(&self) -> &'static str {
        "AWS::Lambda::Function"
    }
}

impl<'a> LogicalIdentified for Function<'a> {
    fn logical_id(&self) -> &LogicalId {
        &self.logical_id
    }
}
