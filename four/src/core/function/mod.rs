mod getatt;
mod join;
mod reference;
mod sub;

pub use getatt::{GetAtt, HaveAtt};
pub use join::Join;
pub(crate) use join::JoinElement;
pub(crate) use reference::RefInner;
pub use reference::{Ref, Referenced};

use crate::core::convert::WillBe;

pub fn r#ref<R: Referenced>(wrapped: R) -> WillBe<R::To> {
    WillBe::new(Box::new(Ref::new(wrapped)))
}

pub fn get_att<A, H: HaveAtt<A>>(resource: &H) -> WillBe<A> {
    WillBe::new(Box::new(GetAtt::new(resource)))
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;
    use crate::core::{
        logical_id::{LogicalId, LogicalIdentified},
        pseudo::AccountId,
        resource::ManagedResource,
    };

    #[test]
    fn test_ref_pseudo_param() {
        let param = r#ref(AccountId);
        let s = serde_json::to_string(&param).unwrap();
        assert_eq!(s, r#"{"Ref":"AWS::AccountId"}"#);
    }

    #[test]
    fn test_get_att() {
        #[derive(Clone, Serialize)]
        struct A {
            id: LogicalId,
        }

        impl ManagedResource for A {
            fn resource_type(&self) -> &'static str {
                "resource type of A"
            }
        }

        impl LogicalIdentified for A {
            fn logical_id(&self) -> &crate::core::LogicalId {
                &self.id
            }
        }

        struct B;

        impl HaveAtt<B> for A {
            const KEY: &'static str = "name of B";
        }

        let a = A {
            id: LogicalId::try_from("id-of-A").unwrap(),
        };
        let s = get_att(&a);
        let s = serde_json::to_string(&s).unwrap();
        assert_eq!(s, r#"{"Fn::GetAtt":["id-of-A","name of B"]}"#);
    }
}
