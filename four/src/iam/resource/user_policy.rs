use crate::{
    core::{
        convert::WillBe,
        function::reference::{RefInner, Referenced},
        logical_id::LogicalId,
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

impl Referenced for UserPolicy {
    type To = UserName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}