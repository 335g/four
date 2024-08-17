use crate::{
    core::{
        convert::{WillBe, WillMappable},
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        service::IAM,
        Arn, LogicalId,
    },
    iam::{resource::role::RoleName, util::Path},
};
use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::InstanceProfile"]
pub struct InstanceProfile {
    logical_id: LogicalId,
    instance_profile_name: Option<WillBe<InstanceProfileName>>,
    path: Option<Path>,
    roles: Vec<WillBe<RoleName>>,
}

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct InstanceProfileName(String);

impl WillMappable<String> for InstanceProfileName {}

#[derive(Debug)]
pub struct InstanceProfileId;

impl Referenced for InstanceProfile {
    type To = InstanceProfileId;

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
