use std::default::Default;
use rocket::{get, State};
use rocket::serde::json::Json;
use crate::six_degrees_config::SixDegreesConfig;
use crate::tmdb::{Person, PersonClient, PersonSearchResult};

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

#[get("/person/<id>")]
pub async fn person_detail(id: i32, controller: &State<RequestController>) -> Json<Person> {
    Json(controller.person_client.get_by_id(id).await.unwrap())
}

#[get("/search/person/<query>")]
pub async fn person_search(query: &str, controller: &State<RequestController>) -> Json<PersonSearchResult> {
    Json(controller.person_client.search(query).await.unwrap())
}
