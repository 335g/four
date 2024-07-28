use dyn_clone::DynClone;
use serde::{ser::SerializeMap as _, Serialize};

use crate::function::reference::{RefInner, Referenced};

pub trait JoinElement: erased_serde::Serialize + DynClone {}
erased_serde::serialize_trait_object!(JoinElement);
dyn_clone::clone_trait_object!(JoinElement);

impl<T> JoinElement for T where T: erased_serde::Serialize + Clone {}

#[derive(Clone)]
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

#[macro_export]
macro_rules! fn_join {
    ($join:expr, [$($elem:expr),+]) => {
        {
            let mut all_elements = $join.0;
            let mut new_elements: Vec<Box<dyn erased_serde::Serialize>> = vec![$(Box::new($elem)),*];
            all_elements.append(&mut new_elements);
            Join(all_elements)
        }
    };

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
        map.serialize_entry("Fn::Join", &self.0)?;
        map.end()
    }
}
