use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        service::Lambda,
        Arn, LogicalId,
    },
    lambda::LooseFunctionName,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Url"]
pub struct Url {
    logical_id: LogicalId,
    auth_type: AuthType,
    cors: Option<Cors>,
    invoke_mode: Option<InvokeMode>,
    qualifier: Option<Qualifier>,
    target_function_arn: LooseFunctionName,
}

#[derive(Debug, Clone, Serialize)]
pub enum AuthType {
    #[serde(rename(serialize = "AWS_IAM"))]
    AwsIam,
    #[serde(rename(serialize = "NONE"))]
    None,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cors {
    allow_credentials: Option<bool>,
    allow_headers: Option<Vec<Header>>,
    allow_methods: Option<Vec<Method>>,
    allow_origins: Option<Vec<Origin>>,
    expose_headers: Option<Vec<Header>>,
    max_age: Option<MaxAge>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Get,
    Put,
    Head,
    Post,
    Patch,
    Delete,
    #[serde(rename(serialize = "*"))]
    Any,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Origin {
    #[serde(rename(serialize = "*"))]
    Any,
    Url(url::Url),
}

#[nutype(
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, Serialize)
)]
pub struct Header(String);

#[nutype(validate(less_or_equal = 86400), derive(Debug, Clone, Serialize))]
pub struct MaxAge(usize);

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InvokeMode {
    #[serde(rename(serialize = "BUFFERED"))]
    Buffered,
    #[serde(rename(serialize = "RESPONSE_STREAM"))]
    ResponseStream,
}

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"^[0-9][a-zA-Z0-9-_]*$"#),
    derive(Debug, Clone, Serialize)
)]
pub struct Qualifier(String);

#[derive(Debug, Clone, Serialize)]
pub struct UrlId;

impl Referenced for Url {
    type To = UrlId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionArn(Arn<Lambda>);

impl HaveAtt<FunctionArn> for Url {
    const KEY: &'static str = "FunctionArn";
}

impl HaveAtt<url::Url> for Url {
    const KEY: &'static str = "FunctionUrl";
}
