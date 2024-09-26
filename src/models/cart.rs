use crate::env;
use crate::models::product::Product;
use crate::utils::make_backend_url;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::product::PriceLevel;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ShoppingCart {
    pub items: HashMap<u32, CartItem>,
    pub comment: Option<String>,
}

impl ShoppingCart {
    pub fn add_item(&mut self, product: Product, quantity: u32, comment: Option<String>) {
        let product_id = product.id;
        let item = CartItem {
            product,
            quantity,
            comment,
        };
        self.items.insert(product_id, item);
    }

    pub fn remove_item(&mut self, product_id: u32) {
        self.items.remove(&product_id);
    }

    pub fn update_item_quantity(&mut self, product_id: u32, quantity: u32) {
        if let Some(item) = self.items.get_mut(&product_id) {
            item.quantity = quantity;
        }
    }

    pub fn update_item_comment(&mut self, product_id: u32, comment: Option<String>) {
        if let Some(item) = self.items.get_mut(&product_id) {
            item.comment = comment;
        }
    }

    pub fn get_total_price(&self, price_level: &PriceLevel, discount: f32) -> f32 {
        self.items
            .values()
            .map(|item| item.product.get_price(price_level, discount) * item.quantity as f32)
            .sum()
    }

    pub fn get_item_count(&self) -> usize {
        self.items.len()
    }

    pub fn get_total_quantity(&self) -> u32 {
        self.items.values().map(|item| item.quantity).sum()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub async fn submit(&mut self, access_token: &Option<String>) -> Result<(), String> {
        let url = make_backend_url(env::APP_CART_URL);

        let client = Client::new();
        let request = match access_token {
            Some(token) => client.post(url).bearer_auth(token),
            None => client.post(url),
        };
        match request.json(&self).send().await {
            Ok(response) => match response.status().is_success() {
                true => Ok(()),
                false => Err("Failed to submit cart".to_string()),
            },
            Err(_) => Err("Failed to submit cart".to_string()),
        }
    }
}
