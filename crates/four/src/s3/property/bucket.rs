use nutype::nutype;

#[nutype(
    validate(
        len_char_min = 3,
        len_char_max = 63,
        regex = r#"^[0-9a-z][0-9A-Za-z\.\-]+[0-9a-z]$"#,
        predicate = |s| {
            // TODO: custsom validation (cf. https://docs.aws.amazon.com/ja_jp/AmazonS3/latest/userguide/bucketnamingrules.html)
            true
        }
    ),
    derive(Debug, Clone, Deserialize, Serialize, Deref)
)]
pub struct BucketName(String);

#[nutype(
    validate(len_char_min = 1,),
    derive(Debug, Clone, Deserialize, Serialize, Deref)
)]
pub struct Key(String);
