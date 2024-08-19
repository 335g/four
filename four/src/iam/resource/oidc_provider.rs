use four_derive::ManagedResource;
use url::Url;

use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        LogicalId, Tag,
    },
    iam::OIDCProviderArn,
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

impl Referenced for OIDCProvider {
    type To = OIDCProviderArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<OIDCProviderArn> for OIDCProvider {
    const KEY: &'static str = "Arn";
}
