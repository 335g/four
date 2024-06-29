use nutype::nutype;
use serde::{Serialize, Serializer};

use crate::{
    fn_join,
    function::{Join, Ref},
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
    pub fn builder(service: String, resource: String, account: Account) -> RefNameAccount {
        RefNameAccount {
            service,
            resource,
            account,
        }
    }
}

impl Serialize for ARN {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fn_join![
            "arn:",
            self.partition,
            self.service.clone(),
            self.region,
            self.account.clone(),
            self.resource.clone()
        ]
        .serialize(serializer)
    }
}

pub struct RefNameAccount {
    service: String,
    resource: String,
    account: Account,
}

impl RefNameAccount {
    pub fn region(self, region: Region) -> RefNameRegion {
        RefNameRegion {
            service: self.service,
            resource: self.resource,
            account: self.account,
            region,
        }
    }

    pub fn partition(self, partition: Partition) -> RefNamePartition {
        RefNamePartition {
            service: self.service,
            resouce: self.resource,
            account: self.account,
            partition,
        }
    }

    pub fn build(self) -> ARN {
        todo!()
    }
}

pub struct RefNameRegion {
    service: String,
    resource: String,
    account: Account,
    region: Region,
}

impl RefNameRegion {
    pub fn partition(self, partition: Partition) -> RefNameRegionPartition {
        RefNameRegionPartition {
            service: self.service,
            resouce: self.resource,
            account: self.account,
            region: self.region,
            partition,
        }
    }

    pub fn build(self) -> ARN {
        todo!()
    }
}

pub struct RefNamePartition {
    service: String,
    resouce: String,
    account: Account,
    partition: Partition,
}

impl RefNamePartition {
    pub fn region(self, region: Region) -> RefNameRegionPartition {
        RefNameRegionPartition {
            service: self.service,
            resouce: self.resouce,
            account: self.account,
            region,
            partition: self.partition,
        }
    }

    pub fn build(self) -> ARN {
        todo!()
    }
}

pub struct RefNameRegionPartition {
    service: String,
    resouce: String,
    account: Account,
    region: Region,
    partition: Partition,
}

impl RefNameRegionPartition {
    pub fn build(self) -> ARN {
        ARN {
            partition: self.partition,
            service: self.service,
            region: self.region,
            account: self.account,
            resource: self.resouce,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Partition {
    Ref,
    Aws,
    China,
    GovCloudUS,
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

#[nutype(validate(regex = r#"\d{8}"#), derive(Debug, Clone, Serialize))]
pub struct Account(String);
