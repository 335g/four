use nutype::nutype;
use serde::{Serialize, Serializer};

use crate::{function::reference::Ref, pseudo};

#[derive(Debug, Clone)]
pub enum Account {
    Ref,
    Null,
    Aws,
    Detail(AccountDetail),
}

impl Account {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Account::Ref => None,
            Account::Null => Some(""),
            Account::Aws => Some("aws"),
            Account::Detail(s) => Some(s.as_str()),
        }
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Account::Ref => Ref::new(pseudo::AccountId).serialize(serializer),
            Account::Null => "".serialize(serializer),
            Account::Aws => "aws".serialize(serializer),
            Account::Detail(x) => x.serialize(serializer),
        }
    }
}

#[nutype(validate(regex = r#"\d{8}"#), derive(Debug, Clone, Serialize, Deref))]
pub struct AccountDetail(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_ref() {
        let s = serde_json::to_string(&Account::Ref).unwrap();
        assert_eq!(s, r#"{"Ref":"AWS::AccountId"}"#);
    }

    #[test]
    fn test_account_null() {
        let s = serde_json::to_string(&Account::Null).unwrap();
        assert_eq!(s, r#""""#);
    }

    #[test]
    fn test_account_aws() {
        let s = serde_json::to_string(&Account::Aws).unwrap();
        assert_eq!(s, r#""aws""#);
    }

    #[test]
    fn test_account_detail() {
        let account_id = "12345678";
        let account = AccountDetail::new(account_id).unwrap();
        let s = serde_json::to_string(&Account::Detail(account)).unwrap();
        assert_eq!(s, format!("\"{account_id}\""));
    }
}
