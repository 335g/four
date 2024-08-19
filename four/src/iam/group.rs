use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;

#[nutype(
    validate(not_empty, len_char_max = 128, regex = r#"[\w+=,.@-]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct GroupName(String);

static GROUP_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[\w+=,.@-]+"#).unwrap());

fn valid_groups(groups: &Vec<String>) -> bool {
    groups
        .into_iter()
        .map(|g| !g.is_empty() && g.len() < 128 && GROUP_REGEX.is_match(g))
        .fold(false, |acc, x| acc || x)
}

#[nutype(
    validate(predicate = valid_groups),
    derive(Debug, Clone, Serialize)
)]
pub struct Groups(Vec<String>);
