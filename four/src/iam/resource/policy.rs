use crate::{
    core::{
        convert::{WillBe, WillMappable},
        function::reference::{RefInner, Referenced},
        logical_id::LogicalId,
    },
    iam::{
        property::policy_document::PolicyDocument,
        resource::{group::GroupName, role::RoleName, user::UserName},
    },
};
use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Policy"]
pub struct Policy {
    logical_id: LogicalId,
    groups: Option<Vec<WillBe<GroupName>>>,
    policy_document: PolicyDocument,
    policy_name: WillBe<PolicyName>,
    roles: Option<Vec<WillBe<RoleName>>>,
    users: Option<Vec<WillBe<UserName>>>,
}

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct PolicyName(String);

impl WillMappable<String> for PolicyName {}

#[derive(Debug)]
pub struct PolicyId;

impl Referenced for Policy {
    type To = PolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
