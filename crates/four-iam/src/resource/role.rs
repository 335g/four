use four_core::{
    convert::WillBeString,
    logical_id::{LogicalId, LogicalIdentified},
    resource::ManagedResource,
};
use serde::ser::{Serialize, SerializeMap};

use crate::property::{
    action,
    effect::Effect,
    managed_policy::ManagedPolicy,
    policy::Policy,
    principal::Principal,
    statement::{ActionList, PrincipalList, Statement},
    version::Version,
};

pub struct Role {
    assume_role_policy_document: Policy,
    role_name: Option<WillBeString>,
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

    // pub fn lambda_execution() -> Self {
    //     let version = Version::latest();
    //     let effect = Effect::Allow;
    //     let principals = PrincipalList::Applicable(Principal::service("lambda"));
    //     let actions = ActionList::Applicable(vec![Box::new(action::sts::AssumeRole)]);
    //     let statement = Statement::new(effect, actions, Some(principals));
    //     let assume_role_policy_document = Policy::latest(vec![statement]);
    //     let managed_policy = ManagedPolicy::lambda_basic_execution_role();

    //     Role {
    //         assume_role_policy_document,
    //         managed_policies: vec![managed_policy],
    //         role_name: None,
    //     }
    // }

    pub fn name(mut self, name: WillBeString) -> Self {
        self.role_name = Some(name);
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

struct RoleInner<'a> {
    assume_role_policy_document: &'a Policy,
    role_name: &'a Option<WillBeString>,
    managed_policy_arns: &'a Option<Vec<ManagedPolicy>>,
}

impl<'a> RoleInner<'a> {
    fn new(
        assume_role_policy_document: &'a Policy,
        role_name: &'a Option<WillBeString>,
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
