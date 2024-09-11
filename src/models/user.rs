use crate::env;
use crate::models::product::PriceLevel;
use leptos_oidc::{Algorithm, Auth};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct User {
    pub sub: String,
    pub name: String,
    pub price_level: PriceLevel,
    pub discount: f32,
    pub email: String,
    pub phone: Option<String>,
}

impl User {
    pub fn from_auth(auth: &Auth) -> User {
        auth.decoded_id_token::<User>(Algorithm::RS256, &[env::APP_SIGNIN_CLIENT])
            .flatten()
            .map(|token| token.claims)
            .unwrap_or_default()
    }
}
