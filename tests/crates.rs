mod common;

#[cfg(test)]
mod tests {
    
    use reqwest::Client;
    use serde_json::{json, Value};
    use crate::common;

    #[tokio::test]
    async fn test_create_crates() {
        let client = Client::new();
        let rustacean = common::crate_create_common(&client).await;
        let id = rustacean["id"].as_i64().expect("No ID found") as i32;
        common::crate_delete_common(&client, id).await;
        common::rustacean_delete_common(&client, id).await;
    }

    #[tokio::test]
    async fn test_get_crates() {
        let client = Client::new();
        let response = client
            .get(&format!("{}/crates", common::host()))
            .send()
            .await
            .expect("Failed to retrieve rustaceans");
        assert!(response.status().is_success());
        let crates: Vec<Value> = response.json().await.expect("Invalid response");
        assert!(!crates.is_empty());
    }

    #[tokio::test]
    async fn test_view_crates() {
        let client = Client::new();
        let a_crate = common::crate_create_common(&client).await;
        let id = a_crate["id"].as_i64().expect("No ID found") as i32;
        let rustaceans_id = a_crate["rustaceans_id"].as_i64().expect("No ID found") as i32;

        let view_response = client
            .get(&format!("{}/crates/{}", common::host(), id))
            .send()
            .await
            .expect("Failed to view rustacean");

        assert!(view_response.status().is_success());

        let viewed_crates: Value = view_response.json().await.expect("Invalid response");
        assert_eq!(viewed_crates["id"].as_i64().expect("No ID found") as i32, id);

        common::crate_delete_common(&client, id).await;
        common::rustacean_delete_common(&client, rustaceans_id).await;
    }

    #[tokio::test]
    async fn test_update_crates() {
        let client = Client::new();
        let a_crate = common::crate_create_common(&client).await;
        let id = a_crate["id"].as_i64().expect("No ID found") as i32;
        let rustaceans_id = a_crate["rustaceans_id"].as_i64().expect("No ID found") as i32;

        let update_crate = json!({
            "id": id,
            "rustaceans_id": rustaceans_id,
            "name": "CR",
            "code": "Swift Dev",
            "version": "2.0.0",
            "description": "23",
            "created_at":  a_crate["created_at"]
        });

        let update_response = client
            .put(&format!("{}/crates/{}", common::host(), id))
            .json(&update_crate)
            .send()
            .await
            .expect("Failed to update rustacean");

        assert!(update_response.status().is_success());

        common::crate_delete_common(&client, id).await;
        common::rustacean_delete_common(&client, rustaceans_id).await;
    }

    #[tokio::test]
    async fn test_delete_crates() {
        let client = Client::new();

        let new_rustacean = json!({
            "name": "Test33 Rustacean",
            "email": "test11@example.com",
        });
    
        let response = client
            .post(&format!("{}/rustaceans", common::host()))
            .json(&new_rustacean)
            .send()
            .await
            .expect("Failed to create rustacean");
        assert!(response.status().is_success());
    
        let rustacean: Value = response.json().await.expect("Invalid response");
        let rustaceans_id = rustacean["id"].as_i64().expect("No ID found") as i32;
    
        let new_crates = json!({
            "rustaceans_id": rustaceans_id,
            "name": "Charles",
            "code": "Web apis service",
            "version": "1.0.0",
            "description": "",
        });
    
        let response = client
            .post(&format!("{}/crates", common::host()))
            .json(&new_crates)
            .send()
            .await
            .expect("Failed to create rustacean");
        assert!(response.status().is_success());
    
        let a_crate: Value = response.json().await.expect("Invalid response");
        let id = a_crate["id"].as_i64().expect("No ID found") as i32;

        let delete_response = client
            .delete(&format!("{}/crates/{}", common::host(), id))
            .send()
            .await
            .expect("Failed to delete rustacean");

        assert!(delete_response.status().is_success());

        let delete_response = client
            .delete(&format!("{}/rustaceans/{}", common::host(), rustaceans_id))
            .send()
            .await
            .expect("Failed to delete rustacean");

        assert!(delete_response.status().is_success());
    }

  
}