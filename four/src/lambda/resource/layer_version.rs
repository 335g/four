use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::{
    core::{
        convert::WillMappable,
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        logical_id::LogicalId,
        service::IAM,
        Arn,
    },
    lambda::property::{architecture::Architecture, runtime::Runtime},
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::LayerVersion"]
pub struct LayerVersion {
    logical_id: LogicalId,
    compatible_architectures: Option<CompatibleArchitectures>,
    compatible_runtimes: Option<CompatibleRuntimes>,
    content: Content,
    description: Option<Description>,
    layer_name: Option<LayerName>,
    license_info: Option<LicenseInfo>,
}

#[nutype(
    validate(predicate = |architectures| architectures.len() <= 2),
    derive(Debug, Clone, Serialize),
)]
pub struct CompatibleArchitectures(Vec<Architecture>);

#[nutype(
    validate(predicate = |runtimes| runtimes.len() <= 15),
    derive(Debug, Clone, Serialize),
)]
pub struct CompatibleRuntimes(Vec<Runtime>);

#[derive(Debug, Clone, Serialize)]
pub struct Content {
    // TODO: define type in response to Amazon S3 support feature
    s3_bucket: String,
    s3_key: String,
    s3_object_version: String,
}

#[nutype(
    validate(len_char_min = 0, len_char_max = 256),
    derive(Debug, Clone, Serialize)
)]
pub struct Description(String);

#[nutype(
    validate(
        len_char_min = 1,
        len_char_max = 140,
        regex = r#"(arn:[a-zA-Z0-9-]+:lambda:[a-zA-Z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+)|[a-zA-Z0-9-_]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct LayerName(String);

#[nutype(validate(len_char_max = 512), derive(Debug, Clone, Serialize))]
pub struct LicenseInfo(String);

#[derive(Debug, Clone, Serialize)]
pub struct LayerVersionArn(Arn<IAM>);

impl Referenced for LayerVersion {
    type To = LayerVersionArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl WillMappable<LayerName> for LayerVersionArn {}

impl HaveAtt<LayerVersionArn> for LayerVersion {}
impl Attribute for LayerVersionArn {
    fn name() -> &'static str {
        "LayerVersionArn"
    }
}
