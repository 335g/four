use crate::{function::reference::Ref, pseudo};
use regex::Regex;
use serde::{Serialize, Serializer};
use std::ops::Deref;
use std::sync::LazyLock;

static ACCOUNT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\d{12}"#).unwrap());

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

impl TryFrom<&str> for Account {
    type Error = AccountDetailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let account = AccountDetail::try_from(value)?;
        Ok(Account::Detail(account))
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

#[derive(Debug, Clone)]
pub struct AccountDetail(String);

impl Deref for AccountDetail {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for AccountDetail {
    type Error = AccountDetailError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_string();
        if ACCOUNT_REGEX.is_match(&s) {
            Ok(AccountDetail(s))
        } else {
            Err(AccountDetailError::Invalid(s))
        }
    }
}

impl Serialize for AccountDetail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccountDetailError {
    #[error("Invalid account: {0}")]
    Invalid(String),
}

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
        let account_id = "123456789012";
        let account = AccountDetail::try_from(account_id).unwrap();
        let s = serde_json::to_string(&Account::Detail(account)).unwrap();
        assert_eq!(s, format!("\"{account_id}\""));
    }
}
