use serde::Serialize;

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
pub enum Principal {
    Service(Vec<ServicePrincipal>),
}

impl From<ServicePrincipal> for Principal {
    fn from(value: ServicePrincipal) -> Self {
        let mut v = Vec::with_capacity(1);
        v.push(value);

        Principal::Service(v)
    }
}

impl From<Vec<ServicePrincipal>> for Principal {
    fn from(value: Vec<ServicePrincipal>) -> Self {
        Principal::Service(value)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ServicePrincipal {
    Lambda,
}

impl Serialize for ServicePrincipal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let elem = match self {
            ServicePrincipal::Lambda => "lambda.amazonaws.com",
        };
        elem.serialize(serializer)
    }
}
