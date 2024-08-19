mod access_key;
mod arn;
mod group;
mod id;
mod instance_profile;
mod property;
pub mod resource;
mod util;

pub use access_key::{AccessKeyId, AccessKeyStatus, SecretAccessKey};
pub use arn::{GroupArn, InstanceProfileArn, ManagedPolicyArn};
pub use group::{GroupName, GroupNameError, Groups};
pub use id::{GroupPolicyId, InstanceProfileId};
pub use instance_profile::{InstanceProfileName, InstanceProfileNameError};
pub use property::{
    action,
    effect::Effect,
    policy_document::PolicyDocument,
    principal::{Principal, ServicePrincipal},
    statement::{ActionOr, PrincipalOr, Statement, StatementBuilder1, StatementBuilder2},
};
pub use util::{Path, PathError};
