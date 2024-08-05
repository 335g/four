use four::{
    convert::{WillBe, WillMappable},
    logical_id::LogicalId,
    ManagedResource,
};
use serde::Serialize;

use crate::property::policy_document::PolicyDocument;

use super::user::UserName;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Policy"]
pub struct Policy {
    logical_id: LogicalId,
    policy_name: WillBe<PolicyName>,
    policy_document: PolicyDocument,
    users: Option<Vec<WillBe<UserName>>>,
}

impl Policy {
    pub fn new(logical_id: LogicalId, name: WillBe<String>, document: PolicyDocument) -> Policy {
        Policy {
            logical_id,
            policy_name: name.map(),
            policy_document: document,
            users: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PolicyName(String);

impl WillMappable<String> for PolicyName {}
