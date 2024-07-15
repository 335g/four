use crate::function::reference::{RefInner, Referenced};
use serde::Serialize;

macro_rules! pseudo_param {
    ($($name:ident),*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum PseudoParam {
            $($name($name)),*
        }

        impl Serialize for PseudoParam {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(PseudoParam::$name(x) => x.serialize(serializer)),*
                }
            }
        }

        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&format!("AWS::{}", stringify!($name)))
                }
            }

            impl Referenced for $name {
                fn referenced(self) -> RefInner {
                    RefInner::PseudoParam(PseudoParam::$name($name))
                }
            }
        )*
    };
}

pseudo_param!(
    AccountId,
    NotificationARNs,
    NoValue,
    Partition,
    Region,
    StackId,
    StackName,
    URLSuffix
);
