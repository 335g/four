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

use crate::resource::role::RoleName;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::InstanceProfile"]
pub struct InstanceProfile {
    logical_id: LogicalId,
    instance_profile_name: Option<WillBe<InstanceProfileName>>,
    path: Option<String>,
    roles: Vec<WillBe<RoleName>>,
}

impl InstanceProfile {
    pub fn new(logical_id: LogicalId, roles: Vec<WillBe<RoleName>>) -> InstanceProfile {
        InstanceProfile {
            logical_id,
            instance_profile_name: None,
            path: None,
            roles,
        }
    }

    pub fn instance_profile_name(mut self, name: WillBe<InstanceProfileName>) -> InstanceProfile {
        self.instance_profile_name = Some(name);
        self
    }

    pub fn path(mut self, path: String) -> InstanceProfile {
        self.path = Some(path);
        self
    }
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
