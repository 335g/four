use nutype::nutype;
use serde::Serialize;

use crate::{arn_builder, service::IAM, Account, Arn, Partition};

#[nutype(validate(len_char_max = 1000), derive(Debug, Clone, Serialize))]
pub struct ManagedPolicyDescription(String);

#[derive(Debug, Clone, Serialize)]
pub struct AWSManagedPolicy(Arn<IAM>);

macro_rules! aws_managed_policy {
    ($(($name:ident, $resource:expr)),*) => {
        impl AWSManagedPolicy {
            $(pub fn $name() -> Self {
                let arn = arn_builder($resource, Account::Aws)
                    .partition(Partition::Ref)
                    .build(IAM);
                AWSManagedPolicy(arn)
            })*
        }
    };
}

aws_managed_policy!(
    (lambda_full_access, "policy/AWSLambda_FullAccess"),
    (lambda_read_only_access, "policy/AWSLambda_ReadOnlyAccess"),
    (
        lambda_basic_execution_role,
        "policy/service-role/AWSLambdaBasicExecutionRole"
    ),
    (
        lambda_dynamo_db_execution_role,
        "policy/service-role/AWSLambdaDynamoDBExecutionRole"
    ),
    (
        lambda_eni_management_access,
        "policy/service-role/AWSLambdaENIManagementAccess"
    ),
    (lambda_execute, "policy/AWSLambdaExecute"),
    (
        lambda_invocation_dynamo_db,
        "policy/AWSLambdaInvocation-DynamoDB"
    ),
    (
        lambda_kinesis_execution_role,
        "policy/service-role/AWSLambdaKinesisExecutionRole"
    ),
    (
        lambda_msk_execution_role,
        "policy/service-role/AWSLambdaMSKExecutionRole"
    ),
    (
        lambda_replicator,
        "policy/aws-service-role/AWSLambdaReplicator"
    ),
    (lambda_role, "policy/service-role/AWSLambdaRole"),
    (
        lambda_sqs_queue_execution_role,
        "policy/service-role/AWSLambdaSQSQueueExecutionRole"
    ),
    (
        lambda_vpc_access_execution_role,
        "policy/service-role/AWSLambdaVPCAccessExecutionRole"
    )
);
