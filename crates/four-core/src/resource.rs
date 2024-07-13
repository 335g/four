use std::collections::HashMap;

use serde::Serialize;

use crate::logical_id::LogicalIdentified;

/// Serialized to become CloudFormation Template
#[derive(Serialize)]
pub struct Template {
    #[serde(rename(serialize = "AWSTemplateFormatVersion"))]
    format_version: String,
    #[serde(rename(serialize = "Description"))]
    description: String,
    #[serde(rename(serialize = "Resources"))]
    resources: HashMap<String, Box<dyn ManagedResource>>,
}

// TODO: impl
pub trait ReferencedResource {}

/// [Resource] section
/// cf. https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/resources-section-structure.html
pub trait ManagedResource: erased_serde::Serialize + LogicalIdentified {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(ManagedResource);
