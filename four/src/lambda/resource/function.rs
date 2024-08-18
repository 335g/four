use crate::{
    core::{
        convert::{WillBe, WillMappable},
        function::{HaveAtt, RefInner, Referenced},
        service::Lambda,
        Arn, LogicalId,
    },
    iam::resource::role::RoleArn,
    lambda::property::{
        architecture::Architectures, code::Code, handler::Handler, runtime::Runtime,
    },
};
use four_derive::ManagedResource;
use nutype::nutype;
use serde::Serialize;

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

#[nutype(validate(len_char_min = 1), derive(Debug, Clone, Serialize))]
pub struct FunctionName(String);

impl WillMappable<String> for FunctionName {}

#[nutype(
    validate(greater_or_equal = 128, less_or_equal = 10240),
    derive(Debug, Clone, Serialize)
)]
pub struct MemorySize(usize);

#[nutype(
    validate(greater = 0, less_or_equal = 900),
    derive(Debug, Clone, Serialize)
)]
pub struct Timeout(usize);

#[derive(Debug, Clone, Serialize)]
pub struct FunctionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for FunctionArn {
    fn from(value: Arn<Lambda>) -> Self {
        FunctionArn(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{arn_builder, service::IAM, Account, AccountDetail};

    use super::*;

    #[test]
    fn test_function1() {
        let id = LogicalId::try_from("function-id").unwrap();
        let code = Code::new("mybucket", "mykey");
        let account = Account::Detail(AccountDetail::try_from("123456789012").unwrap());

        let role: RoleArn = arn_builder("abc", account).build(IAM).into();
        let arch = Architectures::x86_64();
        let function = Function::new(id, code, role.into());
    }
}
