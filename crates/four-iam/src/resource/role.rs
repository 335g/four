use serde::Serialize;

use crate::property::policy::Policy;

pub trait RoleType {}

// TODO: replace String to ARN
pub struct RoleRef(String);

// TODO: replace String to ARN
impl From<String> for RoleRef {
    fn from(value: String) -> Self {
        RoleRef(value)
    }
}

impl RoleType for RoleRef {}

#[derive(Serialize)]
pub struct Role {
    #[serde(rename(serialize = "AssumeRolePolicyDocument"))]
    policy: Policy,
}
