mod alias;
mod arn;
mod event_invoke_config;
mod function;
mod id;
mod layer_version;
mod layer_version_permission;
mod permission;
mod property;
pub mod resource;
mod url;
mod version;

pub use alias::{
    AliasDescription, AliasDescriptionError, AliasName, AliasNameError, AliasRoutingConfiguration,
    FunctionVersion, FunctionVersionError, VersionWeight,
};
pub use arn::{FunctionArn, LayerVersionArn, VersionArn};
pub use event_invoke_config::{
    MaximumEventAgeInSeconds, MaximumEventAgeInSecondsError, MaximumRetryAttempts,
    MaximumRetryAttemptsError,
};
pub use function::{
    FunctionName, FunctionNameError, MemorySize, MemorySizeError, Timeout, TimeoutError,
};
pub use id::{LayerVersionPermissionId, UrlId};
pub use layer_version::{
    CompatibleArchitectures, CompatibleArchitecturesError, CompatibleRuntimes,
    CompatibleRuntimesError, FunctionContent, LayerName, LayerNameError, LayerVersionDescription,
    LayerVersionDescriptionError, LicenseInfo, LicenseInfoError,
};
pub use layer_version_permission::{
    GetLayerVersionAction, LayerVersionPrincipal, OrganizationId, OrganizationIdError,
};
pub use permission::{
    EventSourceToken, EventSourceTokenError, PermissionPrincipal, PrincipalOrgID,
    PrincipalOrgIDError,
};
pub use property::{
    architecture::{Architecture, Architectures},
    code::Code,
    function_name::LooseFunctionName,
    handler::{Handler, HandlerError},
    runtime::Runtime,
};
use serde::Serialize;
pub use url::{
    AuthType, Cors, Header, HeaderError, InvokeMode, MaxAge, MaxAgeError, Method, Origin,
};
pub use version::{
    RuntimePolicy, RuntimeVersionArn, RuntimeVersionArnError, VersionDescription,
    VersionDescriptionError, VersionNumber,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProvisionedConcurrencyConfiguration {
    provisioned_concurrent_executions: usize,
}

impl ProvisionedConcurrencyConfiguration {
    pub fn new(executions: usize) -> Self {
        Self {
            provisioned_concurrent_executions: executions,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Qualifier {
    Version(u64),
    Alias(AliasName),
    Latest,
}

impl Serialize for Qualifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Qualifier::Version(x) => x.serialize(serializer),
            Qualifier::Alias(a) => a.serialize(serializer),
            Qualifier::Latest => "$LATEST".serialize(serializer),
        }
    }
}
