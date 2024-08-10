use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
pub enum Version {
    #[serde(rename(serialize = "2012-10-17"))]
    V2012_10_17,
    #[serde(rename(serialize = "2008-10-17"))]
    V2008_10_17,
}

impl Version {
    pub fn latest() -> Version {
        Version::V2012_10_17
    }
}
