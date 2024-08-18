use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{
        property::{policy_document::PolicyDocument, GroupName, GroupPolicyId},
        resource::policy::PolicyName,
    },
};
use four_derive::ManagedResource;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::GroupPolicy"]
pub struct GroupPolicy {
    logical_id: LogicalId,
    group_name: WillBe<GroupName>,
    policy_document: Option<PolicyDocument>,
    policy_name: WillBe<PolicyName>,
}

impl Referenced for GroupPolicy {
    type To = GroupPolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
