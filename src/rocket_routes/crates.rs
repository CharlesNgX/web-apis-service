use crate::repositories;
use crate::models::*;

use serde_json::Value;
use rocket::serde::json::{json, Json};
use rocket::http::Status;
use rocket::response::status::Custom;

#[get("/crates")]
pub fn get_crates() -> Result<Value, Value> {
    repositories::CrateReposity::find_mutilple(100)
        .map(|crates| json!(crates) )
        .map_err(|e| json!(e.to_string()))
}

#[get("/crates/<id>")]
pub fn view_crates(id: i32) -> Result<Value, Value> {
    repositories::CrateReposity::find(id)
        .map(|a_crate| json!(a_crate) )
        .map_err(|e| json!(e.to_string()))
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub fn create_crates(new_crate: Json<NewCrate>) -> Result<Value, Value> {
    repositories::CrateReposity::create(new_crate.into_inner())
        .map(|a_crate| json!(a_crate) )
        .map_err(|e| json!(e.to_string()))
}

#[put("/crates/<id>", format = "json", data = "<a_crate>")]
pub fn update_crates(id: i32, a_crate: Json<Crate>) -> Result<Value, Value> {
    repositories::CrateReposity::save(id, a_crate.into_inner())
        .map(|a_crate| json!(a_crate) )
        .map_err(|e| json!(e.to_string()))
}

#[delete("/crates/<id>")]
pub fn delete_crates(id: i32) -> Result<Value, Custom<Value>> {
    repositories::CrateReposity::delete(id)
        .map(|result|  json!(result) )
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())) )
}