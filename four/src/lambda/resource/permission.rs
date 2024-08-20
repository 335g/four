use crate::{
    core::{AccountDetail, AnyArn, LogicalId},
    iam::action::lambda::LambdaAction,
    lambda::{AuthType, EventSourceToken, LooseFunctionName, PermissionPrincipal, PrincipalOrgID},
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Permission"]
pub struct Permission {
    logical_id: LogicalId,
    action: Box<dyn LambdaAction>,
    event_source_token: Option<EventSourceToken>,
    function_name: LooseFunctionName,
    function_url_auth_type: Option<AuthType>,
    principal: Option<PermissionPrincipal>,

    // TODO: rename (four_derive)
    principal_org_i_d: Option<PrincipalOrgID>,
    source_account: Option<AccountDetail>,
    source_arn: Option<AnyArn>,
}
