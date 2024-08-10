use dyn_clone::DynClone;

pub trait Action: erased_serde::Serialize + DynClone + std::fmt::Debug {}

erased_serde::serialize_trait_object!(Action);
dyn_clone::clone_trait_object!(Action);

macro_rules! impl_action {
    ( $service:ident, [$($t:ident),*] ) => {
        pub mod $service {
            use serde::Serialize;
            $(
            #[derive(Debug, Clone, Copy)]
            pub struct $t;

            impl super::Action for $t {}

            impl Serialize for $t {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let s = format!("{}:{}", stringify!($service), stringify!($t));
                    serializer.serialize_str(&s)
                }
            })*
        }
    };
}

impl_action!(sts, [AssumeRole]);
