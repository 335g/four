use crate::{
    core::LogicalId,
    function::{HaveAtt, RefInner, Referenced},
    lambda::{
        FunctionArn, FunctionVersion, LooseFunctionName, ProvisionedConcurrencyConfiguration,
        RuntimePolicy, VersionArn, VersionDescription,
    },
    ManagedResource,
};

/// [AWS::Lambda::Version]
///
/// The AWS::Lambda::Version resource creates a version from the current code
/// and configuration of a function. Use versions to create a snapshot of your function code
/// and configuration that doesn't change.
///
/// ```
/// use four::{
///     LogicalId, Account, Partition, arn_builder, service, r#ref,
///     lambda::{
///         resource::{Function, Version},
///         Handler, Runtime, VersionDescription, ProvisionedConcurrencyConfiguration,
///     },
///     iam::RoleArn,
/// };
///
/// let account = Account::try_from("123456789012").unwrap();
/// let role_arn: RoleArn = arn_builder("role/lambda-role", account)
///     .partition(Partition::Aws)
///     .build(service::IAM)
///     .into();
/// let function_id = LogicalId::try_from("function").unwrap();
/// let handler = Handler::try_from("bootstrap").unwrap();
/// let runtime = Runtime::ProvidedAl2023;
/// let function = Function::zip(function_id, "mybucket", "mykey.zip", role_arn.into(), handler, runtime);
///
/// let version_id = LogicalId::try_from("version").unwrap();
/// let function_name = r#ref(function).into();
/// let version = Version::new(version_id, function_name)
///     .description(VersionDescription::try_new("v1").unwrap())
///     .provisioned_concurrency_config(ProvisionedConcurrencyConfiguration::new(20));
///
/// let lhs = serde_json::to_string(&version).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::Version",
///         "Properties": {
///             "Description": "v1",
///             "FunctionName": { "Ref": "function" },
///             "ProvisionedConcurrencyConfig": {
///                 "ProvisionedConcurrentExecutions": 20
///             }
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
///
/// ```
///
/// [AWS::Lambda::Version]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html
///
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Version"]
pub struct Version {
    logical_id: LogicalId,
    code_sha256: Option<String>,
    description: Option<VersionDescription>,
    function_name: LooseFunctionName,
    provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    runtime_policy: Option<RuntimePolicy>,
}

impl Referenced for Version {
    type To = VersionArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<FunctionArn> for Version {
    const KEY: &'static str = "FunctionArn";
}

impl HaveAtt<FunctionVersion> for Version {
    const KEY: &'static str = "Version";
}
