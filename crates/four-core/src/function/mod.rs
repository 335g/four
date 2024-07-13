pub mod join;
pub mod reference;

use reference::Ref;

pub fn r#ref<T>(wrapped: T) -> Box<Ref<T>> {
    Ref::new(wrapped).boxed()
}
