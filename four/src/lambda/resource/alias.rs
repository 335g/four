use crate::{
    convert::WillBe,
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
///     LogicalId, r#ref, get_att,
///     lambda::{
///         AliasName, AliasRoutingConfiguration, VersionWeight, FunctionVersion,
///         Handler, Runtime, LooseFunctionName, version_weight,
///         resource::{Function, Alias, Version},
///     }
/// };
///
/// let function_id = LogicalId::try_from("function").unwrap();
/// let role_id = LogicalId::try_from("role").unwrap();
/// let handler = Handler::try_from("bootstrap").unwrap();
/// let runtime = Runtime::ProvidedAl2023;
/// let (function, role) = Function::zip_with_role(function_id, role_id, "mybucket", "mykey.zip", handler, runtime);
///
/// let function_name: LooseFunctionName = r#ref(function.clone()).into();
/// let v1_id = LogicalId::try_from("v1").unwrap();
/// let v1 = Version::new(v1_id, function_name.clone());
/// let v1_version = get_att::<FunctionVersion, _>(&v1);
///
/// let v2_id = LogicalId::try_from("v2").unwrap();
/// let v2 = Version::new(v2_id, function_name.clone());
/// let v2_version = get_att::<FunctionVersion, _>(&v2);
///
/// let alias_id = LogicalId::try_from("alias").unwrap();
/// let alias_name = AliasName::try_new("BLUE").unwrap();
/// let routing_config = version_weight(v2_version, 0.5);
/// let alias = Alias::new(alias_id, function_name, v1_version, alias_name)
///     .routing_config(routing_config);
///
/// let lhs = serde_json::to_string(&alias).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::Alias",
///         "Properties": {
///             "FunctionName": {"Ref": "function"},
///             "FunctionVersion": {"Fn::GetAtt": ["v1", "Version"]},
///             "Name": "BLUE",
///             "RoutingConfig": {
///                 "AdditionalVersionWeights": [
///                     {
///                         "FunctionVersion": {"Fn::GetAtt": ["v2", "Version"]},
///                         "FunctionWeight": 0.5
///                     }
///                 ]
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
/// [AWS::Lambda::Alias]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Alias"]
pub struct Alias {
    logical_id: LogicalId,
    description: Option<AliasDescription>,
    function_name: LooseFunctionName,
    function_version: WillBe<FunctionVersion>,
    name: AliasName,
    provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    routing_config: Option<AliasRoutingConfiguration>,
}
