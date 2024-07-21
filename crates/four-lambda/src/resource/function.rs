use four::{
    convert::WillBe,
    logical_id::{LogicalId, LogicalIdentified},
    ManagedResource,
};
use four_iam::resource::role::RoleArn;
use serde::{self, ser::SerializeMap, Serialize};

use crate::property::{
    architecture::{self, Architecture, Architectures},
    code::Code,
    handler::Handler,
    runtime::Runtime,
};

#[derive(ManagedResource)]
#[resource_type = "AWS::Lambda::Function"]
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
