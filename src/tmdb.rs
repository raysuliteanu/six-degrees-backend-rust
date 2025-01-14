use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::six_degrees_config::SixDegreesConfig;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PersonSearchResult {
    page: i32,
    total_pages: i32,
    total_results: i32,
    results: Vec<Person>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Person {
    id: i32,
    name: String,
    popularity: f32,
    #[serde(default)]
    known_for: Vec<Credit>,
    adult: bool,
    #[serde(default)]
    also_known_as: Vec<String>,
    biography: Option<String>,
    birthday: Option<String>,
    deathday: Option<String>,
    gender: i8,
    homepage: Option<String>,
    imdb_id: Option<String>,
    known_for_department: Option<String>,
    place_of_birth: Option<String>,
    profile_path: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Credit {
    id: i32,
    title: String,
    overview: String,
    media_type: String,
    release_date: String,
    popularity: f32,
    adult: bool,
    genre_ids: Vec<i32>,
    video: bool,
    original_language: String,
    original_title: String,
    poster_path: String,
    backdrop_path: String,
}

pub struct PersonClient {
    tmdb_client: reqwest::Client,
    search_url: String,
    details_url: String,
}

const BEARER: &str = "Bearer";

impl Default for PersonClient {
    fn default() -> Self {
        PersonClient::new(None)
    }
}

impl PersonClient {
    pub fn new(config: Option<SixDegreesConfig>) -> PersonClient {
        if let Some(config) = config {
            let headers = Self::create_headers(config.api_token);
            match Self::create_client(headers) {
                Ok(client) => {
                    PersonClient {
                        tmdb_client: client,
                        search_url: format!("{}/search/person", config.base_url),
                        details_url: format!("{}/person", config.base_url),
                    }
                }
                Err(error) => {
                    panic!("{:?}", error);
                }
            }
        } else {
            panic!("no configuration provided");
        }
    }

    fn create_client(headers: HeaderMap) -> reqwest::Result<reqwest::Client> {
        reqwest::Client::builder().default_headers(headers).build()
    }

    fn create_headers(auth_token: String) -> HeaderMap {
        let auth_value = Self::create_auth_header(auth_token);
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_value);
        headers
    }

    fn create_auth_header(auth_token: String) -> HeaderValue {
        let bearer_token = format!("{} {}", BEARER, auth_token);
        let mut auth_value = HeaderValue::from_str(bearer_token.as_str()).unwrap();
        auth_value.set_sensitive(true);
        auth_value
    }

    pub async fn get_by_id(&self, id: i32) -> Option<Person> {
        let url = format!("{}/{}", self.details_url, id);

        if let Ok(response) = self.tmdb_client.get(url).send().await.unwrap().text().await {
            let person = serde_json::from_str::<Person>(dbg!(response.as_str()));
            Some(person.unwrap())
        } else {
            None
        }
    }

    pub async fn search(&self, query: &str) -> reqwest::Result<PersonSearchResult> {
        let url = format!("{}?query={}", self.search_url, query);
        let response = self.tmdb_client.get(url).send().await.unwrap().text().await;

        if let Ok(result) = response {
            let person_search_result =
                serde_json::from_str::<PersonSearchResult>(dbg!(result.as_str()));
            Ok(person_search_result.unwrap())
        } else {
            Err(response.err().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const TOKEN_ENV_VAR: &str = "TMDB_API_TOKEN";

    fn setup() -> SixDegreesConfig {
        let token: String = env::var(TOKEN_ENV_VAR).unwrap().to_string();
        SixDegreesConfig {
            api_token: token,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn person_search() {
        let person_search = PersonClient::new(Some(setup()));
        let result = person_search.search("nicole+kidman").await;
        if let Ok(person_search_result) = result {
            assert_eq!(person_search_result.total_results, 1);
            let search_results = person_search_result.results;
            assert_eq!(search_results.len(), 1);
            assert!(!search_results.first().unwrap().known_for.is_empty())
        }
    }

    #[tokio::test]
    async fn person_by_id() {
        let person_search = PersonClient::new(Some(setup()));
        let result = person_search.get_by_id(2227).await;
        if let Some(person) = result {
            assert_eq!(person.id, 2227);
            assert_eq!(person.name, "Nicole Kidman")
        }
    }
}
