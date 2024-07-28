use four::{account::Account, arn::Arn, partition::Partition, service::IAM};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AWSManagedPolicy(Arn<IAM>);

impl AWSManagedPolicy {
    /// LambdaBasicExecutionRole
    /// cf. https://docs.aws.amazon.com/ja_jp/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html
    pub fn lambda_basic_execution_role() -> Self {
        let arn = Arn::builder(
            IAM,
            "policy/service-role/AWSLambdaBasicExecutionRole",
            Account::Aws,
        )
        .partition(Partition::Ref)
        .build();

        AWSManagedPolicy(arn)
    }
}
