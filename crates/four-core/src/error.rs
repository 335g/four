#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid resource")]
    InvalidResource,
}
