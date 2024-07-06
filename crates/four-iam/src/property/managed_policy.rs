use std::sync::LazyLock;

use four_core::{Account, Partition, ARN};
use serde::Serialize;

#[derive(Serialize)]
pub struct ManagedPolicy(ARN);

/// LambdaBasicExecutionRole
/// cf. https://docs.aws.amazon.com/ja_jp/aws-managed-policy/latest/reference/AWSLambdaBasicExecutionRole.html
pub static LAMBDA_BASIC_EXECUTION_ROLE: LazyLock<ManagedPolicy> = LazyLock::new(|| {
    let arn = ARN::builder("iam", "", Account::Aws)
        .partition(Partition::Aws)
        .build();
    ManagedPolicy(arn)
});
