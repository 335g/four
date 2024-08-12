use four_derive::ManagedResource;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::core::{
    arn::Arn,
    function::{
        getatt::{Attribute, HaveAtt},
        reference::{RefInner, Referenced},
    },
    logical_id::LogicalId,
    service::IAM,
    tag::Tag,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::SAMLProvider"]
pub struct SAMLProvider {
    logical_id: LogicalId,
    name: Option<SAMLProviderName>,
    saml_metadata_document: Option<SAMLMetadataDocument>,
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Error)]
pub enum SAMLProviderError {
    #[error("invalid name: {0}")]
    InvalidName(String),

    #[error("invalid document: {0}")]
    InvalidDocument(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct SAMLProviderName(String);

impl TryFrom<&str> for SAMLProviderName {
    type Error = SAMLProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 128 {
            return Err(SAMLProviderError::InvalidName(value.to_string()));
        }

        let pattern = Regex::new(r#"[\w._-]+"#).unwrap();
        if !pattern.is_match(value) {
            Err(SAMLProviderError::InvalidName(value.to_string()))
        } else {
            Ok(SAMLProviderName(value.to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SAMLMetadataDocument(String);

impl TryFrom<&str> for SAMLMetadataDocument {
    type Error = SAMLProviderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        if len < 1000 || len > 10000000 {
            Err(SAMLProviderError::InvalidDocument(value.to_string()))
        } else {
            Ok(SAMLMetadataDocument(value.to_string()))
        }
    }
}

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
