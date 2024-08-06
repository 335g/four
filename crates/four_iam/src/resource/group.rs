use four::{
    arn::Arn,
    convert::{WillBe, WillMappable},
    function::{
        getatt::{Attribute, HaveAtt},
        reference::Referenced,
    },
    logical_id::LogicalId,
    service::IAM,
    ManagedResource,
};
use serde::Serialize;

use crate::resource::{managed_policy::ManagedPolicyArn, policy::Policy};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Group"]
pub struct Group {
    logical_id: LogicalId,
    group_name: Option<WillBe<GroupName>>,
    managed_policy_arns: Option<Vec<WillBe<ManagedPolicyArn>>>,
    path: Option<String>,
    policies: Option<Vec<Policy>>,
}

impl Group {
    pub fn new(logical_id: LogicalId) -> Group {
        Group {
            logical_id,
            group_name: None,
            managed_policy_arns: None,
            path: None,
            policies: None,
        }
    }

    pub fn group_name(mut self, name: WillBe<GroupName>) -> Group {
        self.group_name = Some(name);
        self
    }

    pub fn managed_policy_arns(mut self, arns: Vec<WillBe<ManagedPolicyArn>>) -> Group {
        self.managed_policy_arns = Some(arns);
        self
    }

    pub fn path(mut self, path: String) -> Group {
        self.path = Some(path);
        self
    }

    pub fn policies(mut self, policies: Vec<Policy>) -> Group {
        self.policies = Some(policies);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupName(String);

impl WillMappable<String> for GroupName {}

impl Referenced for Group {
    type To = GroupName;

    fn referenced(&self) -> four::function::reference::RefInner {
        four::function::reference::RefInner::Id(self.logical_id.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupArn(Arn<IAM>);

impl Attribute for GroupArn {
    fn name() -> &'static str {
        "Arn"
    }
}

impl HaveAtt<GroupArn> for Group {}