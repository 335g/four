use crate::{
    core::{
        account::Account,
        arn::Arn,
        convert::WillBe,
        function::reference::{RefInner, Referenced},
        logical_id::LogicalId,
        partition::Partition,
        service::IAM,
    },
    iam::{property::policy_document::PolicyDocument, util::Path},
};
use four_derive::ManagedResource;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::ManagedPolicy"]
pub struct ManagedPolicy {
    logical_id: LogicalId,
    description: Option<Description>,
    managed_policy_name: Option<WillBe<String>>,
    path: Option<Path>,
    policy_document: PolicyDocument,
}

#[derive(Debug, Clone, Serialize)]
pub struct Description(String);

impl TryFrom<&str> for Description {
    type Error = DescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        if len > 1000 {
            Err(DescriptionError::Over(len))
        } else {
            Ok(Description(value.to_string()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DescriptionError {
    #[error("character count exceeds the specified value(1000): {0}")]
    Over(usize),
}

#[derive(Debug, Clone, Serialize)]
pub struct ManagedPolicyArn(Arn<IAM>);

impl Referenced for ManagedPolicy {
    type To = ManagedPolicyArn;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AWSManagedPolicy(Arn<IAM>);

macro_rules! aws_managed_policy {
    ($(($name:ident, $resource:expr)),*) => {
        impl AWSManagedPolicy {
            $(pub fn $name() -> Self {
                let arn = Arn::builder(IAM, $resource, Account::Aws)
                    .partition(Partition::Ref)
                    .build();
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
