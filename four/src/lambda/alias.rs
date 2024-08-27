use derive_new::new;
use nutype::nutype;
use serde::Serialize;

use crate::convert::WillBe;

#[nutype(
    validate(not_empty, len_char_max = 256),
    derive(Debug, Clone, Serialize)
)]
pub struct AliasDescription(String);

#[nutype(
    validate(regex = r#"\$LATEST|[0-9]+"#),
    derive(Debug, Clone, Serialize)
)]
pub struct FunctionVersion(String);

#[nutype(validate(regex = r#"[a-zA-Z0-9-_]+"#), derive(Debug, Clone, Serialize))]
pub struct AliasName(String);

#[derive(Debug, Clone, Serialize, new)]
#[serde(rename_all = "PascalCase")]
pub struct AliasRoutingConfiguration {
    additional_version_weights: Vec<VersionWeight>,
}

#[derive(Debug, Clone, Serialize, new)]
#[serde(rename_all = "PascalCase")]
pub struct VersionWeight {
    function_version: WillBe<FunctionVersion>,
    function_weight: f32,
}

pub fn version_weight(version: WillBe<FunctionVersion>, weight: f32) -> AliasRoutingConfiguration {
    AliasRoutingConfiguration {
        additional_version_weights: vec![VersionWeight::new(version, weight)],
    }
}

pub fn version_weights<W: IntoIterator<Item = (WillBe<FunctionVersion>, f32)>>(
    weights: W,
) -> AliasRoutingConfiguration {
    let weights = weights
        .into_iter()
        .map(|(v, w)| VersionWeight::new(v, w))
        .collect();
    AliasRoutingConfiguration {
        additional_version_weights: weights,
    }
}
