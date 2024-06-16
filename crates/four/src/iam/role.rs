use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Role {
    version: Version,
    id: Id<Role>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Version {
    Latest,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Id<T> {
    id: Uuid,
    _phanto: PhantomData<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statement {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum Effect {
    Allow,
    Deny,
}
