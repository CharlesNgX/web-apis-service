mod common;

#[cfg(test)]
mod tests {
    
    use reqwest::Client;
    use serde_json::{json, Value};
    use crate::common;

    #[tokio::test]
    async fn test_create_crates() {
        let client = Client::new();
        let user = common::user_create_common(&client).await;
        let id = user["id"].as_i64().expect("No ID found") as i32;


        // common::crate_delete_common(&client, id).await;
        // common::rustacean_delete_common(&client, id).await;
    }

    #[tokio::test]
    async fn test_view_user() {
        let client = Client::new();
        let user = common::user_create_common(&client).await;
        let id = user["id"].as_i64().expect("No ID found") as i32;

        let view_response = client
            .get(&format!("{}/users/{}", common::host(), id))
            .send()
            .await
            .expect("Failed to view user");

        assert!(view_response.status().is_success());

        let viewed_rustacean: Value = view_response.json().await.expect("Invalid response");
        assert_eq!(viewed_rustacean["id"].as_i64().expect("No ID found") as i32, id);
    }

}