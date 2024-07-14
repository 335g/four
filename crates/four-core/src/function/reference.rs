use serde::{ser::SerializeMap, Serialize};

use crate::{
    logical_id::{LogicalId, LogicalIdentified},
    Parameter, WillBe,
};

pub trait Referenced {
    type Ref: Serialize;

    fn referenced(&self) -> &Self::Ref;
}

impl<T> Referenced for T
where
    T: LogicalIdentified,
{
    type Ref = LogicalId;

    fn referenced(&self) -> &<Self as Referenced>::Ref {
        self.logical_id()
    }
}

pub struct Ref<T>(T);

impl<T> Ref<T> {
    pub fn new(x: T) -> Ref<T> {
        Ref(x)
    }
}

impl<T> Serialize for Ref<T>
where
    T: Referenced,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Ref", self.0.referenced())?;
        map.end()
    }
}

impl<T> WillBe for Ref<Parameter<T>> {
    type Figure = T;
}
