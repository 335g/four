use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{
        path::Path,
        property::{policy_document::PolicyDocument, ManagedPolicyDescription},
        resource::{role::RoleName, user::UserName},
        Groups, ManagedPolicyArn,
    },
};
use four_derive::ManagedResource;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::ManagedPolicy"]
pub struct ManagedPolicy {
    logical_id: LogicalId,
    description: Option<ManagedPolicyDescription>,
    groups: Option<Groups>,
    managed_policy_name: Option<WillBe<String>>,
    path: Option<Path>,
    policy_document: PolicyDocument,
    roles: Option<Vec<WillBe<RoleName>>>,
    users: Option<Vec<WillBe<UserName>>>,
}

impl Referenced for ManagedPolicy {
    type To = ManagedPolicyArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
