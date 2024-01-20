#[macro_use] extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;
mod rocket_routes;

use rocket_routes::DbConn;
use rocket_routes::RedisConn;
use rocket_routes::authorization::create_user;
use rocket_routes::authorization::login;
use rocket_routes::authorization::view_user;
use rocket_routes::crates::*;
use rocket_routes::rustaceans::*;
use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            create_rustaceans,
            update_rustaceans,
            delete_rustaceans
        ])
        .mount("/", routes![
            get_crates,
            view_crates,
            create_crates,
            update_crates,
            delete_crates
        ])
        .mount("/", routes![
            view_user,
            create_user,
            login
        ])
        .attach(RedisConn::init())
        .attach(DbConn::init())
        .launch()
        .await;
}
