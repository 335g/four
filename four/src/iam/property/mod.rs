pub mod action;
pub mod effect;
mod managed_policy;
pub mod policy_document;
pub mod principal;
pub mod statement;

pub use managed_policy::{
    AWSManagedPolicy, ManagedPolicyDescription, ManagedPolicyDescriptionError,
};
