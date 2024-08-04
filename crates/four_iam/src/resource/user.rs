use four::{
    arn::Arn,
    convert::{WillBe, WillMappable},
    logical_id::LogicalId,
    service::IAM,
    ManagedResource,
};
use serde::Serialize;

use crate::property::policy_document::PolicyDocument;

use super::group::GroupName;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::User"]
pub struct User {
    logical_id: LogicalId,
    groups: Option<Vec<WillBe<GroupName>>>,
    login_profile: Option<LoginProfile>,
    managed_policy_arns: Option<Vec<WillBe<Arn<IAM>>>>,
    path: Option<String>,
    policies: Option<Vec<Policy>>,
    user_name: Option<WillBe<UserName>>,
}

impl User {
    pub fn new(logical_id: LogicalId) -> User {
        User {
            logical_id,
            groups: None,
            login_profile: None,
            managed_policy_arns: None,
            path: None,
            policies: None,
            user_name: None,
        }
    }

    pub fn groups(mut self, groups: Vec<WillBe<GroupName>>) -> User {
        self.groups = Some(groups);
        self
    }

    pub fn login_profile(mut self, profile: LoginProfile) -> User {
        self.login_profile = Some(profile);
        self
    }

    pub fn managed_policy_arns(mut self, arns: Vec<WillBe<Arn<IAM>>>) -> User {
        self.managed_policy_arns = Some(arns);
        self
    }

    pub fn path(mut self, path: String) -> User {
        self.path = Some(path);
        self
    }

    pub fn policies(mut self, policies: Vec<Policy>) -> User {
        self.policies = Some(policies);
        self
    }

    pub fn user_name(mut self, user_name: WillBe<UserName>) -> User {
        self.user_name = Some(user_name);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginProfile {
    password: String,
    password_reset_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Policy {
    policy_document: PolicyDocument,
    policy_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserName(String);

impl WillMappable<String> for UserName {}
