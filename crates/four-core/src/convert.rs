pub trait WillBe: erased_serde::Serialize {
    type Value;
}

erased_serde::serialize_trait_object!(WillBe<Value = String>);
erased_serde::serialize_trait_object!(WillBe<Value = f64>);

pub type WillBeString = Box<dyn WillBe<Value = String>>;
pub type WillBeNumber = Box<dyn WillBe<Value = f64>>;
