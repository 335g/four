mod error;
mod property;
mod resource;

pub use error::S3Error as Error;
pub use property::bucket::{BucketName, BucketNameError, Key, KeyError};
