use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::core::{
    function::{Attribute, HaveAtt, RefInner, Referenced},
    service::IAM,
    Arn, LogicalId, Tag,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::SAMLProvider"]
pub struct SAMLProvider {
    logical_id: LogicalId,
    name: Option<SAMLProviderName>,
    saml_metadata_document: Option<SAMLMetadataDocument>,
    tags: Option<Vec<Tag>>,
}

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w._-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct SAMLProviderName(String);

#[nutype(
    validate(len_char_min = 1000, len_char_max = 10000000),
    derive(Debug, Clone, Serialize)
)]
pub struct SAMLMetadataDocument(String);

#[derive(Debug, Clone, Serialize)]
pub struct SAMLProviderArn(Arn<IAM>);

impl From<Arn<IAM>> for SAMLProviderArn {
    fn from(value: Arn<IAM>) -> Self {
        SAMLProviderArn(value)
    }
}

impl Referenced for SAMLProvider {
    type To = SAMLProviderArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<SAMLProviderArn> for SAMLProvider {}

impl Attribute for SAMLProviderArn {
    fn name() -> &'static str {
        "Arn"
    }
}
