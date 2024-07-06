use std::collections::HashMap;

use serde::Serialize;

use crate::{function::reference::Referenced, logical_id::LogicalIdentified, LogicalId};

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
pub trait ReferencedResource: LogicalIdentified {}

pub trait ManagedResource: erased_serde::Serialize + LogicalIdentified {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(ManagedResource);
