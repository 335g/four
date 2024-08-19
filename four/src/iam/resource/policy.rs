use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{
        property::policy_document::PolicyDocument, GroupName, PolicyId, PolicyName, RoleName,
        UserName,
    },
    ManagedResource,
};

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

impl Referenced for Policy {
    type To = PolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
