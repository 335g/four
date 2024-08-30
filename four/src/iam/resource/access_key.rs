use crate::{
    core::{
        convert::WillBe,
        function::{HaveAtt, RefInner, Referenced},
        LogicalId,
    },
    iam::{AccessKeyId, AccessKeyStatus, SecretAccessKey, UserName},
    ManagedResource,
};

/// [AWS::IAM::AccessKey]
///
/// Creates a new AWS secret access key and corresponding AWS access key ID
/// for the specified user. The default status for new keys is Active.
/// For information about quotas on the number of keys you can create,
/// see IAM and AWS STS quotas in the IAM User Guide.
///
/// ```
/// use four::{
///     LogicalId,
///     iam::{
///         resource::AccessKey,
///         UserName,
///     },
/// };
///
/// let access_key_id = LogicalId::try_from("myaccesskey").unwrap();
/// let user_name = UserName::try_new("myuser-name").unwrap();
/// let access_key = AccessKey::new(access_key_id, user_name.into());
///
/// let lhs = serde_json::to_string(&access_key).unwrap();
/// let mut rhs = r#"
///     {
///         "Type": "AWS::IAM::AccessKey",
///         "Properties": {
///             "UserName": "myuser-name"
///         }
///     }
/// "#.to_string();
/// rhs.retain(|c| c != '\n' && c != ' ');
///
/// assert_eq!(lhs, rhs);
///
/// ```
///
/// [AWS::IAM::AccessKey]: https://docs.aws.amazon.com/ja_jp/AWSCloudFormation/latest/UserGuide/aws-resource-iam-accesskey.html
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
