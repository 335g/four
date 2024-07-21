pub use four_core::{
    account::{Account, AccountDetail, AccountDetailError},
    arn::{Arn, RefNameAccount, RefNamePartition, RefNameRegion, RefNameRegionPartition},
    convert::{WillBe, WillFrom, WillMappable},
    fn_join,
    function::{
        join::Join,
        reference::{Ref, Referenced},
    },
    logical_id::{LogicalId, LogicalIdentified},
    parameter::{NumberParameterBuilder, Parameter, ParameterError, StringParameterBuilder},
    pseudo,
    region::{Region, RegionDetail},
    resource::ManagedResource,
    template::{Template, TemplateVersion},
};
pub use four_macros::ManagedResource;
