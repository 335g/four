use std::marker::PhantomData;

use serde::Serialize;

pub trait WillFrom: erased_serde::Serialize {}

impl<T> WillFrom for T where T: Serialize {}

erased_serde::serialize_trait_object!(WillFrom);

pub trait WillMappable<F> {}

impl<F> WillMappable<F> for F {}

pub struct WillBe<T> {
    from: Box<dyn WillFrom>,
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

impl<T> From<Box<T>> for WillBe<T>
where
    T: WillFrom + 'static,
{
    fn from(value: Box<T>) -> Self {
        WillBe {
            from: value,
            to: PhantomData,
        }
    }
}

impl<T> Serialize for WillBe<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.from.serialize(serializer)
    }
}
