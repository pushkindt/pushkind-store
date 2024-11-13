use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;

use pushkind_store::models::api::ApiClient;

struct MockApiClient {
    base_path: PathBuf,
}

impl MockApiClient {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
}

impl ApiClient for MockApiClient {
    async fn fetch_data<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        _access_token: Option<String>,
    ) -> Result<T> {
        let endpoint = endpoint.replace("/", "_");
        let file_path = self.base_path.join(format!("{}.json", endpoint));

        // Read the JSON file contents
        let file_content = fs::read_to_string(&file_path).context(format!(
            "Failed to read mock data from {}",
            file_path.display()
        ))?;

        // Deserialize the JSON content into the expected type
        let data = serde_json::from_str(&file_content)
            .context(format!("Failed to parse JSON from {}", file_path.display()))?;

        Ok(data)
    }
}

#[tokio::test]
async fn test_fetch_data_with_mock() -> Result<()> {
    // Set the base path to the directory where mock JSON files are stored
    let base_path = PathBuf::from("tests/test_data");

    // Create a mock API client with the base path
    let mock_client = MockApiClient::new(base_path);

    // Fetch data for a specific endpoint
    let data: serde_json::Value = mock_client.fetch_data("example/endpoint", None).await?;

    // Perform assertions on the fetched data
    assert_eq!(data["key"], "value");

    Ok(())
}
