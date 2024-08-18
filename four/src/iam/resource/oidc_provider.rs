use four_derive::ManagedResource;
use serde::Serialize;
use url::Url;

use crate::core::{
    function::{HaveAtt, RefInner, Referenced},
    service::IAM,
    Arn, LogicalId, Tag,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::OIDCProvider"]
pub struct OIDCProvider {
    logical_id: LogicalId,
    client_id_list: Option<Vec<String>>,
    tags: Option<Vec<Tag>>,
    thumbprint_list: Option<Vec<String>>,
    url: Option<Url>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OIDCProviderArn(Arn<IAM>);

impl From<Arn<IAM>> for OIDCProviderArn {
    fn from(value: Arn<IAM>) -> Self {
        OIDCProviderArn(value)
    }
}

impl Referenced for OIDCProvider {
    type To = OIDCProviderArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<OIDCProviderArn> for OIDCProvider {
    const KEY: &'static str = "Arn";
}
