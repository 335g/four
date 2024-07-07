use serde::Serialize;

#[non_exhaustive]
#[derive(Serialize)]
pub enum Principal {
    Service(String),
}

impl Principal {
    pub fn service(name: &str) -> Self {
        Principal::Service(format!("{name}.amazonaws.com"))
    }
}
