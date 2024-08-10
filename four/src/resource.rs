use dyn_clone::DynClone;

use crate::logical_id::LogicalIdentified;

// TODO: impl unmanaged resource

/// [Resource] section
/// cf. https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/resources-section-structure.html
pub trait ManagedResource: erased_serde::Serialize + DynClone + LogicalIdentified {
    fn resource_type(&self) -> &'static str;
}

erased_serde::serialize_trait_object!(ManagedResource);
dyn_clone::clone_trait_object!(ManagedResource);
