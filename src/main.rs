#[macro_use]
extern crate rocket;

use figment::providers::Serialized;
use figment::{
    providers::{Env, Format, Yaml},
    Figment, Result,
};
use log::info;
use six_degrees_backend_rust::request_controller::{
    person_detail, person_search, RequestController,
};
use six_degrees_backend_rust::six_degrees_config::SixDegreesConfig;

#[launch]
fn rocket() -> _ {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();

    match load_config() {
        Ok(app_config) => {
            let controller = RequestController::new(Some(app_config));

            rocket::build()
                .manage(controller)
                .mount("/", routes![person_detail, person_search])
        }
        Err(error) => {
            panic!("{:?}", error);
        }
    }
}

fn load_config() -> Result<SixDegreesConfig> {
    let config: SixDegreesConfig = Figment::from(Serialized::defaults(SixDegreesConfig::default()))
        .merge(Yaml::file("6d.yaml"))
        .merge(Env::prefixed("TMDB_"))
        .merge(Env::prefixed("SIX_DEGREES_"))
        .extract()?;

    info!("{:?}", config);

    Ok(config)
}
