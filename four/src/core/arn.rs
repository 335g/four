use serde::{Serialize, Serializer};

use crate::core::{
    account::Account,
    function::join::{Join, JoinElement},
    partition::Partition,
    region::Region,
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

impl<Se> Arn<Se>
where
    Se: Service,
{
    pub fn builder(service: Se, resource: &str, account: Account) -> RefNameAccount<Se> {
        RefNameAccount {
            service,
            account,
            resource: resource.to_string(),
        }
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
        #[derive(Clone)]
        enum StringOr {
            String(String),
            Or(Box<dyn JoinElement>),
        }

        let mut contents: Vec<StringOr> = vec![];
        if let Some(partition) = self.partition.to_str() {
            contents.push(StringOr::String("arn".to_string()));
            contents.push(StringOr::String(partition.to_string()));
            contents.push(StringOr::String(self.service.to_string()));
        } else {
            contents.push(StringOr::String("arn".to_string()));
            contents.push(StringOr::Or(Box::new(self.partition)));
            contents.push(StringOr::String(self.service.to_string()));
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
}

pub struct RefNameAccount<Se> {
    service: Se,
    account: Account,
    resource: String,
}

impl<Se> RefNameAccount<Se>
where
    Se: Service,
{
    pub fn region(self, region: Region) -> RefNameRegion<Se> {
        RefNameRegion {
            service: self.service,
            region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn partition(self, partition: Partition) -> RefNamePartition<Se> {
        RefNamePartition {
            partition,
            service: self.service,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build(self) -> Arn<Se> {
        Arn {
            partition: Partition::Ref,
            service: self.service,
            region: Region::Null,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNameRegion<Se> {
    service: Se,
    region: Region,
    account: Account,
    resource: String,
}

impl<Se> RefNameRegion<Se>
where
    Se: Service,
{
    pub fn partition(self, partition: Partition) -> RefNameRegionPartition<Se> {
        RefNameRegionPartition {
            partition,
            service: self.service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build(self) -> Arn<Se> {
        Arn {
            partition: Partition::Ref,
            service: self.service,
            region: self.region,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNamePartition<Se> {
    partition: Partition,
    service: Se,
    account: Account,
    resource: String,
}

impl<Se> RefNamePartition<Se>
where
    Se: Service,
{
    pub fn region(self, region: Region) -> RefNameRegionPartition<Se> {
        RefNameRegionPartition {
            partition: self.partition,
            service: self.service,
            region,
            account: self.account,
            resource: self.resource,
        }
    }

    pub fn build(self) -> Arn<Se> {
        Arn {
            partition: self.partition,
            service: self.service,
            region: Region::Null,
            account: self.account,
            resource: self.resource,
        }
    }
}

pub struct RefNameRegionPartition<Se> {
    partition: Partition,
    service: Se,
    region: Region,
    account: Account,
    resource: String,
}

impl<Se> RefNameRegionPartition<Se>
where
    Se: Service,
{
    pub fn build(self) -> Arn<Se> {
        Arn {
            partition: self.partition,
            service: self.service,
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
        let arn = Arn::builder(service::IAM, "r", Account::Aws)
            .partition(Partition::Aws)
            .region(Region::Ref)
            .build();
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#"{"Fn::Join":["arn:aws:iam:",{"Ref":"AWS::Region"},":aws:r"]}"#;
        assert_eq!(s, rhs);
    }

    #[test]
    fn test_arn2() {
        let arn = Arn::builder(service::IAM, "r", Account::Aws)
            .partition(Partition::Aws)
            .build();
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#""arn:aws:iam::aws:r""#;
        assert_eq!(s, rhs);
    }

    #[test]
    fn test_arn3() {
        // minimum settings
        //  partition => pseudo::Partition
        //  region => no region
        let arn = Arn::builder(service::IAM, "r", Account::Aws).build();
        let s = serde_json::to_string(&arn).unwrap();

        let rhs = r#"{"Fn::Join":["arn:",{"Ref":"AWS::Partition"},":iam::aws:r"]}"#;
        assert_eq!(s, rhs);
    }
}
