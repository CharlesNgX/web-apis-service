use serde_json::{json, Value};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref HOST: String = {
        match env::var("HOST") {
            Ok(v) => v,
            Err(e) => e.to_string(),
        }
    };
}

pub fn host() -> &'static String {
    &HOST
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
    let id = rustacean["id"].as_i64().expect("No ID found") as i32;

    let new_crates = json!({
        "rustaceans_id": id,
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