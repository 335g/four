use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        service::IAM,
        Arn, LogicalId,
    },
    iam::{path::Path, GroupName, LoginProfile, Policy, UserArn, UserName},
    ManagedResource,
};

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

impl Referenced for User {
    type To = UserName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<UserArn> for User {
    const KEY: &'static str = "Arn";
}
