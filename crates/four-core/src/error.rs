#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid resource")]
    InvalidResource,

    #[error("Invalid logical identifier: {0}")]
    InvalidLogicalId(String),
}
