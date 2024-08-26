use serde::{Deserialize, Serialize};

use crate::models::product::PriceLevel;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct User {
    pub sub: String,
    pub name: String,
    pub price_level: PriceLevel,
    pub email: String,
}
