use nutype::nutype;

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w._-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct SAMLProviderName(String);

#[nutype(
    validate(len_char_min = 1000, len_char_max = 10000000),
    derive(Debug, Clone, Serialize)
)]
pub struct SAMLMetadataDocument(String);
