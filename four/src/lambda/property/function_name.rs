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

impl From<WillBe<FunctionName>> for LooseFunctionName {
    fn from(value: WillBe<FunctionName>) -> Self {
        LooseFunctionName::Name(value)
    }
}

impl From<WillBe<FunctionArn>> for LooseFunctionName {
    fn from(value: WillBe<FunctionArn>) -> Self {
        LooseFunctionName::Arn(value)
    }
}

impl From<WillBe<PartialArn>> for LooseFunctionName {
    fn from(value: WillBe<PartialArn>) -> Self {
        LooseFunctionName::Partial(value)
    }
}
