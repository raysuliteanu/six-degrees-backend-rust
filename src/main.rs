#[macro_use] extern crate rocket;

use figment::{Figment, providers::{Env, Format, Yaml}, Result};
use rocket::serde::json::Json;
use rocket::State;

use six_degrees_backend_rust::request_controller::RequestController;
use six_degrees_backend_rust::six_degrees_config::SixDegreesConfig;
use six_degrees_backend_rust::tmdb::{Person, PersonSearchResult};

#[get("/person/<id>")]
async fn person_detail(id: i32, controller: &State<RequestController>) -> Json<Person> {
    Json(controller.person_client.get_by_id(id).await.unwrap())
}

#[get("/search/person/<query>")]
async fn person_search(query: &str, controller: &State<RequestController>) -> Json<PersonSearchResult> {
    Json(controller.person_client.search(query.to_string()).await.unwrap())
}

#[launch]
fn rocket() -> _ {
    match load_config() {
        Ok(app_config) => {
            println!("{:?}", app_config);

            // todo: use app_config
            
            let controller = RequestController::default();
    
            rocket::build()
                .manage(controller)
                .mount("/", routes![person_detail, person_search])    
        },
        Err(error) => {
            panic!("{:?}", error);
        }
    }
}

fn load_config() -> Result<SixDegreesConfig> {
    let config : SixDegreesConfig = Figment::new()
        .merge(Env::prefixed("TMDB"))
        .merge(Env::prefixed("SIX_DEGREES"))
        .join(Yaml::file("6d.yaml"))
        .extract()?;

    Ok(config)
}
