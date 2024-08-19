mod access_key;
mod arn;
mod group;
mod id;
mod instance_profile;
mod path;
mod property;
pub mod resource;
mod role;

pub use access_key::{AccessKeyStatus, SecretAccessKey};
pub use arn::{GroupArn, InstanceProfileArn, ManagedPolicyArn, OIDCProviderArn, RoleArn};
pub use group::{GroupName, GroupNameError, Groups};
pub use id::{AccessKeyId, GroupPolicyId, InstanceProfileId, PolicyId, RoleId, RolePolicyId};
pub use instance_profile::{InstanceProfileName, InstanceProfileNameError};
pub use path::{Path, PathError};
pub use property::{
    action,
    effect::Effect,
    policy_document::PolicyDocument,
    principal::{Principal, ServicePrincipal},
    statement::{ActionOr, PrincipalOr, Statement, StatementBuilder1, StatementBuilder2},
};
pub use role::RoleName;
