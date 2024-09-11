use crate::utils::make_backend_url;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub children: Vec<(i32, String)>,
}

impl Category {
    pub async fn load(root: u32, access_token: &Option<String>) -> Option<Category> {
        let url = make_backend_url(&format!("api/category/{}", root));

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
            Ok(category) => Some(category),
            Err(_) => None,
        }
    }
}
