use crate::env;
use crate::utils::make_backend_url;
use reqwest;
use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use leptos_oidc::utils::ParamBuilder;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Default)]
pub enum PriceLevel {
    #[default]
    online_store = 0,
    marketplace = 1,
    small_wholesale = 2,
    large_wholesale = 3,
    distributor = 4,
    exclusive = 5,
    chains_vat = 6,
    chains_vat_promo = 7,
    chains_no_vat = 8,
    chains_no_vat_promo = 9,
    msrp_chains = 10,
    msrp_retail = 11,
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
    pub images: Option<Vec<String>>,
}

impl Product {
    pub fn get_image(&self) -> String {
        let image = match &self.image {
            Some(image) => image,
            None => env::APP_DEFAULT_PRODUCT_IMAGE,
        };
        match image
            .get(0..4)
            .map_or(false, |prefix| prefix.eq_ignore_ascii_case("http"))
        {
            true => image.to_string(),
            false => make_backend_url(image),
        }
    }

    pub async fn load(id: u32, access_token: Option<String>) -> Option<Product> {
        let url = make_backend_url(&format!("api/product/{}", id));
        let client = reqwest::Client::new();
        let request = match access_token {
            Some(token) => client.get(url).bearer_auth(token),
            None => client.get(url),
        };
        let response = match request.send().await {
            Ok(response) => response,
            Err(_) => return None,
        };
        (response.json().await).unwrap_or_default()
    }
}

impl Products {
    async fn load_from_url(url: &str, access_token: &Option<String>) -> Option<Products> {
        let client = reqwest::Client::new();
        let request = match access_token {
            Some(token) => client.get(url).bearer_auth(token),
            None => client.get(url),
        };
        let response = match request.send().await {
            Ok(response) => response,
            Err(_) => return None,
        };
        (response.json().await).unwrap_or_default()
    }

    pub async fn load(
        cat_id: u32,
        page: Option<u32>,
        tag: &Option<String>,
        sort_by: &Option<String>,
        access_token: &Option<String>,
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

        Products::load_from_url(&url, access_token).await
    }

    pub async fn search(
        key: &str,
        page: Option<u32>,
        access_token: &Option<String>,
    ) -> Option<Products> {
        let page = match page {
            Some(page) => format!("&page={}", page),
            None => "".to_string(),
        };

        let url = make_backend_url(&format!("api/products/search?q={}{}", key, page));
        Products::load_from_url(&url, access_token).await
    }
}

impl fmt::Display for PriceLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            PriceLevel::online_store => "ИНТЕРНЕТ",
            PriceLevel::marketplace => "МАРКЕТ",
            PriceLevel::small_wholesale => "МЕЛКИЙ ОПТ",
            PriceLevel::large_wholesale => "КРУПНЫЙ ОПТ",
            PriceLevel::distributor => "ДИСТРИБЬЮТОР",
            PriceLevel::exclusive => "ЭКСКЛЮЗИВ",
            PriceLevel::chains_vat => "СЕТИ С НДС",
            PriceLevel::chains_vat_promo => "СЕТИ С НДС ПРОМО",
            PriceLevel::chains_no_vat => "СЕТИ БЕЗ НДС",
            PriceLevel::chains_no_vat_promo => "СЕТИ БЕЗ НДС ПРОМО",
            PriceLevel::msrp_chains => "РРЦ СЕТИ",
            PriceLevel::msrp_retail => "РРЦ РОЗНИЦА",
        };
        write!(f, "{}", repr)
    }
}
