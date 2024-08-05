use four::{
    convert::{WillBe, WillMappable},
    function::reference::Referenced,
    logical_id::LogicalId,
    ManagedResource,
};
use serde::Serialize;

use crate::property::policy_document::PolicyDocument;

use crate::resource::{group::GroupName, role::RoleName, user::UserName};

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

impl Policy {
    pub fn new(logical_id: LogicalId, name: WillBe<String>, document: PolicyDocument) -> Policy {
        Policy {
            logical_id,
            groups: None,
            policy_document: document,
            policy_name: name.map(),
            roles: None,
            users: None,
        }
    }

    pub fn groups(mut self, groups: Vec<WillBe<GroupName>>) -> Policy {
        self.groups = Some(groups);
        self
    }

    pub fn roles(mut self, roles: Vec<WillBe<RoleName>>) -> Policy {
        self.roles = Some(roles);
        self
    }

    pub fn users(mut self, users: Vec<WillBe<UserName>>) -> Policy {
        self.users = Some(users);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PolicyName(String);

impl WillMappable<String> for PolicyName {}

impl Referenced for Policy {
    type To = PolicyName;

    fn referenced(&self) -> four::function::reference::RefInner {
        four::function::reference::RefInner::Id(self.logical_id.clone())
    }
}
