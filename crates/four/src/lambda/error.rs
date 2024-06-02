use crate::{
    common::property::{EnvironmentKeyError, EnvironmentValueError},
    lambda::property::{
        CodeSigningConfigArnError, DeadLetterConfigTargetArnError, DescriptionError,
    },
};

use super::property::EphemeralStorageValueError;

#[derive(Debug, thiserror::Error)]
pub enum LambdaError {
    #[error("invalid code signing config arn: {0:?}")]
    InvalidCodeSigningConfigArn(#[from] CodeSigningConfigArnError),

    #[error("invalid dead leatter queue target arn: {0:?}")]
    InvalidDeadLetterQueueTargetArn(#[from] DeadLetterConfigTargetArnError),

    #[error("invalid description: {0:?}")]
    InvalidDescription(#[from] DescriptionError),

    #[error("invalid environemnt key: {0:?}")]
    InvalidEnvironmentKey(#[from] EnvironmentKeyError),

    #[error("invalid environemnt value: {0:?}")]
    InvalidEnvironmentValue(#[from] EnvironmentValueError),

    #[error("invalid ephemeral storage: {0:?}")]
    InvalidEphemeralStorage(#[from] EphemeralStorageValueError),
}
