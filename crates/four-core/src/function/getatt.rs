use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};

use crate::logical_id::{LogicalId, LogicalIdentified};

pub trait HaveAtt<A>: LogicalIdentified {}

pub struct GetAtt<'a, A> {
    resource: &'a dyn HaveAtt<A>,
}

impl<'a, A> GetAtt<'a, A>
where
    A: Attribute,
{
    pub(crate) fn new(resource: &'a dyn HaveAtt<A>) -> GetAtt<'a, A> {
        GetAtt { resource }
    }
}

impl<A> Serialize for GetAtt<'_, A>
where
    A: Attribute,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let inner = GetAttInner {
            logical_id: self.resource.logical_id(),
            name: A::name(),
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
