use four::{
    account::Account,
    arn::{Arn, Partition},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ManagedPolicy(Arn);

impl ManagedPolicy {
    /// LambdaBasicExecutionRole
    /// cf. https://docs.aws.amazon.com/ja_jp/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html
    pub fn lambda_basic_execution_role() -> Self {
        let arn = Arn::builder(
            "iam",
            "policy/service-role/AWSLambdaBasicExecutionRole",
            Account::Aws,
        )
        .partition(Partition::Ref)
        .build();
        ManagedPolicy(arn)
    }
}
