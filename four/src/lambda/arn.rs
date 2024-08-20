use serde::Serialize;

use crate::{
    service::{Lambda, IAM},
    Arn,
};

#[derive(Debug, Clone, Serialize)]
pub struct FunctionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for FunctionArn {
    fn from(value: Arn<Lambda>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerVersionArn(Arn<IAM>);

impl From<Arn<IAM>> for LayerVersionArn {
    fn from(value: Arn<IAM>) -> Self {
        Self(value)
    }
}
