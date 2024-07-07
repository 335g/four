use std::collections::HashMap;

use serde::Serialize;

use crate::logical_id::{LogicalId, LogicalIdentified};

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

pub trait ManagedResource: erased_serde::Serialize {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(ManagedResource);

pub struct IdentifiedResource {
    logical_id: LogicalId,
    resource: Box<dyn ManagedResource>,
}

impl IdentifiedResource {
    pub fn new<R: ManagedResource + 'static>(logical_id: LogicalId, resource: R) -> Self {
        let resource: Box<dyn ManagedResource> = Box::new(resource);
        Self {
            logical_id,
            resource,
        }
    }
}

impl LogicalIdentified for IdentifiedResource {
    fn logical_id(&self) -> LogicalId {
        self.logical_id.clone()
    }
}
