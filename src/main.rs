use figment::{Figment, providers::{Format, Yaml, Env}, Result};
use six_degrees_backend_rust::six_degrees_config::SixDegreesConfig;
// use six_degrees_backend_rust::tmdb;

#[macro_use] extern crate rocket;

#[get("/person")]
fn person_detail() -> &'static str {
    "Hello, world!"
}

#[get("/search/person")]
fn person_search() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {

    match load_config() {
        Ok(app_config) => {
            println!("{:?}", app_config);

            // todo: use app_config
    
            rocket::build()
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
