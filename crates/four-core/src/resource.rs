use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};

use crate::{logical_id::LogicalIdentified, Parameter};

// TODO: impl
pub trait ReferencedResource {}

/// [Resource] section
/// cf. https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/resources-section-structure.html
pub trait ManagedResource: erased_serde::Serialize + LogicalIdentified {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(ManagedResource);
