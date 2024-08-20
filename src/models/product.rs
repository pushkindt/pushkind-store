use crate::env;
use crate::utils::make_backend_url;
use reqwest;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Products {
    pub total: u32,
    pub page: u32,
    pub pages: u32,
    pub products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Product {
    pub id: u32,
    pub vendor: String,
    pub name: String,
    pub sku: String,
    pub price: f32,
    pub prices: Option<HashMap<String, f32>>,
    pub image: Option<String>,
    pub measurement: Option<String>,
    pub cat_id: u32,
    pub category: String,
    pub description: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
}

impl Product {
    pub fn get_image(&self) -> String {
        let image = match &self.image {
            Some(image) => image,
            None => env::APP_DEFAULT_PRODUCT_IMAGE,
        };
        make_backend_url(image)
    }

    pub async fn load(id: u32) -> Option<Product> {
        let url = make_backend_url(&format!("api/product/{}", id));
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(_) => return None,
        };
        match response.json().await {
            Ok(product) => product,
            Err(_) => None,
        }
    }
}

impl Products {
    async fn load_from_url(url: String) -> Option<Products> {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(_) => return None,
        };
        match response.json().await {
            Ok(products) => products,
            Err(_) => None,
        }
    }

    pub async fn load(cat_id: i32, page: i32) -> Option<Products> {
        Products::load_from_url(make_backend_url(&format!(
            "api/category/{}/products?page={}",
            cat_id, page
        )))
        .await
    }

    pub async fn search(key: String, page: i32) -> Option<Products> {
        Products::load_from_url(make_backend_url(&format!(
            "api/products/search?q={}&page={}",
            key, page
        )))
        .await
    }
}
