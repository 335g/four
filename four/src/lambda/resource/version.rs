use crate::{
    core::LogicalId,
    function::{HaveAtt, RefInner, Referenced},
    lambda::{
        FunctionArn, LooseFunctionName, ProvisionedConcurrencyConfiguration, RuntimePolicy,
        VersionArn, VersionDescription, VersionNumber,
    },
    ManagedResource,
};

/// The [AWS::Lambda::Version]
///
/// The AWS::Lambda::Version resource creates a version from the current code
/// and configuration of a function. Use versions to create a snapshot of your function code
/// and configuration that doesn't change.
///
/// ```
/// use four::{
///     LogicalId, Account, Partition, arn_builder, service,
///     lambda::{
///         resource::{Function, Version},
///         Handler, Runtime,
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
/// let version_id = LogicalId::try_from("MyLayer");
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

impl HaveAtt<VersionNumber> for Version {
    const KEY: &'static str = "Version";
}
