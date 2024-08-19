use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}
