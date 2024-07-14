pub mod getatt;
pub mod join;
pub mod reference;

use getatt::{Attribute, GetAtt, HaveAtt};
use reference::Ref;

use crate::{logical_id::LogicalIdentified, WillBe};

pub fn r#ref<T: LogicalIdentified + 'static, U>(wrapped: T) -> WillBe<'static, U> {
    WillBe::new(Box::new(Ref::new(wrapped)))
}

pub fn get_att<'a, A: Attribute + 'a, H: HaveAtt<A>, U>(resource: &'a H) -> WillBe<U> {
    WillBe::new(Box::new(GetAtt::new(resource)))
}
