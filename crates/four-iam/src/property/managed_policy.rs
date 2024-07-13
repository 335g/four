use four_core::{Account, Partition, ARN};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ManagedPolicy(ARN);

impl ManagedPolicy {
    /// LambdaBasicExecutionRole
    /// cf. https://docs.aws.amazon.com/ja_jp/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html
    pub fn lambda_basic_execution_role() -> Self {
        let arn = ARN::builder(
            "iam",
            "policy/service-role/AWSLambdaBasicExecutionRole",
            Account::Aws,
        )
        .partition(Partition::Ref)
        .build();
        ManagedPolicy(arn)
    }
}
