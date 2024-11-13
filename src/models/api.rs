use anyhow::{Context, Result};
use reqwest;
use serde::de::DeserializeOwned;
// use serde::Deserialize;

pub trait ApiClient {
    fn fetch_data<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        access_token: Option<String>,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
}

// pub struct Api {
//     base_url: String,
// }

// impl Api {
//     pub fn new(base_url: String) -> Self {
//         Api { base_url }
//     }
// }

// impl ApiClient for Api {
//     async fn fetch_data<T: DeserializeOwned>(
//         &self,
//         endpoint: &str,
//         access_token: Option<String>,
//     ) -> Result<T> {
//         let client = reqwest::Client::new();

//         let url = format!("{}{}", self.base_url, endpoint);

//         let request = match access_token {
//             Some(token) => client.get(url).bearer_auth(token),
//             None => client.get(url),
//         };
//         let response = request
//             .send()
//             .await
//             .context(format!("Failed to fetch data from {endpoint}"))?;
//         let response = response
//             .json::<T>()
//             .await
//             .context(format!("Failed to parse response from {endpoint}"))?;
//         Ok(response)
//     }
// }
