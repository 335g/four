use nutype::nutype;
use regex::Regex;

use std::sync::LazyLock;

static BUCKET_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[0-9a-z]([0-9a-z\.-])+[0-9a-z]$"#).unwrap());

static IP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap());

fn bucket_name_predicate(s: &str) -> bool {
    if !BUCKET_NAME_REGEX.is_match(s) {
        return false;
    }

    if s.contains("..") {
        return false;
    }

    if IP_REGEX.is_match(s) {
        return false;
    }

    if s.starts_with("xn--")
        || s.starts_with("sthree-")
        || s.starts_with("sthree-configurator")
        || s.ends_with("-s3alias")
        || s.ends_with("--ol-s3")
    {
        return false;
    }

    true
}

#[nutype(
    validate(
        len_char_min = 3,
        len_char_max = 63,
        predicate = bucket_name_predicate,
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct BucketName(String);
