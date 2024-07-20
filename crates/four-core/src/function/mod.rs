pub mod getatt;
pub mod join;
pub mod reference;
pub mod sub;

use getatt::{Attribute, GetAtt, HaveAtt};
use reference::{Ref, Referenced};

use crate::WillBe;

pub fn r#ref<R: Referenced, U>(wrapped: R) -> WillBe<U> {
    WillBe::new(Box::new(Ref::new(wrapped)))
}

pub fn get_att<A: Attribute, H: HaveAtt<A>>(resource: &H) -> WillBe<A> {
    WillBe::new(Box::new(GetAtt::new(resource)))
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;
    use crate::{
        logical_id::{LogicalId, LogicalIdentified},
        parameter::Parameter,
        pseudo::AccountId,
        resource::ManagedResource,
    };

    #[test]
    fn test_ref_pseudo_param() {
        let param = r#ref::<_, String>(AccountId);
        let s = serde_json::to_string(&param).unwrap();
        assert_eq!(s, r#"{"Ref":"AWS::AccountId"}"#);
    }

    #[test]
    fn test_ref_param() {
        let param_id = LogicalId::try_from("id-of-param").unwrap();
        let param = Parameter::string().build(param_id).unwrap();
        let param = r#ref::<_, String>(&param);
        let s = serde_json::to_string(&param).unwrap();
        assert_eq!(s, r#"{"Ref":"id-of-param"}"#);
    }

    #[test]
    fn test_get_att() {
        #[derive(Serialize)]
        struct A {
            id: LogicalId,
        }

        impl ManagedResource for A {
            fn resource_type(&self) -> &'static str {
                "resource type of A"
            }
        }

        impl LogicalIdentified for A {
            fn logical_id(&self) -> &crate::logical_id::LogicalId {
                &self.id
            }
        }

        struct B;

        impl HaveAtt<B> for A {}

        impl Attribute for B {
            fn name() -> &'static str {
                "name of B"
            }
        }

        let a = A {
            id: LogicalId::try_from("id-of-A").unwrap(),
        };
        let s = get_att(&a);
        let s = serde_json::to_string(&s).unwrap();
        assert_eq!(s, r#"{"Fn::GetAtt":["id-of-A","name of B"]}"#);
    }
}
