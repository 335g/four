use crate::{
    core::{
        convert::WillBe,
        function::reference::{RefInner, Referenced},
        LogicalId,
    },
    iam::{
        property::policy_document::PolicyDocument,
        resource::{policy::PolicyName, user::UserName},
    },
};
use four_derive::ManagedResource;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::UserPolicy"]
pub struct UserPolicy {
    logical_id: LogicalId,
    policy_document: Option<PolicyDocument>,
    policy_name: WillBe<PolicyName>,
    user_name: WillBe<UserName>,
}

#[derive(Debug)]
pub struct UserPolicyId;

impl Referenced for UserPolicy {
    type To = UserPolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
