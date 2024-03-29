use std::str::FromStr;

use crate::mail::HtmlMailer;
use crate::models::{Credentials, UserModel, RoleCode, NewUser};
use crate::repositories::UserRepository;
use crate::auth::{*, self};

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use chrono::{Utc, Datelike};
use tera::Context;

use super::{DbConn, RedisConn, server_error, load_template_engine};

#[post("/login", format="json", data="<credentials>")]
pub async fn login(mut c: Connection<DbConn>, mut cache: Connection<RedisConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut c, &credentials.username).await
        .map_err(|e| server_error(&e.into()) )?;
    
    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")) )?;

    cache.set_ex::<String, i32, ()>(
        format!("sessions/{}", session_id),
        user.id, 
        3*60*60
    )
    .await
    .map_err(|e| server_error(&e.into()) )?;

    Ok(json!({
        "token": session_id
    }))
}

#[post("/users", format="json", data="<new_user_model>")]
pub async fn create_user(mut c: Connection<DbConn>, new_user_model: Json<UserModel>) -> Result<Value, Custom<Value>> {
    let model: UserModel = new_user_model.into_inner();
    let role = RoleCode::from_str(&model.role_code).ok();
    
    let new_user = NewUser {
        username: model.user.username,
        password: auth::hash_password(model.user.password).unwrap() 
    };

    if let Some(role) = role {
        let tera = load_template_engine().expect("cannot load template engine");
        return UserRepository::create(&mut c, new_user, vec![role])
            .await
            .map(|user| {
            
                let year = Utc::now().year();
                let mut context = Context::new();
                context.insert("users", &vec![&user]);
                context.insert("year", &year);

                let smtp_host = std::env::var("SMTP_HOST")
                    .expect("Cannot load SMTP host from environment");
                let smtp_username = std::env::var("SMTP_USERNAME")
                    .expect("Cannot load SMTP username from environment");
                let smtp_password =  std::env::var("SMTP_PASSWORD")
                    .expect("Cannot load SMTP password from environment");

                let mailer = HtmlMailer { template_engine: tera, smtp_host, smtp_username, smtp_password };
                let _ = mailer.send("charles.ng.x@gmail.com".to_string(), "email/digest.html", context).map_err(|e| panic!("{:?}", e) );
                json!(user)
            })
            .map_err(|e| server_error(&e.into()) )
    } else {
        return Err(Custom(Status::InternalServerError, json!("Something went wrong")))
    }
}

#[get("/users/<id>")]
pub async fn view_user(mut c: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    UserRepository::find(&mut c, id).await
        .map(|user| json!(user) )
        .map_err(|e| server_error(&e.into()) )
}