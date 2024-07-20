use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BucketKey(String);
