use std::default::Default;
use crate::tmdb::PersonClient;

pub struct RequestController {
    pub person_client: PersonClient,
}

impl RequestController {
    pub fn new() -> Self {
        RequestController {
            person_client: PersonClient::default(),
        }
    }
}

impl Default for RequestController {
    fn default() -> Self {
        RequestController::new()
    }
}