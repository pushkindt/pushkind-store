use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum PriceLevels {
    online_store,
    marketplace,
    small_wholesale,
    large_wholesale,
    distributor,
    exclusive,
    retail,
    retail_promo,
}

impl Default for PriceLevels {
    fn default() -> Self {
        PriceLevels::online_store
    }
}

impl PriceLevels {
    pub fn to_string(&self) -> String {
        match self {
            PriceLevels::online_store => "ИНТЕРНЕТ",
            PriceLevels::marketplace => "МАРКЕТ",
            PriceLevels::small_wholesale => "МЕЛКИЙ ОПТ",
            PriceLevels::large_wholesale => "КРУПНЫЙ ОПТ",
            PriceLevels::distributor => "ДИСТРИБЬЮТОР",
            PriceLevels::exclusive => "ЭКСКЛЮЗИВ",
            PriceLevels::retail => "СЕТИ",
            PriceLevels::retail_promo => "СЕТИ ПРОМО",
        }
        .to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub sub: String,
    pub name: String,
    pub price_level: PriceLevels,
    pub email: String,
}
