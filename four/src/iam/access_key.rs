use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum AccessKeyStatus {
    Inactive,
    Active,
}

#[derive(Debug)]
pub struct AccessKeyId;

#[derive(Debug, Clone, Serialize)]
pub struct SecretAccessKey(String);
