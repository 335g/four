use serde::Serialize;

use crate::{service::Lambda, Arn};

#[derive(Debug, Clone, Serialize)]
pub struct FunctionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for FunctionArn {
    fn from(value: Arn<Lambda>) -> Self {
        FunctionArn(value)
    }
}
