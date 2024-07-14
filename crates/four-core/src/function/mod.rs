pub mod join;
pub mod reference;

use reference::Ref;

pub fn r#ref<T>(wrapped: T) -> Box<Ref<T>> {
    Box::new(Ref::new(wrapped))
}
