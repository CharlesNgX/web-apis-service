use serde_json::{json, Value};
use std::env;

pub fn host() -> String {
    let host = match env::var("HOST") {
        Ok(v) => { v },
        Err(e) => { e.to_string() }
    };
    return host
}

pub async fn rustacean_create_common(client: &reqwest::Client) -> Value {

    let new_rustacean = json!({
        "name": "Test33 Rustacean",
        "email": "test11@example.com",
    });

    let response = client
        .post(&format!("{}/rustaceans", self::host()))
        .json(&new_rustacean)
        .send()
        .await
        .expect("Failed to create rustacean");
    assert!(response.status().is_success());

    response.json().await.expect("Invalid response")
}

pub async fn rustacean_delete_common(client: &reqwest::Client, id: i32) {
    let _ = client
        .delete(&format!("{}/rustaceans/{}", self::host(), id))
        .send()
        .await
        .expect("Failed to clean up test rustacean");
}

#[allow(dead_code)]
pub async fn crate_create_common(client: &reqwest::Client) -> Value {

    let rustacean = self::rustacean_create_common(&client).await;
    let rustaceans_id = rustacean["id"].as_i64().expect("No ID found") as i32;

    let new_crates = json!({
        "rustaceans_id": rustaceans_id,
        "name": "Charles",
        "code": "Web apis service",
        "version": "1.0.0",
        "description": "",
    });

    let response = client
        .post(&format!("{}/crates", self::host()))
        .json(&new_crates)
        .send()
        .await
        .expect("Failed to create rustacean");
    assert!(response.status().is_success());

    response.json().await.expect("Invalid response")
}

#[allow(dead_code)]
pub async fn crate_delete_common(client: &reqwest::Client, id: i32) {
    let _ = client
        .delete(&format!("{}/crates/{}", self::host(), id))
        .send()
        .await
        .expect("Failed to clean up test rustacean");
}

pub async fn user_create_common(client: &reqwest::Client) -> Value {

    let new_user = json!({
        "user": {
            "password": "12345",
            "username": "Test_Admin",
        },
        "role_code": "admin"
    });

    let response = client
        .post(&format!("{}/users", self::host()))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to create rustacean");
    assert!(response.status().is_success());

    response.json().await.expect("Invalid response")
}