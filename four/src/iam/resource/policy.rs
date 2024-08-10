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

#[derive(Debug, Clone, Serialize)]
pub struct PolicyName(String);

impl WillMappable<String> for PolicyName {}

impl Referenced for Policy {
    type To = PolicyName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
