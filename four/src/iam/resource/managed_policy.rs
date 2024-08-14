use std::sync::LazyLock;

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
use nutype::nutype;
use regex::Regex;
use serde::Serialize;

use super::{role::RoleName, user::UserName};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::ManagedPolicy"]
pub struct ManagedPolicy {
    logical_id: LogicalId,
    description: Option<Description>,
    groups: Option<Groups>,
    managed_policy_name: Option<WillBe<String>>,
    path: Option<Path>,
    policy_document: PolicyDocument,
    roles: Option<Vec<WillBe<RoleName>>>,
    users: Option<Vec<WillBe<UserName>>>,
}

#[nutype(validate(len_char_max = 1000), derive(Debug, Clone, Serialize))]
pub struct Description(String);

static GROUP_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[\w+=,.@-]+"#).unwrap());

fn valid_groups(groups: &Vec<String>) -> bool {
    groups
        .into_iter()
        .map(|g| !g.is_empty() && g.len() < 128 && GROUP_REGEX.is_match(g))
        .fold(false, |acc, x| acc || x)
}

#[nutype(
    validate(predicate = valid_groups),
    derive(Debug, Clone, Serialize)
)]
pub struct Groups(Vec<String>);

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
