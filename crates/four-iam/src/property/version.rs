#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Version {
    #[serde(rename(serialize = "2012-10-17"))]
    V2012_10_17,

    #[serde(rename(serialize = "2008-10-17"))]
    V2008_10_17,
}

impl Version {
    pub fn latest() -> Self {
        Version::V2012_10_17
    }
}
