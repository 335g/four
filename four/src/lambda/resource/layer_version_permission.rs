use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::{
    core::{
        function::reference::{RefInner, Referenced},
        LogicalId,
    },
    lambda::resource::layer_version::LayerName,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::LayerVersionPermission"]
pub struct LayerVersionPermission {
    logical_id: LogicalId,
    action: Action,
    layer_version_arn: LayerName,
    organization_id: Option<OrganizationId>,
    principal: Principal,
}

#[derive(Debug, Clone)]
pub struct Action;

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("lambda:GetLayerVersion")
    }
}

#[nutype(
    validate(len_char_max = 34, regex = r#"o-[a-z0-9]{10,32}"#),
    derive(Debug, Clone, Serialize)
)]
pub struct OrganizationId(String);

#[nutype(
    validate(regex = r#"\d{12}|\*|arn:(aws[a-zA-Z-]*):iam::\d{12}:root"#),
    derive(Debug, Clone, Serialize)
)]
pub struct Principal(String);

#[derive(Debug, Clone, Serialize)]
pub struct LayerVersionPermissionId;

impl Referenced for LayerVersionPermission {
    type To = LayerVersionPermissionId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
