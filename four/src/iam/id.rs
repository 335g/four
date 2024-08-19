use serde::Serialize;

#[derive(Debug)]
pub struct AccessKeyId;

#[derive(Debug)]
pub struct InstanceProfileId;

#[derive(Debug)]
pub struct GroupPolicyId;

#[derive(Debug)]
pub struct PolicyId;

#[derive(Debug)]
pub struct RolePolicyId;

#[derive(Debug, Clone, Serialize)]
pub struct RoleId(String);
