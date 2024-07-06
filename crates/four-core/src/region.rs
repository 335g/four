#![allow(dead_code)]

use serde::{Serialize, Serializer};

use crate::{function::reference::Ref, pseudo_param};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Region {
    Ref,
    Null,
    Detail(RegionDetail),
}

impl Region {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Region::Ref => None,
            Region::Null => Some(""),
            Region::Detail(s) => Some(s.to_str()),
        }
    }
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Region::Ref => Ref::new(pseudo_param::Region).serialize(serializer),
            Region::Null => "".serialize(serializer),
            Region::Detail(x) => x.serialize(serializer),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum RegionDetail {
    UsEast1,
    ApNortheast1,
}

impl RegionDetail {
    fn to_str(&self) -> &'static str {
        match self {
            RegionDetail::UsEast1 => "us-east-1",
            RegionDetail::ApNortheast1 => "ap-northeast-1",
        }
    }
}

impl Serialize for RegionDetail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_str().serialize(serializer)
    }
}
