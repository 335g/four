use four_core::{
    arn::Arn,
    convert::WillMappable,
    function::getatt::{Attribute, HaveAtt},
    logical_id::{LogicalId, LogicalIdentified},
    resource::ManagedResource,
    WillBe,
};
use serde::{ser::SerializeMap, Serialize};

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

impl Serialize for Bucket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct BucketInner<'a> {
            bucket_name: &'a Option<WillBe<BucketName>>,
        }

        impl Serialize for BucketInner<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(None)?;
                if let Some(bucket_name) = self.bucket_name {
                    map.serialize_entry("BucketName", bucket_name)?;
                }
                map.end()
            }
        }

        let inner = BucketInner {
            bucket_name: &self.bucket_name,
        };

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.logical_id, &inner)?;
        map.end()
    }
}

impl LogicalIdentified for Bucket {
    fn logical_id(&self) -> &LogicalId {
        &self.logical_id
    }
}

impl ManagedResource for Bucket {
    fn resource_type(&self) -> &'static str {
        "AWS::S3::Bucket"
    }
}

#[derive(Debug, Serialize)]
pub struct BucketName(String);

impl BucketName {
    pub fn will(self) -> WillBe<BucketName> {
        WillBe::new(Box::new(self))
    }
}

impl WillMappable<String> for BucketName {}

#[derive(Debug, Serialize)]
pub struct BucketArn(Arn);

impl HaveAtt<BucketArn> for Bucket {}

impl Attribute for BucketArn {
    fn name() -> &'static str {
        "Arn"
    }
}
