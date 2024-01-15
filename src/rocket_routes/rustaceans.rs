use crate::repositories;
use crate::models::*;
use crate::DbConn;

use rocket::response::status::NoContent;
use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;
use rocket::response::status::Custom;

#[get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Value> {
    db.run(|c| {
        repositories::RustaceanRepository::find_mutilple(c, 100)
        .map(|rustaceans| json!(rustaceans) )
        .map_err(|e| json!(e.to_string()))
    }).await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustaceans(db: DbConn, id: i32) -> Result<Value, Value> {
    db.run(move |c| {
        repositories::RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean) )
            .map_err(|e| json!(e.to_string()))
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustaceans(db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Value> {
    db.run(|c| {
        repositories::RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean) )
            .map_err(|e| json!(e.to_string()))
    }).await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustaceans(db: DbConn, id: i32, rustacean: Json<Rustacean>) -> Result<Value, Value> {
    db.run(move |c| {
        repositories::RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean) )
            .map_err(|e| json!(e.to_string()))
    }).await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustaceans(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        repositories::RustaceanRepository::delete(c, id)
            .map(|_|  NoContent )
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}