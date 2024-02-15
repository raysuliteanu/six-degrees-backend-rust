use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::header::HeaderValue;

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
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Credit {
    id: i32,
    title: String,
    overview: String,
    media_type: String,
    release_date: String,
    popularity: f32,
}

pub struct PersonClient {
    tmdb_client: reqwest::blocking::Client,
    search_url: String,
    details_url: String
}

impl PersonClient {
    const BASE_URL_V3: &'static str = "https://api.themoviedb.org/3";
    const BEARER: &'static str = "Bearer";
    const TOKEN_ENV_VAR: &'static str = "TMDB_TOKEN";

    pub fn new() -> PersonClient {
        let auth_value = Self::create_auth_header();
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_value);
        if let Ok(client) = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
        {
            PersonClient {
                tmdb_client: client,
                search_url: format!("{}/search/person", Self::BASE_URL_V3),
                details_url: format!("{}/person", Self::BASE_URL_V3),
            }
        } else {
            // todo: handle error
            panic!("could not initialize TMDB client")
        }
    }

    fn create_auth_header() -> HeaderValue {
        let token = env::var(Self::TOKEN_ENV_VAR).unwrap().to_string();
        let bearer_token = format!("{} {}", Self::BEARER, token);
        let mut auth_value = header::HeaderValue::from_str(bearer_token.as_str()).unwrap();
        auth_value.set_sensitive(true);
        auth_value
    }

    pub fn get_by_id(&self, id: i32) -> Option<Person> {
        let url = format!("{}/{}", self.details_url, id);
        let response = self.tmdb_client.get(url).send();

        if let Ok(result) = response {
            let text = result.text().unwrap();
            let person = serde_json::from_str::<Person>(dbg!(text.as_str()));
            Some(person.unwrap())
        } else {
            eprintln!("{:?}", response.as_ref().unwrap().status().to_string());
            None
        }
    }

    pub fn search(&self, query: String) -> reqwest::Result<PersonSearchResult> {
        let url = format!("{}?query={}", self.search_url, query);
        let response = self.tmdb_client
            .get(url)
            .send();

        if let Ok(result) = response {
            let text = result.text().unwrap();
            let person_search_result =
                serde_json::from_str::<PersonSearchResult>(dbg!(text.as_str()));
            Ok(person_search_result.unwrap())
        } else {
            eprintln!("{:?}", response.as_ref().unwrap().status().to_string());
            Err(response.err().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_search() {
        let person_search = PersonClient::new();
        let result = person_search.search("nicole+kidman".to_string());
        if let Ok(person_search_result) = result {
            assert_eq!(person_search_result.total_results, 1);
            let search_results = person_search_result.results;
            assert_eq!(search_results.len(), 1);
            assert!(search_results.get(0).unwrap().known_for.len() > 0)
        } else {
            panic!("{:?}", result);
        }
    }

    #[test]
    fn person_by_id() {
        let person_search = PersonClient::new();
        let result = person_search.get_by_id(2227);
        if let Some(person) = result {
            assert_eq!(person.id, 2227);
            assert_eq!(person.name, "Nicole Kidman")
        }
    }
}
