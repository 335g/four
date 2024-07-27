use four::{
    arn::Arn,
    convert::{WillBe, WillMappable},
    function::getatt::{Attribute, HaveAtt},
    logical_id::LogicalId,
    service::IAM,
    ManagedResource,
};
use serde::Serialize;

use crate::property::{
    action,
    assume_role_policy_document::AssumeRolePolicyDocument,
    managed_policy::ManagedPolicy,
    principal::{Principal, ServicePrincipal},
    statement::Statement,
};

#[derive(ManagedResource)]
#[resource_type = "AWS::IAM::Role"]
pub struct Role {
    logical_id: LogicalId,
    assume_role_policy_document: AssumeRolePolicyDocument,
    description: Option<String>,
    role_name: Option<WillBe<RoleName>>,
    managed_policy_arns: Option<Vec<ManagedPolicy>>,
}

impl Role {
    pub fn new(
        assume_role_policy_document: AssumeRolePolicyDocument,
        logical_id: LogicalId,
    ) -> Self {
        Self {
            logical_id,
            assume_role_policy_document,
            description: None,
            role_name: None,
            managed_policy_arns: None,
        }
    }

    pub fn lambda_execution(logical_id: LogicalId) -> Self {
        let statement = Statement::allow()
            .action(vec![Box::new(action::sts::AssumeRole)])
            .principal(Principal::from(ServicePrincipal::Lambda));
        let assume_role_policy_document = AssumeRolePolicyDocument::latest(vec![statement]);
        let role = Role::new(assume_role_policy_document, logical_id);

        role
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn name(mut self, name: WillBe<String>) -> Self {
        self.role_name = Some(name.map());
        self
    }

    pub fn managed_policy_arns(mut self, arns: Vec<ManagedPolicy>) -> Self {
        self.managed_policy_arns = Some(arns);
        self
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
    fn from(value: Arn<IAM>) -> Self {
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
    use crate::property::action;

    #[test]
    fn test_role1() {
        let role_id = LogicalId::try_from("role-id").unwrap();
        let statement = Statement::allow()
            .action(vec![Box::new(action::sts::AssumeRole)])
            .principal(Principal::from(ServicePrincipal::Lambda));
        let assume_role_policy_document = AssumeRolePolicyDocument::latest(vec![statement]);
        let role = Role::new(assume_role_policy_document, role_id);
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
