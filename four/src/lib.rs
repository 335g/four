#![allow(dead_code)]

pub mod core;

#[cfg(feature = "iam")]
pub mod iam;
#[cfg(feature = "lambda")]
pub mod lambda;
