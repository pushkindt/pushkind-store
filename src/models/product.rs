use crate::env;
use crate::utils::make_backend_url;
use reqwest;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use leptos_oidc::utils::ParamBuilder;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum PriceLevel {
    online_store = 0,
    marketplace = 1,
    small_wholesale = 2,
    large_wholesale = 3,
    distributor = 4,
    exclusive = 5,
    retail = 6,
    retail_promo = 7,
}

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
    pub cat_id: u32,
    pub category: String,
    pub prices: Option<HashMap<PriceLevel, f32>>,
    pub image: Option<String>,
    pub measurement: Option<String>,
    pub description: Option<String>,
    pub options: Option<HashMap<String, Vec<String>>>,
    pub tags: Option<Vec<String>>,
}

impl Product {
    pub fn get_price(&self, price_level: &PriceLevel) -> f32 {
        match &self.prices {
            Some(prices) => match prices.get(price_level) {
                Some(price) => *price,
                None => self.price,
            },
            None => self.price,
        }
    }

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
    async fn load_from_url(url: &str) -> Option<Products> {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(_) => return None,
        };
        match response.json().await {
            Ok(products) => products,
            Err(_) => None,
        }
    }

    pub async fn load(
        cat_id: u32,
        page: Option<u32>,
        tag: &Option<String>,
        sort_by: &Option<String>,
    ) -> Option<Products> {
        let url = format!("api/category/{}/products", cat_id);

        let url = if let Some(tag) = tag {
            url.push_param_query("tag", tag)
        } else {
            url
        };
        let url = if let Some(page) = page {
            url.push_param_query("page", page.to_string())
        } else {
            url
        };
        let url = if let Some(sort_by) = sort_by {
            url.push_param_query("sort_by", sort_by)
        } else {
            url
        };

        let url = make_backend_url(&url);

        Products::load_from_url(&url).await
    }

    pub async fn search(key: &str, page: Option<u32>) -> Option<Products> {
        let page = match page {
            Some(page) => format!("&page={}", page),
            None => "".to_string(),
        };

        let url = make_backend_url(&format!("api/products/search?q={}{}", key, page));
        Products::load_from_url(&url).await
    }
}

impl Default for PriceLevel {
    fn default() -> Self {
        PriceLevel::online_store
    }
}

impl PriceLevel {
    pub fn to_string(&self) -> String {
        match self {
            PriceLevel::online_store => "ИНТЕРНЕТ",
            PriceLevel::marketplace => "МАРКЕТ",
            PriceLevel::small_wholesale => "МЕЛКИЙ ОПТ",
            PriceLevel::large_wholesale => "КРУПНЫЙ ОПТ",
            PriceLevel::distributor => "ДИСТРИБЬЮТОР",
            PriceLevel::exclusive => "ЭКСКЛЮЗИВ",
            PriceLevel::retail => "СЕТИ",
            PriceLevel::retail_promo => "СЕТИ ПРОМО",
        }
        .to_string()
    }
}
