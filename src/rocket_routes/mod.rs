pub mod rustaceans;
pub mod crates;
pub mod authorization;

pub use crate::models::User;
use crate::repositories::UserRepository;

use std::error::Error;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::Request;
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde_json::{Value, json};

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct RedisConn(rocket_db_pools::deadpool_redis::Pool);

fn server_error(e: &Box<dyn Error> ) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Something went wrong"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = request.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer" );

        match session_header {
            Some(v) => { 
                let mut db = request.guard::<Connection<DbConn>>()
                    .await
                    .expect("Can not connect to Postgres in request guard");

                let mut redis = request.guard::<Connection<RedisConn>>()
                    .await
                    .expect("Can not connect to Redis in request guard");
                let result = redis.get::<String, i32>(format!("sessions/{}", v[1])).await;
                match result {
                    Ok(v) => {
                        if let Ok(user) = UserRepository::find(&mut db, v).await {
                            return Outcome::Success(user)
                        }
                    },
                    Err(_) => { eprintln!("eeeererer"); }
                };
            },
            None => { }
        };

        Outcome::Error((Status::Unauthorized, ()))
    }

}

