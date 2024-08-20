use crate::models::product::Product;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ShoppingCart {
    pub items: HashMap<u32, CartItem>,
}
