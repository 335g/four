use serde::{ser::SerializeMap, Serialize};

use crate::logical_id::{LogicalId, LogicalIdentified};

pub(crate) trait Referenced {
    type To: Serialize;

    fn r#ref(&self) -> Self::To;
}

impl<T> Referenced for T
where
    T: LogicalIdentified,
{
    type To = LogicalId;

    fn r#ref(&self) -> <Self as Referenced>::To {
        self.logical_id().clone()
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
        map.serialize_entry("Ref", &self.0.r#ref())?;
        map.end()
    }
}
