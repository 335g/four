use std::collections::HashMap;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::error::Error;

#[derive(Serialize)]
pub struct Template {
    #[serde(rename(serialize = "AWSTemplateFormatVersion"))]
    format_version: String,

    #[serde(rename(serialize = "Description"))]
    description: String,

    #[serde(rename(serialize = "Resources"))]
    resources: HashMap<String, Resource>,
}

#[derive(Serialize)]
pub struct Resource {
    #[serde(rename(serialize = "Type"))]
    r#type: String,

    #[serde(rename(serialize = "Properties"))]
    properties: Map<String, Value>,
}

pub trait IntoResource: Serialize + Sized {
    fn resource_type() -> &'static str;

    fn resource(self) -> Result<Resource, Error> {
        let properties = serde_json::to_value(&self).map_err(|e| Error::InvalidResource)?;
        let ty = Self::resource_type();
        match properties {
            Value::Object(x) => Ok(Resource {
                r#type: ty.to_string(),
                properties: x,
            }),
            _ => return Err(Error::InvalidResource),
        }
    }
}
