use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct IndexersSettings {
    #[serde(default)]
    pub tokens: TokenIndexerSettings,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct TokenIndexerSettings {
    #[serde(default = "default_batch_size")]
    pub batch: u64,
}

fn default_batch_size() -> u64 {
    50
}
