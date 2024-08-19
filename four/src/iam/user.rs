use crate::iam::PolicyDocument;
use nutype::nutype;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginProfile {
    password: String,
    password_reset_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Policy {
    policy_document: PolicyDocument,
    policy_name: String,
}

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct UserName(String);
