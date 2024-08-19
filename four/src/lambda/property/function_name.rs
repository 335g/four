use serde::Serialize;

use crate::{
    core::{convert::WillBe, PartialArn},
    lambda::{FunctionArn, FunctionName},
};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LooseFunctionName {
    Name(WillBe<FunctionName>),
    Arn(WillBe<FunctionArn>),
    Partial(WillBe<PartialArn>),
}
