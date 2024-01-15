#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

mod models;
mod repositories;
mod schema;
mod rocket_routes;

use rocket::fairing::AdHoc;
use rocket::{Rocket, Build};
use rocket_routes::crates::*;
use rocket_routes::rustaceans::*;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>>  {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
    DbConn::get_one(&rocket).await
        .expect("---- 333 Failed database connection ----")
        .run(|conn| match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => { },
            Err(e) => {
                panic!("faild conn {e:?}");
            }
         })
        .await;
    Ok(rocket)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
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
        .attach(DbConn::fairing())
        .attach(AdHoc::try_on_ignite("Running DB Migrations", run_migrations))
        .launch()
        .await;
}
