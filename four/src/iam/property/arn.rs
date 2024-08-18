use serde::Serialize;

use crate::{service::IAM, Arn};

#[derive(Debug, Clone, Serialize)]
pub struct GroupArn(Arn<IAM>);

#[derive(Debug, Clone, Serialize)]
pub struct InstanceProfileArn(Arn<IAM>);
