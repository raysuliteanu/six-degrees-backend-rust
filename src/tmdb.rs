use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;

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

pub struct PersonSearch {
    tmdb_client: reqwest::blocking::Client,
}

impl PersonSearch {
    const BEARER: &'static str = "Bearer ";

    pub fn new() -> PersonSearch {
        let bearer_token = String::from(Self::BEARER);
        let token = env::var("TMDB_TOKEN").unwrap().to_string();
        let bearer_token = bearer_token + token.as_str();
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(bearer_token.as_str()).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        if let Ok(client) = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
        {
            PersonSearch {
                tmdb_client: client,
            }
        } else {
            // todo: handle error
            panic!("could not initialize TMDB client")
        }
    }

    pub fn person_search(&self, query: String) -> reqwest::Result<PersonSearchResult> {
        let response = self
            .tmdb_client
            .get("https://api.themoviedb.org/3/search/person")
            .query(&[("query", query)])
            .send();

        if let Ok(result) = response {
            let text = result.text().unwrap();
            let person_search_result = serde_json::from_str::<PersonSearchResult>(dbg!(text.as_str()));
            Ok(person_search_result.unwrap())
        } else {
            eprintln!("{:?}", response.as_ref().unwrap().status().to_string());
            Err(response.err().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tmdb::PersonSearch;

    #[test]
    fn person_search() {
        let person_search = PersonSearch::new();
        let result = person_search.person_search("nicole+kidman".to_string());
        if let Ok(person_search_result) = result {
            assert_eq!(person_search_result.total_results, 1);
            let search_results = person_search_result.results;
            assert_eq!(search_results.len(), 1);
            assert!(search_results.get(0).unwrap().known_for.len() > 0)
        } else {
            panic!("{:?}", result);
        }
    }
}
