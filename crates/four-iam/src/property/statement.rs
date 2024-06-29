#![allow(dead_code)]

use serde::Serialize;

use crate::property::{action::Action, effect::Effect};

use super::principal::Principal;

#[derive(Serialize)]
pub struct Statement {
    effect: Effect,
    #[serde(flatten)]
    action: ActionList,
    #[serde(flatten)]
    principal: Option<PrincipalList>,
}

#[derive(Serialize)]
pub enum ActionList {
    #[serde(rename(serialize = "Action"))]
    Applicable(Vec<Box<dyn Action>>),
    #[serde(rename(serialize = "NotAction"))]
    Excepting(Vec<Box<dyn Action>>),
}

#[derive(Serialize)]
pub enum PrincipalList {
    #[serde(rename(serialize = "Principal"))]
    Applicable(Principal),
    #[serde(rename(serialize = "NotPrincipal"))]
    Excepting(Principal),
}
