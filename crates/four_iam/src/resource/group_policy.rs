use four::{
    convert::WillBe,
    function::reference::{RefInner, Referenced},
    logical_id::LogicalId,
    ManagedResource,
};
use serde::Serialize;

use crate::property::policy_document::PolicyDocument;
use crate::resource::policy::PolicyName;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::GroupPolicy"]
pub struct GroupPolicy {
    logical_id: LogicalId,
    group_name: WillBe<GroupName>,
    policy_document: Option<PolicyDocument>,
    policy_name: WillBe<PolicyName>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupName(String);

impl Referenced for GroupPolicy {
    type To = GroupName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
