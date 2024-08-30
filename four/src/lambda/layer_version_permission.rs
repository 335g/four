use nutype::nutype;
use serde::Serialize;

use crate::AccountDetail;

#[derive(Debug, Clone)]
pub struct GetLayerVersionAction;

impl Serialize for GetLayerVersionAction {
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

#[derive(Debug, Clone)]
pub enum LayerVersionPrincipal {
    Account(AccountDetail),
    Any,
}

impl Serialize for LayerVersionPrincipal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LayerVersionPrincipal::Account(x) => x.serialize(serializer),
            LayerVersionPrincipal::Any => "*".serialize(serializer),
        }
    }
}
