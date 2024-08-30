use crate::iam::property::{action::Action, effect::Effect, principal::Principal};
use serde::{ser::SerializeMap, Serialize};

#[derive(Debug, Clone)]
pub struct Statement {
    effect: Effect,
    action: ActionOr,
    principal: Option<PrincipalOr>,
    resource: Option<String>,
}

impl Statement {
    pub fn allow_actions(actions: Vec<Box<dyn Action>>) -> StatementBuilder {
        StatementBuilder {
            effect: Effect::Allow,
            action: ActionOr::Action(actions),
            principal: None,
            resource: None,
        }
    }

    pub fn deny_actions(actions: Vec<Box<dyn Action>>) -> StatementBuilder {
        StatementBuilder {
            effect: Effect::Deny,
            action: ActionOr::Action(actions),
            principal: None,
            resource: None,
        }
    }

    pub fn allow_no_actions(actions: Vec<Box<dyn Action>>) -> StatementBuilder {
        StatementBuilder {
            effect: Effect::Allow,
            action: ActionOr::NotAction(actions),
            principal: None,
            resource: None,
        }
    }

    pub fn deny_no_actions(actions: Vec<Box<dyn Action>>) -> StatementBuilder {
        StatementBuilder {
            effect: Effect::Deny,
            action: ActionOr::NotAction(actions),
            principal: None,
            resource: None,
        }
    }
}

impl Serialize for Statement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("Effect", &self.effect)?;
        match &self.action {
            ActionOr::Action(action) => map.serialize_entry("Action", action)?,
            ActionOr::NotAction(action) => map.serialize_entry("NotAction", action)?,
        }
        if let Some(principal) = self.principal.as_ref() {
            match principal {
                PrincipalOr::Principal(p) => map.serialize_entry("Principal", p)?,
                PrincipalOr::NotPrincipal(p) => map.serialize_entry("NotPrincipal", p)?,
            }
        }
        if let Some(resource) = self.resource.as_ref() {
            map.serialize_entry("Resource", resource)?;
        }

        map.end()
    }
}

#[derive(Debug)]
pub struct StatementBuilder {
    effect: Effect,
    action: ActionOr,
    principal: Option<PrincipalOr>,
    resource: Option<String>,
}

impl StatementBuilder {
    pub fn build(self) -> Result<Statement, StatementBuildError> {
        // TODO: ERROR

        let statement = Statement {
            effect: self.effect,
            action: self.action,
            principal: self.principal,
            resource: self.resource,
        };

        Ok(statement)
    }

    pub fn principal(mut self, principal: Principal) -> Self {
        self.principal = Some(PrincipalOr::Principal(principal));
        self
    }

    pub fn no_principal(mut self, principal: Principal) -> Self {
        self.principal = Some(PrincipalOr::NotPrincipal(principal));
        self
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StatementBuildError {}

#[derive(Debug, Clone)]
pub enum ActionOr {
    Action(Vec<Box<dyn Action>>),
    NotAction(Vec<Box<dyn Action>>),
}

#[derive(Debug, Clone)]
pub enum PrincipalOr {
    Principal(Principal),
    NotPrincipal(Principal),
}
