use crate::repositories;
use crate::models::*;

use serde_json::Value;
use rocket::serde::json::{json, Json};
use rocket::http::Status;
use rocket::response::status::Custom;

#[get("/rustaceans")]
pub fn get_rustaceans() -> Result<Value, Value> {
    repositories::RustaceanRepository::find_mutilple(100)
        .map(|rustaceans| json!(rustaceans) )
        .map_err(|e| json!(e.to_string()))
}

#[get("/rustaceans/<id>")]
pub fn view_rustaceans(id: i32) -> Result<Value, Value> {
    repositories::RustaceanRepository::find(id)
        .map(|rustacean| json!(rustacean) )
        .map_err(|e| json!(e.to_string()))
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub fn create_rustaceans(new_rustacean: Json<NewRustacean>) -> Result<Value, Value> {
    repositories::RustaceanRepository::create(new_rustacean.into_inner())
        .map(|rustacean| json!(rustacean) )
        .map_err(|e| json!(e.to_string()))
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub fn update_rustaceans(id: i32, rustacean: Json<Rustacean>) -> Result<Value, Value> {
    repositories::RustaceanRepository::save(id, rustacean.into_inner())
        .map(|rustacean| json!(rustacean) )
        .map_err(|e| json!(e.to_string()))
}

#[delete("/rustaceans/<id>")]
pub fn delete_rustaceans(id: i32) -> Result<Value, Custom<Value>> {
    repositories::RustaceanRepository::delete(id)
        .map(|result|  json!(result) )
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())) )
}