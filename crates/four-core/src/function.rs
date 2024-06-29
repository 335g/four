use serde::{ser::SerializeMap, Serialize};

pub struct Ref<T>(T);

impl<T> Ref<T> {
    pub fn new(x: T) -> Ref<T> {
        Ref(x)
    }
}

impl<T> Serialize for Ref<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("Ref", &self.0)?;
        map.end()
    }
}

pub struct Join(Vec<Box<dyn erased_serde::Serialize>>);

impl Join {
    pub(crate) fn new(xs: Vec<Box<dyn erased_serde::Serialize>>) -> Self {
        Self(xs)
    }
}

#[macro_export]
macro_rules! fn_join {
    ($($elem:expr),+) => {
        Join::new(vec![$(Box::new($elem)),*])
    };
}

impl Serialize for Join {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("Fn:Join", &self.0)?;
        map.end()
    }
}
