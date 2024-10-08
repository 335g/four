use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    iam::{path::Path, GroupArn, GroupName, GroupPolicy, ManagedPolicyArn},
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Group"]
pub struct Group {
    logical_id: LogicalId,
    group_name: Option<WillBe<GroupName>>,
    managed_policy_arns: Option<Vec<WillBe<ManagedPolicyArn>>>,
    path: Option<Path>,
    policies: Option<Vec<GroupPolicy>>,
}

impl Referenced for Group {
    type To = GroupName;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<GroupArn> for Group {
    const KEY: &'static str = "Arn";
}
