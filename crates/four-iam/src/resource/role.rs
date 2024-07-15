use four_core::{
    convert::WillMappable,
    function::getatt::{Attribute, HaveAtt},
    logical_id::{LogicalId, LogicalIdentified},
    resource::ManagedResource,
    resource_name::Arn,
    WillBe,
};
use serde::{ser::SerializeMap as _, Serialize};

use crate::property::{
    action::{self, Action},
    effect::Effect,
    managed_policy::ManagedPolicy,
    policy::Policy,
    principal::Principal,
    statement::{ActionList, PrincipalList, Statement},
};

pub struct Role {
    assume_role_policy_document: Policy,
    role_name: Option<WillBe<RoleName>>,
    managed_policy_arns: Option<Vec<ManagedPolicy>>,
    logical_id: LogicalId,
}

impl Role {
    pub fn new(assume_role_policy_document: Policy, logical_id: LogicalId) -> Self {
        Self {
            assume_role_policy_document,
            role_name: None,
            managed_policy_arns: None,
            logical_id,
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

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let logical_id = self.logical_id.clone();
        let inner = RoleInner::new(
            &self.assume_role_policy_document,
            &self.role_name,
            &self.managed_policy_arns,
        );

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.logical_id, &inner)?;
        map.end()
    }
}

impl LogicalIdentified for Role {
    fn logical_id(&self) -> &LogicalId {
        &self.logical_id
    }
}

impl ManagedResource for Role {
    fn resource_type(&self) -> &'static str {
        "AWS::IAM::Role"
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

struct RoleInner<'a> {
    assume_role_policy_document: &'a Policy,
    role_name: &'a Option<WillBe<RoleName>>,
    managed_policy_arns: &'a Option<Vec<ManagedPolicy>>,
}

impl<'a> RoleInner<'a> {
    fn new(
        assume_role_policy_document: &'a Policy,
        role_name: &'a Option<WillBe<RoleName>>,
        managed_policy_arns: &'a Option<Vec<ManagedPolicy>>,
    ) -> RoleInner<'a> {
        Self {
            assume_role_policy_document,
            role_name,
            managed_policy_arns,
        }
    }
}

impl Serialize for RoleInner<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry(
            "AssumeRolePolicyDocument",
            &self.assume_role_policy_document,
        )?;

        if let Some(role_name) = self.role_name {
            map.serialize_entry("RoleName", role_name)?;
        }
        if let Some(managed_policy_arns) = self.managed_policy_arns {
            map.serialize_entry("ManagedPolicyArns", managed_policy_arns)?;
        }

        map.end()
    }
}
