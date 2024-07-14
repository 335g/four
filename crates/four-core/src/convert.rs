pub trait WillBe: erased_serde::Serialize {
    type Figure;
}

erased_serde::serialize_trait_object!(WillBe<Figure = String>);
erased_serde::serialize_trait_object!(WillBe<Figure = f64>);

pub type WillBeString = Box<dyn WillBe<Figure = String>>;
pub type WillBeNumber = Box<dyn WillBe<Figure = f64>>;
