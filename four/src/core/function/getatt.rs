use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};

use crate::core::logical_id::{LogicalId, LogicalIdentified};

pub trait HaveAtt<A>: LogicalIdentified {}

#[derive(Debug, Clone)]
pub struct GetAtt {
    logical_id: LogicalId,
    name: &'static str,
}

impl GetAtt {
    pub(crate) fn new<A: Attribute>(resource: &dyn HaveAtt<A>) -> GetAtt {
        GetAtt {
            logical_id: resource.logical_id().clone(),
            name: A::name(),
        }
    }
}

impl Serialize for GetAtt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let inner = GetAttInner {
            logical_id: &self.logical_id,
            name: self.name,
        };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Fn::GetAtt", &inner)?;
        map.end()
    }
}

pub trait Attribute {
    fn name() -> &'static str;
}

struct GetAttInner<'a> {
    logical_id: &'a LogicalId,
    name: &'a str,
}

impl Serialize for GetAttInner<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(self.logical_id)?;
        seq.serialize_element(self.name)?;
        seq.end()
    }
}
