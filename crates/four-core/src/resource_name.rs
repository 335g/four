use nutype::nutype;
use serde::{Serialize, Serializer};

use crate::{
    fn_join,
    function::{join::Join, reference::Ref},
    pseudo_param,
    region::Region,
};

#[derive(Debug)]
pub struct ARN {
    partition: Partition,
    service: String,
    region: Region,
    account: Account,
    resource: String,
}

impl ARN {
    pub fn builder(service: &str, resource: &str, account: Account) -> RefNameAccount {
        RefNameAccount {
            service: service.to_string(),
            account,
            resource: resource.to_string(),
        }
    }
}

impl Serialize for ARN {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let service = self.service.clone();
        let join = if let Some(partition) = self.partition.to_str() {
            fn_join!(format!("arn:{partition}:{service}"))
        } else {
            fn_join!("arn:", self.partition, service)
        };

        let join = if let Some(region) = self.region.to_str() {
            fn_join!(join, [format!(":{region}")])
        } else {
            fn_join!(join, [":", self.region])
        };

        let join = if let Some(account) = self.account.to_str() {
            fn_join!(join, [format!(":{account}")])
        } else {
            fn_join!(join, [":", self.account.clone()])
        };

        let resource = self.resource.clone();
        let join = fn_join!(join, [format!(":{resource}")]);

        join.serialize(serializer)
    }
}

pub struct RefNameAccount {
    service: String,
    account: Account,
    resource: String,
}

impl RefNameAccount {
    pub fn region(self, region: Region) -> RefNameRegion {
        RefNameRegion {
            service: self.service,
            region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn partition(self, partition: Partition) -> RefNamePartition {
        RefNamePartition {
            partition,
            service: self.service,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build(self) -> ARN {
        ARN {
            partition: Partition::Ref,
            service: self.service,
            region: Region::Null,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNameRegion {
    service: String,
    region: Region,
    account: Account,
    resource: String,
}

impl RefNameRegion {
    pub fn partition(self, partition: Partition) -> RefNameRegionPartition {
        RefNameRegionPartition {
            partition,
            service: self.service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build(self) -> ARN {
        ARN {
            partition: Partition::Ref,
            service: self.service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNamePartition {
    partition: Partition,
    service: String,
    account: Account,
    resource: String,
}

impl RefNamePartition {
    pub fn region(self, region: Region) -> RefNameRegionPartition {
        RefNameRegionPartition {
            partition: self.partition,
            service: self.service,
            region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build(self) -> ARN {
        ARN {
            partition: self.partition,
            service: self.service,
            region: Region::Null,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNameRegionPartition {
    partition: Partition,
    service: String,
    region: Region,
    account: Account,
    resource: String,
}

impl RefNameRegionPartition {
    pub fn build(self) -> ARN {
        ARN {
            partition: self.partition,
            service: self.service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Partition {
    Ref,
    Aws,
    China,
    GovCloudUS,
}

impl Partition {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Partition::Ref => None,
            Partition::Aws => Some("aws"),
            Partition::China => Some("aws-cn"),
            Partition::GovCloudUS => Some("aws-us-gov"),
        }
    }
}

impl Serialize for Partition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Partition::Ref => Ref::new(pseudo_param::Partition).serialize(serializer),
            Partition::Aws => "aws".serialize(serializer),
            Partition::China => "aws-cn".serialize(serializer),
            Partition::GovCloudUS => "aws-us-gov".serialize(serializer),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Account {
    Ref,
    Null,
    Aws,
    Detail(AccountDetail),
}

impl Account {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Account::Ref => None,
            Account::Null => Some(""),
            Account::Aws => Some("aws"),
            Account::Detail(s) => Some(s.as_str()),
        }
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Account::Ref => Ref::new(pseudo_param::AccountId).serialize(serializer),
            Account::Null => "".serialize(serializer),
            Account::Aws => "aws".serialize(serializer),
            Account::Detail(x) => x.serialize(serializer),
        }
    }
}

#[nutype(validate(regex = r#"\d{8}"#), derive(Debug, Clone, Serialize, Deref))]
pub struct AccountDetail(String);
