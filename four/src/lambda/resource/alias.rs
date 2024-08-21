use crate::{
    core::LogicalId,
    lambda::{
        AliasDescription, AliasName, AliasRoutingConfiguration, FunctionVersion, LooseFunctionName,
        ProvisionedConcurrencyConfiguration,
    },
};
use four_derive::ManagedResource;

/// [AWS::Lambda::Alias]
///
/// The AWS::Lambda::Alias resource creates an alias for a Lambda function version.
/// Use aliases to provide clients with a function identifier that you can update
/// to invoke a different version. You can also map an alias to split invocation requests
/// between two versions. Use the RoutingConfig parameter to specify a second version
/// and the percentage of invocation requests that it receives.
///
/// ```
/// use four::{
///     LogicalId,
///     lambda::resource::{Function, Alias},
/// };
///
///
///
/// ```
///
/// [AWS::Lambda::Alias]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Alias"]
pub struct Alias {
    logical_id: LogicalId,
    description: Option<AliasDescription>,
    function_name: LooseFunctionName,
    function_version: FunctionVersion,
    name: AliasName,
    provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    routing_config: Option<AliasRoutingConfiguration>,
}
