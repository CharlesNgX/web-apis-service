use crate::DbConn;
use crate::repositories;
use crate::models::*;

use rocket::response::status::NoContent;
use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;
use rocket::response::status::Custom;

#[get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Value> {
    db.run(|c| {
        repositories::CrateReposity::find_mutilple(c, 100)
            .map(|crates| json!(crates) )
            .map_err(|e| json!(e.to_string()))
    }).await
}

#[get("/crates/<id>")]
pub async fn view_crates(db: DbConn, id: i32) -> Result<Value, Value> {
    db.run(move |c| {
        repositories::CrateReposity::find(c, id)
            .map(|a_crate| json!(a_crate) )
            .map_err(|e| json!(e.to_string()))
    }).await
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crates(db: DbConn, new_crate: Json<NewCrate>) -> Result<Value, Value> {
    db.run(|c| {
        repositories::CrateReposity::create(c, new_crate.into_inner())
            .map(|a_crate| json!(a_crate) )
            .map_err(|e| json!(e.to_string()))
    }).await
}

#[put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crates(db: DbConn, id: i32, a_crate: Json<Crate>) -> Result<Value, Value> {
    db.run(move |c| {
        repositories::CrateReposity::save(c, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate) )
            .map_err(|e| {
                error!("{:?}", e.to_string());
                json!(e.to_string())
            })
    }).await
}

#[delete("/crates/<id>")]
pub async fn delete_crates(db: DbConn, id: i32) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        repositories::CrateReposity::delete(c, id)
            .map(|_|  NoContent )
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())) )
    }).await
}