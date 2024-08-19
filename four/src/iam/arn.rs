use serde::Serialize;

use crate::{service::IAM, Arn};

#[derive(Debug, Clone, Serialize)]
pub struct GroupArn(Arn<IAM>);

impl From<Arn<IAM>> for GroupArn {
    fn from(value: Arn<IAM>) -> Self {
        GroupArn(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InstanceProfileArn(Arn<IAM>);

impl From<Arn<IAM>> for InstanceProfileArn {
    fn from(value: Arn<IAM>) -> Self {
        InstanceProfileArn(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ManagedPolicyArn(Arn<IAM>);

impl From<Arn<IAM>> for ManagedPolicyArn {
    fn from(value: Arn<IAM>) -> Self {
        ManagedPolicyArn(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OIDCProviderArn(Arn<IAM>);

impl From<Arn<IAM>> for OIDCProviderArn {
    fn from(value: Arn<IAM>) -> Self {
        OIDCProviderArn(value)
    }
}
