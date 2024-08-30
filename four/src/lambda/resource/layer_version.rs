use crate::{
    core::{
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    lambda::{
        arn::LayerVersionArn, layer_version::CompatibleRuntimes, CompatibleArchitectures,
        FunctionContent, LayerName, LayerVersionDescription, LicenseInfo,
    },
    ManagedResource,
};

/// [AWS::Lambda::LayerVersion]
///
/// The AWS::Lambda::LayerVersion resource creates a Lambda layer from a ZIP archive.
///
/// ```
/// use four::{
///     LogicalId,
///     lambda::{
///         resource::LayerVersion,
///         Architecture, Runtime, FunctionContent,
///         LayerVersionDescription, LayerName, LicenseInfo
///     },
/// };
///
/// let layer_id = LogicalId::try_from("MyLayer").unwrap();
/// let content = FunctionContent {
///     s3_bucket: "my-bucket-us-west-2-123456789012".to_string(),
///     s3_key: "layer.zip".to_string(),
///     s3_object_version: None,
/// };
/// let layer = LayerVersion::new(layer_id, content)
///     .compatible_architectures(vec![Architecture::X86_64].try_into().unwrap())
///     .compatible_runtimes(vec![Runtime::Python3_6, Runtime::Python3_7].try_into().unwrap())
///     .description("ThisIsMyLayer".try_into().unwrap())
///     .layer_name("my-layer".try_into().unwrap())
///     .license_info("MIT".try_into().unwrap());
///
/// let lhs = serde_json::to_string(&layer).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::LayerVersion",
///         "Properties": {
///             "CompatibleArchitectures": ["x86_64"],
///             "CompatibleRuntimes": ["python3.6", "python3.7"],
///             "Content": {
///                 "S3Bucket": "my-bucket-us-west-2-123456789012",
///                 "S3Key": "layer.zip"
///             },
///             "Description": "ThisIsMyLayer",
///             "LayerName": "my-layer",
///             "LicenseInfo": "MIT"
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
/// ```
///
/// [AWS::Lambda::LayerVersion]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::LayerVersion"]
pub struct LayerVersion {
    logical_id: LogicalId,
    compatible_architectures: Option<CompatibleArchitectures>,
    compatible_runtimes: Option<CompatibleRuntimes>,
    content: FunctionContent,
    description: Option<LayerVersionDescription>,
    layer_name: Option<LayerName>,
    license_info: Option<LicenseInfo>,
}

impl Referenced for LayerVersion {
    type To = LayerVersionArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<LayerVersionArn> for LayerVersion {
    const KEY: &'static str = "LayerVersionArn";
}
