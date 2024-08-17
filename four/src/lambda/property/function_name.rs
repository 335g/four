use serde::Serialize;

use crate::{
    core::{convert::WillBe, PartialArn},
    lambda::resource::function::FunctionArn,
};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FunctionName {
    Name(String),
    Arn(WillBe<FunctionArn>),
    Partial(WillBe<PartialArn>),
}
