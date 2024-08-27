use crate::{
    core::{AccountDetail, AnyArn, LogicalId},
    iam::action::lambda::LambdaAction,
    lambda::{AuthType, EventSourceToken, LooseFunctionName, PermissionPrincipal, PrincipalOrgID},
    ManagedResource,
};

/// [AWS::Lambda::Permission]
///
/// The AWS::Lambda::Permission resource grants an AWS service or
/// another account permission to use a function. You can apply
/// the policy at the function level, or specify a qualifier to restrict
/// access to a single version or alias. If you use a qualifier,
/// the invoker must use the full Amazon Resource Name (ARN) of that version or
/// alias to invoke the function.
///
/// To grant permission to another account,
/// specify the account ID as the Principal. To grant permission to
/// an organization defined in AWS Organizations, specify the organization ID
/// as the PrincipalOrgID. For AWS services, the principal is a domain-style
/// identifier defined by the service, like s3.amazonaws.com or sns.amazonaws.com.
/// For AWS services, you can also specify the ARN of the associated
/// resource as the SourceArn. If you grant permission to a service principal
/// without specifying the source, other accounts could potentially configure
/// resources in their account to invoke your Lambda function.
///
/// If your function has a function URL, you can specify the FunctionUrlAuthType
/// parameter. This adds a condition to your permission that only applies when
/// your function URL's AuthType matches the specified FunctionUrlAuthType.
/// For more information about the AuthType parameter, see Security
/// and auth model for Lambda function URLs.
///
/// This resource adds a statement to a resource-based permission
/// policy for the function. For more information about function policies,
/// see Lambda Function Policies.
///
/// ex. Cross Account Invoke
///
/// Grant account 123456789012 permission to invoke a function
/// resource named `lambdaFunction` created in the same template.
///
/// ```
/// use four::{
///     LogicalId, r#ref, Account,
///     lambda::{
///         resource::{Permission, Function},
///         Handler, Runtime, PermissionPrincipal,
///     },
///     iam::{
///         action::lambda::InvokeFunction,
///     },
/// };
///
/// let function_id = LogicalId::try_from("lambdaFunction").unwrap();
/// let role_id = LogicalId::try_from("role").unwrap();
/// let handler = Handler::try_from("bootstrap").unwrap();
/// let runtime = Runtime::ProvidedAl2023;
/// let (function, role) = Function::zip_with_role(function_id, role_id, "mybucket", "mykey.zip", handler, runtime);
///
/// let permission_id = LogicalId::try_from("permission").unwrap();
/// let action = Box::new(InvokeFunction);
/// let function_name = r#ref(function.clone()).into();
/// let permission = Permission::new(permission_id, action, function_name)
///     .principal(PermissionPrincipal::Account("123456789012".try_into().unwrap()));
///
/// let lhs = serde_json::to_string(&permission).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::Permission",
///         "Properties": {
///             "Action": "lambda:InvokeFunction",
///             "FunctionName": {"Ref": "lambdaFunction"},
///             "Principal": "123456789012"
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
/// ```
///
/// [AWS::Lambda::Permission]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Permission"]
pub struct Permission {
    logical_id: LogicalId,
    action: Box<dyn LambdaAction>,
    event_source_token: Option<EventSourceToken>,
    function_name: LooseFunctionName,
    function_url_auth_type: Option<AuthType>,
    principal: Option<PermissionPrincipal>,

    // TODO: rename (four_derive)
    principal_org_i_d: Option<PrincipalOrgID>,
    source_account: Option<AccountDetail>,
    source_arn: Option<AnyArn>,
}
