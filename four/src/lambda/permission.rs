use nutype::nutype;
use serde::Serialize;

use crate::{iam::ServicePrincipal, AccountDetail};

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
