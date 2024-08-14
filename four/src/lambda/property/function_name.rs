use serde::Serialize;

use crate::{
    core::{arn::PartialArn, convert::WillBe},
    lambda::resource::function::FunctionArn,
};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FunctionName {
    Name(String),
    Arn(WillBe<FunctionArn>),
    Partial(WillBe<PartialArn>),
}
