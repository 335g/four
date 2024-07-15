pub mod getatt;
pub mod join;
pub mod reference;

use getatt::{Attribute, GetAtt, HaveAtt};
use reference::{Ref, Referenced};

use crate::WillBe;

pub fn r#ref<R: Referenced, U>(wrapped: R) -> WillBe<U> {
    WillBe::new(Box::new(Ref::new(wrapped)))
}

pub fn get_att<A: Attribute, H: HaveAtt<A>>(resource: &H) -> WillBe<A> {
    WillBe::new(Box::new(GetAtt::new(resource)))
}
