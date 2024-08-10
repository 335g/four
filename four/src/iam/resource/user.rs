use crate::{
    core::{
        arn::Arn,
        convert::{WillBe, WillMappable},
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        logical_id::LogicalId,
        service::IAM,
    },
    iam::{property::policy_document::PolicyDocument, resource::group::GroupName, util::Path},
};
use four_derive::ManagedResource;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::User"]
pub struct User {
    logical_id: LogicalId,
    groups: Option<Vec<WillBe<GroupName>>>,
    login_profile: Option<LoginProfile>,
    managed_policy_arns: Option<Vec<WillBe<Arn<IAM>>>>,
    path: Option<Path>,
    policies: Option<Vec<Policy>>,
    user_name: Option<WillBe<UserName>>,
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

impl Referenced for User {
    type To = UserName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserArn(Arn<IAM>);

impl HaveAtt<UserArn> for User {}

impl Attribute for UserArn {
    fn name() -> &'static str {
        "Arn"
    }
}
