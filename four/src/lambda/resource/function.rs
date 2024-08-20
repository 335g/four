use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    iam::RoleArn,
    lambda::{
        property::{architecture::Architectures, code::Code, handler::Handler, runtime::Runtime},
        FunctionArn, FunctionName, MemorySize, MemorySizeError, Timeout, TimeoutError,
    },
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::Lambda::Function"]
pub struct Function {
    logical_id: LogicalId,
    architectures: Option<Architectures>,
    code: Code,
    description: Option<String>,
    function_name: Option<WillBe<FunctionName>>,
    handler: Option<Handler>,
    memory_size: Option<MemorySize>,
    role: WillBe<RoleArn>,
    runtime: Option<Runtime>,
    timeout: Option<Timeout>,
}

impl Function {
    pub fn zip(
        logical_id: LogicalId,
        bucket_name: &str,
        key: &str,
        role: WillBe<RoleArn>,
        handler: Handler,
        runtime: Runtime,
    ) -> Function {
        let code = Code::new(bucket_name, key);

        Function::new(logical_id, code, role)
            .handler(handler)
            .runtime(runtime)
    }

    pub fn memory_size_value(mut self, value: usize) -> Result<Self, MemorySizeError> {
        let value = MemorySize::try_new(value)?;
        self.memory_size = Some(value);
        Ok(self)
    }

    pub fn timeout_value(mut self, timeout: usize) -> Result<Self, TimeoutError> {
        let timeout = Timeout::try_new(timeout)?;
        self.timeout = Some(timeout);
        Ok(self)
    }
}

impl Referenced for Function {
    type To = FunctionName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<FunctionArn> for Function {
    const KEY: &'static str = "Arn";
}
