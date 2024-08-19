use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    iam::{
        path::Path, resource::role::RoleName, InstanceProfileArn, InstanceProfileId,
        InstanceProfileName,
    },
};
use four_derive::ManagedResource;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::InstanceProfile"]
pub struct InstanceProfile {
    logical_id: LogicalId,
    instance_profile_name: Option<WillBe<InstanceProfileName>>,
    path: Option<Path>,
    roles: Vec<WillBe<RoleName>>,
}

impl Referenced for InstanceProfile {
    type To = InstanceProfileId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<InstanceProfileArn> for InstanceProfile {
    const KEY: &'static str = "Arn";
}
