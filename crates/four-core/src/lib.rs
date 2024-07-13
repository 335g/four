mod convert;
///
/// ```rust
/// use four::{
///     Result,
///     iam::resource::Role,
///     lambda::resource::Function,
/// };
///
/// #[four::stack(
///     description = "",
/// )]
/// async fn stack() -> Result<()> {
///     let mut role = Role::lambda_basic_execution();
///     role.name = "role1";
///
///     let function = Function::builder(roke, code).build()?;
///     
///     Ok(())
/// }
/// ```
///
///
///
///
///
mod error;
mod function;
mod logical_id;
mod macros;
mod parameter;
mod pseudo_param;
mod region;
mod resource;
mod resource_name;

pub use convert::{WillBe, WillBeString};
pub use error::Error;
pub use function::{r#ref, reference::Ref};
pub use logical_id::{LogicalId, LogicalIdentified};
pub use parameter::Parameter;
pub use resource::ManagedResource;
pub use resource_name::{Account, Partition, ARN};
