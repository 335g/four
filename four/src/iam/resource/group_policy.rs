use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{property::policy_document::PolicyDocument, resource::policy::PolicyName},
};
use four_derive::ManagedResource;
use nutype::nutype;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::GroupPolicy"]
pub struct GroupPolicy {
    logical_id: LogicalId,
    group_name: WillBe<GroupName>,
    policy_document: Option<PolicyDocument>,
    policy_name: WillBe<PolicyName>,
}

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct GroupName(String);

#[derive(Debug)]
pub struct GroupPolicyId;

impl Referenced for GroupPolicy {
    type To = GroupPolicyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
