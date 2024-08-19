use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        LogicalId, Tag,
    },
    iam::{SAMLMetadataDocument, SAMLProviderArn, SAMLProviderName},
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::SAMLProvider"]
pub struct SAMLProvider {
    logical_id: LogicalId,
    name: Option<SAMLProviderName>,
    saml_metadata_document: Option<SAMLMetadataDocument>,
    tags: Option<Vec<Tag>>,
}

impl Referenced for SAMLProvider {
    type To = SAMLProviderArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<SAMLProviderArn> for SAMLProvider {
    const KEY: &'static str = "Arn";
}
