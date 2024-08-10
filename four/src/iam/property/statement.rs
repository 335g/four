use crate::iam::property::{action::Action, effect::Effect, principal::Principal};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    effect: Effect,

    #[serde(flatten)]
    action: ActionOr,

    #[serde(flatten)]
    principal: PrincipalOr,
}

impl Statement {
    pub fn allow() -> StatementBuilder1 {
        StatementBuilder1 {
            effect: Effect::Allow,
        }
    }

    pub fn deny() -> StatementBuilder1 {
        StatementBuilder1 {
            effect: Effect::Deny,
        }
    }
}

pub struct StatementBuilder1 {
    effect: Effect,
}

impl StatementBuilder1 {
    pub fn action(self, action: Vec<Box<dyn Action>>) -> StatementBuilder2 {
        StatementBuilder2 {
            effect: self.effect,
            action: ActionOr::Action(action),
        }
    }

    pub fn not_action(self, action: Vec<Box<dyn Action>>) -> StatementBuilder2 {
        StatementBuilder2 {
            effect: self.effect,
            action: ActionOr::NotAction(action),
        }
    }
}

pub struct StatementBuilder2 {
    effect: Effect,
    action: ActionOr,
}

impl StatementBuilder2 {
    pub fn principal(self, principal: Principal) -> Statement {
        Statement {
            effect: self.effect,
            action: self.action,
            principal: PrincipalOr::Principal(principal),
        }
    }

    pub fn no_principal(self, principal: Principal) -> Statement {
        Statement {
            effect: self.effect,
            action: self.action,
            principal: PrincipalOr::NotPrincipal(principal),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ActionOr {
    Action(Vec<Box<dyn Action>>),
    NotAction(Vec<Box<dyn Action>>),
}

#[derive(Debug, Clone, Serialize)]
pub enum PrincipalOr {
    Principal(Principal),
    NotPrincipal(Principal),
}
