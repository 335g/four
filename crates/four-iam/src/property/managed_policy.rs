use std::sync::LazyLock;

use four_core::{Account, Partition, ARN};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ManagedPolicy(ARN);

/// LambdaBasicExecutionRole
/// cf. https://docs.aws.amazon.com/ja_jp/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html
pub static LAMBDA_BASIC_EXECUTION_ROLE: LazyLock<ManagedPolicy> = LazyLock::new(|| {
    let arn = ARN::builder(
        "iam",
        "policy/service-role/AWSLambdaBasicExecutionRole",
        Account::Aws,
    )
    .partition(Partition::Ref)
    .build();
    ManagedPolicy(arn)
});
