use crate::{
    core::{
        convert::WillBe,
        function::{RefInner, Referenced},
        LogicalId,
    },
    iam::{GroupName, UserName, UserToGroupAdditionId},
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::UserToGroupAddition"]
pub struct UserToGroupAddition {
    logical_id: LogicalId,
    group_name: WillBe<GroupName>,
    users: Vec<WillBe<UserName>>,
}

impl Referenced for UserToGroupAddition {
    type To = UserToGroupAdditionId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}
