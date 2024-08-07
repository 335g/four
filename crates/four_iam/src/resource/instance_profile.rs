use four::{
    arn::Arn,
    convert::{WillBe, WillMappable},
    function::{
        getatt::{Attribute, HaveAtt},
        reference::Referenced,
    },
    logical_id::LogicalId,
    service::IAM,
    ManagedResource,
};
use serde::Serialize;

use crate::{resource::role::RoleName, util::Path};

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

    fn referenced(&self) -> four::function::reference::RefInner {
        four::function::reference::RefInner::Id(self.logical_id.clone())
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
