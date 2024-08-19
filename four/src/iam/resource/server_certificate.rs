use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        LogicalId, Tag,
    },
    iam::{
        CertificateBody, CertificateChain, Path, PrivateKey, ServerCertificateArn,
        ServerCertificateName,
    },
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::ServerCertificate"]
pub struct ServerCertificate {
    logical_id: LogicalId,
    certificate_body: Option<CertificateBody>,
    certificate_chain: Option<CertificateChain>,
    path: Option<Path>,
    private_key: Option<PrivateKey>,
    tags: Option<Vec<Tag>>,
}

impl Referenced for ServerCertificate {
    type To = ServerCertificateName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<ServerCertificateArn> for ServerCertificate {
    const KEY: &'static str = "Arn";
}
