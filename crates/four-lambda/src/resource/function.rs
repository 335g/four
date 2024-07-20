use four_core::{
    convert::WillBe,
    logical_id::{LogicalId, LogicalIdentified},
    resource::ManagedResource,
};
use four_iam::resource::role::RoleArn;
use serde::{self, ser::SerializeMap, Serialize};

use crate::property::{
    architecture::{self, Architecture, Architectures},
    code::Code,
    handler::Handler,
    runtime::Runtime,
};

pub struct Function {
    logical_id: LogicalId,
    architectures: Architectures,
    code: Code,
    handler: Option<Handler>,
    role: WillBe<RoleArn>,
    runtime: Option<Runtime>,
}

impl Function {
    pub fn new(logical_id: LogicalId, role: WillBe<RoleArn>, code: Code) -> Function {
        let architectures = Architectures::x86_64();

        Function {
            logical_id,
            architectures,
            code,
            handler: None,
            role,
            runtime: None,
        }
    }

    pub fn zip(
        logical_id: LogicalId,
        role: WillBe<RoleArn>,
        bucket_name: &str,
        key: &str,
        handler: Handler,
        runtime: Runtime,
    ) -> Function {
        let code = Code::new(bucket_name, key);

        Function::new(logical_id, role, code)
            .handler(handler)
            .runtime(runtime)
    }

    pub fn architecture(mut self, architecture: Architecture) -> Self {
        self.architectures = Architectures::with(architecture);
        self
    }

    pub fn handler(mut self, handler: Handler) -> Self {
        self.handler = Some(handler);
        self
    }

    pub fn runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }
}

impl Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct FunctionInner<'a> {
            architectures: &'a Architectures,
            code: &'a Code,
            handler: &'a Option<Handler>,
            role: &'a WillBe<RoleArn>,
            runtime: &'a Option<Runtime>,
        }

        impl Serialize for FunctionInner<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("Architectures", self.architectures)?;
                map.serialize_entry("Code", self.code)?;
                if let Some(handler) = self.handler {
                    map.serialize_entry("Handler", handler)?;
                }
                map.serialize_entry("Role", self.role)?;
                if let Some(runtime) = self.runtime {
                    map.serialize_entry("Runtime", runtime)?;
                }
                map.end()
            }
        }

        let inner = FunctionInner {
            architectures: &self.architectures,
            code: &self.code,
            handler: &self.handler,
            role: &self.role,
            runtime: &self.runtime,
        };

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(self.logical_id(), &inner)?;
        map.end()
    }
}

impl ManagedResource for Function {
    fn resource_type(&self) -> &'static str {
        "AWS::Lambda::Function"
    }
}

impl LogicalIdentified for Function {
    fn logical_id(&self) -> &LogicalId {
        &self.logical_id
    }
}
