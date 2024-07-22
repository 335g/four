use std::marker::PhantomData;

use serde::Serialize;

pub trait WillFrom: erased_serde::Serialize {}

impl<T> WillFrom for T where T: Serialize {}

erased_serde::serialize_trait_object!(WillFrom);

pub trait WillMappable<F> {}

impl<F> WillMappable<F> for F {}

#[derive(Serialize)]
pub struct WillBe<T> {
    #[serde(flatten)]
    from: Box<dyn WillFrom>,

    #[serde(skip)]
    to: PhantomData<T>,
}

impl<T> WillBe<T> {
    pub(crate) fn new(from: Box<dyn WillFrom>) -> Self {
        Self {
            from,
            to: PhantomData,
        }
    }

    pub fn map<U>(self) -> WillBe<U>
    where
        U: WillMappable<T>,
    {
        WillBe {
            from: self.from,
            to: PhantomData,
        }
    }
}

impl<T> From<T> for WillBe<T>
where
    T: WillFrom + 'static,
{
    fn from(value: T) -> Self {
        WillBe {
            from: Box::new(value),
            to: PhantomData,
        }
    }
}
