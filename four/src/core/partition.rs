use serde::{Serialize, Serializer};

use crate::core::{function::reference::Ref, pseudo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Partition {
    Ref,
    Aws,
    China,
    GovCloudUS,
}

impl Partition {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Partition::Ref => None,
            Partition::Aws => Some("aws"),
            Partition::China => Some("aws-cn"),
            Partition::GovCloudUS => Some("aws-us-gov"),
        }
    }
}

impl Serialize for Partition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Partition::Ref => Ref::new(pseudo::Partition).serialize(serializer),
            Partition::Aws => "aws".serialize(serializer),
            Partition::China => "aws-cn".serialize(serializer),
            Partition::GovCloudUS => "aws-us-gov".serialize(serializer),
        }
    }
}
