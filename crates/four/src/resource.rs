use std::collections::HashMap;

use proc_macro::TokenStream;
use serde::Serialize;
use syn::parse_macro_input;

#[derive(Serialize)]
pub struct Template {
    // #[serde(rename(serialize = "AWSTemplateFormatVersion"))]
    format_version: String,
    // #[serde(rename(serialize = "Description"))]
    description: String,
    // #[serde(rename(serialize = "Resources"))]
    resources: HashMap<String, Box<dyn Resource>>,
}

pub trait Resource: erased_serde::Serialize {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(Resource);
