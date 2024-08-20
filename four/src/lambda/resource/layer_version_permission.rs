use crate::{
    core::{
        function::{RefInner, Referenced},
        LogicalId,
    },
    lambda::{
        GetLayerVersionAction, LayerName, LayerVersionPermissionId, OrganizationId, Principal,
    },
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::LayerVersionPermission"]
pub struct LayerVersionPermission {
    logical_id: LogicalId,
    action: GetLayerVersionAction,
    layer_version_arn: LayerName,
    organization_id: Option<OrganizationId>,
    principal: Principal,
}

impl Referenced for LayerVersionPermission {
    type To = LayerVersionPermissionId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
