use crate::{
    convert::{WillBe, WillMappable},
    core::LogicalId,
    lambda::{FunctionName, MaximumEventAgeInSeconds, MaximumRetryAttempts, Qualifier},
    ManagedResource,
};

/// [AWS::Lambda::EventInvokeConfig]
///
/// The `AWS::Lambda::EventInvokeConfig` resource configures options
/// for asynchronous invocation on a version or an alias.
///
/// By default, Lambda retries an asynchronous invocation twice if the
/// function returns an error. It retains events in a queue for up to six hours.
/// When an event fails all processing attempts or stays in the
/// asynchronous invocation queue for too long, Lambda discards it.
///
/// ```
/// use four::{
///     LogicalId, r#ref, get_att,
///     convert::WillBe,
///     lambda::{
///         resource::{Function, EventInvokeConfig, Version},
///         Runtime, FunctionVersion, Qualifier,
///     },
/// };
///
/// let function_id = "function".try_into().unwrap();
/// let role_id = "role".try_into().unwrap();
/// let handler = "bootstrap".try_into().unwrap();
/// let runtime = Runtime::ProvidedAl2023;
/// let (function, role) = Function::zip_with_role(function_id, role_id, "mybucket", "mykey.zip", handler, runtime);
///
/// let version_id = "version".try_into().unwrap();
/// let function_name = r#ref(function.clone());
/// let version = Version::new(version_id, function_name.clone().into());
///
/// let config_id = "asyncconfig".try_into().unwrap();
/// let version: WillBe<FunctionVersion> = get_att::<FunctionVersion, _>(&version);
/// let qualifier = version.map::<Qualifier>();
/// let config = EventInvokeConfig::new(config_id, function_name, qualifier)
///     .maximum_event_age_in_seconds(300.try_into().unwrap())
///     .maximum_retry_attempts(1.try_into().unwrap());
///
/// let lhs = serde_json::to_string(&config).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::EventInvokeConfig",
///         "Properties": {
///             "FunctionName": { "Ref": "function" },
///             "MaximumEventAgeInSeconds": 300,
///             "MaximumRetryAttempts": 1,
///             "Qualifier": { "Fn::GetAtt": ["version", "Version"] }
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
/// ```
///
/// [AWS::Lambda::EventInvokeConfig]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::EventInvokeConfig"]
pub struct EventInvokeConfig {
    logical_id: LogicalId,
    function_name: WillBe<FunctionName>,
    maximum_event_age_in_seconds: Option<MaximumEventAgeInSeconds>,
    maximum_retry_attempts: Option<MaximumRetryAttempts>,
    qualifier: WillBe<Qualifier>,
}
