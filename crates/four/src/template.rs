use std::collections::HashMap;

use serde::{ser::SerializeMap, Serialize};

use crate::{
    logical_id::{LogicalId, LogicalIdentified},
    parameter::Parameter,
    resource::ManagedResource,
};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum TemplateVersion {
    V20100909,
}

impl TemplateVersion {
    pub fn latest() -> TemplateVersion {
        TemplateVersion::V20100909
    }
}

impl Serialize for TemplateVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TemplateVersion::V20100909 => "2010-09-09".serialize(serializer),
        }
    }
}

/// Serialized to be
pub struct Template {
    format_version: TemplateVersion,
    description: Option<String>,
    resources: HashMap<LogicalId, Box<dyn ManagedResource>>,
    string_parameters: Vec<Parameter<String>>,
    number_parameters: Vec<Parameter<f64>>,
}

impl Template {
    pub fn new(
        string_parameters: Vec<Parameter<String>>,
        number_parameters: Vec<Parameter<f64>>,
    ) -> Self {
        Self {
            format_version: TemplateVersion::latest(),
            description: None,
            resources: HashMap::new(),
            string_parameters,
            number_parameters,
        }
    }

    pub fn insert(
        &mut self,
        resource: Box<dyn ManagedResource>,
    ) -> Option<Box<dyn ManagedResource>> {
        self.resources
            .insert(resource.logical_id().clone(), resource)
    }
}

impl Serialize for Template {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("AWSTemplateFormatVersion", &self.format_version)?;
        if !self.string_parameters.is_empty() || !self.number_parameters.is_empty() {
            let inner = SerParameter {
                strings: &self.string_parameters,
                numbers: &self.number_parameters,
            };

            map.serialize_entry("Parameters", &inner)?;
        }
        if let Some(description) = &self.description {
            map.serialize_entry("Description", description)?;
        }
        map.serialize_entry("Resources", &self.resources)?;

        map.end()
    }
}

struct SerParameter<'a> {
    strings: &'a [Parameter<String>],
    numbers: &'a [Parameter<f64>],
}

impl Serialize for SerParameter<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = self.strings.len() + self.numbers.len();
        let mut map = serializer.serialize_map(Some(len))?;
        for e in self.strings {
            map.serialize_entry(e.logical_id(), e)?;
        }
        for e in self.numbers {
            map.serialize_entry(e.logical_id(), e)?;
        }
        map.end()
    }
}
