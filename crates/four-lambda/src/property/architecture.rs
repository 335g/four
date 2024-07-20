use std::arch::x86_64;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Architecture {
    #[serde(rename(serialize = "x86_64"))]
    X86_64,
    #[serde(rename(serialize = "arm64"))]
    Arm64,
}

#[derive(Debug, Serialize)]
pub struct Architectures(Vec<Architecture>);

impl Architectures {
    pub fn x86_64() -> Architectures {
        Architectures::with(Architecture::X86_64)
    }

    pub fn arm64() -> Architectures {
        Architectures::with(Architecture::Arm64)
    }

    pub(crate) fn with(architecture: Architecture) -> Architectures {
        let mut inner = Vec::with_capacity(1);
        inner.push(architecture);

        Architectures(inner)
    }
}
