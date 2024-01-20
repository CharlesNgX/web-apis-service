use crate::repositories::CrateReposity;
use crate::models::*;

use rocket::response::status::NoContent;
use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;
use rocket::response::status::Custom;

use rocket_db_pools::Connection;
use super::{server_error, DbConn};

#[get("/crates")]
pub async fn get_crates(mut c: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    CrateReposity::find_mutilple(&mut c, 100).await
        .map(|crates| json!(crates) )
        .map_err(|e| server_error(&e.into()) )
}

#[get("/crates/<id>")]
pub async fn view_crates(mut c: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateReposity::find(&mut c, id).await
        .map(|a_crate| json!(a_crate) )
        .map_err(|e| server_error(&e.into()) )
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crates(mut c: Connection<DbConn>, new_crate: Json<NewCrate>) -> Result<Value, Custom<Value>> {
    CrateReposity::create(&mut c, new_crate.into_inner()).await
        .map(|a_crate| json!(a_crate) )
        .map_err(|e| server_error(&e.into()) )
}

#[put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crates(mut c: Connection<DbConn>, id: i32, a_crate: Json<Crate>) -> Result<Value, Custom<Value>> {
    CrateReposity::save(&mut c, id, a_crate.into_inner()).await
        .map(|a_crate| json!(a_crate) )
        .map_err(|e| server_error(&e.into()) )
}

#[delete("/crates/<id>")]
pub async fn delete_crates(mut c: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateReposity::delete(&mut c, id).await
        .map(|_|  NoContent )
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())) )
}