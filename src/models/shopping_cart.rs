use crate::models::product::Product;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ShoppingCart {
    // product_id, (quantity, product)
    pub items: HashMap<i32, (u32, Product)>,
}
