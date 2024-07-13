pub trait WillBe: erased_serde::Serialize {
    type Figure;
}

erased_serde::serialize_trait_object!(WillBe<Figure = String>);

pub type WillBeString = Box<dyn WillBe<Figure = String>>;
