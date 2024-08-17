use serde::{Serialize, Serializer};

use crate::core::{
    account::Account, function::Join, function::JoinElement, partition::Partition, region::Region,
    service::Service,
};

/// Amazon Resource Name (ARN)
///
#[derive(Debug, Clone)]
pub struct Arn<Se: Service> {
    partition: Partition,
    service: Se,
    region: Region,
    account: Account,
    resource: String,
}

/// Amazon Resource Name (ARN) for type erasure
///
#[derive(Debug, Clone)]
pub struct AnyArn {
    partition: Partition,
    service: Box<dyn Service>,
    region: Region,
    account: Account,
    resource: String,
}

pub fn arn_builder(resource: &str, account: Account) -> RefNameAccount {
    RefNameAccount {
        account,
        resource: resource.to_string(),
    }
}

impl<Se> From<Arn<Se>> for AnyArn
where
    Se: Service,
{
    fn from(value: Arn<Se>) -> AnyArn {
        AnyArn {
            partition: value.partition,
            service: Box::new(value.service),
            region: value.region,
            account: value.account,
            resource: value.resource,
        }
    }
}

fn serialize<S>(
    partition: &Partition,
    service: &dyn Service,
    region: &Region,
    account: &Account,
    resource: &str,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Clone)]
    enum StringOr {
        String(String),
        Or(Box<dyn JoinElement>),
    }

    let mut contents: Vec<StringOr> = vec![];
    if let Some(partition) = partition.to_str() {
        contents.push(StringOr::String("arn".to_string()));
        contents.push(StringOr::String(partition.to_string()));
        contents.push(StringOr::String(service.to_string()));
    } else {
        contents.push(StringOr::String("arn".to_string()));
        contents.push(StringOr::Or(Box::new(partition.clone())));
        contents.push(StringOr::String(service.to_string()));
    }

    if let Some(region) = region.to_str() {
        contents.push(StringOr::String(region.to_string()));
    } else {
        contents.push(StringOr::Or(Box::new(region.clone())));
    }

    if let Some(account) = account.to_str() {
        contents.push(StringOr::String(account.to_string()));
    } else {
        contents.push(StringOr::Or(Box::new(account.clone())));
    }

    contents.push(StringOr::String(resource.to_string()));

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
            .map(|x| -> Box<dyn JoinElement> {
                match x {
                    StringOr::String(x) => Box::new(x),
                    StringOr::Or(x) => Box::new(x),
                }
            })
            .collect::<Vec<_>>();

        Join(contents).serialize(serializer)
    }
}

impl Serialize for AnyArn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize(
            &self.partition,
            &*self.service,
            &self.region,
            &self.account,
            &self.resource,
            serializer,
        )
    }
}

impl<Se> Serialize for Arn<Se>
where
    Se: Service,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize(
            &self.partition,
            &self.service,
            &self.region,
            &self.account,
            &self.resource,
            serializer,
        )
    }
}

pub struct RefNameAccount {
    account: Account,
    resource: String,
}

impl RefNameAccount {
    pub fn region(self, region: Region) -> RefNameRegion {
        RefNameRegion {
            region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn partition(self, partition: Partition) -> RefNamePartition {
        RefNamePartition {
            partition,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build<Se: Service>(self, service: Se) -> Arn<Se> {
        Arn {
            partition: Partition::Ref,
            service,
            region: Region::Null,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNameRegion {
    region: Region,
    account: Account,
    resource: String,
}

impl RefNameRegion {
    pub fn partition(self, partition: Partition) -> RefNameRegionPartition {
        RefNameRegionPartition {
            partition,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build<Se: Service>(self, service: Se) -> Arn<Se> {
        Arn {
            partition: Partition::Ref,
            service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNamePartition {
    partition: Partition,
    account: Account,
    resource: String,
}

impl RefNamePartition {
    pub fn region(self, region: Region) -> RefNameRegionPartition {
        RefNameRegionPartition {
            partition: self.partition,
            region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build<Se: Service>(self, service: Se) -> Arn<Se> {
        Arn {
            partition: self.partition,
            service,
            region: Region::Null,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNameRegionPartition {
    partition: Partition,
    region: Region,
    account: Account,
    resource: String,
}

impl RefNameRegionPartition {
    pub fn build<Se: Service>(self, service: Se) -> Arn<Se> {
        Arn {
            partition: self.partition,
            service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartialArn {
    account: Account,
    resource: String,
}

impl PartialArn {
    pub fn new(account: Account, resource: String) -> Self {
        Self { account, resource }
    }
}

impl Serialize for PartialArn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut contents: Vec<Box<dyn JoinElement>> = vec![];
        if let Some(account) = self.account.to_str() {
            contents.push(Box::new(format!("{account}:{}", self.resource)));
        } else {
            contents.push(Box::new(self.account.clone()));
            contents.push(Box::new(self.resource.clone()));
        }

        Join(contents).serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{partition::Partition, service};

    use super::*;

    #[test]
    fn test_arn1() {
        let arn = arn_builder("r", Account::Aws)
            .partition(Partition::Aws)
            .region(Region::Ref)
            .build(service::IAM);
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#"{"Fn::Join":["arn:aws:iam:",{"Ref":"AWS::Region"},":aws:r"]}"#;
        assert_eq!(s, rhs);
    }

    #[test]
    fn test_arn2() {
        let arn = arn_builder("r", Account::Aws)
            .partition(Partition::Aws)
            .build(service::IAM);
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#""arn:aws:iam::aws:r""#;
        assert_eq!(s, rhs);
    }

    #[test]
    fn test_arn3() {
        // minimum settings
        //  partition => pseudo::Partition
        //  region => no region
        let arn = arn_builder("r", Account::Aws).build(service::IAM);
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#"{"Fn::Join":["arn:",{"Ref":"AWS::Partition"},":iam::aws:r"]}"#;
        assert_eq!(s, rhs);
    }
}
