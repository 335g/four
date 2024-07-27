use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Effect {
    Allow,
    Deny,
}
