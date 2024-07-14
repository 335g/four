pub mod getatt;
pub mod join;
pub mod reference;

use getatt::{Attribute, GetAtt, HaveAtt};
use reference::Ref;

pub fn r#ref<T>(wrapped: T) -> Box<Ref<T>> {
    Box::new(Ref::new(wrapped))
}

pub fn get_att<A: Attribute, H: HaveAtt<A>>(
    resource: &H,
) -> GetAtt<'_, A, <H as HaveAtt<A>>::Value> {
    GetAtt::new(resource)
}
