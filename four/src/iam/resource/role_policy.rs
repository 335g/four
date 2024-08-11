use crate::{
    core::{
        convert::WillBe,
        function::reference::{RefInner, Referenced},
        logical_id::LogicalId,
    },
    iam::{
        property::policy_document::PolicyDocument,
        resource::{policy::PolicyName, role::RoleName},
    },
};
use four_derive::ManagedResource;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::RolePolicy"]
pub struct RolePolicy {
    logical_id: LogicalId,
    policy_document: Option<PolicyDocument>,
    policy_name: WillBe<PolicyName>,
    role_name: WillBe<RoleName>,
}

#[derive(Debug)]
pub struct RolePolicyId;

impl Referenced for RolePolicy {
    type To = RolePolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
