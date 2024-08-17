use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::{
    core::{
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        service::IAM,
        Arn, LogicalId, Tag,
    },
    iam::util::Path,
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

#[nutype(
    validate(
        not_empty,
        len_char_max = 16384,
        regex = r#"[\u0009\u000A\u000D\u0020-\u00FF]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct CertificateBody(String);

#[nutype(
    validate(
        not_empty,
        len_char_max = 2097152,
        regex = r#"[\u0009\u000A\u000D\u0020-\u00FF]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct CertificateChain(String);

#[nutype(
    validate(
        not_empty,
        len_char_max = 16384,
        regex = r#"[\u0009\u000A\u000D\u0020-\u00FF]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct PrivateKey(String);

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct ServerCertificateName(String);

#[derive(Debug, Clone, Serialize)]
pub struct ServerCertificateArn(Arn<IAM>);

impl From<Arn<IAM>> for ServerCertificateArn {
    fn from(value: Arn<IAM>) -> Self {
        ServerCertificateArn(value)
    }
}

impl Referenced for ServerCertificate {
    type To = ServerCertificateName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<ServerCertificateArn> for ServerCertificate {}
impl Attribute for ServerCertificateArn {
    fn name() -> &'static str {
        "Arn"
    }
}
