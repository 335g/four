use std::ops::Deref as _;

use four_core::ManagedResource;
use serde::ser::{Serialize, SerializeMap};

use crate::property::{
    action,
    effect::Effect,
    managed_policy::{ManagedPolicy, LAMBDA_BASIC_EXECUTION_ROLE},
    policy::Policy,
    principal::Principal,
    statement::{ActionList, PrincipalList, Statement},
    version::Version,
};

pub struct Role {
    assume_role_policy_document: Policy,
    managed_policies: Vec<ManagedPolicy>,
}

impl Role {
    pub fn new(assume_role_policy_document: Policy, managed_policies: Vec<ManagedPolicy>) -> Self {
        Role {
            assume_role_policy_document,
            managed_policies,
        }
    }
    pub fn lambda_execution() -> Self {
        let version = Version::latest();
        let effect = Effect::Allow;
        let principals = PrincipalList::Applicable(Principal::service("lambda"));
        let actions = ActionList::Applicable(vec![Box::new(action::sts::AssumeRole)]);
        let statement = Statement::new(effect, actions, Some(principals));
        let assume_role_policy_document = Policy::latest(vec![statement]);
        let managed_policy = LAMBDA_BASIC_EXECUTION_ROLE.deref().clone();

        Role {
            assume_role_policy_document,
            managed_policies: vec![managed_policy],
        }
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let count = if self.managed_policies.is_empty() {
            Some(1)
        } else {
            Some(2)
        };

        let mut map = serializer.serialize_map(count)?;
        map.serialize_entry(
            "AssumeRolePolicyDocument",
            &self.assume_role_policy_document,
        )?;

        if !self.managed_policies.is_empty() {
            map.serialize_entry("ManagedPolicyArns", &self.managed_policies)?;
        }

        map.end()
    }
}

impl ManagedResource for Role {
    fn resource_type(&self) -> &'static str {
        "AWS::IAM::Role"
    }
}
