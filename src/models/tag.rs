use crate::utils::make_backend_url;
use reqwest;

type Tags = Vec<String>;

pub async fn load_tags(access_token: &Option<String>) -> Option<Tags> {
    let url = make_backend_url("api/tags");
    let client = reqwest::Client::new();
    let request = match access_token {
        Some(token) => client.get(url).bearer_auth(token),
        None => client.get(url),
    };
    let response = match request.send().await {
        Ok(response) => response,
        Err(_) => return None,
    };
    match response.json().await {
        Ok(tags) => Some(tags),
        Err(_) => None,
    }
}
