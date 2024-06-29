use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct Template {
    #[serde(rename(serialize = "AWSTemplateFormatVersion"))]
    format_version: String,
    #[serde(rename(serialize = "Description"))]
    description: String,
    #[serde(rename(serialize = "Resources"))]
    resources: HashMap<String, Box<dyn Construct>>,
}

pub trait Construct: erased_serde::Serialize {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(Construct);
