use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum AccessKeyStatus {
    Inactive,
    Active,
}

/// Secret access key for the specified [`AccessKey`].
/// For example: wJalrXUtnFEMI/K7MDENG/bPxRfiCYzEXAMPLEKEY.
#[derive(Debug, Clone, Serialize)]
pub enum SecretAccessKey {}
