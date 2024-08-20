use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    iam::RoleArn,
    lambda::{
        property::{architecture::Architectures, code::Code, handler::Handler, runtime::Runtime},
        FunctionArn, FunctionName, MemorySize, MemorySizeError, Timeout, TimeoutError,
    },
    ManagedResource,
};

/// [The AWS::Lambda::Function](https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html)
///
/// The AWS::Lambda::Function resource creates a Lambda function.
/// To create a function, you need a deployment package and an execution role.
/// The deployment package is a .zip file archive or container image
/// that contains your function code. The execution role grants the function permission
/// to use AWS services, such as Amazon CloudWatch Logs for log streaming and AWS X-Ray
/// for request tracing. You set the package type to Image if the deployment
/// package is a container image. For a container image, the code property must include
/// the URI of a container image in the Amazon ECR registry. You do not need to specify
/// the handler and runtime properties. You set the package type to Zip if the deployment
/// package is a .zip file archive. For a .zip file archive, the code property specifies
/// the location of the .zip file. You must also specify the handler and runtime properties.
/// For a Python example, see Deploy Python Lambda functions with .zip file archives.
/// You can use code signing if your deployment package is a .zip file archive.
/// To enable code signing for this function, specify the ARN of a code-signing configuration.
/// When a user attempts to deploy a code package with UpdateFunctionCode,
/// Lambda checks that the code package has a valid signature from a trusted publisher.
/// The code-signing configuration includes a set of signing profiles, which define the
/// trusted publishers for this function. Note that you configure provisioned concurrency
/// on a AWS::Lambda::Version or a AWS::Lambda::Alias. For a complete introduction to
/// Lambda functions, see What is Lambda? in the Lambda developer guide.
///
/// ---
///
/// First, we will only deal with cases where the zip file generated by [cargo-lambda]
/// is stored in an S3 bucket.
///
/// ex1. Simple Function
/// ```
/// use four::{
///     LogicalId, Arn, arn_builder, service, Account, Partition,
///     lambda::{resource::Function, Handler, Runtime},
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
/// let lhs = serde_json::to_string(&function).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::Lambda::Function",
///         "Properties": {
///             "Code": {
///                 "S3Bucket": "mybucket",
///                 "S3Key": "mykey.zip"
///             },
///             "Handler": "bootstrap",
///             "Role": "arn:aws:iam::123456789012:role/lambda-role",
///             "Runtime": "provided.al2023"
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
/// ```
///
/// [cargo-lambda]: https://github.com/cargo-lambda/cargo-lambda
#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Function"]
pub struct Function {
    logical_id: LogicalId,
    architectures: Option<Architectures>,
    code: Code,
    description: Option<String>,
    function_name: Option<WillBe<FunctionName>>,
    handler: Option<Handler>,
    memory_size: Option<MemorySize>,
    role: WillBe<RoleArn>,
    runtime: Option<Runtime>,
    timeout: Option<Timeout>,
}

impl Function {
    pub fn zip(
        logical_id: LogicalId,
        bucket_name: &str,
        key: &str,
        role: WillBe<RoleArn>,
        handler: Handler,
        runtime: Runtime,
    ) -> Function {
        let code = Code::new(bucket_name, key);

        Function::new(logical_id, code, role)
            .handler(handler)
            .runtime(runtime)
    }

    pub fn memory_size_value(mut self, value: usize) -> Result<Self, MemorySizeError> {
        let value = MemorySize::try_new(value)?;
        self.memory_size = Some(value);
        Ok(self)
    }

    pub fn timeout_value(mut self, timeout: usize) -> Result<Self, TimeoutError> {
        let timeout = Timeout::try_new(timeout)?;
        self.timeout = Some(timeout);
        Ok(self)
    }
}

impl Referenced for Function {
    type To = FunctionName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<FunctionArn> for Function {
    const KEY: &'static str = "Arn";
}
