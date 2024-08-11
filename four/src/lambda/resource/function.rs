use crate::{
    core::{
        arn::Arn,
        convert::{WillBe, WillMappable},
        function::{
            getatt::{Attribute, HaveAtt},
            reference::{RefInner, Referenced},
        },
        logical_id::LogicalId,
        service::Lambda,
    },
    iam::resource::role::RoleArn,
    lambda::property::{
        architecture::Architectures, code::Code, handler::Handler, runtime::Runtime,
    },
};
use four_derive::ManagedResource;
use serde::Serialize;
use thiserror::Error;

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

    pub fn memory_size_value(mut self, value: usize) -> Result<Self, FunctionError> {
        let value = MemorySize::try_from(value)?;
        self.memory_size = Some(value);
        Ok(self)
    }

    pub fn timeout_value(mut self, timeout: usize) -> Result<Self, FunctionError> {
        let timeout = Timeout::try_from(timeout)?;
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

impl HaveAtt<FunctionArn> for Function {}

impl Attribute for FunctionArn {
    fn name() -> &'static str {
        "Arn"
    }
}

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("invalid fucntion name: {0}")]
    InvalidFunctionName(String),

    #[error("invalid memory size: {0}")]
    InvalidMemorySize(usize),

    #[error("invalid timeout: {0}")]
    InvalidTimeout(usize),
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionName(String);

impl TryFrom<&str> for FunctionName {
    type Error = FunctionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            Err(FunctionError::InvalidFunctionName(value.to_string()))
        } else {
            Ok(FunctionName(value.to_string()))
        }
    }
}

impl WillMappable<String> for FunctionName {}

#[derive(Debug, Clone, Serialize)]
pub struct MemorySize(usize);

impl TryFrom<usize> for MemorySize {
    type Error = FunctionError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 128 || value > 10240 {
            Err(FunctionError::InvalidMemorySize(value))
        } else {
            Ok(MemorySize(value))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Timeout(usize);

impl TryFrom<usize> for Timeout {
    type Error = FunctionError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 || value > 900 {
            Err(FunctionError::InvalidTimeout(value))
        } else {
            Ok(Timeout(value))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionArn(Arn<Lambda>);

impl From<Arn<Lambda>> for FunctionArn {
    fn from(value: Arn<Lambda>) -> Self {
        FunctionArn(value)
    }
}

#[cfg(test)]
mod tests {
    use four::{
        account::{Account, AccountDetail},
        service::IAM,
    };

    use super::*;

    #[test]
    fn test_function1() {
        let id = LogicalId::try_from("function-id").unwrap();
        let code = Code::new("mybucket", "mykey");
        let account = Account::Detail(AccountDetail::try_from("123456789012").unwrap());

        let role: RoleArn = Arn::builder(IAM, "abc", account).build().into();
        let arch = Architectures::x86_64();
        let function = Function::new(id, code, role.into());
    }
}