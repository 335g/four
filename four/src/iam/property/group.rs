use serde::Serialize;

use crate::{service, Arn};

#[derive(Debug, Clone, Serialize)]
pub struct GroupArn(Arn<service::IAM>);
