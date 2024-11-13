use crate::models::api::ApiClient;

type Tags = Vec<String>;

pub async fn load_tags(api: impl ApiClient, access_token: Option<&str>) -> Option<Tags> {
    match api.fetch_data("api/tags", access_token).await {
        Ok(tags) => Some(tags),
        Err(_) => None,
    }
}
