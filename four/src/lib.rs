#![allow(dead_code)]
#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod core;
pub use core::*;

#[cfg(feature = "iam")]
#[cfg_attr(docsrs, doc(cfg(feature = "iam")))]
pub mod iam;

#[cfg(feature = "lambda")]
#[cfg_attr(docsrs, doc(cfg(feature = "lambda")))]
pub mod lambda;

pub use four_derive::ManagedResource;
