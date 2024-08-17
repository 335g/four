use std::io::Write;

use dotenvy::dotenv;
use four::{
    core::{function::get_att, template::Template, LogicalId},
    iam::{
        resource::{role::RoleArn, Role},
        Principal, ServicePrincipal,
    },
    lambda::{resource::Function, Handler, Runtime},
};

fn main() -> anyhow::Result<()> {
    dotenv()?;

    let role_id = LogicalId::try_from("roleid").unwrap();
    let role = Role::assume_role(role_id, Principal::from(ServicePrincipal::Lambda));
    let role_arn = get_att::<RoleArn, _>(&role);

    let function_id = LogicalId::try_from("functionid").unwrap();
    let handler = Handler::try_from("bootstrap").unwrap();
    let runtime = Runtime::ProvidedAl2023;
    let bucket_name = std::env::var("BUCKET_NAME")?;
    let key = std::env::var("LAMBDA_KEY")?;

    let function = Function::zip(function_id, &bucket_name, &key, role_arn, handler, runtime);

    let mut template = Template::new();
    template.insert_resource(Box::new(role));
    template.insert_resource(Box::new(function));

    let template_json = serde_json::to_string_pretty(&template).expect("is valid template");
    println!("{}", template_json,);

    let mut f = std::fs::File::create("output.json")?;
    f.write_all(template_json.as_bytes())?;

    Ok(())
}
