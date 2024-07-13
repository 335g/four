use crate::function::reference::Referenced;
use serde::Serialize;

macro_rules! pseudo_param {
    ($name:ident) => {
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
            type Ref = Self;

            fn referenced(&self) -> &Self {
                self
            }
        }
    };
}

pseudo_param!(AccountId);
pseudo_param!(NotificationARNs);
pseudo_param!(NoValue);
pseudo_param!(Partition);
pseudo_param!(Region);
pseudo_param!(StackId);
pseudo_param!(StackName);
pseudo_param!(URLSuffix);
