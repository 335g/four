pub trait Service {
    fn to_string(&self) -> String;
}

macro_rules! services {
    ($(($service_name:ident, $stringify_service:expr)),*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $service_name;

            impl serde::Serialize for $service_name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&self.to_string())
                }
            }

            impl Service for $service_name {
                fn to_string(&self) -> String {
                    format!("{}", stringify!($stringify_service))
                }
            }
        )*
    };
}

services!(
    (Amplify, "amplify"),
    (APIGateway, "apigateway"),
    (AppFlow, "appflow"),
    (AppMesh, "appmesh"),
    (AppRunner, "apprunner"),
    (AppSync, "appsync"),
    (Athena, "athena"),
    (Batch, "batch"),
    (Cloud9, "cloud9"),
    (CloudFormation, "cloudformation"),
    (CloudFront, "cloudfront"),
    (CloudShell, "cloudshell"),
    (CloudWatch, "cloudwatch"),
    (CloudWatchLogs, "logs"),
    (CodeArtifact, "codeartifact"),
    (CodeBuild, "codebuild"),
    (CodeCommit, "codecommit"),
    (CodeDeploy, "codedeploy"),
    (CodeGuru, "codeguru"),
    (CodePipeline, "codepipeline"),
    (Comprehend, "comprehend"),
    (ControlTower, "controltower"),
    (DataExchange, "dataexchange"),
    (DataPipeline, "datapipeline"),
    (DynamoDB, "dynamodb"),
    (EC2, "ec2"),
    (ECR, "ecr"),
    (ECS, "ecs"),
    (ElastiCache, "elasticache"),
    (ElasticFilesystem, "elasticfilesystem"),
    (EKS, "eks"),
    (ElasticBeanstalk, "elasticbeanstalk"),
    (EMR, "elasticmapreduce"),
    (EventBridge, "events"),
    (Glue, "glue"),
    (GlueDataBrew, "databrew"),
    (Gracier, "gracier"),
    (GuardDuty, "guardduty"),
    (IAM, "iam"),
    (IoTCore, "iot"),
    (Kendra, "kendra"),
    (Kinesis, "kinesis"),
    (LakeFormation, "lakeformation"),
    (Lambda, "lambda"),
    (Lightsail, "lightsail"),
    (Macie, "macie"),
    (Macie2, "macie2"),
    (Neptune, "neptune-db"),
    (Organizations, "organizations"),
    (QuickSight, "quicksight"),
    (RDS, "rds"),
    (RDSData, "rds-data"),
    (RDSDB, "rds-db"),
    (Redshift, "redshift"),
    (Route53, "route53"),
    (S3, "s3"),
    (SageMaker, "sagemaker"),
    (SecretsManager, "secretsmanager"),
    (SimpleEmail, "sms"),
    (SimpleQueue, "sqs"),
    (SimpleNotification, "sns"),
    (SingleSignOn, "sso"),
    (StorageGateway, "storagegateway"),
    (StepFunctions, "states"),
    (Sts, "sts"),
    (SystemsManager, "ssm"),
    (Transcribe, "transcribe"),
    (Translate, "translate"),
    (WAF, "waf"),
    (XRay, "xray")
);
