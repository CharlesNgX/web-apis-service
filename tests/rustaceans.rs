mod common;

#[cfg(test)]
mod tests {
    
    use reqwest::Client;
    use serde_json::{json, Value};
    use crate::common;

    #[tokio::test]
    async fn test_create_rustaceans() {
        let client = Client::new();
        let rustacean: Value = common::rustacean_create_common(&client).await;
        let id = rustacean["id"].as_i64().expect("No ID found") as i32;
        common::rustacean_delete_common(&client, id).await;
    }

    #[tokio::test]
    async fn test_get_rustaceans() {
        let client = Client::new();
        let response = client
            .get(&format!("{}/rustaceans", common::host()))
            .send()
            .await
            .expect("Failed to retrieve rustaceans");
        assert!(response.status().is_success());
        let rustaceans: Vec<Value> = response.json().await.expect("Invalid response");
        assert!(!rustaceans.is_empty());
    }

    #[tokio::test]
    async fn test_view_rustaceans() {
        let client = Client::new();
        let rustacean = common::rustacean_create_common(&client).await;
        let id = rustacean["id"].as_i64().expect("No ID found") as i32;

        let view_response = client
            .get(&format!("{}/rustaceans/{}", common::host(), id))
            .send()
            .await
            .expect("Failed to view rustacean");

        assert!(view_response.status().is_success());

        let viewed_rustacean: Value = view_response.json().await.expect("Invalid response");
        assert_eq!(viewed_rustacean["id"].as_i64().expect("No ID found") as i32, id);

        common::rustacean_delete_common(&client, id).await;
    }

    #[tokio::test]
    async fn test_update_rustaceans() {
        let client = Client::new();
        let rustacean = common::rustacean_create_common(&client).await;
        let id = rustacean["id"].as_i64().expect("No ID found") as i32;

        let updated_rustacean = json!({
            "id": id,
            "name": "Updated Test Rustacean",
            "email": "updated_test@example.com",
            "created_at": rustacean["created_at"],
        });

        let update_response = client
            .put(&format!("{}/rustaceans/{}", common::host(), id))
            .json(&updated_rustacean)
            .send()
            .await
            .expect("Failed to update rustacean");

        assert!(update_response.status().is_success());

        common::rustacean_delete_common(&client, id).await;
    }

    #[tokio::test]
    async fn test_delete_rustaceans() {
        let client = Client::new();

        let new_rustacean = json!({
            "name": "Test Rustacean Delete",
            "email": "test_delete@example.com",
        });

        let create_response = client
            .post(&format!("{}/rustaceans", common::host()))
            .json(&new_rustacean)
            .send()
            .await
            .expect("Failed to create rustacean for delete test");

        let rustacean: Value = create_response.json().await.expect("Invalid response");
        let id = rustacean["id"].as_i64().expect("No ID found") as i32;

        let delete_response = client
            .delete(&format!("{}/rustaceans/{}", common::host(), id))
            .send()
            .await
            .expect("Failed to delete rustacean");
        assert!(delete_response.status().is_success());
    }

  
}