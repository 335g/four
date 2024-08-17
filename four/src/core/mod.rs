mod account;
mod arn;
pub mod convert;
pub mod function;
pub mod logical_id;
pub mod parameter;
pub mod partition;
pub mod pseudo;
pub mod region;
pub mod resource;
pub mod service;
pub mod tag;
pub mod template;

pub use account::{Account, AccountDetail, AccountDetailError};
pub use arn::{
    arn_builder, AnyArn, Arn, PartialArn, RefNameAccount, RefNamePartition, RefNameRegion,
    RefNameRegionPartition,
};
