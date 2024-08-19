use nutype::nutype;

#[nutype(
    validate(
        not_empty,
        len_char_max = 16384,
        regex = r#"[\u0009\u000A\u000D\u0020-\u00FF]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct CertificateBody(String);

#[nutype(
    validate(
        not_empty,
        len_char_max = 2097152,
        regex = r#"[\u0009\u000A\u000D\u0020-\u00FF]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct CertificateChain(String);

#[nutype(
    validate(
        not_empty,
        len_char_max = 16384,
        regex = r#"[\u0009\u000A\u000D\u0020-\u00FF]+"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct PrivateKey(String);

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct ServerCertificateName(String);
