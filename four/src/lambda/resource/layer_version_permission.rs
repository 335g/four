use crate::{
    core::{
        function::{RefInner, Referenced},
        LogicalId,
    },
    lambda::{
        GetLayerVersionAction, LayerVersionArn, LayerVersionPermissionId, LayerVersionPrincipal,
        OrganizationId,
    },
    ManagedResource,
};

/// [AWS::Lambda::LayerVersionPermission]
///
/// The AWS::Lambda::LayerVersionPermission resource adds permissions to the
/// resource-based policy of a version of an Lambda layer. Use this action
/// to grant layer usage permission to other accounts. You can grant permission
/// to a single account, all AWS accounts, or all accounts in an organization.
///
/// ```
/// use four::{
///     LogicalId, arn_builder, Account, Region, RegionDetail, service, Partition,
///     lambda::{
///         resource::LayerVersionPermission,
///         GetLayerVersionAction, LayerName, OrganizationId, LayerVersionPrincipal,
///     },
/// };
///
/// let permission_id = LogicalId::try_from("permission").unwrap();
/// let action = GetLayerVersionAction;
/// let account = Account::try_from("123456789012").unwrap();
/// let layer_version_arn = arn_builder("layer:my-layer:1", account)
///     .region(Region::Detail(RegionDetail::UsEast1))
///     .partition(Partition::Aws)
///     .build(service::Lambda)
///     .into();
/// let principal = LayerVersionPrincipal::Any;
/// let permission = LayerVersionPermission::new(permission_id, action, layer_version_arn, principal)
///     .organization_id(OrganizationId::try_new("o-t194hfs8cz").unwrap());
///
/// let lhs = serde_json::to_string(&permission).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::LayerVersionPermission",
///         "Properties": {
///             "Action": "lambda:GetLayerVersion",
///             "LayerVersionArn": "arn:aws:lambda:us-east-1:123456789012:layer:my-layer:1",
///             "OrganizationId": "o-t194hfs8cz",
///             "Principal": "*"
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
/// ```
///
/// [AWS::Lambda::LayerVersionPermission]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::LayerVersionPermission"]
pub struct LayerVersionPermission {
    logical_id: LogicalId,
    action: GetLayerVersionAction,
    layer_version_arn: LayerVersionArn,
    organization_id: Option<OrganizationId>,
    principal: LayerVersionPrincipal,
}

impl Referenced for LayerVersionPermission {
    type To = LayerVersionPermissionId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
