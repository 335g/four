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
pub mod error;
pub mod iam;
pub mod lambda;
pub mod resource;
pub mod s3;
