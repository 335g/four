use serde::{ser::SerializeMap, Serialize};

use crate::core::{function::join::Join, logical_id::LogicalId, pseudo::PseudoParam};
pub trait Referenced {
    type To;

    fn referenced(&self) -> RefInner;
}

#[derive(Debug, Clone)]
pub enum RefInner {
    Id(LogicalId),
    PseudoParam(PseudoParam),
    Join(Join),
}

impl Serialize for RefInner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RefInner::Id(id) => id.serialize(serializer),
            RefInner::PseudoParam(param) => param.serialize(serializer),
            RefInner::Join(join) => join.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ref {
    inner: RefInner,
}

impl Ref {
    pub fn new<R: Referenced>(r: R) -> Ref {
        Ref {
            inner: r.referenced(),
        }
    }
}

impl Serialize for Ref {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Ref", &self.inner)?;
        map.end()
    }
}
