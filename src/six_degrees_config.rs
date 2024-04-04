use serde::{Deserialize, Serialize};

const BASE_URL_V3: &str = "https://api.themoviedb.org/3";

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct SixDegreesConfig {
    pub base_url: String,
    pub api_token: String,
}

impl Default for SixDegreesConfig {
    fn default() -> Self {
        SixDegreesConfig {
            base_url: BASE_URL_V3.to_string(),
            api_token: "".to_string(),
        }
    }
}

