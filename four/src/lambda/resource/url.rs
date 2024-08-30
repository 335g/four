use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    lambda::{AuthType, Cors, FunctionArn, InvokeMode, LooseFunctionName, Qualifier, UrlId},
    ManagedResource,
};

/// [AWS::Lambda::Url]
///
/// The AWS::Lambda::Url resource creates a function URL with the specified configuration parameters.
/// A function URL is a dedicated HTTP(S) endpoint that you can use to invoke your function.
///
///
/// [AWS::Lambda::Url]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html
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

impl Referenced for Url {
    type To = UrlId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<FunctionArn> for Url {
    const KEY: &'static str = "FunctionArn";
}

impl HaveAtt<url::Url> for Url {
    const KEY: &'static str = "FunctionUrl";
}
