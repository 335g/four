#![allow(dead_code)]

use serde::{Serialize, Serializer};

use crate::{function::Ref, pseudo_param};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Region {
    Ref,
    UsEast1,
    ApNortheast1,
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Region::Ref => Ref::new(pseudo_param::Region).serialize(serializer),
            Region::UsEast1 => "us-east-1".serialize(serializer),
            Region::ApNortheast1 => "ap-northeast-1".serialize(serializer),
        }
    }
}
