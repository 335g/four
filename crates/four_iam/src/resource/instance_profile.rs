use four::{
    convert::{WillBe, WillMappable},
    logical_id::LogicalId,
    ManagedResource,
};
use serde::Serialize;

use super::role::RoleArn;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::InstanceProfile"]
pub struct InstanceProfile {
    logical_id: LogicalId,
    instance_profile_name: WillBe<InstanceProfileName>,
    path: String,
    roles: Vec<WillBe<RoleArn>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstanceProfileName(String);

impl WillMappable<String> for InstanceProfileName {}
