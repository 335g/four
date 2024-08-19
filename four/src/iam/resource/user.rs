use crate::{
    core::{
        convert::{WillBe, WillMappable},
        function::{HaveAtt, RefInner, Referenced},
        service::IAM,
        Arn, LogicalId,
    },
    iam::{path::Path, property::policy_document::PolicyDocument, GroupName},
};
use four_derive::ManagedResource;
use nutype::nutype;
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

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
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

impl HaveAtt<UserArn> for User {
    const KEY: &'static str = "Arn";
}
