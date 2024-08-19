use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{
        property::policy_document::PolicyDocument, resource::policy::PolicyName, RoleName,
        RolePolicyId,
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

impl Referenced for RolePolicy {
    type To = RolePolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
