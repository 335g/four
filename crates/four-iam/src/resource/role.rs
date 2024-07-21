use four::{
    arn::Arn,
    convert::{WillBe, WillMappable},
    function::getatt::{Attribute, HaveAtt},
    logical_id::LogicalId,
    ManagedResource,
};
use serde::Serialize;

use crate::property::{
    action::{self, Action},
    effect::Effect,
    managed_policy::ManagedPolicy,
    policy::Policy,
    principal::Principal,
    statement::{ActionList, PrincipalList, Statement},
};

#[derive(ManagedResource)]
#[resource_type = "AWS::IAM::Role"]
pub struct Role {
    logical_id: LogicalId,
    assume_role_policy_document: Policy,
    role_name: std::option::Option<WillBe<RoleName>>,
    managed_policy_arns: Option<Vec<ManagedPolicy>>,
}

impl Role {
    pub fn new(assume_role_policy_document: Policy, logical_id: LogicalId) -> Self {
        Self {
            logical_id,
            assume_role_policy_document,
            role_name: None,
            managed_policy_arns: None,
        }
    }

    pub fn lambda_execution(id: LogicalId) -> Self {
        let actions: Vec<Box<dyn Action>> = vec![Box::new(action::sts::AssumeRole)];
        let statement = Statement::builder()
            .effect(Effect::Allow)
            .principal(PrincipalList::Applicable(Principal::service("lambda")))
            .action(ActionList::Applicable(actions))
            .build();
        let statements = vec![statement];
        let assume_role_policy_document = Policy::latest(statements);
        let role = Role::new(assume_role_policy_document, id)
            .managed_policy_arns(vec![ManagedPolicy::lambda_basic_execution_role()]);

        role
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

#[derive(Debug, Serialize)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn will(self) -> WillBe<RoleName> {
        WillBe::new(Box::new(self))
    }
}

impl WillMappable<String> for RoleName {}

#[derive(Debug, Serialize)]
pub struct RoleArn(Arn);

#[derive(Debug, Serialize)]
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
