use crate::{
    core::{
        convert::{WillBe, WillMappable},
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        logical_id::LogicalId,
        service::IAM,
        Arn,
    },
    iam::{
        resource::{managed_policy::ManagedPolicyArn, policy::Policy},
        util::Path,
    },
};
use four_derive::ManagedResource;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Group"]
pub struct Group {
    logical_id: LogicalId,
    group_name: Option<WillBe<GroupName>>,
    managed_policy_arns: Option<Vec<WillBe<ManagedPolicyArn>>>,
    path: Option<Path>,
    policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupName(String);

impl WillMappable<String> for GroupName {}

impl Referenced for Group {
    type To = GroupName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupArn(Arn<IAM>);

impl Attribute for GroupArn {
    fn name() -> &'static str {
        "Arn"
    }
}

impl HaveAtt<GroupArn> for Group {}
