use dyn_clone::DynClone;

pub trait Action: erased_serde::Serialize + DynClone + std::fmt::Debug {}

erased_serde::serialize_trait_object!(Action);
dyn_clone::clone_trait_object!(Action);

macro_rules! impl_action {
    ( $service:ident, $action:ident, [$($t:ident),*] ) => {
        pub mod $service {
            use serde::Serialize;

            pub trait $action: super::Action {}
            erased_serde::serialize_trait_object!($action);
            dyn_clone::clone_trait_object!($action);

            $(
            #[derive(Debug, Clone, Copy)]
            pub struct $t;

            impl super::Action for $t {}
            impl $action for $t {}

            impl Serialize for $t {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let s = format!("{}:{}", stringify!($service), stringify!($t));
                    serializer.serialize_str(&s)
                }
            })*
        }
    };
}

impl_action!(sts, StsAction, [AssumeRole]);
impl_action!(
    lambda,
    LambdaAction,
    [
        AddLayerVersionPermission,
        AddPermission,
        CreateAlias,
        CreateCodeSigningConfig,
        CreateEventSourceMapping,
        CreateFunction,
        CreateFunctionUrlConfig,
        DeleteAlias,
        DeleteCodeSigningConfig,
        DeleteEventSourceMapping,
        DeleteFunction,
        DeleteFunctionCodeSigningConfig,
        DeleteFunctionConcurrency,
        DeleteFunctionEventInvokeConfig,
        DeleteFunctionUrlConfig,
        DeleteLayerVersion,
        DeleteProvisionedConcurrencyConfig,
        DisableReplication,
        EnableReplication,
        GetAccountSettings,
        GetAlias,
        GetCodeSigningConfig,
        GetEventSourceMapping,
        GetFunction,
        GetFunctionCodeSigningConfig,
        GetFunctionConcurrency,
        GetFunctionConfiguration,
        GetFunctionEventInvokeConfig,
        GetFunctionUrlConfig,
        GetLayerVersion,
        GetLayerVersionPolicy,
        GetPolicy,
        GetProvisionedConcurrencyConfig,
        GetRuntimeManagementConfig,
        InvokeAsync,
        InvokeFunction,
        InvokeFunctionUrl,
        ListAliases,
        ListCodeSigningConfigs,
        ListEventSourceMappings,
        ListFuctionEventInvokeConfigs,
        ListFunctionUrlConfigs,
        ListFunctions,
        ListFunctionsByCodeSigningConfig,
        ListLayerVersions,
        ListLayers,
        ListProvisionedConcurrencyConfigs,
        ListTags,
        ListVersionsByFunction,
        PublishLayerVersion,
        PutFunctionCodeSigningConfig,
        PutFunctionConcurrency,
        PutFunctionEventInvokeConfig,
        PutProvisionedConcurrencyConfig,
        PutRuntimeManagementConfig,
        RemoveLayerVersionPermission,
        RemovePermission,
        TagResource,
        UntagResource,
        UpdateAlias,
        UpdateCodeSigningConfig,
        UpdateEventSourceMapping,
        UpdateFunctionCode,
        UpdateFunctionCodeSigningConfig,
        UpdateFunctionConfiguration,
        UpdateFunctionEventInvokeConfig,
        UpdateFunctionUrlConfig
    ]
);
