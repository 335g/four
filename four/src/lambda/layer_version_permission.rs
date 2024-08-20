use nutype::nutype;
use serde::Serialize;

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

#[nutype(
    validate(regex = r#"\d{12}|\*|arn:(aws[a-zA-Z-]*):iam::\d{12}:root"#),
    derive(Debug, Clone, Serialize)
)]
pub struct Principal(String);
