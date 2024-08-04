use four::{convert::WillBe, logical_id::LogicalId, ManagedResource};
use serde::Serialize;

#[derive(ManagedResource, Clone)]
#[resource_type = "AWS::IAM::Group"]
pub struct Group {
    logical_id: LogicalId,
    group_name: Option<WillBe<GroupName>>,
    path: Option<String>,
    // policies: Option<Vec<
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupName(String);
