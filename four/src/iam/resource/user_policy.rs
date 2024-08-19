use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{property::policy_document::PolicyDocument, PolicyName, UserName, UserPolicyId},
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::UserPolicy"]
pub struct UserPolicy {
    logical_id: LogicalId,
    policy_document: Option<PolicyDocument>,
    policy_name: WillBe<PolicyName>,
    user_name: WillBe<UserName>,
}

impl Referenced for UserPolicy {
    type To = UserPolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
