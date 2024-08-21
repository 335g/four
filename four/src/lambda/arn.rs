use serde::Serialize;

use crate::{service::Lambda, Arn};

#[derive(Debug, Clone, Serialize)]
pub struct FunctionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for FunctionArn {
    fn from(value: Arn<Lambda>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerVersionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for LayerVersionArn {
    fn from(value: Arn<Lambda>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VersionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for VersionArn {
    fn from(value: Arn<Lambda>) -> Self {
        Self(value)
    }
}
