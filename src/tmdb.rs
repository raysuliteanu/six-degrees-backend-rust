use std::env;

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

const BASE_URL_V3: &str = "https://api.themoviedb.org/3";
const BEARER: &str = "Bearer";
const TOKEN_ENV_VAR: &str = "TMDB_TOKEN";

impl Default for PersonClient {
    fn default() -> Self {
        PersonClient::new(None)
    }
}

impl PersonClient {
    pub fn new(config: Option<SixDegreesConfig>) -> PersonClient {
        let headers = Self::create_headers();
        if let Ok(client) = Self::create_client(headers) {
            let mut base_url = BASE_URL_V3.to_string();
            if let Some(config) = config {
                base_url = config.base_url;
            }
            PersonClient {
                tmdb_client: client,
                search_url: format!("{}/search/person", base_url),
                details_url: format!("{}/person", base_url),
            }
        } else {
            // todo: handle error
            panic!("could not initialize TMDB client")
        }
    }

    fn create_client(headers: HeaderMap) -> reqwest::Result<reqwest::Client> {
        reqwest::Client::builder().default_headers(headers).build()
    }

    fn create_headers() -> HeaderMap {
        let auth_value = Self::create_auth_header();
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_value);
        headers
    }

    fn create_auth_header() -> HeaderValue {
        let token = env::var(TOKEN_ENV_VAR).unwrap().to_string();
        let bearer_token = format!("{} {}", BEARER, token);
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

    pub async fn search(&self, query: String) -> reqwest::Result<PersonSearchResult> {
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

    #[tokio::test]
    async fn person_search() {
        let person_search = PersonClient::default();
        let result = person_search.search("nicole+kidman".to_string()).await;
        if let Ok(person_search_result) = result {
            assert_eq!(person_search_result.total_results, 1);
            let search_results = person_search_result.results;
            assert_eq!(search_results.len(), 1);
            assert!(!search_results.first().unwrap().known_for.is_empty())
        }
    }

    #[tokio::test]
    async fn person_by_id() {
        let person_search = PersonClient::default();
        let result = person_search.get_by_id(2227).await;
        if let Some(person) = result {
            assert_eq!(person.id, 2227);
            assert_eq!(person.name, "Nicole Kidman")
        }
    }
}
