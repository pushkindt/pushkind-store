use anyhow::{Context, Result};
use reqwest;
use serde::de::DeserializeOwned;

pub trait ApiClient {
    fn fetch_data<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        access_token: Option<&str>,
    ) -> impl std::future::Future<Output = Result<T>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Api {
    pub base_url: String,
}

impl Api {
    pub fn new(base_url: String) -> Self {
        Api { base_url }
    }
}

impl ApiClient for Api {
    async fn fetch_data<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        access_token: Option<&str>,
    ) -> Result<T> {
        // Step 1. Create the URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Step 2. Build a reqwest client
        let client = reqwest::Client::new();
        let request = match access_token {
            Some(token) => client.get(url).bearer_auth(token),
            None => client.get(url),
        };

        // Step 3. Make an HTTP call
        let response = request
            .send()
            .await
            .context(format!("Failed to fetch data from {endpoint}"))?;

        // Step 4. Check for HTTP status
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(anyhow::anyhow!(
                "HTTP error {status}: {body} from {endpoint}"
            ));
        }

        // Step 5. Parse and return JSON
        let response = response
            .json::<T>()
            .await
            .context(format!("Failed to parse response from {endpoint}"))?;
        Ok(response)
    }
}
