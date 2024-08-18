mod access_key;
pub mod action;
mod arn;
pub mod effect;
pub mod group;
mod group_policy;
mod id;
mod instance_profile;
pub mod policy_document;
pub mod principal;
pub mod statement;
pub mod version;

pub use access_key::{AccessKeyId, AccessKeyStatus, SecretAccessKey};
pub use arn::{GroupArn, InstanceProfileArn};
pub use group_policy::{GroupName, GroupNameError, GroupPolicyId};
pub use id::InstanceProfileId;
pub use instance_profile::{InstanceProfileName, InstanceProfileNameError};
