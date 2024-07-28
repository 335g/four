use crate::{
    account::Account,
    arn::Arn,
    function::reference::{RefInner, Referenced},
    service::{CloudFormation, SimpleNotification},
};
use serde::Serialize;

macro_rules! pseudo_param {
    ($(($name:ident, $to:ty)),*) => {
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
                type To = $to;
                fn referenced(&self) -> RefInner {
                    RefInner::PseudoParam(PseudoParam::$name($name))
                }
            }
        )*
    };
}

pseudo_param!(
    (AccountId, Account),
    (NotificationARNs, Vec<Arn<SimpleNotification>>),
    (NoValue, ()),
    (Partition, String),
    (Region, String),
    (StackId, Arn<CloudFormation>),
    (StackName, String),
    (URLSuffix, String)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_id() {
        let s = serde_json::to_string(&AccountId).unwrap();
        assert_eq!(s, r#""AWS::AccountId""#);
    }

    #[test]
    fn test_notification_arns() {
        let s = serde_json::to_string(&NotificationARNs).unwrap();
        assert_eq!(s, r#""AWS::NotificationARNs""#);
    }

    #[test]
    fn test_no_value() {
        let s = serde_json::to_string(&NoValue).unwrap();
        assert_eq!(s, r#""AWS::NoValue""#);
    }

    #[test]
    fn test_partition() {
        let s = serde_json::to_string(&Partition).unwrap();
        assert_eq!(s, r#""AWS::Partition""#);
    }

    #[test]
    fn test_region() {
        let s = serde_json::to_string(&Region).unwrap();
        assert_eq!(s, r#""AWS::Region""#);
    }

    #[test]
    fn test_stack_id() {
        let s = serde_json::to_string(&StackId).unwrap();
        assert_eq!(s, r#""AWS::StackId""#);
    }

    #[test]
    fn test_stack_name() {
        let s = serde_json::to_string(&StackName).unwrap();
        assert_eq!(s, r#""AWS::StackName""#);
    }

    #[test]
    fn test_url_suffix() {
        let s = serde_json::to_string(&URLSuffix).unwrap();
        assert_eq!(s, r#""AWS::URLSuffix""#);
    }
}
