use four_core::{LogicalId, ARN};
use serde::{ser::SerializeMap, Serialize};

use crate::property::{managed_policy::ManagedPolicy, policy::Policy};

pub struct Role {
    policy: Policy,
    managed_policies: Vec<ManagedPolicy>,
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
        map.serialize_entry("AssumeRolePolicyDocument", &self.policy)?;

        if !self.managed_policies.is_empty() {
            map.serialize_entry("ManagedPolicyArns", &self.managed_policies)?;
        }

        map.end()
    }
}
