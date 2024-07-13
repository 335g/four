#![allow(dead_code)]

use std::marker::PhantomData;

use serde::Serialize;

use crate::property::{action::Action, effect::Effect};

use super::principal::Principal;

#[derive(Serialize)]
pub struct Statement {
    effect: Effect,
    #[serde(flatten)]
    action: Option<ActionList>,
    #[serde(flatten)]
    principal: PrincipalList,
}

impl Statement {
    // pub fn new(effect: Effect, action: ActionList, principal: Option<PrincipalList>) -> Self {
    //     Statement {
    //         effect,
    //         action,
    //         principal,
    //     }
    // }
    pub fn builder() -> Statement1Builder {
        Statement1Builder
    }
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

pub struct Statement1Builder;

impl Statement1Builder {
    pub fn effect(self, effect: Effect) -> Statement2Builder<PrincipalTypeInit> {
        Statement2Builder {
            effect,
            principal: None,
            action: None,
            _ghost: PhantomData,
        }
    }
}

pub struct Statement2Builder<T> {
    effect: Effect,
    principal: Option<PrincipalList>,
    action: Option<ActionList>,

    _ghost: PhantomData<T>,
}

impl<T> Statement2Builder<T> {
    pub fn principal(self, principal: PrincipalList) -> Statement2Builder<PrincipalTypeFixed> {
        Statement2Builder {
            effect: self.effect,
            principal: Some(principal),
            action: self.action,
            _ghost: PhantomData,
        }
    }

    pub fn action(mut self, action: ActionList) -> Self {
        self.action = Some(action);
        self
    }
}

impl Statement2Builder<PrincipalTypeFixed> {
    pub fn build(self) -> Statement {
        Statement {
            effect: self.effect,
            action: self.action,
            principal: self.principal.unwrap(),
        }
    }
}

pub enum PrincipalTypeInit {}
pub enum PrincipalTypeFixed {}
