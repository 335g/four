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
mod macros;
mod pseudo_param;
mod region;
mod resource;
mod resource_name;

pub use error::Error;
pub use resource::Construct;
pub use resource_name::{Account, AccountError, Partition};
