use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        service::IAM,
        Arn, LogicalId,
    },
    iam::{
        action::Action,
        property::{
            action, policy_document::PolicyDocument, principal::Principal, statement::Statement,
        },
        AWSManagedPolicy, RoleArn, RoleId, RoleName,
    },
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Role"]
pub struct Role {
    logical_id: LogicalId,
    assume_role_policy_document: PolicyDocument,
    description: Option<String>,
    role_name: Option<WillBe<RoleName>>,
    managed_policy_arns: Option<Vec<AWSManagedPolicy>>,
}

impl Role {
    pub fn assume_role(id: LogicalId, principal: Principal) -> Self {
        let actions: Vec<Box<dyn Action>> = vec![Box::new(action::sts::AssumeRole)];
        let statement = Statement::allow_actions(actions)
            .principal(principal)
            .build()
            .expect("valid statement");
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

impl HaveAtt<RoleArn> for Role {
    const KEY: &'static str = "Arn";
}
impl HaveAtt<RoleId> for Role {
    const KEY: &'static str = "RoleId";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iam::property::{action, principal::ServicePrincipal};

    #[test]
    fn test_role1() {
        let role_id = LogicalId::try_from("role-id").unwrap();
        let actions: Vec<Box<dyn Action>> = vec![Box::new(action::sts::AssumeRole)];
        let statement = Statement::allow_actions(actions)
            .principal(Principal::from(ServicePrincipal::Lambda))
            .build()
            .expect("");
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
