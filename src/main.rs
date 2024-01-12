#[macro_use] extern crate rocket;

mod models;
mod repositories;
mod schema;
mod rocket_routes;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::{Rocket, Build};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_routes::crates::*;
use rocket_routes::rustaceans::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}, {e}", database_url))
}

#[derive(Default)]
struct RocketIgnite;

#[async_trait]
impl Fairing for RocketIgnite {
    fn info(&self) -> Info {
        Info {
            name: "Rocket Ignite",
            kind: Kind::Ignite | Kind::Response
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> { 
        Ok(rocket.mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            create_rustaceans,
            update_rustaceans,
            delete_rustaceans
        ]).mount("/", routes![
            get_crates,
            view_crates,
            create_crates,
            update_crates,
            delete_crates
        ])
        )
    }
}

#[rocket::main]
async fn main() {

    let _ = rocket::build()
        .attach(RocketIgnite::default())
        .launch()
        .await;
}
