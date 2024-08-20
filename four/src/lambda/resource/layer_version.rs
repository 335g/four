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
