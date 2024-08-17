use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

use crate::{
    core::{AccountDetail, AnyArn, LogicalId},
    iam::{action::lambda::LambdaAction, ServicePrincipal},
    lambda::{property::function_name::FunctionName, resource::url::AuthType},
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Permission"]
pub struct Permission {
    logical_id: LogicalId,
    action: Box<dyn LambdaAction>,
    event_source_token: Option<EventSourceToken>,
    function_name: FunctionName,
    function_url_auth_type: Option<AuthType>,
    principal: Option<Principal>,

    // TODO: rename (four_derive)
    principal_org_i_d: Option<PrincipalOrgID>,
    source_account: Option<AccountDetail>,
    source_arn: Option<AnyArn>,
}

#[nutype(
    validate(not_empty, len_char_max = 256, regex = r#"^[a-zA-Z0-9._\-]+$"#),
    derive(Debug, Clone, Serialize)
)]
pub struct EventSourceToken(String);

#[derive(Debug, Clone)]
enum Principal {
    Service(ServicePrincipal),
    Any,
    Account(AccountDetail),
}

impl Serialize for Principal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Principal::Service(x) => x.serialize(serializer),
            Principal::Any => "*".serialize(serializer),
            Principal::Account(x) => x.serialize(serializer),
        }
    }
}

#[nutype(
    validate(len_char_min = 12, len_char_max = 34, regex = r#"^o-[a-z0-9]{10,32}$"#),
    derive(Debug, Clone, Serialize)
)]
pub struct PrincipalOrgID(String);
