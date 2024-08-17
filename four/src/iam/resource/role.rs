use crate::{
    core::{
        convert::{WillBe, WillMappable},
        function::{Attribute, HaveAtt, RefInner, Referenced},
        service::IAM,
        Arn, LogicalId,
    },
    iam::property::{
        action, policy_document::PolicyDocument, principal::Principal, statement::Statement,
    },
};
use four_derive::ManagedResource;
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Role"]
pub struct Role {
    logical_id: LogicalId,
    assume_role_policy_document: PolicyDocument,
    description: Option<String>,
    role_name: Option<WillBe<RoleName>>,
    managed_policy_arns: Option<Vec<WillBe<Arn<IAM>>>>,
}

impl Role {
    pub fn assume_role(id: LogicalId, principal: Principal) -> Self {
        let statement = Statement::allow()
            .action(vec![Box::new(action::sts::AssumeRole)])
            .principal(principal);
        let policy_document = PolicyDocument::latest(vec![statement]);

        Self::new(id, policy_document)
    }
}

impl Referenced for Role {
    type To = WillBe<RoleName>;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl WillMappable<String> for RoleName {}

#[derive(Debug, Clone, Serialize)]
pub struct RoleArn(Arn<IAM>);

impl From<Arn<IAM>> for RoleArn {
    fn from(value: Arn<IAM>) -> RoleArn {
        RoleArn(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleId(String);

impl HaveAtt<RoleArn> for Role {}
impl HaveAtt<RoleId> for Role {}

impl Attribute for RoleArn {
    fn name() -> &'static str {
        "Arn"
    }
}

impl Attribute for RoleId {
    fn name() -> &'static str {
        "RoleId"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iam::property::{action, principal::ServicePrincipal};

    #[test]
    fn test_role1() {
        let role_id = LogicalId::try_from("role-id").unwrap();
        let statement = Statement::allow()
            .action(vec![Box::new(action::sts::AssumeRole)])
            .principal(Principal::from(ServicePrincipal::Lambda));
        let assume_role_policy_document = PolicyDocument::latest(vec![statement]);
        let role = Role::new(role_id, assume_role_policy_document);
        let mut rhs = r#"{
            "Type": "AWS::IAM::Role",
            "Properties": {
                "AssumeRolePolicyDocument": {
                    "Version": "2012-10-17",
                    "Statement": [
                        {
                            "Effect": "Allow",
                            "Action": [ "sts:AssumeRole" ],
                            "Principal": {
                                "Service": [ "lambda.amazonaws.com" ]
                            }
                        }
                    ]
                }
            }}"#
        .to_string();
        rhs.retain(|c| c != ' ' && c != '\n');
        assert_eq!(serde_json::to_string(&role).unwrap(), rhs);
    }
}
