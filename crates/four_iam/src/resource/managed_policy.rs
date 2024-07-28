use four::{
    account::Account, arn::Arn, logical_id::LogicalId, partition::Partition, service::IAM,
    ManagedResource,
};
use serde::Serialize;

use crate::property::policy_document::PolicyDocument;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::ManagedPolicy"]
pub struct CustomerManagedPolicy {
    logical_id: LogicalId,
    policy_document: PolicyDocument,
}

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
