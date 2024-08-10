use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Architecture {
    #[serde(rename(serialize = "x86_64"))]
    X86_64,
    #[serde(rename(serialize = "arm64"))]
    Arm64,
}

#[derive(Debug, Clone, Serialize)]
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
