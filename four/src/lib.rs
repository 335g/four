#![allow(dead_code)]

mod core;

#[cfg(feature = "iam")]
mod iam;
#[cfg(feature = "lambda")]
mod lambda;
