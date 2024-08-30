use crate::core::function::reference::{RefInner, Referenced};
use dyn_clone::DynClone;
use serde::{ser::SerializeMap as _, Serialize};
use std::fmt::Debug;

pub trait JoinElement: erased_serde::Serialize + DynClone + Debug {}
erased_serde::serialize_trait_object!(JoinElement);
dyn_clone::clone_trait_object!(JoinElement);

impl<T> JoinElement for T where T: erased_serde::Serialize + Clone + Debug {}

#[derive(Debug, Clone)]
pub struct Join(pub(crate) Vec<Box<dyn JoinElement>>);

impl Join {
    #[allow(dead_code)]
    pub(crate) fn new(xs: Vec<Box<dyn JoinElement>>) -> Self {
        Self(xs)
    }
}

impl Referenced for Join {
    type To = String;

    fn referenced(&self) -> RefInner {
        RefInner::Join(self.clone())
    }
}

impl Serialize for Join {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let head: Box<dyn JoinElement> = Box::new("");
        let tail: Box<dyn JoinElement> = Box::new(&self.0);
        let value = vec![head, tail];

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("Fn::Join", &value)?;
        map.end()
    }
}
