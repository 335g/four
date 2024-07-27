use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MemorySize(usize);

impl TryFrom<usize> for MemorySize {
    type Error = MemorySizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 128 || value > 10240 {
            Err(MemorySizeError::Invalid(value))
        } else {
            Ok(MemorySize(value))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MemorySizeError {
    #[error("invalid memory size: {0}")]
    Invalid(usize),
}
