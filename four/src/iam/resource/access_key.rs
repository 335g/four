use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    iam::{AccessKeyId, AccessKeyStatus, SecretAccessKey, UserName},
    ManagedResource,
};

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::AccessKey"]
pub struct AccessKey {
    logical_id: LogicalId,
    serial: Option<i64>,
    status: Option<AccessKeyStatus>,
    user_name: WillBe<UserName>,
}

impl Referenced for AccessKey {
    type To = AccessKeyId;

    fn referenced(&self) -> RefInner {
        RefInner::Id(self.logical_id.clone())
    }
}

impl HaveAtt<SecretAccessKey> for AccessKey {
    const KEY: &'static str = "SecretAccessKey";
}
