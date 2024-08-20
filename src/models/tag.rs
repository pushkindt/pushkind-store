use crate::utils::make_backend_url;
use reqwest;

type Tags = Vec<String>;

pub async fn load_tags() -> Option<Tags> {
    let url = make_backend_url("api/tags");
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_) => return None,
    };
    match response.json().await {
        Ok(tags) => Some(tags),
        Err(_) => None,
    }
}
