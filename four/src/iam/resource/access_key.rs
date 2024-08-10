use crate::{
    core::{
        convert::WillBe,
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        logical_id::LogicalId,
    },
    iam::resource::user::UserName,
};
use four_derive::ManagedResource;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::AccessKey"]
pub struct AccessKey {
    logical_id: LogicalId,
    serial: Option<i64>,
    status: Option<AccessKeyStatus>,
    user_name: WillBe<UserName>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AccessKeyStatus {
    Inactive,
    Active,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccessKeyId(String);

impl Referenced for AccessKey {
    type To = AccessKeyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SecretAccessKey(String);

impl HaveAtt<SecretAccessKey> for AccessKey {}

impl Attribute for SecretAccessKey {
    fn name() -> &'static str {
        "SecretAccessKey"
    }
}
