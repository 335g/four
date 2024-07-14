// #[macro_export]
// macro_rules! impl_will_be {
//     ($($t:ty),*) => {
//         pub trait WillBe: erased_serde::Serialize {
//             type Value;
//         }

//         $(erased_serde::serialize_trait_object!(WillBe<Value = $t>);)*
//     };
// }

// impl_will_be!(String, f64);

// pub type WillBeString = dyn WillBe<Value = String>;
// pub type WillBeNumber = dyn WillBe<Value = f64>;

use std::marker::PhantomData;

use serde::Serialize;

pub trait WillFrom: erased_serde::Serialize {}

impl<T> WillFrom for T where T: Serialize {}

erased_serde::serialize_trait_object!(WillFrom);

#[derive(Serialize)]
pub struct WillBe<'a, T> {
    #[serde(flatten)]
    from: Box<dyn WillFrom + 'a>,

    #[serde(skip)]
    to: PhantomData<T>,
}

impl<'a, T> WillBe<'a, T> {
    pub fn new(from: Box<dyn WillFrom + 'a>) -> Self {
        Self {
            from,
            to: PhantomData,
        }
    }
}
