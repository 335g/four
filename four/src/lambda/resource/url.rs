use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    lambda::{AuthType, Cors, FunctionArn, InvokeMode, LooseFunctionName, Qualifier, UrlId},
    ManagedResource,
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
