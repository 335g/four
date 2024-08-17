mod account;
mod arn;
pub mod convert;
pub mod function;
mod logical_id;
mod parameter;
mod partition;
pub mod pseudo;
mod region;
mod resource;
pub mod service;
mod tag;
pub mod template;

pub use account::{Account, AccountDetail, AccountDetailError};
pub use arn::{
    arn_builder, AnyArn, Arn, PartialArn, RefNameAccount, RefNamePartition, RefNameRegion,
    RefNameRegionPartition,
};
pub use function::{get_att, r#ref};
pub use logical_id::{LogicalId, LogicalIdError, LogicalIdentified};
pub use parameter::{
    NumberParameterBuilder, Parameter, ParameterError, ParameterType, StringParameterBuilder,
};
pub use partition::Partition;
pub use region::{Region, RegionDetail};
pub use resource::ManagedResource;
pub use tag::{Tag, TagError, TagKey, TagValue};
pub use template::Template;
