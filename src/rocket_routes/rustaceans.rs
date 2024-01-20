use crate::repositories::RustaceanRepository;
use crate::models::*;

use rocket::response::status::NoContent;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status::Custom;

use rocket_db_pools::Connection;
use super::{DbConn, server_error};

#[get("/rustaceans")]
pub async fn get_rustaceans(mut c: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_mutilple(&mut c, 100).await
            .map(|rustaceans| json!(rustaceans) )
            .map_err(|e| server_error(&e.into()) )
}

#[get("/rustaceans/<id>")]
pub async fn view_rustaceans(mut c: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut c, id).await
        .map(|rustacean| json!(rustacean) )
        .map_err(|e| server_error(&e.into()) )
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustaceans(mut c: Connection<DbConn>, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::create(&mut c, new_rustacean.into_inner()).await
        .map(|rustacean| json!(rustacean) )
        .map_err(|e| server_error(&e.into()) )
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustaceans(mut c: Connection<DbConn>, id: i32, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::save(&mut c, id, rustacean.into_inner()).await
        .map(|rustacean| json!(rustacean) )
        .map_err(|e| server_error(&e.into()) )
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustaceans(mut c: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut c, id).await
        .map(|_|  NoContent )
        .map_err(|e| server_error(&e.into()) )
}