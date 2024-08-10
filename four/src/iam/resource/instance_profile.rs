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
    iam::{resource::role::RoleName, util::Path},
};
use four_derive::ManagedResource;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::InstanceProfile"]
pub struct InstanceProfile {
    logical_id: LogicalId,
    instance_profile_name: Option<WillBe<InstanceProfileName>>,
    path: Option<Path>,
    roles: Vec<WillBe<RoleName>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstanceProfileName(String);

impl WillMappable<String> for InstanceProfileName {}

impl Referenced for InstanceProfile {
    type To = InstanceProfileName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InstanceProfileArn(Arn<IAM>);

impl Attribute for InstanceProfileArn {
    fn name() -> &'static str {
        "Arn"
    }
}

impl HaveAtt<InstanceProfileName> for InstanceProfile {}
