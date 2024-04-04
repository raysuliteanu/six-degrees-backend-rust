use std::default::Default;
use crate::six_degrees_config::SixDegreesConfig;
use crate::tmdb::PersonClient;

pub struct RequestController {
    pub person_client: PersonClient,
}

impl RequestController {
    pub fn new(config: Option<SixDegreesConfig>) -> Self {
        RequestController {
            person_client: PersonClient::new(config),
        }
    }
}

impl Default for RequestController {
    fn default() -> Self {
        RequestController::new(None)
    }
}