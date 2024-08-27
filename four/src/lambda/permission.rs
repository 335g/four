use nutype::nutype;
use serde::Serialize;

use crate::{iam::ServicePrincipal, AccountDetail, AccountDetailError};

#[nutype(
    validate(not_empty, len_char_max = 256, regex = r#"^[a-zA-Z0-9._\-]+$"#),
    derive(Debug, Clone, Serialize)
)]
pub struct EventSourceToken(String);

#[derive(Debug, Clone)]
pub enum PermissionPrincipal {
    Service(ServicePrincipal),
    Any,
    Account(AccountDetail),
}

impl From<ServicePrincipal> for PermissionPrincipal {
    fn from(value: ServicePrincipal) -> Self {
        PermissionPrincipal::Service(value)
    }
}

impl From<AccountDetail> for PermissionPrincipal {
    fn from(value: AccountDetail) -> Self {
        PermissionPrincipal::Account(value)
    }
}

impl TryFrom<&str> for PermissionPrincipal {
    type Error = AccountDetailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let account = AccountDetail::try_from(value)?;
        let principal = PermissionPrincipal::Account(account);
        Ok(principal)
    }
}

impl Serialize for PermissionPrincipal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PermissionPrincipal::Service(x) => x.serialize(serializer),
            PermissionPrincipal::Any => "*".serialize(serializer),
            PermissionPrincipal::Account(x) => x.serialize(serializer),
        }
    }
}

#[nutype(
    validate(len_char_min = 12, len_char_max = 34, regex = r#"^o-[a-z0-9]{10,32}$"#),
    derive(Debug, Clone, Serialize)
)]
pub struct PrincipalOrgID(String);
