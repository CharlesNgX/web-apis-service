// Generated by diesel_ext
use crate::schema::*;
use diesel::{Queryable, Insertable, associations::Associations};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Rustacean, foreign_key = rustaceans_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustaceans_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Deserialize, Insertable)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustaceans_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = rustaceans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
