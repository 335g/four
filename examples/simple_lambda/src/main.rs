use four::{function::get_att, logical_id::LogicalId, template::Template};
use four_iam::{
    property::{
        action,
        effect::Effect,
        policy::Policy,
        principal::{Principal, ServicePrincipal},
        statement::{ActionOr, PrincipalOr, Statement},
    },
    resource::role::{Role, RoleArn},
};
use four_lambda::{
    property::{handler::Handler, runtime::Runtime},
    resource::function::Function,
};

fn main() -> anyhow::Result<()> {
    let role_id = LogicalId::try_from("role-id").unwrap();
    let role = Role::lambda_execution(role_id);
    let role_arn = get_att::<RoleArn, _>(&role);

    let function_id = LogicalId::try_from("function-id").unwrap();
    let handler = Handler::try_from("bootstrap").unwrap();
    let runtime = Runtime::ProvidedAl2023;
    let function = Function::zip(
        function_id,
        "mybucketname",
        "mykey",
        role_arn,
        handler,
        runtime,
    );

    let mut template = Template::new(vec![], vec![]);
    template.insert(Box::new(role));
    template.insert(Box::new(function));

    println!(
        "{}",
        serde_json::to_string_pretty(&template).expect("is valid template"),
    );

    Ok(())
}
