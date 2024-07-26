pub trait Service {
    fn to_string(&self) -> String;
}

macro_rules! services {
    ($($service_name:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $service_name;

            impl serde::Serialize for $service_name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&self.to_string())
                }
            }

            impl Service for $service_name {
                fn to_string(&self) -> String {
                    format!("{}", stringify!($service_name)).to_lowercase()
                }
            }
        )*
    };
}

services!(EC2, IAM, Lambda, S3);
