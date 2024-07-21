#![allow(dead_code)]

use serde::{Serialize, Serializer};

use crate::{function::reference::Ref, pseudo};

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
            Region::Ref => Ref::new(pseudo::Region).serialize(serializer),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_ref() {
        let s = serde_json::to_string(&Region::Ref).unwrap();
        assert_eq!(s, r#"{"Ref":"AWS::Region"}"#);
    }

    #[test]
    fn test_region_null() {
        let s = serde_json::to_string(&Region::Null).unwrap();
        assert_eq!(s, r#""""#);
    }

    #[test]
    fn test_region_detail1() {
        let s = serde_json::to_string(&Region::Detail(RegionDetail::ApNortheast1)).unwrap();
        assert_eq!(s, r#""ap-northeast-1""#);
    }
}
