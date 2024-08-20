use nutype::nutype;
use serde::Serialize;

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
