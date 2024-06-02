use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::property::{Environment, EnvironmentKey, EnvironmentValue},
    error::Error,
    lambda::{
        error::LambdaError,
        property::{
            Architectures, CodeSigningConfigArn, DeadLetterConfig, DeadLetterConfigTargetArn,
            Description, EphemeralStorage, EphemeralStorageValue,
        },
    },
    template::{LogicalId, Resource},
};

#[derive(Debug, Clone)]
pub struct Function {
    logical_id: LogicalId,
    inner: InnerFunction,
}

impl Function {
    pub fn new(
        logical_id: &str,
        architectures: Option<Architectures>,
        code_signing_config_arn: Option<String>,
        dead_letter_config: Option<String>,
        description: Option<String>,
        environment: Option<HashMap<String, String>>,
        ephemeral_storage: Option<usize>,
    ) -> Result<Self, Error> {
        let inner = InnerFunction::new(
            architectures,
            code_signing_config_arn,
            dead_letter_config,
            description,
            environment,
            ephemeral_storage,
        )?;
        let logical_id = LogicalId::try_from(logical_id.to_string())?;

        Ok(Self { logical_id, inner })
    }
}

impl Resource for Function {
    fn resource_type() -> &'static str {
        "AWS::Lambda::Function"
    }

    fn logical_id(&self) -> &LogicalId {
        &self.logical_id
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InnerFunction {
    architectures: Vec<Architectures>,
    code_signing_config_arn: Option<CodeSigningConfigArn>,
    dead_letter_config: Option<DeadLetterConfig>,
    description: Option<Description>,
    environment: Option<Environment>,
    ephemeral_storage: Option<EphemeralStorage>,
}

impl InnerFunction {
    fn new(
        architectures: Option<Architectures>,
        code_signing_config_arn: Option<String>,
        dead_letter_config: Option<String>,
        description: Option<String>,
        environment: Option<HashMap<String, String>>,
        ephemeral_storage: Option<usize>,
    ) -> Result<Self, LambdaError> {
        let code_signing_config_arn = code_signing_config_arn
            .map(|s| CodeSigningConfigArn::new(s))
            .transpose()
            .map_err(|e| LambdaError::InvalidCodeSigningConfigArn(e))?;
        let dead_letter_config = dead_letter_config
            .map(|s| DeadLetterConfigTargetArn::new(s))
            .transpose()
            .map_err(|e| LambdaError::InvalidDeadLetterQueueTargetArn(e))?
            .map(|x| DeadLetterConfig::new(x));
        let description = description
            .map(|s| Description::new(s))
            .transpose()
            .map_err(|e| LambdaError::InvalidDescription(e))?;
        let environment = environment
            .map(|s| {
                s.into_iter()
                    .map(|(k, v)| {
                        let k = EnvironmentKey::new(k)
                            .map_err(|e| LambdaError::InvalidEnvironmentKey(e));
                        let v = EnvironmentValue::new(v);
                        k.and_then(|k| {
                            v.map(|v| (k, v))
                                .map_err(|e| LambdaError::InvalidEnvironmentValue(e))
                        })
                    })
                    .collect::<Result<Environment, _>>()
            })
            .transpose()?;
        let ephemeral_storage = ephemeral_storage
            .map(|x| EphemeralStorageValue::new(x))
            .transpose()
            .map_err(|e| LambdaError::InvalidEphemeralStorage(e))?
            .map(|x| EphemeralStorage::new(x));

        Ok(Self {
            architectures: vec![architectures.unwrap_or_default()],
            code_signing_config_arn,
            dead_letter_config,
            description,
            environment,
            ephemeral_storage,
        })
    }
}
