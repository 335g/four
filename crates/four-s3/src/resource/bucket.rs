use four::{
    arn::Arn,
    convert::{WillBe, WillMappable},
    function::getatt::{Attribute, HaveAtt},
    logical_id::{LogicalId, LogicalIdentified},
    ManagedResource,
};
use serde::Serialize;

#[derive(ManagedResource)]
#[resource_type = "AWS::S3::Bucket"]
pub struct Bucket {
    logical_id: LogicalId,
    bucket_name: Option<WillBe<BucketName>>,
}

impl Bucket {
    pub fn new(logical_id: LogicalId) -> Self {
        Bucket {
            logical_id,
            bucket_name: None,
        }
    }

    pub fn name(mut self, name: WillBe<String>) -> Self {
        self.bucket_name = Some(name.map());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct BucketName(String);

impl WillMappable<String> for BucketName {}

#[derive(Debug, Serialize)]
pub struct BucketArn(Arn);

impl HaveAtt<BucketArn> for Bucket {}

impl Attribute for BucketArn {
    fn name() -> &'static str {
        "Arn"
    }
}
