use std::io::Write;

use dotenvy::dotenv;
use four::{function::get_att, logical_id::LogicalId, template::Template};
use four_iam::resource::role::{Role, RoleArn};
use four_lambda::{
    property::{handler::Handler, runtime::Runtime},
    resource::function::Function,
};

fn main() -> anyhow::Result<()> {
    dotenv()?;

    let role_id = LogicalId::try_from("roleid").unwrap();
    let role = Role::lambda_execution(role_id);
    let role_arn = get_att::<RoleArn, _>(&role);

    let function_id = LogicalId::try_from("functionid").unwrap();
    let handler = Handler::try_from("bootstrap").unwrap();
    let runtime = Runtime::ProvidedAl2023;
    let bucket_name = std::env::var("BUCKET_NAME")?;
    let key = std::env::var("LAMBDA_KEY")?;

    let function = Function::zip(function_id, &bucket_name, &key, role_arn, handler, runtime);

    let mut template = Template::new(vec![], vec![]);
    template.insert(Box::new(role));
    template.insert(Box::new(function));

    let template_json = serde_json::to_string_pretty(&template).expect("is valid template");
    println!("{}", template_json,);

    let mut f = std::fs::File::create("output.json")?;
    f.write_all(template_json.as_bytes())?;

    Ok(())
}
