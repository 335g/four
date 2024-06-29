#![allow(dead_code)]

use serde::Serialize;

#[derive(Serialize)]
pub enum Effect {
    Allow,
    Deny,
}
