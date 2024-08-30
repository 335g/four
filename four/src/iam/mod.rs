mod access_key;
mod arn;
mod group;
mod id;
mod instance_profile;
mod path;
mod policy;
mod property;
pub mod resource;
mod role;
mod saml_provider;
mod server_certificate;
mod user;

pub use access_key::{AccessKeyStatus, SecretAccessKey};
pub use arn::{
    GroupArn, InstanceProfileArn, ManagedPolicyArn, OIDCProviderArn, RoleArn, SAMLProviderArn,
    ServerCertificateArn, UserArn,
};
pub use group::{
    GroupName, GroupNameError, GroupPolicy, GroupPolicyName, GroupPolicyNameError, Groups,
};
pub use id::{
    AccessKeyId, GroupPolicyId, InstanceProfileId, PolicyId, RoleId, RolePolicyId, UserPolicyId,
    UserToGroupAdditionId,
};
pub use instance_profile::{InstanceProfileName, InstanceProfileNameError};
pub use path::{Path, PathError};
pub use policy::{PolicyName, PolicyNameError};
pub use property::{
    action,
    effect::Effect,
    policy_document::{PolicyDocument, PolicyDocumentVersion},
    principal::{Principal, ServicePrincipal},
    statement::{ActionOr, PrincipalOr, Statement},
    AWSManagedPolicy, ManagedPolicyDescription, ManagedPolicyDescriptionError,
};
pub use role::RoleName;
pub use saml_provider::{
    SAMLMetadataDocument, SAMLMetadataDocumentError, SAMLProviderName, SAMLProviderNameError,
};
pub use server_certificate::{
    CertificateBody, CertificateBodyError, CertificateChain, CertificateChainError, PrivateKey,
    PrivateKeyError, ServerCertificateName, ServerCertificateNameError,
};
pub use user::{LoginProfile, Policy, UserName, UserNameError};
