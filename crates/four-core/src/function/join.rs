use serde::{ser::SerializeMap as _, Serialize};

pub struct Join(pub(crate) Vec<Box<dyn erased_serde::Serialize>>);

impl Join {
    pub(crate) fn new(xs: Vec<Box<dyn erased_serde::Serialize>>) -> Self {
        Self(xs)
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
