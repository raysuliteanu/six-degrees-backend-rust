use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug)]
pub struct SixDegreesConfig {
    #[serde(default)]
    pub base_url: String,
}

