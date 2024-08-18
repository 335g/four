use crate::core::logical_id::{LogicalId, LogicalIdentified};
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};

pub trait HaveAtt<A>: LogicalIdentified {
    const KEY: &'static str;
}

#[derive(Debug, Clone)]
pub struct GetAtt {
    logical_id: LogicalId,
    name: &'static str,
}

impl GetAtt {
    pub(crate) fn new<A, H: HaveAtt<A>>(resource: &H) -> GetAtt {
        GetAtt {
            logical_id: resource.logical_id().clone(),
            name: H::KEY,
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
