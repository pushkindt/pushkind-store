use crate::utils::make_backend_url;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub children: Vec<(i32, String)>,
}

impl Category {
    pub async fn load(root: i32) -> Option<Category> {
        let url = make_backend_url(&format!("api/category/{}", root));
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(_) => return None,
        };
        match response.json().await {
            Ok(category) => Some(category),
            Err(_) => None,
        }
    }
}
