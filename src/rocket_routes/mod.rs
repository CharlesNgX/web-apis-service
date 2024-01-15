use std::error::Error;

use rocket::response::status::Custom;
use rocket::http::Status;
use serde_json::{Value, json};

pub mod rustaceans;
pub mod crates;

fn server_error(e: &Box<dyn Error> ) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Something went wrong"))
}