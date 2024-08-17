mod property;
pub mod resource;
mod util;

pub use property::{
    action,
    effect::Effect,
    policy_document::PolicyDocument,
    principal::{Principal, ServicePrincipal},
    statement::{ActionOr, PrincipalOr, Statement, StatementBuilder1, StatementBuilder2},
};
pub use util::{Path, PathError};
