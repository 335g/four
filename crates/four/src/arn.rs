use serde::{Serialize, Serializer};

use crate::{
    account::Account,
    function::{join::Join, reference::Ref},
    pseudo,
    region::Region,
};

/// Amazon Resource Name (ARN)
///
#[derive(Debug, Clone)]
pub struct Arn {
    partition: Partition,
    service: String,
    region: Region,
    account: Account,
    resource: String,
}

impl Arn {
    pub fn builder(service: &str, resource: &str, account: Account) -> RefNameAccount {
        RefNameAccount {
            service: service.to_string(),
            account,
            resource: resource.to_string(),
        }
    }
}

impl Serialize for Arn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        enum StringOr {
            String(String),
            Or(Box<dyn erased_serde::Serialize>),
        }

        let mut contents: Vec<StringOr> = vec![];
        let service = self.service.clone();
        if let Some(partition) = self.partition.to_str() {
            contents.push(StringOr::String("arn".to_string()));
            contents.push(StringOr::String(partition.to_string()));
            contents.push(StringOr::String(service));
        } else {
            contents.push(StringOr::String("arn".to_string()));
            contents.push(StringOr::Or(Box::new(self.partition)));
            contents.push(StringOr::String(service));
        }

        if let Some(region) = self.region.to_str() {
            contents.push(StringOr::String(region.to_string()));
        } else {
            contents.push(StringOr::Or(Box::new(self.region)));
        }

        if let Some(account) = self.account.to_str() {
            contents.push(StringOr::String(account.to_string()));
        } else {
            contents.push(StringOr::Or(Box::new(self.account.clone())));
        }

        contents.push(StringOr::String(self.resource.clone()));

        let contents = contents
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, x)| {
                if i != 0 {
                    let tail = acc.pop().expect("at least 1 value");
                    match (tail, x) {
                        (StringOr::String(tail), StringOr::String(x)) => {
                            acc.push(StringOr::String(format!("{tail}:{x}")));
                        }
                        (StringOr::String(tail), StringOr::Or(x)) => {
                            acc.push(StringOr::String(format!("{tail}:")));
                            acc.push(StringOr::Or(x));
                        }
                        (StringOr::Or(tail), StringOr::String(x)) => {
                            acc.push(StringOr::Or(tail));
                            acc.push(StringOr::String(format!(":{x}")));
                        }
                        (StringOr::Or(tail), StringOr::Or(x)) => {
                            acc.push(StringOr::Or(tail));
                            acc.push(StringOr::String(":".to_string()));
                            acc.push(StringOr::Or(x));
                        }
                    }
                } else {
                    acc.push(x);
                }

                acc
            });

        if contents.len() == 1 {
            let elem = &contents[0];
            let StringOr::String(elem) = elem else {
                unreachable!("first element is string")
            };

            elem.serialize(serializer)
        } else {
            let contents = contents
                .into_iter()
                .map(|x| -> Box<dyn erased_serde::Serialize> {
                    match x {
                        StringOr::String(x) => Box::new(x),
                        StringOr::Or(x) => Box::new(x),
                    }
                })
                .collect::<Vec<_>>();

            Join(contents).serialize(serializer)
        }
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

    pub fn build(self) -> Arn {
        Arn {
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

    pub fn build(self) -> Arn {
        Arn {
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

    pub fn build(self) -> Arn {
        Arn {
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
    pub fn build(self) -> Arn {
        Arn {
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
            Partition::Ref => Ref::new(pseudo::Partition).serialize(serializer),
            Partition::Aws => "aws".serialize(serializer),
            Partition::China => "aws-cn".serialize(serializer),
            Partition::GovCloudUS => "aws-us-gov".serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arn1() {
        let arn = Arn::builder("s", "r", Account::Aws)
            .partition(Partition::Aws)
            .region(Region::Ref)
            .build();
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#"{"Fn::Join":["arn:aws:s:",{"Ref":"AWS::Region"},":aws:r"]}"#;
        assert_eq!(s, rhs);
    }

    #[test]
    fn test_arn2() {
        let arn = Arn::builder("s", "r", Account::Aws)
            .partition(Partition::Aws)
            .build();
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#""arn:aws:s::aws:r""#;
        assert_eq!(s, rhs);
    }

    #[test]
    fn test_arn3() {
        // minimum settings
        //  partition => pseudo::Partition
        //  region => no region
        let arn = Arn::builder("s", "r", Account::Aws).build();
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#"{"Fn::Join":["arn:",{"Ref":"AWS::Partition"},":s::aws:r"]}"#;
        assert_eq!(s, rhs);
    }
}
